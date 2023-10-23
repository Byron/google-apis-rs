use super::*;
/// This represents an enterprise admin who can manage the enterprise in the managed Google Play store.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Administrator {
    /// The admin's email address.
    
    pub email: Option<String>,
}

impl client::Part for Administrator {}


/// A token authorizing an admin to access an iframe.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create web token enterprises](EnterpriseCreateWebTokenCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdministratorWebToken {
    /// An opaque token to be passed to the Play front-end to generate an iframe.
    
    pub token: Option<String>,
}

impl client::ResponseResult for AdministratorWebToken {}


/// Specification for a token used to generate iframes. The token specifies what data the admin is allowed to modify and the URI the iframe is allowed to communiate with.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create web token enterprises](EnterpriseCreateWebTokenCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdministratorWebTokenSpec {
    /// Options for displaying the Managed Configuration page.
    #[serde(rename="managedConfigurations")]
    
    pub managed_configurations: Option<AdministratorWebTokenSpecManagedConfigurations>,
    /// The URI of the parent frame hosting the iframe. To prevent XSS, the iframe may not be hosted at other URIs. This URI must be https. Use whitespaces to separate multiple parent URIs.
    
    pub parent: Option<String>,
    /// Deprecated. Use PlaySearch.approveApps.
    
    pub permission: Option<Vec<AdministratorWebTokenSpecPermissionEnum>>,
    /// Options for displaying the managed Play Search apps page.
    #[serde(rename="playSearch")]
    
    pub play_search: Option<AdministratorWebTokenSpecPlaySearch>,
    /// Options for displaying the Private Apps page.
    #[serde(rename="privateApps")]
    
    pub private_apps: Option<AdministratorWebTokenSpecPrivateApps>,
    /// Options for displaying the Organize apps page.
    #[serde(rename="storeBuilder")]
    
    pub store_builder: Option<AdministratorWebTokenSpecStoreBuilder>,
    /// Options for displaying the Web Apps page.
    #[serde(rename="webApps")]
    
    pub web_apps: Option<AdministratorWebTokenSpecWebApps>,
    /// Options for displaying the Zero Touch page.
    #[serde(rename="zeroTouch")]
    
    pub zero_touch: Option<AdministratorWebTokenSpecZeroTouch>,
}

impl client::RequestValue for AdministratorWebTokenSpec {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdministratorWebTokenSpecManagedConfigurations {
    /// Whether the Managed Configuration page is displayed. Default is true.
    
    pub enabled: Option<bool>,
}

impl client::Part for AdministratorWebTokenSpecManagedConfigurations {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdministratorWebTokenSpecPlaySearch {
    /// Allow access to the iframe in approve mode. Default is false.
    #[serde(rename="approveApps")]
    
    pub approve_apps: Option<bool>,
    /// Whether the managed Play Search apps page is displayed. Default is true.
    
    pub enabled: Option<bool>,
}

impl client::Part for AdministratorWebTokenSpecPlaySearch {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdministratorWebTokenSpecPrivateApps {
    /// Whether the Private Apps page is displayed. Default is true.
    
    pub enabled: Option<bool>,
}

impl client::Part for AdministratorWebTokenSpecPrivateApps {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdministratorWebTokenSpecStoreBuilder {
    /// Whether the Organize apps page is displayed. Default is true.
    
    pub enabled: Option<bool>,
}

impl client::Part for AdministratorWebTokenSpecStoreBuilder {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdministratorWebTokenSpecWebApps {
    /// Whether the Web Apps page is displayed. Default is true.
    
    pub enabled: Option<bool>,
}

impl client::Part for AdministratorWebTokenSpecWebApps {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdministratorWebTokenSpecZeroTouch {
    /// Whether zero-touch embedded UI is usable with this token. If enabled, the admin can link zero-touch customers to this enterprise.
    
    pub enabled: Option<bool>,
}

impl client::Part for AdministratorWebTokenSpecZeroTouch {}


/// Represents the list of app restrictions available to be pre-configured for the product.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get app restrictions schema products](ProductGetAppRestrictionsSchemaCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppRestrictionsSchema {
    /// Deprecated.
    
    pub kind: Option<String>,
    /// The set of restrictions that make up this schema.
    
    pub restrictions: Option<Vec<AppRestrictionsSchemaRestriction>>,
}

impl client::ResponseResult for AppRestrictionsSchema {}


/// An event generated when a new app version is uploaded to Google Play and its app restrictions schema changed. To fetch the app restrictions schema for an app, use Products.getAppRestrictionsSchema on the EMM API.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppRestrictionsSchemaChangeEvent {
    /// The id of the product (e.g. "app:com.google.android.gm") for which the app restriction schema changed. This field will always be present.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
}

impl client::Part for AppRestrictionsSchemaChangeEvent {}


/// A restriction in the App Restriction Schema represents a piece of configuration that may be pre-applied.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppRestrictionsSchemaRestriction {
    /// The default value of the restriction. bundle and bundleArray restrictions never have a default value.
    #[serde(rename="defaultValue")]
    
    pub default_value: Option<AppRestrictionsSchemaRestrictionRestrictionValue>,
    /// A longer description of the restriction, giving more detail of what it affects.
    
    pub description: Option<String>,
    /// For choice or multiselect restrictions, the list of possible entries' human-readable names.
    
    pub entry: Option<Vec<String>>,
    /// For choice or multiselect restrictions, the list of possible entries' machine-readable values. These values should be used in the configuration, either as a single string value for a choice restriction or in a stringArray for a multiselect restriction.
    #[serde(rename="entryValue")]
    
    pub entry_value: Option<Vec<String>>,
    /// The unique key that the product uses to identify the restriction, e.g. "com.google.android.gm.fieldname".
    
    pub key: Option<String>,
    /// For bundle or bundleArray restrictions, the list of nested restrictions. A bundle restriction is always nested within a bundleArray restriction, and a bundleArray restriction is at most two levels deep.
    #[serde(rename="nestedRestriction")]
    
    pub nested_restriction: Option<Vec<AppRestrictionsSchemaRestriction>>,
    /// The type of the restriction.
    #[serde(rename="restrictionType")]
    
    pub restriction_type: Option<AppRestrictionsSchemaRestrictionRestrictionTypeEnum>,
    /// The name of the restriction.
    
    pub title: Option<String>,
}

impl client::Part for AppRestrictionsSchemaRestriction {}


/// A typed value for the restriction.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppRestrictionsSchemaRestrictionRestrictionValue {
    /// The type of the value being provided.
    #[serde(rename="type")]
    
    pub type_: Option<AppRestrictionsSchemaRestrictionRestrictionValueTypeEnum>,
    /// The boolean value - this will only be present if type is bool.
    #[serde(rename="valueBool")]
    
    pub value_bool: Option<bool>,
    /// The integer value - this will only be present if type is integer.
    #[serde(rename="valueInteger")]
    
    pub value_integer: Option<i32>,
    /// The list of string values - this will only be present if type is multiselect.
    #[serde(rename="valueMultiselect")]
    
    pub value_multiselect: Option<Vec<String>>,
    /// The string value - this will be present for types string, choice and hidden.
    #[serde(rename="valueString")]
    
    pub value_string: Option<String>,
}

impl client::Part for AppRestrictionsSchemaRestrictionRestrictionValue {}


/// List of states set by the app.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppState {
    /// List of keyed app states. This field will always be present.
    #[serde(rename="keyedAppState")]
    
    pub keyed_app_state: Option<Vec<KeyedAppState>>,
    /// The package name of the app. This field will always be present.
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
}

impl client::Part for AppState {}


/// An event generated when a new version of an app is uploaded to Google Play. Notifications are sent for new public versions only: alpha, beta, or canary versions do not generate this event. To fetch up-to-date version history for an app, use Products.Get on the EMM API.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppUpdateEvent {
    /// The id of the product (e.g. "app:com.google.android.gm") that was updated. This field will always be present.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
}

impl client::Part for AppUpdateEvent {}


/// This represents a single version of the app.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppVersion {
    /// True if this version is a production APK.
    #[serde(rename="isProduction")]
    
    pub is_production: Option<bool>,
    /// Deprecated, use trackId instead.
    
    pub track: Option<AppVersionTrackEnum>,
    /// Track ids that the app version is published in. Replaces the track field (deprecated), but doesn't include the production track (see isProduction instead).
    #[serde(rename="trackId")]
    
    pub track_id: Option<Vec<String>>,
    /// Unique increasing identifier for the app version.
    #[serde(rename="versionCode")]
    
    pub version_code: Option<i32>,
    /// The string used in the Play store by the app developer to identify the version. The string is not necessarily unique or localized (for example, the string could be "1.4").
    #[serde(rename="versionString")]
    
    pub version_string: Option<String>,
}

impl client::Part for AppVersion {}


/// Information on an approval URL.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApprovalUrlInfo {
    /// A URL that displays a product's permissions and that can also be used to approve the product with the Products.approve call.
    #[serde(rename="approvalUrl")]
    
    pub approval_url: Option<String>,
}

impl client::Part for ApprovalUrlInfo {}


/// An AuthenticationToken is used by the EMM’s device policy client on a device to provision the given EMM-managed user on that device.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [generate authentication token users](UserGenerateAuthenticationTokenCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuthenticationToken {
    /// The authentication token to be passed to the device policy client on the device where it can be used to provision the account for which this token was generated.
    
    pub token: Option<String>,
}

impl client::ResponseResult for AuthenticationToken {}


/// The auto-install constraint. Defines a set of restrictions for installation. At least one of the fields must be set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoInstallConstraint {
    /// Charging state constraint.
    #[serde(rename="chargingStateConstraint")]
    
    pub charging_state_constraint: Option<AutoInstallConstraintChargingStateConstraintEnum>,
    /// Device idle state constraint.
    #[serde(rename="deviceIdleStateConstraint")]
    
    pub device_idle_state_constraint: Option<AutoInstallConstraintDeviceIdleStateConstraintEnum>,
    /// Network type constraint.
    #[serde(rename="networkTypeConstraint")]
    
    pub network_type_constraint: Option<AutoInstallConstraintNetworkTypeConstraintEnum>,
}

impl client::Part for AutoInstallConstraint {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoInstallPolicy {
    /// The constraints for auto-installing the app. You can specify a maximum of one constraint.
    #[serde(rename="autoInstallConstraint")]
    
    pub auto_install_constraint: Option<Vec<AutoInstallConstraint>>,
    /// The auto-install mode. If unset defaults to "doNotAutoInstall".
    #[serde(rename="autoInstallMode")]
    
    pub auto_install_mode: Option<AutoInstallPolicyAutoInstallModeEnum>,
    /// The priority of the install, as an unsigned integer. A lower number means higher priority.
    #[serde(rename="autoInstallPriority")]
    
    pub auto_install_priority: Option<i32>,
    /// The minimum version of the app. If a lower version of the app is installed, then the app will be auto-updated according to the auto-install constraints, instead of waiting for the regular auto-update. You can set a minimum version code for at most 20 apps per device.
    #[serde(rename="minimumVersionCode")]
    
    pub minimum_version_code: Option<i32>,
}

impl client::Part for AutoInstallPolicy {}


/// A configuration variables resource contains the managed configuration settings ID to be applied to a single user, as well as the variable set that is attributed to the user. The variable set will be used to replace placeholders in the managed configuration settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigurationVariables {
    /// The ID of the managed configurations settings.
    #[serde(rename="mcmId")]
    
    pub mcm_id: Option<String>,
    /// The variable set that is attributed to the user.
    #[serde(rename="variableSet")]
    
    pub variable_set: Option<Vec<VariableSet>>,
}

impl client::Part for ConfigurationVariables {}


/// Response message for create enrollment token.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create enrollment token enterprises](EnterpriseCreateEnrollmentTokenCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateEnrollmentTokenResponse {
    /// Enrollment token.
    #[serde(rename="enrollmentToken")]
    
    pub enrollment_token: Option<String>,
}

impl client::ResponseResult for CreateEnrollmentTokenResponse {}


/// A Devices resource represents a mobile device managed by the EMM and belonging to a specific enterprise user.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [force report upload devices](DeviceForceReportUploadCall) (none)
/// * [get devices](DeviceGetCall) (response)
/// * [get state devices](DeviceGetStateCall) (none)
/// * [list devices](DeviceListCall) (none)
/// * [set state devices](DeviceSetStateCall) (none)
/// * [update devices](DeviceUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Device {
    /// The Google Play Services Android ID for the device encoded as a lowercase hex string. For example, "123456789abcdef0".
    #[serde(rename="androidId")]
    
    pub android_id: Option<String>,
    /// Identifies the extent to which the device is controlled by a managed Google Play EMM in various deployment configurations. Possible values include: - "managedDevice", a device that has the EMM's device policy controller (DPC) as the device owner. - "managedProfile", a device that has a profile managed by the DPC (DPC is profile owner) in addition to a separate, personal profile that is unavailable to the DPC. - "containerApp", no longer used (deprecated). - "unmanagedProfile", a device that has been allowed (by the domain's admin, using the Admin Console to enable the privilege) to use managed Google Play, but the profile is itself not owned by a DPC. 
    #[serde(rename="managementType")]
    
    pub management_type: Option<DeviceManagementTypeEnum>,
    /// The policy enforced on the device.
    
    pub policy: Option<Policy>,
    /// The device report updated with the latest app states.
    
    pub report: Option<DeviceReport>,
}

impl client::RequestValue for Device {}
impl client::Resource for Device {}
impl client::ResponseResult for Device {}


/// Device report updated with the latest app states for managed apps on the device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceReport {
    /// List of app states set by managed apps on the device. App states are defined by the app's developers. This field will always be present.
    #[serde(rename="appState")]
    
    pub app_state: Option<Vec<AppState>>,
    /// The timestamp of the last report update in milliseconds since epoch. This field will always be present.
    #[serde(rename="lastUpdatedTimestampMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_updated_timestamp_millis: Option<i64>,
}

impl client::Part for DeviceReport {}


/// An event generated when an updated device report is available.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceReportUpdateEvent {
    /// The Android ID of the device. This field will always be present.
    #[serde(rename="deviceId")]
    
    pub device_id: Option<String>,
    /// The device report updated with the latest app states. This field will always be present.
    
    pub report: Option<DeviceReport>,
    /// The ID of the user. This field will always be present.
    #[serde(rename="userId")]
    
    pub user_id: Option<String>,
}

impl client::Part for DeviceReportUpdateEvent {}


/// The state of a user’s device, as accessed by the getState and setState methods on device resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get state devices](DeviceGetStateCall) (response)
/// * [set state devices](DeviceSetStateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceState {
    /// The state of the Google account on the device. "enabled" indicates that the Google account on the device can be used to access Google services (including Google Play), while "disabled" means that it cannot. A new device is initially in the "disabled" state.
    #[serde(rename="accountState")]
    
    pub account_state: Option<DeviceStateAccountStateEnum>,
}

impl client::RequestValue for DeviceState {}
impl client::ResponseResult for DeviceState {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list devices](DeviceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DevicesListResponse {
    /// A managed device.
    
    pub device: Option<Vec<Device>>,
}

impl client::ResponseResult for DevicesListResponse {}


/// An Enterprises resource represents the binding between an EMM and a specific organization. That binding can be instantiated in one of two different ways using this API as follows: - For Google managed domain customers, the process involves using Enterprises.enroll and Enterprises.setAccount (in conjunction with artifacts obtained from the Admin console and the Google API Console) and submitted to the EMM through a more-or-less manual process. - For managed Google Play Accounts customers, the process involves using Enterprises.generateSignupUrl and Enterprises.completeSignup in conjunction with the managed Google Play sign-up UI (Google-provided mechanism) to create the binding without manual steps. As an EMM, you can support either or both approaches in your EMM console. See Create an Enterprise for details.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [acknowledge notification set enterprises](EnterpriseAcknowledgeNotificationSetCall) (none)
/// * [complete signup enterprises](EnterpriseCompleteSignupCall) (response)
/// * [create enrollment token enterprises](EnterpriseCreateEnrollmentTokenCall) (none)
/// * [create web token enterprises](EnterpriseCreateWebTokenCall) (none)
/// * [enroll enterprises](EnterpriseEnrollCall) (request|response)
/// * [generate signup url enterprises](EnterpriseGenerateSignupUrlCall) (none)
/// * [get enterprises](EnterpriseGetCall) (response)
/// * [get service account enterprises](EnterpriseGetServiceAccountCall) (none)
/// * [get store layout enterprises](EnterpriseGetStoreLayoutCall) (none)
/// * [list enterprises](EnterpriseListCall) (none)
/// * [pull notification set enterprises](EnterprisePullNotificationSetCall) (none)
/// * [send test push notification enterprises](EnterpriseSendTestPushNotificationCall) (none)
/// * [set account enterprises](EnterpriseSetAccountCall) (none)
/// * [set store layout enterprises](EnterpriseSetStoreLayoutCall) (none)
/// * [unenroll enterprises](EnterpriseUnenrollCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Enterprise {
    /// Admins of the enterprise. This is only supported for enterprises created via the EMM-initiated flow.
    
    pub administrator: Option<Vec<Administrator>>,
    /// Output only. Settings for Google-provided user authentication.
    #[serde(rename="googleAuthenticationSettings")]
    
    pub google_authentication_settings: Option<GoogleAuthenticationSettings>,
    /// The unique ID for the enterprise.
    
    pub id: Option<String>,
    /// The name of the enterprise, for example, "Example, Inc".
    
    pub name: Option<String>,
    /// The enterprise's primary domain, such as "example.com".
    #[serde(rename="primaryDomain")]
    
    pub primary_domain: Option<String>,
}

impl client::RequestValue for Enterprise {}
impl client::Resource for Enterprise {}
impl client::ResponseResult for Enterprise {}


/// A service account that can be used to authenticate as the enterprise to API calls that require such authentication.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set account enterprises](EnterpriseSetAccountCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnterpriseAccount {
    /// The email address of the service account.
    #[serde(rename="accountEmail")]
    
    pub account_email: Option<String>,
}

impl client::RequestValue for EnterpriseAccount {}
impl client::ResponseResult for EnterpriseAccount {}


/// An authentication URL configuration for the authenticator app of an identity provider.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnterpriseAuthenticationAppLinkConfig {
    /// An authentication url.
    
    pub uri: Option<String>,
}

impl client::Part for EnterpriseAuthenticationAppLinkConfig {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list enterprises](EnterpriseListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnterprisesListResponse {
    /// An enterprise.
    
    pub enterprise: Option<Vec<Enterprise>>,
}

impl client::ResponseResult for EnterprisesListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [send test push notification enterprises](EnterpriseSendTestPushNotificationCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnterprisesSendTestPushNotificationResponse {
    /// The message ID of the test push notification that was sent.
    #[serde(rename="messageId")]
    
    pub message_id: Option<String>,
    /// The name of the Cloud Pub/Sub topic to which notifications for this enterprise's enrolled account will be sent.
    #[serde(rename="topicName")]
    
    pub topic_name: Option<String>,
}

impl client::ResponseResult for EnterprisesSendTestPushNotificationResponse {}


/// The presence of an Entitlements resource indicates that a user has the right to use a particular app. Entitlements are user specific, not device specific. This allows a user with an entitlement to an app to install the app on all their devices. It’s also possible for a user to hold an entitlement to an app without installing the app on any device. The API can be used to create an entitlement. As an option, you can also use the API to trigger the installation of an app on all a user’s managed devices at the same time the entitlement is created. If the app is free, creating the entitlement also creates a group license for that app. For paid apps, creating the entitlement consumes one license, and that license remains consumed until the entitlement is removed. If the enterprise hasn’t purchased enough licenses, then no entitlement is created and the installation fails. An entitlement is also not created for an app if the app requires permissions that the enterprise hasn’t accepted. If an entitlement is deleted, the app may be uninstalled from a user’s device. As a best practice, uninstall the app by calling Installs.delete() before deleting the entitlement. Entitlements for apps that a user pays for on an unmanaged profile have “userPurchase” as the entitlement reason. These entitlements cannot be removed via the API.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete entitlements](EntitlementDeleteCall) (none)
/// * [get entitlements](EntitlementGetCall) (response)
/// * [list entitlements](EntitlementListCall) (none)
/// * [update entitlements](EntitlementUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Entitlement {
    /// The ID of the product that the entitlement is for. For example, "app:com.google.android.gm".
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// The reason for the entitlement. For example, "free" for free apps. This property is temporary: it will be replaced by the acquisition kind field of group licenses.
    
    pub reason: Option<EntitlementReasonEnum>,
}

impl client::RequestValue for Entitlement {}
impl client::Resource for Entitlement {}
impl client::ResponseResult for Entitlement {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list entitlements](EntitlementListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EntitlementsListResponse {
    /// An entitlement of a user to a product (e.g. an app). For example, a free app that they have installed, or a paid app that they have been allocated a license to.
    
    pub entitlement: Option<Vec<Entitlement>>,
}

impl client::ResponseResult for EntitlementsListResponse {}


/// Contains settings for Google-provided user authentication.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAuthenticationSettings {
    /// Whether dedicated devices are allowed.
    #[serde(rename="dedicatedDevicesAllowed")]
    
    pub dedicated_devices_allowed: Option<GoogleAuthenticationSettingDedicatedDevicesAllowedEnum>,
    /// Whether Google authentication is required.
    #[serde(rename="googleAuthenticationRequired")]
    
    pub google_authentication_required: Option<GoogleAuthenticationSettingGoogleAuthenticationRequiredEnum>,
}

impl client::Part for GoogleAuthenticationSettings {}


/// Group license objects allow you to keep track of licenses (called entitlements) for both free and paid apps. For a free app, a group license is created when an enterprise admin first approves the product in Google Play or when the first entitlement for the product is created for a user via the API. For a paid app, a group license object is only created when an enterprise admin purchases the product in Google Play for the first time. Use the API to query group licenses. A Grouplicenses resource includes the total number of licenses purchased (paid apps only) and the total number of licenses currently in use. In other words, the total number of Entitlements that exist for the product. Only one group license object is created per product and group license objects are never deleted. If a product is unapproved, its group license remains. This allows enterprise admins to keep track of any remaining entitlements for the product.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get grouplicenses](GrouplicenseGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GroupLicense {
    /// How this group license was acquired. "bulkPurchase" means that this Grouplicenses resource was created because the enterprise purchased licenses for this product; otherwise, the value is "free" (for free products).
    #[serde(rename="acquisitionKind")]
    
    pub acquisition_kind: Option<GroupLicenseAcquisitionKindEnum>,
    /// Whether the product to which this group license relates is currently approved by the enterprise. Products are approved when a group license is first created, but this approval may be revoked by an enterprise admin via Google Play. Unapproved products will not be visible to end users in collections, and new entitlements to them should not normally be created.
    
    pub approval: Option<GroupLicenseApprovalEnum>,
    /// The total number of provisioned licenses for this product. Returned by read operations, but ignored in write operations.
    #[serde(rename="numProvisioned")]
    
    pub num_provisioned: Option<i32>,
    /// The number of purchased licenses (possibly in multiple purchases). If this field is omitted, then there is no limit on the number of licenses that can be provisioned (for example, if the acquisition kind is "free").
    #[serde(rename="numPurchased")]
    
    pub num_purchased: Option<i32>,
    /// The permission approval status of the product. This field is only set if the product is approved. Possible states are: - "currentApproved", the current set of permissions is approved, but additional permissions will require the administrator to reapprove the product (If the product was approved without specifying the approved permissions setting, then this is the default behavior.), - "needsReapproval", the product has unapproved permissions. No additional product licenses can be assigned until the product is reapproved, - "allCurrentAndFutureApproved", the current permissions are approved and any future permission updates will be automatically approved without administrator review. 
    
    pub permissions: Option<GroupLicensePermissionsEnum>,
    /// The ID of the product that the license is for. For example, "app:com.google.android.gm".
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
}

impl client::Resource for GroupLicense {}
impl client::ResponseResult for GroupLicense {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list grouplicenseusers](GrouplicenseuserListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GroupLicenseUsersListResponse {
    /// A user of an enterprise.
    
    pub user: Option<Vec<User>>,
}

impl client::ResponseResult for GroupLicenseUsersListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list grouplicenses](GrouplicenseListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GroupLicensesListResponse {
    /// A group license for a product approved for use in the enterprise.
    #[serde(rename="groupLicense")]
    
    pub group_license: Option<Vec<GroupLicense>>,
}

impl client::ResponseResult for GroupLicensesListResponse {}


/// The existence of an Installs resource indicates that an app is installed on a particular device (or that an install is pending). The API can be used to create an install resource using the update method. This triggers the actual install of the app on the device. If the user does not already have an entitlement for the app, then an attempt is made to create one. If this fails (for example, because the app is not free and there is no available license), then the creation of the install fails. The API can also be used to update an installed app. If the update method is used on an existing install, then the app will be updated to the latest available version. Note that it is not possible to force the installation of a specific version of an app: the version code is read-only. If a user installs an app themselves (as permitted by the enterprise), then again an install resource and possibly an entitlement resource are automatically created. The API can also be used to delete an install resource, which triggers the removal of the app from the device. Note that deleting an install does not automatically remove the corresponding entitlement, even if there are no remaining installs. The install resource will also be deleted if the user uninstalls the app themselves.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete installs](InstallDeleteCall) (none)
/// * [get installs](InstallGetCall) (response)
/// * [list installs](InstallListCall) (none)
/// * [update installs](InstallUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Install {
    /// Install state. The state "installPending" means that an install request has recently been made and download to the device is in progress. The state "installed" means that the app has been installed. This field is read-only.
    #[serde(rename="installState")]
    
    pub install_state: Option<InstallInstallStateEnum>,
    /// The ID of the product that the install is for. For example, "app:com.google.android.gm".
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// The version of the installed product. Guaranteed to be set only if the install state is "installed".
    #[serde(rename="versionCode")]
    
    pub version_code: Option<i32>,
}

impl client::RequestValue for Install {}
impl client::Resource for Install {}
impl client::ResponseResult for Install {}


/// An event generated when an app installation failed on a device
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstallFailureEvent {
    /// The Android ID of the device. This field will always be present.
    #[serde(rename="deviceId")]
    
    pub device_id: Option<String>,
    /// Additional details on the failure if applicable.
    #[serde(rename="failureDetails")]
    
    pub failure_details: Option<String>,
    /// The reason for the installation failure. This field will always be present.
    #[serde(rename="failureReason")]
    
    pub failure_reason: Option<InstallFailureEventFailureReasonEnum>,
    /// The id of the product (e.g. "app:com.google.android.gm") for which the install failure event occured. This field will always be present.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// The ID of the user. This field will always be present.
    #[serde(rename="userId")]
    
    pub user_id: Option<String>,
}

impl client::Part for InstallFailureEvent {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list installs](InstallListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstallsListResponse {
    /// An installation of an app for a user on a specific device. The existence of an install implies that the user must have an entitlement to the app.
    
    pub install: Option<Vec<Install>>,
}

impl client::ResponseResult for InstallsListResponse {}


/// Represents a keyed app state containing a key, timestamp, severity level, optional description, and optional data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KeyedAppState {
    /// Additional field intended for machine-readable data. For example, a number or JSON object. To prevent XSS, we recommend removing any HTML from the data before displaying it.
    
    pub data: Option<String>,
    /// Key indicating what the app is providing a state for. The content of the key is set by the app's developer. To prevent XSS, we recommend removing any HTML from the key before displaying it. This field will always be present.
    
    pub key: Option<String>,
    /// Free-form, human-readable message describing the app state. For example, an error message. To prevent XSS, we recommend removing any HTML from the message before displaying it.
    
    pub message: Option<String>,
    /// Severity of the app state. This field will always be present.
    
    pub severity: Option<KeyedAppStateSeverityEnum>,
    /// Timestamp of when the app set the state in milliseconds since epoch. This field will always be present.
    #[serde(rename="stateTimestampMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub state_timestamp_millis: Option<i64>,
}

impl client::Part for KeyedAppState {}


/// A localized string with its locale.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocalizedText {
    /// The BCP47 tag for a locale. (e.g. "en-US", "de").
    
    pub locale: Option<String>,
    /// The text localized in the associated locale.
    
    pub text: Option<String>,
}

impl client::Part for LocalizedText {}


/// Maintenance window for managed Google Play Accounts. This allows Play store to update the apps on the foreground in the designated window.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MaintenanceWindow {
    /// Duration of the maintenance window, in milliseconds. The duration must be between 30 minutes and 24 hours (inclusive).
    #[serde(rename="durationMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub duration_ms: Option<i64>,
    /// Start time of the maintenance window, in milliseconds after midnight on the device. Windows can span midnight.
    #[serde(rename="startTimeAfterMidnightMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start_time_after_midnight_ms: Option<i64>,
}

impl client::Part for MaintenanceWindow {}


/// A managed configuration resource contains the set of managed properties defined by the app developer in the app’s managed configurations schema, as well as any configuration variables defined for the user.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get managedconfigurationsfordevice](ManagedconfigurationsfordeviceGetCall) (response)
/// * [update managedconfigurationsfordevice](ManagedconfigurationsfordeviceUpdateCall) (request|response)
/// * [get managedconfigurationsforuser](ManagedconfigurationsforuserGetCall) (response)
/// * [update managedconfigurationsforuser](ManagedconfigurationsforuserUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedConfiguration {
    /// Contains the ID of the managed configuration profile and the set of configuration variables (if any) defined for the user.
    #[serde(rename="configurationVariables")]
    
    pub configuration_variables: Option<ConfigurationVariables>,
    /// Deprecated.
    
    pub kind: Option<String>,
    /// The set of managed properties for this configuration.
    #[serde(rename="managedProperty")]
    
    pub managed_property: Option<Vec<ManagedProperty>>,
    /// The ID of the product that the managed configuration is for, e.g. "app:com.google.android.gm".
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
}

impl client::RequestValue for ManagedConfiguration {}
impl client::ResponseResult for ManagedConfiguration {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list managedconfigurationsfordevice](ManagedconfigurationsfordeviceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedConfigurationsForDeviceListResponse {
    /// A managed configuration for an app on a specific device.
    #[serde(rename="managedConfigurationForDevice")]
    
    pub managed_configuration_for_device: Option<Vec<ManagedConfiguration>>,
}

impl client::ResponseResult for ManagedConfigurationsForDeviceListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list managedconfigurationsforuser](ManagedconfigurationsforuserListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedConfigurationsForUserListResponse {
    /// A managed configuration for an app for a specific user.
    #[serde(rename="managedConfigurationForUser")]
    
    pub managed_configuration_for_user: Option<Vec<ManagedConfiguration>>,
}

impl client::ResponseResult for ManagedConfigurationsForUserListResponse {}


/// A managed configurations settings resource contains the set of managed properties that have been configured for an Android app to be applied to a set of users. The app's developer would have defined configurable properties in the managed configurations schema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedConfigurationsSettings {
    /// The last updated time of the managed configuration settings in milliseconds since 1970-01-01T00:00:00Z.
    #[serde(rename="lastUpdatedTimestampMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_updated_timestamp_millis: Option<i64>,
    /// The ID of the managed configurations settings.
    #[serde(rename="mcmId")]
    
    pub mcm_id: Option<String>,
    /// The name of the managed configurations settings.
    
    pub name: Option<String>,
}

impl client::Part for ManagedConfigurationsSettings {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list managedconfigurationssettings](ManagedconfigurationssettingListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedConfigurationsSettingsListResponse {
    /// A managed configurations settings for an app that may be assigned to a group of users in an enterprise.
    #[serde(rename="managedConfigurationsSettings")]
    
    pub managed_configurations_settings: Option<Vec<ManagedConfigurationsSettings>>,
}

impl client::ResponseResult for ManagedConfigurationsSettingsListResponse {}


/// A managed property of a managed configuration. The property must match one of the properties in the app restrictions schema of the product. Exactly one of the value fields must be populated, and it must match the property's type in the app restrictions schema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedProperty {
    /// The unique key that identifies the property.
    
    pub key: Option<String>,
    /// The boolean value - this will only be present if type of the property is bool.
    #[serde(rename="valueBool")]
    
    pub value_bool: Option<bool>,
    /// The bundle of managed properties - this will only be present if type of the property is bundle.
    #[serde(rename="valueBundle")]
    
    pub value_bundle: Option<ManagedPropertyBundle>,
    /// The list of bundles of properties - this will only be present if type of the property is bundle_array.
    #[serde(rename="valueBundleArray")]
    
    pub value_bundle_array: Option<Vec<ManagedPropertyBundle>>,
    /// The integer value - this will only be present if type of the property is integer.
    #[serde(rename="valueInteger")]
    
    pub value_integer: Option<i32>,
    /// The string value - this will only be present if type of the property is string, choice or hidden.
    #[serde(rename="valueString")]
    
    pub value_string: Option<String>,
    /// The list of string values - this will only be present if type of the property is multiselect.
    #[serde(rename="valueStringArray")]
    
    pub value_string_array: Option<Vec<String>>,
}

impl client::Part for ManagedProperty {}


/// A bundle of managed properties.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedPropertyBundle {
    /// The list of managed properties.
    #[serde(rename="managedProperty")]
    
    pub managed_property: Option<Vec<ManagedProperty>>,
}

impl client::Part for ManagedPropertyBundle {}


/// An event generated when a new device is ready to be managed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NewDeviceEvent {
    /// The Android ID of the device. This field will always be present.
    #[serde(rename="deviceId")]
    
    pub device_id: Option<String>,
    /// Policy app on the device.
    #[serde(rename="dpcPackageName")]
    
    pub dpc_package_name: Option<String>,
    /// Identifies the extent to which the device is controlled by an Android EMM in various deployment configurations. Possible values include: - "managedDevice", a device where the DPC is set as device owner, - "managedProfile", a device where the DPC is set as profile owner. 
    #[serde(rename="managementType")]
    
    pub management_type: Option<NewDeviceEventManagementTypeEnum>,
    /// The ID of the user. This field will always be present.
    #[serde(rename="userId")]
    
    pub user_id: Option<String>,
}

impl client::Part for NewDeviceEvent {}


/// An event generated when new permissions are added to an app.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NewPermissionsEvent {
    /// The set of permissions that the enterprise admin has already approved for this application. Use Permissions.Get on the EMM API to retrieve details about these permissions.
    #[serde(rename="approvedPermissions")]
    
    pub approved_permissions: Option<Vec<String>>,
    /// The id of the product (e.g. "app:com.google.android.gm") for which new permissions were added. This field will always be present.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// The set of permissions that the app is currently requesting. Use Permissions.Get on the EMM API to retrieve details about these permissions.
    #[serde(rename="requestedPermissions")]
    
    pub requested_permissions: Option<Vec<String>>,
}

impl client::Part for NewPermissionsEvent {}


/// A notification of one event relating to an enterprise.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Notification {
    /// Notifications about new app restrictions schema changes.
    #[serde(rename="appRestrictionsSchemaChangeEvent")]
    
    pub app_restrictions_schema_change_event: Option<AppRestrictionsSchemaChangeEvent>,
    /// Notifications about app updates.
    #[serde(rename="appUpdateEvent")]
    
    pub app_update_event: Option<AppUpdateEvent>,
    /// Notifications about device report updates.
    #[serde(rename="deviceReportUpdateEvent")]
    
    pub device_report_update_event: Option<DeviceReportUpdateEvent>,
    /// The ID of the enterprise for which the notification is sent. This will always be present.
    #[serde(rename="enterpriseId")]
    
    pub enterprise_id: Option<String>,
    /// Notifications about an app installation failure.
    #[serde(rename="installFailureEvent")]
    
    pub install_failure_event: Option<InstallFailureEvent>,
    /// Notifications about new devices.
    #[serde(rename="newDeviceEvent")]
    
    pub new_device_event: Option<NewDeviceEvent>,
    /// Notifications about new app permissions.
    #[serde(rename="newPermissionsEvent")]
    
    pub new_permissions_event: Option<NewPermissionsEvent>,
    /// Type of the notification.
    #[serde(rename="notificationType")]
    
    pub notification_type: Option<NotificationNotificationTypeEnum>,
    /// Notifications about changes to a product's approval status.
    #[serde(rename="productApprovalEvent")]
    
    pub product_approval_event: Option<ProductApprovalEvent>,
    /// Notifications about product availability changes.
    #[serde(rename="productAvailabilityChangeEvent")]
    
    pub product_availability_change_event: Option<ProductAvailabilityChangeEvent>,
    /// The time when the notification was published in milliseconds since 1970-01-01T00:00:00Z. This will always be present.
    #[serde(rename="timestampMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub timestamp_millis: Option<i64>,
}

impl client::Part for Notification {}


/// A resource returned by the PullNotificationSet API, which contains a collection of notifications for enterprises associated with the service account authenticated for the request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [pull notification set enterprises](EnterprisePullNotificationSetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NotificationSet {
    /// The notifications received, or empty if no notifications are present.
    
    pub notification: Option<Vec<Notification>>,
    /// The notification set ID, required to mark the notification as received with the Enterprises.AcknowledgeNotification API. This will be omitted if no notifications are present.
    #[serde(rename="notificationSetId")]
    
    pub notification_set_id: Option<String>,
}

impl client::ResponseResult for NotificationSet {}


/// Information about the current page. List operations that supports paging return only one "page" of results. This protocol buffer message describes the page that has been returned.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PageInfo {
    /// Maximum number of results returned in one page. ! The number of results included in the API response.
    #[serde(rename="resultPerPage")]
    
    pub result_per_page: Option<i32>,
    /// Index of the first result returned in the current page.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// Total number of results available on the backend ! The total number of results in the result set.
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
}

impl client::Part for PageInfo {}


/// A Permissions resource represents some extra capability, to be granted to an Android app, which requires explicit consent. An enterprise admin must consent to these permissions on behalf of their users before an entitlement for the app can be created. The permissions collection is read-only. The information provided for each permission (localized name and description) is intended to be used in the MDM user interface when obtaining consent from the enterprise.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get permissions](PermissionGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Permission {
    /// A longer description of the Permissions resource, giving more details of what it affects.
    
    pub description: Option<String>,
    /// The name of the permission.
    
    pub name: Option<String>,
    /// An opaque string uniquely identifying the permission.
    #[serde(rename="permissionId")]
    
    pub permission_id: Option<String>,
}

impl client::Resource for Permission {}
impl client::ResponseResult for Permission {}


/// The device policy for a given managed device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// Recommended alternative: autoUpdateMode which is set per app, provides greater flexibility around update frequency. When autoUpdateMode is set to AUTO_UPDATE_POSTPONED or AUTO_UPDATE_HIGH_PRIORITY, this field has no effect. "choiceToTheUser" allows the device's user to configure the app update policy. "always" enables auto updates. "never" disables auto updates. "wifiOnly" enables auto updates only when the device is connected to wifi.
    #[serde(rename="autoUpdatePolicy")]
    
    pub auto_update_policy: Option<PolicyAutoUpdatePolicyEnum>,
    /// Whether the device reports app states to the EMM. The default value is "deviceReportDisabled".
    #[serde(rename="deviceReportPolicy")]
    
    pub device_report_policy: Option<PolicyDeviceReportPolicyEnum>,
    /// The maintenance window defining when apps running in the foreground should be updated.
    #[serde(rename="maintenanceWindow")]
    
    pub maintenance_window: Option<MaintenanceWindow>,
    /// The availability granted to the device for the specified products. "all" gives the device access to all products, regardless of approval status. "all" does not enable automatic visibility of "alpha" or "beta" tracks. "whitelist" grants the device access the products specified in productPolicy[]. Only products that are approved or products that were previously approved (products with revoked approval) by the enterprise can be whitelisted. If no value is provided, the availability set at the user level is applied by default.
    #[serde(rename="productAvailabilityPolicy")]
    
    pub product_availability_policy: Option<PolicyProductAvailabilityPolicyEnum>,
    /// The list of product policies. The productAvailabilityPolicy needs to be set to WHITELIST or ALL for the product policies to be applied.
    #[serde(rename="productPolicy")]
    
    pub product_policy: Option<Vec<ProductPolicy>>,
}

impl client::Part for Policy {}


/// A Products resource represents an app in the Google Play store that is available to at least some users in the enterprise. (Some apps are restricted to a single enterprise, and no information about them is made available outside that enterprise.) The information provided for each product (localized name, icon, link to the full Google Play details page) is intended to allow a basic representation of the product within an EMM user interface.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [approve products](ProductApproveCall) (none)
/// * [generate approval url products](ProductGenerateApprovalUrlCall) (none)
/// * [get products](ProductGetCall) (response)
/// * [get app restrictions schema products](ProductGetAppRestrictionsSchemaCall) (none)
/// * [get permissions products](ProductGetPermissionCall) (none)
/// * [list products](ProductListCall) (none)
/// * [unapprove products](ProductUnapproveCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Product {
    /// The app restriction schema
    #[serde(rename="appRestrictionsSchema")]
    
    pub app_restrictions_schema: Option<AppRestrictionsSchema>,
    /// The tracks visible to the enterprise.
    #[serde(rename="appTracks")]
    
    pub app_tracks: Option<Vec<TrackInfo>>,
    /// App versions currently available for this product.
    #[serde(rename="appVersion")]
    
    pub app_version: Option<Vec<AppVersion>>,
    /// The name of the author of the product (for example, the app developer).
    #[serde(rename="authorName")]
    
    pub author_name: Option<String>,
    /// The countries which this app is available in.
    #[serde(rename="availableCountries")]
    
    pub available_countries: Option<Vec<String>>,
    /// Deprecated, use appTracks instead.
    #[serde(rename="availableTracks")]
    
    pub available_tracks: Option<Vec<ProductAvailableTracksEnum>>,
    /// The app category (e.g. RACING, SOCIAL, etc.)
    
    pub category: Option<String>,
    /// The content rating for this app.
    #[serde(rename="contentRating")]
    
    pub content_rating: Option<ProductContentRatingEnum>,
    /// The localized promotional description, if available.
    
    pub description: Option<String>,
    /// A link to the (consumer) Google Play details page for the product.
    #[serde(rename="detailsUrl")]
    
    pub details_url: Option<String>,
    /// How and to whom the package is made available. The value publicGoogleHosted means that the package is available through the Play store and not restricted to a specific enterprise. The value privateGoogleHosted means that the package is a private app (restricted to an enterprise) but hosted by Google. The value privateSelfHosted means that the package is a private app (restricted to an enterprise) and is privately hosted.
    #[serde(rename="distributionChannel")]
    
    pub distribution_channel: Option<ProductDistributionChannelEnum>,
    /// Noteworthy features (if any) of this product.
    
    pub features: Option<Vec<ProductFeaturesEnum>>,
    /// A link to an image that can be used as an icon for the product. This image is suitable for use at up to 512px x 512px.
    #[serde(rename="iconUrl")]
    
    pub icon_url: Option<String>,
    /// The approximate time (within 7 days) the app was last published, expressed in milliseconds since epoch.
    #[serde(rename="lastUpdatedTimestampMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_updated_timestamp_millis: Option<i64>,
    /// The minimum Android SDK necessary to run the app.
    #[serde(rename="minAndroidSdkVersion")]
    
    pub min_android_sdk_version: Option<i32>,
    /// A list of permissions required by the app.
    
    pub permissions: Option<Vec<ProductPermission>>,
    /// A string of the form *app:<package name>*. For example, app:com.google.android.gm represents the Gmail app.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// Whether this product is free, free with in-app purchases, or paid. If the pricing is unknown, this means the product is not generally available anymore (even though it might still be available to people who own it).
    #[serde(rename="productPricing")]
    
    pub product_pricing: Option<ProductProductPricingEnum>,
    /// A description of the recent changes made to the app.
    #[serde(rename="recentChanges")]
    
    pub recent_changes: Option<String>,
    /// Deprecated.
    #[serde(rename="requiresContainerApp")]
    
    pub requires_container_app: Option<bool>,
    /// A list of screenshot links representing the app.
    #[serde(rename="screenshotUrls")]
    
    pub screenshot_urls: Option<Vec<String>>,
    /// The certificate used to sign this product.
    #[serde(rename="signingCertificate")]
    
    pub signing_certificate: Option<ProductSigningCertificate>,
    /// A link to a smaller image that can be used as an icon for the product. This image is suitable for use at up to 128px x 128px.
    #[serde(rename="smallIconUrl")]
    
    pub small_icon_url: Option<String>,
    /// The name of the product.
    
    pub title: Option<String>,
    /// A link to the managed Google Play details page for the product, for use by an Enterprise admin.
    #[serde(rename="workDetailsUrl")]
    
    pub work_details_url: Option<String>,
}

impl client::Resource for Product {}
impl client::ResponseResult for Product {}


/// An event generated when a product's approval status is changed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductApprovalEvent {
    /// Whether the product was approved or unapproved. This field will always be present.
    
    pub approved: Option<ProductApprovalEventApprovedEnum>,
    /// The id of the product (e.g. "app:com.google.android.gm") for which the approval status has changed. This field will always be present.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
}

impl client::Part for ProductApprovalEvent {}


/// An event generated whenever a product's availability changes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductAvailabilityChangeEvent {
    /// The new state of the product. This field will always be present.
    #[serde(rename="availabilityStatus")]
    
    pub availability_status: Option<ProductAvailabilityChangeEventAvailabilityStatusEnum>,
    /// The id of the product (e.g. "app:com.google.android.gm") for which the product availability changed. This field will always be present.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
}

impl client::Part for ProductAvailabilityChangeEvent {}


/// A product permissions resource represents the set of permissions required by a specific app and whether or not they have been accepted by an enterprise admin. The API can be used to read the set of permissions, and also to update the set to indicate that permissions have been accepted.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductPermission {
    /// An opaque string uniquely identifying the permission.
    #[serde(rename="permissionId")]
    
    pub permission_id: Option<String>,
    /// Whether the permission has been accepted or not.
    
    pub state: Option<ProductPermissionStateEnum>,
}

impl client::Part for ProductPermission {}


/// Information about the permissions required by a specific app and whether they have been accepted by the enterprise.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get permissions products](ProductGetPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductPermissions {
    /// The permissions required by the app.
    
    pub permission: Option<Vec<ProductPermission>>,
    /// The ID of the app that the permissions relate to, e.g. "app:com.google.android.gm".
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
}

impl client::ResponseResult for ProductPermissions {}


/// The policy for a product.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductPolicy {
    /// The auto-install policy for the product.
    #[serde(rename="autoInstallPolicy")]
    
    pub auto_install_policy: Option<AutoInstallPolicy>,
    /// The auto-update mode for the product.
    #[serde(rename="autoUpdateMode")]
    
    pub auto_update_mode: Option<ProductPolicyAutoUpdateModeEnum>,
    /// An authentication URL configuration for the authenticator app of an identity provider. This helps to launch the identity provider's authenticator app during the authentication happening in a private app using Android WebView. Authenticator app should already be the [default handler](https://developer.android.com/training/app-links/verify-site-associations) for the authentication url on the device.
    #[serde(rename="enterpriseAuthenticationAppLinkConfigs")]
    
    pub enterprise_authentication_app_link_configs: Option<Vec<EnterpriseAuthenticationAppLinkConfig>>,
    /// The managed configuration for the product.
    #[serde(rename="managedConfiguration")]
    
    pub managed_configuration: Option<ManagedConfiguration>,
    /// The ID of the product. For example, "app:com.google.android.gm".
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// Grants the device visibility to the specified product release track(s), identified by trackIds. The list of release tracks of a product can be obtained by calling Products.Get.
    #[serde(rename="trackIds")]
    
    pub track_ids: Option<Vec<String>>,
    /// Deprecated. Use trackIds instead.
    
    pub tracks: Option<Vec<ProductPolicyTracksEnum>>,
}

impl client::Part for ProductPolicy {}


/// A set of products.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get available product set users](UserGetAvailableProductSetCall) (response)
/// * [set available product set users](UserSetAvailableProductSetCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductSet {
    /// The list of product IDs making up the set of products.
    #[serde(rename="productId")]
    
    pub product_id: Option<Vec<String>>,
    /// The interpretation of this product set. "unknown" should never be sent and is ignored if received. "whitelist" means that the user is entitled to access the product set. "includeAll" means that all products are accessible, including products that are approved, products with revoked approval, and products that have never been approved. "allApproved" means that the user is entitled to access all products that are approved for the enterprise. If the value is "allApproved" or "includeAll", the productId field is ignored. If no value is provided, it is interpreted as "whitelist" for backwards compatibility. Further "allApproved" or "includeAll" does not enable automatic visibility of "alpha" or "beta" tracks for Android app. Use ProductVisibility to enable "alpha" or "beta" tracks per user.
    #[serde(rename="productSetBehavior")]
    
    pub product_set_behavior: Option<ProductSetProductSetBehaviorEnum>,
    /// Additional list of product IDs making up the product set. Unlike the productID array, in this list It's possible to specify which tracks (alpha, beta, production) of a product are visible to the user. See ProductVisibility and its fields for more information. Specifying the same product ID both here and in the productId array is not allowed and it will result in an error.
    #[serde(rename="productVisibility")]
    
    pub product_visibility: Option<Vec<ProductVisibility>>,
}

impl client::RequestValue for ProductSet {}
impl client::ResponseResult for ProductSet {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductSigningCertificate {
    /// The base64 urlsafe encoded SHA1 hash of the certificate. (This field is deprecated in favor of SHA2-256. It should not be used and may be removed at any time.)
    #[serde(rename="certificateHashSha1")]
    
    pub certificate_hash_sha1: Option<String>,
    /// The base64 urlsafe encoded SHA2-256 hash of the certificate.
    #[serde(rename="certificateHashSha256")]
    
    pub certificate_hash_sha256: Option<String>,
}

impl client::Part for ProductSigningCertificate {}


/// A product to be made visible to a user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductVisibility {
    /// The product ID to make visible to the user. Required for each item in the productVisibility list.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// Grants the user visibility to the specified product track(s), identified by trackIds.
    #[serde(rename="trackIds")]
    
    pub track_ids: Option<Vec<String>>,
    /// Deprecated. Use trackIds instead.
    
    pub tracks: Option<Vec<ProductVisibilityTracksEnum>>,
}

impl client::Part for ProductVisibility {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [approve products](ProductApproveCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductsApproveRequest {
    /// The approval URL that was shown to the user. Only the permissions shown to the user with that URL will be accepted, which may not be the product's entire set of permissions. For example, the URL may only display new permissions from an update after the product was approved, or not include new permissions if the product was updated since the URL was generated.
    #[serde(rename="approvalUrlInfo")]
    
    pub approval_url_info: Option<ApprovalUrlInfo>,
    /// Sets how new permission requests for the product are handled. "allPermissions" automatically approves all current and future permissions for the product. "currentPermissionsOnly" approves the current set of permissions for the product, but any future permissions added through updates will require manual reapproval. If not specified, only the current set of permissions will be approved.
    #[serde(rename="approvedPermissions")]
    
    pub approved_permissions: Option<ProductsApproveRequestApprovedPermissionsEnum>,
}

impl client::RequestValue for ProductsApproveRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [generate approval url products](ProductGenerateApprovalUrlCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductsGenerateApprovalUrlResponse {
    /// A URL that can be rendered in an iframe to display the permissions (if any) of a product. This URL can be used to approve the product only once and only within 24 hours of being generated, using the Products.approve call. If the product is currently unapproved and has no permissions, this URL will point to an empty page. If the product is currently approved, a URL will only be generated if that product has added permissions since it was last approved, and the URL will only display those new permissions that have not yet been accepted.
    
    pub url: Option<String>,
}

impl client::ResponseResult for ProductsGenerateApprovalUrlResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list products](ProductListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductsListResponse {
    /// General pagination information.
    #[serde(rename="pageInfo")]
    
    pub page_info: Option<PageInfo>,
    /// Information about a product (e.g. an app) in the Google Play store, for display to an enterprise admin.
    
    pub product: Option<Vec<Product>>,
    /// Pagination information for token pagination.
    #[serde(rename="tokenPagination")]
    
    pub token_pagination: Option<TokenPagination>,
}

impl client::ResponseResult for ProductsListResponse {}


/// A service account identity, including the name and credentials that can be used to authenticate as the service account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get service account enterprises](EnterpriseGetServiceAccountCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceAccount {
    /// Credentials that can be used to authenticate as this ServiceAccount.
    
    pub key: Option<ServiceAccountKey>,
    /// The account name of the service account, in the form of an email address. Assigned by the server.
    
    pub name: Option<String>,
}

impl client::ResponseResult for ServiceAccount {}


/// Credentials that can be used to authenticate as a service account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert serviceaccountkeys](ServiceaccountkeyInsertCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceAccountKey {
    /// The body of the private key credentials file, in string format. This is only populated when the ServiceAccountKey is created, and is not stored by Google.
    
    pub data: Option<String>,
    /// An opaque, unique identifier for this ServiceAccountKey. Assigned by the server.
    
    pub id: Option<String>,
    /// Public key data for the credentials file. This is an X.509 cert. If you are using the googleCredentials key type, this is identical to the cert that can be retrieved by using the X.509 cert url inside of the credentials file.
    #[serde(rename="publicData")]
    
    pub public_data: Option<String>,
    /// The file format of the generated key data.
    #[serde(rename="type")]
    
    pub type_: Option<ServiceAccountKeyTypeEnum>,
}

impl client::RequestValue for ServiceAccountKey {}
impl client::Resource for ServiceAccountKey {}
impl client::ResponseResult for ServiceAccountKey {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list serviceaccountkeys](ServiceaccountkeyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceAccountKeysListResponse {
    /// The service account credentials.
    #[serde(rename="serviceAccountKey")]
    
    pub service_account_key: Option<Vec<ServiceAccountKey>>,
}

impl client::ResponseResult for ServiceAccountKeysListResponse {}


/// A resource returned by the GenerateSignupUrl API, which contains the Signup URL and Completion Token.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [generate signup url enterprises](EnterpriseGenerateSignupUrlCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SignupInfo {
    /// An opaque token that will be required, along with the Enterprise Token, for obtaining the enterprise resource from CompleteSignup.
    #[serde(rename="completionToken")]
    
    pub completion_token: Option<String>,
    /// Deprecated.
    
    pub kind: Option<String>,
    /// A URL under which the Admin can sign up for an enterprise. The page pointed to cannot be rendered in an iframe.
    
    pub url: Option<String>,
}

impl client::ResponseResult for SignupInfo {}


/// Definition of a managed Google Play store cluster, a list of products displayed as part of a store page.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get storelayoutclusters](StorelayoutclusterGetCall) (response)
/// * [insert storelayoutclusters](StorelayoutclusterInsertCall) (request|response)
/// * [update storelayoutclusters](StorelayoutclusterUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StoreCluster {
    /// Unique ID of this cluster. Assigned by the server. Immutable once assigned.
    
    pub id: Option<String>,
    /// Ordered list of localized strings giving the name of this page. The text displayed is the one that best matches the user locale, or the first entry if there is no good match. There needs to be at least one entry.
    
    pub name: Option<Vec<LocalizedText>>,
    /// String (US-ASCII only) used to determine order of this cluster within the parent page's elements. Page elements are sorted in lexicographic order of this field. Duplicated values are allowed, but ordering between elements with duplicate order is undefined. The value of this field is never visible to a user, it is used solely for the purpose of defining an ordering. Maximum length is 256 characters.
    #[serde(rename="orderInPage")]
    
    pub order_in_page: Option<String>,
    /// List of products in the order they are displayed in the cluster. There should not be duplicates within a cluster.
    #[serde(rename="productId")]
    
    pub product_id: Option<Vec<String>>,
}

impl client::RequestValue for StoreCluster {}
impl client::ResponseResult for StoreCluster {}


/// General setting for the managed Google Play store layout, currently only specifying the page to display the first time the store is opened.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get store layout enterprises](EnterpriseGetStoreLayoutCall) (response)
/// * [set store layout enterprises](EnterpriseSetStoreLayoutCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StoreLayout {
    /// The ID of the store page to be used as the homepage. The homepage is the first page shown in the managed Google Play Store. Not specifying a homepage is equivalent to setting the store layout type to "basic".
    #[serde(rename="homepageId")]
    
    pub homepage_id: Option<String>,
    /// The store layout type. By default, this value is set to "basic" if the homepageId field is not set, and to "custom" otherwise. If set to "basic", the layout will consist of all approved apps that have been whitelisted for the user.
    #[serde(rename="storeLayoutType")]
    
    pub store_layout_type: Option<StoreLayoutStoreLayoutTypeEnum>,
}

impl client::RequestValue for StoreLayout {}
impl client::ResponseResult for StoreLayout {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list storelayoutclusters](StorelayoutclusterListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StoreLayoutClustersListResponse {
    /// A store cluster of an enterprise.
    
    pub cluster: Option<Vec<StoreCluster>>,
}

impl client::ResponseResult for StoreLayoutClustersListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list storelayoutpages](StorelayoutpageListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StoreLayoutPagesListResponse {
    /// A store page of an enterprise.
    
    pub page: Option<Vec<StorePage>>,
}

impl client::ResponseResult for StoreLayoutPagesListResponse {}


/// Definition of a managed Google Play store page, made of a localized name and links to other pages. A page also contains clusters defined as a subcollection.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get storelayoutpages](StorelayoutpageGetCall) (response)
/// * [insert storelayoutpages](StorelayoutpageInsertCall) (request|response)
/// * [update storelayoutpages](StorelayoutpageUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StorePage {
    /// Unique ID of this page. Assigned by the server. Immutable once assigned.
    
    pub id: Option<String>,
    /// Ordered list of pages a user should be able to reach from this page. The list can't include this page. It is recommended that the basic pages are created first, before adding the links between pages. The API doesn't verify that the pages exist or the pages are reachable.
    
    pub link: Option<Vec<String>>,
    /// Ordered list of localized strings giving the name of this page. The text displayed is the one that best matches the user locale, or the first entry if there is no good match. There needs to be at least one entry.
    
    pub name: Option<Vec<LocalizedText>>,
}

impl client::RequestValue for StorePage {}
impl client::ResponseResult for StorePage {}


/// Pagination information returned by a List operation when token pagination is enabled. List operations that supports paging return only one "page" of results. This protocol buffer message describes the page that has been returned. When using token pagination, clients should use the next/previous token to get another page of the result. The presence or absence of next/previous token indicates whether a next/previous page is available and provides a mean of accessing this page. ListRequest.page_token should be set to either next_page_token or previous_page_token to access another page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TokenPagination {
    /// Tokens to pass to the standard list field 'page_token'. Whenever available, tokens are preferred over manipulating start_index.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// no description provided
    #[serde(rename="previousPageToken")]
    
    pub previous_page_token: Option<String>,
}

impl client::Part for TokenPagination {}


/// Id to name association of a track.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TrackInfo {
    /// A modifiable name for a track. This is the visible name in the play developer console.
    #[serde(rename="trackAlias")]
    
    pub track_alias: Option<String>,
    /// Unmodifiable, unique track identifier. This identifier is the releaseTrackId in the url of the play developer console page that displays the track information.
    #[serde(rename="trackId")]
    
    pub track_id: Option<String>,
}

impl client::Part for TrackInfo {}


/// A Users resource represents an account associated with an enterprise. The account may be specific to a device or to an individual user (who can then use the account across multiple devices). The account may provide access to managed Google Play only, or to other Google services, depending on the identity model: - The Google managed domain identity model requires synchronization to Google account sources (via primaryEmail). - The managed Google Play Accounts identity model provides a dynamic means for enterprises to create user or device accounts as needed. These accounts provide access to managed Google Play. 
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete users](UserDeleteCall) (none)
/// * [generate authentication token users](UserGenerateAuthenticationTokenCall) (none)
/// * [get users](UserGetCall) (response)
/// * [get available product set users](UserGetAvailableProductSetCall) (none)
/// * [insert users](UserInsertCall) (request|response)
/// * [list users](UserListCall) (none)
/// * [revoke device access users](UserRevokeDeviceAccesCall) (none)
/// * [set available product set users](UserSetAvailableProductSetCall) (none)
/// * [update users](UserUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// A unique identifier you create for this user, such as "user342" or "asset#44418". Do not use personally identifiable information (PII) for this property. Must always be set for EMM-managed users. Not set for Google-managed users.
    #[serde(rename="accountIdentifier")]
    
    pub account_identifier: Option<String>,
    /// The type of account that this user represents. A userAccount can be installed on multiple devices, but a deviceAccount is specific to a single device. An EMM-managed user (emmManaged) can be either type (userAccount, deviceAccount), but a Google-managed user (googleManaged) is always a userAccount.
    #[serde(rename="accountType")]
    
    pub account_type: Option<UserAccountTypeEnum>,
    /// The name that will appear in user interfaces. Setting this property is optional when creating EMM-managed users. If you do set this property, use something generic about the organization (such as "Example, Inc.") or your name (as EMM). Not used for Google-managed user accounts. @mutable androidenterprise.users.update
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The unique ID for the user.
    
    pub id: Option<String>,
    /// The entity that manages the user. With googleManaged users, the source of truth is Google so EMMs have to make sure a Google Account exists for the user. With emmManaged users, the EMM is in charge.
    #[serde(rename="managementType")]
    
    pub management_type: Option<UserManagementTypeEnum>,
    /// The user's primary email address, for example, "jsmith@example.com". Will always be set for Google managed users and not set for EMM managed users.
    #[serde(rename="primaryEmail")]
    
    pub primary_email: Option<String>,
}

impl client::RequestValue for User {}
impl client::Resource for User {}
impl client::ResponseResult for User {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list users](UserListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsersListResponse {
    /// A user of an enterprise.
    
    pub user: Option<Vec<User>>,
}

impl client::ResponseResult for UsersListResponse {}


/// A variable set is a key-value pair of EMM-provided placeholders and its corresponding value, which is attributed to a user. For example, $FIRSTNAME could be a placeholder, and its value could be Alice. Placeholders should start with a '$' sign and should be alphanumeric only.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VariableSet {
    /// The placeholder string; defined by EMM.
    
    pub placeholder: Option<String>,
    /// The value of the placeholder, specific to the user.
    #[serde(rename="userValue")]
    
    pub user_value: Option<String>,
}

impl client::Part for VariableSet {}


/// A WebApps resource represents a web app created for an enterprise. Web apps are published to managed Google Play and can be distributed like other Android apps. On a user’s device, a web app opens its specified URL.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get webapps](WebappGetCall) (response)
/// * [insert webapps](WebappInsertCall) (request|response)
/// * [update webapps](WebappUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebApp {
    /// The display mode of the web app. Possible values include: - "minimalUi", the device's status bar, navigation bar, the app's URL, and a refresh button are visible when the app is open. For HTTP URLs, you can only select this option. - "standalone", the device's status bar and navigation bar are visible when the app is open. - "fullScreen", the app opens in full screen mode, hiding the device's status and navigation bars. All browser UI elements, page URL, system status bar and back button are not visible, and the web app takes up the entirety of the available display area. 
    #[serde(rename="displayMode")]
    
    pub display_mode: Option<WebAppDisplayModeEnum>,
    /// A list of icons representing this website. If absent, a default icon (for create) or the current icon (for update) will be used.
    
    pub icons: Option<Vec<WebAppIcon>>,
    /// A flag whether the app has been published to the Play store yet.
    #[serde(rename="isPublished")]
    
    pub is_published: Option<bool>,
    /// The start URL, i.e. the URL that should load when the user opens the application.
    #[serde(rename="startUrl")]
    
    pub start_url: Option<String>,
    /// The title of the web app as displayed to the user (e.g., amongst a list of other applications, or as a label for an icon).
    
    pub title: Option<String>,
    /// The current version of the app. Note that the version can automatically increase during the lifetime of the web app, while Google does internal housekeeping to keep the web app up-to-date.
    #[serde(rename="versionCode")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version_code: Option<i64>,
    /// The ID of the application. A string of the form "app:<package name>" where the package name always starts with the prefix "com.google.enterprise.webapp." followed by a random id.
    #[serde(rename="webAppId")]
    
    pub web_app_id: Option<String>,
}

impl client::RequestValue for WebApp {}
impl client::Resource for WebApp {}
impl client::ResponseResult for WebApp {}


/// Icon for a web app.
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


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list webapps](WebappListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebAppsListResponse {
    /// The manifest describing a web app.
    #[serde(rename="webApp")]
    
    pub web_app: Option<Vec<WebApp>>,
}

impl client::ResponseResult for WebAppsListResponse {}


