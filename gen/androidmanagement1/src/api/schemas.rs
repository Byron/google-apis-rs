use super::*;
/// Security policies set to secure values by default. To maintain the security posture of a device, we don't recommend overriding any of the default values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdvancedSecurityOverrides {
    /// Controls Common Criteria Mode—security standards defined in the Common Criteria for Information Technology Security Evaluation (https://www.commoncriteriaportal.org/) (CC). Enabling Common Criteria Mode increases certain security components on a device, including AES-GCM encryption of Bluetooth Long Term Keys, and Wi-Fi configuration stores.Warning: Common Criteria Mode enforces a strict security model typically only required for IT products used in national security systems and other highly sensitive organizations. Standard device use may be affected. Only enabled if required.
    #[serde(rename="commonCriteriaMode")]
    
    pub common_criteria_mode: Option<AdvancedSecurityOverrideCommonCriteriaModeEnum>,
    /// Controls access to developer settings: developer options and safe boot. Replaces safeBootDisabled (deprecated) and debuggingFeaturesAllowed (deprecated).
    #[serde(rename="developerSettings")]
    
    pub developer_settings: Option<AdvancedSecurityOverrideDeveloperSettingsEnum>,
    /// Whether Google Play Protect verification (https://support.google.com/accounts/answer/2812853) is enforced. Replaces ensureVerifyAppsEnabled (deprecated).
    #[serde(rename="googlePlayProtectVerifyApps")]
    
    pub google_play_protect_verify_apps: Option<AdvancedSecurityOverrideGooglePlayProtectVerifyAppsEnum>,
    /// Personal apps that can read work profile notifications using a NotificationListenerService (https://developer.android.com/reference/android/service/notification/NotificationListenerService). By default, no personal apps (aside from system apps) can read work notifications. Each value in the list must be a package name.
    #[serde(rename="personalAppsThatCanReadWorkNotifications")]
    
    pub personal_apps_that_can_read_work_notifications: Option<Vec<String>>,
    /// The policy for untrusted apps (apps from unknown sources) enforced on the device. Replaces install_unknown_sources_allowed (deprecated).
    #[serde(rename="untrustedAppsPolicy")]
    
    pub untrusted_apps_policy: Option<AdvancedSecurityOverrideUntrustedAppsPolicyEnum>,
}

impl client::Part for AdvancedSecurityOverrides {}


/// Configuration for an always-on VPN connection.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AlwaysOnVpnPackage {
    /// Disallows networking when the VPN is not connected.
    #[serde(rename="lockdownEnabled")]
    
    pub lockdown_enabled: Option<bool>,
    /// The package name of the VPN app.
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
}

impl client::Part for AlwaysOnVpnPackage {}


/// A compliance rule condition which is satisfied if the Android Framework API level on the device doesn't meet a minimum requirement. There can only be one rule with this type of condition per policy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApiLevelCondition {
    /// The minimum desired Android Framework API level. If the device doesn't meet the minimum requirement, this condition is satisfied. Must be greater than zero.
    #[serde(rename="minApiLevel")]
    
    pub min_api_level: Option<i32>,
}

impl client::Part for ApiLevelCondition {}


/// Id to name association of a app track.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppTrackInfo {
    /// The track name associated with the trackId, set in the Play Console. The name is modifiable from Play Console.
    #[serde(rename="trackAlias")]
    
    pub track_alias: Option<String>,
    /// The unmodifiable unique track identifier, taken from the releaseTrackId in the URL of the Play Console page that displays the app’s track information.
    #[serde(rename="trackId")]
    
    pub track_id: Option<String>,
}

impl client::Part for AppTrackInfo {}


/// This represents a single version of the app.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppVersion {
    /// If the value is True, it indicates that this version is a production track.
    
    pub production: Option<bool>,
    /// Track identifiers that the app version is published in. This does not include the production track (see production instead).
    #[serde(rename="trackIds")]
    
    pub track_ids: Option<Vec<String>>,
    /// Unique increasing identifier for the app version.
    #[serde(rename="versionCode")]
    
    pub version_code: Option<i32>,
    /// The string used in the Play store by the app developer to identify the version. The string is not necessarily unique or localized (for example, the string could be "1.4").
    #[serde(rename="versionString")]
    
    pub version_string: Option<String>,
}

impl client::Part for AppVersion {}


/// Information about an app.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [applications get enterprises](EnterpriseApplicationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Application {
    /// Whether this app is free, free with in-app purchases, or paid. If the pricing is unspecified, this means the app is not generally available anymore (even though it might still be available to people who own it).
    #[serde(rename="appPricing")]
    
    pub app_pricing: Option<ApplicationAppPricingEnum>,
    /// Application tracks visible to the enterprise.
    #[serde(rename="appTracks")]
    
    pub app_tracks: Option<Vec<AppTrackInfo>>,
    /// Versions currently available for this app.
    #[serde(rename="appVersions")]
    
    pub app_versions: Option<Vec<AppVersion>>,
    /// The name of the author of the apps (for example, the app developer).
    
    pub author: Option<String>,
    /// The countries which this app is available in as per ISO 3166-1 alpha-2.
    #[serde(rename="availableCountries")]
    
    pub available_countries: Option<Vec<String>>,
    /// The app category (e.g. RACING, SOCIAL, etc.)
    
    pub category: Option<String>,
    /// The content rating for this app.
    #[serde(rename="contentRating")]
    
    pub content_rating: Option<ApplicationContentRatingEnum>,
    /// The localized promotional description, if available.
    
    pub description: Option<String>,
    /// How and to whom the package is made available.
    #[serde(rename="distributionChannel")]
    
    pub distribution_channel: Option<ApplicationDistributionChannelEnum>,
    /// Noteworthy features (if any) of this app.
    
    pub features: Option<Vec<ApplicationFeaturesEnum>>,
    /// Full app description, if available.
    #[serde(rename="fullDescription")]
    
    pub full_description: Option<String>,
    /// A link to an image that can be used as an icon for the app. This image is suitable for use up to a pixel size of 512 x 512.
    #[serde(rename="iconUrl")]
    
    pub icon_url: Option<String>,
    /// The set of managed properties available to be pre-configured for the app.
    #[serde(rename="managedProperties")]
    
    pub managed_properties: Option<Vec<ManagedProperty>>,
    /// The minimum Android SDK necessary to run the app.
    #[serde(rename="minAndroidSdkVersion")]
    
    pub min_android_sdk_version: Option<i32>,
    /// The name of the app in the form enterprises/{enterprise}/applications/{package_name}.
    
    pub name: Option<String>,
    /// The permissions required by the app.
    
    pub permissions: Option<Vec<ApplicationPermission>>,
    /// A link to the (consumer) Google Play details page for the app.
    #[serde(rename="playStoreUrl")]
    
    pub play_store_url: Option<String>,
    /// A localised description of the recent changes made to the app.
    #[serde(rename="recentChanges")]
    
    pub recent_changes: Option<String>,
    /// A list of screenshot links representing the app.
    #[serde(rename="screenshotUrls")]
    
    pub screenshot_urls: Option<Vec<String>>,
    /// A link to a smaller image that can be used as an icon for the app. This image is suitable for use up to a pixel size of 128 x 128.
    #[serde(rename="smallIconUrl")]
    
    pub small_icon_url: Option<String>,
    /// The title of the app. Localized.
    
    pub title: Option<String>,
    /// Output only. The approximate time (within 7 days) the app was last published.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for Application {}


/// An app-related event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApplicationEvent {
    /// The creation time of the event.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// App event type.
    #[serde(rename="eventType")]
    
    pub event_type: Option<ApplicationEventEventTypeEnum>,
}

impl client::Part for ApplicationEvent {}


/// A permission required by the app.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApplicationPermission {
    /// A longer description of the permission, providing more detail on what it affects. Localized.
    
    pub description: Option<String>,
    /// The name of the permission. Localized.
    
    pub name: Option<String>,
    /// An opaque string uniquely identifying the permission. Not localized.
    #[serde(rename="permissionId")]
    
    pub permission_id: Option<String>,
}

impl client::Part for ApplicationPermission {}


/// Policy for an individual app.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApplicationPolicy {
    /// List of the app’s track IDs that a device belonging to the enterprise can access. If the list contains multiple track IDs, devices receive the latest version among all accessible tracks. If the list contains no track IDs, devices only have access to the app’s production track. More details about each track are available in AppTrackInfo.
    #[serde(rename="accessibleTrackIds")]
    
    pub accessible_track_ids: Option<Vec<String>>,
    /// Specifies whether the app is allowed networking when the VPN is not connected and alwaysOnVpnPackage.lockdownEnabled is enabled. If set to VPN_LOCKDOWN_ENFORCED, the app is not allowed networking, and if set to VPN_LOCKDOWN_EXEMPTION, the app is allowed networking. Only supported on devices running Android 10 and above. If this is not supported by the device, the device will contain a NonComplianceDetail with non_compliance_reason set to API_LEVEL and a fieldPath. If this is not applicable to the app, the device will contain a NonComplianceDetail with non_compliance_reason set to UNSUPPORTED and a fieldPath. The fieldPath is set to applications[i].alwaysOnVpnLockdownExemption, where i is the index of the package in the applications policy.
    #[serde(rename="alwaysOnVpnLockdownExemption")]
    
    pub always_on_vpn_lockdown_exemption: Option<ApplicationPolicyAlwaysOnVpnLockdownExemptionEnum>,
    /// Controls the auto-update mode for the app.
    #[serde(rename="autoUpdateMode")]
    
    pub auto_update_mode: Option<ApplicationPolicyAutoUpdateModeEnum>,
    /// Controls whether the app can communicate with itself across a device’s work and personal profiles, subject to user consent.
    #[serde(rename="connectedWorkAndPersonalApp")]
    
    pub connected_work_and_personal_app: Option<ApplicationPolicyConnectedWorkAndPersonalAppEnum>,
    /// The default policy for all permissions requested by the app. If specified, this overrides the policy-level default_permission_policy which applies to all apps. It does not override the permission_grants which applies to all apps.
    #[serde(rename="defaultPermissionPolicy")]
    
    pub default_permission_policy: Option<ApplicationPolicyDefaultPermissionPolicyEnum>,
    /// The scopes delegated to the app from Android Device Policy.
    #[serde(rename="delegatedScopes")]
    
    pub delegated_scopes: Option<Vec<ApplicationPolicyDelegatedScopesEnum>>,
    /// Whether the app is disabled. When disabled, the app data is still preserved.
    
    pub disabled: Option<bool>,
    /// Configuration to enable this app as an extension app, with the capability of interacting with Android Device Policy offline.This field can be set for at most one app.
    #[serde(rename="extensionConfig")]
    
    pub extension_config: Option<ExtensionConfig>,
    /// The type of installation to perform.
    #[serde(rename="installType")]
    
    pub install_type: Option<ApplicationPolicyInstallTypeEnum>,
    /// Whether the app is allowed to lock itself in full-screen mode. DEPRECATED. Use InstallType KIOSK or kioskCustomLauncherEnabled to configure a dedicated device.
    #[serde(rename="lockTaskAllowed")]
    
    pub lock_task_allowed: Option<bool>,
    /// Managed configuration applied to the app. The format for the configuration is dictated by the ManagedProperty values supported by the app. Each field name in the managed configuration must match the key field of the ManagedProperty. The field value must be compatible with the type of the ManagedProperty: *type* *JSON value* BOOL true or false STRING string INTEGER number CHOICE string MULTISELECT array of strings HIDDEN string BUNDLE_ARRAY array of objects 
    #[serde(rename="managedConfiguration")]
    
    pub managed_configuration: Option<HashMap<String, json::Value>>,
    /// The managed configurations template for the app, saved from the managed configurations iframe. This field is ignored if managed_configuration is set.
    #[serde(rename="managedConfigurationTemplate")]
    
    pub managed_configuration_template: Option<ManagedConfigurationTemplate>,
    /// The minimum version of the app that runs on the device. If set, the device attempts to update the app to at least this version code. If the app is not up-to-date, the device will contain a NonComplianceDetail with non_compliance_reason set to APP_NOT_UPDATED. The app must already be published to Google Play with a version code greater than or equal to this value. At most 20 apps may specify a minimum version code per policy.
    #[serde(rename="minimumVersionCode")]
    
    pub minimum_version_code: Option<i32>,
    /// The package name of the app. For example, com.google.android.youtube for the YouTube app.
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
    /// Explicit permission grants or denials for the app. These values override the default_permission_policy and permission_grants which apply to all apps.
    #[serde(rename="permissionGrants")]
    
    pub permission_grants: Option<Vec<PermissionGrant>>,
    /// Specifies whether the app installed in the work profile is allowed to add widgets to the home screen.
    #[serde(rename="workProfileWidgets")]
    
    pub work_profile_widgets: Option<ApplicationPolicyWorkProfileWidgetsEnum>,
}

impl client::Part for ApplicationPolicy {}


/// Information reported about an installed app.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApplicationReport {
    /// The source of the package.
    #[serde(rename="applicationSource")]
    
    pub application_source: Option<ApplicationReportApplicationSourceEnum>,
    /// The display name of the app.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The list of app events which have occurred in the last 30 hours.
    
    pub events: Option<Vec<ApplicationEvent>>,
    /// The package name of the app that installed this app.
    #[serde(rename="installerPackageName")]
    
    pub installer_package_name: Option<String>,
    /// List of keyed app states reported by the app.
    #[serde(rename="keyedAppStates")]
    
    pub keyed_app_states: Option<Vec<KeyedAppState>>,
    /// Package name of the app.
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
    /// The SHA-256 hash of the app's APK file, which can be used to verify the app hasn't been modified. Each byte of the hash value is represented as a two-digit hexadecimal number.
    #[serde(rename="packageSha256Hash")]
    
    pub package_sha256_hash: Option<String>,
    /// The SHA-1 hash of each android.content.pm.Signature (https://developer.android.com/reference/android/content/pm/Signature.html) associated with the app package. Each byte of each hash value is represented as a two-digit hexadecimal number.
    #[serde(rename="signingKeyCertFingerprints")]
    
    pub signing_key_cert_fingerprints: Option<Vec<String>>,
    /// Application state.
    
    pub state: Option<ApplicationReportStateEnum>,
    /// The app version code, which can be used to determine whether one version is more recent than another.
    #[serde(rename="versionCode")]
    
    pub version_code: Option<i32>,
    /// The app version as displayed to the user.
    #[serde(rename="versionName")]
    
    pub version_name: Option<String>,
}

impl client::Part for ApplicationReport {}


/// Settings controlling the behavior of application reports.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApplicationReportingSettings {
    /// Whether removed apps are included in application reports.
    #[serde(rename="includeRemovedApps")]
    
    pub include_removed_apps: Option<bool>,
}

impl client::Part for ApplicationReportingSettings {}


/// An action to block access to apps and data on a fully managed device or in a work profile. This action also triggers a device or work profile to displays a user-facing notification with information (where possible) on how to correct the compliance issue. Note: wipeAction must also be specified.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BlockAction {
    /// Number of days the policy is non-compliant before the device or work profile is blocked. To block access immediately, set to 0. blockAfterDays must be less than wipeAfterDays.
    #[serde(rename="blockAfterDays")]
    
    pub block_after_days: Option<i32>,
    /// Specifies the scope of this BlockAction. Only applicable to devices that are company-owned.
    #[serde(rename="blockScope")]
    
    pub block_scope: Option<BlockActionBlockScopeEnum>,
}

impl client::Part for BlockAction {}


/// Controls apps' access to private keys. The rule determines which private key, if any, Android Device Policy grants to the specified app. Access is granted either when the app calls KeyChain.choosePrivateKeyAlias (https://developer.android.com/reference/android/security/KeyChain#choosePrivateKeyAlias%28android.app.Activity,%20android.security.KeyChainAliasCallback,%20java.lang.String[],%20java.security.Principal[],%20java.lang.String,%20int,%20java.lang.String%29) (or any overloads) to request a private key alias for a given URL, or for rules that are not URL-specific (that is, if urlPattern is not set, or set to the empty string or .*) on Android 11 and above, directly so that the app can call KeyChain.getPrivateKey (https://developer.android.com/reference/android/security/KeyChain#getPrivateKey%28android.content.Context,%20java.lang.String%29), without first having to call KeyChain.choosePrivateKeyAlias.When an app calls KeyChain.choosePrivateKeyAlias if more than one choosePrivateKeyRules matches, the last matching rule defines which key alias to return.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChoosePrivateKeyRule {
    /// The package names to which this rule applies. The hash of the signing certificate for each app is verified against the hash provided by Play. If no package names are specified, then the alias is provided to all apps that call KeyChain.choosePrivateKeyAlias (https://developer.android.com/reference/android/security/KeyChain#choosePrivateKeyAlias%28android.app.Activity,%20android.security.KeyChainAliasCallback,%20java.lang.String[],%20java.security.Principal[],%20java.lang.String,%20int,%20java.lang.String%29) or any overloads (but not without calling KeyChain.choosePrivateKeyAlias, even on Android 11 and above). Any app with the same Android UID as a package specified here will have access when they call KeyChain.choosePrivateKeyAlias.
    #[serde(rename="packageNames")]
    
    pub package_names: Option<Vec<String>>,
    /// The alias of the private key to be used.
    #[serde(rename="privateKeyAlias")]
    
    pub private_key_alias: Option<String>,
    /// The URL pattern to match against the URL of the request. If not set or empty, it matches all URLs. This uses the regular expression syntax of java.util.regex.Pattern.
    #[serde(rename="urlPattern")]
    
    pub url_pattern: Option<String>,
}

impl client::Part for ChoosePrivateKeyRule {}


/// Parameters associated with the CLEAR_APP_DATA command to clear the data of specified apps from the device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClearAppsDataParams {
    /// The package names of the apps whose data will be cleared when the command is executed.
    #[serde(rename="packageNames")]
    
    pub package_names: Option<Vec<String>>,
}

impl client::Part for ClearAppsDataParams {}


/// Status of the CLEAR_APP_DATA command to clear the data of specified apps from the device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClearAppsDataStatus {
    /// The per-app results, a mapping from package names to the respective clearing result.
    
    pub results: Option<HashMap<String, PerAppResult>>,
}

impl client::Part for ClearAppsDataStatus {}


/// A command.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices issue command enterprises](EnterpriseDeviceIssueCommandCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Command {
    /// Parameters for the CLEAR_APP_DATA command to clear the data of specified apps from the device. See ClearAppsDataParams. If this is set, then it is suggested that type should not be set. In this case, the server automatically sets it to CLEAR_APP_DATA. It is also acceptable to explicitly set type to CLEAR_APP_DATA.
    #[serde(rename="clearAppsDataParams")]
    
    pub clear_apps_data_params: Option<ClearAppsDataParams>,
    /// Output only. Status of the CLEAR_APP_DATA command to clear the data of specified apps from the device. See ClearAppsDataStatus.
    #[serde(rename="clearAppsDataStatus")]
    
    pub clear_apps_data_status: Option<ClearAppsDataStatus>,
    /// The timestamp at which the command was created. The timestamp is automatically generated by the server.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The duration for which the command is valid. The command will expire if not executed by the device during this time. The default duration if unspecified is ten minutes. There is no maximum duration.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub duration: Option<client::chrono::Duration>,
    /// If the command failed, an error code explaining the failure. This is not set when the command is cancelled by the caller.
    #[serde(rename="errorCode")]
    
    pub error_code: Option<CommandErrorCodeEnum>,
    /// For commands of type RESET_PASSWORD, optionally specifies the new password.
    #[serde(rename="newPassword")]
    
    pub new_password: Option<String>,
    /// For commands of type RESET_PASSWORD, optionally specifies flags.
    #[serde(rename="resetPasswordFlags")]
    
    pub reset_password_flags: Option<Vec<CommandResetPasswordFlagsEnum>>,
    /// The type of the command.
    #[serde(rename="type")]
    
    pub type_: Option<CommandTypeEnum>,
    /// The resource name of the user that owns the device in the form enterprises/{enterpriseId}/users/{userId}. This is automatically generated by the server based on the device the command is sent to.
    #[serde(rename="userName")]
    
    pub user_name: Option<String>,
}

impl client::RequestValue for Command {}


/// Information about Common Criteria Mode—security standards defined in the Common Criteria for Information Technology Security Evaluation (https://www.commoncriteriaportal.org/) (CC).This information is only available if statusReportingSettings.commonCriteriaModeEnabled is true in the device's policy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommonCriteriaModeInfo {
    /// Whether Common Criteria Mode is enabled.
    #[serde(rename="commonCriteriaModeStatus")]
    
    pub common_criteria_mode_status: Option<CommonCriteriaModeInfoCommonCriteriaModeStatusEnum>,
}

impl client::Part for CommonCriteriaModeInfo {}


/// A rule declaring which mitigating actions to take when a device is not compliant with its policy. For every rule, there is always an implicit mitigating action to set policy_compliant to false for the Device resource, and display a message on the device indicating that the device is not compliant with its policy. Other mitigating actions may optionally be taken as well, depending on the field values in the rule.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ComplianceRule {
    /// A condition which is satisfied if the Android Framework API level on the device doesn't meet a minimum requirement.
    #[serde(rename="apiLevelCondition")]
    
    pub api_level_condition: Option<ApiLevelCondition>,
    /// If set to true, the rule includes a mitigating action to disable apps so that the device is effectively disabled, but app data is preserved. If the device is running an app in locked task mode, the app will be closed and a UI showing the reason for non-compliance will be displayed.
    #[serde(rename="disableApps")]
    
    pub disable_apps: Option<bool>,
    /// A condition which is satisfied if there exists any matching NonComplianceDetail for the device.
    #[serde(rename="nonComplianceDetailCondition")]
    
    pub non_compliance_detail_condition: Option<NonComplianceDetailCondition>,
    /// If set, the rule includes a mitigating action to disable apps specified in the list, but app data is preserved.
    #[serde(rename="packageNamesToDisable")]
    
    pub package_names_to_disable: Option<Vec<String>>,
}

impl client::Part for ComplianceRule {}


/// Contact details for managed Google Play enterprises.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContactInfo {
    /// Email address for a point of contact, which will be used to send important announcements related to managed Google Play.
    #[serde(rename="contactEmail")]
    
    pub contact_email: Option<String>,
    /// The email of the data protection officer. The email is validated but not verified.
    #[serde(rename="dataProtectionOfficerEmail")]
    
    pub data_protection_officer_email: Option<String>,
    /// The name of the data protection officer.
    #[serde(rename="dataProtectionOfficerName")]
    
    pub data_protection_officer_name: Option<String>,
    /// The phone number of the data protection officer The phone number is validated but not verified.
    #[serde(rename="dataProtectionOfficerPhone")]
    
    pub data_protection_officer_phone: Option<String>,
    /// The email of the EU representative. The email is validated but not verified.
    #[serde(rename="euRepresentativeEmail")]
    
    pub eu_representative_email: Option<String>,
    /// The name of the EU representative.
    #[serde(rename="euRepresentativeName")]
    
    pub eu_representative_name: Option<String>,
    /// The phone number of the EU representative. The phone number is validated but not verified.
    #[serde(rename="euRepresentativePhone")]
    
    pub eu_representative_phone: Option<String>,
}

impl client::Part for ContactInfo {}


/// This feature is not generally available.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContentProviderEndpoint {
    /// This feature is not generally available.
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
    /// Required. This feature is not generally available.
    #[serde(rename="signingCertsSha256")]
    
    pub signing_certs_sha256: Option<Vec<String>>,
    /// This feature is not generally available.
    
    pub uri: Option<String>,
}

impl client::Part for ContentProviderEndpoint {}


/// Cross-profile policies applied on the device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CrossProfilePolicies {
    /// Whether text copied from one profile (personal or work) can be pasted in the other profile.
    #[serde(rename="crossProfileCopyPaste")]
    
    pub cross_profile_copy_paste: Option<CrossProfilePolicyCrossProfileCopyPasteEnum>,
    /// Whether data from one profile (personal or work) can be shared with apps in the other profile. Specifically controls simple data sharing via intents. Management of other cross-profile communication channels, such as contact search, copy/paste, or connected work & personal apps, are configured separately.
    #[serde(rename="crossProfileDataSharing")]
    
    pub cross_profile_data_sharing: Option<CrossProfilePolicyCrossProfileDataSharingEnum>,
    /// Whether contacts stored in the work profile can be shown in personal profile contact searches and incoming calls.
    #[serde(rename="showWorkContactsInPersonalProfile")]
    
    pub show_work_contacts_in_personal_profile: Option<CrossProfilePolicyShowWorkContactsInPersonalProfileEnum>,
    /// Specifies the default behaviour for work profile widgets. If the policy does not specify work_profile_widgets for a specific application, it will behave according to the value specified here.
    #[serde(rename="workProfileWidgetsDefault")]
    
    pub work_profile_widgets_default: Option<CrossProfilePolicyWorkProfileWidgetsDefaultEnum>,
}

impl client::Part for CrossProfilePolicies {}


/// Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: A full date, with non-zero year, month, and day values. A month and day, with a zero year (for example, an anniversary). A year on its own, with a zero month and a zero day. A year and month, with a zero day (for example, a credit card expiration date).Related types: google.type.TimeOfDay google.type.DateTime google.protobuf.Timestamp
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


/// A device owned by an enterprise. Unless otherwise noted, all fields are read-only and can’t be modified by enterprises.devices.patch.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices get enterprises](EnterpriseDeviceGetCall) (response)
/// * [devices patch enterprises](EnterpriseDevicePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Device {
    /// The API level of the Android platform version running on the device.
    #[serde(rename="apiLevel")]
    
    pub api_level: Option<i32>,
    /// Reports for apps installed on the device. This information is only available when application_reports_enabled is true in the device's policy.
    #[serde(rename="applicationReports")]
    
    pub application_reports: Option<Vec<ApplicationReport>>,
    /// The password requirements currently applied to the device. The applied requirements may be slightly different from those specified in passwordPolicies in some cases. fieldPath is set based on passwordPolicies.
    #[serde(rename="appliedPasswordPolicies")]
    
    pub applied_password_policies: Option<Vec<PasswordRequirements>>,
    /// The name of the policy currently applied to the device.
    #[serde(rename="appliedPolicyName")]
    
    pub applied_policy_name: Option<String>,
    /// The version of the policy currently applied to the device.
    #[serde(rename="appliedPolicyVersion")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub applied_policy_version: Option<i64>,
    /// The state currently applied to the device.
    #[serde(rename="appliedState")]
    
    pub applied_state: Option<DeviceAppliedStateEnum>,
    /// Information about Common Criteria Mode—security standards defined in the Common Criteria for Information Technology Security Evaluation (https://www.commoncriteriaportal.org/) (CC).This information is only available if statusReportingSettings.commonCriteriaModeEnabled is true in the device's policy.
    #[serde(rename="commonCriteriaModeInfo")]
    
    pub common_criteria_mode_info: Option<CommonCriteriaModeInfo>,
    /// Device settings information. This information is only available if deviceSettingsEnabled is true in the device's policy.
    #[serde(rename="deviceSettings")]
    
    pub device_settings: Option<DeviceSettings>,
    /// If the device state is DISABLED, an optional message that is displayed on the device indicating the reason the device is disabled. This field can be modified by a patch request.
    #[serde(rename="disabledReason")]
    
    pub disabled_reason: Option<UserFacingMessage>,
    /// Detailed information about displays on the device. This information is only available if displayInfoEnabled is true in the device's policy.
    
    pub displays: Option<Vec<Display>>,
    /// The time of device enrollment.
    #[serde(rename="enrollmentTime")]
    
    pub enrollment_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// If the device was enrolled with an enrollment token with additional data provided, this field contains that data.
    #[serde(rename="enrollmentTokenData")]
    
    pub enrollment_token_data: Option<String>,
    /// If the device was enrolled with an enrollment token, this field contains the name of the token.
    #[serde(rename="enrollmentTokenName")]
    
    pub enrollment_token_name: Option<String>,
    /// Detailed information about the device hardware.
    #[serde(rename="hardwareInfo")]
    
    pub hardware_info: Option<HardwareInfo>,
    /// Hardware status samples in chronological order. This information is only available if hardwareStatusEnabled is true in the device's policy.
    #[serde(rename="hardwareStatusSamples")]
    
    pub hardware_status_samples: Option<Vec<HardwareStatus>>,
    /// Deprecated.
    #[serde(rename="lastPolicyComplianceReportTime")]
    
    pub last_policy_compliance_report_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The last time the device fetched its policy.
    #[serde(rename="lastPolicySyncTime")]
    
    pub last_policy_sync_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The last time the device sent a status report.
    #[serde(rename="lastStatusReportTime")]
    
    pub last_status_report_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The type of management mode Android Device Policy takes on the device. This influences which policy settings are supported.
    #[serde(rename="managementMode")]
    
    pub management_mode: Option<DeviceManagementModeEnum>,
    /// Events related to memory and storage measurements in chronological order. This information is only available if memoryInfoEnabled is true in the device's policy.
    #[serde(rename="memoryEvents")]
    
    pub memory_events: Option<Vec<MemoryEvent>>,
    /// Memory information: contains information about device memory and storage.
    #[serde(rename="memoryInfo")]
    
    pub memory_info: Option<MemoryInfo>,
    /// The name of the device in the form enterprises/{enterpriseId}/devices/{deviceId}.
    
    pub name: Option<String>,
    /// Device network information. This information is only available if networkInfoEnabled is true in the device's policy.
    #[serde(rename="networkInfo")]
    
    pub network_info: Option<NetworkInfo>,
    /// Details about policy settings that the device is not compliant with.
    #[serde(rename="nonComplianceDetails")]
    
    pub non_compliance_details: Option<Vec<NonComplianceDetail>>,
    /// Ownership of the managed device.
    
    pub ownership: Option<DeviceOwnershipEnum>,
    /// Whether the device is compliant with its policy.
    #[serde(rename="policyCompliant")]
    
    pub policy_compliant: Option<bool>,
    /// The name of the policy applied to the device, in the form enterprises/{enterpriseId}/policies/{policyId}. If not specified, the policy_name for the device's user is applied. This field can be modified by a patch request. You can specify only the policyId when calling enterprises.devices.patch, as long as the policyId doesn’t contain any slashes. The rest of the policy name is inferred.
    #[serde(rename="policyName")]
    
    pub policy_name: Option<String>,
    /// Power management events on the device in chronological order. This information is only available if powerManagementEventsEnabled is true in the device's policy.
    #[serde(rename="powerManagementEvents")]
    
    pub power_management_events: Option<Vec<PowerManagementEvent>>,
    /// If the same physical device has been enrolled multiple times, this field contains its previous device names. The serial number is used as the unique identifier to determine if the same physical device has enrolled previously. The names are in chronological order.
    #[serde(rename="previousDeviceNames")]
    
    pub previous_device_names: Option<Vec<String>>,
    /// Device's security posture value that reflects how secure the device is.
    #[serde(rename="securityPosture")]
    
    pub security_posture: Option<SecurityPosture>,
    /// Detailed information about the device software. This information is only available if softwareInfoEnabled is true in the device's policy.
    #[serde(rename="softwareInfo")]
    
    pub software_info: Option<SoftwareInfo>,
    /// The state to be applied to the device. This field can be modified by a patch request. Note that when calling enterprises.devices.patch, ACTIVE and DISABLED are the only allowable values. To enter the device into a DELETED state, call enterprises.devices.delete.
    
    pub state: Option<DeviceStateEnum>,
    /// Map of selected system properties name and value related to the device. This information is only available if systemPropertiesEnabled is true in the device's policy.
    #[serde(rename="systemProperties")]
    
    pub system_properties: Option<HashMap<String, String>>,
    /// The user who owns the device.
    
    pub user: Option<User>,
    /// The resource name of the user that owns this device in the form enterprises/{enterpriseId}/users/{userId}.
    #[serde(rename="userName")]
    
    pub user_name: Option<String>,
}

impl client::RequestValue for Device {}
impl client::ResponseResult for Device {}


/// Information about security related device settings on device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceSettings {
    /// Whether ADB (https://developer.android.com/studio/command-line/adb.html) is enabled on the device.
    #[serde(rename="adbEnabled")]
    
    pub adb_enabled: Option<bool>,
    /// Whether developer mode is enabled on the device.
    #[serde(rename="developmentSettingsEnabled")]
    
    pub development_settings_enabled: Option<bool>,
    /// Encryption status from DevicePolicyManager.
    #[serde(rename="encryptionStatus")]
    
    pub encryption_status: Option<DeviceSettingEncryptionStatusEnum>,
    /// Whether the device is secured with PIN/password.
    #[serde(rename="isDeviceSecure")]
    
    pub is_device_secure: Option<bool>,
    /// Whether the storage encryption is enabled.
    #[serde(rename="isEncrypted")]
    
    pub is_encrypted: Option<bool>,
    /// Whether installing apps from unknown sources is enabled.
    #[serde(rename="unknownSourcesEnabled")]
    
    pub unknown_sources_enabled: Option<bool>,
    /// Whether Google Play Protect verification (https://support.google.com/accounts/answer/2812853) is enforced on the device.
    #[serde(rename="verifyAppsEnabled")]
    
    pub verify_apps_enabled: Option<bool>,
}

impl client::Part for DeviceSettings {}


/// Device display information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Display {
    /// Display density expressed as dots-per-inch.
    
    pub density: Option<i32>,
    /// Unique display id.
    #[serde(rename="displayId")]
    
    pub display_id: Option<i32>,
    /// Display height in pixels.
    
    pub height: Option<i32>,
    /// Name of the display.
    
    pub name: Option<String>,
    /// Refresh rate of the display in frames per second.
    #[serde(rename="refreshRate")]
    
    pub refresh_rate: Option<i32>,
    /// State of the display.
    
    pub state: Option<DisplayStateEnum>,
    /// Display width in pixels.
    
    pub width: Option<i32>,
}

impl client::Part for Display {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } 
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices operations cancel enterprises](EnterpriseDeviceOperationCancelCall) (response)
/// * [devices operations delete enterprises](EnterpriseDeviceOperationDeleteCall) (response)
/// * [devices delete enterprises](EnterpriseDeviceDeleteCall) (response)
/// * [enrollment tokens delete enterprises](EnterpriseEnrollmentTokenDeleteCall) (response)
/// * [policies delete enterprises](EnterprisePolicyDeleteCall) (response)
/// * [web apps delete enterprises](EnterpriseWebAppDeleteCall) (response)
/// * [delete enterprises](EnterpriseDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// An enrollment token.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [enrollment tokens create enterprises](EnterpriseEnrollmentTokenCreateCall) (request|response)
/// * [enrollment tokens get enterprises](EnterpriseEnrollmentTokenGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnrollmentToken {
    /// Optional, arbitrary data associated with the enrollment token. This could contain, for example, the ID of an org unit the device is assigned to after enrollment. After a device enrolls with the token, this data will be exposed in the enrollment_token_data field of the Device resource. The data must be 1024 characters or less; otherwise, the creation request will fail.
    #[serde(rename="additionalData")]
    
    pub additional_data: Option<String>,
    /// Controls whether personal usage is allowed on a device provisioned with this enrollment token.For company-owned devices: Enabling personal usage allows the user to set up a work profile on the device. Disabling personal usage requires the user provision the device as a fully managed device.For personally-owned devices: Enabling personal usage allows the user to set up a work profile on the device. Disabling personal usage will prevent the device from provisioning. Personal usage cannot be disabled on personally-owned device.
    #[serde(rename="allowPersonalUsage")]
    
    pub allow_personal_usage: Option<EnrollmentTokenAllowPersonalUsageEnum>,
    /// The length of time the enrollment token is valid, ranging from 1 minute to Durations.MAX_VALUE (https://developers.google.com/protocol-buffers/docs/reference/java/com/google/protobuf/util/Durations.html#MAX_VALUE), approximately 10,000 years. If not specified, the default duration is 1 hour. Please note that if requested duration causes the resulting expiration_timestamp to exceed Timestamps.MAX_VALUE (https://developers.google.com/protocol-buffers/docs/reference/java/com/google/protobuf/util/Timestamps.html#MAX_VALUE), then expiration_timestamp is coerced to Timestamps.MAX_VALUE.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub duration: Option<client::chrono::Duration>,
    /// The expiration time of the token. This is a read-only field generated by the server.
    #[serde(rename="expirationTimestamp")]
    
    pub expiration_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The name of the enrollment token, which is generated by the server during creation, in the form enterprises/{enterpriseId}/enrollmentTokens/{enrollmentTokenId}.
    
    pub name: Option<String>,
    /// Whether the enrollment token is for one time use only. If the flag is set to true, only one device can use it for registration.
    #[serde(rename="oneTimeOnly")]
    
    pub one_time_only: Option<bool>,
    /// The name of the policy initially applied to the enrolled device, in the form enterprises/{enterpriseId}/policies/{policyId}. If not specified, the policy_name for the device’s user is applied. If user_name is also not specified, enterprises/{enterpriseId}/policies/default is applied by default. When updating this field, you can specify only the policyId as long as the policyId doesn’t contain any slashes. The rest of the policy name will be inferred.
    #[serde(rename="policyName")]
    
    pub policy_name: Option<String>,
    /// A JSON string whose UTF-8 representation can be used to generate a QR code to enroll a device with this enrollment token. To enroll a device using NFC, the NFC record must contain a serialized java.util.Properties representation of the properties in the JSON.
    #[serde(rename="qrCode")]
    
    pub qr_code: Option<String>,
    /// The user associated with this enrollment token. If it's specified when the enrollment token is created and the user does not exist, the user will be created. This field must not contain personally identifiable information. Only the account_identifier field needs to be set.
    
    pub user: Option<User>,
    /// The token value that's passed to the device and authorizes the device to enroll. This is a read-only field generated by the server.
    
    pub value: Option<String>,
}

impl client::RequestValue for EnrollmentToken {}
impl client::ResponseResult for EnrollmentToken {}


/// The configuration applied to an enterprise.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [applications get enterprises](EnterpriseApplicationGetCall) (none)
/// * [devices operations cancel enterprises](EnterpriseDeviceOperationCancelCall) (none)
/// * [devices operations delete enterprises](EnterpriseDeviceOperationDeleteCall) (none)
/// * [devices operations get enterprises](EnterpriseDeviceOperationGetCall) (none)
/// * [devices operations list enterprises](EnterpriseDeviceOperationListCall) (none)
/// * [devices delete enterprises](EnterpriseDeviceDeleteCall) (none)
/// * [devices get enterprises](EnterpriseDeviceGetCall) (none)
/// * [devices issue command enterprises](EnterpriseDeviceIssueCommandCall) (none)
/// * [devices list enterprises](EnterpriseDeviceListCall) (none)
/// * [devices patch enterprises](EnterpriseDevicePatchCall) (none)
/// * [enrollment tokens create enterprises](EnterpriseEnrollmentTokenCreateCall) (none)
/// * [enrollment tokens delete enterprises](EnterpriseEnrollmentTokenDeleteCall) (none)
/// * [enrollment tokens get enterprises](EnterpriseEnrollmentTokenGetCall) (none)
/// * [enrollment tokens list enterprises](EnterpriseEnrollmentTokenListCall) (none)
/// * [policies delete enterprises](EnterprisePolicyDeleteCall) (none)
/// * [policies get enterprises](EnterprisePolicyGetCall) (none)
/// * [policies list enterprises](EnterprisePolicyListCall) (none)
/// * [policies patch enterprises](EnterprisePolicyPatchCall) (none)
/// * [web apps create enterprises](EnterpriseWebAppCreateCall) (none)
/// * [web apps delete enterprises](EnterpriseWebAppDeleteCall) (none)
/// * [web apps get enterprises](EnterpriseWebAppGetCall) (none)
/// * [web apps list enterprises](EnterpriseWebAppListCall) (none)
/// * [web apps patch enterprises](EnterpriseWebAppPatchCall) (none)
/// * [web tokens create enterprises](EnterpriseWebTokenCreateCall) (none)
/// * [create enterprises](EnterpriseCreateCall) (request|response)
/// * [delete enterprises](EnterpriseDeleteCall) (none)
/// * [get enterprises](EnterpriseGetCall) (response)
/// * [list enterprises](EnterpriseListCall) (none)
/// * [patch enterprises](EnterprisePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Enterprise {
    /// Deprecated and unused.
    #[serde(rename="appAutoApprovalEnabled")]
    
    pub app_auto_approval_enabled: Option<bool>,
    /// The enterprise contact info of an EMM-managed enterprise.
    #[serde(rename="contactInfo")]
    
    pub contact_info: Option<ContactInfo>,
    /// The types of Google Pub/Sub notifications enabled for the enterprise.
    #[serde(rename="enabledNotificationTypes")]
    
    pub enabled_notification_types: Option<Vec<EnterpriseEnabledNotificationTypesEnum>>,
    /// The name of the enterprise displayed to users.
    #[serde(rename="enterpriseDisplayName")]
    
    pub enterprise_display_name: Option<String>,
    /// An image displayed as a logo during device provisioning. Supported types are: image/bmp, image/gif, image/x-ico, image/jpeg, image/png, image/webp, image/vnd.wap.wbmp, image/x-adobe-dng.
    
    pub logo: Option<ExternalData>,
    /// The name of the enterprise which is generated by the server during creation, in the form enterprises/{enterpriseId}.
    
    pub name: Option<String>,
    /// A color in RGB format that indicates the predominant color to display in the device management app UI. The color components are stored as follows: (red << 16) | (green << 8) | blue, where the value of each component is between 0 and 255, inclusive.
    #[serde(rename="primaryColor")]
    
    pub primary_color: Option<i32>,
    /// The topic which Pub/Sub notifications are published to, in the form projects/{project}/topics/{topic}. This field is only required if Pub/Sub notifications are enabled.
    #[serde(rename="pubsubTopic")]
    
    pub pubsub_topic: Option<String>,
    /// Sign-in details of the enterprise.
    #[serde(rename="signinDetails")]
    
    pub signin_details: Option<Vec<SigninDetail>>,
    /// Terms and conditions that must be accepted when provisioning a device for this enterprise. A page of terms is generated for each value in this list.
    #[serde(rename="termsAndConditions")]
    
    pub terms_and_conditions: Option<Vec<TermsAndConditions>>,
}

impl client::RequestValue for Enterprise {}
impl client::Resource for Enterprise {}
impl client::ResponseResult for Enterprise {}


/// Configuration to enable an app as an extension app, with the capability of interacting with Android Device Policy offline. For Android versions 13 and above, extension apps are exempt from battery restrictions so will not be placed into the restricted App Standby Bucket (https://developer.android.com/topic/performance/appstandby#restricted-bucket). Extensions apps are also protected against users clearing their data or force-closing the application, although admins can continue to use the clear app data command (https://developer.android.com/management/reference/rest/v1/enterprises.devices/issueCommand#CommandType) on extension apps if needed for Android 13 and above.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExtensionConfig {
    /// Fully qualified class name of the receiver service class for Android Device Policy to notify the extension app of any local command status updates.
    #[serde(rename="notificationReceiver")]
    
    pub notification_receiver: Option<String>,
    /// Hex-encoded SHA-256 hash of the signing certificate of the extension app. Only hexadecimal string representations of 64 characters are valid.If not specified, the signature for the corresponding package name is obtained from the Play Store instead.If this list is empty, the signature of the extension app on the device must match the signature obtained from the Play Store for the app to be able to communicate with Android Device Policy.If this list is not empty, the signature of the extension app on the device must match one of the entries in this list for the app to be able to communicate with Android Device Policy.In production use cases, it is recommended to leave this empty.
    #[serde(rename="signingKeyFingerprintsSha256")]
    
    pub signing_key_fingerprints_sha256: Option<Vec<String>>,
}

impl client::Part for ExtensionConfig {}


/// Data hosted at an external location. The data is to be downloaded by Android Device Policy and verified against the hash.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExternalData {
    /// The base-64 encoded SHA-256 hash of the content hosted at url. If the content doesn't match this hash, Android Device Policy won't use the data.
    #[serde(rename="sha256Hash")]
    
    pub sha256_hash: Option<String>,
    /// The absolute URL to the data, which must use either the http or https scheme. Android Device Policy doesn't provide any credentials in the GET request, so the URL must be publicly accessible. Including a long, random component in the URL may be used to prevent attackers from discovering the URL.
    
    pub url: Option<String>,
}

impl client::Part for ExternalData {}


/// A system freeze period. When a device’s clock is within the freeze period, all incoming system updates (including security patches) are blocked and won’t be installed. When a device is outside the freeze period, normal update behavior applies. Leap years are ignored in freeze period calculations, in particular: * If Feb. 29th is set as the start or end date of a freeze period, the freeze period will start or end on Feb. 28th instead. * When a device’s system clock reads Feb. 29th, it’s treated as Feb. 28th. * When calculating the number of days in a freeze period or the time between two freeze periods, Feb. 29th is ignored and not counted as a day.Note: For Freeze Periods to take effect, SystemUpdateType cannot be specified as SYSTEM_UPDATE_TYPE_UNSPECIFIED, because freeze periods require a defined policy to be specified.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FreezePeriod {
    /// The end date (inclusive) of the freeze period. Must be no later than 90 days from the start date. If the end date is earlier than the start date, the freeze period is considered wrapping year-end. Note: year must not be set. For example, {"month": 1,"date": 30}.
    #[serde(rename="endDate")]
    
    pub end_date: Option<Date>,
    /// The start date (inclusive) of the freeze period. Note: year must not be set. For example, {"month": 1,"date": 30}.
    #[serde(rename="startDate")]
    
    pub start_date: Option<Date>,
}

impl client::Part for FreezePeriod {}


/// Information about device hardware. The fields related to temperature thresholds are only available if hardwareStatusEnabled is true in the device's policy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HardwareInfo {
    /// Battery shutdown temperature thresholds in Celsius for each battery on the device.
    #[serde(rename="batteryShutdownTemperatures")]
    
    pub battery_shutdown_temperatures: Option<Vec<f32>>,
    /// Battery throttling temperature thresholds in Celsius for each battery on the device.
    #[serde(rename="batteryThrottlingTemperatures")]
    
    pub battery_throttling_temperatures: Option<Vec<f32>>,
    /// Brand of the device. For example, Google.
    
    pub brand: Option<String>,
    /// CPU shutdown temperature thresholds in Celsius for each CPU on the device.
    #[serde(rename="cpuShutdownTemperatures")]
    
    pub cpu_shutdown_temperatures: Option<Vec<f32>>,
    /// CPU throttling temperature thresholds in Celsius for each CPU on the device.
    #[serde(rename="cpuThrottlingTemperatures")]
    
    pub cpu_throttling_temperatures: Option<Vec<f32>>,
    /// Baseband version. For example, MDM9625_104662.22.05.34p.
    #[serde(rename="deviceBasebandVersion")]
    
    pub device_baseband_version: Option<String>,
    /// Output only. ID that uniquely identifies a personally-owned device in a particular organization. On the same physical device when enrolled with the same organization, this ID persists across setups and even factory resets. This ID is available on personally-owned devices with a work profile on devices running Android 12 and above.
    #[serde(rename="enterpriseSpecificId")]
    
    pub enterprise_specific_id: Option<String>,
    /// GPU shutdown temperature thresholds in Celsius for each GPU on the device.
    #[serde(rename="gpuShutdownTemperatures")]
    
    pub gpu_shutdown_temperatures: Option<Vec<f32>>,
    /// GPU throttling temperature thresholds in Celsius for each GPU on the device.
    #[serde(rename="gpuThrottlingTemperatures")]
    
    pub gpu_throttling_temperatures: Option<Vec<f32>>,
    /// Name of the hardware. For example, Angler.
    
    pub hardware: Option<String>,
    /// Manufacturer. For example, Motorola.
    
    pub manufacturer: Option<String>,
    /// The model of the device. For example, Asus Nexus 7.
    
    pub model: Option<String>,
    /// The device serial number.
    #[serde(rename="serialNumber")]
    
    pub serial_number: Option<String>,
    /// Device skin shutdown temperature thresholds in Celsius.
    #[serde(rename="skinShutdownTemperatures")]
    
    pub skin_shutdown_temperatures: Option<Vec<f32>>,
    /// Device skin throttling temperature thresholds in Celsius.
    #[serde(rename="skinThrottlingTemperatures")]
    
    pub skin_throttling_temperatures: Option<Vec<f32>>,
}

impl client::Part for HardwareInfo {}


/// Hardware status. Temperatures may be compared to the temperature thresholds available in hardwareInfo to determine hardware health.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HardwareStatus {
    /// Current battery temperatures in Celsius for each battery on the device.
    #[serde(rename="batteryTemperatures")]
    
    pub battery_temperatures: Option<Vec<f32>>,
    /// Current CPU temperatures in Celsius for each CPU on the device.
    #[serde(rename="cpuTemperatures")]
    
    pub cpu_temperatures: Option<Vec<f32>>,
    /// CPU usages in percentage for each core available on the device. Usage is 0 for each unplugged core. Empty array implies that CPU usage is not supported in the system.
    #[serde(rename="cpuUsages")]
    
    pub cpu_usages: Option<Vec<f32>>,
    /// The time the measurements were taken.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Fan speeds in RPM for each fan on the device. Empty array means that there are no fans or fan speed is not supported on the system.
    #[serde(rename="fanSpeeds")]
    
    pub fan_speeds: Option<Vec<f32>>,
    /// Current GPU temperatures in Celsius for each GPU on the device.
    #[serde(rename="gpuTemperatures")]
    
    pub gpu_temperatures: Option<Vec<f32>>,
    /// Current device skin temperatures in Celsius.
    #[serde(rename="skinTemperatures")]
    
    pub skin_temperatures: Option<Vec<f32>>,
}

impl client::Part for HardwareStatus {}


/// Keyed app state reported by the app.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KeyedAppState {
    /// The creation time of the app state on the device.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optionally, a machine-readable value to be read by the EMM. For example, setting values that the admin can choose to query against in the EMM console (e.g. “notify me if the battery_warning data < 10”).
    
    pub data: Option<String>,
    /// The key for the app state. Acts as a point of reference for what the app is providing state for. For example, when providing managed configuration feedback, this key could be the managed configuration key.
    
    pub key: Option<String>,
    /// The time the app state was most recently updated.
    #[serde(rename="lastUpdateTime")]
    
    pub last_update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optionally, a free-form message string to explain the app state. If the state was triggered by a particular value (e.g. a managed configuration value), it should be included in the message.
    
    pub message: Option<String>,
    /// The severity of the app state.
    
    pub severity: Option<KeyedAppStateSeverityEnum>,
}

impl client::Part for KeyedAppState {}


/// Settings controlling the behavior of a device in kiosk mode. To enable kiosk mode, set kioskCustomLauncherEnabled to true or specify an app in the policy with installType KIOSK.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KioskCustomization {
    /// Specifies whether the Settings app is allowed in kiosk mode.
    #[serde(rename="deviceSettings")]
    
    pub device_settings: Option<KioskCustomizationDeviceSettingsEnum>,
    /// Sets the behavior of a device in kiosk mode when a user presses and holds (long-presses) the Power button.
    #[serde(rename="powerButtonActions")]
    
    pub power_button_actions: Option<KioskCustomizationPowerButtonActionsEnum>,
    /// Specifies whether system info and notifications are disabled in kiosk mode.
    #[serde(rename="statusBar")]
    
    pub status_bar: Option<KioskCustomizationStatusBarEnum>,
    /// Specifies whether system error dialogs for crashed or unresponsive apps are blocked in kiosk mode. When blocked, the system will force-stop the app as if the user chooses the "close app" option on the UI.
    #[serde(rename="systemErrorWarnings")]
    
    pub system_error_warnings: Option<KioskCustomizationSystemErrorWarningsEnum>,
    /// Specifies which navigation features are enabled (e.g. Home, Overview buttons) in kiosk mode.
    #[serde(rename="systemNavigation")]
    
    pub system_navigation: Option<KioskCustomizationSystemNavigationEnum>,
}

impl client::Part for KioskCustomization {}


/// An action to launch an app.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LaunchAppAction {
    /// Package name of app to be launched
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
}

impl client::Part for LaunchAppAction {}


/// Response to a request to list devices for a given enterprise.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices list enterprises](EnterpriseDeviceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDevicesResponse {
    /// The list of devices.
    
    pub devices: Option<Vec<Device>>,
    /// If there are more results, a token to retrieve next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListDevicesResponse {}


/// Response to a request to list enrollment tokens for a given enterprise.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [enrollment tokens list enterprises](EnterpriseEnrollmentTokenListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListEnrollmentTokensResponse {
    /// The list of enrollment tokens.
    #[serde(rename="enrollmentTokens")]
    
    pub enrollment_tokens: Option<Vec<EnrollmentToken>>,
    /// If there are more results, a token to retrieve next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListEnrollmentTokensResponse {}


/// Response to a request to list enterprises.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list enterprises](EnterpriseListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListEnterprisesResponse {
    /// The list of enterprises.
    
    pub enterprises: Option<Vec<Enterprise>>,
    /// If there are more results, a token to retrieve next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListEnterprisesResponse {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices operations list enterprises](EnterpriseDeviceOperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOperationsResponse {
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of operations that matches the specified filter in the request.
    
    pub operations: Option<Vec<Operation>>,
}

impl client::ResponseResult for ListOperationsResponse {}


/// Response to a request to list policies for a given enterprise.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [policies list enterprises](EnterprisePolicyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListPoliciesResponse {
    /// If there are more results, a token to retrieve next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of policies.
    
    pub policies: Option<Vec<Policy>>,
}

impl client::ResponseResult for ListPoliciesResponse {}


/// Response to a request to list web apps for a given enterprise.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [web apps list enterprises](EnterpriseWebAppListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListWebAppsResponse {
    /// If there are more results, a token to retrieve next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of web apps.
    #[serde(rename="webApps")]
    
    pub web_apps: Option<Vec<WebApp>>,
}

impl client::ResponseResult for ListWebAppsResponse {}


/// The managed configurations template for the app, saved from the managed configurations iframe.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedConfigurationTemplate {
    /// Optional, a map containing configuration variables defined for the configuration.
    #[serde(rename="configurationVariables")]
    
    pub configuration_variables: Option<HashMap<String, String>>,
    /// The ID of the managed configurations template.
    #[serde(rename="templateId")]
    
    pub template_id: Option<String>,
}

impl client::Part for ManagedConfigurationTemplate {}


/// Managed property.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedProperty {
    /// The default value of the property. BUNDLE_ARRAY properties don't have a default value.
    #[serde(rename="defaultValue")]
    
    pub default_value: Option<json::Value>,
    /// A longer description of the property, providing more detail of what it affects. Localized.
    
    pub description: Option<String>,
    /// For CHOICE or MULTISELECT properties, the list of possible entries.
    
    pub entries: Option<Vec<ManagedPropertyEntry>>,
    /// The unique key that the app uses to identify the property, e.g. "com.google.android.gm.fieldname".
    
    pub key: Option<String>,
    /// For BUNDLE_ARRAY properties, the list of nested properties. A BUNDLE_ARRAY property is at most two levels deep.
    #[serde(rename="nestedProperties")]
    
    pub nested_properties: Option<Vec<ManagedProperty>>,
    /// The name of the property. Localized.
    
    pub title: Option<String>,
    /// The type of the property.
    #[serde(rename="type")]
    
    pub type_: Option<ManagedPropertyTypeEnum>,
}

impl client::Part for ManagedProperty {}


/// An entry of a managed property.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedPropertyEntry {
    /// The human-readable name of the value. Localized.
    
    pub name: Option<String>,
    /// The machine-readable value of the entry, which should be used in the configuration. Not localized.
    
    pub value: Option<String>,
}

impl client::Part for ManagedPropertyEntry {}


/// An event related to memory and storage measurements.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MemoryEvent {
    /// The number of free bytes in the medium, or for EXTERNAL_STORAGE_DETECTED, the total capacity in bytes of the storage medium.
    #[serde(rename="byteCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub byte_count: Option<i64>,
    /// The creation time of the event.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Event type.
    #[serde(rename="eventType")]
    
    pub event_type: Option<MemoryEventEventTypeEnum>,
}

impl client::Part for MemoryEvent {}


/// Information about device memory and storage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MemoryInfo {
    /// Total internal storage on device in bytes.
    #[serde(rename="totalInternalStorage")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_internal_storage: Option<i64>,
    /// Total RAM on device in bytes.
    #[serde(rename="totalRam")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_ram: Option<i64>,
}

impl client::Part for MemoryInfo {}


/// Device network info.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkInfo {
    /// IMEI number of the GSM device. For example, A1000031212.
    
    pub imei: Option<String>,
    /// MEID number of the CDMA device. For example, A00000292788E1.
    
    pub meid: Option<String>,
    /// Alphabetic name of current registered operator. For example, Vodafone.
    #[serde(rename="networkOperatorName")]
    
    pub network_operator_name: Option<String>,
    /// Provides telephony information associated with each SIM card on the device. Only supported on fully managed devices starting from Android API level 23.
    #[serde(rename="telephonyInfos")]
    
    pub telephony_infos: Option<Vec<TelephonyInfo>>,
    /// Wi-Fi MAC address of the device. For example, 7c:11:11:11:11:11.
    #[serde(rename="wifiMacAddress")]
    
    pub wifi_mac_address: Option<String>,
}

impl client::Part for NetworkInfo {}


/// Provides detail about non-compliance with a policy setting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NonComplianceDetail {
    /// If the policy setting could not be applied, the current value of the setting on the device.
    #[serde(rename="currentValue")]
    
    pub current_value: Option<json::Value>,
    /// For settings with nested fields, if a particular nested field is out of compliance, this specifies the full path to the offending field. The path is formatted in the same way the policy JSON field would be referenced in JavaScript, that is: 1) For object-typed fields, the field name is followed by a dot then by a subfield name. 2) For array-typed fields, the field name is followed by the array index enclosed in brackets. For example, to indicate a problem with the url field in the externalData field in the 3rd application, the path would be applications[2].externalData.url
    #[serde(rename="fieldPath")]
    
    pub field_path: Option<String>,
    /// If package_name is set and the non-compliance reason is APP_NOT_INSTALLED or APP_NOT_UPDATED, the detailed reason the app can't be installed or updated.
    #[serde(rename="installationFailureReason")]
    
    pub installation_failure_reason: Option<NonComplianceDetailInstallationFailureReasonEnum>,
    /// The reason the device is not in compliance with the setting.
    #[serde(rename="nonComplianceReason")]
    
    pub non_compliance_reason: Option<NonComplianceDetailNonComplianceReasonEnum>,
    /// The package name indicating which app is out of compliance, if applicable.
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
    /// The name of the policy setting. This is the JSON field name of a top-level Policy field.
    #[serde(rename="settingName")]
    
    pub setting_name: Option<String>,
    /// Additional context for specific_non_compliance_reason.
    #[serde(rename="specificNonComplianceContext")]
    
    pub specific_non_compliance_context: Option<SpecificNonComplianceContext>,
    /// The policy-specific reason the device is not in compliance with the setting.
    #[serde(rename="specificNonComplianceReason")]
    
    pub specific_non_compliance_reason: Option<NonComplianceDetailSpecificNonComplianceReasonEnum>,
}

impl client::Part for NonComplianceDetail {}


/// A compliance rule condition which is satisfied if there exists any matching NonComplianceDetail for the device. A NonComplianceDetail matches a NonComplianceDetailCondition if all the fields which are set within the NonComplianceDetailCondition match the corresponding NonComplianceDetail fields.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NonComplianceDetailCondition {
    /// The reason the device is not in compliance with the setting. If not set, then this condition matches any reason.
    #[serde(rename="nonComplianceReason")]
    
    pub non_compliance_reason: Option<NonComplianceDetailConditionNonComplianceReasonEnum>,
    /// The package name of the app that's out of compliance. If not set, then this condition matches any package name.
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
    /// The name of the policy setting. This is the JSON field name of a top-level Policy field. If not set, then this condition matches any setting name.
    #[serde(rename="settingName")]
    
    pub setting_name: Option<String>,
}

impl client::Part for NonComplianceDetailCondition {}


/// This feature is not generally available.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OncCertificateProvider {
    /// This feature is not generally available.
    #[serde(rename="certificateReferences")]
    
    pub certificate_references: Option<Vec<String>>,
    /// This feature is not generally available.
    #[serde(rename="contentProviderEndpoint")]
    
    pub content_provider_endpoint: Option<ContentProviderEndpoint>,
}

impl client::Part for OncCertificateProvider {}


/// Additional context for non-compliance related to Wi-Fi configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OncWifiContext {
    /// The GUID of non-compliant Wi-Fi configuration.
    #[serde(rename="wifiGuid")]
    
    pub wifi_guid: Option<String>,
}

impl client::Part for OncWifiContext {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices operations get enterprises](EnterpriseDeviceOperationGetCall) (response)
/// * [devices issue command enterprises](EnterpriseDeviceIssueCommandCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<Status>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the name should be a resource name ending with operations/{unique_id}.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as Delete, the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type XxxResponse, where Xxx is the original method name. For example, if the original method name is TakeSnapshot(), the inferred response type is TakeSnapshotResponse.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for Operation {}


/// A list of package names.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PackageNameList {
    /// A list of package names.
    #[serde(rename="packageNames")]
    
    pub package_names: Option<Vec<String>>,
}

impl client::Part for PackageNameList {}


/// Additional context for non-compliance related to password policies.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PasswordPoliciesContext {
    /// The scope of non-compliant password.
    #[serde(rename="passwordPolicyScope")]
    
    pub password_policy_scope: Option<PasswordPoliciesContextPasswordPolicyScopeEnum>,
}

impl client::Part for PasswordPoliciesContext {}


/// Requirements for the password used to unlock a device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PasswordRequirements {
    /// Number of incorrect device-unlock passwords that can be entered before a device is wiped. A value of 0 means there is no restriction.
    #[serde(rename="maximumFailedPasswordsForWipe")]
    
    pub maximum_failed_passwords_for_wipe: Option<i32>,
    /// Password expiration timeout.
    #[serde(rename="passwordExpirationTimeout")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub password_expiration_timeout: Option<client::chrono::Duration>,
    /// The length of the password history. After setting this field, the user won't be able to enter a new password that is the same as any password in the history. A value of 0 means there is no restriction.
    #[serde(rename="passwordHistoryLength")]
    
    pub password_history_length: Option<i32>,
    /// The minimum allowed password length. A value of 0 means there is no restriction. Only enforced when password_quality is NUMERIC, NUMERIC_COMPLEX, ALPHABETIC, ALPHANUMERIC, or COMPLEX.
    #[serde(rename="passwordMinimumLength")]
    
    pub password_minimum_length: Option<i32>,
    /// Minimum number of letters required in the password. Only enforced when password_quality is COMPLEX.
    #[serde(rename="passwordMinimumLetters")]
    
    pub password_minimum_letters: Option<i32>,
    /// Minimum number of lower case letters required in the password. Only enforced when password_quality is COMPLEX.
    #[serde(rename="passwordMinimumLowerCase")]
    
    pub password_minimum_lower_case: Option<i32>,
    /// Minimum number of non-letter characters (numerical digits or symbols) required in the password. Only enforced when password_quality is COMPLEX.
    #[serde(rename="passwordMinimumNonLetter")]
    
    pub password_minimum_non_letter: Option<i32>,
    /// Minimum number of numerical digits required in the password. Only enforced when password_quality is COMPLEX.
    #[serde(rename="passwordMinimumNumeric")]
    
    pub password_minimum_numeric: Option<i32>,
    /// Minimum number of symbols required in the password. Only enforced when password_quality is COMPLEX.
    #[serde(rename="passwordMinimumSymbols")]
    
    pub password_minimum_symbols: Option<i32>,
    /// Minimum number of upper case letters required in the password. Only enforced when password_quality is COMPLEX.
    #[serde(rename="passwordMinimumUpperCase")]
    
    pub password_minimum_upper_case: Option<i32>,
    /// The required password quality.
    #[serde(rename="passwordQuality")]
    
    pub password_quality: Option<PasswordRequirementPasswordQualityEnum>,
    /// The scope that the password requirement applies to.
    #[serde(rename="passwordScope")]
    
    pub password_scope: Option<PasswordRequirementPasswordScopeEnum>,
    /// The length of time after a device or work profile is unlocked using a strong form of authentication (password, PIN, pattern) that it can be unlocked using any other authentication method (e.g. fingerprint, trust agents, face). After the specified time period elapses, only strong forms of authentication can be used to unlock the device or work profile.
    #[serde(rename="requirePasswordUnlock")]
    
    pub require_password_unlock: Option<PasswordRequirementRequirePasswordUnlockEnum>,
    /// Controls whether a unified lock is allowed for the device and the work profile, on devices running Android 9 and above with a work profile. This can be set only if password_scope is set to SCOPE_PROFILE, the policy will be rejected otherwise. If user has not set a separate work lock and this field is set to REQUIRE_SEPARATE_WORK_LOCK, a NonComplianceDetail is reported with nonComplianceReason set to USER_ACTION.
    #[serde(rename="unifiedLockSettings")]
    
    pub unified_lock_settings: Option<PasswordRequirementUnifiedLockSettingsEnum>,
}

impl client::Part for PasswordRequirements {}


/// The result of an attempt to clear the data of a single app.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PerAppResult {
    /// The result of an attempt to clear the data of a single app.
    #[serde(rename="clearingResult")]
    
    pub clearing_result: Option<PerAppResultClearingResultEnum>,
}

impl client::Part for PerAppResult {}


/// Configuration for an Android permission and its grant state.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PermissionGrant {
    /// The Android permission or group, e.g. android.permission.READ_CALENDAR or android.permission_group.CALENDAR.
    
    pub permission: Option<String>,
    /// The policy for granting the permission.
    
    pub policy: Option<PermissionGrantPolicyEnum>,
}

impl client::Part for PermissionGrant {}


/// A default activity for handling intents that match a particular intent filter. Note: To set up a kiosk, use InstallType to KIOSK rather than use persistent preferred activities.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersistentPreferredActivity {
    /// The intent actions to match in the filter. If any actions are included in the filter, then an intent's action must be one of those values for it to match. If no actions are included, the intent action is ignored.
    
    pub actions: Option<Vec<String>>,
    /// The intent categories to match in the filter. An intent includes the categories that it requires, all of which must be included in the filter in order to match. In other words, adding a category to the filter has no impact on matching unless that category is specified in the intent.
    
    pub categories: Option<Vec<String>>,
    /// The activity that should be the default intent handler. This should be an Android component name, e.g. com.android.enterprise.app/.MainActivity. Alternatively, the value may be the package name of an app, which causes Android Device Policy to choose an appropriate activity from the app to handle the intent.
    #[serde(rename="receiverActivity")]
    
    pub receiver_activity: Option<String>,
}

impl client::Part for PersistentPreferredActivity {}


/// Policies for apps in the personal profile of a company-owned device with a work profile.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonalApplicationPolicy {
    /// The type of installation to perform.
    #[serde(rename="installType")]
    
    pub install_type: Option<PersonalApplicationPolicyInstallTypeEnum>,
    /// The package name of the application.
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
}

impl client::Part for PersonalApplicationPolicy {}


/// Policies controlling personal usage on a company-owned device with a work profile.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonalUsagePolicies {
    /// Account types that can't be managed by the user.
    #[serde(rename="accountTypesWithManagementDisabled")]
    
    pub account_types_with_management_disabled: Option<Vec<String>>,
    /// If true, the camera is disabled on the personal profile.
    #[serde(rename="cameraDisabled")]
    
    pub camera_disabled: Option<bool>,
    /// Controls how long the work profile can stay off. The duration must be at least 3 days.
    #[serde(rename="maxDaysWithWorkOff")]
    
    pub max_days_with_work_off: Option<i32>,
    /// Policy applied to applications in the personal profile.
    #[serde(rename="personalApplications")]
    
    pub personal_applications: Option<Vec<PersonalApplicationPolicy>>,
    /// Used together with personalApplications to control how apps in the personal profile are allowed or blocked.
    #[serde(rename="personalPlayStoreMode")]
    
    pub personal_play_store_mode: Option<PersonalUsagePolicyPersonalPlayStoreModeEnum>,
    /// If true, screen capture is disabled for all users.
    #[serde(rename="screenCaptureDisabled")]
    
    pub screen_capture_disabled: Option<bool>,
}

impl client::Part for PersonalUsagePolicies {}


/// A policy resource represents a group of settings that govern the behavior of a managed device and the apps installed on it.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [policies get enterprises](EnterprisePolicyGetCall) (response)
/// * [policies patch enterprises](EnterprisePolicyPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// Account types that can't be managed by the user.
    #[serde(rename="accountTypesWithManagementDisabled")]
    
    pub account_types_with_management_disabled: Option<Vec<String>>,
    /// Whether adding new users and profiles is disabled.
    #[serde(rename="addUserDisabled")]
    
    pub add_user_disabled: Option<bool>,
    /// Whether adjusting the master volume is disabled. Also mutes the device.
    #[serde(rename="adjustVolumeDisabled")]
    
    pub adjust_volume_disabled: Option<bool>,
    /// Security policies set to secure values by default. To maintain the security posture of a device, we don't recommend overriding any of the default values.
    #[serde(rename="advancedSecurityOverrides")]
    
    pub advanced_security_overrides: Option<AdvancedSecurityOverrides>,
    /// Configuration for an always-on VPN connection. Use with vpn_config_disabled to prevent modification of this setting.
    #[serde(rename="alwaysOnVpnPackage")]
    
    pub always_on_vpn_package: Option<AlwaysOnVpnPackage>,
    /// The app tracks for Android Device Policy the device can access. The device receives the latest version among all accessible tracks. If no tracks are specified, then the device only uses the production track.
    #[serde(rename="androidDevicePolicyTracks")]
    
    pub android_device_policy_tracks: Option<Vec<PolicyAndroidDevicePolicyTracksEnum>>,
    /// Deprecated. Use autoUpdateMode instead.When autoUpdateMode is set to AUTO_UPDATE_POSTPONED or AUTO_UPDATE_HIGH_PRIORITY, this field has no effect.The app auto update policy, which controls when automatic app updates can be applied.
    #[serde(rename="appAutoUpdatePolicy")]
    
    pub app_auto_update_policy: Option<PolicyAppAutoUpdatePolicyEnum>,
    /// Policy applied to apps.
    
    pub applications: Option<Vec<ApplicationPolicy>>,
    /// Whether auto date, time, and time zone are enabled on a company-owned device. If this is set, then autoTimeRequired is ignored.
    #[serde(rename="autoDateAndTimeZone")]
    
    pub auto_date_and_time_zone: Option<PolicyAutoDateAndTimeZoneEnum>,
    /// Whether auto time is required, which prevents the user from manually setting the date and time. If autoDateAndTimeZone is set, this field is ignored.
    #[serde(rename="autoTimeRequired")]
    
    pub auto_time_required: Option<bool>,
    /// Whether applications other than the ones configured in applications are blocked from being installed. When set, applications that were installed under a previous policy but no longer appear in the policy are automatically uninstalled.
    #[serde(rename="blockApplicationsEnabled")]
    
    pub block_applications_enabled: Option<bool>,
    /// Whether configuring bluetooth is disabled.
    #[serde(rename="bluetoothConfigDisabled")]
    
    pub bluetooth_config_disabled: Option<bool>,
    /// Whether bluetooth contact sharing is disabled.
    #[serde(rename="bluetoothContactSharingDisabled")]
    
    pub bluetooth_contact_sharing_disabled: Option<bool>,
    /// Whether bluetooth is disabled. Prefer this setting over bluetooth_config_disabled because bluetooth_config_disabled can be bypassed by the user.
    #[serde(rename="bluetoothDisabled")]
    
    pub bluetooth_disabled: Option<bool>,
    /// Controls the use of the camera and whether the user has access to the camera access toggle.
    #[serde(rename="cameraAccess")]
    
    pub camera_access: Option<PolicyCameraAccessEnum>,
    /// If camera_access is set to any value other than CAMERA_ACCESS_UNSPECIFIED, this has no effect. Otherwise this field controls whether cameras are disabled: If true, all cameras are disabled, otherwise they are available. For fully managed devices this field applies for all apps on the device. For work profiles, this field applies only to apps in the work profile, and the camera access of apps outside the work profile is unaffected.
    #[serde(rename="cameraDisabled")]
    
    pub camera_disabled: Option<bool>,
    /// Whether configuring cell broadcast is disabled.
    #[serde(rename="cellBroadcastsConfigDisabled")]
    
    pub cell_broadcasts_config_disabled: Option<bool>,
    /// Rules for determining apps' access to private keys. See ChoosePrivateKeyRule for details.
    #[serde(rename="choosePrivateKeyRules")]
    
    pub choose_private_key_rules: Option<Vec<ChoosePrivateKeyRule>>,
    /// Rules declaring which mitigating actions to take when a device is not compliant with its policy. When the conditions for multiple rules are satisfied, all of the mitigating actions for the rules are taken. There is a maximum limit of 100 rules. Use policy enforcement rules instead.
    #[serde(rename="complianceRules")]
    
    pub compliance_rules: Option<Vec<ComplianceRule>>,
    /// Whether creating windows besides app windows is disabled.
    #[serde(rename="createWindowsDisabled")]
    
    pub create_windows_disabled: Option<bool>,
    /// Whether configuring user credentials is disabled.
    #[serde(rename="credentialsConfigDisabled")]
    
    pub credentials_config_disabled: Option<bool>,
    /// Cross-profile policies applied on the device.
    #[serde(rename="crossProfilePolicies")]
    
    pub cross_profile_policies: Option<CrossProfilePolicies>,
    /// Whether roaming data services are disabled.
    #[serde(rename="dataRoamingDisabled")]
    
    pub data_roaming_disabled: Option<bool>,
    /// Whether the user is allowed to enable debugging features.
    #[serde(rename="debuggingFeaturesAllowed")]
    
    pub debugging_features_allowed: Option<bool>,
    /// The default permission policy for runtime permission requests.
    #[serde(rename="defaultPermissionPolicy")]
    
    pub default_permission_policy: Option<PolicyDefaultPermissionPolicyEnum>,
    /// The device owner information to be shown on the lock screen.
    #[serde(rename="deviceOwnerLockScreenInfo")]
    
    pub device_owner_lock_screen_info: Option<UserFacingMessage>,
    /// Whether encryption is enabled
    #[serde(rename="encryptionPolicy")]
    
    pub encryption_policy: Option<PolicyEncryptionPolicyEnum>,
    /// Whether app verification is force-enabled.
    #[serde(rename="ensureVerifyAppsEnabled")]
    
    pub ensure_verify_apps_enabled: Option<bool>,
    /// Whether factory resetting from settings is disabled.
    #[serde(rename="factoryResetDisabled")]
    
    pub factory_reset_disabled: Option<bool>,
    /// Email addresses of device administrators for factory reset protection. When the device is factory reset, it will require one of these admins to log in with the Google account email and password to unlock the device. If no admins are specified, the device won't provide factory reset protection.
    #[serde(rename="frpAdminEmails")]
    
    pub frp_admin_emails: Option<Vec<String>>,
    /// Whether the user is allowed to have fun. Controls whether the Easter egg game in Settings is disabled.
    #[serde(rename="funDisabled")]
    
    pub fun_disabled: Option<bool>,
    /// Whether user installation of apps is disabled.
    #[serde(rename="installAppsDisabled")]
    
    pub install_apps_disabled: Option<bool>,
    /// This field has no effect.
    #[serde(rename="installUnknownSourcesAllowed")]
    
    pub install_unknown_sources_allowed: Option<bool>,
    /// If true, this disables the Lock Screen (https://source.android.com/docs/core/display/multi_display/lock-screen) for primary and/or secondary displays.
    #[serde(rename="keyguardDisabled")]
    
    pub keyguard_disabled: Option<bool>,
    /// Disabled keyguard customizations, such as widgets.
    #[serde(rename="keyguardDisabledFeatures")]
    
    pub keyguard_disabled_features: Option<Vec<PolicyKeyguardDisabledFeaturesEnum>>,
    /// Whether the kiosk custom launcher is enabled. This replaces the home screen with a launcher that locks down the device to the apps installed via the applications setting. Apps appear on a single page in alphabetical order. Use kioskCustomization to further configure the kiosk device behavior.
    #[serde(rename="kioskCustomLauncherEnabled")]
    
    pub kiosk_custom_launcher_enabled: Option<bool>,
    /// Settings controlling the behavior of a device in kiosk mode. To enable kiosk mode, set kioskCustomLauncherEnabled to true or specify an app in the policy with installType KIOSK.
    #[serde(rename="kioskCustomization")]
    
    pub kiosk_customization: Option<KioskCustomization>,
    /// The degree of location detection enabled.
    #[serde(rename="locationMode")]
    
    pub location_mode: Option<PolicyLocationModeEnum>,
    /// A message displayed to the user in the device administators settings screen.
    #[serde(rename="longSupportMessage")]
    
    pub long_support_message: Option<UserFacingMessage>,
    /// Maximum time in milliseconds for user activity until the device locks. A value of 0 means there is no restriction.
    #[serde(rename="maximumTimeToLock")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub maximum_time_to_lock: Option<i64>,
    /// Controls the use of the microphone and whether the user has access to the microphone access toggle. This applies only on fully managed devices.
    #[serde(rename="microphoneAccess")]
    
    pub microphone_access: Option<PolicyMicrophoneAccessEnum>,
    /// The minimum allowed Android API level.
    #[serde(rename="minimumApiLevel")]
    
    pub minimum_api_level: Option<i32>,
    /// Whether configuring mobile networks is disabled.
    #[serde(rename="mobileNetworksConfigDisabled")]
    
    pub mobile_networks_config_disabled: Option<bool>,
    /// Whether adding or removing accounts is disabled.
    #[serde(rename="modifyAccountsDisabled")]
    
    pub modify_accounts_disabled: Option<bool>,
    /// Whether the user mounting physical external media is disabled.
    #[serde(rename="mountPhysicalMediaDisabled")]
    
    pub mount_physical_media_disabled: Option<bool>,
    /// The name of the policy in the form enterprises/{enterpriseId}/policies/{policyId}.
    
    pub name: Option<String>,
    /// Whether the network escape hatch is enabled. If a network connection can't be made at boot time, the escape hatch prompts the user to temporarily connect to a network in order to refresh the device policy. After applying policy, the temporary network will be forgotten and the device will continue booting. This prevents being unable to connect to a network if there is no suitable network in the last policy and the device boots into an app in lock task mode, or the user is otherwise unable to reach device settings.Note: Setting wifiConfigDisabled to true will override this setting under specific circumstances. Please see wifiConfigDisabled for further details.
    #[serde(rename="networkEscapeHatchEnabled")]
    
    pub network_escape_hatch_enabled: Option<bool>,
    /// Whether resetting network settings is disabled.
    #[serde(rename="networkResetDisabled")]
    
    pub network_reset_disabled: Option<bool>,
    /// This feature is not generally available.
    #[serde(rename="oncCertificateProviders")]
    
    pub onc_certificate_providers: Option<Vec<OncCertificateProvider>>,
    /// Network configuration for the device. See configure networks for more information.
    #[serde(rename="openNetworkConfiguration")]
    
    pub open_network_configuration: Option<HashMap<String, json::Value>>,
    /// Whether using NFC to beam data from apps is disabled.
    #[serde(rename="outgoingBeamDisabled")]
    
    pub outgoing_beam_disabled: Option<bool>,
    /// Whether outgoing calls are disabled.
    #[serde(rename="outgoingCallsDisabled")]
    
    pub outgoing_calls_disabled: Option<bool>,
    /// Password requirement policies. Different policies can be set for work profile or fully managed devices by setting the password_scope field in the policy.
    #[serde(rename="passwordPolicies")]
    
    pub password_policies: Option<Vec<PasswordRequirements>>,
    /// Password requirements. The field password_requirements.require_password_unlock must not be set. DEPRECATED - Use passwordPolicies.Note:Complexity-based values of PasswordQuality, that is, COMPLEXITY_LOW, COMPLEXITY_MEDIUM, and COMPLEXITY_HIGH, cannot be used here. unified_lock_settings cannot be used here.
    #[serde(rename="passwordRequirements")]
    
    pub password_requirements: Option<PasswordRequirements>,
    /// Explicit permission or group grants or denials for all apps. These values override the default_permission_policy.
    #[serde(rename="permissionGrants")]
    
    pub permission_grants: Option<Vec<PermissionGrant>>,
    /// Specifies permitted accessibility services. If the field is not set, any accessibility service can be used. If the field is set, only the accessibility services in this list and the system's built-in accessibility service can be used. In particular, if the field is set to empty, only the system's built-in accessibility servicess can be used. This can be set on fully managed devices and on work profiles. When applied to a work profile, this affects both the personal profile and the work profile.
    #[serde(rename="permittedAccessibilityServices")]
    
    pub permitted_accessibility_services: Option<PackageNameList>,
    /// If present, only the input methods provided by packages in this list are permitted. If this field is present, but the list is empty, then only system input methods are permitted.
    #[serde(rename="permittedInputMethods")]
    
    pub permitted_input_methods: Option<PackageNameList>,
    /// Default intent handler activities.
    #[serde(rename="persistentPreferredActivities")]
    
    pub persistent_preferred_activities: Option<Vec<PersistentPreferredActivity>>,
    /// Policies managing personal usage on a company-owned device.
    #[serde(rename="personalUsagePolicies")]
    
    pub personal_usage_policies: Option<PersonalUsagePolicies>,
    /// This mode controls which apps are available to the user in the Play Store and the behavior on the device when apps are removed from the policy.
    #[serde(rename="playStoreMode")]
    
    pub play_store_mode: Option<PolicyPlayStoreModeEnum>,
    /// Rules that define the behavior when a particular policy can not be applied on device
    #[serde(rename="policyEnforcementRules")]
    
    pub policy_enforcement_rules: Option<Vec<PolicyEnforcementRule>>,
    /// Controls whether preferential network service is enabled on the work profile. For example, an organization may have an agreement with a carrier that all of the work data from its employees' devices will be sent via a network service dedicated for enterprise use. An example of a supported preferential network service is the enterprise slice on 5G networks. This has no effect on fully managed devices.
    #[serde(rename="preferentialNetworkService")]
    
    pub preferential_network_service: Option<PolicyPreferentialNetworkServiceEnum>,
    /// Allows showing UI on a device for a user to choose a private key alias if there are no matching rules in ChoosePrivateKeyRules. For devices below Android P, setting this may leave enterprise keys vulnerable.
    #[serde(rename="privateKeySelectionEnabled")]
    
    pub private_key_selection_enabled: Option<bool>,
    /// The network-independent global HTTP proxy. Typically proxies should be configured per-network in open_network_configuration. However for unusual configurations like general internal filtering a global HTTP proxy may be useful. If the proxy is not accessible, network access may break. The global proxy is only a recommendation and some apps may ignore it.
    #[serde(rename="recommendedGlobalProxy")]
    
    pub recommended_global_proxy: Option<ProxyInfo>,
    /// Whether removing other users is disabled.
    #[serde(rename="removeUserDisabled")]
    
    pub remove_user_disabled: Option<bool>,
    /// Whether rebooting the device into safe boot is disabled.
    #[serde(rename="safeBootDisabled")]
    
    pub safe_boot_disabled: Option<bool>,
    /// Whether screen capture is disabled.
    #[serde(rename="screenCaptureDisabled")]
    
    pub screen_capture_disabled: Option<bool>,
    /// Whether changing the user icon is disabled.
    #[serde(rename="setUserIconDisabled")]
    
    pub set_user_icon_disabled: Option<bool>,
    /// Whether changing the wallpaper is disabled.
    #[serde(rename="setWallpaperDisabled")]
    
    pub set_wallpaper_disabled: Option<bool>,
    /// Action to take during the setup process. At most one action may be specified.
    #[serde(rename="setupActions")]
    
    pub setup_actions: Option<Vec<SetupAction>>,
    /// Whether location sharing is disabled. share_location_disabled is supported for both fully managed devices and personally owned work profiles.
    #[serde(rename="shareLocationDisabled")]
    
    pub share_location_disabled: Option<bool>,
    /// A message displayed to the user in the settings screen wherever functionality has been disabled by the admin. If the message is longer than 200 characters it may be truncated.
    #[serde(rename="shortSupportMessage")]
    
    pub short_support_message: Option<UserFacingMessage>,
    /// Flag to skip hints on the first use. Enterprise admin can enable the system recommendation for apps to skip their user tutorial and other introductory hints on first start-up.
    #[serde(rename="skipFirstUseHintsEnabled")]
    
    pub skip_first_use_hints_enabled: Option<bool>,
    /// Whether sending and receiving SMS messages is disabled.
    #[serde(rename="smsDisabled")]
    
    pub sms_disabled: Option<bool>,
    /// Whether the status bar is disabled. This disables notifications, quick settings, and other screen overlays that allow escape from full-screen mode. DEPRECATED. To disable the status bar on a kiosk device, use InstallType KIOSK or kioskCustomLauncherEnabled.
    #[serde(rename="statusBarDisabled")]
    
    pub status_bar_disabled: Option<bool>,
    /// Status reporting settings
    #[serde(rename="statusReportingSettings")]
    
    pub status_reporting_settings: Option<StatusReportingSettings>,
    /// The battery plugged in modes for which the device stays on. When using this setting, it is recommended to clear maximum_time_to_lock so that the device doesn't lock itself while it stays on.
    #[serde(rename="stayOnPluggedModes")]
    
    pub stay_on_plugged_modes: Option<Vec<PolicyStayOnPluggedModesEnum>>,
    /// The system update policy, which controls how OS updates are applied. If the update type is WINDOWED, the update window will automatically apply to Play app updates as well.
    #[serde(rename="systemUpdate")]
    
    pub system_update: Option<SystemUpdate>,
    /// Whether configuring tethering and portable hotspots is disabled.
    #[serde(rename="tetheringConfigDisabled")]
    
    pub tethering_config_disabled: Option<bool>,
    /// Whether user uninstallation of applications is disabled. This prevents apps from being uninstalled, even those removed using applications
    #[serde(rename="uninstallAppsDisabled")]
    
    pub uninstall_apps_disabled: Option<bool>,
    /// If microphone_access is set to any value other than MICROPHONE_ACCESS_UNSPECIFIED, this has no effect. Otherwise this field controls whether microphones are disabled: If true, all microphones are disabled, otherwise they are available. This is available only on fully managed devices.
    #[serde(rename="unmuteMicrophoneDisabled")]
    
    pub unmute_microphone_disabled: Option<bool>,
    /// Configuration of device activity logging.
    #[serde(rename="usageLog")]
    
    pub usage_log: Option<UsageLog>,
    /// Whether transferring files over USB is disabled. This is supported only on company-owned devices.
    #[serde(rename="usbFileTransferDisabled")]
    
    pub usb_file_transfer_disabled: Option<bool>,
    /// Whether USB storage is enabled. Deprecated.
    #[serde(rename="usbMassStorageEnabled")]
    
    pub usb_mass_storage_enabled: Option<bool>,
    /// The version of the policy. This is a read-only field. The version is incremented each time the policy is updated.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version: Option<i64>,
    /// Whether configuring VPN is disabled.
    #[serde(rename="vpnConfigDisabled")]
    
    pub vpn_config_disabled: Option<bool>,
    /// Whether configuring Wi-Fi access points is disabled. Note: If a network connection can't be made at boot time and configuring Wi-Fi is disabled then network escape hatch will be shown in order to refresh the device policy (see networkEscapeHatchEnabled).
    #[serde(rename="wifiConfigDisabled")]
    
    pub wifi_config_disabled: Option<bool>,
    /// DEPRECATED - Use wifi_config_disabled.
    #[serde(rename="wifiConfigsLockdownEnabled")]
    
    pub wifi_configs_lockdown_enabled: Option<bool>,
}

impl client::RequestValue for Policy {}
impl client::ResponseResult for Policy {}


/// A rule that defines the actions to take if a device or work profile is not compliant with the policy specified in settingName.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PolicyEnforcementRule {
    /// An action to block access to apps and data on a company owned device or in a work profile. This action also triggers a user-facing notification with information (where possible) on how to correct the compliance issue. Note: wipeAction must also be specified.
    #[serde(rename="blockAction")]
    
    pub block_action: Option<BlockAction>,
    /// The top-level policy to enforce. For example, applications or passwordPolicies.
    #[serde(rename="settingName")]
    
    pub setting_name: Option<String>,
    /// An action to reset a company owned device or delete a work profile. Note: blockAction must also be specified.
    #[serde(rename="wipeAction")]
    
    pub wipe_action: Option<WipeAction>,
}

impl client::Part for PolicyEnforcementRule {}


/// Additional details regarding the security posture of the device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostureDetail {
    /// Corresponding admin-facing advice to mitigate this security risk and improve the security posture of the device.
    
    pub advice: Option<Vec<UserFacingMessage>>,
    /// A specific security risk that negatively affects the security posture of the device.
    #[serde(rename="securityRisk")]
    
    pub security_risk: Option<PostureDetailSecurityRiskEnum>,
}

impl client::Part for PostureDetail {}


/// A power management event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PowerManagementEvent {
    /// For BATTERY_LEVEL_COLLECTED events, the battery level as a percentage.
    #[serde(rename="batteryLevel")]
    
    pub battery_level: Option<f32>,
    /// The creation time of the event.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Event type.
    #[serde(rename="eventType")]
    
    pub event_type: Option<PowerManagementEventEventTypeEnum>,
}

impl client::Part for PowerManagementEvent {}


/// Configuration info for an HTTP proxy. For a direct proxy, set the host, port, and excluded_hosts fields. For a PAC script proxy, set the pac_uri field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProxyInfo {
    /// For a direct proxy, the hosts for which the proxy is bypassed. The host names may contain wildcards such as *.example.com.
    #[serde(rename="excludedHosts")]
    
    pub excluded_hosts: Option<Vec<String>>,
    /// The host of the direct proxy.
    
    pub host: Option<String>,
    /// The URI of the PAC script used to configure the proxy.
    #[serde(rename="pacUri")]
    
    pub pac_uri: Option<String>,
    /// The port of the direct proxy.
    
    pub port: Option<i32>,
}

impl client::Part for ProxyInfo {}


/// The security posture of the device, as determined by the current device state and the policies applied.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SecurityPosture {
    /// Device's security posture value.
    #[serde(rename="devicePosture")]
    
    pub device_posture: Option<SecurityPostureDevicePostureEnum>,
    /// Additional details regarding the security posture of the device.
    #[serde(rename="postureDetails")]
    
    pub posture_details: Option<Vec<PostureDetail>>,
}

impl client::Part for SecurityPosture {}


/// An action executed during setup.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetupAction {
    /// Description of this action.
    
    pub description: Option<UserFacingMessage>,
    /// An action to launch an app. The app will be launched with an intent containing an extra with key com.google.android.apps.work.clouddpc.EXTRA_LAUNCHED_AS_SETUP_ACTION set to the boolean value true to indicate that this is a setup action flow. If SetupAction references an app, the corresponding installType in the application policy must be set as REQUIRED_FOR_SETUP or said setup will fail.
    #[serde(rename="launchApp")]
    
    pub launch_app: Option<LaunchAppAction>,
    /// Title of this action.
    
    pub title: Option<UserFacingMessage>,
}

impl client::Part for SetupAction {}


/// A resource containing sign in details for an enterprise.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SigninDetail {
    /// Controls whether personal usage is allowed on a device provisioned with this enrollment token.For company-owned devices: Enabling personal usage allows the user to set up a work profile on the device. Disabling personal usage requires the user provision the device as a fully managed device.For personally-owned devices: Enabling personal usage allows the user to set up a work profile on the device. Disabling personal usage will prevent the device from provisioning. Personal usage cannot be disabled on personally-owned device.
    #[serde(rename="allowPersonalUsage")]
    
    pub allow_personal_usage: Option<SigninDetailAllowPersonalUsageEnum>,
    /// A JSON string whose UTF-8 representation can be used to generate a QR code to enroll a device with this enrollment token. To enroll a device using NFC, the NFC record must contain a serialized java.util.Properties representation of the properties in the JSON. This is a read-only field generated by the server.
    #[serde(rename="qrCode")]
    
    pub qr_code: Option<String>,
    /// An enterprise wide enrollment token used to trigger custom sign-in flow. This is a read-only field generated by the server.
    #[serde(rename="signinEnrollmentToken")]
    
    pub signin_enrollment_token: Option<String>,
    /// Sign-in URL for authentication when device is provisioned with a sign-in enrollment token. The sign-in endpoint should finish authentication flow with a URL in the form of https://enterprise.google.com/android/enroll?et= for a successful login, or https://enterprise.google.com/android/enroll/invalid for a failed login.
    #[serde(rename="signinUrl")]
    
    pub signin_url: Option<String>,
}

impl client::Part for SigninDetail {}


/// An enterprise signup URL.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create signup urls](SignupUrlCreateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SignupUrl {
    /// The name of the resource. Use this value in the signupUrl field when calling enterprises.create to complete the enterprise signup flow.
    
    pub name: Option<String>,
    /// A URL where an enterprise admin can register their enterprise. The page can't be rendered in an iframe.
    
    pub url: Option<String>,
}

impl client::Resource for SignupUrl {}
impl client::ResponseResult for SignupUrl {}


/// Information about device software.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SoftwareInfo {
    /// Android build ID string meant for displaying to the user. For example, shamu-userdebug 6.0.1 MOB30I 2756745 dev-keys.
    #[serde(rename="androidBuildNumber")]
    
    pub android_build_number: Option<String>,
    /// Build time.
    #[serde(rename="androidBuildTime")]
    
    pub android_build_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The Android Device Policy app version code.
    #[serde(rename="androidDevicePolicyVersionCode")]
    
    pub android_device_policy_version_code: Option<i32>,
    /// The Android Device Policy app version as displayed to the user.
    #[serde(rename="androidDevicePolicyVersionName")]
    
    pub android_device_policy_version_name: Option<String>,
    /// The user-visible Android version string. For example, 6.0.1.
    #[serde(rename="androidVersion")]
    
    pub android_version: Option<String>,
    /// The system bootloader version number, e.g. 0.6.7.
    #[serde(rename="bootloaderVersion")]
    
    pub bootloader_version: Option<String>,
    /// SHA-256 hash of android.content.pm.Signature (https://developer.android.com/reference/android/content/pm/Signature.html) associated with the system package, which can be used to verify that the system build hasn't been modified.
    #[serde(rename="deviceBuildSignature")]
    
    pub device_build_signature: Option<String>,
    /// Kernel version, for example, 2.6.32.9-g103d848.
    #[serde(rename="deviceKernelVersion")]
    
    pub device_kernel_version: Option<String>,
    /// An IETF BCP 47 language code for the primary locale on the device.
    #[serde(rename="primaryLanguageCode")]
    
    pub primary_language_code: Option<String>,
    /// Security patch level, e.g. 2016-05-01.
    #[serde(rename="securityPatchLevel")]
    
    pub security_patch_level: Option<String>,
    /// Information about a potential pending system update.
    #[serde(rename="systemUpdateInfo")]
    
    pub system_update_info: Option<SystemUpdateInfo>,
}

impl client::Part for SoftwareInfo {}


/// Additional context for SpecificNonComplianceReason.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpecificNonComplianceContext {
    /// Additional context for non-compliance related to Wi-Fi configuration. See ONC_WIFI_INVALID_VALUE and ONC_WIFI_API_LEVEL
    #[serde(rename="oncWifiContext")]
    
    pub onc_wifi_context: Option<OncWifiContext>,
    /// Additional context for non-compliance related to password policies. See PASSWORD_POLICIES_PASSWORD_EXPIRED and PASSWORD_POLICIES_PASSWORD_NOT_SUFFICIENT.
    #[serde(rename="passwordPoliciesContext")]
    
    pub password_policies_context: Option<PasswordPoliciesContext>,
}

impl client::Part for SpecificNonComplianceContext {}


/// The Status type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by gRPC (https://github.com/grpc). Each Status message contains three pieces of data: error code, error message, and error details.You can find out more about this error model and how to work with it in the API Design Guide (https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for Status {}


/// Settings controlling the behavior of status reports.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StatusReportingSettings {
    /// Application reporting settings. Only applicable if application_reports_enabled is true.
    #[serde(rename="applicationReportingSettings")]
    
    pub application_reporting_settings: Option<ApplicationReportingSettings>,
    /// Whether app reports are enabled.
    #[serde(rename="applicationReportsEnabled")]
    
    pub application_reports_enabled: Option<bool>,
    /// Whether Common Criteria Mode reporting is enabled.
    #[serde(rename="commonCriteriaModeEnabled")]
    
    pub common_criteria_mode_enabled: Option<bool>,
    /// Whether device settings reporting is enabled.
    #[serde(rename="deviceSettingsEnabled")]
    
    pub device_settings_enabled: Option<bool>,
    /// Whether displays reporting is enabled. Report data is not available for personally owned devices with work profiles.
    #[serde(rename="displayInfoEnabled")]
    
    pub display_info_enabled: Option<bool>,
    /// Whether hardware status reporting is enabled. Report data is not available for personally owned devices with work profiles.
    #[serde(rename="hardwareStatusEnabled")]
    
    pub hardware_status_enabled: Option<bool>,
    /// Whether memory event reporting is enabled.
    #[serde(rename="memoryInfoEnabled")]
    
    pub memory_info_enabled: Option<bool>,
    /// Whether network info reporting is enabled.
    #[serde(rename="networkInfoEnabled")]
    
    pub network_info_enabled: Option<bool>,
    /// Whether power management event reporting is enabled. Report data is not available for personally owned devices with work profiles.
    #[serde(rename="powerManagementEventsEnabled")]
    
    pub power_management_events_enabled: Option<bool>,
    /// Whether software info reporting is enabled.
    #[serde(rename="softwareInfoEnabled")]
    
    pub software_info_enabled: Option<bool>,
    /// Whether system properties reporting is enabled.
    #[serde(rename="systemPropertiesEnabled")]
    
    pub system_properties_enabled: Option<bool>,
}

impl client::Part for StatusReportingSettings {}


/// Configuration for managing system updates
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SystemUpdate {
    /// If the type is WINDOWED, the end of the maintenance window, measured as the number of minutes after midnight in device's local time. This value must be between 0 and 1439, inclusive. If this value is less than start_minutes, then the maintenance window spans midnight. If the maintenance window specified is smaller than 30 minutes, the actual window is extended to 30 minutes beyond the start time.
    #[serde(rename="endMinutes")]
    
    pub end_minutes: Option<i32>,
    /// An annually repeating time period in which over-the-air (OTA) system updates are postponed to freeze the OS version running on a device. To prevent freezing the device indefinitely, each freeze period must be separated by at least 60 days.
    #[serde(rename="freezePeriods")]
    
    pub freeze_periods: Option<Vec<FreezePeriod>>,
    /// If the type is WINDOWED, the start of the maintenance window, measured as the number of minutes after midnight in the device's local time. This value must be between 0 and 1439, inclusive.
    #[serde(rename="startMinutes")]
    
    pub start_minutes: Option<i32>,
    /// The type of system update to configure.
    #[serde(rename="type")]
    
    pub type_: Option<SystemUpdateTypeEnum>,
}

impl client::Part for SystemUpdate {}


/// Information about a potential pending system update.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SystemUpdateInfo {
    /// The time when the update was first available. A zero value indicates that this field is not set. This field is set only if an update is available (that is, updateStatus is neither UPDATE_STATUS_UNKNOWN nor UP_TO_DATE).
    #[serde(rename="updateReceivedTime")]
    
    pub update_received_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The status of an update: whether an update exists and what type it is.
    #[serde(rename="updateStatus")]
    
    pub update_status: Option<SystemUpdateInfoUpdateStatusEnum>,
}

impl client::Part for SystemUpdateInfo {}


/// Telephony information associated with a given SIM card on the device. Only supported on fully managed devices starting from Android API level 23.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TelephonyInfo {
    /// The carrier name associated with this SIM card.
    #[serde(rename="carrierName")]
    
    pub carrier_name: Option<String>,
    /// The phone number associated with this SIM card.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
}

impl client::Part for TelephonyInfo {}


/// A terms and conditions page to be accepted during provisioning.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TermsAndConditions {
    /// A well-formatted HTML string. It will be parsed on the client with android.text.Html#fromHtml.
    
    pub content: Option<UserFacingMessage>,
    /// A short header which appears above the HTML content.
    
    pub header: Option<UserFacingMessage>,
}

impl client::Part for TermsAndConditions {}


/// Controls types of device activity logs collected from the device and reported via Pub/Sub notification (https://developers.google.com/android/management/notifications).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsageLog {
    /// Specifies which log types are enabled. Note that users will receive on-device messaging when usage logging is enabled.
    #[serde(rename="enabledLogTypes")]
    
    pub enabled_log_types: Option<Vec<UsageLogEnabledLogTypesEnum>>,
    /// Specifies which of the enabled log types can be uploaded over mobile data. By default logs are queued for upload when the device connects to WiFi.
    #[serde(rename="uploadOnCellularAllowed")]
    
    pub upload_on_cellular_allowed: Option<Vec<UsageLogUploadOnCellularAllowedEnum>>,
}

impl client::Part for UsageLog {}


/// A user belonging to an enterprise.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// A unique identifier you create for this user, such as user342 or asset#44418. This field must be set when the user is created and can't be updated. This field must not contain personally identifiable information (PII). This identifier must be 1024 characters or less; otherwise, the update policy request will fail.
    #[serde(rename="accountIdentifier")]
    
    pub account_identifier: Option<String>,
}

impl client::Part for User {}


/// Provides a user-facing message with locale info. The maximum message length is 4096 characters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserFacingMessage {
    /// The default message displayed if no localized message is specified or the user's locale doesn't match with any of the localized messages. A default message must be provided if any localized messages are provided.
    #[serde(rename="defaultMessage")]
    
    pub default_message: Option<String>,
    /// A map containing pairs, where locale is a well-formed BCP 47 language (https://www.w3.org/International/articles/language-tags/) code, such as en-US, es-ES, or fr.
    #[serde(rename="localizedMessages")]
    
    pub localized_messages: Option<HashMap<String, String>>,
}

impl client::Part for UserFacingMessage {}


/// A web app.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [web apps create enterprises](EnterpriseWebAppCreateCall) (request|response)
/// * [web apps get enterprises](EnterpriseWebAppGetCall) (response)
/// * [web apps patch enterprises](EnterpriseWebAppPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebApp {
    /// The display mode of the web app.
    #[serde(rename="displayMode")]
    
    pub display_mode: Option<WebAppDisplayModeEnum>,
    /// A list of icons for the web app. Must have at least one element.
    
    pub icons: Option<Vec<WebAppIcon>>,
    /// The name of the web app, which is generated by the server during creation in the form enterprises/{enterpriseId}/webApps/{packageName}.
    
    pub name: Option<String>,
    /// The start URL, i.e. the URL that should load when the user opens the application.
    #[serde(rename="startUrl")]
    
    pub start_url: Option<String>,
    /// The title of the web app as displayed to the user (e.g., amongst a list of other applications, or as a label for an icon).
    
    pub title: Option<String>,
    /// The current version of the app.Note that the version can automatically increase during the lifetime of the web app, while Google does internal housekeeping to keep the web app up-to-date.
    #[serde(rename="versionCode")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version_code: Option<i64>,
}

impl client::RequestValue for WebApp {}
impl client::ResponseResult for WebApp {}


/// An icon for a web app. Supported formats are: png, jpg and webp.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebAppIcon {
    /// The actual bytes of the image in a base64url encoded string (c.f. RFC4648, section 5 "Base 64 Encoding with URL and Filename Safe Alphabet"). - The image type can be png or jpg. - The image should ideally be square. - The image should ideally have a size of 512x512. 
    #[serde(rename="imageData")]
    
    pub image_data: Option<String>,
}

impl client::Part for WebAppIcon {}


/// A web token used to access the managed Google Play iframe.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [web tokens create enterprises](EnterpriseWebTokenCreateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebToken {
    /// The features to enable. Use this if you want to control exactly which feature(s) will be activated; leave empty to allow all features.Restrictions / things to note: - If no features are listed here, all features are enabled — this is the default behavior where you give access to all features to your admins. - This must not contain any FEATURE_UNSPECIFIED values. - Repeated values are ignored 
    #[serde(rename="enabledFeatures")]
    
    pub enabled_features: Option<Vec<WebTokenEnabledFeaturesEnum>>,
    /// The name of the web token, which is generated by the server during creation in the form enterprises/{enterpriseId}/webTokens/{webTokenId}.
    
    pub name: Option<String>,
    /// The URL of the parent frame hosting the iframe with the embedded UI. To prevent XSS, the iframe may not be hosted at other URLs. The URL must use the https scheme.
    #[serde(rename="parentFrameUrl")]
    
    pub parent_frame_url: Option<String>,
    /// Permissions available to an admin in the embedded UI. An admin must have all of these permissions in order to view the UI. This field is deprecated.
    
    pub permissions: Option<Vec<WebTokenPermissionsEnum>>,
    /// The token value which is used in the hosting page to generate the iframe with the embedded UI. This is a read-only field generated by the server.
    
    pub value: Option<String>,
}

impl client::RequestValue for WebToken {}
impl client::ResponseResult for WebToken {}


/// An action to reset a company owned device or delete a work profile. Note: blockAction must also be specified.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WipeAction {
    /// Whether the factory-reset protection data is preserved on the device. This setting doesn’t apply to work profiles.
    #[serde(rename="preserveFrp")]
    
    pub preserve_frp: Option<bool>,
    /// Number of days the policy is non-compliant before the device or work profile is wiped. wipeAfterDays must be greater than blockAfterDays.
    #[serde(rename="wipeAfterDays")]
    
    pub wipe_after_days: Option<i32>,
}

impl client::Part for WipeAction {}


