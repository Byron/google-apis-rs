use super::*;
/// The request for creating an IdpCredential with its associated payload. An InboundSamlSsoProfile can own up to 2 credentials.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [idp credentials add inbound saml sso profiles](InboundSamlSsoProfileIdpCredentialAddCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddIdpCredentialRequest {
    /// PEM encoded x509 certificate containing the public key for verifying IdP signatures.
    #[serde(rename="pemData")]
    
    pub pem_data: Option<String>,
}

impl client::RequestValue for AddIdpCredentialRequest {}


/// Request to cancel sent invitation for target email in UserInvitation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [userinvitations cancel customers](CustomerUserinvitationCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelUserInvitationRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelUserInvitationRequest {}


/// The response message for MembershipsService.CheckTransitiveMembership.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [memberships check transitive membership groups](GroupMembershipCheckTransitiveMembershipCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CheckTransitiveMembershipResponse {
    /// Response does not include the possible roles of a member since the behavior of this rpc is not all-or-nothing unlike the other rpcs. So, it may not be possible to list all the roles definitively, due to possible lack of authorization in some of the paths.
    #[serde(rename="hasMembership")]
    
    pub has_membership: Option<bool>,
}

impl client::ResponseResult for CheckTransitiveMembershipResponse {}


/// Information of a DSA public key.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DsaPublicKeyInfo {
    /// Key size in bits (size of parameter P).
    #[serde(rename="keySize")]
    
    pub key_size: Option<i32>,
}

impl client::Part for DsaPublicKeyInfo {}


/// Dynamic group metadata like queries and status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicGroupMetadata {
    /// Memberships will be the union of all queries. Only one entry with USER resource is currently supported. Customers can create up to 100 dynamic groups.
    
    pub queries: Option<Vec<DynamicGroupQuery>>,
    /// Output only. Status of the dynamic group.
    
    pub status: Option<DynamicGroupStatus>,
}

impl client::Part for DynamicGroupMetadata {}


/// Defines a query on a resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicGroupQuery {
    /// Query that determines the memberships of the dynamic group. Examples: All users with at least one `organizations.department` of engineering. `user.organizations.exists(org, org.department=='engineering')` All users with at least one location that has `area` of `foo` and `building_id` of `bar`. `user.locations.exists(loc, loc.area=='foo' && loc.building_id=='bar')` All users with any variation of the name John Doe (case-insensitive queries add `equalsIgnoreCase()` to the value being queried). `user.name.value.equalsIgnoreCase('jOhn DoE')`
    
    pub query: Option<String>,
    /// Resource type for the Dynamic Group Query
    #[serde(rename="resourceType")]
    
    pub resource_type: Option<String>,
}

impl client::Part for DynamicGroupQuery {}


/// The current status of a dynamic group along with timestamp.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicGroupStatus {
    /// Status of the dynamic group.
    
    pub status: Option<String>,
    /// The latest time at which the dynamic group is guaranteed to be in the given status. If status is `UP_TO_DATE`, the latest time at which the dynamic group was confirmed to be up-to-date. If status is `UPDATING_MEMBERSHIPS`, the time at which dynamic group was created.
    #[serde(rename="statusTime")]
    
    pub status_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for DynamicGroupStatus {}


/// A unique identifier for an entity in the Cloud Identity Groups API. An entity can represent either a group with an optional `namespace` or a user without a `namespace`. The combination of `id` and `namespace` must be unique; however, the same `id` can be used with different `namespace`s.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EntityKey {
    /// The ID of the entity. For Google-managed entities, the `id` should be the email address of an existing group or user. For external-identity-mapped entities, the `id` must be a string conforming to the Identity Source's requirements. Must be unique within a `namespace`.
    
    pub id: Option<String>,
    /// The namespace in which the entity exists. If not specified, the `EntityKey` represents a Google-managed entity such as a Google user or a Google Group. If specified, the `EntityKey` represents an external-identity-mapped group. The namespace must correspond to an identity source created in Admin Console and must be in the form of `identitysources/{identity_source}`.
    
    pub namespace: Option<String>,
}

impl client::Part for EntityKey {}


/// The `MembershipRole` expiry details.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExpiryDetail {
    /// The time at which the `MembershipRole` will expire.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for ExpiryDetail {}


/// Resource representing the Android specific attributes of a Device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAppsCloudidentityDevicesV1AndroidAttributes {
    /// Whether applications from unknown sources can be installed on device.
    #[serde(rename="enabledUnknownSources")]
    
    pub enabled_unknown_sources: Option<bool>,
    /// Whether this account is on an owner/primary profile. For phones, only true for owner profiles. Android 4+ devices can have secondary or restricted user profiles.
    #[serde(rename="ownerProfileAccount")]
    
    pub owner_profile_account: Option<bool>,
    /// Ownership privileges on device.
    #[serde(rename="ownershipPrivilege")]
    
    pub ownership_privilege: Option<String>,
    /// Whether device supports Android work profiles. If false, this service will not block access to corp data even if an administrator turns on the "Enforce Work Profile" policy.
    #[serde(rename="supportsWorkProfile")]
    
    pub supports_work_profile: Option<bool>,
}

impl client::Part for GoogleAppsCloudidentityDevicesV1AndroidAttributes {}


/// Request message for approving the device to access user data.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [device users approve devices](DeviceDeviceUserApproveCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAppsCloudidentityDevicesV1ApproveDeviceUserRequest {
    /// Optional. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer}`, where customer is the customer to whom the device belongs.
    
    pub customer: Option<String>,
}

impl client::RequestValue for GoogleAppsCloudidentityDevicesV1ApproveDeviceUserRequest {}


/// Request message for blocking account on device.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [device users block devices](DeviceDeviceUserBlockCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAppsCloudidentityDevicesV1BlockDeviceUserRequest {
    /// Optional. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer}`, where customer is the customer to whom the device belongs.
    
    pub customer: Option<String>,
}

impl client::RequestValue for GoogleAppsCloudidentityDevicesV1BlockDeviceUserRequest {}


/// Request message for cancelling an unfinished device wipe.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [cancel wipe devices](DeviceCancelWipeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAppsCloudidentityDevicesV1CancelWipeDeviceRequest {
    /// Optional. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer}`, where customer is the customer to whom the device belongs.
    
    pub customer: Option<String>,
}

impl client::RequestValue for GoogleAppsCloudidentityDevicesV1CancelWipeDeviceRequest {}


/// Request message for cancelling an unfinished user account wipe.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [device users cancel wipe devices](DeviceDeviceUserCancelWipeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAppsCloudidentityDevicesV1CancelWipeDeviceUserRequest {
    /// Optional. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer}`, where customer is the customer to whom the device belongs.
    
    pub customer: Option<String>,
}

impl client::RequestValue for GoogleAppsCloudidentityDevicesV1CancelWipeDeviceUserRequest {}


/// Represents the state associated with an API client calling the Devices API. Resource representing ClientState and supports updates from API users
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [device users client states get devices](DeviceDeviceUserClientStateGetCall) (response)
/// * [device users client states patch devices](DeviceDeviceUserClientStatePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAppsCloudidentityDevicesV1ClientState {
    /// The caller can specify asset tags for this resource
    #[serde(rename="assetTags")]
    
    pub asset_tags: Option<Vec<String>>,
    /// The compliance state of the resource as specified by the API client.
    #[serde(rename="complianceState")]
    
    pub compliance_state: Option<String>,
    /// Output only. The time the client state data was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// This field may be used to store a unique identifier for the API resource within which these CustomAttributes are a field.
    #[serde(rename="customId")]
    
    pub custom_id: Option<String>,
    /// The token that needs to be passed back for concurrency control in updates. Token needs to be passed back in UpdateRequest
    
    pub etag: Option<String>,
    /// The Health score of the resource. The Health score is the callers specification of the condition of the device from a usability point of view. For example, a third-party device management provider may specify a health score based on its compliance with organizational policies.
    #[serde(rename="healthScore")]
    
    pub health_score: Option<String>,
    /// The map of key-value attributes stored by callers specific to a device. The total serialized length of this map may not exceed 10KB. No limit is placed on the number of attributes in a map.
    #[serde(rename="keyValuePairs")]
    
    pub key_value_pairs: Option<HashMap<String, GoogleAppsCloudidentityDevicesV1CustomAttributeValue>>,
    /// Output only. The time the client state data was last updated.
    #[serde(rename="lastUpdateTime")]
    
    pub last_update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The management state of the resource as specified by the API client.
    
    pub managed: Option<String>,
    /// Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the ClientState in format: `devices/{device}/deviceUsers/{device_user}/clientState/{partner}`, where partner corresponds to the partner storing the data. For partners belonging to the "BeyondCorp Alliance", this is the partner ID specified to you by Google. For all other callers, this is a string of the form: `{customer}-suffix`, where `customer` is your customer ID. The *suffix* is any string the caller specifies. This string will be displayed verbatim in the administration console. This suffix is used in setting up Custom Access Levels in Context-Aware Access. Your organization's customer ID can be obtained from the URL: `GET https://www.googleapis.com/admin/directory/v1/customers/my_customer` The `id` field in the response contains the customer ID starting with the letter 'C'. The customer ID to be used in this API is the string after the letter 'C' (not including 'C')
    
    pub name: Option<String>,
    /// Output only. The owner of the ClientState
    #[serde(rename="ownerType")]
    
    pub owner_type: Option<String>,
    /// A descriptive cause of the health score.
    #[serde(rename="scoreReason")]
    
    pub score_reason: Option<String>,
}

impl client::RequestValue for GoogleAppsCloudidentityDevicesV1ClientState {}
impl client::ResponseResult for GoogleAppsCloudidentityDevicesV1ClientState {}


/// Additional custom attribute values may be one of these types
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAppsCloudidentityDevicesV1CustomAttributeValue {
    /// Represents a boolean value.
    #[serde(rename="boolValue")]
    
    pub bool_value: Option<bool>,
    /// Represents a double value.
    #[serde(rename="numberValue")]
    
    pub number_value: Option<f64>,
    /// Represents a string value.
    #[serde(rename="stringValue")]
    
    pub string_value: Option<String>,
}

impl client::Part for GoogleAppsCloudidentityDevicesV1CustomAttributeValue {}


/// A Device within the Cloud Identity Devices API. Represents a Device known to Google Cloud, independent of the device ownership, type, and whether it is assigned or in use by a user.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create devices](DeviceCreateCall) (request)
/// * [get devices](DeviceGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAppsCloudidentityDevicesV1Device {
    /// Output only. Attributes specific to Android devices.
    #[serde(rename="androidSpecificAttributes")]
    
    pub android_specific_attributes: Option<GoogleAppsCloudidentityDevicesV1AndroidAttributes>,
    /// Asset tag of the device.
    #[serde(rename="assetTag")]
    
    pub asset_tag: Option<String>,
    /// Output only. Baseband version of the device.
    #[serde(rename="basebandVersion")]
    
    pub baseband_version: Option<String>,
    /// Output only. Device bootloader version. Example: 0.6.7.
    #[serde(rename="bootloaderVersion")]
    
    pub bootloader_version: Option<String>,
    /// Output only. Device brand. Example: Samsung.
    
    pub brand: Option<String>,
    /// Output only. Build number of the device.
    #[serde(rename="buildNumber")]
    
    pub build_number: Option<String>,
    /// Output only. Represents whether the Device is compromised.
    #[serde(rename="compromisedState")]
    
    pub compromised_state: Option<String>,
    /// Output only. When the Company-Owned device was imported. This field is empty for BYOD devices.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Unique identifier for the device.
    #[serde(rename="deviceId")]
    
    pub device_id: Option<String>,
    /// Output only. Type of device.
    #[serde(rename="deviceType")]
    
    pub device_type: Option<String>,
    /// Output only. Whether developer options is enabled on device.
    #[serde(rename="enabledDeveloperOptions")]
    
    pub enabled_developer_options: Option<bool>,
    /// Output only. Whether USB debugging is enabled on device.
    #[serde(rename="enabledUsbDebugging")]
    
    pub enabled_usb_debugging: Option<bool>,
    /// Output only. Device encryption state.
    #[serde(rename="encryptionState")]
    
    pub encryption_state: Option<String>,
    /// Output only. IMEI number of device if GSM device; empty otherwise.
    
    pub imei: Option<String>,
    /// Output only. Kernel version of the device.
    #[serde(rename="kernelVersion")]
    
    pub kernel_version: Option<String>,
    /// Most recent time when device synced with this service.
    #[serde(rename="lastSyncTime")]
    
    pub last_sync_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Management state of the device
    #[serde(rename="managementState")]
    
    pub management_state: Option<String>,
    /// Output only. Device manufacturer. Example: Motorola.
    
    pub manufacturer: Option<String>,
    /// Output only. MEID number of device if CDMA device; empty otherwise.
    
    pub meid: Option<String>,
    /// Output only. Model name of device. Example: Pixel 3.
    
    pub model: Option<String>,
    /// Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Device in format: `devices/{device}`, where device is the unique id assigned to the Device.
    
    pub name: Option<String>,
    /// Output only. Mobile or network operator of device, if available.
    #[serde(rename="networkOperator")]
    
    pub network_operator: Option<String>,
    /// Output only. OS version of the device. Example: Android 8.1.0.
    #[serde(rename="osVersion")]
    
    pub os_version: Option<String>,
    /// Output only. Domain name for Google accounts on device. Type for other accounts on device. On Android, will only be populated if |ownership_privilege| is |PROFILE_OWNER| or |DEVICE_OWNER|. Does not include the account signed in to the device policy app if that account's domain has only one account. Examples: "com.example", "xyz.com".
    #[serde(rename="otherAccounts")]
    
    pub other_accounts: Option<Vec<String>>,
    /// Output only. Whether the device is owned by the company or an individual
    #[serde(rename="ownerType")]
    
    pub owner_type: Option<String>,
    /// Output only. OS release version. Example: 6.0.
    #[serde(rename="releaseVersion")]
    
    pub release_version: Option<String>,
    /// Output only. OS security patch update time on device.
    #[serde(rename="securityPatchTime")]
    
    pub security_patch_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Serial Number of device. Example: HT82V1A01076.
    #[serde(rename="serialNumber")]
    
    pub serial_number: Option<String>,
    /// WiFi MAC addresses of device.
    #[serde(rename="wifiMacAddresses")]
    
    pub wifi_mac_addresses: Option<Vec<String>>,
}

impl client::RequestValue for GoogleAppsCloudidentityDevicesV1Device {}
impl client::ResponseResult for GoogleAppsCloudidentityDevicesV1Device {}


/// Represents a user’s use of a Device in the Cloud Identity Devices API. A DeviceUser is a resource representing a user’s use of a Device
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [device users get devices](DeviceDeviceUserGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAppsCloudidentityDevicesV1DeviceUser {
    /// Compromised State of the DeviceUser object
    #[serde(rename="compromisedState")]
    
    pub compromised_state: Option<String>,
    /// When the user first signed in to the device
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Most recent time when user registered with this service.
    #[serde(rename="firstSyncTime")]
    
    pub first_sync_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Default locale used on device, in IETF BCP-47 format.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Output only. Last time when user synced with policies.
    #[serde(rename="lastSyncTime")]
    
    pub last_sync_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Management state of the user on the device.
    #[serde(rename="managementState")]
    
    pub management_state: Option<String>,
    /// Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the DeviceUser in format: `devices/{device}/deviceUsers/{device_user}`, where `device_user` uniquely identifies a user's use of a device.
    
    pub name: Option<String>,
    /// Password state of the DeviceUser object
    #[serde(rename="passwordState")]
    
    pub password_state: Option<String>,
    /// Output only. User agent on the device for this specific user
    #[serde(rename="userAgent")]
    
    pub user_agent: Option<String>,
    /// Email address of the user registered on the device.
    #[serde(rename="userEmail")]
    
    pub user_email: Option<String>,
}

impl client::ResponseResult for GoogleAppsCloudidentityDevicesV1DeviceUser {}


/// Response message that is returned in ListClientStates.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [device users client states list devices](DeviceDeviceUserClientStateListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAppsCloudidentityDevicesV1ListClientStatesResponse {
    /// Client states meeting the list restrictions.
    #[serde(rename="clientStates")]
    
    pub client_states: Option<Vec<GoogleAppsCloudidentityDevicesV1ClientState>>,
    /// Token to retrieve the next page of results. Empty if there are no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleAppsCloudidentityDevicesV1ListClientStatesResponse {}


/// Response message that is returned from the ListDeviceUsers method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [device users list devices](DeviceDeviceUserListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAppsCloudidentityDevicesV1ListDeviceUsersResponse {
    /// Devices meeting the list restrictions.
    #[serde(rename="deviceUsers")]
    
    pub device_users: Option<Vec<GoogleAppsCloudidentityDevicesV1DeviceUser>>,
    /// Token to retrieve the next page of results. Empty if there are no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleAppsCloudidentityDevicesV1ListDeviceUsersResponse {}


/// Response message that is returned from the ListDevices method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list devices](DeviceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAppsCloudidentityDevicesV1ListDevicesResponse {
    /// Devices meeting the list restrictions.
    
    pub devices: Option<Vec<GoogleAppsCloudidentityDevicesV1Device>>,
    /// Token to retrieve the next page of results. Empty if there are no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleAppsCloudidentityDevicesV1ListDevicesResponse {}


/// Response containing resource names of the DeviceUsers associated with the caller’s credentials.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [device users lookup devices](DeviceDeviceUserLookupCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAppsCloudidentityDevicesV1LookupSelfDeviceUsersResponse {
    /// The customer resource name that may be passed back to other Devices API methods such as List, Get, etc.
    
    pub customer: Option<String>,
    /// [Resource names](https://cloud.google.com/apis/design/resource_names) of the DeviceUsers in the format: `devices/{device}/deviceUsers/{user_resource}`, where device is the unique ID assigned to a Device and user_resource is the unique user ID
    
    pub names: Option<Vec<String>>,
    /// Token to retrieve the next page of results. Empty if there are no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleAppsCloudidentityDevicesV1LookupSelfDeviceUsersResponse {}


/// Request message for wiping all data on the device.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [wipe devices](DeviceWipeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAppsCloudidentityDevicesV1WipeDeviceRequest {
    /// Optional. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer}`, where customer is the customer to whom the device belongs.
    
    pub customer: Option<String>,
    /// Optional. Specifies if a user is able to factory reset a device after a Device Wipe. On iOS, this is called "Activation Lock", while on Android, this is known as "Factory Reset Protection". If true, this protection will be removed from the device, so that a user can successfully factory reset. If false, the setting is untouched on the device.
    #[serde(rename="removeResetLock")]
    
    pub remove_reset_lock: Option<bool>,
}

impl client::RequestValue for GoogleAppsCloudidentityDevicesV1WipeDeviceRequest {}


/// Request message for starting an account wipe on device.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [device users wipe devices](DeviceDeviceUserWipeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAppsCloudidentityDevicesV1WipeDeviceUserRequest {
    /// Optional. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer}`, where customer is the customer to whom the device belongs.
    
    pub customer: Option<String>,
}

impl client::RequestValue for GoogleAppsCloudidentityDevicesV1WipeDeviceUserRequest {}


/// A group within the Cloud Identity Groups API. A `Group` is a collection of entities, where each entity is either a user, another group, or a service account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [memberships check transitive membership groups](GroupMembershipCheckTransitiveMembershipCall) (none)
/// * [memberships create groups](GroupMembershipCreateCall) (none)
/// * [memberships delete groups](GroupMembershipDeleteCall) (none)
/// * [memberships get groups](GroupMembershipGetCall) (none)
/// * [memberships get membership graph groups](GroupMembershipGetMembershipGraphCall) (none)
/// * [memberships list groups](GroupMembershipListCall) (none)
/// * [memberships lookup groups](GroupMembershipLookupCall) (none)
/// * [memberships modify membership roles groups](GroupMembershipModifyMembershipRoleCall) (none)
/// * [memberships search transitive groups groups](GroupMembershipSearchTransitiveGroupCall) (none)
/// * [memberships search transitive memberships groups](GroupMembershipSearchTransitiveMembershipCall) (none)
/// * [create groups](GroupCreateCall) (request)
/// * [delete groups](GroupDeleteCall) (none)
/// * [get groups](GroupGetCall) (response)
/// * [get security settings groups](GroupGetSecuritySettingCall) (none)
/// * [list groups](GroupListCall) (none)
/// * [lookup groups](GroupLookupCall) (none)
/// * [patch groups](GroupPatchCall) (request)
/// * [search groups](GroupSearchCall) (none)
/// * [update security settings groups](GroupUpdateSecuritySettingCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Group {
    /// Output only. The time when the `Group` was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// An extended description to help users determine the purpose of a `Group`. Must not be longer than 4,096 characters.
    
    pub description: Option<String>,
    /// The display name of the `Group`.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Optional. Dynamic group metadata like queries and status.
    #[serde(rename="dynamicGroupMetadata")]
    
    pub dynamic_group_metadata: Option<DynamicGroupMetadata>,
    /// Required. The `EntityKey` of the `Group`.
    #[serde(rename="groupKey")]
    
    pub group_key: Option<EntityKey>,
    /// Required. One or more label entries that apply to the Group. Currently supported labels contain a key with an empty value. Google Groups are the default type of group and have a label with a key of `cloudidentity.googleapis.com/groups.discussion_forum` and an empty value. Existing Google Groups can have an additional label with a key of `cloudidentity.googleapis.com/groups.security` and an empty value added to them. **This is an immutable change and the security label cannot be removed once added.** Dynamic groups have a label with a key of `cloudidentity.googleapis.com/groups.dynamic`. Identity-mapped groups for Cloud Search have a label with a key of `system/groups/external` and an empty value.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The [resource name](https://cloud.google.com/apis/design/resource_names) of the `Group`. Shall be of the form `groups/{group}`.
    
    pub name: Option<String>,
    /// Required. Immutable. The resource name of the entity under which this `Group` resides in the Cloud Identity resource hierarchy. Must be of the form `identitysources/{identity_source}` for external [identity-mapped groups](https://support.google.com/a/answer/9039510) or `customers/{customer_id}` for Google Groups. The `customer_id` must begin with "C" (for example, 'C046psxkn'). [Find your customer ID.] (https://support.google.com/cloudidentity/answer/10070793)
    
    pub parent: Option<String>,
    /// Output only. The time when the `Group` was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Group {}
impl client::Resource for Group {}
impl client::ResponseResult for Group {}


/// Message representing a transitive group of a user or a group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GroupRelation {
    /// Display name for this group.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Resource name for this group.
    
    pub group: Option<String>,
    /// Entity key has an id and a namespace. In case of discussion forums, the id will be an email address without a namespace.
    #[serde(rename="groupKey")]
    
    pub group_key: Option<EntityKey>,
    /// Labels for Group resource.
    
    pub labels: Option<HashMap<String, String>>,
    /// The relation between the member and the transitive group.
    #[serde(rename="relationType")]
    
    pub relation_type: Option<String>,
    /// Membership roles of the member for the group.
    
    pub roles: Option<Vec<TransitiveMembershipRole>>,
}

impl client::Part for GroupRelation {}


/// Credential for verifying signatures produced by the Identity Provider.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [idp credentials get inbound saml sso profiles](InboundSamlSsoProfileIdpCredentialGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdpCredential {
    /// Output only. Information of a DSA public key.
    #[serde(rename="dsaKeyInfo")]
    
    pub dsa_key_info: Option<DsaPublicKeyInfo>,
    /// Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the credential.
    
    pub name: Option<String>,
    /// Output only. Information of a RSA public key.
    #[serde(rename="rsaKeyInfo")]
    
    pub rsa_key_info: Option<RsaPublicKeyInfo>,
    /// Output only. Time when the `IdpCredential` was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for IdpCredential {}


/// A [SAML 2.0](https://www.oasis-open.org/standards#samlv2.0) federation between a Google enterprise customer and a SAML identity provider.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [idp credentials add inbound saml sso profiles](InboundSamlSsoProfileIdpCredentialAddCall) (none)
/// * [idp credentials delete inbound saml sso profiles](InboundSamlSsoProfileIdpCredentialDeleteCall) (none)
/// * [idp credentials get inbound saml sso profiles](InboundSamlSsoProfileIdpCredentialGetCall) (none)
/// * [idp credentials list inbound saml sso profiles](InboundSamlSsoProfileIdpCredentialListCall) (none)
/// * [create inbound saml sso profiles](InboundSamlSsoProfileCreateCall) (request)
/// * [delete inbound saml sso profiles](InboundSamlSsoProfileDeleteCall) (none)
/// * [get inbound saml sso profiles](InboundSamlSsoProfileGetCall) (response)
/// * [list inbound saml sso profiles](InboundSamlSsoProfileListCall) (none)
/// * [patch inbound saml sso profiles](InboundSamlSsoProfilePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InboundSamlSsoProfile {
    /// Immutable. The customer. For example: `customers/C0123abc`.
    
    pub customer: Option<String>,
    /// Human-readable name of the SAML SSO profile.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// SAML identity provider configuration.
    #[serde(rename="idpConfig")]
    
    pub idp_config: Option<SamlIdpConfig>,
    /// Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the SAML SSO profile.
    
    pub name: Option<String>,
    /// SAML service provider configuration for this SAML SSO profile. These are the service provider details provided by Google that should be configured on the corresponding identity provider.
    #[serde(rename="spConfig")]
    
    pub sp_config: Option<SamlSpConfig>,
}

impl client::RequestValue for InboundSamlSsoProfile {}
impl client::Resource for InboundSamlSsoProfile {}
impl client::ResponseResult for InboundSamlSsoProfile {}


/// Targets with “set” SSO assignments and their respective assignments.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create inbound sso assignments](InboundSsoAssignmentCreateCall) (request)
/// * [delete inbound sso assignments](InboundSsoAssignmentDeleteCall) (none)
/// * [get inbound sso assignments](InboundSsoAssignmentGetCall) (response)
/// * [list inbound sso assignments](InboundSsoAssignmentListCall) (none)
/// * [patch inbound sso assignments](InboundSsoAssignmentPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InboundSsoAssignment {
    /// Immutable. The customer. For example: `customers/C0123abc`.
    
    pub customer: Option<String>,
    /// Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Inbound SSO Assignment.
    
    pub name: Option<String>,
    /// Must be zero (which is the default value so it can be omitted) for assignments with `target_org_unit` set and must be greater-than-or-equal-to one for assignments with `target_group` set.
    
    pub rank: Option<i32>,
    /// SAML SSO details. Must be set if and only if `sso_mode` is set to `SAML_SSO`.
    #[serde(rename="samlSsoInfo")]
    
    pub saml_sso_info: Option<SamlSsoInfo>,
    /// Assertions about users assigned to an IdP will always be accepted from that IdP. This controls whether/when Google should redirect a user to the IdP. Unset (defaults) is the recommended configuration.
    #[serde(rename="signInBehavior")]
    
    pub sign_in_behavior: Option<SignInBehavior>,
    /// Inbound SSO behavior.
    #[serde(rename="ssoMode")]
    
    pub sso_mode: Option<String>,
    /// Immutable. Must be of the form `groups/{group}`.
    #[serde(rename="targetGroup")]
    
    pub target_group: Option<String>,
    /// Immutable. Must be of the form `orgUnits/{org_unit}`.
    #[serde(rename="targetOrgUnit")]
    
    pub target_org_unit: Option<String>,
}

impl client::RequestValue for InboundSsoAssignment {}
impl client::Resource for InboundSsoAssignment {}
impl client::ResponseResult for InboundSsoAssignment {}


/// Response for IsInvitableUser RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [userinvitations is invitable user customers](CustomerUserinvitationIsInvitableUserCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IsInvitableUserResponse {
    /// Returns true if the email address is invitable.
    #[serde(rename="isInvitableUser")]
    
    pub is_invitable_user: Option<bool>,
}

impl client::ResponseResult for IsInvitableUserResponse {}


/// Response message for ListGroups operation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list groups](GroupListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListGroupsResponse {
    /// Groups returned in response to list request. The results are not sorted.
    
    pub groups: Option<Vec<Group>>,
    /// Token to retrieve the next page of results, or empty if there are no more results available for listing.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListGroupsResponse {}


/// Response of the InboundSamlSsoProfilesService.ListIdpCredentials method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [idp credentials list inbound saml sso profiles](InboundSamlSsoProfileIdpCredentialListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListIdpCredentialsResponse {
    /// The IdpCredentials from the specified InboundSamlSsoProfile.
    #[serde(rename="idpCredentials")]
    
    pub idp_credentials: Option<Vec<IdpCredential>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListIdpCredentialsResponse {}


/// Response of the InboundSamlSsoProfilesService.ListInboundSamlSsoProfiles method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list inbound saml sso profiles](InboundSamlSsoProfileListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListInboundSamlSsoProfilesResponse {
    /// List of InboundSamlSsoProfiles.
    #[serde(rename="inboundSamlSsoProfiles")]
    
    pub inbound_saml_sso_profiles: Option<Vec<InboundSamlSsoProfile>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListInboundSamlSsoProfilesResponse {}


/// Response of the InboundSsoAssignmentsService.ListInboundSsoAssignments method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list inbound sso assignments](InboundSsoAssignmentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListInboundSsoAssignmentsResponse {
    /// The assignments.
    #[serde(rename="inboundSsoAssignments")]
    
    pub inbound_sso_assignments: Option<Vec<InboundSsoAssignment>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListInboundSsoAssignmentsResponse {}


/// The response message for MembershipsService.ListMemberships.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [memberships list groups](GroupMembershipListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMembershipsResponse {
    /// The `Membership`s under the specified `parent`.
    
    pub memberships: Option<Vec<Membership>>,
    /// A continuation token to retrieve the next page of results, or empty if there are no more results available.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListMembershipsResponse {}


/// Response message for UserInvitation listing request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [userinvitations list customers](CustomerUserinvitationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListUserInvitationsResponse {
    /// The token for the next page. If not empty, indicates that there may be more `UserInvitation` resources that match the listing request; this value can be used in a subsequent ListUserInvitationsRequest to get continued results with the current list call.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of UserInvitation resources.
    #[serde(rename="userInvitations")]
    
    pub user_invitations: Option<Vec<UserInvitation>>,
}

impl client::ResponseResult for ListUserInvitationsResponse {}


/// The response message for GroupsService.LookupGroupName.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [lookup groups](GroupLookupCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LookupGroupNameResponse {
    /// The [resource name](https://cloud.google.com/apis/design/resource_names) of the looked-up `Group`.
    
    pub name: Option<String>,
}

impl client::ResponseResult for LookupGroupNameResponse {}


/// The response message for MembershipsService.LookupMembershipName.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [memberships lookup groups](GroupMembershipLookupCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LookupMembershipNameResponse {
    /// The [resource name](https://cloud.google.com/apis/design/resource_names) of the looked-up `Membership`. Must be of the form `groups/{group}/memberships/{membership}`.
    
    pub name: Option<String>,
}

impl client::ResponseResult for LookupMembershipNameResponse {}


/// Message representing a transitive membership of a group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MemberRelation {
    /// Resource name for this member.
    
    pub member: Option<String>,
    /// Entity key has an id and a namespace. In case of discussion forums, the id will be an email address without a namespace.
    #[serde(rename="preferredMemberKey")]
    
    pub preferred_member_key: Option<Vec<EntityKey>>,
    /// The relation between the group and the transitive member.
    #[serde(rename="relationType")]
    
    pub relation_type: Option<String>,
    /// The membership role details (i.e name of role and expiry time).
    
    pub roles: Option<Vec<TransitiveMembershipRole>>,
}

impl client::Part for MemberRelation {}


/// The definition of MemberRestriction
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MemberRestriction {
    /// The evaluated state of this restriction on a group.
    
    pub evaluation: Option<RestrictionEvaluation>,
    /// Member Restriction as defined by CEL expression. Supported restrictions are: `member.customer_id` and `member.type`. Valid values for `member.type` are `1`, `2` and `3`. They correspond to USER, SERVICE_ACCOUNT, and GROUP respectively. The value for `member.customer_id` only supports `groupCustomerId()` currently which means the customer id of the group will be used for restriction. Supported operators are `&&`, `||` and `==`, corresponding to AND, OR, and EQUAL. Examples: Allow only service accounts of given customer to be members. `member.type == 2 && member.customer_id == groupCustomerId()` Allow only users or groups to be members. `member.type == 1 || member.type == 3`
    
    pub query: Option<String>,
}

impl client::Part for MemberRestriction {}


/// A membership within the Cloud Identity Groups API. A `Membership` defines a relationship between a `Group` and an entity belonging to that `Group`, referred to as a “member”.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [memberships create groups](GroupMembershipCreateCall) (request)
/// * [memberships get groups](GroupMembershipGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Membership {
    /// Output only. The time when the `Membership` was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The [resource name](https://cloud.google.com/apis/design/resource_names) of the `Membership`. Shall be of the form `groups/{group}/memberships/{membership}`.
    
    pub name: Option<String>,
    /// Required. Immutable. The `EntityKey` of the member.
    #[serde(rename="preferredMemberKey")]
    
    pub preferred_member_key: Option<EntityKey>,
    /// The `MembershipRole`s that apply to the `Membership`. If unspecified, defaults to a single `MembershipRole` with `name` `MEMBER`. Must not contain duplicate `MembershipRole`s with the same `name`.
    
    pub roles: Option<Vec<MembershipRole>>,
    /// Output only. The type of the membership.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Output only. The time when the `Membership` was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Membership {}
impl client::ResponseResult for Membership {}


/// A membership role within the Cloud Identity Groups API. A `MembershipRole` defines the privileges granted to a `Membership`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MembershipRole {
    /// The expiry details of the `MembershipRole`. Expiry details are only supported for `MEMBER` `MembershipRoles`. May be set if `name` is `MEMBER`. Must not be set if `name` is any other value.
    #[serde(rename="expiryDetail")]
    
    pub expiry_detail: Option<ExpiryDetail>,
    /// The name of the `MembershipRole`. Must be one of `OWNER`, `MANAGER`, `MEMBER`.
    
    pub name: Option<String>,
    /// Evaluations of restrictions applied to parent group on this membership.
    #[serde(rename="restrictionEvaluations")]
    
    pub restriction_evaluations: Option<RestrictionEvaluations>,
}

impl client::Part for MembershipRole {}


/// The evaluated state of this restriction.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MembershipRoleRestrictionEvaluation {
    /// Output only. The current state of the restriction
    
    pub state: Option<String>,
}

impl client::Part for MembershipRoleRestrictionEvaluation {}


/// The request message for MembershipsService.ModifyMembershipRoles.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [memberships modify membership roles groups](GroupMembershipModifyMembershipRoleCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ModifyMembershipRolesRequest {
    /// The `MembershipRole`s to be added. Adding or removing roles in the same request as updating roles is not supported. Must not be set if `update_roles_params` is set.
    #[serde(rename="addRoles")]
    
    pub add_roles: Option<Vec<MembershipRole>>,
    /// The `name`s of the `MembershipRole`s to be removed. Adding or removing roles in the same request as updating roles is not supported. It is not possible to remove the `MEMBER` `MembershipRole`. If you wish to delete a `Membership`, call MembershipsService.DeleteMembership instead. Must not contain `MEMBER`. Must not be set if `update_roles_params` is set.
    #[serde(rename="removeRoles")]
    
    pub remove_roles: Option<Vec<String>>,
    /// The `MembershipRole`s to be updated. Updating roles in the same request as adding or removing roles is not supported. Must not be set if either `add_roles` or `remove_roles` is set.
    #[serde(rename="updateRolesParams")]
    
    pub update_roles_params: Option<Vec<UpdateMembershipRolesParams>>,
}

impl client::RequestValue for ModifyMembershipRolesRequest {}


/// The response message for MembershipsService.ModifyMembershipRoles.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [memberships modify membership roles groups](GroupMembershipModifyMembershipRoleCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ModifyMembershipRolesResponse {
    /// The `Membership` resource after modifying its `MembershipRole`s.
    
    pub membership: Option<Membership>,
}

impl client::ResponseResult for ModifyMembershipRolesResponse {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [userinvitations cancel customers](CustomerUserinvitationCancelCall) (response)
/// * [userinvitations send customers](CustomerUserinvitationSendCall) (response)
/// * [device users client states patch devices](DeviceDeviceUserClientStatePatchCall) (response)
/// * [device users approve devices](DeviceDeviceUserApproveCall) (response)
/// * [device users block devices](DeviceDeviceUserBlockCall) (response)
/// * [device users cancel wipe devices](DeviceDeviceUserCancelWipeCall) (response)
/// * [device users delete devices](DeviceDeviceUserDeleteCall) (response)
/// * [device users wipe devices](DeviceDeviceUserWipeCall) (response)
/// * [cancel wipe devices](DeviceCancelWipeCall) (response)
/// * [create devices](DeviceCreateCall) (response)
/// * [delete devices](DeviceDeleteCall) (response)
/// * [wipe devices](DeviceWipeCall) (response)
/// * [memberships create groups](GroupMembershipCreateCall) (response)
/// * [memberships delete groups](GroupMembershipDeleteCall) (response)
/// * [memberships get membership graph groups](GroupMembershipGetMembershipGraphCall) (response)
/// * [create groups](GroupCreateCall) (response)
/// * [delete groups](GroupDeleteCall) (response)
/// * [patch groups](GroupPatchCall) (response)
/// * [update security settings groups](GroupUpdateSecuritySettingCall) (response)
/// * [idp credentials add inbound saml sso profiles](InboundSamlSsoProfileIdpCredentialAddCall) (response)
/// * [idp credentials delete inbound saml sso profiles](InboundSamlSsoProfileIdpCredentialDeleteCall) (response)
/// * [create inbound saml sso profiles](InboundSamlSsoProfileCreateCall) (response)
/// * [delete inbound saml sso profiles](InboundSamlSsoProfileDeleteCall) (response)
/// * [patch inbound saml sso profiles](InboundSamlSsoProfilePatchCall) (response)
/// * [create inbound sso assignments](InboundSsoAssignmentCreateCall) (response)
/// * [delete inbound sso assignments](InboundSsoAssignmentDeleteCall) (response)
/// * [patch inbound sso assignments](InboundSsoAssignmentPatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<Status>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for Operation {}


/// The evaluated state of this restriction.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RestrictionEvaluation {
    /// Output only. The current state of the restriction
    
    pub state: Option<String>,
}

impl client::Part for RestrictionEvaluation {}


/// Evaluations of restrictions applied to parent group on this membership.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RestrictionEvaluations {
    /// Evaluation of the member restriction applied to this membership. Empty if the user lacks permission to view the restriction evaluation.
    #[serde(rename="memberRestrictionEvaluation")]
    
    pub member_restriction_evaluation: Option<MembershipRoleRestrictionEvaluation>,
}

impl client::Part for RestrictionEvaluations {}


/// Information of a RSA public key.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RsaPublicKeyInfo {
    /// Key size in bits (size of the modulus).
    #[serde(rename="keySize")]
    
    pub key_size: Option<i32>,
}

impl client::Part for RsaPublicKeyInfo {}


/// SAML IDP (identity provider) configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SamlIdpConfig {
    /// The **Change Password URL** of the identity provider. Users will be sent to this URL when changing their passwords at `myaccount.google.com`. This takes precedence over the change password URL configured at customer-level. Must use `HTTPS`.
    #[serde(rename="changePasswordUri")]
    
    pub change_password_uri: Option<String>,
    /// Required. The SAML **Entity ID** of the identity provider.
    #[serde(rename="entityId")]
    
    pub entity_id: Option<String>,
    /// The **Logout Redirect URL** (sign-out page URL) of the identity provider. When a user clicks the sign-out link on a Google page, they will be redirected to this URL. This is a pure redirect with no attached SAML `LogoutRequest` i.e. SAML single logout is currently not supported. Must use `HTTPS`.
    #[serde(rename="logoutRedirectUri")]
    
    pub logout_redirect_uri: Option<String>,
    /// Required. The `SingleSignOnService` endpoint location (sign-in page URL) of the identity provider. This is the URL where the `AuthnRequest` will be sent. Must use `HTTPS`. Currently assumed to accept the `HTTP-Redirect` binding.
    #[serde(rename="singleSignOnServiceUri")]
    
    pub single_sign_on_service_uri: Option<String>,
}

impl client::Part for SamlIdpConfig {}


/// SAML SP (service provider) configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SamlSpConfig {
    /// Output only. The SAML **Assertion Consumer Service (ACS) URL** to be used for the IDP-initiated login. Currently assumed to accept response messages via the `HTTP-POST` binding.
    #[serde(rename="assertionConsumerServiceUri")]
    
    pub assertion_consumer_service_uri: Option<String>,
    /// Output only. The SAML **Entity ID** for this service provider.
    #[serde(rename="entityId")]
    
    pub entity_id: Option<String>,
}

impl client::Part for SamlSpConfig {}


/// Details that are applicable when `sso_mode` == `SAML_SSO`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SamlSsoInfo {
    /// Required. Name of the `InboundSamlSsoProfile` to use. Must be of the form `inboundSamlSsoProfiles/{inbound_saml_sso_profile}`. 
    #[serde(rename="inboundSamlSsoProfile")]
    
    pub inbound_saml_sso_profile: Option<String>,
}

impl client::Part for SamlSsoInfo {}


/// The response message for GroupsService.SearchGroups.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search groups](GroupSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchGroupsResponse {
    /// The `Group` resources that match the search query.
    
    pub groups: Option<Vec<Group>>,
    /// A continuation token to retrieve the next page of results, or empty if there are no more results available.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for SearchGroupsResponse {}


/// The response message for MembershipsService.SearchTransitiveGroups.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [memberships search transitive groups groups](GroupMembershipSearchTransitiveGroupCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchTransitiveGroupsResponse {
    /// List of transitive groups satisfying the query.
    
    pub memberships: Option<Vec<GroupRelation>>,
    /// Token to retrieve the next page of results, or empty if there are no more results available for listing.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for SearchTransitiveGroupsResponse {}


/// The response message for MembershipsService.SearchTransitiveMemberships.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [memberships search transitive memberships groups](GroupMembershipSearchTransitiveMembershipCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchTransitiveMembershipsResponse {
    /// List of transitive members satisfying the query.
    
    pub memberships: Option<Vec<MemberRelation>>,
    /// Token to retrieve the next page of results, or empty if there are no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for SearchTransitiveMembershipsResponse {}


/// The definition of security settings.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get security settings groups](GroupGetSecuritySettingCall) (response)
/// * [update security settings groups](GroupUpdateSecuritySettingCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SecuritySettings {
    /// The Member Restriction value
    #[serde(rename="memberRestriction")]
    
    pub member_restriction: Option<MemberRestriction>,
    /// Output only. The resource name of the security settings. Shall be of the form `groups/{group_id}/securitySettings`.
    
    pub name: Option<String>,
}

impl client::RequestValue for SecuritySettings {}
impl client::ResponseResult for SecuritySettings {}


/// A request to send email for inviting target user corresponding to the UserInvitation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [userinvitations send customers](CustomerUserinvitationSendCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SendUserInvitationRequest { _never_set: Option<bool> }

impl client::RequestValue for SendUserInvitationRequest {}


/// Controls sign-in behavior.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SignInBehavior {
    /// When to redirect sign-ins to the IdP.
    #[serde(rename="redirectCondition")]
    
    pub redirect_condition: Option<String>,
}

impl client::Part for SignInBehavior {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
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


/// Message representing the role of a TransitiveMembership.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransitiveMembershipRole {
    /// TransitiveMembershipRole in string format. Currently supported TransitiveMembershipRoles: `"MEMBER"`, `"OWNER"`, and `"MANAGER"`.
    
    pub role: Option<String>,
}

impl client::Part for TransitiveMembershipRole {}


/// The details of an update to a `MembershipRole`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateMembershipRolesParams {
    /// The fully-qualified names of fields to update. May only contain the field `expiry_detail.expire_time`.
    #[serde(rename="fieldMask")]
    
    pub field_mask: Option<client::FieldMask>,
    /// The `MembershipRole`s to be updated. Only `MEMBER` `MembershipRole` can currently be updated.
    #[serde(rename="membershipRole")]
    
    pub membership_role: Option<MembershipRole>,
}

impl client::Part for UpdateMembershipRolesParams {}


/// The `UserInvitation` resource represents an email that can be sent to an unmanaged user account inviting them to join the customer’s Google Workspace or Cloud Identity account. An unmanaged account shares an email address domain with the Google Workspace or Cloud Identity account but is not managed by it yet. If the user accepts the `UserInvitation`, the user account will become managed.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [userinvitations get customers](CustomerUserinvitationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserInvitation {
    /// Number of invitation emails sent to the user.
    #[serde(rename="mailsSentCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub mails_sent_count: Option<i64>,
    /// Shall be of the form `customers/{customer}/userinvitations/{user_email_address}`.
    
    pub name: Option<String>,
    /// State of the `UserInvitation`.
    
    pub state: Option<String>,
    /// Time when the `UserInvitation` was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for UserInvitation {}


