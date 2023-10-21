use super::*;
/// Defines a unique identifier of a beacon as broadcast by the device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdvertisedId {
    /// The actual beacon identifier, as broadcast by the beacon hardware. Must be
    /// [base64](http://tools.ietf.org/html/rfc4648#section-4) encoded in HTTP
    /// requests, and will be so encoded (with padding) in responses. The base64
    /// encoding should be of the binary byte-stream and not any textual (such as
    /// hex) representation thereof.
    /// Required.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub id: Option<Vec<u8>>,
    /// Specifies the identifier type.
    /// Required.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for AdvertisedId {}


/// A subset of attachment information served via the
/// `beaconinfo.getforobserved` method, used when your users encounter your
/// beacons.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AttachmentInfo {
    /// An opaque data container for client-provided data.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// The distance away from the beacon at which this attachment should be
    /// delivered to a mobile app.
    /// 
    /// Setting this to a value greater than zero indicates that the app should
    /// behave as if the beacon is "seen" when the mobile device is less than this
    /// distance away from the beacon.
    /// 
    /// Different attachments on the same beacon can have different max distances.
    /// 
    /// Note that even though this value is expressed with fractional meter
    /// precision, real-world behavior is likley to be much less precise than one
    /// meter, due to the nature of current Bluetooth radio technology.
    /// 
    /// Optional. When not set or zero, the attachment should be delivered at the
    /// beacon's outer limit of detection.
    #[serde(rename="maxDistanceMeters")]
    
    pub max_distance_meters: Option<f64>,
    /// Specifies what kind of attachment this is. Tells a client how to
    /// interpret the `data` field. Format is <var>namespace/type</var>, for
    /// example <code>scrupulous-wombat-12345/welcome-message</code>
    #[serde(rename="namespacedType")]
    
    pub namespaced_type: Option<String>,
}

impl client::Part for AttachmentInfo {}


/// Details of a beacon device.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [attachments batch delete beacons](BeaconAttachmentBatchDeleteCall) (none)
/// * [attachments create beacons](BeaconAttachmentCreateCall) (none)
/// * [attachments delete beacons](BeaconAttachmentDeleteCall) (none)
/// * [attachments list beacons](BeaconAttachmentListCall) (none)
/// * [diagnostics list beacons](BeaconDiagnosticListCall) (none)
/// * [activate beacons](BeaconActivateCall) (none)
/// * [deactivate beacons](BeaconDeactivateCall) (none)
/// * [decommission beacons](BeaconDecommissionCall) (none)
/// * [delete beacons](BeaconDeleteCall) (none)
/// * [get beacons](BeaconGetCall) (response)
/// * [list beacons](BeaconListCall) (none)
/// * [register beacons](BeaconRegisterCall) (request|response)
/// * [update beacons](BeaconUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Beacon {
    /// The identifier of a beacon as advertised by it. This field must be
    /// populated when registering. It may be empty when updating a beacon
    /// record because it is ignored in updates.
    /// 
    /// When registering a beacon that broadcasts Eddystone-EID, this field
    /// should contain a "stable" Eddystone-UID that identifies the beacon and
    /// links it to its attachments. The stable Eddystone-UID is only used for
    /// administering the beacon.
    #[serde(rename="advertisedId")]
    
    pub advertised_id: Option<AdvertisedId>,
    /// Resource name of this beacon. A beacon name has the format
    /// "beacons/N!beaconId" where the beaconId is the base16 ID broadcast by
    /// the beacon and N is a code for the beacon's type. Possible values are
    /// `3` for Eddystone, `1` for iBeacon, or `5` for AltBeacon.
    /// 
    /// This field must be left empty when registering. After reading a beacon,
    /// clients can use the name for future operations.
    #[serde(rename="beaconName")]
    
    pub beacon_name: Option<String>,
    /// Free text used to identify and describe the beacon. Maximum length 140
    /// characters.
    /// Optional.
    
    pub description: Option<String>,
    /// Write-only registration parameters for beacons using Eddystone-EID
    /// (remotely resolved ephemeral ID) format. This information will not be
    /// populated in API responses. When submitting this data, the `advertised_id`
    /// field must contain an ID of type Eddystone-UID. Any other ID type will
    /// result in an error.
    #[serde(rename="ephemeralIdRegistration")]
    
    pub ephemeral_id_registration: Option<EphemeralIdRegistration>,
    /// Expected location stability. This is set when the beacon is registered or
    /// updated, not automatically detected in any way.
    /// Optional.
    #[serde(rename="expectedStability")]
    
    pub expected_stability: Option<String>,
    /// The indoor level information for this beacon, if known. As returned by the
    /// Google Maps API.
    /// Optional.
    #[serde(rename="indoorLevel")]
    
    pub indoor_level: Option<IndoorLevel>,
    /// The location of the beacon, expressed as a latitude and longitude pair.
    /// This location is given when the beacon is registered or updated. It does
    /// not necessarily indicate the actual current location of the beacon.
    /// Optional.
    #[serde(rename="latLng")]
    
    pub lat_lng: Option<LatLng>,
    /// The [Google Places API](https://developers.google.com/places/place-id) Place ID of the place where
    /// the beacon is deployed. This is given when the beacon is registered or
    /// updated, not automatically detected in any way.
    /// Optional.
    #[serde(rename="placeId")]
    
    pub place_id: Option<String>,
    /// Properties of the beacon device, for example battery type or firmware
    /// version.
    /// Optional.
    
    pub properties: Option<HashMap<String, String>>,
    /// Some beacons may require a user to provide an authorization key before
    /// changing any of its configuration (e.g. broadcast frames, transmit power).
    /// This field provides a place to store and control access to that key.
    /// This field is populated in responses to `GET /v1beta1/beacons/3!beaconId`
    /// from users with write access to the given beacon. That is to say: If the
    /// user is authorized to write the beacon's confidential data in the service,
    /// the service considers them authorized to configure the beacon. Note
    /// that this key grants nothing on the service, only on the beacon itself.
    #[serde(rename="provisioningKey")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub provisioning_key: Option<Vec<u8>>,
    /// Current status of the beacon.
    /// Required.
    
    pub status: Option<String>,
}

impl client::RequestValue for Beacon {}
impl client::Resource for Beacon {}
impl client::ResponseResult for Beacon {}


/// Project-specific data associated with a beacon.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [attachments create beacons](BeaconAttachmentCreateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BeaconAttachment {
    /// Resource name of this attachment. Attachment names have the format:
    /// <code>beacons/<var>beacon_id</var>/attachments/<var>attachment_id</var></code>.
    /// Leave this empty on creation.
    #[serde(rename="attachmentName")]
    
    pub attachment_name: Option<String>,
    /// The UTC time when this attachment was created, in milliseconds since the
    /// UNIX epoch.
    #[serde(rename="creationTimeMs")]
    
    pub creation_time_ms: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// An opaque data container for client-provided data. Must be
    /// [base64](http://tools.ietf.org/html/rfc4648#section-4) encoded in HTTP
    /// requests, and will be so encoded (with padding) in responses.
    /// Required.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// The distance away from the beacon at which this attachment should be
    /// delivered to a mobile app.
    /// 
    /// Setting this to a value greater than zero indicates that the app should
    /// behave as if the beacon is "seen" when the mobile device is less than this
    /// distance away from the beacon.
    /// 
    /// Different attachments on the same beacon can have different max distances.
    /// 
    /// Note that even though this value is expressed with fractional meter
    /// precision, real-world behavior is likley to be much less precise than one
    /// meter, due to the nature of current Bluetooth radio technology.
    /// 
    /// Optional. When not set or zero, the attachment should be delivered at the
    /// beacon's outer limit of detection.
    /// 
    /// Negative values are invalid and return an error.
    #[serde(rename="maxDistanceMeters")]
    
    pub max_distance_meters: Option<f64>,
    /// Specifies what kind of attachment this is. Tells a client how to
    /// interpret the `data` field. Format is <var>namespace/type</var>. Namespace
    /// provides type separation between clients. Type describes the type of
    /// `data`, for use by the client when parsing the `data` field.
    /// Required.
    #[serde(rename="namespacedType")]
    
    pub namespaced_type: Option<String>,
}

impl client::RequestValue for BeaconAttachment {}
impl client::ResponseResult for BeaconAttachment {}


/// A subset of beacon information served via the `beaconinfo.getforobserved`
/// method, which you call when users of your app encounter your beacons.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BeaconInfo {
    /// The ID advertised by the beacon.
    #[serde(rename="advertisedId")]
    
    pub advertised_id: Option<AdvertisedId>,
    /// Attachments matching the type(s) requested.
    /// May be empty if no attachment types were requested.
    
    pub attachments: Option<Vec<AttachmentInfo>>,
    /// The name under which the beacon is registered.
    #[serde(rename="beaconName")]
    
    pub beacon_name: Option<String>,
}

impl client::Part for BeaconInfo {}


/// Represents a whole or partial calendar date, e.g. a birthday. The time of day
/// and time zone are either specified elsewhere or are not significant. The date
/// is relative to the Proleptic Gregorian Calendar. This can represent:
/// 
/// * A full date, with non-zero year, month and day values
/// * A month and day value, with a zero year, e.g. an anniversary
/// * A year on its own, with zero month and day values
/// * A year and month value, with a zero day, e.g. a credit card expiration date
/// 
/// Related types are google.type.TimeOfDay and `google.protobuf.Timestamp`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Date {
    /// Day of month. Must be from 1 to 31 and valid for the year and month, or 0
    /// if specifying a year by itself or a year and month where the day is not
    /// significant.
    
    pub day: Option<i32>,
    /// Month of year. Must be from 1 to 12, or 0 if specifying a year without a
    /// month and day.
    
    pub month: Option<i32>,
    /// Year of date. Must be from 1 to 9999, or 0 if specifying a date without
    /// a year.
    
    pub year: Option<i32>,
}

impl client::Part for Date {}


/// Response for a request to delete attachments.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [attachments batch delete beacons](BeaconAttachmentBatchDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteAttachmentsResponse {
    /// The number of attachments that were deleted.
    #[serde(rename="numDeleted")]
    
    pub num_deleted: Option<i32>,
}

impl client::ResponseResult for DeleteAttachmentsResponse {}


/// Diagnostics for a single beacon.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Diagnostics {
    /// An unordered list of Alerts that the beacon has.
    
    pub alerts: Option<Vec<String>>,
    /// Resource name of the beacon. For Eddystone-EID beacons, this may
    /// be the beacon's current EID, or the beacon's "stable" Eddystone-UID.
    #[serde(rename="beaconName")]
    
    pub beacon_name: Option<String>,
    /// The date when the battery is expected to be low. If the value is missing
    /// then there is no estimate for when the battery will be low.
    /// This value is only an estimate, not an exact date.
    #[serde(rename="estimatedLowBatteryDate")]
    
    pub estimated_low_battery_date: Option<Date>,
}

impl client::Part for Diagnostics {}


/// A generic empty message that you can re-use to avoid defining duplicated
/// empty messages in your APIs. A typical example is to use it as the request
/// or the response type of an API method. For instance:
/// 
/// ````text
/// service Foo {
///   rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);
/// }
/// ````
/// 
/// The JSON representation for `Empty` is empty JSON object `{}`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [attachments delete beacons](BeaconAttachmentDeleteCall) (response)
/// * [activate beacons](BeaconActivateCall) (response)
/// * [deactivate beacons](BeaconDeactivateCall) (response)
/// * [decommission beacons](BeaconDecommissionCall) (response)
/// * [delete beacons](BeaconDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Write-only registration parameters for beacons using Eddystone-EID format.
/// Two ways of securely registering an Eddystone-EID beacon with the service
/// are supported:
/// 
/// 1. Perform an ECDH key exchange via this API, including a previous call
///    to `GET /v1beta1/eidparams`. In this case the fields
///    `beacon_ecdh_public_key` and `service_ecdh_public_key` should be
///    populated and `beacon_identity_key` should not be populated. This
///    method ensures that only the two parties in the ECDH key exchange can
///    compute the identity key, which becomes a secret between them.
/// 2. Derive or obtain the beacon's identity key via other secure means
///    (perhaps an ECDH key exchange between the beacon and a mobile device
///    or any other secure method), and then submit the resulting identity key
///    to the service. In this case `beacon_identity_key` field should be
///    populated, and neither of `beacon_ecdh_public_key` nor
///    `service_ecdh_public_key` fields should be. The security of this method
///    depends on how securely the parties involved (in particular the
///    bluetooth client) handle the identity key, and obviously on how
///    securely the identity key was generated.
/// 
/// See [the Eddystone
/// specification](https://github.com/google/eddystone/tree/master/eddystone-eid)
/// at GitHub.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EphemeralIdRegistration {
    /// The beacon's public key used for the Elliptic curve Diffie-Hellman
    /// key exchange. When this field is populated, `service_ecdh_public_key`
    /// must also be populated, and `beacon_identity_key` must not be.
    #[serde(rename="beaconEcdhPublicKey")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub beacon_ecdh_public_key: Option<Vec<u8>>,
    /// The private key of the beacon. If this field is populated,
    /// `beacon_ecdh_public_key` and `service_ecdh_public_key` must not be
    /// populated.
    #[serde(rename="beaconIdentityKey")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub beacon_identity_key: Option<Vec<u8>>,
    /// The initial clock value of the beacon. The beacon's clock must have
    /// begun counting at this value immediately prior to transmitting this
    /// value to the resolving service. Significant delay in transmitting this
    /// value to the service risks registration or resolution failures. If a
    /// value is not provided, the default is zero.
    #[serde(rename="initialClockValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub initial_clock_value: Option<u64>,
    /// An initial ephemeral ID calculated using the clock value submitted as
    /// `initial_clock_value`, and the secret key generated by the
    /// Diffie-Hellman key exchange using `service_ecdh_public_key` and
    /// `service_ecdh_public_key`. This initial EID value will be used by the
    /// service to confirm that the key exchange process was successful.
    #[serde(rename="initialEid")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub initial_eid: Option<Vec<u8>>,
    /// Indicates the nominal period between each rotation of the beacon's
    /// ephemeral ID. "Nominal" because the beacon should randomize the
    /// actual interval. See [the spec at
    /// github](https://github.com/google/eddystone/tree/master/eddystone-eid)
    /// for details. This value corresponds to a power-of-two scaler on the
    /// beacon's clock: when the scaler value is K, the beacon will begin
    /// broadcasting a new ephemeral ID on average every 2^K seconds.
    #[serde(rename="rotationPeriodExponent")]
    
    pub rotation_period_exponent: Option<u32>,
    /// The service's public key used for the Elliptic curve Diffie-Hellman
    /// key exchange. When this field is populated, `beacon_ecdh_public_key`
    /// must also be populated, and `beacon_identity_key` must not be.
    #[serde(rename="serviceEcdhPublicKey")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub service_ecdh_public_key: Option<Vec<u8>>,
}

impl client::Part for EphemeralIdRegistration {}


/// Information a client needs to provision and register beacons that
/// broadcast Eddystone-EID format beacon IDs, using Elliptic curve
/// Diffie-Hellman key exchange. See
/// [the Eddystone
/// specification](https://github.com/google/eddystone/tree/master/eddystone-eid)
/// at GitHub.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get eidparams](MethodGetEidparamCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EphemeralIdRegistrationParams {
    /// Indicates the maximum rotation period supported by the service.
    /// See
    /// EddystoneEidRegistration.rotation_period_exponent
    #[serde(rename="maxRotationPeriodExponent")]
    
    pub max_rotation_period_exponent: Option<u32>,
    /// Indicates the minimum rotation period supported by the service.
    /// See
    /// EddystoneEidRegistration.rotation_period_exponent
    #[serde(rename="minRotationPeriodExponent")]
    
    pub min_rotation_period_exponent: Option<u32>,
    /// The beacon service's public key for use by a beacon to derive its
    /// Identity Key using Elliptic Curve Diffie-Hellman key exchange.
    #[serde(rename="serviceEcdhPublicKey")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub service_ecdh_public_key: Option<Vec<u8>>,
}

impl client::ResponseResult for EphemeralIdRegistrationParams {}


/// Request for beacon and attachment information about beacons that
/// a mobile client has encountered “in the wild”.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [getforobserved beaconinfo](BeaconinfoGetforobservedCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetInfoForObservedBeaconsRequest {
    /// Specifies what kind of attachments to include in the response.
    /// When given, the response will include only attachments of the given types.
    /// When empty, no attachments will be returned. Must be in the format
    /// <var>namespace/type</var>. Accepts `*` to specify all types in
    /// all namespaces owned by the client.
    /// Optional.
    #[serde(rename="namespacedTypes")]
    
    pub namespaced_types: Option<Vec<String>>,
    /// The beacons that the client has encountered.
    /// At least one must be given.
    
    pub observations: Option<Vec<Observation>>,
}

impl client::RequestValue for GetInfoForObservedBeaconsRequest {}


/// Information about the requested beacons, optionally including attachment
/// data.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [getforobserved beaconinfo](BeaconinfoGetforobservedCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetInfoForObservedBeaconsResponse {
    /// Public information about beacons.
    /// May be empty if the request matched no beacons.
    
    pub beacons: Option<Vec<BeaconInfo>>,
}

impl client::ResponseResult for GetInfoForObservedBeaconsResponse {}


/// Indoor level, a human-readable string as returned by Google Maps APIs,
/// useful to indicate which floor of a building a beacon is located on.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IndoorLevel {
    /// The name of this level.
    
    pub name: Option<String>,
}

impl client::Part for IndoorLevel {}


/// An object representing a latitude/longitude pair. This is expressed as a pair
/// of doubles representing degrees latitude and degrees longitude. Unless
/// specified otherwise, this must conform to the
/// <a href="http://www.unoosa.org/pdf/icg/2012/template/WGS_84.pdf">WGS84
/// standard</a>. Values must be within normalized ranges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LatLng {
    /// The latitude in degrees. It must be in the range [-90.0, +90.0].
    
    pub latitude: Option<f64>,
    /// The longitude in degrees. It must be in the range [-180.0, +180.0].
    
    pub longitude: Option<f64>,
}

impl client::Part for LatLng {}


/// Response to `ListBeaconAttachments` that contains the requested attachments.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [attachments list beacons](BeaconAttachmentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListBeaconAttachmentsResponse {
    /// The attachments that corresponded to the request params.
    
    pub attachments: Option<Vec<BeaconAttachment>>,
}

impl client::ResponseResult for ListBeaconAttachmentsResponse {}


/// Response that contains list beacon results and pagination help.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list beacons](BeaconListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListBeaconsResponse {
    /// The beacons that matched the search criteria.
    
    pub beacons: Option<Vec<Beacon>>,
    /// An opaque pagination token that the client may provide in their next
    /// request to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Estimate of the total number of beacons matched by the query. Higher
    /// values may be less accurate.
    #[serde(rename="totalCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_count: Option<i64>,
}

impl client::ResponseResult for ListBeaconsResponse {}


/// Response that contains the requested diagnostics.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [diagnostics list beacons](BeaconDiagnosticListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDiagnosticsResponse {
    /// The diagnostics matching the given request.
    
    pub diagnostics: Option<Vec<Diagnostics>>,
    /// Token that can be used for pagination. Returned only if the
    /// request matches more beacons than can be returned in this response.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListDiagnosticsResponse {}


/// Response to ListNamespacesRequest that contains all the project’s namespaces.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list namespaces](NamespaceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListNamespacesResponse {
    /// The attachments that corresponded to the request params.
    
    pub namespaces: Option<Vec<Namespace>>,
}

impl client::ResponseResult for ListNamespacesResponse {}


/// An attachment namespace defines read and write access for all the attachments
/// created under it. Each namespace is globally unique, and owned by one
/// project which is the only project that can create attachments under it.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list namespaces](NamespaceListCall) (none)
/// * [update namespaces](NamespaceUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Namespace {
    /// Resource name of this namespace. Namespaces names have the format:
    /// <code>namespaces/<var>namespace</var></code>.
    #[serde(rename="namespaceName")]
    
    pub namespace_name: Option<String>,
    /// Specifies what clients may receive attachments under this namespace
    /// via `beaconinfo.getforobserved`.
    #[serde(rename="servingVisibility")]
    
    pub serving_visibility: Option<String>,
}

impl client::RequestValue for Namespace {}
impl client::Resource for Namespace {}
impl client::ResponseResult for Namespace {}


/// Represents one beacon observed once.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Observation {
    /// The ID advertised by the beacon the client has encountered.
    /// 
    /// If the submitted `advertised_id` type is Eddystone-EID, then the client
    /// must be authorized to resolve the given beacon. Otherwise no data will be
    /// returned for that beacon.
    /// Required.
    #[serde(rename="advertisedId")]
    
    pub advertised_id: Option<AdvertisedId>,
    /// The array of telemetry bytes received from the beacon. The server is
    /// responsible for parsing it. This field may frequently be empty, as
    /// with a beacon that transmits telemetry only occasionally.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub telemetry: Option<Vec<u8>>,
    /// Time when the beacon was observed.
    #[serde(rename="timestampMs")]
    
    pub timestamp_ms: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for Observation {}


