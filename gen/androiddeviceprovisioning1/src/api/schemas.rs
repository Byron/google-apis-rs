use super::*;
/// Request message to claim a device on behalf of a customer.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices claim partners](PartnerDeviceClaimCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClaimDeviceRequest {
    /// The ID of the customer for whom the device is being claimed.
    #[serde(rename="customerId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub customer_id: Option<i64>,
    /// Required. Required. The device identifier of the device to claim.
    #[serde(rename="deviceIdentifier")]
    
    pub device_identifier: Option<DeviceIdentifier>,
    /// Optional. The metadata to attach to the device.
    #[serde(rename="deviceMetadata")]
    
    pub device_metadata: Option<DeviceMetadata>,
    /// The Google Workspace customer ID.
    #[serde(rename="googleWorkspaceCustomerId")]
    
    pub google_workspace_customer_id: Option<String>,
    /// Optional. Must and can only be set for Chrome OS devices.
    #[serde(rename="preProvisioningToken")]
    
    pub pre_provisioning_token: Option<String>,
    /// Required. The section type of the device's provisioning record.
    #[serde(rename="sectionType")]
    
    pub section_type: Option<ClaimDeviceRequestSectionTypeEnum>,
    /// Optional. Must and can only be set when DeviceProvisioningSectionType is SECTION_TYPE_SIM_LOCK. The unique identifier of the SimLock profile (go/simlock/profiles).
    #[serde(rename="simlockProfileId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub simlock_profile_id: Option<i64>,
}

impl client::RequestValue for ClaimDeviceRequest {}


/// Response message containing device id of the claim.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices claim partners](PartnerDeviceClaimCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClaimDeviceResponse {
    /// The device ID of the claimed device.
    #[serde(rename="deviceId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub device_id: Option<i64>,
    /// The resource name of the device in the format `partners/[PARTNER_ID]/devices/[DEVICE_ID]`.
    #[serde(rename="deviceName")]
    
    pub device_name: Option<String>,
}

impl client::ResponseResult for ClaimDeviceResponse {}


/// Request to claim devices asynchronously in batch. Claiming a device adds the device to zero-touch enrollment and shows the device in the customer’s view of the portal.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices claim async partners](PartnerDeviceClaimAsyncCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClaimDevicesRequest {
    /// Required. A list of device claims.
    
    pub claims: Option<Vec<PartnerClaim>>,
}

impl client::RequestValue for ClaimDevicesRequest {}


/// A reseller, vendor, or customer in the zero-touch reseller and customer APIs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customers create partners](PartnerCustomerCreateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Company {
    /// Optional. Email address of customer's users in the admin role. Each email address must be associated with a Google Account.
    #[serde(rename="adminEmails")]
    
    pub admin_emails: Option<Vec<String>>,
    /// Output only. The ID of the company. Assigned by the server.
    #[serde(rename="companyId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub company_id: Option<i64>,
    /// Required. The name of the company. For example _XYZ Corp_. Displayed to the company's employees in the zero-touch enrollment portal.
    #[serde(rename="companyName")]
    
    pub company_name: Option<String>,
    /// Output only. The Google Workspace account associated with this customer. Only used for customer Companies.
    #[serde(rename="googleWorkspaceAccount")]
    
    pub google_workspace_account: Option<GoogleWorkspaceAccount>,
    /// Input only. The preferred locale of the customer represented as a BCP47 language code. This field is validated on input and requests containing unsupported language codes will be rejected. Supported language codes: Arabic (ar) Chinese (Hong Kong) (zh-HK) Chinese (Simplified) (zh-CN) Chinese (Traditional) (zh-TW) Czech (cs) Danish (da) Dutch (nl) English (UK) (en-GB) English (US) (en-US) Filipino (fil) Finnish (fi) French (fr) German (de) Hebrew (iw) Hindi (hi) Hungarian (hu) Indonesian (id) Italian (it) Japanese (ja) Korean (ko) Norwegian (Bokmal) (no) Polish (pl) Portuguese (Brazil) (pt-BR) Portuguese (Portugal) (pt-PT) Russian (ru) Spanish (es) Spanish (Latin America) (es-419) Swedish (sv) Thai (th) Turkish (tr) Ukrainian (uk) Vietnamese (vi)
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Output only. The API resource name of the company. The resource name is one of the following formats: * `partners/[PARTNER_ID]/customers/[CUSTOMER_ID]` * `partners/[PARTNER_ID]/vendors/[VENDOR_ID]` * `partners/[PARTNER_ID]/vendors/[VENDOR_ID]/customers/[CUSTOMER_ID]` Assigned by the server.
    
    pub name: Option<String>,
    /// Required. Input only. Email address of customer's users in the owner role. At least one `owner_email` is required. Owners share the same access as admins but can also add, delete, and edit your organization's portal users.
    #[serde(rename="ownerEmails")]
    
    pub owner_emails: Option<Vec<String>>,
    /// Input only. If set to true, welcome email will not be sent to the customer. It is recommended to skip the welcome email if devices will be claimed with additional DEVICE_PROTECTION service, as the customer will receive separate emails at device claim time. This field is ignored if this is not a Zero-touch customer.
    #[serde(rename="skipWelcomeEmail")]
    
    pub skip_welcome_email: Option<bool>,
    /// Output only. Whether any user from the company has accepted the latest Terms of Service (ToS). See TermsStatus.
    #[serde(rename="termsStatus")]
    
    pub terms_status: Option<CompanyTermsStatusEnum>,
}

impl client::ResponseResult for Company {}


/// A configuration collects the provisioning options for Android devices. Each configuration combines the following: * The EMM device policy controller (DPC) installed on the devices. * EMM policies enforced on the devices. * Metadata displayed on the device to help users during setup. Customers can add as many configurations as they need. However, zero-touch enrollment works best when a customer sets a default configuration that’s applied to any new devices the organization purchases.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [configurations create customers](CustomerConfigurationCreateCall) (request|response)
/// * [configurations get customers](CustomerConfigurationGetCall) (response)
/// * [configurations patch customers](CustomerConfigurationPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Configuration {
    /// Required. The name of the organization. Zero-touch enrollment shows this organization name to device users during device provisioning.
    #[serde(rename="companyName")]
    
    pub company_name: Option<String>,
    /// Output only. The ID of the configuration. Assigned by the server.
    #[serde(rename="configurationId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub configuration_id: Option<i64>,
    /// Required. A short name that describes the configuration's purpose. For example, _Sales team_ or _Temporary employees_. The zero-touch enrollment portal displays this name to IT admins.
    #[serde(rename="configurationName")]
    
    pub configuration_name: Option<String>,
    /// Required. The email address that device users can contact to get help. Zero-touch enrollment shows this email address to device users before device provisioning. The value is validated on input.
    #[serde(rename="contactEmail")]
    
    pub contact_email: Option<String>,
    /// Required. The telephone number that device users can call, using another device, to get help. Zero-touch enrollment shows this number to device users before device provisioning. Accepts numerals, spaces, the plus sign, hyphens, and parentheses.
    #[serde(rename="contactPhone")]
    
    pub contact_phone: Option<String>,
    /// A message, containing one or two sentences, to help device users get help or give them more details about what’s happening to their device. Zero-touch enrollment shows this message before the device is provisioned.
    #[serde(rename="customMessage")]
    
    pub custom_message: Option<String>,
    /// The JSON-formatted EMM provisioning extras that are passed to the DPC.
    #[serde(rename="dpcExtras")]
    
    pub dpc_extras: Option<String>,
    /// Required. The resource name of the selected DPC (device policy controller) in the format `customers/[CUSTOMER_ID]/dpcs/*`. To list the supported DPCs, call `customers.dpcs.list`.
    #[serde(rename="dpcResourcePath")]
    
    pub dpc_resource_path: Option<String>,
    /// Required. Whether this is the default configuration that zero-touch enrollment applies to any new devices the organization purchases in the future. Only one customer configuration can be the default. Setting this value to `true`, changes the previous default configuration's `isDefault` value to `false`.
    #[serde(rename="isDefault")]
    
    pub is_default: Option<bool>,
    /// Output only. The API resource name in the format `customers/[CUSTOMER_ID]/configurations/[CONFIGURATION_ID]`. Assigned by the server.
    
    pub name: Option<String>,
}

impl client::RequestValue for Configuration {}
impl client::ResponseResult for Configuration {}


/// Request message to create a customer.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customers create partners](PartnerCustomerCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateCustomerRequest {
    /// Required. The company data to populate the new customer. Must contain a value for `companyName` and at least one `owner_email` that's associated with a Google Account. The values for `companyId` and `name` must be empty.
    
    pub customer: Option<Company>,
}

impl client::RequestValue for CreateCustomerRequest {}


/// Request message for customer to assign a configuration to device.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices apply configuration customers](CustomerDeviceApplyConfigurationCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomerApplyConfigurationRequest {
    /// Required. The configuration applied to the device in the format `customers/[CUSTOMER_ID]/configurations/[CONFIGURATION_ID]`.
    
    pub configuration: Option<String>,
    /// Required. The device the configuration is applied to. There are custom validations in ApplyConfigurationRequestValidator
    
    pub device: Option<DeviceReference>,
}

impl client::RequestValue for CustomerApplyConfigurationRequest {}


/// Response message of customer’s listing configuration.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [configurations list customers](CustomerConfigurationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomerListConfigurationsResponse {
    /// The configurations.
    
    pub configurations: Option<Vec<Configuration>>,
}

impl client::ResponseResult for CustomerListConfigurationsResponse {}


/// Response message for listing my customers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list customers](CustomerListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomerListCustomersResponse {
    /// The customer accounts the calling user is a member of.
    
    pub customers: Option<Vec<Company>>,
    /// A token used to access the next page of results. Omitted if no further results are available.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for CustomerListCustomersResponse {}


/// Response message of customer’s liting devices.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices list customers](CustomerDeviceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomerListDevicesResponse {
    /// The customer's devices.
    
    pub devices: Option<Vec<Device>>,
    /// A token used to access the next page of results. Omitted if no further results are available.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for CustomerListDevicesResponse {}


/// Response message of customer’s listing DPCs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [dpcs list customers](CustomerDpcListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomerListDpcsResponse {
    /// The list of DPCs available to the customer that support zero-touch enrollment.
    
    pub dpcs: Option<Vec<Dpc>>,
}

impl client::ResponseResult for CustomerListDpcsResponse {}


/// Request message for customer to remove the configuration from device.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices remove configuration customers](CustomerDeviceRemoveConfigurationCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomerRemoveConfigurationRequest {
    /// Required. The device to remove the configuration from. There are custom validations in RemoveConfigurationRequestValidator
    
    pub device: Option<DeviceReference>,
}

impl client::RequestValue for CustomerRemoveConfigurationRequest {}


/// Request message for customer to unclaim a device.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices unclaim customers](CustomerDeviceUnclaimCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomerUnclaimDeviceRequest {
    /// Required. The device to unclaim. There are custom validations in UnclaimDeviceRequestValidator.
    
    pub device: Option<DeviceReference>,
}

impl client::RequestValue for CustomerUnclaimDeviceRequest {}


/// An Android or Chrome OS device registered for zero-touch enrollment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices get customers](CustomerDeviceGetCall) (response)
/// * [devices get partners](PartnerDeviceGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Device {
    /// Output only. The provisioning claims for a device. Devices claimed for zero-touch enrollment have a claim with the type `SECTION_TYPE_ZERO_TOUCH`. Call `partners.devices.unclaim` or `partners.devices.unclaimAsync` to remove the device from zero-touch enrollment.
    
    pub claims: Option<Vec<DeviceClaim>>,
    /// Not available to resellers.
    
    pub configuration: Option<String>,
    /// Output only. The ID of the device. Assigned by the server.
    #[serde(rename="deviceId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub device_id: Option<i64>,
    /// The hardware IDs that identify a manufactured device. To learn more, read [Identifiers](https://developers.google.com/zero-touch/guides/identifiers).
    #[serde(rename="deviceIdentifier")]
    
    pub device_identifier: Option<DeviceIdentifier>,
    /// The metadata attached to the device. Structured as key-value pairs. To learn more, read [Device metadata](https://developers.google.com/zero-touch/guides/metadata).
    #[serde(rename="deviceMetadata")]
    
    pub device_metadata: Option<DeviceMetadata>,
    /// Output only. The API resource name in the format `partners/[PARTNER_ID]/devices/[DEVICE_ID]`. Assigned by the server.
    
    pub name: Option<String>,
}

impl client::ResponseResult for Device {}


/// A record of a device claimed by a reseller for a customer. Devices claimed for zero-touch enrollment have a claim with the type `SECTION_TYPE_ZERO_TOUCH`. To learn more, read [Claim devices for customers](https://developers.google.com/zero-touch/guides/how-it-works#claim).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceClaim {
    /// The Additional service registered for the device.
    #[serde(rename="additionalService")]
    
    pub additional_service: Option<DeviceClaimAdditionalServiceEnum>,
    /// The ID of the Google Workspace account that owns the Chrome OS device.
    #[serde(rename="googleWorkspaceCustomerId")]
    
    pub google_workspace_customer_id: Option<String>,
    /// The ID of the Customer that purchased the device.
    #[serde(rename="ownerCompanyId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub owner_company_id: Option<i64>,
    /// The ID of the reseller that claimed the device.
    #[serde(rename="resellerId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub reseller_id: Option<i64>,
    /// Output only. The type of claim made on the device.
    #[serde(rename="sectionType")]
    
    pub section_type: Option<DeviceClaimSectionTypeEnum>,
    /// The timestamp when the device will exit ‘vacation mode’. This value is present iff the device is in 'vacation mode'.
    #[serde(rename="vacationModeExpireTime")]
    
    pub vacation_mode_expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The timestamp when the device was put into ‘vacation mode’. This value is present iff the device is in 'vacation mode'.
    #[serde(rename="vacationModeStartTime")]
    
    pub vacation_mode_start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for DeviceClaim {}


/// Encapsulates hardware and product IDs to identify a manufactured device. To understand requirements on identifier sets, read [Identifiers](https://developers.google.com/zero-touch/guides/identifiers).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceIdentifier {
    /// An identifier provided by OEMs, carried through the production and sales process. Only applicable to Chrome OS devices.
    #[serde(rename="chromeOsAttestedDeviceId")]
    
    pub chrome_os_attested_device_id: Option<String>,
    /// The type of the device
    #[serde(rename="deviceType")]
    
    pub device_type: Option<DeviceIdentifierDeviceTypeEnum>,
    /// The device’s IMEI number. Validated on input.
    
    pub imei: Option<String>,
    /// The device manufacturer’s name. Matches the device’s built-in value returned from `android.os.Build.MANUFACTURER`. Allowed values are listed in [Android manufacturers](https://developers.google.com/zero-touch/resources/manufacturer-names#manufacturers-names).
    
    pub manufacturer: Option<String>,
    /// The device’s MEID number.
    
    pub meid: Option<String>,
    /// The device model’s name. Allowed values are listed in [Android models](https://developers.google.com/zero-touch/resources/manufacturer-names#model-names) and [Chrome OS models](https://support.google.com/chrome/a/answer/10130175#identify_compatible).
    
    pub model: Option<String>,
    /// The manufacturer's serial number for the device. This value might not be unique across different device models.
    #[serde(rename="serialNumber")]
    
    pub serial_number: Option<String>,
}

impl client::Part for DeviceIdentifier {}


/// Metadata entries that can be attached to a `Device`. To learn more, read [Device metadata](https://developers.google.com/zero-touch/guides/metadata).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices metadata partners](PartnerDeviceMetadataCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceMetadata {
    /// Metadata entries recorded as key-value pairs.
    
    pub entries: Option<HashMap<String, String>>,
}

impl client::ResponseResult for DeviceMetadata {}


/// A `DeviceReference` is an API abstraction that lets you supply a _device_ argument to a method using one of the following identifier types: * A numeric API resource ID. * Real-world hardware IDs, such as IMEI number, belonging to the manufactured device. Methods that operate on devices take a `DeviceReference` as a parameter type because it's more flexible for the caller. To learn more about device identifiers, read [Identifiers](https://developers.google.com/zero-touch/guides/identifiers).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceReference {
    /// The ID of the device.
    #[serde(rename="deviceId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub device_id: Option<i64>,
    /// The hardware IDs of the device.
    #[serde(rename="deviceIdentifier")]
    
    pub device_identifier: Option<DeviceIdentifier>,
}

impl client::Part for DeviceReference {}


/// An EMM's DPC ([device policy controller](http://developer.android.com/work/dpc/build-dpc.html)). Zero-touch enrollment installs a DPC (listed in the `Configuration`) on a device to maintain the customer's mobile policies. All the DPCs listed by the API support zero-touch enrollment and are available in Google Play.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Dpc {
    /// Output only. The title of the DPC app in Google Play. For example, _Google Apps Device Policy_. Useful in an application's user interface.
    #[serde(rename="dpcName")]
    
    pub dpc_name: Option<String>,
    /// Output only. The API resource name in the format `customers/[CUSTOMER_ID]/dpcs/[DPC_ID]`. Assigned by the server. To maintain a reference to a DPC across customer accounts, persist and match the last path component (`DPC_ID`).
    
    pub name: Option<String>,
    /// Output only. The DPC's Android application ID that looks like a Java package name. Zero-touch enrollment installs the DPC app onto a device using this identifier.
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
}

impl client::Part for Dpc {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [configurations delete customers](CustomerConfigurationDeleteCall) (response)
/// * [devices apply configuration customers](CustomerDeviceApplyConfigurationCall) (response)
/// * [devices remove configuration customers](CustomerDeviceRemoveConfigurationCall) (response)
/// * [devices unclaim customers](CustomerDeviceUnclaimCall) (response)
/// * [devices unclaim partners](PartnerDeviceUnclaimCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Request to find devices.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices find by identifier partners](PartnerDeviceFindByIdentifierCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FindDevicesByDeviceIdentifierRequest {
    /// Required. Required. The device identifier to search for.
    #[serde(rename="deviceIdentifier")]
    
    pub device_identifier: Option<DeviceIdentifier>,
    /// Required. The maximum number of devices to show in a page of results. Must be between 1 and 100 inclusive.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub limit: Option<i64>,
    /// A token specifying which result page to return.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
}

impl client::RequestValue for FindDevicesByDeviceIdentifierRequest {}


/// Response containing found devices.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices find by identifier partners](PartnerDeviceFindByIdentifierCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FindDevicesByDeviceIdentifierResponse {
    /// Found devices.
    
    pub devices: Option<Vec<Device>>,
    /// A token used to access the next page of results. Omitted if no further results are available.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The total count of items in the list irrespective of pagination.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for FindDevicesByDeviceIdentifierResponse {}


/// Request to find devices by customers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices find by owner partners](PartnerDeviceFindByOwnerCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FindDevicesByOwnerRequest {
    /// The list of customer IDs to search for.
    #[serde(rename="customerId")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub customer_id: Option<Vec<i64>>,
    /// The list of IDs of Google Workspace accounts to search for.
    #[serde(rename="googleWorkspaceCustomerId")]
    
    pub google_workspace_customer_id: Option<Vec<String>>,
    /// Required. The maximum number of devices to show in a page of results. Must be between 1 and 100 inclusive.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub limit: Option<i64>,
    /// A token specifying which result page to return.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
    /// Required. The section type of the device's provisioning record.
    #[serde(rename="sectionType")]
    
    pub section_type: Option<FindDevicesByOwnerRequestSectionTypeEnum>,
}

impl client::RequestValue for FindDevicesByOwnerRequest {}


/// Response containing found devices.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices find by owner partners](PartnerDeviceFindByOwnerCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FindDevicesByOwnerResponse {
    /// The customer's devices.
    
    pub devices: Option<Vec<Device>>,
    /// A token used to access the next page of results. Omitted if no further results are available.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The total count of items in the list irrespective of pagination.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for FindDevicesByOwnerResponse {}


/// A Google Workspace customer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleWorkspaceAccount {
    /// Required. The customer ID.
    #[serde(rename="customerId")]
    
    pub customer_id: Option<String>,
    /// Output only. The pre-provisioning tokens previously used to claim devices.
    #[serde(rename="preProvisioningTokens")]
    
    pub pre_provisioning_tokens: Option<Vec<String>>,
}

impl client::Part for GoogleWorkspaceAccount {}


/// Response message of all customers related to this partner.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customers list partners](PartnerCustomerListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCustomersResponse {
    /// List of customers related to this reseller partner.
    
    pub customers: Option<Vec<Company>>,
    /// A token to retrieve the next page of results. Omitted if no further results are available.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The total count of items in the list irrespective of pagination.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for ListCustomersResponse {}


/// Response message to list customers of the vendor.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [vendors customers list partners](PartnerVendorCustomerListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListVendorCustomersResponse {
    /// List of customers of the vendor.
    
    pub customers: Option<Vec<Company>>,
    /// A token to retrieve the next page of results. Omitted if no further results are available.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The total count of items in the list irrespective of pagination.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for ListVendorCustomersResponse {}


/// Response message to list vendors of the partner.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [vendors list partners](PartnerVendorListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListVendorsResponse {
    /// A token to retrieve the next page of results. Omitted if no further results are available.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The total count of items in the list irrespective of pagination.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
    /// List of vendors of the reseller partner. Fields `name`, `companyId` and `companyName` are populated to the Company object.
    
    pub vendors: Option<Vec<Company>>,
}

impl client::ResponseResult for ListVendorsResponse {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get operations](OperationGetCall) (response)
/// * [devices claim async partners](PartnerDeviceClaimAsyncCall) (response)
/// * [devices unclaim async partners](PartnerDeviceUnclaimAsyncCall) (response)
/// * [devices update metadata async partners](PartnerDeviceUpdateMetadataAsyncCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// This field will always be not set if the operation is created by `claimAsync`, `unclaimAsync`, or `updateMetadataAsync`. In this case, error information for each device is set in `response.perDeviceStatus.result.status`.
    
    pub error: Option<Status>,
    /// This field will contain a `DevicesLongRunningOperationMetadata` object if the operation is created by `claimAsync`, `unclaimAsync`, or `updateMetadataAsync`.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// This field will contain a `DevicesLongRunningOperationResponse` object if the operation is created by `claimAsync`, `unclaimAsync`, or `updateMetadataAsync`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::Resource for Operation {}
impl client::ResponseResult for Operation {}


/// Identifies one claim request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PartnerClaim {
    /// The ID of the customer for whom the device is being claimed.
    #[serde(rename="customerId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub customer_id: Option<i64>,
    /// Required. Required. Device identifier of the device.
    #[serde(rename="deviceIdentifier")]
    
    pub device_identifier: Option<DeviceIdentifier>,
    /// Required. The metadata to attach to the device at claim.
    #[serde(rename="deviceMetadata")]
    
    pub device_metadata: Option<DeviceMetadata>,
    /// The Google Workspace customer ID.
    #[serde(rename="googleWorkspaceCustomerId")]
    
    pub google_workspace_customer_id: Option<String>,
    /// Optional. Must and can only be set for Chrome OS devices.
    #[serde(rename="preProvisioningToken")]
    
    pub pre_provisioning_token: Option<String>,
    /// Required. The section type of the device's provisioning record.
    #[serde(rename="sectionType")]
    
    pub section_type: Option<PartnerClaimSectionTypeEnum>,
}

impl client::Part for PartnerClaim {}


/// Identifies one unclaim request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PartnerUnclaim {
    /// Required. Device ID of the device.
    #[serde(rename="deviceId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub device_id: Option<i64>,
    /// Required. Device identifier of the device.
    #[serde(rename="deviceIdentifier")]
    
    pub device_identifier: Option<DeviceIdentifier>,
    /// Required. The section type of the device's provisioning record.
    #[serde(rename="sectionType")]
    
    pub section_type: Option<PartnerUnclaimSectionTypeEnum>,
    /// Optional. The duration of the vacation unlock starting from when the request is processed. (1 day is treated as 24 hours)
    #[serde(rename="vacationModeDays")]
    
    pub vacation_mode_days: Option<i32>,
    /// Optional. The expiration time of the vacation unlock.
    #[serde(rename="vacationModeExpireTime")]
    
    pub vacation_mode_expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for PartnerUnclaim {}


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


/// Request message to unclaim a device.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices unclaim partners](PartnerDeviceUnclaimCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UnclaimDeviceRequest {
    /// Required. The device ID returned by `ClaimDevice`.
    #[serde(rename="deviceId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub device_id: Option<i64>,
    /// Required. The device identifier you used when you claimed this device.
    #[serde(rename="deviceIdentifier")]
    
    pub device_identifier: Option<DeviceIdentifier>,
    /// Required. The section type of the device's provisioning record.
    #[serde(rename="sectionType")]
    
    pub section_type: Option<UnclaimDeviceRequestSectionTypeEnum>,
    /// The duration of the vacation unlock starting from when the request is processed. (1 day is treated as 24 hours)
    #[serde(rename="vacationModeDays")]
    
    pub vacation_mode_days: Option<i32>,
    /// The expiration time of the vacation unlock.
    #[serde(rename="vacationModeExpireTime")]
    
    pub vacation_mode_expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for UnclaimDeviceRequest {}


/// Request to unclaim devices asynchronously in batch.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices unclaim async partners](PartnerDeviceUnclaimAsyncCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UnclaimDevicesRequest {
    /// Required. The list of devices to unclaim.
    
    pub unclaims: Option<Vec<PartnerUnclaim>>,
}

impl client::RequestValue for UnclaimDevicesRequest {}


/// Request to update device metadata in batch.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices update metadata async partners](PartnerDeviceUpdateMetadataAsyncCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateDeviceMetadataInBatchRequest {
    /// Required. The list of metadata updates.
    
    pub updates: Option<Vec<UpdateMetadataArguments>>,
}

impl client::RequestValue for UpdateDeviceMetadataInBatchRequest {}


/// Request to set metadata for a device.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices metadata partners](PartnerDeviceMetadataCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateDeviceMetadataRequest {
    /// Required. The metadata to attach to the device.
    #[serde(rename="deviceMetadata")]
    
    pub device_metadata: Option<DeviceMetadata>,
}

impl client::RequestValue for UpdateDeviceMetadataRequest {}


/// Identifies metadata updates to one device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateMetadataArguments {
    /// Required. Device ID of the device.
    #[serde(rename="deviceId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub device_id: Option<i64>,
    /// Required. Device identifier.
    #[serde(rename="deviceIdentifier")]
    
    pub device_identifier: Option<DeviceIdentifier>,
    /// Required. The metadata to update.
    #[serde(rename="deviceMetadata")]
    
    pub device_metadata: Option<DeviceMetadata>,
}

impl client::Part for UpdateMetadataArguments {}


