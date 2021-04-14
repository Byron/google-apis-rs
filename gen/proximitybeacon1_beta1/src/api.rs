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
    /// View and modify your beacons
    UserlocationBeaconRegistry,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::UserlocationBeaconRegistry => "https://www.googleapis.com/auth/userlocation.beacon.registry",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::UserlocationBeaconRegistry
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Proximitybeacon related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// use proximitybeacon1_beta1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use proximitybeacon1_beta1::Proximitybeacon;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.beacons().diagnostics_list("beaconName")
///              .project_id("duo")
///              .page_token("ipsum")
///              .page_size(-62)
///              .alert_filter("Lorem")
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
pub struct Proximitybeacon<> {
    client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>,
    auth: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, > client::Hub for Proximitybeacon<> {}

impl<'a, > Proximitybeacon<> {

    pub fn new(client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>, authenticator: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>) -> Proximitybeacon<> {
        Proximitybeacon {
            client,
            auth: authenticator,
            _user_agent: "google-api-rust-client/2.0.3".to_string(),
            _base_url: "https://proximitybeacon.googleapis.com/".to_string(),
            _root_url: "https://proximitybeacon.googleapis.com/".to_string(),
        }
    }

    pub fn beaconinfo(&'a self) -> BeaconinfoMethods<'a> {
        BeaconinfoMethods { hub: &self }
    }
    pub fn beacons(&'a self) -> BeaconMethods<'a> {
        BeaconMethods { hub: &self }
    }
    pub fn methods(&'a self) -> MethodMethods<'a> {
        MethodMethods { hub: &self }
    }
    pub fn namespaces(&'a self) -> NamespaceMethods<'a> {
        NamespaceMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/2.0.3`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://proximitybeacon.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://proximitybeacon.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Defines a unique identifier of a beacon as broadcast by the device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdvertisedId {
    /// The actual beacon identifier, as broadcast by the beacon hardware. Must be
    /// [base64](http://tools.ietf.org/html/rfc4648#section-4) encoded in HTTP
    /// requests, and will be so encoded (with padding) in responses. The base64
    /// encoding should be of the binary byte-stream and not any textual (such as
    /// hex) representation thereof.
    /// Required.
    pub id: Option<String>,
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AttachmentInfo {
    /// An opaque data container for client-provided data.
    pub data: Option<String>,
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
/// 
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
    /// The [Google Places API](/places/place-id) Place ID of the place where
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
    pub provisioning_key: Option<String>,
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
/// 
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
    pub creation_time_ms: Option<String>,
    /// An opaque data container for client-provided data. Must be
    /// [base64](http://tools.ietf.org/html/rfc4648#section-4) encoded in HTTP
    /// requests, and will be so encoded (with padding) in responses.
    /// Required.
    pub data: Option<String>,
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
/// 
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EphemeralIdRegistration {
    /// The beacon's public key used for the Elliptic curve Diffie-Hellman
    /// key exchange. When this field is populated, `service_ecdh_public_key`
    /// must also be populated, and `beacon_identity_key` must not be.
    #[serde(rename="beaconEcdhPublicKey")]
    pub beacon_ecdh_public_key: Option<String>,
    /// The private key of the beacon. If this field is populated,
    /// `beacon_ecdh_public_key` and `service_ecdh_public_key` must not be
    /// populated.
    #[serde(rename="beaconIdentityKey")]
    pub beacon_identity_key: Option<String>,
    /// The initial clock value of the beacon. The beacon's clock must have
    /// begun counting at this value immediately prior to transmitting this
    /// value to the resolving service. Significant delay in transmitting this
    /// value to the service risks registration or resolution failures. If a
    /// value is not provided, the default is zero.
    #[serde(rename="initialClockValue")]
    pub initial_clock_value: Option<String>,
    /// An initial ephemeral ID calculated using the clock value submitted as
    /// `initial_clock_value`, and the secret key generated by the
    /// Diffie-Hellman key exchange using `service_ecdh_public_key` and
    /// `service_ecdh_public_key`. This initial EID value will be used by the
    /// service to confirm that the key exchange process was successful.
    #[serde(rename="initialEid")]
    pub initial_eid: Option<String>,
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
    pub service_ecdh_public_key: Option<String>,
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
/// 
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
    pub service_ecdh_public_key: Option<String>,
}

impl client::ResponseResult for EphemeralIdRegistrationParams {}


/// Request for beacon and attachment information about beacons that
/// a mobile client has encountered "in the wild".
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [getforobserved beaconinfo](BeaconinfoGetforobservedCall) (request)
/// 
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
/// 
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
/// 
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
/// 
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
    pub total_count: Option<String>,
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
/// 
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


/// Response to ListNamespacesRequest that contains all the project's namespaces.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list namespaces](NamespaceListCall) (response)
/// 
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
/// 
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
    pub telemetry: Option<String>,
    /// Time when the beacon was observed.
    #[serde(rename="timestampMs")]
    pub timestamp_ms: Option<String>,
}

impl client::Part for Observation {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *beaconinfo* resources.
/// It is not used directly, but through the `Proximitybeacon` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use proximitybeacon1_beta1::Proximitybeacon;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `getforobserved(...)`
/// // to build up your call.
/// let rb = hub.beaconinfo();
/// # }
/// ```
pub struct BeaconinfoMethods<'a>
    where  {

    hub: &'a Proximitybeacon<>,
}

impl<'a> client::MethodsBuilder for BeaconinfoMethods<'a> {}

impl<'a> BeaconinfoMethods<'a> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Given one or more beacon observations, returns any beacon information
    /// and attachments accessible to your application. Authorize by using the
    /// [API
    /// key](https://developers.google.com/beacons/proximity/get-started#request_a_browser_api_key)
    /// for the application.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn getforobserved(&self, request: GetInfoForObservedBeaconsRequest) -> BeaconinfoGetforobservedCall<'a> {
        BeaconinfoGetforobservedCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *beacon* resources.
/// It is not used directly, but through the `Proximitybeacon` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use proximitybeacon1_beta1::Proximitybeacon;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `activate(...)`, `attachments_batch_delete(...)`, `attachments_create(...)`, `attachments_delete(...)`, `attachments_list(...)`, `deactivate(...)`, `decommission(...)`, `delete(...)`, `diagnostics_list(...)`, `get(...)`, `list(...)`, `register(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.beacons();
/// # }
/// ```
pub struct BeaconMethods<'a>
    where  {

    hub: &'a Proximitybeacon<>,
}

impl<'a> client::MethodsBuilder for BeaconMethods<'a> {}

impl<'a> BeaconMethods<'a> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes multiple attachments on a given beacon. This operation is
    /// permanent and cannot be undone.
    /// 
    /// You can optionally specify `namespacedType` to choose which attachments
    /// should be deleted. If you do not specify `namespacedType`,  all your
    /// attachments on the given beacon will be deleted. You also may explicitly
    /// specify `*/*` to delete all.
    /// 
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **Is owner** or **Can edit** permissions in the Google
    /// Developers Console project.
    /// 
    /// # Arguments
    ///
    /// * `beaconName` - The beacon whose attachments should be deleted. A beacon name has the
    ///                  format "beacons/N!beaconId" where the beaconId is the base16 ID broadcast
    ///                  by the beacon and N is a code for the beacon's type. Possible values are
    ///                  `3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
    ///                  for AltBeacon. For Eddystone-EID beacons, you may use either the
    ///                  current EID or the beacon's "stable" UID.
    ///                  Required.
    pub fn attachments_batch_delete(&self, beacon_name: &str) -> BeaconAttachmentBatchDeleteCall<'a> {
        BeaconAttachmentBatchDeleteCall {
            hub: self.hub,
            _beacon_name: beacon_name.to_string(),
            _project_id: Default::default(),
            _namespaced_type: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Associates the given data with the specified beacon. Attachment data must
    /// contain two parts:
    /// <ul>
    /// <li>A namespaced type.</li>
    /// <li>The actual attachment data itself.</li>
    /// </ul>
    /// The namespaced type consists of two parts, the namespace and the type.
    /// The namespace must be one of the values returned by the `namespaces`
    /// endpoint, while the type can be a string of any characters except for the
    /// forward slash (`/`) up to 100 characters in length.
    /// 
    /// Attachment data can be up to 1024 bytes long.
    /// 
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **Is owner** or **Can edit** permissions in the Google
    /// Developers Console project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `beaconName` - Beacon on which the attachment should be created. A beacon name has the
    ///                  format "beacons/N!beaconId" where the beaconId is the base16 ID broadcast
    ///                  by the beacon and N is a code for the beacon's type. Possible values are
    ///                  `3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
    ///                  for AltBeacon. For Eddystone-EID beacons, you may use either the
    ///                  current EID or the beacon's "stable" UID.
    ///                  Required.
    pub fn attachments_create(&self, request: BeaconAttachment, beacon_name: &str) -> BeaconAttachmentCreateCall<'a> {
        BeaconAttachmentCreateCall {
            hub: self.hub,
            _request: request,
            _beacon_name: beacon_name.to_string(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified attachment for the given beacon. Each attachment has
    /// a unique attachment name (`attachmentName`) which is returned when you
    /// fetch the attachment data via this API. You specify this with the delete
    /// request to control which attachment is removed. This operation cannot be
    /// undone.
    /// 
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **Is owner** or **Can edit** permissions in the Google
    /// Developers Console project.
    /// 
    /// # Arguments
    ///
    /// * `attachmentName` - The attachment name (`attachmentName`) of
    ///                      the attachment to remove. For example:
    ///                      `beacons/3!893737abc9/attachments/c5e937-af0-494-959-ec49d12738`. For
    ///                      Eddystone-EID beacons, the beacon ID portion (`3!893737abc9`) may be the
    ///                      beacon's current EID, or its "stable" Eddystone-UID.
    ///                      Required.
    pub fn attachments_delete(&self, attachment_name: &str) -> BeaconAttachmentDeleteCall<'a> {
        BeaconAttachmentDeleteCall {
            hub: self.hub,
            _attachment_name: attachment_name.to_string(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the attachments for the specified beacon that match the specified
    /// namespaced-type pattern.
    /// 
    /// To control which namespaced types are returned, you add the
    /// `namespacedType` query parameter to the request. You must either use
    /// `*/*`, to return all attachments, or the namespace must be one of
    /// the ones returned from the  `namespaces` endpoint.
    /// 
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **viewer**, **Is owner** or **Can edit** permissions in
    /// the Google Developers Console project.
    /// 
    /// # Arguments
    ///
    /// * `beaconName` - Beacon whose attachments should be fetched. A beacon name has the
    ///                  format "beacons/N!beaconId" where the beaconId is the base16 ID broadcast
    ///                  by the beacon and N is a code for the beacon's type. Possible values are
    ///                  `3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
    ///                  for AltBeacon. For Eddystone-EID beacons, you may use either the
    ///                  current EID or the beacon's "stable" UID.
    ///                  Required.
    pub fn attachments_list(&self, beacon_name: &str) -> BeaconAttachmentListCall<'a> {
        BeaconAttachmentListCall {
            hub: self.hub,
            _beacon_name: beacon_name.to_string(),
            _project_id: Default::default(),
            _namespaced_type: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the diagnostics for a single beacon. You can also list diagnostics for
    /// all the beacons owned by your Google Developers Console project by using
    /// the beacon name `beacons/-`.
    /// 
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **viewer**, **Is owner** or **Can edit** permissions in
    /// the Google Developers Console project.
    /// 
    /// # Arguments
    ///
    /// * `beaconName` - Beacon that the diagnostics are for.
    pub fn diagnostics_list(&self, beacon_name: &str) -> BeaconDiagnosticListCall<'a> {
        BeaconDiagnosticListCall {
            hub: self.hub,
            _beacon_name: beacon_name.to_string(),
            _project_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _alert_filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Activates a beacon. A beacon that is active will return information
    /// and attachment data when queried via `beaconinfo.getforobserved`.
    /// Calling this method on an already active beacon will do nothing (but
    /// will return a successful response code).
    /// 
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **Is owner** or **Can edit** permissions in the Google
    /// Developers Console project.
    /// 
    /// # Arguments
    ///
    /// * `beaconName` - Beacon that should be activated. A beacon name has the format
    ///                  "beacons/N!beaconId" where the beaconId is the base16 ID broadcast by
    ///                  the beacon and N is a code for the beacon's type. Possible values are
    ///                  `3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
    ///                  for AltBeacon. For Eddystone-EID beacons, you may use either the
    ///                  current EID or the beacon's "stable" UID.
    ///                  Required.
    pub fn activate(&self, beacon_name: &str) -> BeaconActivateCall<'a> {
        BeaconActivateCall {
            hub: self.hub,
            _beacon_name: beacon_name.to_string(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deactivates a beacon. Once deactivated, the API will not return
    /// information nor attachment data for the beacon when queried via
    /// `beaconinfo.getforobserved`. Calling this method on an already inactive
    /// beacon will do nothing (but will return a successful response code).
    /// 
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **Is owner** or **Can edit** permissions in the Google
    /// Developers Console project.
    /// 
    /// # Arguments
    ///
    /// * `beaconName` - Beacon that should be deactivated. A beacon name has the format
    ///                  "beacons/N!beaconId" where the beaconId is the base16 ID broadcast by
    ///                  the beacon and N is a code for the beacon's type. Possible values are
    ///                  `3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
    ///                  for AltBeacon. For Eddystone-EID beacons, you may use either the
    ///                  current EID or the beacon's "stable" UID.
    ///                  Required.
    pub fn deactivate(&self, beacon_name: &str) -> BeaconDeactivateCall<'a> {
        BeaconDeactivateCall {
            hub: self.hub,
            _beacon_name: beacon_name.to_string(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Decommissions the specified beacon in the service. This beacon will no
    /// longer be returned from `beaconinfo.getforobserved`. This operation is
    /// permanent -- you will not be able to re-register a beacon with this ID
    /// again.
    /// 
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **Is owner** or **Can edit** permissions in the Google
    /// Developers Console project.
    /// 
    /// # Arguments
    ///
    /// * `beaconName` - Beacon that should be decommissioned. A beacon name has the format
    ///                  "beacons/N!beaconId" where the beaconId is the base16 ID broadcast by
    ///                  the beacon and N is a code for the beacon's type. Possible values are
    ///                  `3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
    ///                  for AltBeacon. For Eddystone-EID beacons, you may use either the
    ///                  current EID of the beacon's "stable" UID.
    ///                  Required.
    pub fn decommission(&self, beacon_name: &str) -> BeaconDecommissionCall<'a> {
        BeaconDecommissionCall {
            hub: self.hub,
            _beacon_name: beacon_name.to_string(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified beacon including all diagnostics data for the beacon
    /// as well as any attachments on the beacon (including those belonging to
    /// other projects). This operation cannot be undone.
    /// 
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **Is owner** or **Can edit** permissions in the Google
    /// Developers Console project.
    /// 
    /// # Arguments
    ///
    /// * `beaconName` - Beacon that should be deleted. A beacon name has the format
    ///                  "beacons/N!beaconId" where the beaconId is the base16 ID broadcast by
    ///                  the beacon and N is a code for the beacon's type. Possible values are
    ///                  `3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
    ///                  for AltBeacon. For Eddystone-EID beacons, you may use either the
    ///                  current EID or the beacon's "stable" UID.
    ///                  Required.
    pub fn delete(&self, beacon_name: &str) -> BeaconDeleteCall<'a> {
        BeaconDeleteCall {
            hub: self.hub,
            _beacon_name: beacon_name.to_string(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns detailed information about the specified beacon.
    /// 
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **viewer**, **Is owner** or **Can edit** permissions in
    /// the Google Developers Console project.
    /// 
    /// Requests may supply an Eddystone-EID beacon name in the form:
    /// `beacons/4!beaconId` where the `beaconId` is the base16 ephemeral ID
    /// broadcast by the beacon. The returned `Beacon` object will contain the
    /// beacon's stable Eddystone-UID. Clients not authorized to resolve the
    /// beacon's ephemeral Eddystone-EID broadcast will receive an error.
    /// 
    /// # Arguments
    ///
    /// * `beaconName` - Resource name of this beacon. A beacon name has the format
    ///                  "beacons/N!beaconId" where the beaconId is the base16 ID broadcast by
    ///                  the beacon and N is a code for the beacon's type. Possible values are
    ///                  `3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
    ///                  for AltBeacon. For Eddystone-EID beacons, you may use either the
    ///                  current EID or the beacon's "stable" UID.
    ///                  Required.
    pub fn get(&self, beacon_name: &str) -> BeaconGetCall<'a> {
        BeaconGetCall {
            hub: self.hub,
            _beacon_name: beacon_name.to_string(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Searches the beacon registry for beacons that match the given search
    /// criteria. Only those beacons that the client has permission to list
    /// will be returned.
    /// 
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **viewer**, **Is owner** or **Can edit** permissions in
    /// the Google Developers Console project.
    pub fn list(&self) -> BeaconListCall<'a> {
        BeaconListCall {
            hub: self.hub,
            _q: Default::default(),
            _project_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Registers a previously unregistered beacon given its `advertisedId`.
    /// These IDs are unique within the system. An ID can be registered only once.
    /// 
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **Is owner** or **Can edit** permissions in the Google
    /// Developers Console project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn register(&self, request: Beacon) -> BeaconRegisterCall<'a> {
        BeaconRegisterCall {
            hub: self.hub,
            _request: request,
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the information about the specified beacon. **Any field that you do
    /// not populate in the submitted beacon will be permanently erased**, so you
    /// should follow the "read, modify, write" pattern to avoid inadvertently
    /// destroying data.
    /// 
    /// Changes to the beacon status via this method will be  silently ignored.
    /// To update beacon status, use the separate methods on this API for
    /// activation, deactivation, and decommissioning.
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **Is owner** or **Can edit** permissions in the Google
    /// Developers Console project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `beaconName` - Resource name of this beacon. A beacon name has the format
    ///                  "beacons/N!beaconId" where the beaconId is the base16 ID broadcast by
    ///                  the beacon and N is a code for the beacon's type. Possible values are
    ///                  `3` for Eddystone, `1` for iBeacon, or `5` for AltBeacon.
    ///                  This field must be left empty when registering. After reading a beacon,
    ///                  clients can use the name for future operations.
    pub fn update(&self, request: Beacon, beacon_name: &str) -> BeaconUpdateCall<'a> {
        BeaconUpdateCall {
            hub: self.hub,
            _request: request,
            _beacon_name: beacon_name.to_string(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *namespace* resources.
/// It is not used directly, but through the `Proximitybeacon` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use proximitybeacon1_beta1::Proximitybeacon;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.namespaces();
/// # }
/// ```
pub struct NamespaceMethods<'a>
    where  {

    hub: &'a Proximitybeacon<>,
}

impl<'a> client::MethodsBuilder for NamespaceMethods<'a> {}

impl<'a> NamespaceMethods<'a> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all attachment namespaces owned by your Google Developers Console
    /// project. Attachment data associated with a beacon must include a
    /// namespaced type, and the namespace must be owned by your project.
    /// 
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **viewer**, **Is owner** or **Can edit** permissions in
    /// the Google Developers Console project.
    pub fn list(&self) -> NamespaceListCall<'a> {
        NamespaceListCall {
            hub: self.hub,
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the information about the specified namespace. Only the namespace
    /// visibility can be updated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `namespaceName` - Resource name of this namespace. Namespaces names have the format:
    ///                     <code>namespaces/<var>namespace</var></code>.
    pub fn update(&self, request: Namespace, namespace_name: &str) -> NamespaceUpdateCall<'a> {
        NamespaceUpdateCall {
            hub: self.hub,
            _request: request,
            _namespace_name: namespace_name.to_string(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all free methods, which are not associated with a particular resource.
/// It is not used directly, but through the `Proximitybeacon` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use proximitybeacon1_beta1::Proximitybeacon;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_eidparams(...)`
/// // to build up your call.
/// let rb = hub.methods();
/// # }
/// ```
pub struct MethodMethods<'a>
    where  {

    hub: &'a Proximitybeacon<>,
}

impl<'a> client::MethodsBuilder for MethodMethods<'a> {}

impl<'a> MethodMethods<'a> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the Proximity Beacon API's current public key and associated
    /// parameters used to initiate the Diffie-Hellman key exchange required to
    /// register a beacon that broadcasts the Eddystone-EID format. This key
    /// changes periodically; clients may cache it and re-use the same public key
    /// to provision and register multiple beacons. However, clients should be
    /// prepared to refresh this key when they encounter an error registering an
    /// Eddystone-EID beacon.
    pub fn get_eidparams(&self) -> MethodGetEidparamCall<'a> {
        MethodGetEidparamCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Given one or more beacon observations, returns any beacon information
/// and attachments accessible to your application. Authorize by using the
/// [API
/// key](https://developers.google.com/beacons/proximity/get-started#request_a_browser_api_key)
/// for the application.
///
/// A builder for the *getforobserved* method supported by a *beaconinfo* resource.
/// It is not used directly, but through a `BeaconinfoMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// use proximitybeacon1_beta1::api::GetInfoForObservedBeaconsRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use proximitybeacon1_beta1::Proximitybeacon;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GetInfoForObservedBeaconsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.beaconinfo().getforobserved(req)
///              .doit().await;
/// # }
/// ```
pub struct BeaconinfoGetforobservedCall<'a>
    where  {

    hub: &'a Proximitybeacon<>,
    _request: GetInfoForObservedBeaconsRequest,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a> client::CallBuilder for BeaconinfoGetforobservedCall<'a> {}

impl<'a> BeaconinfoGetforobservedCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GetInfoForObservedBeaconsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beaconinfo.getforobserved",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/beaconinfo:getforobserved";
        
        let key = dlg.api_key();
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone());


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: GetInfoForObservedBeaconsRequest) -> BeaconinfoGetforobservedCall<'a> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconinfoGetforobservedCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconinfoGetforobservedCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Deletes multiple attachments on a given beacon. This operation is
/// permanent and cannot be undone.
/// 
/// You can optionally specify `namespacedType` to choose which attachments
/// should be deleted. If you do not specify `namespacedType`,  all your
/// attachments on the given beacon will be deleted. You also may explicitly
/// specify `*/*` to delete all.
/// 
/// Authenticate using an [OAuth access
/// token](https://developers.google.com/identity/protocols/OAuth2) from a
/// signed-in user with **Is owner** or **Can edit** permissions in the Google
/// Developers Console project.
///
/// A builder for the *attachments.batchDelete* method supported by a *beacon* resource.
/// It is not used directly, but through a `BeaconMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use proximitybeacon1_beta1::Proximitybeacon;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.beacons().attachments_batch_delete("beaconName")
///              .project_id("eos")
///              .namespaced_type("dolor")
///              .doit().await;
/// # }
/// ```
pub struct BeaconAttachmentBatchDeleteCall<'a>
    where  {

    hub: &'a Proximitybeacon<>,
    _beacon_name: String,
    _project_id: Option<String>,
    _namespaced_type: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BeaconAttachmentBatchDeleteCall<'a> {}

impl<'a> BeaconAttachmentBatchDeleteCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, DeleteAttachmentsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beacons.attachments.batchDelete",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("beaconName", self._beacon_name.to_string()));
        if let Some(value) = self._project_id {
            params.push(("projectId", value.to_string()));
        }
        if let Some(value) = self._namespaced_type {
            params.push(("namespacedType", value.to_string()));
        }
        for &field in ["alt", "beaconName", "projectId", "namespacedType"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/{+beaconName}/attachments:batchDelete";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+beaconName}", "beaconName")].iter() {
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
            for param_name in ["beaconName"].iter() {
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
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// The beacon whose attachments should be deleted. A beacon name has the
    /// format "beacons/N!beaconId" where the beaconId is the base16 ID broadcast
    /// by the beacon and N is a code for the beacon's type. Possible values are
    /// `3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
    /// for AltBeacon. For Eddystone-EID beacons, you may use either the
    /// current EID or the beacon's "stable" UID.
    /// Required.
    ///
    /// Sets the *beacon name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn beacon_name(mut self, new_value: &str) -> BeaconAttachmentBatchDeleteCall<'a> {
        self._beacon_name = new_value.to_string();
        self
    }
    /// The project id to delete beacon attachments under. This field can be
    /// used when "*" is specified to mean all attachment namespaces. Projects
    /// may have multiple attachments with multiple namespaces. If "*" is
    /// specified and the projectId string is empty, then the project
    /// making the request is used.
    /// Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> BeaconAttachmentBatchDeleteCall<'a> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// Specifies the namespace and type of attachments to delete in
    /// `namespace/type` format. Accepts `*/*` to specify
    /// "all types in all namespaces".
    /// Optional.
    ///
    /// Sets the *namespaced type* query property to the given value.
    pub fn namespaced_type(mut self, new_value: &str) -> BeaconAttachmentBatchDeleteCall<'a> {
        self._namespaced_type = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconAttachmentBatchDeleteCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconAttachmentBatchDeleteCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::UserlocationBeaconRegistry`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BeaconAttachmentBatchDeleteCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Associates the given data with the specified beacon. Attachment data must
/// contain two parts:
/// <ul>
/// <li>A namespaced type.</li>
/// <li>The actual attachment data itself.</li>
/// </ul>
/// The namespaced type consists of two parts, the namespace and the type.
/// The namespace must be one of the values returned by the `namespaces`
/// endpoint, while the type can be a string of any characters except for the
/// forward slash (`/`) up to 100 characters in length.
/// 
/// Attachment data can be up to 1024 bytes long.
/// 
/// Authenticate using an [OAuth access
/// token](https://developers.google.com/identity/protocols/OAuth2) from a
/// signed-in user with **Is owner** or **Can edit** permissions in the Google
/// Developers Console project.
///
/// A builder for the *attachments.create* method supported by a *beacon* resource.
/// It is not used directly, but through a `BeaconMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// use proximitybeacon1_beta1::api::BeaconAttachment;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use proximitybeacon1_beta1::Proximitybeacon;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BeaconAttachment::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.beacons().attachments_create(req, "beaconName")
///              .project_id("ipsum")
///              .doit().await;
/// # }
/// ```
pub struct BeaconAttachmentCreateCall<'a>
    where  {

    hub: &'a Proximitybeacon<>,
    _request: BeaconAttachment,
    _beacon_name: String,
    _project_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BeaconAttachmentCreateCall<'a> {}

impl<'a> BeaconAttachmentCreateCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, BeaconAttachment)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beacons.attachments.create",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("beaconName", self._beacon_name.to_string()));
        if let Some(value) = self._project_id {
            params.push(("projectId", value.to_string()));
        }
        for &field in ["alt", "beaconName", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/{+beaconName}/attachments";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+beaconName}", "beaconName")].iter() {
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
            for param_name in ["beaconName"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: BeaconAttachment) -> BeaconAttachmentCreateCall<'a> {
        self._request = new_value;
        self
    }
    /// Beacon on which the attachment should be created. A beacon name has the
    /// format "beacons/N!beaconId" where the beaconId is the base16 ID broadcast
    /// by the beacon and N is a code for the beacon's type. Possible values are
    /// `3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
    /// for AltBeacon. For Eddystone-EID beacons, you may use either the
    /// current EID or the beacon's "stable" UID.
    /// Required.
    ///
    /// Sets the *beacon name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn beacon_name(mut self, new_value: &str) -> BeaconAttachmentCreateCall<'a> {
        self._beacon_name = new_value.to_string();
        self
    }
    /// The project id of the project the attachment will belong to. If
    /// the project id is not specified then the project making the request
    /// is used.
    /// Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> BeaconAttachmentCreateCall<'a> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconAttachmentCreateCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconAttachmentCreateCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::UserlocationBeaconRegistry`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BeaconAttachmentCreateCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes the specified attachment for the given beacon. Each attachment has
/// a unique attachment name (`attachmentName`) which is returned when you
/// fetch the attachment data via this API. You specify this with the delete
/// request to control which attachment is removed. This operation cannot be
/// undone.
/// 
/// Authenticate using an [OAuth access
/// token](https://developers.google.com/identity/protocols/OAuth2) from a
/// signed-in user with **Is owner** or **Can edit** permissions in the Google
/// Developers Console project.
///
/// A builder for the *attachments.delete* method supported by a *beacon* resource.
/// It is not used directly, but through a `BeaconMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use proximitybeacon1_beta1::Proximitybeacon;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.beacons().attachments_delete("attachmentName")
///              .project_id("amet")
///              .doit().await;
/// # }
/// ```
pub struct BeaconAttachmentDeleteCall<'a>
    where  {

    hub: &'a Proximitybeacon<>,
    _attachment_name: String,
    _project_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BeaconAttachmentDeleteCall<'a> {}

impl<'a> BeaconAttachmentDeleteCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Empty)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beacons.attachments.delete",
                               http_method: hyper::Method::DELETE });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("attachmentName", self._attachment_name.to_string()));
        if let Some(value) = self._project_id {
            params.push(("projectId", value.to_string()));
        }
        for &field in ["alt", "attachmentName", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/{+attachmentName}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+attachmentName}", "attachmentName")].iter() {
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
            for param_name in ["attachmentName"].iter() {
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
                let mut req_builder = hyper::Request::builder().method(hyper::Method::DELETE).uri(url.clone().into_string())
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// The attachment name (`attachmentName`) of
    /// the attachment to remove. For example:
    /// `beacons/3!893737abc9/attachments/c5e937-af0-494-959-ec49d12738`. For
    /// Eddystone-EID beacons, the beacon ID portion (`3!893737abc9`) may be the
    /// beacon's current EID, or its "stable" Eddystone-UID.
    /// Required.
    ///
    /// Sets the *attachment name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn attachment_name(mut self, new_value: &str) -> BeaconAttachmentDeleteCall<'a> {
        self._attachment_name = new_value.to_string();
        self
    }
    /// The project id of the attachment to delete. If not provided, the project
    /// that is making the request is used.
    /// Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> BeaconAttachmentDeleteCall<'a> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconAttachmentDeleteCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconAttachmentDeleteCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::UserlocationBeaconRegistry`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BeaconAttachmentDeleteCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns the attachments for the specified beacon that match the specified
/// namespaced-type pattern.
/// 
/// To control which namespaced types are returned, you add the
/// `namespacedType` query parameter to the request. You must either use
/// `*/*`, to return all attachments, or the namespace must be one of
/// the ones returned from the  `namespaces` endpoint.
/// 
/// Authenticate using an [OAuth access
/// token](https://developers.google.com/identity/protocols/OAuth2) from a
/// signed-in user with **viewer**, **Is owner** or **Can edit** permissions in
/// the Google Developers Console project.
///
/// A builder for the *attachments.list* method supported by a *beacon* resource.
/// It is not used directly, but through a `BeaconMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use proximitybeacon1_beta1::Proximitybeacon;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.beacons().attachments_list("beaconName")
///              .project_id("ipsum")
///              .namespaced_type("sed")
///              .doit().await;
/// # }
/// ```
pub struct BeaconAttachmentListCall<'a>
    where  {

    hub: &'a Proximitybeacon<>,
    _beacon_name: String,
    _project_id: Option<String>,
    _namespaced_type: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BeaconAttachmentListCall<'a> {}

impl<'a> BeaconAttachmentListCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListBeaconAttachmentsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beacons.attachments.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("beaconName", self._beacon_name.to_string()));
        if let Some(value) = self._project_id {
            params.push(("projectId", value.to_string()));
        }
        if let Some(value) = self._namespaced_type {
            params.push(("namespacedType", value.to_string()));
        }
        for &field in ["alt", "beaconName", "projectId", "namespacedType"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/{+beaconName}/attachments";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+beaconName}", "beaconName")].iter() {
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
            for param_name in ["beaconName"].iter() {
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// Beacon whose attachments should be fetched. A beacon name has the
    /// format "beacons/N!beaconId" where the beaconId is the base16 ID broadcast
    /// by the beacon and N is a code for the beacon's type. Possible values are
    /// `3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
    /// for AltBeacon. For Eddystone-EID beacons, you may use either the
    /// current EID or the beacon's "stable" UID.
    /// Required.
    ///
    /// Sets the *beacon name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn beacon_name(mut self, new_value: &str) -> BeaconAttachmentListCall<'a> {
        self._beacon_name = new_value.to_string();
        self
    }
    /// The project id to list beacon attachments under. This field can be
    /// used when "*" is specified to mean all attachment namespaces. Projects
    /// may have multiple attachments with multiple namespaces. If "*" is
    /// specified and the projectId string is empty, then the project
    /// making the request is used.
    /// Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> BeaconAttachmentListCall<'a> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// Specifies the namespace and type of attachment to include in response in
    /// <var>namespace/type</var> format. Accepts `*/*` to specify
    /// "all types in all namespaces".
    ///
    /// Sets the *namespaced type* query property to the given value.
    pub fn namespaced_type(mut self, new_value: &str) -> BeaconAttachmentListCall<'a> {
        self._namespaced_type = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconAttachmentListCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconAttachmentListCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::UserlocationBeaconRegistry`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BeaconAttachmentListCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// List the diagnostics for a single beacon. You can also list diagnostics for
/// all the beacons owned by your Google Developers Console project by using
/// the beacon name `beacons/-`.
/// 
/// Authenticate using an [OAuth access
/// token](https://developers.google.com/identity/protocols/OAuth2) from a
/// signed-in user with **viewer**, **Is owner** or **Can edit** permissions in
/// the Google Developers Console project.
///
/// A builder for the *diagnostics.list* method supported by a *beacon* resource.
/// It is not used directly, but through a `BeaconMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use proximitybeacon1_beta1::Proximitybeacon;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.beacons().diagnostics_list("beaconName")
///              .project_id("gubergren")
///              .page_token("rebum.")
///              .page_size(-57)
///              .alert_filter("ipsum")
///              .doit().await;
/// # }
/// ```
pub struct BeaconDiagnosticListCall<'a>
    where  {

    hub: &'a Proximitybeacon<>,
    _beacon_name: String,
    _project_id: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _alert_filter: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BeaconDiagnosticListCall<'a> {}

impl<'a> BeaconDiagnosticListCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListDiagnosticsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beacons.diagnostics.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("beaconName", self._beacon_name.to_string()));
        if let Some(value) = self._project_id {
            params.push(("projectId", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._alert_filter {
            params.push(("alertFilter", value.to_string()));
        }
        for &field in ["alt", "beaconName", "projectId", "pageToken", "pageSize", "alertFilter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/{+beaconName}/diagnostics";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+beaconName}", "beaconName")].iter() {
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
            for param_name in ["beaconName"].iter() {
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// Beacon that the diagnostics are for.
    ///
    /// Sets the *beacon name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn beacon_name(mut self, new_value: &str) -> BeaconDiagnosticListCall<'a> {
        self._beacon_name = new_value.to_string();
        self
    }
    /// Requests only diagnostic records for the given project id. If not set,
    /// then the project making the request will be used for looking up
    /// diagnostic records. Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> BeaconDiagnosticListCall<'a> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// Requests results that occur after the `page_token`, obtained from the
    /// response to a previous request. Optional.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> BeaconDiagnosticListCall<'a> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Specifies the maximum number of results to return. Defaults to
    /// 10. Maximum 1000. Optional.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> BeaconDiagnosticListCall<'a> {
        self._page_size = Some(new_value);
        self
    }
    /// Requests only beacons that have the given alert. For example, to find
    /// beacons that have low batteries use `alert_filter=LOW_BATTERY`.
    ///
    /// Sets the *alert filter* query property to the given value.
    pub fn alert_filter(mut self, new_value: &str) -> BeaconDiagnosticListCall<'a> {
        self._alert_filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconDiagnosticListCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconDiagnosticListCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::UserlocationBeaconRegistry`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BeaconDiagnosticListCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Activates a beacon. A beacon that is active will return information
/// and attachment data when queried via `beaconinfo.getforobserved`.
/// Calling this method on an already active beacon will do nothing (but
/// will return a successful response code).
/// 
/// Authenticate using an [OAuth access
/// token](https://developers.google.com/identity/protocols/OAuth2) from a
/// signed-in user with **Is owner** or **Can edit** permissions in the Google
/// Developers Console project.
///
/// A builder for the *activate* method supported by a *beacon* resource.
/// It is not used directly, but through a `BeaconMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use proximitybeacon1_beta1::Proximitybeacon;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.beacons().activate("beaconName")
///              .project_id("est")
///              .doit().await;
/// # }
/// ```
pub struct BeaconActivateCall<'a>
    where  {

    hub: &'a Proximitybeacon<>,
    _beacon_name: String,
    _project_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BeaconActivateCall<'a> {}

impl<'a> BeaconActivateCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Empty)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beacons.activate",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("beaconName", self._beacon_name.to_string()));
        if let Some(value) = self._project_id {
            params.push(("projectId", value.to_string()));
        }
        for &field in ["alt", "beaconName", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/{+beaconName}:activate";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+beaconName}", "beaconName")].iter() {
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
            for param_name in ["beaconName"].iter() {
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
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// Beacon that should be activated. A beacon name has the format
    /// "beacons/N!beaconId" where the beaconId is the base16 ID broadcast by
    /// the beacon and N is a code for the beacon's type. Possible values are
    /// `3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
    /// for AltBeacon. For Eddystone-EID beacons, you may use either the
    /// current EID or the beacon's "stable" UID.
    /// Required.
    ///
    /// Sets the *beacon name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn beacon_name(mut self, new_value: &str) -> BeaconActivateCall<'a> {
        self._beacon_name = new_value.to_string();
        self
    }
    /// The project id of the beacon to activate. If the project id is not
    /// specified then the project making the request is used. The project id
    /// must match the project that owns the beacon.
    /// Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> BeaconActivateCall<'a> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconActivateCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconActivateCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::UserlocationBeaconRegistry`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BeaconActivateCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deactivates a beacon. Once deactivated, the API will not return
/// information nor attachment data for the beacon when queried via
/// `beaconinfo.getforobserved`. Calling this method on an already inactive
/// beacon will do nothing (but will return a successful response code).
/// 
/// Authenticate using an [OAuth access
/// token](https://developers.google.com/identity/protocols/OAuth2) from a
/// signed-in user with **Is owner** or **Can edit** permissions in the Google
/// Developers Console project.
///
/// A builder for the *deactivate* method supported by a *beacon* resource.
/// It is not used directly, but through a `BeaconMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use proximitybeacon1_beta1::Proximitybeacon;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.beacons().deactivate("beaconName")
///              .project_id("ea")
///              .doit().await;
/// # }
/// ```
pub struct BeaconDeactivateCall<'a>
    where  {

    hub: &'a Proximitybeacon<>,
    _beacon_name: String,
    _project_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BeaconDeactivateCall<'a> {}

impl<'a> BeaconDeactivateCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Empty)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beacons.deactivate",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("beaconName", self._beacon_name.to_string()));
        if let Some(value) = self._project_id {
            params.push(("projectId", value.to_string()));
        }
        for &field in ["alt", "beaconName", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/{+beaconName}:deactivate";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+beaconName}", "beaconName")].iter() {
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
            for param_name in ["beaconName"].iter() {
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
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// Beacon that should be deactivated. A beacon name has the format
    /// "beacons/N!beaconId" where the beaconId is the base16 ID broadcast by
    /// the beacon and N is a code for the beacon's type. Possible values are
    /// `3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
    /// for AltBeacon. For Eddystone-EID beacons, you may use either the
    /// current EID or the beacon's "stable" UID.
    /// Required.
    ///
    /// Sets the *beacon name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn beacon_name(mut self, new_value: &str) -> BeaconDeactivateCall<'a> {
        self._beacon_name = new_value.to_string();
        self
    }
    /// The project id of the beacon to deactivate. If the project id is not
    /// specified then the project making the request is used. The project id must
    /// match the project that owns the beacon.
    /// Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> BeaconDeactivateCall<'a> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconDeactivateCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconDeactivateCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::UserlocationBeaconRegistry`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BeaconDeactivateCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Decommissions the specified beacon in the service. This beacon will no
/// longer be returned from `beaconinfo.getforobserved`. This operation is
/// permanent -- you will not be able to re-register a beacon with this ID
/// again.
/// 
/// Authenticate using an [OAuth access
/// token](https://developers.google.com/identity/protocols/OAuth2) from a
/// signed-in user with **Is owner** or **Can edit** permissions in the Google
/// Developers Console project.
///
/// A builder for the *decommission* method supported by a *beacon* resource.
/// It is not used directly, but through a `BeaconMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use proximitybeacon1_beta1::Proximitybeacon;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.beacons().decommission("beaconName")
///              .project_id("Lorem")
///              .doit().await;
/// # }
/// ```
pub struct BeaconDecommissionCall<'a>
    where  {

    hub: &'a Proximitybeacon<>,
    _beacon_name: String,
    _project_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BeaconDecommissionCall<'a> {}

impl<'a> BeaconDecommissionCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Empty)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beacons.decommission",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("beaconName", self._beacon_name.to_string()));
        if let Some(value) = self._project_id {
            params.push(("projectId", value.to_string()));
        }
        for &field in ["alt", "beaconName", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/{+beaconName}:decommission";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+beaconName}", "beaconName")].iter() {
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
            for param_name in ["beaconName"].iter() {
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
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// Beacon that should be decommissioned. A beacon name has the format
    /// "beacons/N!beaconId" where the beaconId is the base16 ID broadcast by
    /// the beacon and N is a code for the beacon's type. Possible values are
    /// `3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
    /// for AltBeacon. For Eddystone-EID beacons, you may use either the
    /// current EID of the beacon's "stable" UID.
    /// Required.
    ///
    /// Sets the *beacon name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn beacon_name(mut self, new_value: &str) -> BeaconDecommissionCall<'a> {
        self._beacon_name = new_value.to_string();
        self
    }
    /// The project id of the beacon to decommission. If the project id is not
    /// specified then the project making the request is used. The project id
    /// must match the project that owns the beacon.
    /// Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> BeaconDecommissionCall<'a> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconDecommissionCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconDecommissionCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::UserlocationBeaconRegistry`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BeaconDecommissionCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes the specified beacon including all diagnostics data for the beacon
/// as well as any attachments on the beacon (including those belonging to
/// other projects). This operation cannot be undone.
/// 
/// Authenticate using an [OAuth access
/// token](https://developers.google.com/identity/protocols/OAuth2) from a
/// signed-in user with **Is owner** or **Can edit** permissions in the Google
/// Developers Console project.
///
/// A builder for the *delete* method supported by a *beacon* resource.
/// It is not used directly, but through a `BeaconMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use proximitybeacon1_beta1::Proximitybeacon;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.beacons().delete("beaconName")
///              .project_id("labore")
///              .doit().await;
/// # }
/// ```
pub struct BeaconDeleteCall<'a>
    where  {

    hub: &'a Proximitybeacon<>,
    _beacon_name: String,
    _project_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BeaconDeleteCall<'a> {}

impl<'a> BeaconDeleteCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Empty)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beacons.delete",
                               http_method: hyper::Method::DELETE });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("beaconName", self._beacon_name.to_string()));
        if let Some(value) = self._project_id {
            params.push(("projectId", value.to_string()));
        }
        for &field in ["alt", "beaconName", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/{+beaconName}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+beaconName}", "beaconName")].iter() {
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
            for param_name in ["beaconName"].iter() {
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
                let mut req_builder = hyper::Request::builder().method(hyper::Method::DELETE).uri(url.clone().into_string())
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// Beacon that should be deleted. A beacon name has the format
    /// "beacons/N!beaconId" where the beaconId is the base16 ID broadcast by
    /// the beacon and N is a code for the beacon's type. Possible values are
    /// `3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
    /// for AltBeacon. For Eddystone-EID beacons, you may use either the
    /// current EID or the beacon's "stable" UID.
    /// Required.
    ///
    /// Sets the *beacon name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn beacon_name(mut self, new_value: &str) -> BeaconDeleteCall<'a> {
        self._beacon_name = new_value.to_string();
        self
    }
    /// The project id of the beacon to delete. If not provided, the project
    /// that is making the request is used.
    /// Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> BeaconDeleteCall<'a> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconDeleteCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconDeleteCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::UserlocationBeaconRegistry`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BeaconDeleteCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns detailed information about the specified beacon.
/// 
/// Authenticate using an [OAuth access
/// token](https://developers.google.com/identity/protocols/OAuth2) from a
/// signed-in user with **viewer**, **Is owner** or **Can edit** permissions in
/// the Google Developers Console project.
/// 
/// Requests may supply an Eddystone-EID beacon name in the form:
/// `beacons/4!beaconId` where the `beaconId` is the base16 ephemeral ID
/// broadcast by the beacon. The returned `Beacon` object will contain the
/// beacon's stable Eddystone-UID. Clients not authorized to resolve the
/// beacon's ephemeral Eddystone-EID broadcast will receive an error.
///
/// A builder for the *get* method supported by a *beacon* resource.
/// It is not used directly, but through a `BeaconMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use proximitybeacon1_beta1::Proximitybeacon;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.beacons().get("beaconName")
///              .project_id("duo")
///              .doit().await;
/// # }
/// ```
pub struct BeaconGetCall<'a>
    where  {

    hub: &'a Proximitybeacon<>,
    _beacon_name: String,
    _project_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BeaconGetCall<'a> {}

impl<'a> BeaconGetCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Beacon)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beacons.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("beaconName", self._beacon_name.to_string()));
        if let Some(value) = self._project_id {
            params.push(("projectId", value.to_string()));
        }
        for &field in ["alt", "beaconName", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/{+beaconName}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+beaconName}", "beaconName")].iter() {
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
            for param_name in ["beaconName"].iter() {
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// Resource name of this beacon. A beacon name has the format
    /// "beacons/N!beaconId" where the beaconId is the base16 ID broadcast by
    /// the beacon and N is a code for the beacon's type. Possible values are
    /// `3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
    /// for AltBeacon. For Eddystone-EID beacons, you may use either the
    /// current EID or the beacon's "stable" UID.
    /// Required.
    ///
    /// Sets the *beacon name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn beacon_name(mut self, new_value: &str) -> BeaconGetCall<'a> {
        self._beacon_name = new_value.to_string();
        self
    }
    /// The project id of the beacon to request. If the project id is not specified
    /// then the project making the request is used. The project id must match the
    /// project that owns the beacon.
    /// Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> BeaconGetCall<'a> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconGetCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconGetCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::UserlocationBeaconRegistry`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BeaconGetCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Searches the beacon registry for beacons that match the given search
/// criteria. Only those beacons that the client has permission to list
/// will be returned.
/// 
/// Authenticate using an [OAuth access
/// token](https://developers.google.com/identity/protocols/OAuth2) from a
/// signed-in user with **viewer**, **Is owner** or **Can edit** permissions in
/// the Google Developers Console project.
///
/// A builder for the *list* method supported by a *beacon* resource.
/// It is not used directly, but through a `BeaconMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use proximitybeacon1_beta1::Proximitybeacon;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.beacons().list()
///              .q("sed")
///              .project_id("no")
///              .page_token("Stet")
///              .page_size(-13)
///              .doit().await;
/// # }
/// ```
pub struct BeaconListCall<'a>
    where  {

    hub: &'a Proximitybeacon<>,
    _q: Option<String>,
    _project_id: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BeaconListCall<'a> {}

impl<'a> BeaconListCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListBeaconsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beacons.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        if let Some(value) = self._q {
            params.push(("q", value.to_string()));
        }
        if let Some(value) = self._project_id {
            params.push(("projectId", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        for &field in ["alt", "q", "projectId", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/beacons";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string(), ());
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// Filter query string that supports the following field filters:
    /// 
    /// * **description:`"<string>"`**
    ///   For example: **description:"Room 3"**
    ///   Returns beacons whose description matches tokens in the string "Room 3"
    ///   (not necessarily that exact string).
    ///   The string must be double-quoted.
    /// * **status:`<enum>`**
    ///   For example: **status:active**
    ///   Returns beacons whose status matches the given value. Values must be
    ///   one of the Beacon.Status enum values (case insensitive). Accepts
    ///   multiple filters which will be combined with OR logic.
    /// * **stability:`<enum>`**
    ///   For example: **stability:mobile**
    ///   Returns beacons whose expected stability matches the given value.
    ///   Values must be one of the Beacon.Stability enum values (case
    ///   insensitive). Accepts multiple filters which will be combined with
    ///   OR logic.
    /// * **place\_id:`"<string>"`**
    ///   For example: **place\_id:"ChIJVSZzVR8FdkgRXGmmm6SslKw="**
    ///   Returns beacons explicitly registered at the given place, expressed as
    ///   a Place ID obtained from [Google Places API](/places/place-id). Does not
    ///   match places inside the given place. Does not consider the beacon's
    ///   actual location (which may be different from its registered place).
    ///   Accepts multiple filters that will be combined with OR logic. The place
    ///   ID must be double-quoted.
    /// * **registration\_time`[<|>|<=|>=]<integer>`**
    ///   For example: **registration\_time>=1433116800**
    ///   Returns beacons whose registration time matches the given filter.
    ///   Supports the operators: <, >, <=, and >=. Timestamp must be expressed as
    ///   an integer number of seconds since midnight January 1, 1970 UTC. Accepts
    ///   at most two filters that will be combined with AND logic, to support
    ///   "between" semantics. If more than two are supplied, the latter ones are
    ///   ignored.
    /// * **lat:`<double> lng:<double> radius:<integer>`**
    ///   For example: **lat:51.1232343 lng:-1.093852 radius:1000**
    ///   Returns beacons whose registered location is within the given circle.
    ///   When any of these fields are given, all are required. Latitude and
    ///   longitude must be decimal degrees between -90.0 and 90.0 and between
    ///   -180.0 and 180.0 respectively. Radius must be an integer number of
    ///   meters between 10 and 1,000,000 (1000 km).
    /// * **property:`"<string>=<string>"`**
    ///   For example: **property:"battery-type=CR2032"**
    ///   Returns beacons which have a property of the given name and value.
    ///   Supports multiple filters which will be combined with OR logic.
    ///   The entire name=value string must be double-quoted as one string.
    /// * **attachment\_type:`"<string>"`**
    ///   For example: **attachment_type:"my-namespace/my-type"**
    ///   Returns beacons having at least one attachment of the given namespaced
    ///   type. Supports "any within this namespace" via the partial wildcard
    ///   syntax: "my-namespace/*". Supports multiple filters which will be
    ///   combined with OR logic. The string must be double-quoted.
    /// * **indoor\_level:`"<string>"`**
    ///   For example: **indoor\_level:"1"**
    ///   Returns beacons which are located on the given indoor level. Accepts
    ///   multiple filters that will be combined with OR logic.
    /// 
    /// Multiple filters on the same field are combined with OR logic (except
    /// registration_time which is combined with AND logic).
    /// Multiple filters on different fields are combined with AND logic.
    /// Filters should be separated by spaces.
    /// 
    /// As with any HTTP query string parameter, the whole filter expression must
    /// be URL-encoded.
    /// 
    /// Example REST request:
    /// `GET
    /// /v1beta1/beacons?q=status:active%20lat:51.123%20lng:-1.095%20radius:1000`
    ///
    /// Sets the *q* query property to the given value.
    pub fn q(mut self, new_value: &str) -> BeaconListCall<'a> {
        self._q = Some(new_value.to_string());
        self
    }
    /// The project id to list beacons under. If not present then the project
    /// credential that made the request is used as the project.
    /// Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> BeaconListCall<'a> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// A pagination token obtained from a previous request to list beacons.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> BeaconListCall<'a> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of records to return for this request, up to a
    /// server-defined upper limit.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> BeaconListCall<'a> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconListCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconListCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::UserlocationBeaconRegistry`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BeaconListCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Registers a previously unregistered beacon given its `advertisedId`.
/// These IDs are unique within the system. An ID can be registered only once.
/// 
/// Authenticate using an [OAuth access
/// token](https://developers.google.com/identity/protocols/OAuth2) from a
/// signed-in user with **Is owner** or **Can edit** permissions in the Google
/// Developers Console project.
///
/// A builder for the *register* method supported by a *beacon* resource.
/// It is not used directly, but through a `BeaconMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// use proximitybeacon1_beta1::api::Beacon;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use proximitybeacon1_beta1::Proximitybeacon;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Beacon::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.beacons().register(req)
///              .project_id("et")
///              .doit().await;
/// # }
/// ```
pub struct BeaconRegisterCall<'a>
    where  {

    hub: &'a Proximitybeacon<>,
    _request: Beacon,
    _project_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BeaconRegisterCall<'a> {}

impl<'a> BeaconRegisterCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Beacon)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beacons.register",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        if let Some(value) = self._project_id {
            params.push(("projectId", value.to_string()));
        }
        for &field in ["alt", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/beacons:register";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string(), ());
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Beacon) -> BeaconRegisterCall<'a> {
        self._request = new_value;
        self
    }
    /// The project id of the project the beacon will be registered to. If
    /// the project id is not specified then the project making the request
    /// is used.
    /// Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> BeaconRegisterCall<'a> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconRegisterCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconRegisterCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::UserlocationBeaconRegistry`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BeaconRegisterCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates the information about the specified beacon. **Any field that you do
/// not populate in the submitted beacon will be permanently erased**, so you
/// should follow the "read, modify, write" pattern to avoid inadvertently
/// destroying data.
/// 
/// Changes to the beacon status via this method will be  silently ignored.
/// To update beacon status, use the separate methods on this API for
/// activation, deactivation, and decommissioning.
/// Authenticate using an [OAuth access
/// token](https://developers.google.com/identity/protocols/OAuth2) from a
/// signed-in user with **Is owner** or **Can edit** permissions in the Google
/// Developers Console project.
///
/// A builder for the *update* method supported by a *beacon* resource.
/// It is not used directly, but through a `BeaconMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// use proximitybeacon1_beta1::api::Beacon;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use proximitybeacon1_beta1::Proximitybeacon;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Beacon::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.beacons().update(req, "beaconName")
///              .project_id("et")
///              .doit().await;
/// # }
/// ```
pub struct BeaconUpdateCall<'a>
    where  {

    hub: &'a Proximitybeacon<>,
    _request: Beacon,
    _beacon_name: String,
    _project_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BeaconUpdateCall<'a> {}

impl<'a> BeaconUpdateCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Beacon)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beacons.update",
                               http_method: hyper::Method::PUT });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("beaconName", self._beacon_name.to_string()));
        if let Some(value) = self._project_id {
            params.push(("projectId", value.to_string()));
        }
        for &field in ["alt", "beaconName", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/{+beaconName}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+beaconName}", "beaconName")].iter() {
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
            for param_name in ["beaconName"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PUT).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Beacon) -> BeaconUpdateCall<'a> {
        self._request = new_value;
        self
    }
    /// Resource name of this beacon. A beacon name has the format
    /// "beacons/N!beaconId" where the beaconId is the base16 ID broadcast by
    /// the beacon and N is a code for the beacon's type. Possible values are
    /// `3` for Eddystone, `1` for iBeacon, or `5` for AltBeacon.
    /// 
    /// This field must be left empty when registering. After reading a beacon,
    /// clients can use the name for future operations.
    ///
    /// Sets the *beacon name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn beacon_name(mut self, new_value: &str) -> BeaconUpdateCall<'a> {
        self._beacon_name = new_value.to_string();
        self
    }
    /// The project id of the beacon to update. If the project id is not
    /// specified then the project making the request is used. The project id
    /// must match the project that owns the beacon.
    /// Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> BeaconUpdateCall<'a> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconUpdateCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconUpdateCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::UserlocationBeaconRegistry`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BeaconUpdateCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists all attachment namespaces owned by your Google Developers Console
/// project. Attachment data associated with a beacon must include a
/// namespaced type, and the namespace must be owned by your project.
/// 
/// Authenticate using an [OAuth access
/// token](https://developers.google.com/identity/protocols/OAuth2) from a
/// signed-in user with **viewer**, **Is owner** or **Can edit** permissions in
/// the Google Developers Console project.
///
/// A builder for the *list* method supported by a *namespace* resource.
/// It is not used directly, but through a `NamespaceMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use proximitybeacon1_beta1::Proximitybeacon;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.namespaces().list()
///              .project_id("et")
///              .doit().await;
/// # }
/// ```
pub struct NamespaceListCall<'a>
    where  {

    hub: &'a Proximitybeacon<>,
    _project_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for NamespaceListCall<'a> {}

impl<'a> NamespaceListCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListNamespacesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "proximitybeacon.namespaces.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        if let Some(value) = self._project_id {
            params.push(("projectId", value.to_string()));
        }
        for &field in ["alt", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/namespaces";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string(), ());
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// The project id to list namespaces under.
    /// Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> NamespaceListCall<'a> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> NamespaceListCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> NamespaceListCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::UserlocationBeaconRegistry`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> NamespaceListCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates the information about the specified namespace. Only the namespace
/// visibility can be updated.
///
/// A builder for the *update* method supported by a *namespace* resource.
/// It is not used directly, but through a `NamespaceMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// use proximitybeacon1_beta1::api::Namespace;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use proximitybeacon1_beta1::Proximitybeacon;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Namespace::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.namespaces().update(req, "namespaceName")
///              .project_id("erat")
///              .doit().await;
/// # }
/// ```
pub struct NamespaceUpdateCall<'a>
    where  {

    hub: &'a Proximitybeacon<>,
    _request: Namespace,
    _namespace_name: String,
    _project_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for NamespaceUpdateCall<'a> {}

impl<'a> NamespaceUpdateCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Namespace)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "proximitybeacon.namespaces.update",
                               http_method: hyper::Method::PUT });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("namespaceName", self._namespace_name.to_string()));
        if let Some(value) = self._project_id {
            params.push(("projectId", value.to_string()));
        }
        for &field in ["alt", "namespaceName", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/{+namespaceName}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+namespaceName}", "namespaceName")].iter() {
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
            for param_name in ["namespaceName"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PUT).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Namespace) -> NamespaceUpdateCall<'a> {
        self._request = new_value;
        self
    }
    /// Resource name of this namespace. Namespaces names have the format:
    /// <code>namespaces/<var>namespace</var></code>.
    ///
    /// Sets the *namespace name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn namespace_name(mut self, new_value: &str) -> NamespaceUpdateCall<'a> {
        self._namespace_name = new_value.to_string();
        self
    }
    /// The project id of the namespace to update. If the project id is not
    /// specified then the project making the request is used. The project id
    /// must match the project that owns the beacon.
    /// Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> NamespaceUpdateCall<'a> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> NamespaceUpdateCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> NamespaceUpdateCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::UserlocationBeaconRegistry`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> NamespaceUpdateCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets the Proximity Beacon API's current public key and associated
/// parameters used to initiate the Diffie-Hellman key exchange required to
/// register a beacon that broadcasts the Eddystone-EID format. This key
/// changes periodically; clients may cache it and re-use the same public key
/// to provision and register multiple beacons. However, clients should be
/// prepared to refresh this key when they encounter an error registering an
/// Eddystone-EID beacon.
///
/// A builder for the *getEidparams* method.
/// It is not used directly, but through a `MethodMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use proximitybeacon1_beta1::Proximitybeacon;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().get_eidparams()
///              .doit().await;
/// # }
/// ```
pub struct MethodGetEidparamCall<'a>
    where  {

    hub: &'a Proximitybeacon<>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for MethodGetEidparamCall<'a> {}

impl<'a> MethodGetEidparamCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, EphemeralIdRegistrationParams)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "proximitybeacon.getEidparams",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(2 + self._additional_params.len());
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/eidparams";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string(), ());
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MethodGetEidparamCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> MethodGetEidparamCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::UserlocationBeaconRegistry`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> MethodGetEidparamCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


