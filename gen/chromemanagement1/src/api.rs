use std::collections::HashMap;
use std::cell::RefCell;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;

use crate::client;

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// See detailed information about apps installed on Chrome browsers and devices managed by your organization
    ChromeManagementAppdetailReadonly,

    /// See reports about devices and Chrome browsers managed within your organization
    ChromeManagementReportReadonly,

    /// See basic device and telemetry information collected from Chrome OS devices or users managed within your organization
    ChromeManagementTelemetryReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::ChromeManagementAppdetailReadonly => "https://www.googleapis.com/auth/chrome.management.appdetails.readonly",
            Scope::ChromeManagementReportReadonly => "https://www.googleapis.com/auth/chrome.management.reports.readonly",
            Scope::ChromeManagementTelemetryReadonly => "https://www.googleapis.com/auth/chrome.management.telemetry.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::ChromeManagementAppdetailReadonly
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all ChromeManagement related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_chromemanagement1 as chromemanagement1;
/// use chromemanagement1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use chromemanagement1::{ChromeManagement, oauth2, hyper, hyper_rustls};
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().apps_android_get("name")
///              .doit().await;
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::Io(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
#[derive(Clone)]
pub struct ChromeManagement<> {
    pub client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>,
    pub auth: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, > client::Hub for ChromeManagement<> {}

impl<'a, > ChromeManagement<> {

    pub fn new(client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>, authenticator: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>) -> ChromeManagement<> {
        ChromeManagement {
            client,
            auth: authenticator,
            _user_agent: "google-api-rust-client/3.0.0".to_string(),
            _base_url: "https://chromemanagement.googleapis.com/".to_string(),
            _root_url: "https://chromemanagement.googleapis.com/".to_string(),
        }
    }

    pub fn customers(&'a self) -> CustomerMethods<'a> {
        CustomerMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/3.0.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://chromemanagement.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://chromemanagement.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Android app information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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
/// 
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
    pub first_publish_time: Option<String>,
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
    pub latest_publish_time: Option<String>,
    /// Output only. Format: name=customers/{customer_id}/apps/{chrome|android|web}/{app_id}@{version}
    pub name: Option<String>,
    /// Output only. The URI pointing to the privacy policy of the app, if it was provided by the developer. Version-specific field that will only be set when the requested app version is found.
    #[serde(rename="privacyPolicyUri")]
    pub privacy_policy_uri: Option<String>,
    /// Output only. The publisher of the item.
    pub publisher: Option<String>,
    /// Output only. Number of reviews received. Chrome Web Store review information will always be for the latest version of an app.
    #[serde(rename="reviewNumber")]
    pub review_number: Option<String>,
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


/// Battery info
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1BatteryInfo {
    /// Output only. Design capacity (mAmpere-hours).
    #[serde(rename="designCapacity")]
    pub design_capacity: Option<String>,
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


/// Sampling data for battery.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1BatterySampleReport {
    /// Output only. Battery charge percentage.
    #[serde(rename="chargeRate")]
    pub charge_rate: Option<i32>,
    /// Output only. Battery current (mA).
    pub current: Option<String>,
    /// Output only. The battery discharge rate measured in mW. Positive if the battery is being discharged, negative if it's being charged.
    #[serde(rename="dischargeRate")]
    pub discharge_rate: Option<i32>,
    /// Output only. Battery remaining capacity (mAmpere-hours).
    #[serde(rename="remainingCapacity")]
    pub remaining_capacity: Option<String>,
    /// Output only. Timestamp of when the sample was collected on device
    #[serde(rename="reportTime")]
    pub report_time: Option<String>,
    /// Output only. Battery status read from sysfs. Example: Discharging
    pub status: Option<String>,
    /// Output only. Temperature in Celsius degrees.
    pub temperature: Option<i32>,
    /// Output only. Battery voltage (millivolt).
    pub voltage: Option<String>,
}

impl client::Part for GoogleChromeManagementV1BatterySampleReport {}


/// Status data for battery.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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
    pub full_charge_capacity: Option<String>,
    /// Output only. Timestamp of when the sample was collected on device
    #[serde(rename="reportTime")]
    pub report_time: Option<String>,
    /// Output only. Sampling data for the battery.
    pub sample: Option<Vec<GoogleChromeManagementV1BatterySampleReport>>,
    /// Output only. Battery serial number.
    #[serde(rename="serialNumber")]
    pub serial_number: Option<String>,
}

impl client::Part for GoogleChromeManagementV1BatteryStatusReport {}


/// Describes a browser version and its install count.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1BrowserVersion {
    /// Output only. The release channel of the installed browser.
    pub channel: Option<String>,
    /// Output only. Count grouped by device_system and major version
    pub count: Option<String>,
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1ChromeAppInfo {
    /// Output only. Whether the app or extension is built and maintained by Google. Version-specific field that will only be set when the requested app version is found.
    #[serde(rename="googleOwned")]
    pub google_owned: Option<bool>,
    /// Output only. Whether the app or extension is in a published state in the Chrome Web Store.
    #[serde(rename="isCwsHosted")]
    pub is_cws_hosted: Option<bool>,
    /// Output only. Whether the app or extension is a theme.
    #[serde(rename="isTheme")]
    pub is_theme: Option<bool>,
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
}

impl client::Part for GoogleChromeManagementV1ChromeAppInfo {}


/// Permission requested by a Chrome app or extension.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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
    pub latest_request_time: Option<String>,
    /// Output only. Total count of requests for this app.
    #[serde(rename="requestCount")]
    pub request_count: Option<String>,
}

impl client::Part for GoogleChromeManagementV1ChromeAppRequest {}


/// Represent one host permission.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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
/// 
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


/// Response containing requested browser versions details and counts.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports count chrome versions customers](CustomerReportCountChromeVersionCall) (response)
/// 
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
/// 
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


/// CPU specs for a CPU.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1CpuInfo {
    /// Output only. The CPU architecture.
    pub architecture: Option<String>,
    /// Output only. The max CPU clock speed in kHz.
    #[serde(rename="maxClockSpeed")]
    pub max_clock_speed: Option<i32>,
    /// Output only. The CPU model name. Example: Intel(R) Core(TM) i5-8250U CPU @ 1.60GHz
    pub model: Option<String>,
}

impl client::Part for GoogleChromeManagementV1CpuInfo {}


/// Contains samples of the cpu status reports.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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
    pub report_time: Option<String>,
    /// Output only. Frequency the report is sampled.
    #[serde(rename="sampleFrequency")]
    pub sample_frequency: Option<String>,
}

impl client::Part for GoogleChromeManagementV1CpuStatusReport {}


/// CPU temperature of a device. Sampled per CPU core in Celsius
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1Device {
    /// Output only. The ID of the device that reported this Chrome browser information.
    #[serde(rename="deviceId")]
    pub device_id: Option<String>,
    /// Output only. The name of the machine within its local network.
    pub machine: Option<String>,
}

impl client::Part for GoogleChromeManagementV1Device {}


/// Status of the single storage device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1DiskInfo {
    /// Output only. Number of bytes read since last boot.
    #[serde(rename="bytesReadThisSession")]
    pub bytes_read_this_session: Option<String>,
    /// Output only. Number of bytes written since last boot.
    #[serde(rename="bytesWrittenThisSession")]
    pub bytes_written_this_session: Option<String>,
    /// Output only. Time spent discarding since last boot. Discarding is writing to clear blocks which are no longer in use. Supported on kernels 4.18+.
    #[serde(rename="discardTimeThisSession")]
    pub discard_time_this_session: Option<String>,
    /// Output only. Disk health.
    pub health: Option<String>,
    /// Output only. Counts the time the disk and queue were busy, so unlike the fields above, parallel requests are not counted multiple times.
    #[serde(rename="ioTimeThisSession")]
    pub io_time_this_session: Option<String>,
    /// Output only. Disk manufacturer.
    pub manufacturer: Option<String>,
    /// Output only. Disk model.
    pub model: Option<String>,
    /// Output only. Time spent reading from disk since last boot.
    #[serde(rename="readTimeThisSession")]
    pub read_time_this_session: Option<String>,
    /// Output only. Disk serial number.
    #[serde(rename="serialNumber")]
    pub serial_number: Option<String>,
    /// Output only. Disk size.
    #[serde(rename="sizeBytes")]
    pub size_bytes: Option<String>,
    /// Output only. Disk type: eMMC / NVMe / ATA / SCSI.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Output only. Disk volumes.
    #[serde(rename="volumeIds")]
    pub volume_ids: Option<Vec<String>>,
    /// Output only. Time spent writing to disk since last boot.
    #[serde(rename="writeTimeThisSession")]
    pub write_time_this_session: Option<String>,
}

impl client::Part for GoogleChromeManagementV1DiskInfo {}


/// Information for a display.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1DisplayInfo {
    /// Output only. Represents the graphics card device id.
    #[serde(rename="deviceId")]
    pub device_id: Option<String>,
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
/// 
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1GraphicsAdapterInfo {
    /// Output only. Adapter name. Example: Mesa DRI Intel(R) UHD Graphics 620 (Kabylake GT2).
    pub adapter: Option<String>,
    /// Output only. Represents the graphics card device id.
    #[serde(rename="deviceId")]
    pub device_id: Option<String>,
    /// Output only. Version of the GPU driver.
    #[serde(rename="driverVersion")]
    pub driver_version: Option<String>,
}

impl client::Part for GoogleChromeManagementV1GraphicsAdapterInfo {}


/// Information of the graphics subsystem.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1GraphicsInfo {
    /// Output only. Information about the graphics adapter (GPU).
    #[serde(rename="adapterInfo")]
    pub adapter_info: Option<GoogleChromeManagementV1GraphicsAdapterInfo>,
}

impl client::Part for GoogleChromeManagementV1GraphicsInfo {}


/// Information of the graphics subsystem.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1GraphicsStatusReport {
    /// Output only. Information about the displays for the device.
    pub displays: Option<Vec<GoogleChromeManagementV1DisplayInfo>>,
    /// Output only. Time at which the graphics data was reported.
    #[serde(rename="reportTime")]
    pub report_time: Option<String>,
}

impl client::Part for GoogleChromeManagementV1GraphicsStatusReport {}


/// Describes an installed app.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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
    pub browser_device_count: Option<String>,
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
    pub os_user_count: Option<String>,
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
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1ListTelemetryDevicesResponse {
    /// Telemetry devices returned in the response.
    pub devices: Option<Vec<GoogleChromeManagementV1TelemetryDevice>>,
    /// Token to specify next page in the list.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleChromeManagementV1ListTelemetryDevicesResponse {}


/// Memory information of a device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1MemoryInfo {
    /// Output only. Amount of available RAM in bytes.
    #[serde(rename="availableRamBytes")]
    pub available_ram_bytes: Option<String>,
    /// Output only. Total RAM in bytes.
    #[serde(rename="totalRamBytes")]
    pub total_ram_bytes: Option<String>,
}

impl client::Part for GoogleChromeManagementV1MemoryInfo {}


/// Contains samples of memory status reports.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1MemoryStatusReport {
    /// Output only. Number of page faults during this collection
    #[serde(rename="pageFaults")]
    pub page_faults: Option<i32>,
    /// Output only. The timestamp in milliseconds representing time at which this report was sampled.
    #[serde(rename="reportTime")]
    pub report_time: Option<String>,
    /// Output only. Frequency the report is sampled.
    #[serde(rename="sampleFrequency")]
    pub sample_frequency: Option<String>,
    /// Output only. Amount of free RAM in bytes (unreliable due to Garbage Collection).
    #[serde(rename="systemRamFreeBytes")]
    pub system_ram_free_bytes: Option<String>,
}

impl client::Part for GoogleChromeManagementV1MemoryStatusReport {}


/// State of visible/configured networks.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1NetworkStatusReport {
    /// Output only. Gateway IP address.
    #[serde(rename="gatewayIpAddress")]
    pub gateway_ip_address: Option<String>,
    /// Output only. LAN IP address.
    #[serde(rename="lanIpAddress")]
    pub lan_ip_address: Option<String>,
    /// Output only. Time at which the network state was reported.
    #[serde(rename="reportTime")]
    pub report_time: Option<String>,
    /// Output only. Frequency the report is sampled.
    #[serde(rename="sampleFrequency")]
    pub sample_frequency: Option<String>,
    /// Output only. Signal strength for wireless networks measured in decibels.
    #[serde(rename="signalStrengthDbm")]
    pub signal_strength_dbm: Option<i32>,
}

impl client::Part for GoogleChromeManagementV1NetworkStatusReport {}


/// Contains information regarding the current OS update status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1OsUpdateStatus {
    /// Output only. Timestamp of the last reboot.
    #[serde(rename="lastRebootTime")]
    pub last_reboot_time: Option<String>,
    /// Output only. Timestamp of the last update check.
    #[serde(rename="lastUpdateCheckTime")]
    pub last_update_check_time: Option<String>,
    /// Output only. Timestamp of the last successful update.
    #[serde(rename="lastUpdateTime")]
    pub last_update_time: Option<String>,
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


/// Status data for storage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1StorageInfo {
    /// The available space for user data storage in the device in bytes.
    #[serde(rename="availableDiskBytes")]
    pub available_disk_bytes: Option<String>,
    /// The total space for user data storage in the device in bytes.
    #[serde(rename="totalDiskBytes")]
    pub total_disk_bytes: Option<String>,
    /// Information for disk volumes
    pub volume: Option<Vec<GoogleChromeManagementV1StorageInfoDiskVolume>>,
}

impl client::Part for GoogleChromeManagementV1StorageInfo {}


/// Information for disk volumes
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1StorageInfoDiskVolume {
    /// Free storage space in bytes.
    #[serde(rename="storageFreeBytes")]
    pub storage_free_bytes: Option<String>,
    /// Total storage space in bytes.
    #[serde(rename="storageTotalBytes")]
    pub storage_total_bytes: Option<String>,
    /// Disk volume id.
    #[serde(rename="volumeId")]
    pub volume_id: Option<String>,
}

impl client::Part for GoogleChromeManagementV1StorageInfoDiskVolume {}


/// Status data for storage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1StorageStatusReport {
    /// Output only. Reports on disk
    pub disk: Option<Vec<GoogleChromeManagementV1DiskInfo>>,
    /// Output only. Timestamp of when the sample was collected on device
    #[serde(rename="reportTime")]
    pub report_time: Option<String>,
}

impl client::Part for GoogleChromeManagementV1StorageStatusReport {}


/// Telemetry data collected from a managed device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1TelemetryDevice {
    /// Output only. Information on battery specs for the device.
    #[serde(rename="batteryInfo")]
    pub battery_info: Option<Vec<GoogleChromeManagementV1BatteryInfo>>,
    /// Output only. Battery reports collected periodically.
    #[serde(rename="batteryStatusReport")]
    pub battery_status_report: Option<Vec<GoogleChromeManagementV1BatteryStatusReport>>,
    /// Output only. Information regarding CPU specs for the device.
    #[serde(rename="cpuInfo")]
    pub cpu_info: Option<Vec<GoogleChromeManagementV1CpuInfo>>,
    /// Output only. CPU status reports collected periodically.
    #[serde(rename="cpuStatusReport")]
    pub cpu_status_report: Option<Vec<GoogleChromeManagementV1CpuStatusReport>>,
    /// Output only. Google Workspace Customer whose enterprise enrolled the device.
    pub customer: Option<String>,
    /// Output only. The unique Directory API ID of the device. This value is the same as the Admin Console's Directory API ID in the Chrome OS Devices tab
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
    /// Output only. Memory status reports collected periodically.
    #[serde(rename="memoryStatusReport")]
    pub memory_status_report: Option<Vec<GoogleChromeManagementV1MemoryStatusReport>>,
    /// Output only. Resource name of the device.
    pub name: Option<String>,
    /// Output only. Network specs collected periodically.
    #[serde(rename="networkStatusReport")]
    pub network_status_report: Option<Vec<GoogleChromeManagementV1NetworkStatusReport>>,
    /// Output only. Organization unit ID of the device.
    #[serde(rename="orgUnitId")]
    pub org_unit_id: Option<String>,
    /// Output only. Contains relevant information regarding ChromeOS update status.
    #[serde(rename="osUpdateStatus")]
    pub os_update_status: Option<Vec<GoogleChromeManagementV1OsUpdateStatus>>,
    /// Output only. Device serial number. This value is the same as the Admin Console's Serial Number in the Chrome OS Devices tab.
    #[serde(rename="serialNumber")]
    pub serial_number: Option<String>,
    /// Output only. Information of storage specs for the device.
    #[serde(rename="storageInfo")]
    pub storage_info: Option<GoogleChromeManagementV1StorageInfo>,
    /// Output only. Storage reports collected periodically.
    #[serde(rename="storageStatusReport")]
    pub storage_status_report: Option<Vec<GoogleChromeManagementV1StorageStatusReport>>,
}

impl client::Part for GoogleChromeManagementV1TelemetryDevice {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleRpcStatus {
    /// The status code, which should be an enum value of google.rpc.Code.
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    pub details: Option<Vec<HashMap<String, String>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    pub message: Option<String>,
}

impl client::Part for GoogleRpcStatus {}


/// Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values. * A month and day, with a zero year (for example, an anniversary). * A year on its own, with a zero month and a zero day. * A year and month, with a zero day (for example, a credit card expiration date). Related types: * google.type.TimeOfDay * google.type.DateTime * google.protobuf.Timestamp
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *customer* resources.
/// It is not used directly, but through the `ChromeManagement` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_chromemanagement1 as chromemanagement1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use chromemanagement1::{ChromeManagement, oauth2, hyper, hyper_rustls};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `apps_android_get(...)`, `apps_chrome_get(...)`, `apps_count_chrome_app_requests(...)`, `apps_web_get(...)`, `reports_count_chrome_versions(...)`, `reports_count_installed_apps(...)`, `reports_find_installed_app_devices(...)` and `telemetry_devices_list(...)`
/// // to build up your call.
/// let rb = hub.customers();
/// # }
/// ```
pub struct CustomerMethods<'a>
    where  {

    hub: &'a ChromeManagement<>,
}

impl<'a> client::MethodsBuilder for CustomerMethods<'a> {}

impl<'a> CustomerMethods<'a> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a specific app for a customer by its resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The app for which details are being queried. Examples: "customers/my_customer/apps/chrome/gmbmikajjgmnabiglmofipeabaddhgne@2.1.2" for the Save to Google Drive Chrome extension version 2.1.2, "customers/my_customer/apps/android/com.google.android.apps.docs" for the Google Drive Android app's latest version.
    pub fn apps_android_get(&self, name: &str) -> CustomerAppAndroidGetCall<'a> {
        CustomerAppAndroidGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a specific app for a customer by its resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The app for which details are being queried. Examples: "customers/my_customer/apps/chrome/gmbmikajjgmnabiglmofipeabaddhgne@2.1.2" for the Save to Google Drive Chrome extension version 2.1.2, "customers/my_customer/apps/android/com.google.android.apps.docs" for the Google Drive Android app's latest version.
    pub fn apps_chrome_get(&self, name: &str) -> CustomerAppChromeGetCall<'a> {
        CustomerAppChromeGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a specific app for a customer by its resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The app for which details are being queried. Examples: "customers/my_customer/apps/chrome/gmbmikajjgmnabiglmofipeabaddhgne@2.1.2" for the Save to Google Drive Chrome extension version 2.1.2, "customers/my_customer/apps/android/com.google.android.apps.docs" for the Google Drive Android app's latest version.
    pub fn apps_web_get(&self, name: &str) -> CustomerAppWebGetCall<'a> {
        CustomerAppWebGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generate summary of app installation requests.
    /// 
    /// # Arguments
    ///
    /// * `customer` - Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    pub fn apps_count_chrome_app_requests(&self, customer: &str) -> CustomerAppCountChromeAppRequestCall<'a> {
        CustomerAppCountChromeAppRequestCall {
            hub: self.hub,
            _customer: customer.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _org_unit_id: Default::default(),
            _order_by: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generate report of installed Chrome versions.
    /// 
    /// # Arguments
    ///
    /// * `customer` - Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    pub fn reports_count_chrome_versions(&self, customer: &str) -> CustomerReportCountChromeVersionCall<'a> {
        CustomerReportCountChromeVersionCall {
            hub: self.hub,
            _customer: customer.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _org_unit_id: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generate report of app installations.
    /// 
    /// # Arguments
    ///
    /// * `customer` - Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    pub fn reports_count_installed_apps(&self, customer: &str) -> CustomerReportCountInstalledAppCall<'a> {
        CustomerReportCountInstalledAppCall {
            hub: self.hub,
            _customer: customer.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _org_unit_id: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generate report of devices that have a specified app installed.
    /// 
    /// # Arguments
    ///
    /// * `customer` - Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    pub fn reports_find_installed_app_devices(&self, customer: &str) -> CustomerReportFindInstalledAppDeviceCall<'a> {
        CustomerReportFindInstalledAppDeviceCall {
            hub: self.hub,
            _customer: customer.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _org_unit_id: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _app_type: Default::default(),
            _app_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all telemetry devices.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    pub fn telemetry_devices_list(&self, parent: &str) -> CustomerTelemetryDeviceListCall<'a> {
        CustomerTelemetryDeviceListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _read_mask: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Get a specific app for a customer by its resource name.
///
/// A builder for the *apps.android.get* method supported by a *customer* resource.
/// It is not used directly, but through a `CustomerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_chromemanagement1 as chromemanagement1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use chromemanagement1::{ChromeManagement, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().apps_android_get("name")
///              .doit().await;
/// # }
/// ```
pub struct CustomerAppAndroidGetCall<'a>
    where  {

    hub: &'a ChromeManagement<>,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for CustomerAppAndroidGetCall<'a> {}

impl<'a> CustomerAppAndroidGetCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleChromeManagementV1AppDetails)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "chromemanagement.customers.apps.android.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+name}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ChromeManagementAppdetailReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["name"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. The app for which details are being queried. Examples: "customers/my_customer/apps/chrome/gmbmikajjgmnabiglmofipeabaddhgne@2.1.2" for the Save to Google Drive Chrome extension version 2.1.2, "customers/my_customer/apps/android/com.google.android.apps.docs" for the Google Drive Android app's latest version.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> CustomerAppAndroidGetCall<'a> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CustomerAppAndroidGetCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> CustomerAppAndroidGetCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ChromeManagementAppdetailReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CustomerAppAndroidGetCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Get a specific app for a customer by its resource name.
///
/// A builder for the *apps.chrome.get* method supported by a *customer* resource.
/// It is not used directly, but through a `CustomerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_chromemanagement1 as chromemanagement1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use chromemanagement1::{ChromeManagement, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().apps_chrome_get("name")
///              .doit().await;
/// # }
/// ```
pub struct CustomerAppChromeGetCall<'a>
    where  {

    hub: &'a ChromeManagement<>,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for CustomerAppChromeGetCall<'a> {}

impl<'a> CustomerAppChromeGetCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleChromeManagementV1AppDetails)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "chromemanagement.customers.apps.chrome.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+name}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ChromeManagementAppdetailReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["name"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. The app for which details are being queried. Examples: "customers/my_customer/apps/chrome/gmbmikajjgmnabiglmofipeabaddhgne@2.1.2" for the Save to Google Drive Chrome extension version 2.1.2, "customers/my_customer/apps/android/com.google.android.apps.docs" for the Google Drive Android app's latest version.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> CustomerAppChromeGetCall<'a> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CustomerAppChromeGetCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> CustomerAppChromeGetCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ChromeManagementAppdetailReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CustomerAppChromeGetCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Get a specific app for a customer by its resource name.
///
/// A builder for the *apps.web.get* method supported by a *customer* resource.
/// It is not used directly, but through a `CustomerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_chromemanagement1 as chromemanagement1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use chromemanagement1::{ChromeManagement, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().apps_web_get("name")
///              .doit().await;
/// # }
/// ```
pub struct CustomerAppWebGetCall<'a>
    where  {

    hub: &'a ChromeManagement<>,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for CustomerAppWebGetCall<'a> {}

impl<'a> CustomerAppWebGetCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleChromeManagementV1AppDetails)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "chromemanagement.customers.apps.web.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+name}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ChromeManagementAppdetailReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["name"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. The app for which details are being queried. Examples: "customers/my_customer/apps/chrome/gmbmikajjgmnabiglmofipeabaddhgne@2.1.2" for the Save to Google Drive Chrome extension version 2.1.2, "customers/my_customer/apps/android/com.google.android.apps.docs" for the Google Drive Android app's latest version.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> CustomerAppWebGetCall<'a> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CustomerAppWebGetCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> CustomerAppWebGetCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ChromeManagementAppdetailReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CustomerAppWebGetCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Generate summary of app installation requests.
///
/// A builder for the *apps.countChromeAppRequests* method supported by a *customer* resource.
/// It is not used directly, but through a `CustomerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_chromemanagement1 as chromemanagement1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use chromemanagement1::{ChromeManagement, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().apps_count_chrome_app_requests("customer")
///              .page_token("sed")
///              .page_size(-2)
///              .org_unit_id("takimata")
///              .order_by("amet.")
///              .doit().await;
/// # }
/// ```
pub struct CustomerAppCountChromeAppRequestCall<'a>
    where  {

    hub: &'a ChromeManagement<>,
    _customer: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _org_unit_id: Option<String>,
    _order_by: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for CustomerAppCountChromeAppRequestCall<'a> {}

impl<'a> CustomerAppCountChromeAppRequestCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleChromeManagementV1CountChromeAppRequestsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "chromemanagement.customers.apps.countChromeAppRequests",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("customer", self._customer.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._org_unit_id {
            params.push(("orgUnitId", value.to_string()));
        }
        if let Some(value) = self._order_by {
            params.push(("orderBy", value.to_string()));
        }
        for &field in ["alt", "customer", "pageToken", "pageSize", "orgUnitId", "orderBy"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+customer}/apps:countChromeAppRequests";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ChromeManagementAppdetailReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+customer}", "customer")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["customer"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    ///
    /// Sets the *customer* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn customer(mut self, new_value: &str) -> CustomerAppCountChromeAppRequestCall<'a> {
        self._customer = new_value.to_string();
        self
    }
    /// Token to specify the page of the request to be returned.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CustomerAppCountChromeAppRequestCall<'a> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return. Maximum and default are 50, anything above will be coerced to 50.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> CustomerAppCountChromeAppRequestCall<'a> {
        self._page_size = Some(new_value);
        self
    }
    /// The ID of the organizational unit.
    ///
    /// Sets the *org unit id* query property to the given value.
    pub fn org_unit_id(mut self, new_value: &str) -> CustomerAppCountChromeAppRequestCall<'a> {
        self._org_unit_id = Some(new_value.to_string());
        self
    }
    /// Field used to order results. Supported fields: * request_count * latest_request_time
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> CustomerAppCountChromeAppRequestCall<'a> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CustomerAppCountChromeAppRequestCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> CustomerAppCountChromeAppRequestCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ChromeManagementAppdetailReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CustomerAppCountChromeAppRequestCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Generate report of installed Chrome versions.
///
/// A builder for the *reports.countChromeVersions* method supported by a *customer* resource.
/// It is not used directly, but through a `CustomerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_chromemanagement1 as chromemanagement1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use chromemanagement1::{ChromeManagement, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().reports_count_chrome_versions("customer")
///              .page_token("ipsum")
///              .page_size(-62)
///              .org_unit_id("Lorem")
///              .filter("gubergren")
///              .doit().await;
/// # }
/// ```
pub struct CustomerReportCountChromeVersionCall<'a>
    where  {

    hub: &'a ChromeManagement<>,
    _customer: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _org_unit_id: Option<String>,
    _filter: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for CustomerReportCountChromeVersionCall<'a> {}

impl<'a> CustomerReportCountChromeVersionCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleChromeManagementV1CountChromeVersionsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "chromemanagement.customers.reports.countChromeVersions",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("customer", self._customer.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._org_unit_id {
            params.push(("orgUnitId", value.to_string()));
        }
        if let Some(value) = self._filter {
            params.push(("filter", value.to_string()));
        }
        for &field in ["alt", "customer", "pageToken", "pageSize", "orgUnitId", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+customer}/reports:countChromeVersions";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ChromeManagementReportReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+customer}", "customer")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["customer"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    ///
    /// Sets the *customer* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn customer(mut self, new_value: &str) -> CustomerReportCountChromeVersionCall<'a> {
        self._customer = new_value.to_string();
        self
    }
    /// Token to specify the page of the request to be returned.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CustomerReportCountChromeVersionCall<'a> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return. Maximum and default are 100.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> CustomerReportCountChromeVersionCall<'a> {
        self._page_size = Some(new_value);
        self
    }
    /// The ID of the organizational unit.
    ///
    /// Sets the *org unit id* query property to the given value.
    pub fn org_unit_id(mut self, new_value: &str) -> CustomerReportCountChromeVersionCall<'a> {
        self._org_unit_id = Some(new_value.to_string());
        self
    }
    /// Query string to filter results, AND-separated fields in EBNF syntax. Note: OR operations are not supported in this filter. Supported filter fields: * last_active_date
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> CustomerReportCountChromeVersionCall<'a> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CustomerReportCountChromeVersionCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> CustomerReportCountChromeVersionCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ChromeManagementReportReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CustomerReportCountChromeVersionCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Generate report of app installations.
///
/// A builder for the *reports.countInstalledApps* method supported by a *customer* resource.
/// It is not used directly, but through a `CustomerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_chromemanagement1 as chromemanagement1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use chromemanagement1::{ChromeManagement, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().reports_count_installed_apps("customer")
///              .page_token("dolor")
///              .page_size(-17)
///              .org_unit_id("ipsum")
///              .order_by("invidunt")
///              .filter("amet")
///              .doit().await;
/// # }
/// ```
pub struct CustomerReportCountInstalledAppCall<'a>
    where  {

    hub: &'a ChromeManagement<>,
    _customer: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _org_unit_id: Option<String>,
    _order_by: Option<String>,
    _filter: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for CustomerReportCountInstalledAppCall<'a> {}

impl<'a> CustomerReportCountInstalledAppCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleChromeManagementV1CountInstalledAppsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "chromemanagement.customers.reports.countInstalledApps",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(8 + self._additional_params.len());
        params.push(("customer", self._customer.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._org_unit_id {
            params.push(("orgUnitId", value.to_string()));
        }
        if let Some(value) = self._order_by {
            params.push(("orderBy", value.to_string()));
        }
        if let Some(value) = self._filter {
            params.push(("filter", value.to_string()));
        }
        for &field in ["alt", "customer", "pageToken", "pageSize", "orgUnitId", "orderBy", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+customer}/reports:countInstalledApps";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ChromeManagementReportReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+customer}", "customer")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["customer"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    ///
    /// Sets the *customer* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn customer(mut self, new_value: &str) -> CustomerReportCountInstalledAppCall<'a> {
        self._customer = new_value.to_string();
        self
    }
    /// Token to specify the page of the request to be returned.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CustomerReportCountInstalledAppCall<'a> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return. Maximum and default are 100.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> CustomerReportCountInstalledAppCall<'a> {
        self._page_size = Some(new_value);
        self
    }
    /// The ID of the organizational unit.
    ///
    /// Sets the *org unit id* query property to the given value.
    pub fn org_unit_id(mut self, new_value: &str) -> CustomerReportCountInstalledAppCall<'a> {
        self._org_unit_id = Some(new_value.to_string());
        self
    }
    /// Field used to order results. Supported order by fields: * app_name * app_type * install_type * number_of_permissions * total_install_count
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> CustomerReportCountInstalledAppCall<'a> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// Query string to filter results, AND-separated fields in EBNF syntax. Note: OR operations are not supported in this filter. Supported filter fields: * app_name * app_type * install_type * number_of_permissions * total_install_count * latest_profile_active_date * permission_name
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> CustomerReportCountInstalledAppCall<'a> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CustomerReportCountInstalledAppCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> CustomerReportCountInstalledAppCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ChromeManagementReportReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CustomerReportCountInstalledAppCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Generate report of devices that have a specified app installed.
///
/// A builder for the *reports.findInstalledAppDevices* method supported by a *customer* resource.
/// It is not used directly, but through a `CustomerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_chromemanagement1 as chromemanagement1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use chromemanagement1::{ChromeManagement, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().reports_find_installed_app_devices("customer")
///              .page_token("ipsum")
///              .page_size(-93)
///              .org_unit_id("ut")
///              .order_by("gubergren")
///              .filter("rebum.")
///              .app_type("est")
///              .app_id("ipsum")
///              .doit().await;
/// # }
/// ```
pub struct CustomerReportFindInstalledAppDeviceCall<'a>
    where  {

    hub: &'a ChromeManagement<>,
    _customer: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _org_unit_id: Option<String>,
    _order_by: Option<String>,
    _filter: Option<String>,
    _app_type: Option<String>,
    _app_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for CustomerReportFindInstalledAppDeviceCall<'a> {}

impl<'a> CustomerReportFindInstalledAppDeviceCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleChromeManagementV1FindInstalledAppDevicesResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "chromemanagement.customers.reports.findInstalledAppDevices",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(10 + self._additional_params.len());
        params.push(("customer", self._customer.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._org_unit_id {
            params.push(("orgUnitId", value.to_string()));
        }
        if let Some(value) = self._order_by {
            params.push(("orderBy", value.to_string()));
        }
        if let Some(value) = self._filter {
            params.push(("filter", value.to_string()));
        }
        if let Some(value) = self._app_type {
            params.push(("appType", value.to_string()));
        }
        if let Some(value) = self._app_id {
            params.push(("appId", value.to_string()));
        }
        for &field in ["alt", "customer", "pageToken", "pageSize", "orgUnitId", "orderBy", "filter", "appType", "appId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+customer}/reports:findInstalledAppDevices";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ChromeManagementReportReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+customer}", "customer")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["customer"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    ///
    /// Sets the *customer* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn customer(mut self, new_value: &str) -> CustomerReportFindInstalledAppDeviceCall<'a> {
        self._customer = new_value.to_string();
        self
    }
    /// Token to specify the page of the request to be returned.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CustomerReportFindInstalledAppDeviceCall<'a> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return. Maximum and default are 100.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> CustomerReportFindInstalledAppDeviceCall<'a> {
        self._page_size = Some(new_value);
        self
    }
    /// The ID of the organizational unit.
    ///
    /// Sets the *org unit id* query property to the given value.
    pub fn org_unit_id(mut self, new_value: &str) -> CustomerReportFindInstalledAppDeviceCall<'a> {
        self._org_unit_id = Some(new_value.to_string());
        self
    }
    /// Field used to order results. Supported order by fields: * machine * device_id
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> CustomerReportFindInstalledAppDeviceCall<'a> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// Query string to filter results, AND-separated fields in EBNF syntax. Note: OR operations are not supported in this filter. Supported filter fields: * last_active_date
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> CustomerReportFindInstalledAppDeviceCall<'a> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// Type of the app.
    ///
    /// Sets the *app type* query property to the given value.
    pub fn app_type(mut self, new_value: &str) -> CustomerReportFindInstalledAppDeviceCall<'a> {
        self._app_type = Some(new_value.to_string());
        self
    }
    /// Unique identifier of the app. For Chrome apps and extensions, the 32-character id (e.g. ehoadneljpdggcbbknedodolkkjodefl). For Android apps, the package name (e.g. com.evernote).
    ///
    /// Sets the *app id* query property to the given value.
    pub fn app_id(mut self, new_value: &str) -> CustomerReportFindInstalledAppDeviceCall<'a> {
        self._app_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CustomerReportFindInstalledAppDeviceCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> CustomerReportFindInstalledAppDeviceCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ChromeManagementReportReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CustomerReportFindInstalledAppDeviceCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// List all telemetry devices.
///
/// A builder for the *telemetry.devices.list* method supported by a *customer* resource.
/// It is not used directly, but through a `CustomerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_chromemanagement1 as chromemanagement1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use chromemanagement1::{ChromeManagement, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().telemetry_devices_list("parent")
///              .read_mask("est")
///              .page_token("gubergren")
///              .page_size(-17)
///              .filter("dolor")
///              .doit().await;
/// # }
/// ```
pub struct CustomerTelemetryDeviceListCall<'a>
    where  {

    hub: &'a ChromeManagement<>,
    _parent: String,
    _read_mask: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _filter: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for CustomerTelemetryDeviceListCall<'a> {}

impl<'a> CustomerTelemetryDeviceListCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleChromeManagementV1ListTelemetryDevicesResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "chromemanagement.customers.telemetry.devices.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        if let Some(value) = self._read_mask {
            params.push(("readMask", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._filter {
            params.push(("filter", value.to_string()));
        }
        for &field in ["alt", "parent", "readMask", "pageToken", "pageSize", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}/telemetry/devices";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ChromeManagementTelemetryReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["parent"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> CustomerTelemetryDeviceListCall<'a> {
        self._parent = new_value.to_string();
        self
    }
    /// Required. Read mask to specify which fields to return.
    ///
    /// Sets the *read mask* query property to the given value.
    pub fn read_mask(mut self, new_value: &str) -> CustomerTelemetryDeviceListCall<'a> {
        self._read_mask = Some(new_value.to_string());
        self
    }
    /// Token to specify next page in the list.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CustomerTelemetryDeviceListCall<'a> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return. Default value is 100. Maximum value is 200.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> CustomerTelemetryDeviceListCall<'a> {
        self._page_size = Some(new_value);
        self
    }
    /// Optional. Only include resources that match the filter. Supported filter fields: - org_unit_id - serial_number 
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> CustomerTelemetryDeviceListCall<'a> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CustomerTelemetryDeviceListCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> CustomerTelemetryDeviceListCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ChromeManagementTelemetryReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CustomerTelemetryDeviceListCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


