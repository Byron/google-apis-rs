use super::*;
/// Associates `members` with a `role`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalAssignment {
    /// The identities the role is assigned to. It can have the following values: * `{user_email}`: An email address that represents a specific Google account. For example: `alice@gmail.com`. * `{group_email}`: An email address that represents a Google group. For example, `viewers@gmail.com`.
    
    pub members: Option<Vec<String>>,
    /// Required. Role that is assigned to `members`.
    
    pub role: Option<String>,
}

impl client::Part for SasPortalAssignment {}


/// The channel with score.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalChannelWithScore {
    /// The frequency range of the channel.
    #[serde(rename="frequencyRange")]
    
    pub frequency_range: Option<SasPortalFrequencyRange>,
    /// The channel score, normalized to be in the range [0,100].
    
    pub score: Option<f64>,
}

impl client::Part for SasPortalChannelWithScore {}


/// Request for CreateSignedDevice.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [deployments devices create signed customers](CustomerDeploymentDeviceCreateSignedCall) (request)
/// * [devices create signed customers](CustomerDeviceCreateSignedCall) (request)
/// * [nodes devices create signed customers](CustomerNodeDeviceCreateSignedCall) (request)
/// * [deployments devices create signed nodes](NodeDeploymentDeviceCreateSignedCall) (request)
/// * [devices create signed nodes](NodeDeviceCreateSignedCall) (request)
/// * [nodes devices create signed nodes](NodeNodeDeviceCreateSignedCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalCreateSignedDeviceRequest {
    /// Required. JSON Web Token signed using a CPI private key. Payload must be the JSON encoding of the device. The user_id field must be set.
    #[serde(rename="encodedDevice")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub encoded_device: Option<Vec<u8>>,
    /// Required. Unique installer id (CPI ID) from the Certified Professional Installers database.
    #[serde(rename="installerId")]
    
    pub installer_id: Option<String>,
}

impl client::RequestValue for SasPortalCreateSignedDeviceRequest {}


/// Entity representing a SAS customer.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get customers](CustomerGetCall) (response)
/// * [patch customers](CustomerPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalCustomer {
    /// Required. Name of the organization that the customer entity represents.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Resource name of the customer.
    
    pub name: Option<String>,
    /// User IDs used by the devices belonging to this customer.
    #[serde(rename="sasUserIds")]
    
    pub sas_user_ids: Option<Vec<String>>,
}

impl client::RequestValue for SasPortalCustomer {}
impl client::ResponseResult for SasPortalCustomer {}


/// The Deployment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [deployments create customers](CustomerDeploymentCreateCall) (request|response)
/// * [deployments get customers](CustomerDeploymentGetCall) (response)
/// * [deployments patch customers](CustomerDeploymentPatchCall) (request|response)
/// * [nodes deployments create customers](CustomerNodeDeploymentCreateCall) (request|response)
/// * [get deployments](DeploymentGetCall) (response)
/// * [deployments get nodes](NodeDeploymentGetCall) (response)
/// * [deployments patch nodes](NodeDeploymentPatchCall) (request|response)
/// * [nodes deployments create nodes](NodeNodeDeploymentCreateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalDeployment {
    /// The deployment's display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The FCC Registration Numbers (FRNs) copied from its direct parent.
    
    pub frns: Option<Vec<String>>,
    /// Output only. Resource name.
    
    pub name: Option<String>,
    /// User ID used by the devices belonging to this deployment. Each deployment should be associated with one unique user ID.
    #[serde(rename="sasUserIds")]
    
    pub sas_user_ids: Option<Vec<String>>,
}

impl client::RequestValue for SasPortalDeployment {}
impl client::ResponseResult for SasPortalDeployment {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [deployments devices create customers](CustomerDeploymentDeviceCreateCall) (request|response)
/// * [deployments devices create signed customers](CustomerDeploymentDeviceCreateSignedCall) (response)
/// * [devices create customers](CustomerDeviceCreateCall) (request|response)
/// * [devices create signed customers](CustomerDeviceCreateSignedCall) (response)
/// * [devices get customers](CustomerDeviceGetCall) (response)
/// * [devices patch customers](CustomerDevicePatchCall) (request|response)
/// * [devices update signed customers](CustomerDeviceUpdateSignedCall) (response)
/// * [nodes devices create customers](CustomerNodeDeviceCreateCall) (request|response)
/// * [nodes devices create signed customers](CustomerNodeDeviceCreateSignedCall) (response)
/// * [devices get deployments](DeploymentDeviceGetCall) (response)
/// * [devices patch deployments](DeploymentDevicePatchCall) (request|response)
/// * [devices update signed deployments](DeploymentDeviceUpdateSignedCall) (response)
/// * [deployments devices create nodes](NodeDeploymentDeviceCreateCall) (request|response)
/// * [deployments devices create signed nodes](NodeDeploymentDeviceCreateSignedCall) (response)
/// * [devices create nodes](NodeDeviceCreateCall) (request|response)
/// * [devices create signed nodes](NodeDeviceCreateSignedCall) (response)
/// * [devices get nodes](NodeDeviceGetCall) (response)
/// * [devices patch nodes](NodeDevicePatchCall) (request|response)
/// * [devices update signed nodes](NodeDeviceUpdateSignedCall) (response)
/// * [nodes devices create nodes](NodeNodeDeviceCreateCall) (request|response)
/// * [nodes devices create signed nodes](NodeNodeDeviceCreateSignedCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalDevice {
    /// Output only. Current configuration of the device as registered to the SAS.
    #[serde(rename="activeConfig")]
    
    pub active_config: Option<SasPortalDeviceConfig>,
    /// Output only. Current channels with scores.
    #[serde(rename="currentChannels")]
    
    pub current_channels: Option<Vec<SasPortalChannelWithScore>>,
    /// Device parameters that can be overridden by both SAS Portal and SAS registration requests.
    #[serde(rename="deviceMetadata")]
    
    pub device_metadata: Option<SasPortalDeviceMetadata>,
    /// Device display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The FCC identifier of the device.
    #[serde(rename="fccId")]
    
    pub fcc_id: Option<String>,
    /// Only ranges that are within the allowlists are available for new grants.
    #[serde(rename="grantRangeAllowlists")]
    
    pub grant_range_allowlists: Option<Vec<SasPortalFrequencyRange>>,
    /// Output only. Grants held by the device.
    
    pub grants: Option<Vec<SasPortalDeviceGrant>>,
    /// Output only. The resource path name.
    
    pub name: Option<String>,
    /// Configuration of the device, as specified via SAS Portal API.
    #[serde(rename="preloadedConfig")]
    
    pub preloaded_config: Option<SasPortalDeviceConfig>,
    /// A serial number assigned to the device by the device manufacturer.
    #[serde(rename="serialNumber")]
    
    pub serial_number: Option<String>,
    /// Output only. Device state.
    
    pub state: Option<SasPortalDeviceStateEnum>,
}

impl client::RequestValue for SasPortalDevice {}
impl client::ResponseResult for SasPortalDevice {}


/// Information about the device's air interface.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalDeviceAirInterface {
    /// Conditional. This field specifies the radio access technology that is used for the CBSD.
    #[serde(rename="radioTechnology")]
    
    pub radio_technology: Option<SasPortalDeviceAirInterfaceRadioTechnologyEnum>,
    /// Optional. This field is related to the `radioTechnology` and provides the air interface specification that the CBSD is compliant with at the time of registration.
    #[serde(rename="supportedSpec")]
    
    pub supported_spec: Option<String>,
}

impl client::Part for SasPortalDeviceAirInterface {}


/// Information about the device configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalDeviceConfig {
    /// Information about this device's air interface.
    #[serde(rename="airInterface")]
    
    pub air_interface: Option<SasPortalDeviceAirInterface>,
    /// The call sign of the device operator.
    #[serde(rename="callSign")]
    
    pub call_sign: Option<String>,
    /// FCC category of the device.
    
    pub category: Option<SasPortalDeviceConfigCategoryEnum>,
    /// Installation parameters for the device.
    #[serde(rename="installationParams")]
    
    pub installation_params: Option<SasPortalInstallationParams>,
    /// Output only. Whether the configuration has been signed by a CPI.
    #[serde(rename="isSigned")]
    
    pub is_signed: Option<bool>,
    /// Measurement reporting capabilities of the device.
    #[serde(rename="measurementCapabilities")]
    
    pub measurement_capabilities: Option<Vec<SasPortalDeviceConfigMeasurementCapabilitiesEnum>>,
    /// Information about this device model.
    
    pub model: Option<SasPortalDeviceModel>,
    /// State of the configuration.
    
    pub state: Option<SasPortalDeviceConfigStateEnum>,
    /// Output only. The last time the device configuration was edited.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The identifier of a device user.
    #[serde(rename="userId")]
    
    pub user_id: Option<String>,
}

impl client::Part for SasPortalDeviceConfig {}


/// Device grant. It is an authorization provided by the Spectrum Access System to a device to transmit using specified operating parameters after a successful heartbeat by the device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalDeviceGrant {
    /// Type of channel used.
    #[serde(rename="channelType")]
    
    pub channel_type: Option<SasPortalDeviceGrantChannelTypeEnum>,
    /// The expiration time of the grant.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The transmission frequency range.
    #[serde(rename="frequencyRange")]
    
    pub frequency_range: Option<SasPortalFrequencyRange>,
    /// Grant Id.
    #[serde(rename="grantId")]
    
    pub grant_id: Option<String>,
    /// The transmit expiration time of the last heartbeat.
    #[serde(rename="lastHeartbeatTransmitExpireTime")]
    
    pub last_heartbeat_transmit_expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Maximum Equivalent Isotropically Radiated Power (EIRP) permitted by the grant. The maximum EIRP is in units of dBm/MHz. The value of `maxEirp` represents the average (RMS) EIRP that would be measured by the procedure defined in FCC part 96.41(e)(3).
    #[serde(rename="maxEirp")]
    
    pub max_eirp: Option<f64>,
    /// The DPA move lists on which this grant appears.
    #[serde(rename="moveList")]
    
    pub move_list: Option<Vec<SasPortalDpaMoveList>>,
    /// State of the grant.
    
    pub state: Option<SasPortalDeviceGrantStateEnum>,
    /// If the grant is suspended, the reason(s) for suspension.
    #[serde(rename="suspensionReason")]
    
    pub suspension_reason: Option<Vec<String>>,
}

impl client::Part for SasPortalDeviceGrant {}


/// Device data overridable by both SAS Portal and registration requests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalDeviceMetadata {
    /// If populated, the Antenna Model Pattern to use. Format is: `RecordCreatorId:PatternId`
    #[serde(rename="antennaModel")]
    
    pub antenna_model: Option<String>,
    /// Common Channel Group (CCG). A group of CBSDs in the same ICG requesting a common primary channel assignment. For more details, see [CBRSA-TS-2001 V3.0.0](https://ongoalliance.org/wp-content/uploads/2020/02/CBRSA-TS-2001-V3.0.0_Approved-for-publication.pdf).
    #[serde(rename="commonChannelGroup")]
    
    pub common_channel_group: Option<String>,
    /// Interference Coordination Group (ICG). A group of CBSDs that manage their own interference with the group. For more details, see [CBRSA-TS-2001 V3.0.0](https://ongoalliance.org/wp-content/uploads/2020/02/CBRSA-TS-2001-V3.0.0_Approved-for-publication.pdf).
    #[serde(rename="interferenceCoordinationGroup")]
    
    pub interference_coordination_group: Option<String>,
    /// Output only. Set to `true` if a CPI has validated that they have coordinated with the National Quiet Zone office.
    #[serde(rename="nrqzValidated")]
    
    pub nrqz_validated: Option<bool>,
    /// Output only. National Radio Quiet Zone validation info.
    #[serde(rename="nrqzValidation")]
    
    pub nrqz_validation: Option<SasPortalNrqzValidation>,
}

impl client::Part for SasPortalDeviceMetadata {}


/// Information about the model of the device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalDeviceModel {
    /// The firmware version of the device.
    #[serde(rename="firmwareVersion")]
    
    pub firmware_version: Option<String>,
    /// The hardware version of the device.
    #[serde(rename="hardwareVersion")]
    
    pub hardware_version: Option<String>,
    /// The name of the device model.
    
    pub name: Option<String>,
    /// The software version of the device.
    #[serde(rename="softwareVersion")]
    
    pub software_version: Option<String>,
    /// The name of the device vendor.
    
    pub vendor: Option<String>,
}

impl client::Part for SasPortalDeviceModel {}


/// An entry in a DPA's move list.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalDpaMoveList {
    /// The ID of the DPA.
    #[serde(rename="dpaId")]
    
    pub dpa_id: Option<String>,
    /// The frequency range that the move list affects.
    #[serde(rename="frequencyRange")]
    
    pub frequency_range: Option<SasPortalFrequencyRange>,
}

impl client::Part for SasPortalDpaMoveList {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [deployments delete customers](CustomerDeploymentDeleteCall) (response)
/// * [devices delete customers](CustomerDeviceDeleteCall) (response)
/// * [devices sign device customers](CustomerDeviceSignDeviceCall) (response)
/// * [nodes delete customers](CustomerNodeDeleteCall) (response)
/// * [devices delete deployments](DeploymentDeviceDeleteCall) (response)
/// * [devices sign device deployments](DeploymentDeviceSignDeviceCall) (response)
/// * [deployments delete nodes](NodeDeploymentDeleteCall) (response)
/// * [devices delete nodes](NodeDeviceDeleteCall) (response)
/// * [devices sign device nodes](NodeDeviceSignDeviceCall) (response)
/// * [nodes delete nodes](NodeNodeDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalEmpty { _never_set: Option<bool> }

impl client::ResponseResult for SasPortalEmpty {}


/// Frequency range from `low_frequency` to `high_frequency`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalFrequencyRange {
    /// The highest frequency of the frequency range in MHz.
    #[serde(rename="highFrequencyMhz")]
    
    pub high_frequency_mhz: Option<f64>,
    /// The lowest frequency of the frequency range in MHz.
    #[serde(rename="lowFrequencyMhz")]
    
    pub low_frequency_mhz: Option<f64>,
}

impl client::Part for SasPortalFrequencyRange {}


/// Request for GenerateSecret.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [generate secret installer](InstallerGenerateSecretCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalGenerateSecretRequest { _never_set: Option<bool> }

impl client::RequestValue for SasPortalGenerateSecretRequest {}


/// Response for GenerateSecret.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [generate secret installer](InstallerGenerateSecretCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalGenerateSecretResponse {
    /// The secret generated by the string and used by ValidateInstaller.
    
    pub secret: Option<String>,
}

impl client::ResponseResult for SasPortalGenerateSecretResponse {}


/// Request message for `GetPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get policies](PolicyGetCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalGetPolicyRequest {
    /// Required. The resource for which the policy is being requested.
    
    pub resource: Option<String>,
}

impl client::RequestValue for SasPortalGetPolicyRequest {}


/// Information about the device installation parameters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalInstallationParams {
    /// Boresight direction of the horizontal plane of the antenna in degrees with respect to true north. The value of this parameter is an integer with a value between 0 and 359 inclusive. A value of 0 degrees means true north; a value of 90 degrees means east. This parameter is optional for Category A devices and conditional for Category B devices.
    #[serde(rename="antennaAzimuth")]
    
    pub antenna_azimuth: Option<i32>,
    /// 3-dB antenna beamwidth of the antenna in the horizontal-plane in degrees. This parameter is an unsigned integer having a value between 0 and 360 (degrees) inclusive; it is optional for Category A devices and conditional for Category B devices.
    #[serde(rename="antennaBeamwidth")]
    
    pub antenna_beamwidth: Option<i32>,
    /// Antenna downtilt in degrees and is an integer with a value between -90 and +90 inclusive; a negative value means the antenna is tilted up (above horizontal). This parameter is optional for Category A devices and conditional for Category B devices.
    #[serde(rename="antennaDowntilt")]
    
    pub antenna_downtilt: Option<i32>,
    /// Peak antenna gain in dBi. This parameter is an integer with a value between -127 and +128 (dBi) inclusive.
    #[serde(rename="antennaGain")]
    
    pub antenna_gain: Option<i32>,
    /// As above, but as a DoubleValue.
    #[serde(rename="antennaGainNewField")]
    
    pub antenna_gain_new_field: Option<f64>,
    /// If an external antenna is used, the antenna model is optionally provided in this field. The string has a maximum length of 128 octets.
    #[serde(rename="antennaModel")]
    
    pub antenna_model: Option<String>,
    /// If present, this parameter specifies whether the CBSD is a CPE-CBSD or not.
    #[serde(rename="cpeCbsdIndication")]
    
    pub cpe_cbsd_indication: Option<bool>,
    /// This parameter is the maximum device EIRP in units of dBm/10MHz and is an integer with a value between -127 and +47 (dBm/10 MHz) inclusive. If not included, SAS interprets it as maximum allowable EIRP in units of dBm/10MHz for device category.
    #[serde(rename="eirpCapability")]
    
    pub eirp_capability: Option<i32>,
    /// As above, but as a DoubleValue.
    #[serde(rename="eirpCapabilityNewField")]
    
    pub eirp_capability_new_field: Option<f64>,
    /// Device antenna height in meters. When the `heightType` parameter value is "AGL", the antenna height should be given relative to ground level. When the `heightType` parameter value is "AMSL", it is given with respect to WGS84 datum.
    
    pub height: Option<f64>,
    /// Specifies how the height is measured.
    #[serde(rename="heightType")]
    
    pub height_type: Option<SasPortalInstallationParamHeightTypeEnum>,
    /// A positive number in meters to indicate accuracy of the device antenna horizontal location. This optional parameter should only be present if its value is less than the FCC requirement of 50 meters.
    #[serde(rename="horizontalAccuracy")]
    
    pub horizontal_accuracy: Option<f64>,
    /// Whether the device antenna is indoor or not. `true`: indoor. `false`: outdoor.
    #[serde(rename="indoorDeployment")]
    
    pub indoor_deployment: Option<bool>,
    /// Latitude of the device antenna location in degrees relative to the WGS 84 datum. The allowed range is from -90.000000 to +90.000000. Positive values represent latitudes north of the equator; negative values south of the equator.
    
    pub latitude: Option<f64>,
    /// Longitude of the device antenna location in degrees relative to the WGS 84 datum. The allowed range is from -180.000000 to +180.000000. Positive values represent longitudes east of the prime meridian; negative values west of the prime meridian.
    
    pub longitude: Option<f64>,
    /// A positive number in meters to indicate accuracy of the device antenna vertical location. This optional parameter should only be present if its value is less than the FCC requirement of 3 meters.
    #[serde(rename="verticalAccuracy")]
    
    pub vertical_accuracy: Option<f64>,
}

impl client::Part for SasPortalInstallationParams {}


/// Response for `ListCustomers`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list customers](CustomerListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalListCustomersResponse {
    /// The list of customers that match the request.
    
    pub customers: Option<Vec<SasPortalCustomer>>,
    /// A pagination token returned from a previous call to ListCustomers that indicates from where listing should continue. If the field is missing or empty, it means there are no more customers.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for SasPortalListCustomersResponse {}


/// Response for ListDeployments.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [deployments list customers](CustomerDeploymentListCall) (response)
/// * [nodes deployments list customers](CustomerNodeDeploymentListCall) (response)
/// * [deployments list nodes](NodeDeploymentListCall) (response)
/// * [nodes deployments list nodes](NodeNodeDeploymentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalListDeploymentsResponse {
    /// The deployments that match the request.
    
    pub deployments: Option<Vec<SasPortalDeployment>>,
    /// A pagination token returned from a previous call to ListDeployments that indicates from where listing should continue. If the field is missing or empty, it means there are no more deployments.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for SasPortalListDeploymentsResponse {}


/// Response for ListDevices.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [deployments devices list customers](CustomerDeploymentDeviceListCall) (response)
/// * [devices list customers](CustomerDeviceListCall) (response)
/// * [nodes devices list customers](CustomerNodeDeviceListCall) (response)
/// * [deployments devices list nodes](NodeDeploymentDeviceListCall) (response)
/// * [devices list nodes](NodeDeviceListCall) (response)
/// * [nodes devices list nodes](NodeNodeDeviceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalListDevicesResponse {
    /// The devices that match the request.
    
    pub devices: Option<Vec<SasPortalDevice>>,
    /// A pagination token returned from a previous call to ListDevices that indicates from where listing should continue. If the field is missing or empty, it means there is no more devices.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for SasPortalListDevicesResponse {}


/// Response for ListNodes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [nodes nodes list customers](CustomerNodeNodeListCall) (response)
/// * [nodes list customers](CustomerNodeListCall) (response)
/// * [nodes nodes list nodes](NodeNodeNodeListCall) (response)
/// * [nodes list nodes](NodeNodeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalListNodesResponse {
    /// A pagination token returned from a previous call to ListNodes that indicates from where listing should continue. If the field is missing or empty, it means there is no more nodes.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The nodes that match the request.
    
    pub nodes: Option<Vec<SasPortalNode>>,
}

impl client::ResponseResult for SasPortalListNodesResponse {}


/// Request for MoveDeployment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [deployments move customers](CustomerDeploymentMoveCall) (request)
/// * [deployments move nodes](NodeDeploymentMoveCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalMoveDeploymentRequest {
    /// Required. The name of the new parent resource node or customer to reparent the deployment under.
    
    pub destination: Option<String>,
}

impl client::RequestValue for SasPortalMoveDeploymentRequest {}


/// Request for MoveDevice.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices move customers](CustomerDeviceMoveCall) (request)
/// * [devices move deployments](DeploymentDeviceMoveCall) (request)
/// * [devices move nodes](NodeDeviceMoveCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalMoveDeviceRequest {
    /// Required. The name of the new parent resource node or customer to reparent the device under.
    
    pub destination: Option<String>,
}

impl client::RequestValue for SasPortalMoveDeviceRequest {}


/// Request for MoveNode.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [nodes move customers](CustomerNodeMoveCall) (request)
/// * [nodes move nodes](NodeNodeMoveCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalMoveNodeRequest {
    /// Required. The name of the new parent resource node or customer to reparent the node under.
    
    pub destination: Option<String>,
}

impl client::RequestValue for SasPortalMoveNodeRequest {}


/// The Node.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [nodes nodes create customers](CustomerNodeNodeCreateCall) (request|response)
/// * [nodes create customers](CustomerNodeCreateCall) (request|response)
/// * [nodes get customers](CustomerNodeGetCall) (response)
/// * [nodes patch customers](CustomerNodePatchCall) (request|response)
/// * [nodes nodes create nodes](NodeNodeNodeCreateCall) (request|response)
/// * [nodes create nodes](NodeNodeCreateCall) (request|response)
/// * [nodes get nodes](NodeNodeGetCall) (response)
/// * [nodes patch nodes](NodeNodePatchCall) (request|response)
/// * [get nodes](NodeGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalNode {
    /// The node's display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Resource name.
    
    pub name: Option<String>,
    /// User ids used by the devices belonging to this node.
    #[serde(rename="sasUserIds")]
    
    pub sas_user_ids: Option<Vec<String>>,
}

impl client::RequestValue for SasPortalNode {}
impl client::ResponseResult for SasPortalNode {}


/// Information about National Radio Quiet Zone validation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalNrqzValidation {
    /// Validation case ID.
    #[serde(rename="caseId")]
    
    pub case_id: Option<String>,
    /// CPI who signed the validation.
    #[serde(rename="cpiId")]
    
    pub cpi_id: Option<String>,
    /// Device latitude that's associated with the validation.
    
    pub latitude: Option<f64>,
    /// Device longitude that's associated with the validation.
    
    pub longitude: Option<f64>,
    /// State of the NRQZ validation info.
    
    pub state: Option<SasPortalNrqzValidationStateEnum>,
}

impl client::Part for SasPortalNrqzValidation {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [deployments move customers](CustomerDeploymentMoveCall) (response)
/// * [devices move customers](CustomerDeviceMoveCall) (response)
/// * [nodes move customers](CustomerNodeMoveCall) (response)
/// * [devices move deployments](DeploymentDeviceMoveCall) (response)
/// * [deployments move nodes](NodeDeploymentMoveCall) (response)
/// * [devices move nodes](NodeDeviceMoveCall) (response)
/// * [nodes move nodes](NodeNodeMoveCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalOperation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<SasPortalStatus>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for SasPortalOperation {}


/// Defines an access control policy to the resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get policies](PolicyGetCall) (response)
/// * [set policies](PolicySetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalPolicy {
    /// List of assignments
    
    pub assignments: Option<Vec<SasPortalAssignment>>,
    /// The etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the etag in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An etag is returned in the response to GetPolicy, and systems are expected to put that etag in the request to SetPolicy to ensure that their change will be applied to the same version of the policy. If no etag is provided in the call to GetPolicy, then the existing policy is overwritten blindly.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
}

impl client::ResponseResult for SasPortalPolicy {}


/// Request message for `SetPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set policies](PolicySetCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalSetPolicyRequest {
    /// Optional. Set the field as `true` to disable the onboarding notification.
    #[serde(rename="disableNotification")]
    
    pub disable_notification: Option<bool>,
    /// Required. The policy to be applied to the `resource`.
    
    pub policy: Option<SasPortalPolicy>,
    /// Required. The resource for which the policy is being specified. This policy replaces any existing policy.
    
    pub resource: Option<String>,
}

impl client::RequestValue for SasPortalSetPolicyRequest {}


/// Request for SignDevice.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices sign device customers](CustomerDeviceSignDeviceCall) (request)
/// * [devices sign device deployments](DeploymentDeviceSignDeviceCall) (request)
/// * [devices sign device nodes](NodeDeviceSignDeviceCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalSignDeviceRequest {
    /// Required. The device to sign. The device fields name, fcc_id and serial_number must be set. The user_id field must be set.
    
    pub device: Option<SasPortalDevice>,
}

impl client::RequestValue for SasPortalSignDeviceRequest {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalStatus {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for SasPortalStatus {}


/// Request message for `TestPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [test policies](PolicyTestCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalTestPermissionsRequest {
    /// The set of permissions to check for the `resource`.
    
    pub permissions: Option<Vec<String>>,
    /// Required. The resource for which the permissions are being requested.
    
    pub resource: Option<String>,
}

impl client::RequestValue for SasPortalTestPermissionsRequest {}


/// Response message for `TestPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [test policies](PolicyTestCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalTestPermissionsResponse {
    /// A set of permissions that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for SasPortalTestPermissionsResponse {}


/// Request for UpdateSignedDevice.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices update signed customers](CustomerDeviceUpdateSignedCall) (request)
/// * [devices update signed deployments](DeploymentDeviceUpdateSignedCall) (request)
/// * [devices update signed nodes](NodeDeviceUpdateSignedCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalUpdateSignedDeviceRequest {
    /// Required. The JSON Web Token signed using a CPI private key. Payload must be the JSON encoding of the device. The user_id field must be set.
    #[serde(rename="encodedDevice")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub encoded_device: Option<Vec<u8>>,
    /// Required. Unique installer ID (CPI ID) from the Certified Professional Installers database.
    #[serde(rename="installerId")]
    
    pub installer_id: Option<String>,
}

impl client::RequestValue for SasPortalUpdateSignedDeviceRequest {}


/// Request for ValidateInstaller.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [validate installer](InstallerValidateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalValidateInstallerRequest {
    /// Required. JSON Web Token signed using a CPI private key. Payload must include a "secret" claim whose value is the secret.
    #[serde(rename="encodedSecret")]
    
    pub encoded_secret: Option<String>,
    /// Required. Unique installer id (CPI ID) from the Certified Professional Installers database.
    #[serde(rename="installerId")]
    
    pub installer_id: Option<String>,
    /// Required. Secret returned by the GenerateSecret.
    
    pub secret: Option<String>,
}

impl client::RequestValue for SasPortalValidateInstallerRequest {}


/// Response for ValidateInstaller.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [validate installer](InstallerValidateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SasPortalValidateInstallerResponse { _never_set: Option<bool> }

impl client::ResponseResult for SasPortalValidateInstallerResponse {}


