#![allow(clippy::ptr_arg)]

use std::collections::{BTreeSet, HashMap};

use tokio::time::sleep;

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Clone, Copy)]
pub enum Scope {
    /// See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.
    CloudPlatform,

    /// Private Service: https://www.googleapis.com/auth/maps-platform.places
    MapPlatformPlace,

    /// Private Service: https://www.googleapis.com/auth/maps-platform.places.autocomplete
    MapPlatformPlaceAutocomplete,

    /// Private Service: https://www.googleapis.com/auth/maps-platform.places.details
    MapPlatformPlaceDetail,

    /// Private Service: https://www.googleapis.com/auth/maps-platform.places.getphotomedia
    MapPlatformPlaceGetphotomedia,

    /// Private Service: https://www.googleapis.com/auth/maps-platform.places.nearbysearch
    MapPlatformPlaceNearbysearch,

    /// Private Service: https://www.googleapis.com/auth/maps-platform.places.textsearch
    MapPlatformPlaceTextsearch,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::MapPlatformPlace => "https://www.googleapis.com/auth/maps-platform.places",
            Scope::MapPlatformPlaceAutocomplete => {
                "https://www.googleapis.com/auth/maps-platform.places.autocomplete"
            }
            Scope::MapPlatformPlaceDetail => {
                "https://www.googleapis.com/auth/maps-platform.places.details"
            }
            Scope::MapPlatformPlaceGetphotomedia => {
                "https://www.googleapis.com/auth/maps-platform.places.getphotomedia"
            }
            Scope::MapPlatformPlaceNearbysearch => {
                "https://www.googleapis.com/auth/maps-platform.places.nearbysearch"
            }
            Scope::MapPlatformPlaceTextsearch => {
                "https://www.googleapis.com/auth/maps-platform.places.textsearch"
            }
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for Scope {
    fn default() -> Scope {
        Scope::CloudPlatform
    }
}

// ########
// HUB ###
// ######

/// Central instance to access all MapsPlaces related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_places1 as places1;
/// use places1::{Result, Error};
/// # async fn dox() {
/// use places1::{MapsPlaces, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and
/// // `client_secret`, among other things.
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you,
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let connector = hyper_rustls::HttpsConnectorBuilder::new()
///     .with_native_roots()
///     .unwrap()
///     .https_only()
///     .enable_http2()
///     .build();
///
/// let executor = hyper_util::rt::TokioExecutor::new();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     yup_oauth2::client::CustomHyperClientBuilder::from(
///         hyper_util::client::legacy::Client::builder(executor).build(connector),
///     ),
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http2()
///         .build()
/// );
/// let mut hub = MapsPlaces::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.places().get("name")
///              .session_token("takimata")
///              .region_code("amet.")
///              .language_code("duo")
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
pub struct MapsPlaces<C> {
    pub client: common::Client<C>,
    pub auth: Box<dyn common::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<C> common::Hub for MapsPlaces<C> {}

impl<'a, C> MapsPlaces<C> {
    pub fn new<A: 'static + common::GetToken>(client: common::Client<C>, auth: A) -> MapsPlaces<C> {
        MapsPlaces {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/7.0.0".to_string(),
            _base_url: "https://places.googleapis.com/".to_string(),
            _root_url: "https://places.googleapis.com/".to_string(),
        }
    }

    pub fn places(&'a self) -> PlaceMethods<'a, C> {
        PlaceMethods { hub: self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/7.0.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        std::mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://places.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        std::mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://places.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        std::mem::replace(&mut self._root_url, new_root_url)
    }
}

// ############
// SCHEMAS ###
// ##########
/// A latitude-longitude viewport, represented as two diagonally opposite `low` and `high` points. A viewport is considered a closed region, i.e. it includes its boundary. The latitude bounds must range between -90 to 90 degrees inclusive, and the longitude bounds must range between -180 to 180 degrees inclusive. Various cases include: - If `low` = `high`, the viewport consists of that single point. - If `low.longitude` > `high.longitude`, the longitude range is inverted (the viewport crosses the 180 degree longitude line). - If `low.longitude` = -180 degrees and `high.longitude` = 180 degrees, the viewport includes all longitudes. - If `low.longitude` = 180 degrees and `high.longitude` = -180 degrees, the longitude range is empty. - If `low.latitude` > `high.latitude`, the latitude range is empty. Both `low` and `high` must be populated, and the represented box cannot be empty (as specified by the definitions above). An empty viewport will result in an error. For example, this viewport fully encloses New York City: { "low": { "latitude": 40.477398, "longitude": -74.259087 }, "high": { "latitude": 40.91618, "longitude": -73.70018 } }
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleGeoTypeViewport {
    /// Required. The high point of the viewport.
    pub high: Option<GoogleTypeLatLng>,
    /// Required. The low point of the viewport.
    pub low: Option<GoogleTypeLatLng>,
}

impl common::Part for GoogleGeoTypeViewport {}

/// A relational description of a location. Includes a ranked set of nearby landmarks and precise containing areas and their relationship to the target location.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1AddressDescriptor {
    /// A ranked list of containing or adjacent areas. The most recognizable and precise areas are ranked first.
    pub areas: Option<Vec<GoogleMapsPlacesV1AddressDescriptorArea>>,
    /// A ranked list of nearby landmarks. The most recognizable and nearby landmarks are ranked first.
    pub landmarks: Option<Vec<GoogleMapsPlacesV1AddressDescriptorLandmark>>,
}

impl common::Part for GoogleMapsPlacesV1AddressDescriptor {}

/// Area information and the area's relationship with the target location. Areas includes precise sublocality, neighborhoods, and large compounds that are useful for describing a location.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1AddressDescriptorArea {
    /// Defines the spatial relationship between the target location and the area.
    pub containment: Option<String>,
    /// The area's display name.
    #[serde(rename = "displayName")]
    pub display_name: Option<GoogleTypeLocalizedText>,
    /// The area's resource name.
    pub name: Option<String>,
    /// The area's place id.
    #[serde(rename = "placeId")]
    pub place_id: Option<String>,
}

impl common::Part for GoogleMapsPlacesV1AddressDescriptorArea {}

/// Basic landmark information and the landmark's relationship with the target location. Landmarks are prominent places that can be used to describe a location.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1AddressDescriptorLandmark {
    /// The landmark's display name.
    #[serde(rename = "displayName")]
    pub display_name: Option<GoogleTypeLocalizedText>,
    /// The landmark's resource name.
    pub name: Option<String>,
    /// The landmark's place id.
    #[serde(rename = "placeId")]
    pub place_id: Option<String>,
    /// Defines the spatial relationship between the target location and the landmark.
    #[serde(rename = "spatialRelationship")]
    pub spatial_relationship: Option<String>,
    /// The straight line distance, in meters, between the center point of the target and the center point of the landmark. In some situations, this value can be longer than `travel_distance_meters`.
    #[serde(rename = "straightLineDistanceMeters")]
    pub straight_line_distance_meters: Option<f32>,
    /// The travel distance, in meters, along the road network from the target to the landmark, if known. This value does not take into account the mode of transportation, such as walking, driving, or biking.
    #[serde(rename = "travelDistanceMeters")]
    pub travel_distance_meters: Option<f32>,
    /// A set of type tags for this landmark. For a complete list of possible values, see https://developers.google.com/maps/documentation/places/web-service/place-types.
    pub types: Option<Vec<String>>,
}

impl common::Part for GoogleMapsPlacesV1AddressDescriptorLandmark {}

/// Information about the author of the UGC data. Used in Photo, and Review.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1AuthorAttribution {
    /// Name of the author of the Photo or Review.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// Profile photo URI of the author of the Photo or Review.
    #[serde(rename = "photoUri")]
    pub photo_uri: Option<String>,
    /// URI of the author of the Photo or Review.
    pub uri: Option<String>,
}

impl common::Part for GoogleMapsPlacesV1AuthorAttribution {}

/// Request proto for AutocompletePlaces.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [autocomplete places](PlaceAutocompleteCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1AutocompletePlacesRequest {
    /// Optional. Include pure service area businesses if the field is set to true. Pure service area business is a business that visits or delivers to customers directly but does not serve customers at their business address. For example, businesses like cleaning services or plumbers. Those businesses do not have a physical address or location on Google Maps. Places will not return fields including `location`, `plus_code`, and other location related fields for these businesses.
    #[serde(rename = "includePureServiceAreaBusinesses")]
    pub include_pure_service_area_businesses: Option<bool>,
    /// Optional. If true, the response will include both Place and query predictions. Otherwise the response will only return Place predictions.
    #[serde(rename = "includeQueryPredictions")]
    pub include_query_predictions: Option<bool>,
    /// Optional. Included primary Place type (for example, "restaurant" or "gas_station") in Place Types (https://developers.google.com/maps/documentation/places/web-service/place-types), or only `(regions)`, or only `(cities)`. A Place is only returned if its primary type is included in this list. Up to 5 values can be specified. If no types are specified, all Place types are returned.
    #[serde(rename = "includedPrimaryTypes")]
    pub included_primary_types: Option<Vec<String>>,
    /// Optional. Only include results in the specified regions, specified as up to 15 CLDR two-character region codes. An empty set will not restrict the results. If both `location_restriction` and `included_region_codes` are set, the results will be located in the area of intersection.
    #[serde(rename = "includedRegionCodes")]
    pub included_region_codes: Option<Vec<String>>,
    /// Required. The text string on which to search.
    pub input: Option<String>,
    /// Optional. A zero-based Unicode character offset of `input` indicating the cursor position in `input`. The cursor position may influence what predictions are returned. If empty, defaults to the length of `input`.
    #[serde(rename = "inputOffset")]
    pub input_offset: Option<i32>,
    /// Optional. The language in which to return results. Defaults to en-US. The results may be in mixed languages if the language used in `input` is different from `language_code` or if the returned Place does not have a translation from the local language to `language_code`.
    #[serde(rename = "languageCode")]
    pub language_code: Option<String>,
    /// Optional. Bias results to a specified location. At most one of `location_bias` or `location_restriction` should be set. If neither are set, the results will be biased by IP address, meaning the IP address will be mapped to an imprecise location and used as a biasing signal.
    #[serde(rename = "locationBias")]
    pub location_bias: Option<GoogleMapsPlacesV1AutocompletePlacesRequestLocationBias>,
    /// Optional. Restrict results to a specified location. At most one of `location_bias` or `location_restriction` should be set. If neither are set, the results will be biased by IP address, meaning the IP address will be mapped to an imprecise location and used as a biasing signal.
    #[serde(rename = "locationRestriction")]
    pub location_restriction:
        Option<GoogleMapsPlacesV1AutocompletePlacesRequestLocationRestriction>,
    /// Optional. The origin point from which to calculate geodesic distance to the destination (returned as `distance_meters`). If this value is omitted, geodesic distance will not be returned.
    pub origin: Option<GoogleTypeLatLng>,
    /// Optional. The region code, specified as a CLDR two-character region code. This affects address formatting, result ranking, and may influence what results are returned. This does not restrict results to the specified region. To restrict results to a region, use `region_code_restriction`.
    #[serde(rename = "regionCode")]
    pub region_code: Option<String>,
    /// Optional. A string which identifies an Autocomplete session for billing purposes. Must be a URL and filename safe base64 string with at most 36 ASCII characters in length. Otherwise an INVALID_ARGUMENT error is returned. The session begins when the user starts typing a query, and concludes when they select a place and a call to Place Details or Address Validation is made. Each session can have multiple queries, followed by one Place Details or Address Validation request. The credentials used for each request within a session must belong to the same Google Cloud Console project. Once a session has concluded, the token is no longer valid; your app must generate a fresh token for each session. If the `session_token` parameter is omitted, or if you reuse a session token, the session is charged as if no session token was provided (each request is billed separately). We recommend the following guidelines: * Use session tokens for all Place Autocomplete calls. * Generate a fresh token for each session. Using a version 4 UUID is recommended. * Ensure that the credentials used for all Place Autocomplete, Place Details, and Address Validation requests within a session belong to the same Cloud Console project. * Be sure to pass a unique session token for each new session. Using the same token for more than one session will result in each request being billed individually.
    #[serde(rename = "sessionToken")]
    pub session_token: Option<String>,
}

impl common::RequestValue for GoogleMapsPlacesV1AutocompletePlacesRequest {}

/// The region to search. The results may be biased around the specified region.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1AutocompletePlacesRequestLocationBias {
    /// A circle defined by a center point and radius.
    pub circle: Option<GoogleMapsPlacesV1Circle>,
    /// A viewport defined by a northeast and a southwest corner.
    pub rectangle: Option<GoogleGeoTypeViewport>,
}

impl common::Part for GoogleMapsPlacesV1AutocompletePlacesRequestLocationBias {}

/// The region to search. The results will be restricted to the specified region.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1AutocompletePlacesRequestLocationRestriction {
    /// A circle defined by a center point and radius.
    pub circle: Option<GoogleMapsPlacesV1Circle>,
    /// A viewport defined by a northeast and a southwest corner.
    pub rectangle: Option<GoogleGeoTypeViewport>,
}

impl common::Part for GoogleMapsPlacesV1AutocompletePlacesRequestLocationRestriction {}

/// Response proto for AutocompletePlaces.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [autocomplete places](PlaceAutocompleteCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1AutocompletePlacesResponse {
    /// Contains a list of suggestions, ordered in descending order of relevance.
    pub suggestions: Option<Vec<GoogleMapsPlacesV1AutocompletePlacesResponseSuggestion>>,
}

impl common::ResponseResult for GoogleMapsPlacesV1AutocompletePlacesResponse {}

/// An Autocomplete suggestion result.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1AutocompletePlacesResponseSuggestion {
    /// A prediction for a Place.
    #[serde(rename = "placePrediction")]
    pub place_prediction:
        Option<GoogleMapsPlacesV1AutocompletePlacesResponseSuggestionPlacePrediction>,
    /// A prediction for a query.
    #[serde(rename = "queryPrediction")]
    pub query_prediction:
        Option<GoogleMapsPlacesV1AutocompletePlacesResponseSuggestionQueryPrediction>,
}

impl common::Part for GoogleMapsPlacesV1AutocompletePlacesResponseSuggestion {}

/// Text representing a Place or query prediction. The text may be used as is or formatted.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1AutocompletePlacesResponseSuggestionFormattableText {
    /// A list of string ranges identifying where the input request matched in `text`. The ranges can be used to format specific parts of `text`. The substrings may not be exact matches of `input` if the matching was determined by criteria other than string matching (for example, spell corrections or transliterations). These values are Unicode character offsets of `text`. The ranges are guaranteed to be ordered in increasing offset values.
    pub matches: Option<Vec<GoogleMapsPlacesV1AutocompletePlacesResponseSuggestionStringRange>>,
    /// Text that may be used as is or formatted with `matches`.
    pub text: Option<String>,
}

impl common::Part for GoogleMapsPlacesV1AutocompletePlacesResponseSuggestionFormattableText {}

/// Prediction results for a Place Autocomplete prediction.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1AutocompletePlacesResponseSuggestionPlacePrediction {
    /// The length of the geodesic in meters from `origin` if `origin` is specified. Certain predictions such as routes may not populate this field.
    #[serde(rename = "distanceMeters")]
    pub distance_meters: Option<i32>,
    /// The resource name of the suggested Place. This name can be used in other APIs that accept Place names.
    pub place: Option<String>,
    /// The unique identifier of the suggested Place. This identifier can be used in other APIs that accept Place IDs.
    #[serde(rename = "placeId")]
    pub place_id: Option<String>,
    /// A breakdown of the Place prediction into main text containing the name of the Place and secondary text containing additional disambiguating features (such as a city or region). `structured_format` is recommended for developers who wish to show two separate, but related, UI elements. Developers who wish to show a single UI element may want to use `text` instead. They are two different ways to represent a Place prediction. Users should not try to parse `structured_format` into `text` or vice versa.
    #[serde(rename = "structuredFormat")]
    pub structured_format:
        Option<GoogleMapsPlacesV1AutocompletePlacesResponseSuggestionStructuredFormat>,
    /// Contains the human-readable name for the returned result. For establishment results, this is usually the business name and address. `text` is recommended for developers who wish to show a single UI element. Developers who wish to show two separate, but related, UI elements may want to use `structured_format` instead. They are two different ways to represent a Place prediction. Users should not try to parse `structured_format` into `text` or vice versa. This text may be different from the `display_name` returned by GetPlace. May be in mixed languages if the request `input` and `language_code` are in different languages or if the Place does not have a translation from the local language to `language_code`.
    pub text: Option<GoogleMapsPlacesV1AutocompletePlacesResponseSuggestionFormattableText>,
    /// List of types that apply to this Place from Table A or Table B in https://developers.google.com/maps/documentation/places/web-service/place-types. A type is a categorization of a Place. Places with shared types will share similar characteristics.
    pub types: Option<Vec<String>>,
}

impl common::Part for GoogleMapsPlacesV1AutocompletePlacesResponseSuggestionPlacePrediction {}

/// Prediction results for a Query Autocomplete prediction.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1AutocompletePlacesResponseSuggestionQueryPrediction {
    /// A breakdown of the query prediction into main text containing the query and secondary text containing additional disambiguating features (such as a city or region). `structured_format` is recommended for developers who wish to show two separate, but related, UI elements. Developers who wish to show a single UI element may want to use `text` instead. They are two different ways to represent a query prediction. Users should not try to parse `structured_format` into `text` or vice versa.
    #[serde(rename = "structuredFormat")]
    pub structured_format:
        Option<GoogleMapsPlacesV1AutocompletePlacesResponseSuggestionStructuredFormat>,
    /// The predicted text. This text does not represent a Place, but rather a text query that could be used in a search endpoint (for example, Text Search). `text` is recommended for developers who wish to show a single UI element. Developers who wish to show two separate, but related, UI elements may want to use `structured_format` instead. They are two different ways to represent a query prediction. Users should not try to parse `structured_format` into `text` or vice versa. May be in mixed languages if the request `input` and `language_code` are in different languages or if part of the query does not have a translation from the local language to `language_code`.
    pub text: Option<GoogleMapsPlacesV1AutocompletePlacesResponseSuggestionFormattableText>,
}

impl common::Part for GoogleMapsPlacesV1AutocompletePlacesResponseSuggestionQueryPrediction {}

/// Identifies a substring within a given text.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1AutocompletePlacesResponseSuggestionStringRange {
    /// Zero-based offset of the last Unicode character (exclusive).
    #[serde(rename = "endOffset")]
    pub end_offset: Option<i32>,
    /// Zero-based offset of the first Unicode character of the string (inclusive).
    #[serde(rename = "startOffset")]
    pub start_offset: Option<i32>,
}

impl common::Part for GoogleMapsPlacesV1AutocompletePlacesResponseSuggestionStringRange {}

/// Contains a breakdown of a Place or query prediction into main text and secondary text. For Place predictions, the main text contains the specific name of the Place. For query predictions, the main text contains the query. The secondary text contains additional disambiguating features (such as a city or region) to further identify the Place or refine the query.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1AutocompletePlacesResponseSuggestionStructuredFormat {
    /// Represents the name of the Place or query.
    #[serde(rename = "mainText")]
    pub main_text: Option<GoogleMapsPlacesV1AutocompletePlacesResponseSuggestionFormattableText>,
    /// Represents additional disambiguating features (such as a city or region) to further identify the Place or refine the query.
    #[serde(rename = "secondaryText")]
    pub secondary_text:
        Option<GoogleMapsPlacesV1AutocompletePlacesResponseSuggestionFormattableText>,
}

impl common::Part for GoogleMapsPlacesV1AutocompletePlacesResponseSuggestionStructuredFormat {}

/// Circle with a LatLng as center and radius.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1Circle {
    /// Required. Center latitude and longitude. The range of latitude must be within [-90.0, 90.0]. The range of the longitude must be within [-180.0, 180.0].
    pub center: Option<GoogleTypeLatLng>,
    /// Required. Radius measured in meters. The radius must be within [0.0, 50000.0].
    pub radius: Option<f64>,
}

impl common::Part for GoogleMapsPlacesV1Circle {}

/// A block of content that can be served individually.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1ContentBlock {
    /// Content related to the topic.
    pub content: Option<GoogleTypeLocalizedText>,
    /// The list of resource names of the referenced places. This name can be used in other APIs that accept Place resource names.
    #[serde(rename = "referencedPlaces")]
    pub referenced_places: Option<Vec<String>>,
}

impl common::Part for GoogleMapsPlacesV1ContentBlock {}

/// Experimental: See https://developers.google.com/maps/documentation/places/web-service/experimental/places-generative for more details. Content that is contextual to the place query.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1ContextualContent {
    /// Experimental: See https://developers.google.com/maps/documentation/places/web-service/experimental/places-generative for more details. Justifications for the place.
    pub justifications: Option<Vec<GoogleMapsPlacesV1ContextualContentJustification>>,
    /// Information (including references) about photos of this place, contexual to the place query.
    pub photos: Option<Vec<GoogleMapsPlacesV1Photo>>,
    /// List of reviews about this place, contexual to the place query.
    pub reviews: Option<Vec<GoogleMapsPlacesV1Review>>,
}

impl common::Part for GoogleMapsPlacesV1ContextualContent {}

/// Experimental: See https://developers.google.com/maps/documentation/places/web-service/experimental/places-generative for more details. Justifications for the place. Justifications answers the question of why a place could interest an end user.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1ContextualContentJustification {
    /// Experimental: See https://developers.google.com/maps/documentation/places/web-service/experimental/places-generative for more details.
    #[serde(rename = "businessAvailabilityAttributesJustification")]
    pub business_availability_attributes_justification: Option<
        GoogleMapsPlacesV1ContextualContentJustificationBusinessAvailabilityAttributesJustification,
    >,
    /// Experimental: See https://developers.google.com/maps/documentation/places/web-service/experimental/places-generative for more details.
    #[serde(rename = "reviewJustification")]
    pub review_justification:
        Option<GoogleMapsPlacesV1ContextualContentJustificationReviewJustification>,
}

impl common::Part for GoogleMapsPlacesV1ContextualContentJustification {}

/// Experimental: See https://developers.google.com/maps/documentation/places/web-service/experimental/places-generative for more details. BusinessAvailabilityAttributes justifications. This shows some attributes a business has that could interest an end user.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1ContextualContentJustificationBusinessAvailabilityAttributesJustification
{
    /// If a place provides delivery.
    pub delivery: Option<bool>,
    /// If a place provides dine-in.
    #[serde(rename = "dineIn")]
    pub dine_in: Option<bool>,
    /// If a place provides takeout.
    pub takeout: Option<bool>,
}

impl common::Part
    for GoogleMapsPlacesV1ContextualContentJustificationBusinessAvailabilityAttributesJustification
{
}

/// Experimental: See https://developers.google.com/maps/documentation/places/web-service/experimental/places-generative for more details. User review justifications. This highlights a section of the user review that would interest an end user. For instance, if the search query is "firewood pizza", the review justification highlights the text relevant to the search query.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1ContextualContentJustificationReviewJustification {
    /// no description provided
    #[serde(rename = "highlightedText")]
    pub highlighted_text:
        Option<GoogleMapsPlacesV1ContextualContentJustificationReviewJustificationHighlightedText>,
    /// The review that the highlighted text is generated from.
    pub review: Option<GoogleMapsPlacesV1Review>,
}

impl common::Part for GoogleMapsPlacesV1ContextualContentJustificationReviewJustification {}

/// The text highlighted by the justification. This is a subset of the review itself. The exact word to highlight is marked by the HighlightedTextRange. There could be several words in the text being highlighted.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1ContextualContentJustificationReviewJustificationHighlightedText {
    /// The list of the ranges of the highlighted text.
    #[serde(rename="highlightedTextRanges")]
        pub highlighted_text_ranges: Option<Vec<GoogleMapsPlacesV1ContextualContentJustificationReviewJustificationHighlightedTextHighlightedTextRange>>,
    /// no description provided
        pub text: Option<String>,
}

impl common::Part
    for GoogleMapsPlacesV1ContextualContentJustificationReviewJustificationHighlightedText
{
}

/// The range of highlighted text.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1ContextualContentJustificationReviewJustificationHighlightedTextHighlightedTextRange
{
    /// no description provided
    #[serde(rename = "endIndex")]
    pub end_index: Option<i32>,
    /// no description provided
    #[serde(rename = "startIndex")]
    pub start_index: Option<i32>,
}

impl common::Part for GoogleMapsPlacesV1ContextualContentJustificationReviewJustificationHighlightedTextHighlightedTextRange {}

/// Information about the EV Charge Station hosted in Place. Terminology follows https://afdc.energy.gov/fuels/electricity_infrastructure.html One port could charge one car at a time. One port has one or more connectors. One station has one or more ports.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1EVChargeOptions {
    /// A list of EV charging connector aggregations that contain connectors of the same type and same charge rate.
    #[serde(rename = "connectorAggregation")]
    pub connector_aggregation: Option<Vec<GoogleMapsPlacesV1EVChargeOptionsConnectorAggregation>>,
    /// Number of connectors at this station. However, because some ports can have multiple connectors but only be able to charge one car at a time (e.g.) the number of connectors may be greater than the total number of cars which can charge simultaneously.
    #[serde(rename = "connectorCount")]
    pub connector_count: Option<i32>,
}

impl common::Part for GoogleMapsPlacesV1EVChargeOptions {}

/// EV charging information grouped by [type, max_charge_rate_kw]. Shows EV charge aggregation of connectors that have the same type and max charge rate in kw.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1EVChargeOptionsConnectorAggregation {
    /// The timestamp when the connector availability information in this aggregation was last updated.
    #[serde(rename = "availabilityLastUpdateTime")]
    pub availability_last_update_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Number of connectors in this aggregation that are currently available.
    #[serde(rename = "availableCount")]
    pub available_count: Option<i32>,
    /// Number of connectors in this aggregation.
    pub count: Option<i32>,
    /// The static max charging rate in kw of each connector in the aggregation.
    #[serde(rename = "maxChargeRateKw")]
    pub max_charge_rate_kw: Option<f64>,
    /// Number of connectors in this aggregation that are currently out of service.
    #[serde(rename = "outOfServiceCount")]
    pub out_of_service_count: Option<i32>,
    /// The connector type of this aggregation.
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

impl common::Part for GoogleMapsPlacesV1EVChargeOptionsConnectorAggregation {}

/// The most recent information about fuel options in a gas station. This information is updated regularly.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1FuelOptions {
    /// The last known fuel price for each type of fuel this station has. There is one entry per fuel type this station has. Order is not important.
    #[serde(rename = "fuelPrices")]
    pub fuel_prices: Option<Vec<GoogleMapsPlacesV1FuelOptionsFuelPrice>>,
}

impl common::Part for GoogleMapsPlacesV1FuelOptions {}

/// Fuel price information for a given type.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1FuelOptionsFuelPrice {
    /// The price of the fuel.
    pub price: Option<GoogleTypeMoney>,
    /// The type of fuel.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// The time the fuel price was last updated.
    #[serde(rename = "updateTime")]
    pub update_time: Option<chrono::DateTime<chrono::offset::Utc>>,
}

impl common::Part for GoogleMapsPlacesV1FuelOptionsFuelPrice {}

/// Information about a photo of a place.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1Photo {
    /// This photo's authors.
    #[serde(rename = "authorAttributions")]
    pub author_attributions: Option<Vec<GoogleMapsPlacesV1AuthorAttribution>>,
    /// A link where users can flag a problem with the photo.
    #[serde(rename = "flagContentUri")]
    pub flag_content_uri: Option<String>,
    /// A link to show the photo on Google Maps.
    #[serde(rename = "googleMapsUri")]
    pub google_maps_uri: Option<String>,
    /// The maximum available height, in pixels.
    #[serde(rename = "heightPx")]
    pub height_px: Option<i32>,
    /// Identifier. A reference representing this place photo which may be used to look up this place photo again (also called the API "resource" name: `places/{place_id}/photos/{photo}`).
    pub name: Option<String>,
    /// The maximum available width, in pixels.
    #[serde(rename = "widthPx")]
    pub width_px: Option<i32>,
}

impl common::Part for GoogleMapsPlacesV1Photo {}

/// A photo media from Places API.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [photos get media places](PlacePhotoGetMediaCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1PhotoMedia {
    /// The resource name of a photo media in the format: `places/{place_id}/photos/{photo_reference}/media`.
    pub name: Option<String>,
    /// A short-lived uri that can be used to render the photo.
    #[serde(rename = "photoUri")]
    pub photo_uri: Option<String>,
}

impl common::ResponseResult for GoogleMapsPlacesV1PhotoMedia {}

/// All the information representing a Place.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [get places](PlaceGetCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1Place {
    /// Information about the accessibility options a place offers.
    #[serde(rename = "accessibilityOptions")]
    pub accessibility_options: Option<GoogleMapsPlacesV1PlaceAccessibilityOptions>,
    /// Repeated components for each locality level. Note the following facts about the address_components[] array: - The array of address components may contain more components than the formatted_address. - The array does not necessarily include all the political entities that contain an address, apart from those included in the formatted_address. To retrieve all the political entities that contain a specific address, you should use reverse geocoding, passing the latitude/longitude of the address as a parameter to the request. - The format of the response is not guaranteed to remain the same between requests. In particular, the number of address_components varies based on the address requested and can change over time for the same address. A component can change position in the array. The type of the component can change. A particular component may be missing in a later response.
    #[serde(rename = "addressComponents")]
    pub address_components: Option<Vec<GoogleMapsPlacesV1PlaceAddressComponent>>,
    /// The address descriptor of the place. Address descriptors include additional information that help describe a location using landmarks and areas. See address descriptor regional coverage in https://developers.google.com/maps/documentation/geocoding/address-descriptors/coverage.
    #[serde(rename = "addressDescriptor")]
    pub address_descriptor: Option<GoogleMapsPlacesV1AddressDescriptor>,
    /// The place's address in adr microformat: http://microformats.org/wiki/adr.
    #[serde(rename = "adrFormatAddress")]
    pub adr_format_address: Option<String>,
    /// Place allows dogs.
    #[serde(rename = "allowsDogs")]
    pub allows_dogs: Option<bool>,
    /// A set of data provider that must be shown with this result.
    pub attributions: Option<Vec<GoogleMapsPlacesV1PlaceAttribution>>,
    /// The business status for the place.
    #[serde(rename = "businessStatus")]
    pub business_status: Option<String>,
    /// The consumer alert message for the place when we detect suspicious review activity on a business or a business violates our policies.
    #[serde(rename = "consumerAlert")]
    pub consumer_alert: Option<GoogleMapsPlacesV1PlaceConsumerAlert>,
    /// List of places in which the current place is located.
    #[serde(rename = "containingPlaces")]
    pub containing_places: Option<Vec<GoogleMapsPlacesV1PlaceContainingPlace>>,
    /// Specifies if the business supports curbside pickup.
    #[serde(rename = "curbsidePickup")]
    pub curbside_pickup: Option<bool>,
    /// The hours of operation for the next seven days (including today). The time period starts at midnight on the date of the request and ends at 11:59 pm six days later. This field includes the special_days subfield of all hours, set for dates that have exceptional hours.
    #[serde(rename = "currentOpeningHours")]
    pub current_opening_hours: Option<GoogleMapsPlacesV1PlaceOpeningHours>,
    /// Contains an array of entries for the next seven days including information about secondary hours of a business. Secondary hours are different from a business's main hours. For example, a restaurant can specify drive through hours or delivery hours as its secondary hours. This field populates the type subfield, which draws from a predefined list of opening hours types (such as DRIVE_THROUGH, PICKUP, or TAKEOUT) based on the types of the place. This field includes the special_days subfield of all hours, set for dates that have exceptional hours.
    #[serde(rename = "currentSecondaryOpeningHours")]
    pub current_secondary_opening_hours: Option<Vec<GoogleMapsPlacesV1PlaceOpeningHours>>,
    /// Specifies if the business supports delivery.
    pub delivery: Option<bool>,
    /// Specifies if the business supports indoor or outdoor seating options.
    #[serde(rename = "dineIn")]
    pub dine_in: Option<bool>,
    /// The localized name of the place, suitable as a short human-readable description. For example, "Google Sydney", "Starbucks", "Pyrmont", etc.
    #[serde(rename = "displayName")]
    pub display_name: Option<GoogleTypeLocalizedText>,
    /// Contains a summary of the place. A summary is comprised of a textual overview, and also includes the language code for these if applicable. Summary text must be presented as-is and can not be modified or altered.
    #[serde(rename = "editorialSummary")]
    pub editorial_summary: Option<GoogleTypeLocalizedText>,
    /// The summary of amenities near the EV charging station.
    #[serde(rename = "evChargeAmenitySummary")]
    pub ev_charge_amenity_summary: Option<GoogleMapsPlacesV1PlaceEvChargeAmenitySummary>,
    /// Information of ev charging options.
    #[serde(rename = "evChargeOptions")]
    pub ev_charge_options: Option<GoogleMapsPlacesV1EVChargeOptions>,
    /// A full, human-readable address for this place.
    #[serde(rename = "formattedAddress")]
    pub formatted_address: Option<String>,
    /// The most recent information about fuel options in a gas station. This information is updated regularly.
    #[serde(rename = "fuelOptions")]
    pub fuel_options: Option<GoogleMapsPlacesV1FuelOptions>,
    /// AI-generated summary of the place.
    #[serde(rename = "generativeSummary")]
    pub generative_summary: Option<GoogleMapsPlacesV1PlaceGenerativeSummary>,
    /// Place is good for children.
    #[serde(rename = "goodForChildren")]
    pub good_for_children: Option<bool>,
    /// Place accommodates groups.
    #[serde(rename = "goodForGroups")]
    pub good_for_groups: Option<bool>,
    /// Place is suitable for watching sports.
    #[serde(rename = "goodForWatchingSports")]
    pub good_for_watching_sports: Option<bool>,
    /// Links to trigger different Google Maps actions.
    #[serde(rename = "googleMapsLinks")]
    pub google_maps_links: Option<GoogleMapsPlacesV1PlaceGoogleMapsLinks>,
    /// A URL providing more information about this place.
    #[serde(rename = "googleMapsUri")]
    pub google_maps_uri: Option<String>,
    /// Background color for icon_mask in hex format, e.g. #909CE1.
    #[serde(rename = "iconBackgroundColor")]
    pub icon_background_color: Option<String>,
    /// A truncated URL to an icon mask. User can access different icon type by appending type suffix to the end (eg, ".svg" or ".png").
    #[serde(rename = "iconMaskBaseUri")]
    pub icon_mask_base_uri: Option<String>,
    /// The unique identifier of a place.
    pub id: Option<String>,
    /// A human-readable phone number for the place, in international format.
    #[serde(rename = "internationalPhoneNumber")]
    pub international_phone_number: Option<String>,
    /// Place provides live music.
    #[serde(rename = "liveMusic")]
    pub live_music: Option<bool>,
    /// The position of this place.
    pub location: Option<GoogleTypeLatLng>,
    /// Place has a children's menu.
    #[serde(rename = "menuForChildren")]
    pub menu_for_children: Option<bool>,
    /// If this Place is permanently closed and has moved to a new Place, this field contains the new Place's resource name, in `places/{place_id}` format. If this Place moved multiple times, this field will represent the first moved place. This field will not be populated if this Place has not moved.
    #[serde(rename = "movedPlace")]
    pub moved_place: Option<String>,
    /// If this Place is permanently closed and has moved to a new Place, this field contains the new Place's place ID. If this Place moved multiple times, this field will represent the first moved Place. This field will not be populated if this Place has not moved.
    #[serde(rename = "movedPlaceId")]
    pub moved_place_id: Option<String>,
    /// This Place's resource name, in `places/{place_id}` format. Can be used to look up the Place.
    pub name: Option<String>,
    /// A human-readable phone number for the place, in national format.
    #[serde(rename = "nationalPhoneNumber")]
    pub national_phone_number: Option<String>,
    /// A summary of points of interest near the place.
    #[serde(rename = "neighborhoodSummary")]
    pub neighborhood_summary: Option<GoogleMapsPlacesV1PlaceNeighborhoodSummary>,
    /// Place provides outdoor seating.
    #[serde(rename = "outdoorSeating")]
    pub outdoor_seating: Option<bool>,
    /// Options of parking provided by the place.
    #[serde(rename = "parkingOptions")]
    pub parking_options: Option<GoogleMapsPlacesV1PlaceParkingOptions>,
    /// Payment options the place accepts. If a payment option data is not available, the payment option field will be unset.
    #[serde(rename = "paymentOptions")]
    pub payment_options: Option<GoogleMapsPlacesV1PlacePaymentOptions>,
    /// Information (including references) about photos of this place. A maximum of 10 photos can be returned.
    pub photos: Option<Vec<GoogleMapsPlacesV1Photo>>,
    /// Plus code of the place location lat/long.
    #[serde(rename = "plusCode")]
    pub plus_code: Option<GoogleMapsPlacesV1PlacePlusCode>,
    /// The address in postal address format.
    #[serde(rename = "postalAddress")]
    pub postal_address: Option<GoogleTypePostalAddress>,
    /// Price level of the place.
    #[serde(rename = "priceLevel")]
    pub price_level: Option<String>,
    /// The price range associated with a Place.
    #[serde(rename = "priceRange")]
    pub price_range: Option<GoogleMapsPlacesV1PriceRange>,
    /// The primary type of the given result. This type must be one of the Places API supported types. For example, "restaurant", "cafe", "airport", etc. A place can only have a single primary type. For the complete list of possible values, see Table A and Table B at https://developers.google.com/maps/documentation/places/web-service/place-types. The primary type may be missing if the place's primary type is not a supported type. When a primary type is present, it is always one of the types in the `types` field.
    #[serde(rename = "primaryType")]
    pub primary_type: Option<String>,
    /// The display name of the primary type, localized to the request language if applicable. For the complete list of possible values, see Table A and Table B at https://developers.google.com/maps/documentation/places/web-service/place-types. The primary type may be missing if the place's primary type is not a supported type.
    #[serde(rename = "primaryTypeDisplayName")]
    pub primary_type_display_name: Option<GoogleTypeLocalizedText>,
    /// Indicates whether the place is a pure service area business. Pure service area business is a business that visits or delivers to customers directly but does not serve customers at their business address. For example, businesses like cleaning services or plumbers. Those businesses may not have a physical address or location on Google Maps.
    #[serde(rename = "pureServiceAreaBusiness")]
    pub pure_service_area_business: Option<bool>,
    /// A rating between 1.0 and 5.0, based on user reviews of this place.
    pub rating: Option<f64>,
    /// The regular hours of operation. Note that if a place is always open (24 hours), the `close` field will not be set. Clients can rely on always open (24 hours) being represented as an [`open`](https://developers.google.com/maps/documentation/places/web-service/reference/rest/v1/places#Period) period containing [`day`](https://developers.google.com/maps/documentation/places/web-service/reference/rest/v1/places#Point) with value `0`, [`hour`](https://developers.google.com/maps/documentation/places/web-service/reference/rest/v1/places#Point) with value `0`, and [`minute`](https://developers.google.com/maps/documentation/places/web-service/reference/rest/v1/places#Point) with value `0`.
    #[serde(rename = "regularOpeningHours")]
    pub regular_opening_hours: Option<GoogleMapsPlacesV1PlaceOpeningHours>,
    /// Contains an array of entries for information about regular secondary hours of a business. Secondary hours are different from a business's main hours. For example, a restaurant can specify drive through hours or delivery hours as its secondary hours. This field populates the type subfield, which draws from a predefined list of opening hours types (such as DRIVE_THROUGH, PICKUP, or TAKEOUT) based on the types of the place.
    #[serde(rename = "regularSecondaryOpeningHours")]
    pub regular_secondary_opening_hours: Option<Vec<GoogleMapsPlacesV1PlaceOpeningHours>>,
    /// Specifies if the place supports reservations.
    pub reservable: Option<bool>,
    /// Place has restroom.
    pub restroom: Option<bool>,
    /// AI-generated summary of the place using user reviews.
    #[serde(rename = "reviewSummary")]
    pub review_summary: Option<GoogleMapsPlacesV1PlaceReviewSummary>,
    /// List of reviews about this place, sorted by relevance. A maximum of 5 reviews can be returned.
    pub reviews: Option<Vec<GoogleMapsPlacesV1Review>>,
    /// Specifies if the place serves beer.
    #[serde(rename = "servesBeer")]
    pub serves_beer: Option<bool>,
    /// Specifies if the place serves breakfast.
    #[serde(rename = "servesBreakfast")]
    pub serves_breakfast: Option<bool>,
    /// Specifies if the place serves brunch.
    #[serde(rename = "servesBrunch")]
    pub serves_brunch: Option<bool>,
    /// Place serves cocktails.
    #[serde(rename = "servesCocktails")]
    pub serves_cocktails: Option<bool>,
    /// Place serves coffee.
    #[serde(rename = "servesCoffee")]
    pub serves_coffee: Option<bool>,
    /// Place serves dessert.
    #[serde(rename = "servesDessert")]
    pub serves_dessert: Option<bool>,
    /// Specifies if the place serves dinner.
    #[serde(rename = "servesDinner")]
    pub serves_dinner: Option<bool>,
    /// Specifies if the place serves lunch.
    #[serde(rename = "servesLunch")]
    pub serves_lunch: Option<bool>,
    /// Specifies if the place serves vegetarian food.
    #[serde(rename = "servesVegetarianFood")]
    pub serves_vegetarian_food: Option<bool>,
    /// Specifies if the place serves wine.
    #[serde(rename = "servesWine")]
    pub serves_wine: Option<bool>,
    /// A short, human-readable address for this place.
    #[serde(rename = "shortFormattedAddress")]
    pub short_formatted_address: Option<String>,
    /// A list of sub-destinations related to the place.
    #[serde(rename = "subDestinations")]
    pub sub_destinations: Option<Vec<GoogleMapsPlacesV1PlaceSubDestination>>,
    /// Specifies if the business supports takeout.
    pub takeout: Option<bool>,
    /// IANA Time Zone Database time zone. For example "America/New_York".
    #[serde(rename = "timeZone")]
    pub time_zone: Option<GoogleTypeTimeZone>,
    /// A set of type tags for this result. For example, "political" and "locality". For the complete list of possible values, see Table A and Table B at https://developers.google.com/maps/documentation/places/web-service/place-types
    pub types: Option<Vec<String>>,
    /// The total number of reviews (with or without text) for this place.
    #[serde(rename = "userRatingCount")]
    pub user_rating_count: Option<i32>,
    /// Number of minutes this place's timezone is currently offset from UTC. This is expressed in minutes to support timezones that are offset by fractions of an hour, e.g. X hours and 15 minutes.
    #[serde(rename = "utcOffsetMinutes")]
    pub utc_offset_minutes: Option<i32>,
    /// A viewport suitable for displaying the place on an average-sized map. This viewport should not be used as the physical boundary or the service area of the business.
    pub viewport: Option<GoogleGeoTypeViewport>,
    /// The authoritative website for this place, e.g. a business' homepage. Note that for places that are part of a chain (e.g. an IKEA store), this will usually be the website for the individual store, not the overall chain.
    #[serde(rename = "websiteUri")]
    pub website_uri: Option<String>,
}

impl common::ResponseResult for GoogleMapsPlacesV1Place {}

/// Information about the accessibility options a place offers.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1PlaceAccessibilityOptions {
    /// Places has wheelchair accessible entrance.
    #[serde(rename = "wheelchairAccessibleEntrance")]
    pub wheelchair_accessible_entrance: Option<bool>,
    /// Place offers wheelchair accessible parking.
    #[serde(rename = "wheelchairAccessibleParking")]
    pub wheelchair_accessible_parking: Option<bool>,
    /// Place has wheelchair accessible restroom.
    #[serde(rename = "wheelchairAccessibleRestroom")]
    pub wheelchair_accessible_restroom: Option<bool>,
    /// Place has wheelchair accessible seating.
    #[serde(rename = "wheelchairAccessibleSeating")]
    pub wheelchair_accessible_seating: Option<bool>,
}

impl common::Part for GoogleMapsPlacesV1PlaceAccessibilityOptions {}

/// The structured components that form the formatted address, if this information is available.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1PlaceAddressComponent {
    /// The language used to format this components, in CLDR notation.
    #[serde(rename = "languageCode")]
    pub language_code: Option<String>,
    /// The full text description or name of the address component. For example, an address component for the country Australia may have a long_name of "Australia".
    #[serde(rename = "longText")]
    pub long_text: Option<String>,
    /// An abbreviated textual name for the address component, if available. For example, an address component for the country of Australia may have a short_name of "AU".
    #[serde(rename = "shortText")]
    pub short_text: Option<String>,
    /// An array indicating the type(s) of the address component.
    pub types: Option<Vec<String>>,
}

impl common::Part for GoogleMapsPlacesV1PlaceAddressComponent {}

/// Information about data providers of this place.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1PlaceAttribution {
    /// Name of the Place's data provider.
    pub provider: Option<String>,
    /// URI to the Place's data provider.
    #[serde(rename = "providerUri")]
    pub provider_uri: Option<String>,
}

impl common::Part for GoogleMapsPlacesV1PlaceAttribution {}

/// The consumer alert message for the place when we detect suspicious review activity on a business or a business violates our policies.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1PlaceConsumerAlert {
    /// The details of the consumer alert message.
    pub details: Option<GoogleMapsPlacesV1PlaceConsumerAlertDetails>,
    /// The language code of the consumer alert message. This is a BCP 47 language code.
    #[serde(rename = "languageCode")]
    pub language_code: Option<String>,
    /// The overview of the consumer alert message.
    pub overview: Option<String>,
}

impl common::Part for GoogleMapsPlacesV1PlaceConsumerAlert {}

/// The details of the consumer alert message.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1PlaceConsumerAlertDetails {
    /// The link to show together with the description to provide more information.
    #[serde(rename = "aboutLink")]
    pub about_link: Option<GoogleMapsPlacesV1PlaceConsumerAlertDetailsLink>,
    /// The description of the consumer alert message.
    pub description: Option<String>,
    /// The title to show together with the description.
    pub title: Option<String>,
}

impl common::Part for GoogleMapsPlacesV1PlaceConsumerAlertDetails {}

/// The link to show together with the description to provide more information.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1PlaceConsumerAlertDetailsLink {
    /// The title to show for the link.
    pub title: Option<String>,
    /// The uri of the link.
    pub uri: Option<String>,
}

impl common::Part for GoogleMapsPlacesV1PlaceConsumerAlertDetailsLink {}

/// Info about the place in which this place is located.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1PlaceContainingPlace {
    /// The place id of the place in which this place is located.
    pub id: Option<String>,
    /// The resource name of the place in which this place is located.
    pub name: Option<String>,
}

impl common::Part for GoogleMapsPlacesV1PlaceContainingPlace {}

/// The summary of amenities near the EV charging station. This only applies to places with type `electric_vehicle_charging_station`. The `overview` field is guaranteed to be provided while the other fields are optional.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1PlaceEvChargeAmenitySummary {
    /// A summary of the nearby coffee options.
    pub coffee: Option<GoogleMapsPlacesV1ContentBlock>,
    /// The AI disclosure message "Summarized with Gemini" (and its localized variants). This will be in the language specified in the request if available.
    #[serde(rename = "disclosureText")]
    pub disclosure_text: Option<GoogleTypeLocalizedText>,
    /// A link where users can flag a problem with the summary.
    #[serde(rename = "flagContentUri")]
    pub flag_content_uri: Option<String>,
    /// An overview of the available amenities. This is guaranteed to be provided.
    pub overview: Option<GoogleMapsPlacesV1ContentBlock>,
    /// A summary of the nearby restaurants.
    pub restaurant: Option<GoogleMapsPlacesV1ContentBlock>,
    /// A summary of the nearby stores.
    pub store: Option<GoogleMapsPlacesV1ContentBlock>,
}

impl common::Part for GoogleMapsPlacesV1PlaceEvChargeAmenitySummary {}

/// AI-generated summary of the place.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1PlaceGenerativeSummary {
    /// The AI disclosure message "Summarized with Gemini" (and its localized variants). This will be in the language specified in the request if available.
    #[serde(rename = "disclosureText")]
    pub disclosure_text: Option<GoogleTypeLocalizedText>,
    /// The overview of the place.
    pub overview: Option<GoogleTypeLocalizedText>,
    /// A link where users can flag a problem with the overview summary.
    #[serde(rename = "overviewFlagContentUri")]
    pub overview_flag_content_uri: Option<String>,
}

impl common::Part for GoogleMapsPlacesV1PlaceGenerativeSummary {}

/// Links to trigger different Google Maps actions.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1PlaceGoogleMapsLinks {
    /// A link to show the directions to the place. The link only populates the destination location and uses the default travel mode `DRIVE`.
    #[serde(rename = "directionsUri")]
    pub directions_uri: Option<String>,
    /// A link to show photos of this place on Google Maps.
    #[serde(rename = "photosUri")]
    pub photos_uri: Option<String>,
    /// A link to show this place.
    #[serde(rename = "placeUri")]
    pub place_uri: Option<String>,
    /// A link to show reviews of this place on Google Maps.
    #[serde(rename = "reviewsUri")]
    pub reviews_uri: Option<String>,
    /// A link to write a review for this place on Google Maps.
    #[serde(rename = "writeAReviewUri")]
    pub write_a_review_uri: Option<String>,
}

impl common::Part for GoogleMapsPlacesV1PlaceGoogleMapsLinks {}

/// A summary of points of interest near the place.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1PlaceNeighborhoodSummary {
    /// A detailed description of the neighborhood.
    pub description: Option<GoogleMapsPlacesV1ContentBlock>,
    /// The AI disclosure message "Summarized with Gemini" (and its localized variants). This will be in the language specified in the request if available.
    #[serde(rename = "disclosureText")]
    pub disclosure_text: Option<GoogleTypeLocalizedText>,
    /// A link where users can flag a problem with the summary.
    #[serde(rename = "flagContentUri")]
    pub flag_content_uri: Option<String>,
    /// An overview summary of the neighborhood.
    pub overview: Option<GoogleMapsPlacesV1ContentBlock>,
}

impl common::Part for GoogleMapsPlacesV1PlaceNeighborhoodSummary {}

/// Information about business hour of the place.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1PlaceOpeningHours {
    /// The next time the current opening hours period ends up to 7 days in the future. This field is only populated if the opening hours period is active at the time of serving the request.
    #[serde(rename = "nextCloseTime")]
    pub next_close_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// The next time the current opening hours period starts up to 7 days in the future. This field is only populated if the opening hours period is not active at the time of serving the request.
    #[serde(rename = "nextOpenTime")]
    pub next_open_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Whether the opening hours period is currently active. For regular opening hours and current opening hours, this field means whether the place is open. For secondary opening hours and current secondary opening hours, this field means whether the secondary hours of this place is active.
    #[serde(rename = "openNow")]
    pub open_now: Option<bool>,
    /// The periods that this place is open during the week. The periods are in chronological order, in the place-local timezone. An empty (but not absent) value indicates a place that is never open, e.g. because it is closed temporarily for renovations. The starting day of `periods` is NOT fixed and should not be assumed to be Sunday. The API determines the start day based on a variety of factors. For example, for a 24/7 business, the first period may begin on the day of the request. For other businesses, it might be the first day of the week that they are open. NOTE: The ordering of the `periods` array is independent of the ordering of the `weekday_descriptions` array. Do not assume they will begin on the same day.
    pub periods: Option<Vec<GoogleMapsPlacesV1PlaceOpeningHoursPeriod>>,
    /// A type string used to identify the type of secondary hours.
    #[serde(rename = "secondaryHoursType")]
    pub secondary_hours_type: Option<String>,
    /// Structured information for special days that fall within the period that the returned opening hours cover. Special days are days that could impact the business hours of a place, e.g. Christmas day. Set for current_opening_hours and current_secondary_opening_hours if there are exceptional hours.
    #[serde(rename = "specialDays")]
    pub special_days: Option<Vec<GoogleMapsPlacesV1PlaceOpeningHoursSpecialDay>>,
    /// Localized strings describing the opening hours of this place, one string for each day of the week. NOTE: The order of the days and the start of the week is determined by the locale (language and region). The ordering of the `periods` array is independent of the ordering of the `weekday_descriptions` array. Do not assume they will begin on the same day. Will be empty if the hours are unknown or could not be converted to localized text. Example: "Sun: 18:00–06:00"
    #[serde(rename = "weekdayDescriptions")]
    pub weekday_descriptions: Option<Vec<String>>,
}

impl common::Part for GoogleMapsPlacesV1PlaceOpeningHours {}

/// A period the place remains in open_now status.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1PlaceOpeningHoursPeriod {
    /// The time that the place starts to be closed.
    pub close: Option<GoogleMapsPlacesV1PlaceOpeningHoursPeriodPoint>,
    /// The time that the place starts to be open.
    pub open: Option<GoogleMapsPlacesV1PlaceOpeningHoursPeriodPoint>,
}

impl common::Part for GoogleMapsPlacesV1PlaceOpeningHoursPeriod {}

/// Status changing points.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1PlaceOpeningHoursPeriodPoint {
    /// Date in the local timezone for the place.
    pub date: Option<GoogleTypeDate>,
    /// A day of the week, as an integer in the range 0-6. 0 is Sunday, 1 is Monday, etc.
    pub day: Option<i32>,
    /// The hour in 24 hour format. Ranges from 0 to 23.
    pub hour: Option<i32>,
    /// The minute. Ranges from 0 to 59.
    pub minute: Option<i32>,
    /// Whether or not this endpoint was truncated. Truncation occurs when the real hours are outside the times we are willing to return hours between, so we truncate the hours back to these boundaries. This ensures that at most 24 * 7 hours from midnight of the day of the request are returned.
    pub truncated: Option<bool>,
}

impl common::Part for GoogleMapsPlacesV1PlaceOpeningHoursPeriodPoint {}

/// Structured information for special days that fall within the period that the returned opening hours cover. Special days are days that could impact the business hours of a place, e.g. Christmas day.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1PlaceOpeningHoursSpecialDay {
    /// The date of this special day.
    pub date: Option<GoogleTypeDate>,
}

impl common::Part for GoogleMapsPlacesV1PlaceOpeningHoursSpecialDay {}

/// Information about parking options for the place. A parking lot could support more than one option at the same time.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1PlaceParkingOptions {
    /// Place offers free garage parking.
    #[serde(rename = "freeGarageParking")]
    pub free_garage_parking: Option<bool>,
    /// Place offers free parking lots.
    #[serde(rename = "freeParkingLot")]
    pub free_parking_lot: Option<bool>,
    /// Place offers free street parking.
    #[serde(rename = "freeStreetParking")]
    pub free_street_parking: Option<bool>,
    /// Place offers paid garage parking.
    #[serde(rename = "paidGarageParking")]
    pub paid_garage_parking: Option<bool>,
    /// Place offers paid parking lots.
    #[serde(rename = "paidParkingLot")]
    pub paid_parking_lot: Option<bool>,
    /// Place offers paid street parking.
    #[serde(rename = "paidStreetParking")]
    pub paid_street_parking: Option<bool>,
    /// Place offers valet parking.
    #[serde(rename = "valetParking")]
    pub valet_parking: Option<bool>,
}

impl common::Part for GoogleMapsPlacesV1PlaceParkingOptions {}

/// Payment options the place accepts.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1PlacePaymentOptions {
    /// Place accepts cash only as payment. Places with this attribute may still accept other payment methods.
    #[serde(rename = "acceptsCashOnly")]
    pub accepts_cash_only: Option<bool>,
    /// Place accepts credit cards as payment.
    #[serde(rename = "acceptsCreditCards")]
    pub accepts_credit_cards: Option<bool>,
    /// Place accepts debit cards as payment.
    #[serde(rename = "acceptsDebitCards")]
    pub accepts_debit_cards: Option<bool>,
    /// Place accepts NFC payments.
    #[serde(rename = "acceptsNfc")]
    pub accepts_nfc: Option<bool>,
}

impl common::Part for GoogleMapsPlacesV1PlacePaymentOptions {}

/// Plus code (http://plus.codes) is a location reference with two formats: global code defining a 14mx14m (1/8000th of a degree) or smaller rectangle, and compound code, replacing the prefix with a reference location.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1PlacePlusCode {
    /// Place's compound code, such as "33GV+HQ, Ramberg, Norway", containing the suffix of the global code and replacing the prefix with a formatted name of a reference entity.
    #[serde(rename = "compoundCode")]
    pub compound_code: Option<String>,
    /// Place's global (full) code, such as "9FWM33GV+HQ", representing an 1/8000 by 1/8000 degree area (~14 by 14 meters).
    #[serde(rename = "globalCode")]
    pub global_code: Option<String>,
}

impl common::Part for GoogleMapsPlacesV1PlacePlusCode {}

/// AI-generated summary of the place using user reviews.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1PlaceReviewSummary {
    /// The AI disclosure message "Summarized with Gemini" (and its localized variants). This will be in the language specified in the request if available.
    #[serde(rename = "disclosureText")]
    pub disclosure_text: Option<GoogleTypeLocalizedText>,
    /// A link where users can flag a problem with the summary.
    #[serde(rename = "flagContentUri")]
    pub flag_content_uri: Option<String>,
    /// A link to show reviews of this place on Google Maps.
    #[serde(rename = "reviewsUri")]
    pub reviews_uri: Option<String>,
    /// The summary of user reviews.
    pub text: Option<GoogleTypeLocalizedText>,
}

impl common::Part for GoogleMapsPlacesV1PlaceReviewSummary {}

/// Sub-destinations are specific places associated with a main place. These provide more specific destinations for users who are searching within a large or complex place, like an airport, national park, university, or stadium. For example, sub-destinations at an airport might include associated terminals and parking lots. Sub-destinations return the place ID and place resource name, which can be used in subsequent Place Details (New) requests to fetch richer details, including the sub-destination's display name and location.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1PlaceSubDestination {
    /// The place id of the sub-destination.
    pub id: Option<String>,
    /// The resource name of the sub-destination.
    pub name: Option<String>,
}

impl common::Part for GoogleMapsPlacesV1PlaceSubDestination {}

/// A route polyline. Only supports an [encoded polyline](https://developers.google.com/maps/documentation/utilities/polylinealgorithm), which can be passed as a string and includes compression with minimal lossiness. This is the Routes API default output.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1Polyline {
    /// An [encoded polyline](https://developers.google.com/maps/documentation/utilities/polylinealgorithm), as returned by the [Routes API by default](https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes#polylineencoding). See the [encoder](https://developers.google.com/maps/documentation/utilities/polylineutility) and [decoder](https://developers.google.com/maps/documentation/routes/polylinedecoder) tools.
    #[serde(rename = "encodedPolyline")]
    pub encoded_polyline: Option<String>,
}

impl common::Part for GoogleMapsPlacesV1Polyline {}

/// The price range associated with a Place. `end_price` could be unset, which indicates a range without upper bound (e.g. "More than $100").
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1PriceRange {
    /// The high end of the price range (exclusive). Price should be lower than this amount.
    #[serde(rename = "endPrice")]
    pub end_price: Option<GoogleTypeMoney>,
    /// The low end of the price range (inclusive). Price should be at or above this amount.
    #[serde(rename = "startPrice")]
    pub start_price: Option<GoogleTypeMoney>,
}

impl common::Part for GoogleMapsPlacesV1PriceRange {}

/// Information about a review of a place.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1Review {
    /// This review's author.
    #[serde(rename = "authorAttribution")]
    pub author_attribution: Option<GoogleMapsPlacesV1AuthorAttribution>,
    /// A link where users can flag a problem with the review.
    #[serde(rename = "flagContentUri")]
    pub flag_content_uri: Option<String>,
    /// A link to show the review on Google Maps.
    #[serde(rename = "googleMapsUri")]
    pub google_maps_uri: Option<String>,
    /// A reference representing this place review which may be used to look up this place review again (also called the API "resource" name: `places/{place_id}/reviews/{review}`).
    pub name: Option<String>,
    /// The review text in its original language.
    #[serde(rename = "originalText")]
    pub original_text: Option<GoogleTypeLocalizedText>,
    /// Timestamp for the review.
    #[serde(rename = "publishTime")]
    pub publish_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// A number between 1.0 and 5.0, also called the number of stars.
    pub rating: Option<f64>,
    /// A string of formatted recent time, expressing the review time relative to the current time in a form appropriate for the language and country.
    #[serde(rename = "relativePublishTimeDescription")]
    pub relative_publish_time_description: Option<String>,
    /// The localized text of the review.
    pub text: Option<GoogleTypeLocalizedText>,
    /// The date when the author visited the place. This is truncated to the year and month of the visit.
    #[serde(rename = "visitDate")]
    pub visit_date: Option<GoogleTypeDate>,
}

impl common::Part for GoogleMapsPlacesV1Review {}

/// Encapsulates a set of optional conditions to satisfy when calculating the routes.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1RouteModifiers {
    /// Optional. When set to true, avoids ferries where reasonable, giving preference to routes not containing ferries. Applies only to the `DRIVE` and `TWO_WHEELER` `TravelMode`.
    #[serde(rename = "avoidFerries")]
    pub avoid_ferries: Option<bool>,
    /// Optional. When set to true, avoids highways where reasonable, giving preference to routes not containing highways. Applies only to the `DRIVE` and `TWO_WHEELER` `TravelMode`.
    #[serde(rename = "avoidHighways")]
    pub avoid_highways: Option<bool>,
    /// Optional. When set to true, avoids navigating indoors where reasonable, giving preference to routes not containing indoor navigation. Applies only to the `WALK` `TravelMode`.
    #[serde(rename = "avoidIndoor")]
    pub avoid_indoor: Option<bool>,
    /// Optional. When set to true, avoids toll roads where reasonable, giving preference to routes not containing toll roads. Applies only to the `DRIVE` and `TWO_WHEELER` `TravelMode`.
    #[serde(rename = "avoidTolls")]
    pub avoid_tolls: Option<bool>,
}

impl common::Part for GoogleMapsPlacesV1RouteModifiers {}

/// Parameters to configure the routing calculations to the places in the response, both along a route (where result ranking will be influenced) and for calculating travel times on results.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1RoutingParameters {
    /// Optional. An explicit routing origin that overrides the origin defined in the polyline. By default, the polyline origin is used.
    pub origin: Option<GoogleTypeLatLng>,
    /// Optional. The route modifiers.
    #[serde(rename = "routeModifiers")]
    pub route_modifiers: Option<GoogleMapsPlacesV1RouteModifiers>,
    /// Optional. Specifies how to compute the routing summaries. The server attempts to use the selected routing preference to compute the route. The traffic aware routing preference is only available for the `DRIVE` or `TWO_WHEELER` `travelMode`.
    #[serde(rename = "routingPreference")]
    pub routing_preference: Option<String>,
    /// Optional. The travel mode.
    #[serde(rename = "travelMode")]
    pub travel_mode: Option<String>,
}

impl common::Part for GoogleMapsPlacesV1RoutingParameters {}

/// The duration and distance from the routing origin to a place in the response, and a second leg from that place to the destination, if requested. **Note:** Adding `routingSummaries` in the field mask without also including either the `routingParameters.origin` parameter or the `searchAlongRouteParameters.polyline.encodedPolyline` parameter in the request causes an error.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1RoutingSummary {
    /// A link to show directions on Google Maps using the waypoints from the given routing summary. The route generated by this link is not guaranteed to be the same as the route used to generate the routing summary. The link uses information provided in the request, from fields including `routingParameters` and `searchAlongRouteParameters` when applicable, to generate the directions link.
    #[serde(rename = "directionsUri")]
    pub directions_uri: Option<String>,
    /// The legs of the trip. When you calculate travel duration and distance from a set origin, `legs` contains a single leg containing the duration and distance from the origin to the destination. When you do a search along route, `legs` contains two legs: one from the origin to place, and one from the place to the destination.
    pub legs: Option<Vec<GoogleMapsPlacesV1RoutingSummaryLeg>>,
}

impl common::Part for GoogleMapsPlacesV1RoutingSummary {}

/// A leg is a single portion of a journey from one location to another.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1RoutingSummaryLeg {
    /// The distance of this leg of the trip.
    #[serde(rename = "distanceMeters")]
    pub distance_meters: Option<i32>,
    /// The time it takes to complete this leg of the trip.
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub duration: Option<chrono::Duration>,
}

impl common::Part for GoogleMapsPlacesV1RoutingSummaryLeg {}

/// Request proto for Search Nearby.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [search nearby places](PlaceSearchNearbyCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1SearchNearbyRequest {
    /// Excluded primary Place type (e.g. "restaurant" or "gas_station") from https://developers.google.com/maps/documentation/places/web-service/place-types. Up to 50 types from [Table A](https://developers.google.com/maps/documentation/places/web-service/place-types#table-a) may be specified. If there are any conflicting primary types, i.e. a type appears in both included_primary_types and excluded_primary_types, an INVALID_ARGUMENT error is returned. If a Place type is specified with multiple type restrictions, only places that satisfy all of the restrictions are returned. For example, if we have {included_types = ["restaurant"], excluded_primary_types = ["restaurant"]}, the returned places provide "restaurant" related services but do not operate primarily as "restaurants".
    #[serde(rename = "excludedPrimaryTypes")]
    pub excluded_primary_types: Option<Vec<String>>,
    /// Excluded Place type (eg, "restaurant" or "gas_station") from https://developers.google.com/maps/documentation/places/web-service/place-types. Up to 50 types from [Table A](https://developers.google.com/maps/documentation/places/web-service/place-types#table-a) may be specified. If the client provides both included_types (e.g. restaurant) and excluded_types (e.g. cafe), then the response should include places that are restaurant but not cafe. The response includes places that match at least one of the included_types and none of the excluded_types. If there are any conflicting types, i.e. a type appears in both included_types and excluded_types, an INVALID_ARGUMENT error is returned. If a Place type is specified with multiple type restrictions, only places that satisfy all of the restrictions are returned. For example, if we have {included_types = ["restaurant"], excluded_primary_types = ["restaurant"]}, the returned places provide "restaurant" related services but do not operate primarily as "restaurants".
    #[serde(rename = "excludedTypes")]
    pub excluded_types: Option<Vec<String>>,
    /// Included primary Place type (e.g. "restaurant" or "gas_station") from https://developers.google.com/maps/documentation/places/web-service/place-types. A place can only have a single primary type from the supported types table associated with it. Up to 50 types from [Table A](https://developers.google.com/maps/documentation/places/web-service/place-types#table-a) may be specified. If there are any conflicting primary types, i.e. a type appears in both included_primary_types and excluded_primary_types, an INVALID_ARGUMENT error is returned. If a Place type is specified with multiple type restrictions, only places that satisfy all of the restrictions are returned. For example, if we have {included_types = ["restaurant"], excluded_primary_types = ["restaurant"]}, the returned places provide "restaurant" related services but do not operate primarily as "restaurants".
    #[serde(rename = "includedPrimaryTypes")]
    pub included_primary_types: Option<Vec<String>>,
    /// Included Place type (eg, "restaurant" or "gas_station") from https://developers.google.com/maps/documentation/places/web-service/place-types. Up to 50 types from [Table A](https://developers.google.com/maps/documentation/places/web-service/place-types#table-a) may be specified. If there are any conflicting types, i.e. a type appears in both included_types and excluded_types, an INVALID_ARGUMENT error is returned. If a Place type is specified with multiple type restrictions, only places that satisfy all of the restrictions are returned. For example, if we have {included_types = ["restaurant"], excluded_primary_types = ["restaurant"]}, the returned places provide "restaurant" related services but do not operate primarily as "restaurants".
    #[serde(rename = "includedTypes")]
    pub included_types: Option<Vec<String>>,
    /// Place details will be displayed with the preferred language if available. If the language code is unspecified or unrecognized, place details of any language may be returned, with a preference for English if such details exist. Current list of supported languages: https://developers.google.com/maps/faq#languagesupport.
    #[serde(rename = "languageCode")]
    pub language_code: Option<String>,
    /// Required. The region to search.
    #[serde(rename = "locationRestriction")]
    pub location_restriction: Option<GoogleMapsPlacesV1SearchNearbyRequestLocationRestriction>,
    /// Maximum number of results to return. It must be between 1 and 20 (default), inclusively. If the number is unset, it falls back to the upper limit. If the number is set to negative or exceeds the upper limit, an INVALID_ARGUMENT error is returned.
    #[serde(rename = "maxResultCount")]
    pub max_result_count: Option<i32>,
    /// How results will be ranked in the response.
    #[serde(rename = "rankPreference")]
    pub rank_preference: Option<String>,
    /// The Unicode country/region code (CLDR) of the location where the request is coming from. This parameter is used to display the place details, like region-specific place name, if available. The parameter can affect results based on applicable law. For more information, see https://www.unicode.org/cldr/charts/latest/supplemental/territory_language_information.html. Note that 3-digit region codes are not currently supported.
    #[serde(rename = "regionCode")]
    pub region_code: Option<String>,
    /// Optional. Parameters that affect the routing to the search results.
    #[serde(rename = "routingParameters")]
    pub routing_parameters: Option<GoogleMapsPlacesV1RoutingParameters>,
}

impl common::RequestValue for GoogleMapsPlacesV1SearchNearbyRequest {}

/// The region to search.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1SearchNearbyRequestLocationRestriction {
    /// A circle defined by center point and radius.
    pub circle: Option<GoogleMapsPlacesV1Circle>,
}

impl common::Part for GoogleMapsPlacesV1SearchNearbyRequestLocationRestriction {}

/// Response proto for Search Nearby.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [search nearby places](PlaceSearchNearbyCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1SearchNearbyResponse {
    /// A list of places that meets user's requirements like places types, number of places and specific location restriction.
    pub places: Option<Vec<GoogleMapsPlacesV1Place>>,
    /// A list of routing summaries where each entry associates to the corresponding place in the same index in the `places` field. If the routing summary is not available for one of the places, it will contain an empty entry. This list should have as many entries as the list of places if requested.
    #[serde(rename = "routingSummaries")]
    pub routing_summaries: Option<Vec<GoogleMapsPlacesV1RoutingSummary>>,
}

impl common::ResponseResult for GoogleMapsPlacesV1SearchNearbyResponse {}

/// Request proto for SearchText.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [search text places](PlaceSearchTextCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1SearchTextRequest {
    /// Optional. Set the searchable EV options of a place search request.
    #[serde(rename = "evOptions")]
    pub ev_options: Option<GoogleMapsPlacesV1SearchTextRequestEVOptions>,
    /// Optional. Include pure service area businesses if the field is set to true. Pure service area business is a business that visits or delivers to customers directly but does not serve customers at their business address. For example, businesses like cleaning services or plumbers. Those businesses do not have a physical address or location on Google Maps. Places will not return fields including `location`, `plus_code`, and other location related fields for these businesses.
    #[serde(rename = "includePureServiceAreaBusinesses")]
    pub include_pure_service_area_businesses: Option<bool>,
    /// The requested place type. Full list of types supported: https://developers.google.com/maps/documentation/places/web-service/place-types. Only support one included type.
    #[serde(rename = "includedType")]
    pub included_type: Option<String>,
    /// Place details will be displayed with the preferred language if available. If the language code is unspecified or unrecognized, place details of any language may be returned, with a preference for English if such details exist. Current list of supported languages: https://developers.google.com/maps/faq#languagesupport.
    #[serde(rename = "languageCode")]
    pub language_code: Option<String>,
    /// The region to search. This location serves as a bias which means results around given location might be returned. Cannot be set along with location_restriction.
    #[serde(rename = "locationBias")]
    pub location_bias: Option<GoogleMapsPlacesV1SearchTextRequestLocationBias>,
    /// The region to search. This location serves as a restriction which means results outside given location will not be returned. Cannot be set along with location_bias.
    #[serde(rename = "locationRestriction")]
    pub location_restriction: Option<GoogleMapsPlacesV1SearchTextRequestLocationRestriction>,
    /// Deprecated: Use `page_size` instead. The maximum number of results per page that can be returned. If the number of available results is larger than `max_result_count`, a `next_page_token` is returned which can be passed to `page_token` to get the next page of results in subsequent requests. If 0 or no value is provided, a default of 20 is used. The maximum value is 20; values above 20 will be coerced to 20. Negative values will return an INVALID_ARGUMENT error. If both `max_result_count` and `page_size` are specified, `max_result_count` will be ignored.
    #[serde(rename = "maxResultCount")]
    pub max_result_count: Option<i32>,
    /// Filter out results whose average user rating is strictly less than this limit. A valid value must be a float between 0 and 5 (inclusively) at a 0.5 cadence i.e. [0, 0.5, 1.0, ... , 5.0] inclusively. The input rating will round up to the nearest 0.5(ceiling). For instance, a rating of 0.6 will eliminate all results with a less than 1.0 rating.
    #[serde(rename = "minRating")]
    pub min_rating: Option<f64>,
    /// Used to restrict the search to places that are currently open. The default is false.
    #[serde(rename = "openNow")]
    pub open_now: Option<bool>,
    /// Optional. The maximum number of results per page that can be returned. If the number of available results is larger than `page_size`, a `next_page_token` is returned which can be passed to `page_token` to get the next page of results in subsequent requests. If 0 or no value is provided, a default of 20 is used. The maximum value is 20; values above 20 will be set to 20. Negative values will return an INVALID_ARGUMENT error. If both `max_result_count` and `page_size` are specified, `max_result_count` will be ignored.
    #[serde(rename = "pageSize")]
    pub page_size: Option<i32>,
    /// Optional. A page token, received from a previous TextSearch call. Provide this to retrieve the subsequent page. When paginating, all parameters other than `page_token`, `page_size`, and `max_result_count` provided to TextSearch must match the initial call that provided the page token. Otherwise an INVALID_ARGUMENT error is returned.
    #[serde(rename = "pageToken")]
    pub page_token: Option<String>,
    /// Used to restrict the search to places that are marked as certain price levels. Users can choose any combinations of price levels. Default to select all price levels.
    #[serde(rename = "priceLevels")]
    pub price_levels: Option<Vec<String>>,
    /// How results will be ranked in the response.
    #[serde(rename = "rankPreference")]
    pub rank_preference: Option<String>,
    /// The Unicode country/region code (CLDR) of the location where the request is coming from. This parameter is used to display the place details, like region-specific place name, if available. The parameter can affect results based on applicable law. For more information, see https://www.unicode.org/cldr/charts/latest/supplemental/territory_language_information.html. Note that 3-digit region codes are not currently supported.
    #[serde(rename = "regionCode")]
    pub region_code: Option<String>,
    /// Optional. Additional parameters for routing to results.
    #[serde(rename = "routingParameters")]
    pub routing_parameters: Option<GoogleMapsPlacesV1RoutingParameters>,
    /// Optional. Additional parameters proto for searching along a route.
    #[serde(rename = "searchAlongRouteParameters")]
    pub search_along_route_parameters:
        Option<GoogleMapsPlacesV1SearchTextRequestSearchAlongRouteParameters>,
    /// Used to set strict type filtering for included_type. If set to true, only results of the same type will be returned. Default to false.
    #[serde(rename = "strictTypeFiltering")]
    pub strict_type_filtering: Option<bool>,
    /// Required. The text query for textual search.
    #[serde(rename = "textQuery")]
    pub text_query: Option<String>,
}

impl common::RequestValue for GoogleMapsPlacesV1SearchTextRequest {}

/// Searchable EV options of a place search request.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1SearchTextRequestEVOptions {
    /// Optional. The list of preferred EV connector types. A place that does not support any of the listed connector types is filtered out.
    #[serde(rename = "connectorTypes")]
    pub connector_types: Option<Vec<String>>,
    /// Optional. Minimum required charging rate in kilowatts. A place with a charging rate less than the specified rate is filtered out.
    #[serde(rename = "minimumChargingRateKw")]
    pub minimum_charging_rate_kw: Option<f64>,
}

impl common::Part for GoogleMapsPlacesV1SearchTextRequestEVOptions {}

/// The region to search. This location serves as a bias which means results around given location might be returned.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1SearchTextRequestLocationBias {
    /// A circle defined by center point and radius.
    pub circle: Option<GoogleMapsPlacesV1Circle>,
    /// A rectangle box defined by northeast and southwest corner. `rectangle.high()` must be the northeast point of the rectangle viewport. `rectangle.low()` must be the southwest point of the rectangle viewport. `rectangle.low().latitude()` cannot be greater than `rectangle.high().latitude()`. This will result in an empty latitude range. A rectangle viewport cannot be wider than 180 degrees.
    pub rectangle: Option<GoogleGeoTypeViewport>,
}

impl common::Part for GoogleMapsPlacesV1SearchTextRequestLocationBias {}

/// The region to search. This location serves as a restriction which means results outside given location will not be returned.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1SearchTextRequestLocationRestriction {
    /// A rectangle box defined by northeast and southwest corner. `rectangle.high()` must be the northeast point of the rectangle viewport. `rectangle.low()` must be the southwest point of the rectangle viewport. `rectangle.low().latitude()` cannot be greater than `rectangle.high().latitude()`. This will result in an empty latitude range. A rectangle viewport cannot be wider than 180 degrees.
    pub rectangle: Option<GoogleGeoTypeViewport>,
}

impl common::Part for GoogleMapsPlacesV1SearchTextRequestLocationRestriction {}

/// Specifies a precalculated polyline from the [Routes API](https://developers.google.com/maps/documentation/routes) defining the route to search. Searching along a route is similar to using the `locationBias` or `locationRestriction` request option to bias the search results. However, while the `locationBias` and `locationRestriction` options let you specify a region to bias the search results, this option lets you bias the results along a trip route. Results are not guaranteed to be along the route provided, but rather are ranked within the search area defined by the polyline and, optionally, by the `locationBias` or `locationRestriction` based on minimal detour times from origin to destination. The results might be along an alternate route, especially if the provided polyline does not define an optimal route from origin to destination.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1SearchTextRequestSearchAlongRouteParameters {
    /// Required. The route polyline.
    pub polyline: Option<GoogleMapsPlacesV1Polyline>,
}

impl common::Part for GoogleMapsPlacesV1SearchTextRequestSearchAlongRouteParameters {}

/// Response proto for SearchText.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [search text places](PlaceSearchTextCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleMapsPlacesV1SearchTextResponse {
    /// Experimental: See https://developers.google.com/maps/documentation/places/web-service/experimental/places-generative for more details. A list of contextual contents where each entry associates to the corresponding place in the same index in the places field. The contents that are relevant to the `text_query` in the request are preferred. If the contextual content is not available for one of the places, it will return non-contextual content. It will be empty only when the content is unavailable for this place. This list will have as many entries as the list of places if requested.
    #[serde(rename = "contextualContents")]
    pub contextual_contents: Option<Vec<GoogleMapsPlacesV1ContextualContent>>,
    /// A token that can be sent as `page_token` to retrieve the next page. If this field is omitted or empty, there are no subsequent pages.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    /// A list of places that meet the user's text search criteria.
    pub places: Option<Vec<GoogleMapsPlacesV1Place>>,
    /// A list of routing summaries where each entry associates to the corresponding place in the same index in the `places` field. If the routing summary is not available for one of the places, it will contain an empty entry. This list will have as many entries as the list of places if requested.
    #[serde(rename = "routingSummaries")]
    pub routing_summaries: Option<Vec<GoogleMapsPlacesV1RoutingSummary>>,
    /// A link allows the user to search with the same text query as specified in the request on Google Maps.
    #[serde(rename = "searchUri")]
    pub search_uri: Option<String>,
}

impl common::ResponseResult for GoogleMapsPlacesV1SearchTextResponse {}

/// Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values. * A month and day, with a zero year (for example, an anniversary). * A year on its own, with a zero month and a zero day. * A year and month, with a zero day (for example, a credit card expiration date). Related types: * google.type.TimeOfDay * google.type.DateTime * google.protobuf.Timestamp
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleTypeDate {
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    pub day: Option<i32>,
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    pub month: Option<i32>,
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    pub year: Option<i32>,
}

impl common::Part for GoogleTypeDate {}

/// An object that represents a latitude/longitude pair. This is expressed as a pair of doubles to represent degrees latitude and degrees longitude. Unless specified otherwise, this object must conform to the WGS84 standard. Values must be within normalized ranges.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleTypeLatLng {
    /// The latitude in degrees. It must be in the range [-90.0, +90.0].
    pub latitude: Option<f64>,
    /// The longitude in degrees. It must be in the range [-180.0, +180.0].
    pub longitude: Option<f64>,
}

impl common::Part for GoogleTypeLatLng {}

/// Localized variant of a text in a particular language.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleTypeLocalizedText {
    /// The text's BCP-47 language code, such as "en-US" or "sr-Latn". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
    #[serde(rename = "languageCode")]
    pub language_code: Option<String>,
    /// Localized string in the language corresponding to language_code below.
    pub text: Option<String>,
}

impl common::Part for GoogleTypeLocalizedText {}

/// Represents an amount of money with its currency type.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleTypeMoney {
    /// The three-letter currency code defined in ISO 4217.
    #[serde(rename = "currencyCode")]
    pub currency_code: Option<String>,
    /// Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    pub nanos: Option<i32>,
    /// The whole units of the amount. For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub units: Option<i64>,
}

impl common::Part for GoogleTypeMoney {}

/// Represents a postal address, such as for postal delivery or payments addresses. With a postal address, a postal service can deliver items to a premise, P.O. box, or similar. A postal address is not intended to model geographical locations like roads, towns, or mountains. In typical usage, an address would be created by user input or from importing existing data, depending on the type of process. Advice on address input or editing: - Use an internationalization-ready address widget such as https://github.com/google/libaddressinput. - Users should not be presented with UI elements for input or editing of fields outside countries where that field is used. For more guidance on how to use this schema, see: https://support.google.com/business/answer/6397478.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleTypePostalAddress {
    /// Unstructured address lines describing the lower levels of an address. Because values in `address_lines` do not have type information and may sometimes contain multiple values in a single field (for example, "Austin, TX"), it is important that the line order is clear. The order of address lines should be "envelope order" for the country or region of the address. In places where this can vary (for example, Japan), `address_language` is used to make it explicit (for example, "ja" for large-to-small ordering and "ja-Latn" or "en" for small-to-large). In this way, the most specific line of an address can be selected based on the language. The minimum permitted structural representation of an address consists of a `region_code` with all remaining information placed in the `address_lines`. It would be possible to format such an address very approximately without geocoding, but no semantic reasoning could be made about any of the address components until it was at least partially resolved. Creating an address only containing a `region_code` and `address_lines` and then geocoding is the recommended way to handle completely unstructured addresses (as opposed to guessing which parts of the address should be localities or administrative areas).
    #[serde(rename = "addressLines")]
    pub address_lines: Option<Vec<String>>,
    /// Optional. Highest administrative subdivision which is used for postal addresses of a country or region. For example, this can be a state, a province, an oblast, or a prefecture. For Spain, this is the province and not the autonomous community (for example, "Barcelona" and not "Catalonia"). Many countries don't use an administrative area in postal addresses. For example, in Switzerland, this should be left unpopulated.
    #[serde(rename = "administrativeArea")]
    pub administrative_area: Option<String>,
    /// Optional. BCP-47 language code of the contents of this address (if known). This is often the UI language of the input form or is expected to match one of the languages used in the address' country/region, or their transliterated equivalents. This can affect formatting in certain countries, but is not critical to the correctness of the data and will never affect any validation or other non-formatting related operations. If this value is not known, it should be omitted (rather than specifying a possibly incorrect default). Examples: "zh-Hant", "ja", "ja-Latn", "en".
    #[serde(rename = "languageCode")]
    pub language_code: Option<String>,
    /// Optional. Generally refers to the city or town portion of the address. Examples: US city, IT comune, UK post town. In regions of the world where localities are not well defined or do not fit into this structure well, leave `locality` empty and use `address_lines`.
    pub locality: Option<String>,
    /// Optional. The name of the organization at the address.
    pub organization: Option<String>,
    /// Optional. Postal code of the address. Not all countries use or require postal codes to be present, but where they are used, they may trigger additional validation with other parts of the address (for example, state or zip code validation in the United States).
    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,
    /// Optional. The recipient at the address. This field may, under certain circumstances, contain multiline information. For example, it might contain "care of" information.
    pub recipients: Option<Vec<String>>,
    /// Required. CLDR region code of the country/region of the address. This is never inferred and it is up to the user to ensure the value is correct. See https://cldr.unicode.org/ and https://www.unicode.org/cldr/charts/30/supplemental/territory_information.html for details. Example: "CH" for Switzerland.
    #[serde(rename = "regionCode")]
    pub region_code: Option<String>,
    /// The schema revision of the `PostalAddress`. This must be set to 0, which is the latest revision. All new revisions **must** be backward compatible with old revisions.
    pub revision: Option<i32>,
    /// Optional. Additional, country-specific, sorting code. This is not used in most regions. Where it is used, the value is either a string like "CEDEX", optionally followed by a number (for example, "CEDEX 7"), or just a number alone, representing the "sector code" (Jamaica), "delivery area indicator" (Malawi) or "post office indicator" (Côte d'Ivoire).
    #[serde(rename = "sortingCode")]
    pub sorting_code: Option<String>,
    /// Optional. Sublocality of the address. For example, this can be a neighborhood, borough, or district.
    pub sublocality: Option<String>,
}

impl common::Part for GoogleTypePostalAddress {}

/// Represents a time zone from the [IANA Time Zone Database](https://www.iana.org/time-zones).
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleTypeTimeZone {
    /// IANA Time Zone Database time zone. For example "America/New_York".
    pub id: Option<String>,
    /// Optional. IANA Time Zone Database version number. For example "2019a".
    pub version: Option<String>,
}

impl common::Part for GoogleTypeTimeZone {}

// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *place* resources.
/// It is not used directly, but through the [`MapsPlaces`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_places1 as places1;
///
/// # async fn dox() {
/// use places1::{MapsPlaces, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let connector = hyper_rustls::HttpsConnectorBuilder::new()
///     .with_native_roots()
///     .unwrap()
///     .https_only()
///     .enable_http2()
///     .build();
///
/// let executor = hyper_util::rt::TokioExecutor::new();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     yup_oauth2::client::CustomHyperClientBuilder::from(
///         hyper_util::client::legacy::Client::builder(executor).build(connector),
///     ),
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http2()
///         .build()
/// );
/// let mut hub = MapsPlaces::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `autocomplete(...)`, `get(...)`, `photos_get_media(...)`, `search_nearby(...)` and `search_text(...)`
/// // to build up your call.
/// let rb = hub.places();
/// # }
/// ```
pub struct PlaceMethods<'a, C>
where
    C: 'a,
{
    hub: &'a MapsPlaces<C>,
}

impl<'a, C> common::MethodsBuilder for PlaceMethods<'a, C> {}

impl<'a, C> PlaceMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Get a photo media with a photo reference string.
    ///
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of a photo media in the format: `places/{place_id}/photos/{photo_reference}/media`. The resource name of a photo as returned in a Place object's `photos.name` field comes with the format `places/{place_id}/photos/{photo_reference}`. You need to append `/media` at the end of the photo resource to get the photo media resource name.
    pub fn photos_get_media(&self, name: &str) -> PlacePhotoGetMediaCall<'a, C> {
        PlacePhotoGetMediaCall {
            hub: self.hub,
            _name: name.to_string(),
            _skip_http_redirect: Default::default(),
            _max_width_px: Default::default(),
            _max_height_px: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Returns predictions for the given input.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn autocomplete(
        &self,
        request: GoogleMapsPlacesV1AutocompletePlacesRequest,
    ) -> PlaceAutocompleteCall<'a, C> {
        PlaceAutocompleteCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Get the details of a place based on its resource name, which is a string in the `places/{place_id}` format.
    ///
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of a place, in the `places/{place_id}` format.
    pub fn get(&self, name: &str) -> PlaceGetCall<'a, C> {
        PlaceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _session_token: Default::default(),
            _region_code: Default::default(),
            _language_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Search for places near locations.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn search_nearby(
        &self,
        request: GoogleMapsPlacesV1SearchNearbyRequest,
    ) -> PlaceSearchNearbyCall<'a, C> {
        PlaceSearchNearbyCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Text query based place search.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn search_text(
        &self,
        request: GoogleMapsPlacesV1SearchTextRequest,
    ) -> PlaceSearchTextCall<'a, C> {
        PlaceSearchTextCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

// ###################
// CallBuilders   ###
// #################

/// Get a photo media with a photo reference string.
///
/// A builder for the *photos.getMedia* method supported by a *place* resource.
/// It is not used directly, but through a [`PlaceMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_places1 as places1;
/// # async fn dox() {
/// # use places1::{MapsPlaces, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = MapsPlaces::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.places().photos_get_media("name")
///              .skip_http_redirect(true)
///              .max_width_px(-51)
///              .max_height_px(-12)
///              .doit().await;
/// # }
/// ```
pub struct PlacePhotoGetMediaCall<'a, C>
where
    C: 'a,
{
    hub: &'a MapsPlaces<C>,
    _name: String,
    _skip_http_redirect: Option<bool>,
    _max_width_px: Option<i32>,
    _max_height_px: Option<i32>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for PlacePhotoGetMediaCall<'a, C> {}

impl<'a, C> PlacePhotoGetMediaCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(
        mut self,
    ) -> common::Result<(common::Response, GoogleMapsPlacesV1PhotoMedia)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "places.places.photos.getMedia",
            http_method: hyper::Method::GET,
        });

        for &field in [
            "alt",
            "name",
            "skipHttpRedirect",
            "maxWidthPx",
            "maxHeightPx",
        ]
        .iter()
        {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("name", self._name);
        if let Some(value) = self._skip_http_redirect.as_ref() {
            params.push("skipHttpRedirect", value.to_string());
        }
        if let Some(value) = self._max_width_px.as_ref() {
            params.push("maxWidthPx", value.to_string());
        }
        if let Some(value) = self._max_height_px.as_ref() {
            params.push("maxHeightPx", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+name}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
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
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// Required. The resource name of a photo media in the format: `places/{place_id}/photos/{photo_reference}/media`. The resource name of a photo as returned in a Place object's `photos.name` field comes with the format `places/{place_id}/photos/{photo_reference}`. You need to append `/media` at the end of the photo resource to get the photo media resource name.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> PlacePhotoGetMediaCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// Optional. If set, skip the default HTTP redirect behavior and render a text format (for example, in JSON format for HTTP use case) response. If not set, an HTTP redirect will be issued to redirect the call to the image media. This option is ignored for non-HTTP requests.
    ///
    /// Sets the *skip http redirect* query property to the given value.
    pub fn skip_http_redirect(mut self, new_value: bool) -> PlacePhotoGetMediaCall<'a, C> {
        self._skip_http_redirect = Some(new_value);
        self
    }
    /// Optional. Specifies the maximum desired width, in pixels, of the image. If the image is smaller than the values specified, the original image will be returned. If the image is larger in either dimension, it will be scaled to match the smaller of the two dimensions, restricted to its original aspect ratio. Both the max_height_px and max_width_px properties accept an integer between 1 and 4800, inclusively. If the value is not within the allowed range, an INVALID_ARGUMENT error will be returned. At least one of max_height_px or max_width_px needs to be specified. If neither max_height_px nor max_width_px is specified, an INVALID_ARGUMENT error will be returned.
    ///
    /// Sets the *max width px* query property to the given value.
    pub fn max_width_px(mut self, new_value: i32) -> PlacePhotoGetMediaCall<'a, C> {
        self._max_width_px = Some(new_value);
        self
    }
    /// Optional. Specifies the maximum desired height, in pixels, of the image. If the image is smaller than the values specified, the original image will be returned. If the image is larger in either dimension, it will be scaled to match the smaller of the two dimensions, restricted to its original aspect ratio. Both the max_height_px and max_width_px properties accept an integer between 1 and 4800, inclusively. If the value is not within the allowed range, an INVALID_ARGUMENT error will be returned. At least one of max_height_px or max_width_px needs to be specified. If neither max_height_px nor max_width_px is specified, an INVALID_ARGUMENT error will be returned.
    ///
    /// Sets the *max height px* query property to the given value.
    pub fn max_height_px(mut self, new_value: i32) -> PlacePhotoGetMediaCall<'a, C> {
        self._max_height_px = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> PlacePhotoGetMediaCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PlacePhotoGetMediaCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PlacePhotoGetMediaCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PlacePhotoGetMediaCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PlacePhotoGetMediaCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Returns predictions for the given input.
///
/// A builder for the *autocomplete* method supported by a *place* resource.
/// It is not used directly, but through a [`PlaceMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_places1 as places1;
/// use places1::api::GoogleMapsPlacesV1AutocompletePlacesRequest;
/// # async fn dox() {
/// # use places1::{MapsPlaces, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = MapsPlaces::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleMapsPlacesV1AutocompletePlacesRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.places().autocomplete(req)
///              .doit().await;
/// # }
/// ```
pub struct PlaceAutocompleteCall<'a, C>
where
    C: 'a,
{
    hub: &'a MapsPlaces<C>,
    _request: GoogleMapsPlacesV1AutocompletePlacesRequest,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for PlaceAutocompleteCall<'a, C> {}

impl<'a, C> PlaceAutocompleteCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(
        mut self,
    ) -> common::Result<(
        common::Response,
        GoogleMapsPlacesV1AutocompletePlacesResponse,
    )> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "places.places.autocomplete",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/places:autocomplete";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
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
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(
        mut self,
        new_value: GoogleMapsPlacesV1AutocompletePlacesRequest,
    ) -> PlaceAutocompleteCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> PlaceAutocompleteCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PlaceAutocompleteCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PlaceAutocompleteCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PlaceAutocompleteCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PlaceAutocompleteCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Get the details of a place based on its resource name, which is a string in the `places/{place_id}` format.
///
/// A builder for the *get* method supported by a *place* resource.
/// It is not used directly, but through a [`PlaceMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_places1 as places1;
/// # async fn dox() {
/// # use places1::{MapsPlaces, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = MapsPlaces::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.places().get("name")
///              .session_token("dolor")
///              .region_code("ea")
///              .language_code("ipsum")
///              .doit().await;
/// # }
/// ```
pub struct PlaceGetCall<'a, C>
where
    C: 'a,
{
    hub: &'a MapsPlaces<C>,
    _name: String,
    _session_token: Option<String>,
    _region_code: Option<String>,
    _language_code: Option<String>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for PlaceGetCall<'a, C> {}

impl<'a, C> PlaceGetCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, GoogleMapsPlacesV1Place)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "places.places.get",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "name", "sessionToken", "regionCode", "languageCode"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("name", self._name);
        if let Some(value) = self._session_token.as_ref() {
            params.push("sessionToken", value);
        }
        if let Some(value) = self._region_code.as_ref() {
            params.push("regionCode", value);
        }
        if let Some(value) = self._language_code.as_ref() {
            params.push("languageCode", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+name}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
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
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// Required. The resource name of a place, in the `places/{place_id}` format.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> PlaceGetCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// Optional. A string which identifies an Autocomplete session for billing purposes. Must be a URL and filename safe base64 string with at most 36 ASCII characters in length. Otherwise an INVALID_ARGUMENT error is returned. The session begins when the user starts typing a query, and concludes when they select a place and a call to Place Details or Address Validation is made. Each session can have multiple queries, followed by one Place Details or Address Validation request. The credentials used for each request within a session must belong to the same Google Cloud Console project. Once a session has concluded, the token is no longer valid; your app must generate a fresh token for each session. If the `session_token` parameter is omitted, or if you reuse a session token, the session is charged as if no session token was provided (each request is billed separately). We recommend the following guidelines: * Use session tokens for all Place Autocomplete calls. * Generate a fresh token for each session. Using a version 4 UUID is recommended. * Ensure that the credentials used for all Place Autocomplete, Place Details, and Address Validation requests within a session belong to the same Cloud Console project. * Be sure to pass a unique session token for each new session. Using the same token for more than one session will result in each request being billed individually.
    ///
    /// Sets the *session token* query property to the given value.
    pub fn session_token(mut self, new_value: &str) -> PlaceGetCall<'a, C> {
        self._session_token = Some(new_value.to_string());
        self
    }
    /// Optional. The Unicode country/region code (CLDR) of the location where the request is coming from. This parameter is used to display the place details, like region-specific place name, if available. The parameter can affect results based on applicable law. For more information, see https://www.unicode.org/cldr/charts/latest/supplemental/territory_language_information.html. Note that 3-digit region codes are not currently supported.
    ///
    /// Sets the *region code* query property to the given value.
    pub fn region_code(mut self, new_value: &str) -> PlaceGetCall<'a, C> {
        self._region_code = Some(new_value.to_string());
        self
    }
    /// Optional. Place details will be displayed with the preferred language if available. Current list of supported languages: https://developers.google.com/maps/faq#languagesupport.
    ///
    /// Sets the *language code* query property to the given value.
    pub fn language_code(mut self, new_value: &str) -> PlaceGetCall<'a, C> {
        self._language_code = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> PlaceGetCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PlaceGetCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PlaceGetCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PlaceGetCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PlaceGetCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Search for places near locations.
///
/// A builder for the *searchNearby* method supported by a *place* resource.
/// It is not used directly, but through a [`PlaceMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_places1 as places1;
/// use places1::api::GoogleMapsPlacesV1SearchNearbyRequest;
/// # async fn dox() {
/// # use places1::{MapsPlaces, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = MapsPlaces::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleMapsPlacesV1SearchNearbyRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.places().search_nearby(req)
///              .doit().await;
/// # }
/// ```
pub struct PlaceSearchNearbyCall<'a, C>
where
    C: 'a,
{
    hub: &'a MapsPlaces<C>,
    _request: GoogleMapsPlacesV1SearchNearbyRequest,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for PlaceSearchNearbyCall<'a, C> {}

impl<'a, C> PlaceSearchNearbyCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(
        mut self,
    ) -> common::Result<(common::Response, GoogleMapsPlacesV1SearchNearbyResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "places.places.searchNearby",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/places:searchNearby";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
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
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(
        mut self,
        new_value: GoogleMapsPlacesV1SearchNearbyRequest,
    ) -> PlaceSearchNearbyCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> PlaceSearchNearbyCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PlaceSearchNearbyCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PlaceSearchNearbyCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PlaceSearchNearbyCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PlaceSearchNearbyCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Text query based place search.
///
/// A builder for the *searchText* method supported by a *place* resource.
/// It is not used directly, but through a [`PlaceMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_places1 as places1;
/// use places1::api::GoogleMapsPlacesV1SearchTextRequest;
/// # async fn dox() {
/// # use places1::{MapsPlaces, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = MapsPlaces::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleMapsPlacesV1SearchTextRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.places().search_text(req)
///              .doit().await;
/// # }
/// ```
pub struct PlaceSearchTextCall<'a, C>
where
    C: 'a,
{
    hub: &'a MapsPlaces<C>,
    _request: GoogleMapsPlacesV1SearchTextRequest,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for PlaceSearchTextCall<'a, C> {}

impl<'a, C> PlaceSearchTextCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(
        mut self,
    ) -> common::Result<(common::Response, GoogleMapsPlacesV1SearchTextResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "places.places.searchText",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/places:searchText";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
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
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(
        mut self,
        new_value: GoogleMapsPlacesV1SearchTextRequest,
    ) -> PlaceSearchTextCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> PlaceSearchTextCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PlaceSearchTextCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PlaceSearchTextCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PlaceSearchTextCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PlaceSearchTextCall<'a, C> {
        self._scopes.clear();
        self
    }
}
