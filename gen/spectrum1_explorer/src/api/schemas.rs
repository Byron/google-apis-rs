use super::*;
/// Antenna characteristics provide additional information, such as the antenna height, antenna type, etc. Whether antenna characteristics must be provided in a request depends on the device type and regulatory domain.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AntennaCharacteristics {
    /// The antenna height in meters. Whether the antenna height is required depends on the device type and the regulatory domain. Note that the height may be negative.
    
    pub height: Option<f64>,
    /// If the height is required, then the height type (AGL for above ground level or AMSL for above mean sea level) is also required. The default is AGL.
    #[serde(rename="heightType")]
    
    pub height_type: Option<String>,
    /// The height uncertainty in meters. Whether this is required depends on the regulatory domain.
    #[serde(rename="heightUncertainty")]
    
    pub height_uncertainty: Option<f64>,
}

impl client::Part for AntennaCharacteristics {}


/// This message contains the name and URI of a database.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseSpec {
    /// The display name for a database.
    
    pub name: Option<String>,
    /// The corresponding URI of the database.
    
    pub uri: Option<String>,
}

impl client::Part for DatabaseSpec {}


/// This message is provided by the database to notify devices of an upcoming change to the database URI.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DbUpdateSpec {
    /// A required list of one or more databases. A device should update its preconfigured list of databases to replace (only) the database that provided the response with the specified entries.
    
    pub databases: Option<Vec<DatabaseSpec>>,
}

impl client::Part for DbUpdateSpec {}


/// Device capabilities provide additional information that may be used by a device to provide additional information to the database that may help it to determine available spectrum. If the database does not support device capabilities it will ignore the parameter altogether.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceCapabilities {
    /// An optional list of frequency ranges supported by the device. Each element must contain start and stop frequencies in which the device can operate. Channel identifiers are optional. When specified, the database should not return available spectrum that falls outside these ranges or channel IDs.
    #[serde(rename="frequencyRanges")]
    
    pub frequency_ranges: Option<Vec<FrequencyRange>>,
}

impl client::Part for DeviceCapabilities {}


/// The device descriptor contains parameters that identify the specific device, such as its manufacturer serial number, regulatory-specific identifier (e.g., FCC ID), and any other device characteristics required by regulatory domains.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceDescriptor {
    /// Specifies the ETSI white space device category. Valid values are the strings master and slave. This field is case-insensitive. Consult the ETSI documentation for details about the device types.
    #[serde(rename="etsiEnDeviceCategory")]
    
    pub etsi_en_device_category: Option<String>,
    /// Specifies the ETSI white space device emissions class. The values are represented by numeric strings, such as 1, 2, etc. Consult the ETSI documentation for details about the device types.
    #[serde(rename="etsiEnDeviceEmissionsClass")]
    
    pub etsi_en_device_emissions_class: Option<String>,
    /// Specifies the ETSI white space device type. Valid values are single-letter strings, such as A, B, etc. Consult the ETSI documentation for details about the device types.
    #[serde(rename="etsiEnDeviceType")]
    
    pub etsi_en_device_type: Option<String>,
    /// Specifies the ETSI white space device technology identifier. The string value must not exceed 64 characters in length. Consult the ETSI documentation for details about the device types.
    #[serde(rename="etsiEnTechnologyId")]
    
    pub etsi_en_technology_id: Option<String>,
    /// Specifies the device's FCC certification identifier. The value is an identifier string whose length should not exceed 32 characters. Note that, in practice, a valid FCC ID may be limited to 19 characters.
    #[serde(rename="fccId")]
    
    pub fcc_id: Option<String>,
    /// Specifies the TV Band White Space device type, as defined by the FCC. Valid values are FIXED, MODE_1, MODE_2.
    #[serde(rename="fccTvbdDeviceType")]
    
    pub fcc_tvbd_device_type: Option<String>,
    /// The manufacturer's ID may be required by the regulatory domain. This should represent the name of the device manufacturer, should be consistent across all devices from the same manufacturer, and should be distinct from that of other manufacturers. The string value must not exceed 64 characters in length.
    #[serde(rename="manufacturerId")]
    
    pub manufacturer_id: Option<String>,
    /// The device's model ID may be required by the regulatory domain. The string value must not exceed 64 characters in length.
    #[serde(rename="modelId")]
    
    pub model_id: Option<String>,
    /// The list of identifiers for rulesets supported by the device. A database may require that the device provide this list before servicing the device requests. If the database does not support any of the rulesets specified in the list, the database may refuse to service the device requests. If present, the list must contain at least one entry.
    /// 
    /// For information about the valid requests, see section 9.2 of the PAWS specification. Currently, FccTvBandWhiteSpace-2010 is the only supported ruleset.
    #[serde(rename="rulesetIds")]
    
    pub ruleset_ids: Option<Vec<String>>,
    /// The manufacturer's device serial number; required by the applicable regulatory domain. The length of the value must not exceed 64 characters.
    #[serde(rename="serialNumber")]
    
    pub serial_number: Option<String>,
}

impl client::Part for DeviceDescriptor {}


/// This parameter contains device-owner information required as part of device registration. The regulatory domains may require additional parameters.
/// 
/// All contact information must be expressed using the structure defined by the vCard format specification. Only the contact fields of vCard are supported:  
/// - fn: Full name of an individual 
/// - org: Name of the organization 
/// - adr: Address fields 
/// - tel: Telephone numbers 
/// - email: Email addresses  
/// 
/// Note that the vCard specification defines maximum lengths for each field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceOwner {
    /// The vCard contact information for the device operator is optional, but may be required by specific regulatory domains.
    
    pub operator: Option<Vcard>,
    /// The vCard contact information for the individual or business that owns the device is required.
    
    pub owner: Option<Vcard>,
}

impl client::Part for DeviceOwner {}


/// The device validity element describes whether a particular device is valid to operate in the regulatory domain.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceValidity {
    /// The descriptor of the device for which the validity check was requested. It will always be present.
    #[serde(rename="deviceDesc")]
    
    pub device_desc: Option<DeviceDescriptor>,
    /// The validity status: true if the device is valid for operation, false otherwise. It will always be present.
    #[serde(rename="isValid")]
    
    pub is_valid: Option<bool>,
    /// If the device identifier is not valid, the database may include a reason. The reason may be in any language. The length of the value should not exceed 128 characters.
    
    pub reason: Option<String>,
}

impl client::Part for DeviceValidity {}


/// The start and stop times of an event. This is used to indicate the time period for which a spectrum profile is valid.
/// 
/// Both times are expressed using the format, YYYY-MM-DDThh:mm:ssZ, as defined in RFC3339. The times must be expressed using UTC.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventTime {
    /// The inclusive start of the event. It will be present.
    #[serde(rename="startTime")]
    
    pub start_time: Option<String>,
    /// The exclusive end of the event. It will be present.
    #[serde(rename="stopTime")]
    
    pub stop_time: Option<String>,
}

impl client::Part for EventTime {}


/// A specific range of frequencies together with the associated maximum power level and channel identifier.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FrequencyRange {
    /// The database may include a channel identifier, when applicable. When it is included, the device should treat it as informative. The length of the identifier should not exceed 16 characters.
    #[serde(rename="channelId")]
    
    pub channel_id: Option<String>,
    /// The maximum total power level (EIRP)—computed over the corresponding operating bandwidth—that is permitted within the frequency range. Depending on the context in which the frequency-range element appears, this value may be required. For example, it is required in the available-spectrum response, available-spectrum-batch response, and spectrum-use notification message, but it should not be present (it is not applicable) when the frequency range appears inside a device-capabilities message.
    #[serde(rename="maxPowerDBm")]
    
    pub max_power_d_bm: Option<f64>,
    /// The required inclusive start of the frequency range (in Hertz).
    #[serde(rename="startHz")]
    
    pub start_hz: Option<f64>,
    /// The required exclusive end of the frequency range (in Hertz).
    #[serde(rename="stopHz")]
    
    pub stop_hz: Option<f64>,
}

impl client::Part for FrequencyRange {}


/// This parameter is used to specify the geolocation of the device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeoLocation {
    /// The location confidence level, as an integer percentage, may be required, depending on the regulatory domain. When the parameter is optional and not provided, its value is assumed to be 95. Valid values range from 0 to 99, since, in practice, 100-percent confidence is not achievable. The confidence value is meaningful only when geolocation refers to a point with uncertainty.
    
    pub confidence: Option<i32>,
    /// If present, indicates that the geolocation represents a point. Paradoxically, a point is parameterized using an ellipse, where the center represents the location of the point and the distances along the major and minor axes represent the uncertainty. The uncertainty values may be required, depending on the regulatory domain.
    
    pub point: Option<GeoLocationEllipse>,
    /// If present, indicates that the geolocation represents a region. Database support for regions is optional.
    
    pub region: Option<GeoLocationPolygon>,
}

impl client::Part for GeoLocation {}


/// A "point" with uncertainty is represented using the Ellipse shape.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeoLocationEllipse {
    /// A required geo-spatial point representing the center of the ellipse.
    
    pub center: Option<GeoLocationPoint>,
    /// A floating-point number that expresses the orientation of the ellipse, representing the rotation, in degrees, of the semi-major axis from North towards the East. For example, when the uncertainty is greatest along the North-South direction, orientation is 0 degrees; conversely, if the uncertainty is greatest along the East-West direction, orientation is 90 degrees. When orientation is not present, the orientation is assumed to be 0.
    
    pub orientation: Option<f64>,
    /// A floating-point number that expresses the location uncertainty along the major axis of the ellipse. May be required by the regulatory domain. When the uncertainty is optional, the default value is 0.
    #[serde(rename="semiMajorAxis")]
    
    pub semi_major_axis: Option<f64>,
    /// A floating-point number that expresses the location uncertainty along the minor axis of the ellipse. May be required by the regulatory domain. When the uncertainty is optional, the default value is 0.
    #[serde(rename="semiMinorAxis")]
    
    pub semi_minor_axis: Option<f64>,
}

impl client::Part for GeoLocationEllipse {}


/// A single geolocation on the globe.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeoLocationPoint {
    /// A required floating-point number that expresses the latitude in degrees using the WGS84 datum. For details on this encoding, see the National Imagery and Mapping Agency's Technical Report TR8350.2.
    
    pub latitude: Option<f64>,
    /// A required floating-point number that expresses the longitude in degrees using the WGS84 datum. For details on this encoding, see the National Imagery and Mapping Agency's Technical Report TR8350.2.
    
    pub longitude: Option<f64>,
}

impl client::Part for GeoLocationPoint {}


/// A region is represented using the polygonal shape.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeoLocationPolygon {
    /// When the geolocation describes a region, the exterior field refers to a list of latitude/longitude points that represent the vertices of a polygon. The first and last points must be the same. Thus, a minimum of four points is required. The following polygon restrictions from RFC5491 apply:  
    /// - A connecting line shall not cross another connecting line of the same polygon. 
    /// - The vertices must be defined in a counterclockwise order. 
    /// - The edges of a polygon are defined by the shortest path between two points in space (not a geodesic curve). Consequently, the length between two adjacent vertices should be restricted to a maximum of 130 km. 
    /// - All vertices are assumed to be at the same altitude. 
    /// - Polygon shapes should be restricted to a maximum of 15 vertices (16 points that include the repeated vertex).
    
    pub exterior: Option<Vec<GeoLocationPoint>>,
}

impl client::Part for GeoLocationPolygon {}


/// The schedule of spectrum profiles available at a particular geolocation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeoSpectrumSchedule {
    /// The geolocation identifies the location at which the spectrum schedule applies. It will always be present.
    
    pub location: Option<GeoLocation>,
    /// A list of available spectrum profiles and associated times. It will always be present, and at least one schedule must be included (though it may be empty if there is no available spectrum). More than one schedule may be included to represent future changes to the available spectrum.
    #[serde(rename="spectrumSchedules")]
    
    pub spectrum_schedules: Option<Vec<SpectrumSchedule>>,
}

impl client::Part for GeoSpectrumSchedule {}


/// The request message for a batch available spectrum query protocol.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get spectrum batch paws](PawGetSpectrumBatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PawsGetSpectrumBatchRequest {
    /// Depending on device type and regulatory domain, antenna characteristics may be required.
    
    pub antenna: Option<AntennaCharacteristics>,
    /// The master device may include its device capabilities to limit the available-spectrum batch response to the spectrum that is compatible with its capabilities. The database should not return spectrum that is incompatible with the specified capabilities.
    
    pub capabilities: Option<DeviceCapabilities>,
    /// When the available spectrum request is made on behalf of a specific device (a master or slave device), device descriptor information for the device on whose behalf the request is made is required (in such cases, the requestType parameter must be empty). When a requestType value is specified, device descriptor information may be optional or required according to the rules of the applicable regulatory domain.
    #[serde(rename="deviceDesc")]
    
    pub device_desc: Option<DeviceDescriptor>,
    /// A geolocation list is required. This allows a device to specify its current location plus additional anticipated locations when allowed by the regulatory domain. At least one location must be included. Geolocation must be given as the location of the radiation center of the device's antenna. If a location specifies a region, rather than a point, the database may return an UNIMPLEMENTED error if it does not support query by region.
    /// 
    /// There is no upper limit on the number of locations included in a available spectrum batch request, but the database may restrict the number of locations it supports by returning a response with fewer locations than specified in the batch request. Note that geolocations must be those of the master device (a device with geolocation capability that makes an available spectrum batch request), whether the master device is making the request on its own behalf or on behalf of a slave device (one without geolocation capability).
    
    pub locations: Option<Vec<GeoLocation>>,
    /// When an available spectrum batch request is made by the master device (a device with geolocation capability) on behalf of a slave device (a device without geolocation capability), the rules of the applicable regulatory domain may require the master device to provide its own device descriptor information (in addition to device descriptor information for the slave device in a separate parameter).
    #[serde(rename="masterDeviceDesc")]
    
    pub master_device_desc: Option<DeviceDescriptor>,
    /// Depending on device type and regulatory domain, device owner information may be included in an available spectrum batch request. This allows the device to register and get spectrum-availability information in a single request.
    
    pub owner: Option<DeviceOwner>,
    /// The request type parameter is an optional parameter that can be used to modify an available spectrum batch request, but its use depends on applicable regulatory rules. For example, It may be used to request generic slave device parameters without having to specify the device descriptor for a specific device. When the requestType parameter is missing, the request is for a specific device (master or slave), and the device descriptor parameter for the device on whose behalf the batch request is made is required.
    #[serde(rename="requestType")]
    
    pub request_type: Option<String>,
    /// The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).
    /// 
    /// Required field.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The PAWS version. Must be exactly 1.0.
    /// 
    /// Required field.
    
    pub version: Option<String>,
}

impl client::RequestValue for PawsGetSpectrumBatchRequest {}


/// The response message for the batch available spectrum query contains a schedule of available spectrum for the device at multiple locations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get spectrum batch paws](PawGetSpectrumBatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PawsGetSpectrumBatchResponse {
    /// A database may include the databaseChange parameter to notify a device of a change to its database URI, providing one or more alternate database URIs. The device should use this information to update its list of pre-configured databases by (only) replacing its entry for the responding database with the list of alternate URIs.
    #[serde(rename="databaseChange")]
    
    pub database_change: Option<DbUpdateSpec>,
    /// The database must return in its available spectrum response the device descriptor information it received in the master device's available spectrum batch request.
    #[serde(rename="deviceDesc")]
    
    pub device_desc: Option<DeviceDescriptor>,
    /// The available spectrum batch response must contain a geo-spectrum schedule list, The list may be empty if spectrum is not available. The database may return more than one geo-spectrum schedule to represent future changes to the available spectrum. How far in advance a schedule may be provided depends upon the applicable regulatory domain. The database may return available spectrum for fewer geolocations than requested. The device must not make assumptions about the order of the entries in the list, and must use the geolocation value in each geo-spectrum schedule entry to match available spectrum to a location.
    #[serde(rename="geoSpectrumSchedules")]
    
    pub geo_spectrum_schedules: Option<Vec<GeoSpectrumSchedule>>,
    /// Identifies what kind of resource this is. Value: the fixed string "spectrum#pawsGetSpectrumBatchResponse".
    
    pub kind: Option<String>,
    /// The database may return a constraint on the allowed maximum contiguous bandwidth (in Hertz). A regulatory domain may require the database to return this parameter. When this parameter is present in the response, the device must apply this constraint to its spectrum-selection logic to ensure that no single block of spectrum has bandwidth that exceeds this value.
    #[serde(rename="maxContiguousBwHz")]
    
    pub max_contiguous_bw_hz: Option<f64>,
    /// The database may return a constraint on the allowed maximum total bandwidth (in Hertz), which does not need to be contiguous. A regulatory domain may require the database to return this parameter. When this parameter is present in the available spectrum batch response, the device must apply this constraint to its spectrum-selection logic to ensure that total bandwidth does not exceed this value.
    #[serde(rename="maxTotalBwHz")]
    
    pub max_total_bw_hz: Option<f64>,
    /// For regulatory domains that require a spectrum-usage report from devices, the database must return true for this parameter if the geo-spectrum schedules list is not empty; otherwise, the database should either return false or omit this parameter. If this parameter is present and its value is true, the device must send a spectrum use notify message to the database; otherwise, the device should not send the notification.
    #[serde(rename="needsSpectrumReport")]
    
    pub needs_spectrum_report: Option<bool>,
    /// The database should return ruleset information, which identifies the applicable regulatory authority and ruleset for the available spectrum batch response. If included, the device must use the corresponding ruleset to interpret the response. Values provided in the returned ruleset information, such as maxLocationChange, take precedence over any conflicting values provided in the ruleset information returned in a prior initialization response sent by the database to the device.
    #[serde(rename="rulesetInfo")]
    
    pub ruleset_info: Option<RulesetInfo>,
    /// The database includes a timestamp of the form, YYYY-MM-DDThh:mm:ssZ (Internet timestamp format per RFC3339), in its available spectrum batch response. The timestamp should be used by the device as a reference for the start and stop times specified in the response spectrum schedules.
    
    pub timestamp: Option<String>,
    /// The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).
    /// 
    /// Required field.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The PAWS version. Must be exactly 1.0.
    /// 
    /// Required field.
    
    pub version: Option<String>,
}

impl client::ResponseResult for PawsGetSpectrumBatchResponse {}


/// The request message for the available spectrum query protocol which must include the device’s geolocation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get spectrum paws](PawGetSpectrumCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PawsGetSpectrumRequest {
    /// Depending on device type and regulatory domain, the characteristics of the antenna may be required.
    
    pub antenna: Option<AntennaCharacteristics>,
    /// The master device may include its device capabilities to limit the available-spectrum response to the spectrum that is compatible with its capabilities. The database should not return spectrum that is incompatible with the specified capabilities.
    
    pub capabilities: Option<DeviceCapabilities>,
    /// When the available spectrum request is made on behalf of a specific device (a master or slave device), device descriptor information for that device is required (in such cases, the requestType parameter must be empty). When a requestType value is specified, device descriptor information may be optional or required according to the rules of the applicable regulatory domain.
    #[serde(rename="deviceDesc")]
    
    pub device_desc: Option<DeviceDescriptor>,
    /// The geolocation of the master device (a device with geolocation capability that makes an available spectrum request) is required whether the master device is making the request on its own behalf or on behalf of a slave device (one without geolocation capability). The location must be the location of the radiation center of the master device's antenna. To support mobile devices, a regulatory domain may allow the anticipated position of the master device to be given instead. If the location specifies a region, rather than a point, the database may return an UNIMPLEMENTED error code if it does not support query by region.
    
    pub location: Option<GeoLocation>,
    /// When an available spectrum request is made by the master device (a device with geolocation capability) on behalf of a slave device (a device without geolocation capability), the rules of the applicable regulatory domain may require the master device to provide its own device descriptor information (in addition to device descriptor information for the slave device, which is provided in a separate parameter).
    #[serde(rename="masterDeviceDesc")]
    
    pub master_device_desc: Option<DeviceDescriptor>,
    /// Depending on device type and regulatory domain, device owner information may be included in an available spectrum request. This allows the device to register and get spectrum-availability information in a single request.
    
    pub owner: Option<DeviceOwner>,
    /// The request type parameter is an optional parameter that can be used to modify an available spectrum request, but its use depends on applicable regulatory rules. It may be used, for example, to request generic slave device parameters without having to specify the device descriptor for a specific device. When the requestType parameter is missing, the request is for a specific device (master or slave), and the deviceDesc parameter for the device on whose behalf the request is made is required.
    #[serde(rename="requestType")]
    
    pub request_type: Option<String>,
    /// The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).
    /// 
    /// Required field.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The PAWS version. Must be exactly 1.0.
    /// 
    /// Required field.
    
    pub version: Option<String>,
}

impl client::RequestValue for PawsGetSpectrumRequest {}


/// The response message for the available spectrum query which contains a schedule of available spectrum for the device.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get spectrum paws](PawGetSpectrumCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PawsGetSpectrumResponse {
    /// A database may include the databaseChange parameter to notify a device of a change to its database URI, providing one or more alternate database URIs. The device should use this information to update its list of pre-configured databases by (only) replacing its entry for the responding database with the list of alternate URIs.
    #[serde(rename="databaseChange")]
    
    pub database_change: Option<DbUpdateSpec>,
    /// The database must return, in its available spectrum response, the device descriptor information it received in the master device's available spectrum request.
    #[serde(rename="deviceDesc")]
    
    pub device_desc: Option<DeviceDescriptor>,
    /// Identifies what kind of resource this is. Value: the fixed string "spectrum#pawsGetSpectrumResponse".
    
    pub kind: Option<String>,
    /// The database may return a constraint on the allowed maximum contiguous bandwidth (in Hertz). A regulatory domain may require the database to return this parameter. When this parameter is present in the response, the device must apply this constraint to its spectrum-selection logic to ensure that no single block of spectrum has bandwidth that exceeds this value.
    #[serde(rename="maxContiguousBwHz")]
    
    pub max_contiguous_bw_hz: Option<f64>,
    /// The database may return a constraint on the allowed maximum total bandwidth (in Hertz), which need not be contiguous. A regulatory domain may require the database to return this parameter. When this parameter is present in the available spectrum response, the device must apply this constraint to its spectrum-selection logic to ensure that total bandwidth does not exceed this value.
    #[serde(rename="maxTotalBwHz")]
    
    pub max_total_bw_hz: Option<f64>,
    /// For regulatory domains that require a spectrum-usage report from devices, the database must return true for this parameter if the spectrum schedule list is not empty; otherwise, the database will either return false or omit this parameter. If this parameter is present and its value is true, the device must send a spectrum use notify message to the database; otherwise, the device must not send the notification.
    #[serde(rename="needsSpectrumReport")]
    
    pub needs_spectrum_report: Option<bool>,
    /// The database should return ruleset information, which identifies the applicable regulatory authority and ruleset for the available spectrum response. If included, the device must use the corresponding ruleset to interpret the response. Values provided in the returned ruleset information, such as maxLocationChange, take precedence over any conflicting values provided in the ruleset information returned in a prior initialization response sent by the database to the device.
    #[serde(rename="rulesetInfo")]
    
    pub ruleset_info: Option<RulesetInfo>,
    /// The available spectrum response must contain a spectrum schedule list. The list may be empty if spectrum is not available. The database may return more than one spectrum schedule to represent future changes to the available spectrum. How far in advance a schedule may be provided depends on the applicable regulatory domain.
    #[serde(rename="spectrumSchedules")]
    
    pub spectrum_schedules: Option<Vec<SpectrumSchedule>>,
    /// The database includes a timestamp of the form YYYY-MM-DDThh:mm:ssZ (Internet timestamp format per RFC3339) in its available spectrum response. The timestamp should be used by the device as a reference for the start and stop times specified in the response spectrum schedules.
    
    pub timestamp: Option<String>,
    /// The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).
    /// 
    /// Required field.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The PAWS version. Must be exactly 1.0.
    /// 
    /// Required field.
    
    pub version: Option<String>,
}

impl client::ResponseResult for PawsGetSpectrumResponse {}


/// The initialization request message allows the master device to initiate exchange of capabilities with the database.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [init paws](PawInitCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PawsInitRequest {
    /// The DeviceDescriptor parameter is required. If the database does not support the device or any of the rulesets specified in the device descriptor, it must return an UNSUPPORTED error code in the error response.
    #[serde(rename="deviceDesc")]
    
    pub device_desc: Option<DeviceDescriptor>,
    /// A device's geolocation is required.
    
    pub location: Option<GeoLocation>,
    /// The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).
    /// 
    /// Required field.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The PAWS version. Must be exactly 1.0.
    /// 
    /// Required field.
    
    pub version: Option<String>,
}

impl client::RequestValue for PawsInitRequest {}


/// The initialization response message communicates database parameters to the requesting device.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [init paws](PawInitCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PawsInitResponse {
    /// A database may include the databaseChange parameter to notify a device of a change to its database URI, providing one or more alternate database URIs. The device should use this information to update its list of pre-configured databases by (only) replacing its entry for the responding database with the list of alternate URIs.
    #[serde(rename="databaseChange")]
    
    pub database_change: Option<DbUpdateSpec>,
    /// Identifies what kind of resource this is. Value: the fixed string "spectrum#pawsInitResponse".
    
    pub kind: Option<String>,
    /// The rulesetInfo parameter must be included in the response. This parameter specifies the regulatory domain and parameters applicable to that domain. The database must include the authority field, which defines the regulatory domain for the location specified in the INIT_REQ message.
    #[serde(rename="rulesetInfo")]
    
    pub ruleset_info: Option<RulesetInfo>,
    /// The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).
    /// 
    /// Required field.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The PAWS version. Must be exactly 1.0.
    /// 
    /// Required field.
    
    pub version: Option<String>,
}

impl client::ResponseResult for PawsInitResponse {}


/// The spectrum-use notification message which must contain the geolocation of the Device and parameters required by the regulatory domain.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notify spectrum use paws](PawNotifySpectrumUseCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PawsNotifySpectrumUseRequest {
    /// Device descriptor information is required in the spectrum-use notification message.
    #[serde(rename="deviceDesc")]
    
    pub device_desc: Option<DeviceDescriptor>,
    /// The geolocation of the master device (the device that is sending the spectrum-use notification) to the database is required in the spectrum-use notification message.
    
    pub location: Option<GeoLocation>,
    /// A spectrum list is required in the spectrum-use notification. The list specifies the spectrum that the device expects to use, which includes frequency ranges and maximum power levels. The list may be empty if the device decides not to use any of spectrum. For consistency, the psdBandwidthHz value should match that from one of the spectrum elements in the corresponding available spectrum response previously sent to the device by the database. Note that maximum power levels in the spectrum element must be expressed as power spectral density over the specified psdBandwidthHz value. The actual bandwidth to be used (as computed from the start and stop frequencies) may be different from the psdBandwidthHz value. As an example, when regulatory rules express maximum power spectral density in terms of maximum power over any 100 kHz band, then the psdBandwidthHz value should be set to 100 kHz, even though the actual bandwidth used can be 20 kHz.
    
    pub spectra: Option<Vec<SpectrumMessage>>,
    /// The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).
    /// 
    /// Required field.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The PAWS version. Must be exactly 1.0.
    /// 
    /// Required field.
    
    pub version: Option<String>,
}

impl client::RequestValue for PawsNotifySpectrumUseRequest {}


/// An empty response to the notification.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notify spectrum use paws](PawNotifySpectrumUseCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PawsNotifySpectrumUseResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "spectrum#pawsNotifySpectrumUseResponse".
    
    pub kind: Option<String>,
    /// The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).
    /// 
    /// Required field.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The PAWS version. Must be exactly 1.0.
    /// 
    /// Required field.
    
    pub version: Option<String>,
}

impl client::ResponseResult for PawsNotifySpectrumUseResponse {}


/// The registration request message contains the required registration parameters.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [register paws](PawRegisterCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PawsRegisterRequest {
    /// Antenna characteristics, including its height and height type.
    
    pub antenna: Option<AntennaCharacteristics>,
    /// A DeviceDescriptor is required.
    #[serde(rename="deviceDesc")]
    
    pub device_desc: Option<DeviceDescriptor>,
    /// Device owner information is required.
    #[serde(rename="deviceOwner")]
    
    pub device_owner: Option<DeviceOwner>,
    /// A device's geolocation is required.
    
    pub location: Option<GeoLocation>,
    /// The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).
    /// 
    /// Required field.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The PAWS version. Must be exactly 1.0.
    /// 
    /// Required field.
    
    pub version: Option<String>,
}

impl client::RequestValue for PawsRegisterRequest {}


/// The registration response message simply acknowledges receipt of the request and is otherwise empty.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [register paws](PawRegisterCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PawsRegisterResponse {
    /// A database may include the databaseChange parameter to notify a device of a change to its database URI, providing one or more alternate database URIs. The device should use this information to update its list of pre-configured databases by (only) replacing its entry for the responding database with the list of alternate URIs.
    #[serde(rename="databaseChange")]
    
    pub database_change: Option<DbUpdateSpec>,
    /// Identifies what kind of resource this is. Value: the fixed string "spectrum#pawsRegisterResponse".
    
    pub kind: Option<String>,
    /// The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).
    /// 
    /// Required field.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The PAWS version. Must be exactly 1.0.
    /// 
    /// Required field.
    
    pub version: Option<String>,
}

impl client::ResponseResult for PawsRegisterResponse {}


/// The device validation request message.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [verify device paws](PawVerifyDeviceCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PawsVerifyDeviceRequest {
    /// A list of device descriptors, which specifies the slave devices to be validated, is required.
    #[serde(rename="deviceDescs")]
    
    pub device_descs: Option<Vec<DeviceDescriptor>>,
    /// The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).
    /// 
    /// Required field.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The PAWS version. Must be exactly 1.0.
    /// 
    /// Required field.
    
    pub version: Option<String>,
}

impl client::RequestValue for PawsVerifyDeviceRequest {}


/// The device validation response message.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [verify device paws](PawVerifyDeviceCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PawsVerifyDeviceResponse {
    /// A database may include the databaseChange parameter to notify a device of a change to its database URI, providing one or more alternate database URIs. The device should use this information to update its list of pre-configured databases by (only) replacing its entry for the responding database with the list of alternate URIs.
    #[serde(rename="databaseChange")]
    
    pub database_change: Option<DbUpdateSpec>,
    /// A device validities list is required in the device validation response to report whether each slave device listed in a previous device validation request is valid. The number of entries must match the number of device descriptors listed in the previous device validation request.
    #[serde(rename="deviceValidities")]
    
    pub device_validities: Option<Vec<DeviceValidity>>,
    /// Identifies what kind of resource this is. Value: the fixed string "spectrum#pawsVerifyDeviceResponse".
    
    pub kind: Option<String>,
    /// The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).
    /// 
    /// Required field.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The PAWS version. Must be exactly 1.0.
    /// 
    /// Required field.
    
    pub version: Option<String>,
}

impl client::ResponseResult for PawsVerifyDeviceResponse {}


/// This contains parameters for the ruleset of a regulatory domain that is communicated using the initialization and available-spectrum processes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RulesetInfo {
    /// The regulatory domain to which the ruleset belongs is required. It must be a 2-letter country code. The device should use this to determine additional device behavior required by the associated regulatory domain.
    
    pub authority: Option<String>,
    /// The maximum location change in meters is required in the initialization response, but optional otherwise. When the device changes location by more than this specified distance, it must contact the database to get the available spectrum for the new location. If the device is using spectrum that is no longer available, it must immediately cease use of the spectrum under rules for database-managed spectrum. If this value is provided within the context of an available-spectrum response, it takes precedence over the value within the initialization response.
    #[serde(rename="maxLocationChange")]
    
    pub max_location_change: Option<f64>,
    /// The maximum duration, in seconds, between requests for available spectrum. It is required in the initialization response, but optional otherwise. The device must contact the database to get available spectrum no less frequently than this duration. If the new spectrum information indicates that the device is using spectrum that is no longer available, it must immediately cease use of those frequencies under rules for database-managed spectrum. If this value is provided within the context of an available-spectrum response, it takes precedence over the value within the initialization response.
    #[serde(rename="maxPollingSecs")]
    
    pub max_polling_secs: Option<i32>,
    /// The identifiers of the rulesets supported for the device's location. The database should include at least one applicable ruleset in the initialization response. The device may use the ruleset identifiers to determine parameters to include in subsequent requests. Within the context of the available-spectrum responses, the database should include the identifier of the ruleset that it used to determine the available-spectrum response. If included, the device must use the specified ruleset to interpret the response. If the device does not support the indicated ruleset, it must not operate in the spectrum governed by the ruleset.
    #[serde(rename="rulesetIds")]
    
    pub ruleset_ids: Option<Vec<String>>,
}

impl client::Part for RulesetInfo {}


/// Available spectrum can be logically characterized by a list of frequency ranges and permissible power levels for each range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpectrumMessage {
    /// The bandwidth (in Hertz) for which permissible power levels are specified. For example, FCC regulation would require only one spectrum specification at 6MHz bandwidth, but Ofcom regulation would require two specifications, at 0.1MHz and 8MHz. This parameter may be empty if there is no available spectrum. It will be present otherwise.
    
    pub bandwidth: Option<f64>,
    /// The list of frequency ranges and permissible power levels. The list may be empty if there is no available spectrum, otherwise it will be present.
    #[serde(rename="frequencyRanges")]
    
    pub frequency_ranges: Option<Vec<FrequencyRange>>,
}

impl client::Part for SpectrumMessage {}


/// The spectrum schedule element combines an event time with spectrum profile to define a time period in which the profile is valid.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpectrumSchedule {
    /// The event time expresses when the spectrum profile is valid. It will always be present.
    #[serde(rename="eventTime")]
    
    pub event_time: Option<EventTime>,
    /// A list of spectrum messages representing the usable profile. It will always be present, but may be empty when there is no available spectrum.
    
    pub spectra: Option<Vec<SpectrumMessage>>,
}

impl client::Part for SpectrumSchedule {}


/// A vCard-in-JSON message that contains only the fields needed for PAWS:  
/// - fn: Full name of an individual 
/// - org: Name of the organization 
/// - adr: Address fields 
/// - tel: Telephone numbers 
/// - email: Email addresses
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Vcard {
    /// The street address of the entity.
    
    pub adr: Option<VcardAddress>,
    /// An email address that can be used to reach the contact.
    
    pub email: Option<VcardTypedText>,
    /// The full name of the contact person. For example: John A. Smith.
    #[serde(rename="fn")]
    
    pub fn_: Option<String>,
    /// The organization associated with the registering entity.
    
    pub org: Option<VcardTypedText>,
    /// A telephone number that can be used to call the contact.
    
    pub tel: Option<VcardTelephone>,
}

impl client::Part for Vcard {}


/// The structure used to represent a street address.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VcardAddress {
    /// The postal code associated with the address. For example: 94423.
    
    pub code: Option<String>,
    /// The country name. For example: US.
    
    pub country: Option<String>,
    /// The city or local equivalent portion of the address. For example: San Jose.
    
    pub locality: Option<String>,
    /// An optional post office box number.
    
    pub pobox: Option<String>,
    /// The state or local equivalent portion of the address. For example: CA.
    
    pub region: Option<String>,
    /// The street number and name. For example: 123 Any St.
    
    pub street: Option<String>,
}

impl client::Part for VcardAddress {}


/// The structure used to represent a telephone number.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VcardTelephone {
    /// A nested telephone URI of the form: tel:+1-123-456-7890.
    
    pub uri: Option<String>,
}

impl client::Part for VcardTelephone {}


/// The structure used to represent an organization and an email address.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VcardTypedText {
    /// The text string associated with this item. For example, for an org field: ACME, inc. For an email field: smith@example.com.
    
    pub text: Option<String>,
}

impl client::Part for VcardTypedText {}


