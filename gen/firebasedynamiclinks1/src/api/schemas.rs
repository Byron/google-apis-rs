use super::*;
/// Tracking parameters supported by Dynamic Link.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyticsInfo {
    /// Google Play Campaign Measurements.
    #[serde(rename="googlePlayAnalytics")]
    
    pub google_play_analytics: Option<GooglePlayAnalytics>,
    /// iTunes Connect App Analytics.
    #[serde(rename="itunesConnectAnalytics")]
    
    pub itunes_connect_analytics: Option<ITunesConnectAnalytics>,
}

impl client::Part for AnalyticsInfo {}


/// Android related attributes to the Dynamic Link.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidInfo {
    /// Link to open on Android if the app is not installed.
    #[serde(rename="androidFallbackLink")]
    
    pub android_fallback_link: Option<String>,
    /// If specified, this overrides the ‘link’ parameter on Android.
    #[serde(rename="androidLink")]
    
    pub android_link: Option<String>,
    /// Minimum version code for the Android app. If the installed app’s version code is lower, then the user is taken to the Play Store.
    #[serde(rename="androidMinPackageVersionCode")]
    
    pub android_min_package_version_code: Option<String>,
    /// Android package name of the app.
    #[serde(rename="androidPackageName")]
    
    pub android_package_name: Option<String>,
}

impl client::Part for AndroidInfo {}


/// Request to create a managed Short Dynamic Link.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create managed short links](ManagedShortLinkCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateManagedShortLinkRequest {
    /// Information about the Dynamic Link to be shortened. [Learn more](https://firebase.google.com/docs/reference/dynamic-links/link-shortener).
    #[serde(rename="dynamicLinkInfo")]
    
    pub dynamic_link_info: Option<DynamicLinkInfo>,
    /// Full long Dynamic Link URL with desired query parameters specified. For example, "https://sample.app.goo.gl/?link=http://www.google.com&apn=com.sample", [Learn more](https://firebase.google.com/docs/reference/dynamic-links/link-shortener).
    #[serde(rename="longDynamicLink")]
    
    pub long_dynamic_link: Option<String>,
    /// Link name to associate with the link. It's used for marketer to identify manually-created links in the Firebase console (https://console.firebase.google.com/). Links must be named to be tracked.
    
    pub name: Option<String>,
    /// Google SDK version. Version takes the form "$major.$minor.$patch"
    #[serde(rename="sdkVersion")]
    
    pub sdk_version: Option<String>,
    /// Short Dynamic Link suffix. Optional.
    
    pub suffix: Option<Suffix>,
}

impl client::RequestValue for CreateManagedShortLinkRequest {}


/// Response to create a short Dynamic Link.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create managed short links](ManagedShortLinkCreateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateManagedShortLinkResponse {
    /// Short Dynamic Link value. e.g. https://abcd.app.goo.gl/wxyz
    #[serde(rename="managedShortLink")]
    
    pub managed_short_link: Option<ManagedShortLink>,
    /// Preview link to show the link flow chart. (debug info.)
    #[serde(rename="previewLink")]
    
    pub preview_link: Option<String>,
    /// Information about potential warnings on link creation.
    
    pub warning: Option<Vec<DynamicLinkWarning>>,
}

impl client::ResponseResult for CreateManagedShortLinkResponse {}


/// Request to create a short Dynamic Link.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create short links](ShortLinkCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateShortDynamicLinkRequest {
    /// Information about the Dynamic Link to be shortened. [Learn more](https://firebase.google.com/docs/reference/dynamic-links/link-shortener).
    #[serde(rename="dynamicLinkInfo")]
    
    pub dynamic_link_info: Option<DynamicLinkInfo>,
    /// Full long Dynamic Link URL with desired query parameters specified. For example, "https://sample.app.goo.gl/?link=http://www.google.com&apn=com.sample", [Learn more](https://firebase.google.com/docs/reference/dynamic-links/link-shortener).
    #[serde(rename="longDynamicLink")]
    
    pub long_dynamic_link: Option<String>,
    /// Google SDK version. Version takes the form "$major.$minor.$patch"
    #[serde(rename="sdkVersion")]
    
    pub sdk_version: Option<String>,
    /// Short Dynamic Link suffix. Optional.
    
    pub suffix: Option<Suffix>,
}

impl client::RequestValue for CreateShortDynamicLinkRequest {}


/// Response to create a short Dynamic Link.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create short links](ShortLinkCreateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateShortDynamicLinkResponse {
    /// Preview link to show the link flow chart. (debug info.)
    #[serde(rename="previewLink")]
    
    pub preview_link: Option<String>,
    /// Short Dynamic Link value. e.g. https://abcd.app.goo.gl/wxyz
    #[serde(rename="shortLink")]
    
    pub short_link: Option<String>,
    /// Information about potential warnings on link creation.
    
    pub warning: Option<Vec<DynamicLinkWarning>>,
}

impl client::ResponseResult for CreateShortDynamicLinkResponse {}


/// Desktop related attributes to the Dynamic Link.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DesktopInfo {
    /// Link to open on desktop.
    #[serde(rename="desktopFallbackLink")]
    
    pub desktop_fallback_link: Option<String>,
}

impl client::Part for DesktopInfo {}


/// Signals associated with the device making the request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// Device model name.
    #[serde(rename="deviceModelName")]
    
    pub device_model_name: Option<String>,
    /// Device language code setting.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Device language code setting obtained by executing JavaScript code in WebView.
    #[serde(rename="languageCodeFromWebview")]
    
    pub language_code_from_webview: Option<String>,
    /// Device language code raw setting. iOS does returns language code in different format than iOS WebView. For example WebView returns en_US, but iOS returns en-US. Field below will return raw value returned by iOS.
    #[serde(rename="languageCodeRaw")]
    
    pub language_code_raw: Option<String>,
    /// Device display resolution height.
    #[serde(rename="screenResolutionHeight")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub screen_resolution_height: Option<i64>,
    /// Device display resolution width.
    #[serde(rename="screenResolutionWidth")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub screen_resolution_width: Option<i64>,
    /// Device timezone setting.
    
    pub timezone: Option<String>,
}

impl client::Part for DeviceInfo {}


/// Dynamic Link event stat.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicLinkEventStat {
    /// The number of times this event occurred.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// Link event.
    
    pub event: Option<DynamicLinkEventStatEventEnum>,
    /// Requested platform.
    
    pub platform: Option<DynamicLinkEventStatPlatformEnum>,
}

impl client::Part for DynamicLinkEventStat {}


/// Information about a Dynamic Link.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicLinkInfo {
    /// Parameters used for tracking. See all tracking parameters in the [documentation](https://firebase.google.com/docs/dynamic-links/create-manually).
    #[serde(rename="analyticsInfo")]
    
    pub analytics_info: Option<AnalyticsInfo>,
    /// Android related information. See Android related parameters in the [documentation](https://firebase.google.com/docs/dynamic-links/create-manually).
    #[serde(rename="androidInfo")]
    
    pub android_info: Option<AndroidInfo>,
    /// Desktop related information. See desktop related parameters in the [documentation](https://firebase.google.com/docs/dynamic-links/create-manually).
    #[serde(rename="desktopInfo")]
    
    pub desktop_info: Option<DesktopInfo>,
    /// E.g. https://maps.app.goo.gl, https://maps.page.link, https://g.co/maps More examples can be found in description of getNormalizedUriPrefix in j/c/g/firebase/dynamiclinks/uri/DdlDomain.java Will fallback to dynamic_link_domain is this field is missing
    #[serde(rename="domainUriPrefix")]
    
    pub domain_uri_prefix: Option<String>,
    /// Dynamic Links domain that the project owns, e.g. abcd.app.goo.gl [Learn more](https://firebase.google.com/docs/dynamic-links/android/receive) on how to set up Dynamic Link domain associated with your Firebase project. Required if missing domain_uri_prefix.
    #[serde(rename="dynamicLinkDomain")]
    
    pub dynamic_link_domain: Option<String>,
    /// iOS related information. See iOS related parameters in the [documentation](https://firebase.google.com/docs/dynamic-links/create-manually).
    #[serde(rename="iosInfo")]
    
    pub ios_info: Option<IosInfo>,
    /// The link your app will open, You can specify any URL your app can handle. This link must be a well-formatted URL, be properly URL-encoded, and use the HTTP or HTTPS scheme. See 'link' parameters in the [documentation](https://firebase.google.com/docs/dynamic-links/create-manually). Required.
    
    pub link: Option<String>,
    /// Information of navigation behavior of a Firebase Dynamic Links.
    #[serde(rename="navigationInfo")]
    
    pub navigation_info: Option<NavigationInfo>,
    /// Parameters for social meta tag params. Used to set meta tag data for link previews on social sites.
    #[serde(rename="socialMetaTagInfo")]
    
    pub social_meta_tag_info: Option<SocialMetaTagInfo>,
}

impl client::Part for DynamicLinkInfo {}


/// Analytics stats of a Dynamic Link for a given timeframe.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get link stats](MethodGetLinkStatCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicLinkStats {
    /// Dynamic Link event stats.
    #[serde(rename="linkEventStats")]
    
    pub link_event_stats: Option<Vec<DynamicLinkEventStat>>,
}

impl client::ResponseResult for DynamicLinkStats {}


/// Dynamic Links warning messages.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicLinkWarning {
    /// The warning code.
    #[serde(rename="warningCode")]
    
    pub warning_code: Option<DynamicLinkWarningWarningCodeEnum>,
    /// The document describing the warning, and helps resolve.
    #[serde(rename="warningDocumentLink")]
    
    pub warning_document_link: Option<String>,
    /// The warning message to help developers improve their requests.
    #[serde(rename="warningMessage")]
    
    pub warning_message: Option<String>,
}

impl client::Part for DynamicLinkWarning {}


/// Request for iSDK to execute strong match flow for post-install attribution. This is meant for iOS requests only. Requests from other platforms will not be honored.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [install attribution](MethodInstallAttributionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetIosPostInstallAttributionRequest {
    /// App installation epoch time (https://en.wikipedia.org/wiki/Unix_time). This is a client signal for a more accurate weak match.
    #[serde(rename="appInstallationTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub app_installation_time: Option<i64>,
    /// APP bundle ID.
    #[serde(rename="bundleId")]
    
    pub bundle_id: Option<String>,
    /// Device information.
    
    pub device: Option<DeviceInfo>,
    /// iOS version, ie: 9.3.5. Consider adding "build".
    #[serde(rename="iosVersion")]
    
    pub ios_version: Option<String>,
    /// App post install attribution retrieval information. Disambiguates mechanism (iSDK or developer invoked) to retrieve payload from clicked link.
    #[serde(rename="retrievalMethod")]
    
    pub retrieval_method: Option<GetIosPostInstallAttributionRequestRetrievalMethodEnum>,
    /// Google SDK version. Version takes the form "$major.$minor.$patch"
    #[serde(rename="sdkVersion")]
    
    pub sdk_version: Option<String>,
    /// Possible unique matched link that server need to check before performing fingerprint match. If passed link is short server need to expand the link. If link is long server need to vslidate the link.
    #[serde(rename="uniqueMatchLinkToCheck")]
    
    pub unique_match_link_to_check: Option<String>,
    /// Strong match page information. Disambiguates between default UI and custom page to present when strong match succeeds/fails to find cookie.
    #[serde(rename="visualStyle")]
    
    pub visual_style: Option<GetIosPostInstallAttributionRequestVisualStyleEnum>,
}

impl client::RequestValue for GetIosPostInstallAttributionRequest {}


/// Response for iSDK to execute strong match flow for post-install attribution.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [install attribution](MethodInstallAttributionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetIosPostInstallAttributionResponse {
    /// The minimum version for app, specified by dev through ?imv= parameter. Return to iSDK to allow app to evaluate if current version meets this.
    #[serde(rename="appMinimumVersion")]
    
    pub app_minimum_version: Option<String>,
    /// The confidence of the returned attribution.
    #[serde(rename="attributionConfidence")]
    
    pub attribution_confidence: Option<GetIosPostInstallAttributionResponseAttributionConfidenceEnum>,
    /// The deep-link attributed post-install via one of several techniques (fingerprint, copy unique).
    #[serde(rename="deepLink")]
    
    pub deep_link: Option<String>,
    /// User-agent specific custom-scheme URIs for iSDK to open. This will be set according to the user-agent tha the click was originally made in. There is no Safari-equivalent custom-scheme open URLs. ie: googlechrome://www.example.com ie: firefox://open-url?url=http://www.example.com ie: opera-http://example.com
    #[serde(rename="externalBrowserDestinationLink")]
    
    pub external_browser_destination_link: Option<String>,
    /// The link to navigate to update the app if min version is not met. This is either (in order): 1) fallback link (from ?ifl= parameter, if specified by developer) or 2) AppStore URL (from ?isi= parameter, if specified), or 3) the payload link (from required link= parameter).
    #[serde(rename="fallbackLink")]
    
    pub fallback_link: Option<String>,
    /// Invitation ID attributed post-install via one of several techniques (fingerprint, copy unique).
    #[serde(rename="invitationId")]
    
    pub invitation_id: Option<String>,
    /// Instruction for iSDK to attemmpt to perform strong match. For instance, if browser does not support/allow cookie or outside of support browsers, this will be false.
    #[serde(rename="isStrongMatchExecutable")]
    
    pub is_strong_match_executable: Option<bool>,
    /// Describes why match failed, ie: "discarded due to low confidence". This message will be publicly visible.
    #[serde(rename="matchMessage")]
    
    pub match_message: Option<String>,
    /// Which IP version the request was made from.
    #[serde(rename="requestIpVersion")]
    
    pub request_ip_version: Option<GetIosPostInstallAttributionResponseRequestIpVersionEnum>,
    /// Entire FDL (short or long) attributed post-install via one of several techniques (fingerprint, copy unique).
    #[serde(rename="requestedLink")]
    
    pub requested_link: Option<String>,
    /// The entire FDL, expanded from a short link. It is the same as the requested_link, if it is long. Parameters from this should not be used directly (ie: server can default utm_[campaign|medium|source] to a value when requested_link lack them, server determine the best fallback_link when requested_link specifies >1 fallback links).
    #[serde(rename="resolvedLink")]
    
    pub resolved_link: Option<String>,
    /// Scion campaign value to be propagated by iSDK to Scion at post-install.
    #[serde(rename="utmCampaign")]
    
    pub utm_campaign: Option<String>,
    /// Scion content value to be propagated by iSDK to Scion at app-reopen.
    #[serde(rename="utmContent")]
    
    pub utm_content: Option<String>,
    /// Scion medium value to be propagated by iSDK to Scion at post-install.
    #[serde(rename="utmMedium")]
    
    pub utm_medium: Option<String>,
    /// Scion source value to be propagated by iSDK to Scion at post-install.
    #[serde(rename="utmSource")]
    
    pub utm_source: Option<String>,
    /// Scion term value to be propagated by iSDK to Scion at app-reopen.
    #[serde(rename="utmTerm")]
    
    pub utm_term: Option<String>,
}

impl client::ResponseResult for GetIosPostInstallAttributionResponse {}


/// Request for iSDK to get reopen attribution for app universal link open deeplinking. This endpoint is meant for only iOS requests.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reopen attribution](MethodReopenAttributionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetIosReopenAttributionRequest {
    /// APP bundle ID.
    #[serde(rename="bundleId")]
    
    pub bundle_id: Option<String>,
    /// FDL link to be verified from an app universal link open. The FDL link can be one of: 1) short FDL. e.g. .page.link/, or 2) long FDL. e.g. .page.link/?{query params}, or 3) Invite FDL. e.g. .page.link/i/
    #[serde(rename="requestedLink")]
    
    pub requested_link: Option<String>,
    /// Google SDK version. Version takes the form "$major.$minor.$patch"
    #[serde(rename="sdkVersion")]
    
    pub sdk_version: Option<String>,
}

impl client::RequestValue for GetIosReopenAttributionRequest {}


/// Response for iSDK to get reopen attribution for app universal link open deeplinking. This endpoint is meant for only iOS requests.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reopen attribution](MethodReopenAttributionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetIosReopenAttributionResponse {
    /// The deep-link attributed the app universal link open. For both regular FDL links and invite FDL links.
    #[serde(rename="deepLink")]
    
    pub deep_link: Option<String>,
    /// Optional invitation ID, for only invite typed requested FDL links.
    #[serde(rename="invitationId")]
    
    pub invitation_id: Option<String>,
    /// FDL input value of the "&imv=" parameter, minimum app version to be returned to Google Firebase SDK running on iOS-9.
    #[serde(rename="iosMinAppVersion")]
    
    pub ios_min_app_version: Option<String>,
    /// The entire FDL, expanded from a short link. It is the same as the requested_link, if it is long.
    #[serde(rename="resolvedLink")]
    
    pub resolved_link: Option<String>,
    /// Scion campaign value to be propagated by iSDK to Scion at app-reopen.
    #[serde(rename="utmCampaign")]
    
    pub utm_campaign: Option<String>,
    /// Scion content value to be propagated by iSDK to Scion at app-reopen.
    #[serde(rename="utmContent")]
    
    pub utm_content: Option<String>,
    /// Scion medium value to be propagated by iSDK to Scion at app-reopen.
    #[serde(rename="utmMedium")]
    
    pub utm_medium: Option<String>,
    /// Scion source value to be propagated by iSDK to Scion at app-reopen.
    #[serde(rename="utmSource")]
    
    pub utm_source: Option<String>,
    /// Scion term value to be propagated by iSDK to Scion at app-reopen.
    #[serde(rename="utmTerm")]
    
    pub utm_term: Option<String>,
}

impl client::ResponseResult for GetIosReopenAttributionResponse {}


/// Parameters for Google Play Campaign Measurements. [Learn more](https://developers.google.com/analytics/devguides/collection/android/v4/campaigns#campaign-params)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePlayAnalytics {
    /// Deprecated; FDL SDK does not process nor log it.
    
    pub gclid: Option<String>,
    /// Campaign name; used for keyword analysis to identify a specific product promotion or strategic campaign.
    #[serde(rename="utmCampaign")]
    
    pub utm_campaign: Option<String>,
    /// Campaign content; used for A/B testing and content-targeted ads to differentiate ads or links that point to the same URL.
    #[serde(rename="utmContent")]
    
    pub utm_content: Option<String>,
    /// Campaign medium; used to identify a medium such as email or cost-per-click.
    #[serde(rename="utmMedium")]
    
    pub utm_medium: Option<String>,
    /// Campaign source; used to identify a search engine, newsletter, or other source.
    #[serde(rename="utmSource")]
    
    pub utm_source: Option<String>,
    /// Campaign term; used with paid search to supply the keywords for ads.
    #[serde(rename="utmTerm")]
    
    pub utm_term: Option<String>,
}

impl client::Part for GooglePlayAnalytics {}


/// Parameters for iTunes Connect App Analytics.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ITunesConnectAnalytics {
    /// Affiliate token used to create affiliate-coded links.
    
    pub at: Option<String>,
    /// Campaign text that developers can optionally add to any link in order to track sales from a specific marketing campaign.
    
    pub ct: Option<String>,
    /// iTune media types, including music, podcasts, audiobooks and so on.
    
    pub mt: Option<String>,
    /// Provider token that enables analytics for Dynamic Links from within iTunes Connect.
    
    pub pt: Option<String>,
}

impl client::Part for ITunesConnectAnalytics {}


/// iOS related attributes to the Dynamic Link..
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IosInfo {
    /// iOS App Store ID.
    #[serde(rename="iosAppStoreId")]
    
    pub ios_app_store_id: Option<String>,
    /// iOS bundle ID of the app.
    #[serde(rename="iosBundleId")]
    
    pub ios_bundle_id: Option<String>,
    /// Custom (destination) scheme to use for iOS. By default, we’ll use the bundle ID as the custom scheme. Developer can override this behavior using this param.
    #[serde(rename="iosCustomScheme")]
    
    pub ios_custom_scheme: Option<String>,
    /// Link to open on iOS if the app is not installed.
    #[serde(rename="iosFallbackLink")]
    
    pub ios_fallback_link: Option<String>,
    /// iPad bundle ID of the app.
    #[serde(rename="iosIpadBundleId")]
    
    pub ios_ipad_bundle_id: Option<String>,
    /// If specified, this overrides the ios_fallback_link value on iPads.
    #[serde(rename="iosIpadFallbackLink")]
    
    pub ios_ipad_fallback_link: Option<String>,
    /// iOS minimum version.
    #[serde(rename="iosMinimumVersion")]
    
    pub ios_minimum_version: Option<String>,
}

impl client::Part for IosInfo {}


/// Managed Short Link.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create managed short links](ManagedShortLinkCreateCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedShortLink {
    /// Creation timestamp of the short link.
    #[serde(rename="creationTime")]
    
    pub creation_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Attributes that have been flagged about this short url.
    #[serde(rename="flaggedAttribute")]
    
    pub flagged_attribute: Option<Vec<ManagedShortLinkFlaggedAttributeEnum>>,
    /// Full Dyamic Link info
    
    pub info: Option<DynamicLinkInfo>,
    /// Short durable link url, for example, "https://sample.app.goo.gl/xyz123". Required.
    
    pub link: Option<String>,
    /// Link name defined by the creator. Required.
    #[serde(rename="linkName")]
    
    pub link_name: Option<String>,
    /// Visibility status of link.
    
    pub visibility: Option<ManagedShortLinkVisibilityEnum>,
}

impl client::Resource for ManagedShortLink {}


/// Information of navigation behavior.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NavigationInfo {
    /// If this option is on, FDL click will be forced to redirect rather than show an interstitial page.
    #[serde(rename="enableForcedRedirect")]
    
    pub enable_forced_redirect: Option<bool>,
}

impl client::Part for NavigationInfo {}


/// Parameters for social meta tag params. Used to set meta tag data for link previews on social sites.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SocialMetaTagInfo {
    /// A short description of the link. Optional.
    #[serde(rename="socialDescription")]
    
    pub social_description: Option<String>,
    /// An image url string. Optional.
    #[serde(rename="socialImageLink")]
    
    pub social_image_link: Option<String>,
    /// Title to be displayed. Optional.
    #[serde(rename="socialTitle")]
    
    pub social_title: Option<String>,
}

impl client::Part for SocialMetaTagInfo {}


/// Short Dynamic Link suffix.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Suffix {
    /// Only applies to Option.CUSTOM.
    #[serde(rename="customSuffix")]
    
    pub custom_suffix: Option<String>,
    /// Suffix option.
    
    pub option: Option<SuffixOptionEnum>,
}

impl client::Part for Suffix {}


