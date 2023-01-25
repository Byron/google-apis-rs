use std::collections::HashMap;
use std::cell::RefCell;
use std::default::Default;
use std::collections::BTreeSet;
use std::error::Error as StdError;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;

use hyper::client::connect;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::time::sleep;
use tower_service;
use serde::{Serialize, Deserialize};

use crate::{client, client::GetToken, client::serde_with};

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
/// extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// use proximitybeacon1_beta1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use proximitybeacon1_beta1::{Proximitybeacon, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
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
#[derive(Clone)]
pub struct Proximitybeacon<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, S> client::Hub for Proximitybeacon<S> {}

impl<'a, S> Proximitybeacon<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> Proximitybeacon<S> {
        Proximitybeacon {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.2-beta-1".to_string(),
            _base_url: "https://proximitybeacon.googleapis.com/".to_string(),
            _root_url: "https://proximitybeacon.googleapis.com/".to_string(),
        }
    }

    pub fn beaconinfo(&'a self) -> BeaconinfoMethods<'a, S> {
        BeaconinfoMethods { hub: &self }
    }
    pub fn beacons(&'a self) -> BeaconMethods<'a, S> {
        BeaconMethods { hub: &self }
    }
    pub fn methods(&'a self) -> MethodMethods<'a, S> {
        MethodMethods { hub: &self }
    }
    pub fn namespaces(&'a self) -> NamespaceMethods<'a, S> {
        NamespaceMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.2-beta-1`.
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
/// 
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
/// 
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
/// 
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
/// 
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
/// a mobile client has encountered "in the wild".
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [getforobserved beaconinfo](BeaconinfoGetforobservedCall) (request)
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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


/// Response to ListNamespacesRequest that contains all the project's namespaces.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list namespaces](NamespaceListCall) (response)
/// 
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
/// 
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



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *beaconinfo* resources.
/// It is not used directly, but through the [`Proximitybeacon`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use proximitybeacon1_beta1::{Proximitybeacon, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `getforobserved(...)`
/// // to build up your call.
/// let rb = hub.beaconinfo();
/// # }
/// ```
pub struct BeaconinfoMethods<'a, S>
    where S: 'a {

    hub: &'a Proximitybeacon<S>,
}

impl<'a, S> client::MethodsBuilder for BeaconinfoMethods<'a, S> {}

impl<'a, S> BeaconinfoMethods<'a, S> {
    
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
    pub fn getforobserved(&self, request: GetInfoForObservedBeaconsRequest) -> BeaconinfoGetforobservedCall<'a, S> {
        BeaconinfoGetforobservedCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *beacon* resources.
/// It is not used directly, but through the [`Proximitybeacon`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use proximitybeacon1_beta1::{Proximitybeacon, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `activate(...)`, `attachments_batch_delete(...)`, `attachments_create(...)`, `attachments_delete(...)`, `attachments_list(...)`, `deactivate(...)`, `decommission(...)`, `delete(...)`, `diagnostics_list(...)`, `get(...)`, `list(...)`, `register(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.beacons();
/// # }
/// ```
pub struct BeaconMethods<'a, S>
    where S: 'a {

    hub: &'a Proximitybeacon<S>,
}

impl<'a, S> client::MethodsBuilder for BeaconMethods<'a, S> {}

impl<'a, S> BeaconMethods<'a, S> {
    
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
    pub fn attachments_batch_delete(&self, beacon_name: &str) -> BeaconAttachmentBatchDeleteCall<'a, S> {
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
    pub fn attachments_create(&self, request: BeaconAttachment, beacon_name: &str) -> BeaconAttachmentCreateCall<'a, S> {
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
    pub fn attachments_delete(&self, attachment_name: &str) -> BeaconAttachmentDeleteCall<'a, S> {
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
    pub fn attachments_list(&self, beacon_name: &str) -> BeaconAttachmentListCall<'a, S> {
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
    pub fn diagnostics_list(&self, beacon_name: &str) -> BeaconDiagnosticListCall<'a, S> {
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
    pub fn activate(&self, beacon_name: &str) -> BeaconActivateCall<'a, S> {
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
    pub fn deactivate(&self, beacon_name: &str) -> BeaconDeactivateCall<'a, S> {
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
    pub fn decommission(&self, beacon_name: &str) -> BeaconDecommissionCall<'a, S> {
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
    pub fn delete(&self, beacon_name: &str) -> BeaconDeleteCall<'a, S> {
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
    pub fn get(&self, beacon_name: &str) -> BeaconGetCall<'a, S> {
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
    pub fn list(&self) -> BeaconListCall<'a, S> {
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
    pub fn register(&self, request: Beacon) -> BeaconRegisterCall<'a, S> {
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
    pub fn update(&self, request: Beacon, beacon_name: &str) -> BeaconUpdateCall<'a, S> {
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
/// It is not used directly, but through the [`Proximitybeacon`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use proximitybeacon1_beta1::{Proximitybeacon, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.namespaces();
/// # }
/// ```
pub struct NamespaceMethods<'a, S>
    where S: 'a {

    hub: &'a Proximitybeacon<S>,
}

impl<'a, S> client::MethodsBuilder for NamespaceMethods<'a, S> {}

impl<'a, S> NamespaceMethods<'a, S> {
    
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
    pub fn list(&self) -> NamespaceListCall<'a, S> {
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
    pub fn update(&self, request: Namespace, namespace_name: &str) -> NamespaceUpdateCall<'a, S> {
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
/// It is not used directly, but through the [`Proximitybeacon`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use proximitybeacon1_beta1::{Proximitybeacon, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_eidparams(...)`
/// // to build up your call.
/// let rb = hub.methods();
/// # }
/// ```
pub struct MethodMethods<'a, S>
    where S: 'a {

    hub: &'a Proximitybeacon<S>,
}

impl<'a, S> client::MethodsBuilder for MethodMethods<'a, S> {}

impl<'a, S> MethodMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the Proximity Beacon API's current public key and associated
    /// parameters used to initiate the Diffie-Hellman key exchange required to
    /// register a beacon that broadcasts the Eddystone-EID format. This key
    /// changes periodically; clients may cache it and re-use the same public key
    /// to provision and register multiple beacons. However, clients should be
    /// prepared to refresh this key when they encounter an error registering an
    /// Eddystone-EID beacon.
    pub fn get_eidparams(&self) -> MethodGetEidparamCall<'a, S> {
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
/// It is not used directly, but through a [`BeaconinfoMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// use proximitybeacon1_beta1::api::GetInfoForObservedBeaconsRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use proximitybeacon1_beta1::{Proximitybeacon, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
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
pub struct BeaconinfoGetforobservedCall<'a, S>
    where S: 'a {

    hub: &'a Proximitybeacon<S>,
    _request: GetInfoForObservedBeaconsRequest,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for BeaconinfoGetforobservedCall<'a, S> {}

impl<'a, S> BeaconinfoGetforobservedCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GetInfoForObservedBeaconsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beaconinfo.getforobserved",
                               http_method: hyper::Method::POST });

        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1beta1/beaconinfo:getforobserved";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }


        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
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
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());



                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
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
                            sleep(d).await;
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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: GetInfoForObservedBeaconsRequest) -> BeaconinfoGetforobservedCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconinfoGetforobservedCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconinfoGetforobservedCall<'a, S>
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
/// It is not used directly, but through a [`BeaconMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use proximitybeacon1_beta1::{Proximitybeacon, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.beacons().attachments_batch_delete("beaconName")
///              .project_id("eos")
///              .namespaced_type("dolor")
///              .doit().await;
/// # }
/// ```
pub struct BeaconAttachmentBatchDeleteCall<'a, S>
    where S: 'a {

    hub: &'a Proximitybeacon<S>,
    _beacon_name: String,
    _project_id: Option<String>,
    _namespaced_type: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for BeaconAttachmentBatchDeleteCall<'a, S> {}

impl<'a, S> BeaconAttachmentBatchDeleteCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, DeleteAttachmentsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beacons.attachments.batchDelete",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "beaconName", "projectId", "namespacedType"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("beaconName", self._beacon_name);
        if let Some(value) = self._project_id.as_ref() {
            params.push("projectId", value);
        }
        if let Some(value) = self._namespaced_type.as_ref() {
            params.push("namespacedType", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1beta1/{+beaconName}/attachments:batchDelete";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+beaconName}", "beaconName")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["beaconName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
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
                            sleep(d).await;
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
    pub fn beacon_name(mut self, new_value: &str) -> BeaconAttachmentBatchDeleteCall<'a, S> {
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
    pub fn project_id(mut self, new_value: &str) -> BeaconAttachmentBatchDeleteCall<'a, S> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// Specifies the namespace and type of attachments to delete in
    /// `namespace/type` format. Accepts `*/*` to specify
    /// "all types in all namespaces".
    /// Optional.
    ///
    /// Sets the *namespaced type* query property to the given value.
    pub fn namespaced_type(mut self, new_value: &str) -> BeaconAttachmentBatchDeleteCall<'a, S> {
        self._namespaced_type = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconAttachmentBatchDeleteCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconAttachmentBatchDeleteCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::UserlocationBeaconRegistry`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> BeaconAttachmentBatchDeleteCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> BeaconAttachmentBatchDeleteCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> BeaconAttachmentBatchDeleteCall<'a, S> {
        self._scopes.clear();
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
/// It is not used directly, but through a [`BeaconMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// use proximitybeacon1_beta1::api::BeaconAttachment;
/// # async fn dox() {
/// # use std::default::Default;
/// # use proximitybeacon1_beta1::{Proximitybeacon, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
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
pub struct BeaconAttachmentCreateCall<'a, S>
    where S: 'a {

    hub: &'a Proximitybeacon<S>,
    _request: BeaconAttachment,
    _beacon_name: String,
    _project_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for BeaconAttachmentCreateCall<'a, S> {}

impl<'a, S> BeaconAttachmentCreateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, BeaconAttachment)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beacons.attachments.create",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "beaconName", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("beaconName", self._beacon_name);
        if let Some(value) = self._project_id.as_ref() {
            params.push("projectId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1beta1/{+beaconName}/attachments";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+beaconName}", "beaconName")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["beaconName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
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
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
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
                            sleep(d).await;
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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: BeaconAttachment) -> BeaconAttachmentCreateCall<'a, S> {
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
    pub fn beacon_name(mut self, new_value: &str) -> BeaconAttachmentCreateCall<'a, S> {
        self._beacon_name = new_value.to_string();
        self
    }
    /// The project id of the project the attachment will belong to. If
    /// the project id is not specified then the project making the request
    /// is used.
    /// Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> BeaconAttachmentCreateCall<'a, S> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconAttachmentCreateCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconAttachmentCreateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::UserlocationBeaconRegistry`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> BeaconAttachmentCreateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> BeaconAttachmentCreateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> BeaconAttachmentCreateCall<'a, S> {
        self._scopes.clear();
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
/// It is not used directly, but through a [`BeaconMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use proximitybeacon1_beta1::{Proximitybeacon, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.beacons().attachments_delete("attachmentName")
///              .project_id("amet")
///              .doit().await;
/// # }
/// ```
pub struct BeaconAttachmentDeleteCall<'a, S>
    where S: 'a {

    hub: &'a Proximitybeacon<S>,
    _attachment_name: String,
    _project_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for BeaconAttachmentDeleteCall<'a, S> {}

impl<'a, S> BeaconAttachmentDeleteCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beacons.attachments.delete",
                               http_method: hyper::Method::DELETE });

        for &field in ["alt", "attachmentName", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("attachmentName", self._attachment_name);
        if let Some(value) = self._project_id.as_ref() {
            params.push("projectId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1beta1/{+attachmentName}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+attachmentName}", "attachmentName")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["attachmentName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::DELETE)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
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
                            sleep(d).await;
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
    pub fn attachment_name(mut self, new_value: &str) -> BeaconAttachmentDeleteCall<'a, S> {
        self._attachment_name = new_value.to_string();
        self
    }
    /// The project id of the attachment to delete. If not provided, the project
    /// that is making the request is used.
    /// Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> BeaconAttachmentDeleteCall<'a, S> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconAttachmentDeleteCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconAttachmentDeleteCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::UserlocationBeaconRegistry`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> BeaconAttachmentDeleteCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> BeaconAttachmentDeleteCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> BeaconAttachmentDeleteCall<'a, S> {
        self._scopes.clear();
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
/// It is not used directly, but through a [`BeaconMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use proximitybeacon1_beta1::{Proximitybeacon, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.beacons().attachments_list("beaconName")
///              .project_id("ipsum")
///              .namespaced_type("sed")
///              .doit().await;
/// # }
/// ```
pub struct BeaconAttachmentListCall<'a, S>
    where S: 'a {

    hub: &'a Proximitybeacon<S>,
    _beacon_name: String,
    _project_id: Option<String>,
    _namespaced_type: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for BeaconAttachmentListCall<'a, S> {}

impl<'a, S> BeaconAttachmentListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListBeaconAttachmentsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beacons.attachments.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "beaconName", "projectId", "namespacedType"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("beaconName", self._beacon_name);
        if let Some(value) = self._project_id.as_ref() {
            params.push("projectId", value);
        }
        if let Some(value) = self._namespaced_type.as_ref() {
            params.push("namespacedType", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1beta1/{+beaconName}/attachments";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+beaconName}", "beaconName")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["beaconName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
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
                            sleep(d).await;
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
    pub fn beacon_name(mut self, new_value: &str) -> BeaconAttachmentListCall<'a, S> {
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
    pub fn project_id(mut self, new_value: &str) -> BeaconAttachmentListCall<'a, S> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// Specifies the namespace and type of attachment to include in response in
    /// <var>namespace/type</var> format. Accepts `*/*` to specify
    /// "all types in all namespaces".
    ///
    /// Sets the *namespaced type* query property to the given value.
    pub fn namespaced_type(mut self, new_value: &str) -> BeaconAttachmentListCall<'a, S> {
        self._namespaced_type = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconAttachmentListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconAttachmentListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::UserlocationBeaconRegistry`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> BeaconAttachmentListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> BeaconAttachmentListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> BeaconAttachmentListCall<'a, S> {
        self._scopes.clear();
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
/// It is not used directly, but through a [`BeaconMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use proximitybeacon1_beta1::{Proximitybeacon, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
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
pub struct BeaconDiagnosticListCall<'a, S>
    where S: 'a {

    hub: &'a Proximitybeacon<S>,
    _beacon_name: String,
    _project_id: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _alert_filter: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for BeaconDiagnosticListCall<'a, S> {}

impl<'a, S> BeaconDiagnosticListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListDiagnosticsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beacons.diagnostics.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "beaconName", "projectId", "pageToken", "pageSize", "alertFilter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("beaconName", self._beacon_name);
        if let Some(value) = self._project_id.as_ref() {
            params.push("projectId", value);
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._alert_filter.as_ref() {
            params.push("alertFilter", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1beta1/{+beaconName}/diagnostics";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+beaconName}", "beaconName")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["beaconName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
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
                            sleep(d).await;
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


    /// Beacon that the diagnostics are for.
    ///
    /// Sets the *beacon name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn beacon_name(mut self, new_value: &str) -> BeaconDiagnosticListCall<'a, S> {
        self._beacon_name = new_value.to_string();
        self
    }
    /// Requests only diagnostic records for the given project id. If not set,
    /// then the project making the request will be used for looking up
    /// diagnostic records. Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> BeaconDiagnosticListCall<'a, S> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// Requests results that occur after the `page_token`, obtained from the
    /// response to a previous request. Optional.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> BeaconDiagnosticListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Specifies the maximum number of results to return. Defaults to
    /// 10. Maximum 1000. Optional.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> BeaconDiagnosticListCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// Requests only beacons that have the given alert. For example, to find
    /// beacons that have low batteries use `alert_filter=LOW_BATTERY`.
    ///
    /// Sets the *alert filter* query property to the given value.
    pub fn alert_filter(mut self, new_value: &str) -> BeaconDiagnosticListCall<'a, S> {
        self._alert_filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconDiagnosticListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconDiagnosticListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::UserlocationBeaconRegistry`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> BeaconDiagnosticListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> BeaconDiagnosticListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> BeaconDiagnosticListCall<'a, S> {
        self._scopes.clear();
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
/// It is not used directly, but through a [`BeaconMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use proximitybeacon1_beta1::{Proximitybeacon, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.beacons().activate("beaconName")
///              .project_id("est")
///              .doit().await;
/// # }
/// ```
pub struct BeaconActivateCall<'a, S>
    where S: 'a {

    hub: &'a Proximitybeacon<S>,
    _beacon_name: String,
    _project_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for BeaconActivateCall<'a, S> {}

impl<'a, S> BeaconActivateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beacons.activate",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "beaconName", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("beaconName", self._beacon_name);
        if let Some(value) = self._project_id.as_ref() {
            params.push("projectId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1beta1/{+beaconName}:activate";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+beaconName}", "beaconName")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["beaconName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
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
                            sleep(d).await;
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
    pub fn beacon_name(mut self, new_value: &str) -> BeaconActivateCall<'a, S> {
        self._beacon_name = new_value.to_string();
        self
    }
    /// The project id of the beacon to activate. If the project id is not
    /// specified then the project making the request is used. The project id
    /// must match the project that owns the beacon.
    /// Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> BeaconActivateCall<'a, S> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconActivateCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconActivateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::UserlocationBeaconRegistry`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> BeaconActivateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> BeaconActivateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> BeaconActivateCall<'a, S> {
        self._scopes.clear();
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
/// It is not used directly, but through a [`BeaconMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use proximitybeacon1_beta1::{Proximitybeacon, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.beacons().deactivate("beaconName")
///              .project_id("ea")
///              .doit().await;
/// # }
/// ```
pub struct BeaconDeactivateCall<'a, S>
    where S: 'a {

    hub: &'a Proximitybeacon<S>,
    _beacon_name: String,
    _project_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for BeaconDeactivateCall<'a, S> {}

impl<'a, S> BeaconDeactivateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beacons.deactivate",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "beaconName", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("beaconName", self._beacon_name);
        if let Some(value) = self._project_id.as_ref() {
            params.push("projectId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1beta1/{+beaconName}:deactivate";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+beaconName}", "beaconName")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["beaconName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
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
                            sleep(d).await;
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
    pub fn beacon_name(mut self, new_value: &str) -> BeaconDeactivateCall<'a, S> {
        self._beacon_name = new_value.to_string();
        self
    }
    /// The project id of the beacon to deactivate. If the project id is not
    /// specified then the project making the request is used. The project id must
    /// match the project that owns the beacon.
    /// Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> BeaconDeactivateCall<'a, S> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconDeactivateCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconDeactivateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::UserlocationBeaconRegistry`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> BeaconDeactivateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> BeaconDeactivateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> BeaconDeactivateCall<'a, S> {
        self._scopes.clear();
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
/// It is not used directly, but through a [`BeaconMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use proximitybeacon1_beta1::{Proximitybeacon, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.beacons().decommission("beaconName")
///              .project_id("Lorem")
///              .doit().await;
/// # }
/// ```
pub struct BeaconDecommissionCall<'a, S>
    where S: 'a {

    hub: &'a Proximitybeacon<S>,
    _beacon_name: String,
    _project_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for BeaconDecommissionCall<'a, S> {}

impl<'a, S> BeaconDecommissionCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beacons.decommission",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "beaconName", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("beaconName", self._beacon_name);
        if let Some(value) = self._project_id.as_ref() {
            params.push("projectId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1beta1/{+beaconName}:decommission";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+beaconName}", "beaconName")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["beaconName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
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
                            sleep(d).await;
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
    pub fn beacon_name(mut self, new_value: &str) -> BeaconDecommissionCall<'a, S> {
        self._beacon_name = new_value.to_string();
        self
    }
    /// The project id of the beacon to decommission. If the project id is not
    /// specified then the project making the request is used. The project id
    /// must match the project that owns the beacon.
    /// Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> BeaconDecommissionCall<'a, S> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconDecommissionCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconDecommissionCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::UserlocationBeaconRegistry`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> BeaconDecommissionCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> BeaconDecommissionCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> BeaconDecommissionCall<'a, S> {
        self._scopes.clear();
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
/// It is not used directly, but through a [`BeaconMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use proximitybeacon1_beta1::{Proximitybeacon, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.beacons().delete("beaconName")
///              .project_id("labore")
///              .doit().await;
/// # }
/// ```
pub struct BeaconDeleteCall<'a, S>
    where S: 'a {

    hub: &'a Proximitybeacon<S>,
    _beacon_name: String,
    _project_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for BeaconDeleteCall<'a, S> {}

impl<'a, S> BeaconDeleteCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beacons.delete",
                               http_method: hyper::Method::DELETE });

        for &field in ["alt", "beaconName", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("beaconName", self._beacon_name);
        if let Some(value) = self._project_id.as_ref() {
            params.push("projectId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1beta1/{+beaconName}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+beaconName}", "beaconName")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["beaconName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::DELETE)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
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
                            sleep(d).await;
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
    pub fn beacon_name(mut self, new_value: &str) -> BeaconDeleteCall<'a, S> {
        self._beacon_name = new_value.to_string();
        self
    }
    /// The project id of the beacon to delete. If not provided, the project
    /// that is making the request is used.
    /// Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> BeaconDeleteCall<'a, S> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconDeleteCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconDeleteCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::UserlocationBeaconRegistry`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> BeaconDeleteCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> BeaconDeleteCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> BeaconDeleteCall<'a, S> {
        self._scopes.clear();
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
/// It is not used directly, but through a [`BeaconMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use proximitybeacon1_beta1::{Proximitybeacon, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.beacons().get("beaconName")
///              .project_id("duo")
///              .doit().await;
/// # }
/// ```
pub struct BeaconGetCall<'a, S>
    where S: 'a {

    hub: &'a Proximitybeacon<S>,
    _beacon_name: String,
    _project_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for BeaconGetCall<'a, S> {}

impl<'a, S> BeaconGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Beacon)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beacons.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "beaconName", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("beaconName", self._beacon_name);
        if let Some(value) = self._project_id.as_ref() {
            params.push("projectId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1beta1/{+beaconName}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+beaconName}", "beaconName")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["beaconName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
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
                            sleep(d).await;
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
    pub fn beacon_name(mut self, new_value: &str) -> BeaconGetCall<'a, S> {
        self._beacon_name = new_value.to_string();
        self
    }
    /// The project id of the beacon to request. If the project id is not specified
    /// then the project making the request is used. The project id must match the
    /// project that owns the beacon.
    /// Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> BeaconGetCall<'a, S> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconGetCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::UserlocationBeaconRegistry`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> BeaconGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> BeaconGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> BeaconGetCall<'a, S> {
        self._scopes.clear();
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
/// It is not used directly, but through a [`BeaconMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use proximitybeacon1_beta1::{Proximitybeacon, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
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
pub struct BeaconListCall<'a, S>
    where S: 'a {

    hub: &'a Proximitybeacon<S>,
    _q: Option<String>,
    _project_id: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for BeaconListCall<'a, S> {}

impl<'a, S> BeaconListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListBeaconsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beacons.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "q", "projectId", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        if let Some(value) = self._q.as_ref() {
            params.push("q", value);
        }
        if let Some(value) = self._project_id.as_ref() {
            params.push("projectId", value);
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1beta1/beacons";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
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
                            sleep(d).await;
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
    pub fn q(mut self, new_value: &str) -> BeaconListCall<'a, S> {
        self._q = Some(new_value.to_string());
        self
    }
    /// The project id to list beacons under. If not present then the project
    /// credential that made the request is used as the project.
    /// Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> BeaconListCall<'a, S> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// A pagination token obtained from a previous request to list beacons.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> BeaconListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of records to return for this request, up to a
    /// server-defined upper limit.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> BeaconListCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::UserlocationBeaconRegistry`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> BeaconListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> BeaconListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> BeaconListCall<'a, S> {
        self._scopes.clear();
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
/// It is not used directly, but through a [`BeaconMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// use proximitybeacon1_beta1::api::Beacon;
/// # async fn dox() {
/// # use std::default::Default;
/// # use proximitybeacon1_beta1::{Proximitybeacon, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
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
pub struct BeaconRegisterCall<'a, S>
    where S: 'a {

    hub: &'a Proximitybeacon<S>,
    _request: Beacon,
    _project_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for BeaconRegisterCall<'a, S> {}

impl<'a, S> BeaconRegisterCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Beacon)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beacons.register",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        if let Some(value) = self._project_id.as_ref() {
            params.push("projectId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1beta1/beacons:register";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
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
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
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
                            sleep(d).await;
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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Beacon) -> BeaconRegisterCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The project id of the project the beacon will be registered to. If
    /// the project id is not specified then the project making the request
    /// is used.
    /// Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> BeaconRegisterCall<'a, S> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconRegisterCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconRegisterCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::UserlocationBeaconRegistry`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> BeaconRegisterCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> BeaconRegisterCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> BeaconRegisterCall<'a, S> {
        self._scopes.clear();
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
/// It is not used directly, but through a [`BeaconMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// use proximitybeacon1_beta1::api::Beacon;
/// # async fn dox() {
/// # use std::default::Default;
/// # use proximitybeacon1_beta1::{Proximitybeacon, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
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
pub struct BeaconUpdateCall<'a, S>
    where S: 'a {

    hub: &'a Proximitybeacon<S>,
    _request: Beacon,
    _beacon_name: String,
    _project_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for BeaconUpdateCall<'a, S> {}

impl<'a, S> BeaconUpdateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Beacon)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "proximitybeacon.beacons.update",
                               http_method: hyper::Method::PUT });

        for &field in ["alt", "beaconName", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("beaconName", self._beacon_name);
        if let Some(value) = self._project_id.as_ref() {
            params.push("projectId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1beta1/{+beaconName}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+beaconName}", "beaconName")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["beaconName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
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
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::PUT)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
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
                            sleep(d).await;
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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Beacon) -> BeaconUpdateCall<'a, S> {
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
    pub fn beacon_name(mut self, new_value: &str) -> BeaconUpdateCall<'a, S> {
        self._beacon_name = new_value.to_string();
        self
    }
    /// The project id of the beacon to update. If the project id is not
    /// specified then the project making the request is used. The project id
    /// must match the project that owns the beacon.
    /// Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> BeaconUpdateCall<'a, S> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BeaconUpdateCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BeaconUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::UserlocationBeaconRegistry`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> BeaconUpdateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> BeaconUpdateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> BeaconUpdateCall<'a, S> {
        self._scopes.clear();
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
/// It is not used directly, but through a [`NamespaceMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use proximitybeacon1_beta1::{Proximitybeacon, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.namespaces().list()
///              .project_id("et")
///              .doit().await;
/// # }
/// ```
pub struct NamespaceListCall<'a, S>
    where S: 'a {

    hub: &'a Proximitybeacon<S>,
    _project_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for NamespaceListCall<'a, S> {}

impl<'a, S> NamespaceListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListNamespacesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "proximitybeacon.namespaces.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        if let Some(value) = self._project_id.as_ref() {
            params.push("projectId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1beta1/namespaces";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
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
                            sleep(d).await;
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


    /// The project id to list namespaces under.
    /// Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> NamespaceListCall<'a, S> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> NamespaceListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> NamespaceListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::UserlocationBeaconRegistry`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> NamespaceListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> NamespaceListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> NamespaceListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Updates the information about the specified namespace. Only the namespace
/// visibility can be updated.
///
/// A builder for the *update* method supported by a *namespace* resource.
/// It is not used directly, but through a [`NamespaceMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// use proximitybeacon1_beta1::api::Namespace;
/// # async fn dox() {
/// # use std::default::Default;
/// # use proximitybeacon1_beta1::{Proximitybeacon, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
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
pub struct NamespaceUpdateCall<'a, S>
    where S: 'a {

    hub: &'a Proximitybeacon<S>,
    _request: Namespace,
    _namespace_name: String,
    _project_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for NamespaceUpdateCall<'a, S> {}

impl<'a, S> NamespaceUpdateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Namespace)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "proximitybeacon.namespaces.update",
                               http_method: hyper::Method::PUT });

        for &field in ["alt", "namespaceName", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("namespaceName", self._namespace_name);
        if let Some(value) = self._project_id.as_ref() {
            params.push("projectId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1beta1/{+namespaceName}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+namespaceName}", "namespaceName")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["namespaceName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
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
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::PUT)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
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
                            sleep(d).await;
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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Namespace) -> NamespaceUpdateCall<'a, S> {
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
    pub fn namespace_name(mut self, new_value: &str) -> NamespaceUpdateCall<'a, S> {
        self._namespace_name = new_value.to_string();
        self
    }
    /// The project id of the namespace to update. If the project id is not
    /// specified then the project making the request is used. The project id
    /// must match the project that owns the beacon.
    /// Optional.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> NamespaceUpdateCall<'a, S> {
        self._project_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> NamespaceUpdateCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> NamespaceUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::UserlocationBeaconRegistry`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> NamespaceUpdateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> NamespaceUpdateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> NamespaceUpdateCall<'a, S> {
        self._scopes.clear();
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
/// It is not used directly, but through a [`MethodMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use proximitybeacon1_beta1::{Proximitybeacon, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().get_eidparams()
///              .doit().await;
/// # }
/// ```
pub struct MethodGetEidparamCall<'a, S>
    where S: 'a {

    hub: &'a Proximitybeacon<S>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for MethodGetEidparamCall<'a, S> {}

impl<'a, S> MethodGetEidparamCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, EphemeralIdRegistrationParams)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "proximitybeacon.getEidparams",
                               http_method: hyper::Method::GET });

        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(2 + self._additional_params.len());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1beta1/eidparams";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::UserlocationBeaconRegistry.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
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
                            sleep(d).await;
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


    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MethodGetEidparamCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> MethodGetEidparamCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::UserlocationBeaconRegistry`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MethodGetEidparamCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MethodGetEidparamCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MethodGetEidparamCall<'a, S> {
        self._scopes.clear();
        self
    }
}


