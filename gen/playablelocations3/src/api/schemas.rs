use super::*;
/// Encapsulates impression event details.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3Impression {
    /// An arbitrary, developer-defined type identifier for each type of game
    /// object used in your game.
    /// 
    /// Since players interact with differ types of game objects in different ways,
    /// this field allows you to segregate impression data by type for analysis.
    /// 
    /// You should assign a unique `game_object_type` ID to represent a distinct
    /// type of game object in your game.
    /// 
    /// For example, 1=monster location, 2=powerup location.
    #[serde(rename="gameObjectType")]
    
    pub game_object_type: Option<i32>,
    /// Required. The type of impression event.
    #[serde(rename="impressionType")]
    
    pub impression_type: Option<GoogleMapsPlayablelocationsV3ImpressionImpressionTypeEnum>,
    /// Required. The name of the playable location.
    #[serde(rename="locationName")]
    
    pub location_name: Option<String>,
}

impl client::Part for GoogleMapsPlayablelocationsV3Impression {}


/// A request for logging impressions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [log impressions](MethodLogImpressionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3LogImpressionsRequest {
    /// Required. Information about the client device. For example, device model and
    /// operating system.
    #[serde(rename="clientInfo")]
    
    pub client_info: Option<GoogleMapsUnityClientInfo>,
    /// Required. Impression event details. The maximum number of impression reports that you
    /// can log at once is 50.
    
    pub impressions: Option<Vec<GoogleMapsPlayablelocationsV3Impression>>,
    /// Required. A string that uniquely identifies the log impressions request. This allows
    /// you to detect duplicate requests. We recommend that you use UUIDs for this
    /// value. The value must not exceed 50 characters.
    /// 
    /// You should reuse the `request_id` only when retrying a request in case of
    /// failure. In this case, the request must be identical to the one that
    /// failed.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for GoogleMapsPlayablelocationsV3LogImpressionsRequest {}


/// A response for the LogImpressions method.
/// This method returns no data upon success.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [log impressions](MethodLogImpressionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3LogImpressionsResponse { _never_set: Option<bool> }

impl client::ResponseResult for GoogleMapsPlayablelocationsV3LogImpressionsResponse {}


/// A request for logging your player’s bad location reports.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [log player reports](MethodLogPlayerReportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3LogPlayerReportsRequest {
    /// Required. Information about the client device (for example, device model and
    /// operating system).
    #[serde(rename="clientInfo")]
    
    pub client_info: Option<GoogleMapsUnityClientInfo>,
    /// Required. Player reports. The maximum number of player reports that you can log at
    /// once is 50.
    #[serde(rename="playerReports")]
    
    pub player_reports: Option<Vec<GoogleMapsPlayablelocationsV3PlayerReport>>,
    /// Required. A string that uniquely identifies the log player reports request. This
    /// allows you to detect duplicate requests. We recommend that you use UUIDs
    /// for this value. The value must not exceed 50 characters.
    /// 
    /// You should reuse the `request_id` only when retrying a request in the case
    /// of a failure. In that case, the request must be identical to the one that
    /// failed.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for GoogleMapsPlayablelocationsV3LogPlayerReportsRequest {}


/// A response for the LogPlayerReports
/// method.
/// 
/// This method returns no data upon success.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [log player reports](MethodLogPlayerReportCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3LogPlayerReportsResponse { _never_set: Option<bool> }

impl client::ResponseResult for GoogleMapsPlayablelocationsV3LogPlayerReportsResponse {}


/// A report submitted by a player about a playable location that is considered
/// inappropriate for use in the game.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3PlayerReport {
    /// Language code (in BCP-47 format) indicating the language of the freeform
    /// description provided in `reason_details`. Examples are "en", "en-US" or
    /// "ja-Latn". For more information, see
    /// http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Required. The name of the playable location.
    #[serde(rename="locationName")]
    
    pub location_name: Option<String>,
    /// Required. A free-form description detailing why the playable location is
    /// considered bad.
    #[serde(rename="reasonDetails")]
    
    pub reason_details: Option<String>,
    /// Required. One or more reasons why this playable location is considered bad.
    
    pub reasons: Option<Vec<GoogleMapsPlayablelocationsV3PlayerReportReasonsEnum>>,
}

impl client::Part for GoogleMapsPlayablelocationsV3PlayerReport {}


/// Specifies the area to search for playable locations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3SampleAreaFilter {
    /// Required. The S2 cell ID of the area you want. This must be between cell level 11 and
    /// 14 (inclusive).
    /// 
    /// S2 cells are 64-bit integers that identify areas on the Earth. They are
    /// hierarchical, and can therefore be used for spatial indexing.
    /// 
    /// The S2 geometry library is available in a number of languages:
    /// 
    ///   * [C++](https://github.com/google/s2geometry)
    ///   * [Java](https://github.com/google/s2-geometry-library-java)
    ///   * [Go](https://github.com/golang/geo)
    ///   * [Python](https://github.com/google/s2geometry/tree/master/src/python)
    #[serde(rename="s2CellId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub s2_cell_id: Option<u64>,
}

impl client::Part for GoogleMapsPlayablelocationsV3SampleAreaFilter {}


/// Encapsulates a filter criterion for searching for a set of playable
/// locations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3SampleCriterion {
    /// Specifies which `PlayableLocation` fields are returned.
    /// 
    /// `name` (which is used for logging impressions), `center_point` and
    /// `place_id` (or `plus_code`) are always returned.
    /// 
    /// The following fields are omitted unless you specify them here:
    /// 
    ///   * snapped_point
    ///   * types
    /// 
    /// Note: The more fields you include, the more expensive in terms of data and
    /// associated latency your query will be.
    #[serde(rename="fieldsToReturn")]
    
    pub fields_to_return: Option<client::FieldMask>,
    /// Specifies filtering options, and specifies what will be included in the
    /// result set.
    
    pub filter: Option<GoogleMapsPlayablelocationsV3SampleFilter>,
    /// Required. An arbitrary, developer-defined identifier of the type of game object that
    /// the playable location is used for. This field allows you to specify
    /// criteria per game object type when searching for playable locations.
    /// 
    /// You should assign a unique `game_object_type` ID across all
    /// `request_criteria` to represent a distinct type of game object. For
    /// example, 1=monster location, 2=powerup location.
    /// 
    /// The response contains a map<game_object_type, Response>.
    #[serde(rename="gameObjectType")]
    
    pub game_object_type: Option<i32>,
}

impl client::Part for GoogleMapsPlayablelocationsV3SampleCriterion {}


/// Specifies the filters to use when searching for playable locations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3SampleFilter {
    /// Restricts the set of playable locations to just the
    /// [types](https://developers.google.com/maps/documentation/gaming/tt/types) that you want.
    #[serde(rename="includedTypes")]
    
    pub included_types: Option<Vec<String>>,
    /// Specifies the maximum number of playable locations to return. This value
    /// must not be greater than 1000. The default value is 100.
    /// 
    /// Only the top-ranking playable locations are returned.
    #[serde(rename="maxLocationCount")]
    
    pub max_location_count: Option<i32>,
    /// A set of options that control the spacing between playable locations. By
    /// default the minimum distance between locations is 200m.
    
    pub spacing: Option<GoogleMapsPlayablelocationsV3SampleSpacingOptions>,
}

impl client::Part for GoogleMapsPlayablelocationsV3SampleFilter {}


/// A geographical point suitable for placing game objects in location-based
/// games.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3SamplePlayableLocation {
    /// Required. The latitude and longitude associated with the center of the playable
    /// location.
    /// 
    /// By default, the set of playable locations returned from
    /// SamplePlayableLocations use
    /// center-point coordinates.
    #[serde(rename="centerPoint")]
    
    pub center_point: Option<GoogleTypeLatLng>,
    /// Required. The name of this playable location.
    
    pub name: Option<String>,
    /// A [place ID] (https://developers.google.com/places/place-id)
    #[serde(rename="placeId")]
    
    pub place_id: Option<String>,
    /// A [plus code] (http://openlocationcode.com)
    #[serde(rename="plusCode")]
    
    pub plus_code: Option<String>,
    /// The playable location's coordinates, snapped to the sidewalk of the
    /// nearest road, if a nearby road exists.
    #[serde(rename="snappedPoint")]
    
    pub snapped_point: Option<GoogleTypeLatLng>,
    /// A collection of [Playable Location
    /// Types](https://developers.google.com/maps/documentation/gaming/tt/types) for this playable location. The
    /// first type in the collection is the primary type.
    /// 
    /// Type information might not be available for all playable locations.
    
    pub types: Option<Vec<String>>,
}

impl client::Part for GoogleMapsPlayablelocationsV3SamplePlayableLocation {}


/// A list of PlayableLocation objects that satisfies a single Criterion.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3SamplePlayableLocationList {
    /// A list of playable locations for this game object type.
    
    pub locations: Option<Vec<GoogleMapsPlayablelocationsV3SamplePlayableLocation>>,
}

impl client::Part for GoogleMapsPlayablelocationsV3SamplePlayableLocationList {}


/// Life of a query:
/// 
/// * When a game starts in a new location, your game server issues a
///   SamplePlayableLocations
///   request. The request specifies the S2 cell, and contains one or more
///   “criteria” for filtering:
/// 
/// * Criterion 0: i locations for long-lived bases, or level 0 monsters, or…
/// 
/// * Criterion 1: j locations for short-lived bases, or level 1 monsters, …
/// 
/// * Criterion 2: k locations for random objects.
/// 
/// * etc (up to 5 criterion may be specified).
/// 
/// `PlayableLocationList` will then contain mutually
/// exclusive lists of `PlayableLocation` objects that satisfy each of
/// the criteria. Think of it as a collection of real-world locations that you
/// can then associate with your game state.
/// 
/// Note: These points are impermanent in nature. E.g, parks can close, and
/// places can be removed.
/// 
/// The response specifies how long you can expect the playable locations to
/// last. Once they expire, you should query the `samplePlayableLocations` API
/// again to get a fresh view of the real world.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sample playable locations](MethodSamplePlayableLocationCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3SamplePlayableLocationsRequest {
    /// Required. Specifies the area to search within for playable locations.
    #[serde(rename="areaFilter")]
    
    pub area_filter: Option<GoogleMapsPlayablelocationsV3SampleAreaFilter>,
    /// Required. Specifies one or more (up to 5) criteria for filtering the
    /// returned playable locations.
    
    pub criteria: Option<Vec<GoogleMapsPlayablelocationsV3SampleCriterion>>,
}

impl client::RequestValue for GoogleMapsPlayablelocationsV3SamplePlayableLocationsRequest {}


/// Response for the
/// SamplePlayableLocations
/// method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sample playable locations](MethodSamplePlayableLocationCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3SamplePlayableLocationsResponse {
    /// Each PlayableLocation object corresponds to a game_object_type specified
    /// in the request.
    #[serde(rename="locationsPerGameObjectType")]
    
    pub locations_per_game_object_type: Option<HashMap<String, GoogleMapsPlayablelocationsV3SamplePlayableLocationList>>,
    /// Required. Specifies the "time-to-live" for the set of playable locations. You can use
    /// this value to determine how long to cache the set of playable locations.
    /// After this length of time, your back-end game server should issue a new
    /// SamplePlayableLocations
    /// request to get a fresh set of playable locations (because for example, they
    /// might have been removed, a park might have closed for the day, a
    /// business might have closed permanently).
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub ttl: Option<client::chrono::Duration>,
}

impl client::ResponseResult for GoogleMapsPlayablelocationsV3SamplePlayableLocationsResponse {}


/// A set of options that specifies the separation between playable locations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsPlayablelocationsV3SampleSpacingOptions {
    /// Required. The minimum spacing between any two playable locations, measured in meters.
    /// The minimum value is 30.
    /// The maximum value is 1000.
    /// 
    /// Inputs will be rounded up to the next 10 meter interval.
    /// 
    /// The default value is 200m.
    /// 
    /// Set this field to remove tight clusters of playable locations.
    /// 
    /// Note:
    /// 
    /// The spacing is a greedy algorithm. It optimizes for selecting the highest
    /// ranking locations first, not to maximize the number of locations selected.
    /// Consider the following scenario:
    /// 
    ///   * Rank: A: 2, B: 1, C: 3.
    ///   * Distance: A--200m--B--200m--C
    /// 
    /// If spacing=250, it will pick the highest ranked location [B], not [A, C].
    /// 
    /// 
    /// Note:
    /// 
    /// Spacing works within the game object type itself, as well as the previous
    /// ones.
    /// Suppose three game object types, each with the following spacing:
    /// 
    ///   * X: 400m, Y: undefined, Z: 200m.
    /// 
    /// 1. Add locations for X, within 400m of each other.
    /// 2. Add locations for Y, without any spacing.
    /// 3. Finally, add locations for Z within 200m of each other as well X and Y.
    /// 
    /// The distance diagram between those locations end up as:
    /// 
    ///   * From->To.
    ///   * X->X: 400m
    ///   * Y->X, Y->Y: unspecified.
    ///   * Z->X, Z->Y, Z->Z: 200m.
    #[serde(rename="minSpacingMeters")]
    
    pub min_spacing_meters: Option<f64>,
    /// Specifies whether the minimum spacing constraint applies to the
    /// center-point or to the snapped point of playable locations. The default
    /// value is `CENTER_POINT`.
    /// 
    /// If a snapped point is not available for a playable location, its
    /// center-point is used instead.
    /// 
    /// Set this to the point type used in your game.
    #[serde(rename="pointType")]
    
    pub point_type: Option<GoogleMapsPlayablelocationsV3SampleSpacingOptionPointTypeEnum>,
}

impl client::Part for GoogleMapsPlayablelocationsV3SampleSpacingOptions {}


/// Client information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMapsUnityClientInfo {
    /// API client name and version. For example, the SDK calling the API. The
    /// exact format is up to the client.
    #[serde(rename="apiClient")]
    
    pub api_client: Option<String>,
    /// Application ID, such as the package name on Android and the bundle
    /// identifier on iOS platforms.
    #[serde(rename="applicationId")]
    
    pub application_id: Option<String>,
    /// Application version number, such as "1.2.3". The exact format is
    /// application-dependent.
    #[serde(rename="applicationVersion")]
    
    pub application_version: Option<String>,
    /// Device model as reported by the device. The exact format is
    /// platform-dependent.
    #[serde(rename="deviceModel")]
    
    pub device_model: Option<String>,
    /// Language code (in BCP-47 format) indicating the UI language of the client.
    /// Examples are "en", "en-US" or "ja-Latn". For more information, see
    /// http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Operating system name and version as reported by the OS. For example,
    /// "Mac OS X 10.10.4". The exact format is platform-dependent.
    #[serde(rename="operatingSystem")]
    
    pub operating_system: Option<String>,
    /// Build number/version of the operating system. e.g., the contents of
    /// android.os.Build.ID in Android, or the contents of sysctl "kern.osversion"
    /// in iOS.
    #[serde(rename="operatingSystemBuild")]
    
    pub operating_system_build: Option<String>,
    /// Platform where the application is running.
    
    pub platform: Option<GoogleMapsUnityClientInfoPlatformEnum>,
}

impl client::Part for GoogleMapsUnityClientInfo {}


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
pub struct GoogleTypeLatLng {
    /// The latitude in degrees. It must be in the range [-90.0, +90.0].
    
    pub latitude: Option<f64>,
    /// The longitude in degrees. It must be in the range [-180.0, +180.0].
    
    pub longitude: Option<f64>,
}

impl client::Part for GoogleTypeLatLng {}


