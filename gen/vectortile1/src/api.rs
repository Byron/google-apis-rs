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




// ########
// HUB ###
// ######

/// Central instance to access all SemanticTile related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_vectortile1 as vectortile1;
/// use vectortile1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use vectortile1::{SemanticTile, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// let mut hub = SemanticTile::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.featuretiles().get("name")
///              .region_code("et")
///              .language_code("vero")
///              .enable_unclipped_buildings(false)
///              .enable_private_roads(false)
///              .enable_political_features(false)
///              .enable_modeled_volumes(true)
///              .enable_feature_names(false)
///              .enable_detailed_highway_types(false)
///              .client_tile_version_id("duo")
///              .client_info_user_id("vero")
///              .client_info_platform("vero")
///              .client_info_operating_system("invidunt")
///              .client_info_device_model("Stet")
///              .client_info_application_version("vero")
///              .client_info_application_id("elitr")
///              .client_info_api_client("Lorem")
///              .always_include_building_footprints(true)
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
pub struct SemanticTile<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, S> client::Hub for SemanticTile<S> {}

impl<'a, S> SemanticTile<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> SemanticTile<S> {
        SemanticTile {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.3".to_string(),
            _base_url: "https://vectortile.googleapis.com/".to_string(),
            _root_url: "https://vectortile.googleapis.com/".to_string(),
        }
    }

    pub fn featuretiles(&'a self) -> FeaturetileMethods<'a, S> {
        FeaturetileMethods { hub: &self }
    }
    pub fn terraintiles(&'a self) -> TerraintileMethods<'a, S> {
        TerraintileMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.3`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://vectortile.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://vectortile.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Represents an area. Used to represent regions such as water, parks, etc. Next ID: 10
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Area {
    /// The z-order of this geometry when rendered on a flat basemap. Geometry with a lower z-order should be rendered beneath geometry with a higher z-order. This z-ordering does not imply anything about the altitude of the area relative to the ground, but it can be used to prevent z-fighting. Unlike Area.z_order this can be used to compare with Line.basemap_z_order, and in fact may yield more accurate rendering (where a line may be rendered beneath an area).
    #[serde(rename="basemapZOrder")]
    
    pub basemap_z_order: Option<BasemapZOrder>,
    /// True if the polygon is not entirely internal to the feature that it belongs to: that is, some of the edges are bordering another feature.
    #[serde(rename="hasExternalEdges")]
    
    pub has_external_edges: Option<bool>,
    /// When has_external_edges is true, the polygon has some edges that border another feature. This field indicates the internal edges that do not border another feature. Each value is an index into the vertices array, and denotes the start vertex of the internal edge (the next vertex in the boundary loop is the end of the edge). If the selected vertex is the last vertex in the boundary loop, then the edge between that vertex and the starting vertex of the loop is internal. This field may be used for styling. For example, building parapets could be placed only on the external edges of a building polygon, or water could be lighter colored near the external edges of a body of water. If has_external_edges is false, all edges are internal and this field will be empty.
    #[serde(rename="internalEdges")]
    
    pub internal_edges: Option<Vec<i32>>,
    /// Identifies the boundary loops of the polygon. Only set for INDEXED_TRIANGLE polygons. Each value is an index into the vertices array indicating the beginning of a loop. For instance, values of [2, 5] would indicate loop_data contained 3 loops with indices 0-1, 2-4, and 5-end. This may be used in conjunction with the internal_edges field for styling polygon boundaries. Note that an edge may be on a polygon boundary but still internal to the feature. For example, a feature split across multiple tiles will have an internal polygon boundary edge along the edge of the tile.
    #[serde(rename="loopBreaks")]
    
    pub loop_breaks: Option<Vec<i32>>,
    /// When the polygon encoding is of type INDEXED_TRIANGLES, this contains the indices of the triangle vertices in the vertex_offsets field. There are 3 vertex indices per triangle.
    #[serde(rename="triangleIndices")]
    
    pub triangle_indices: Option<Vec<i32>>,
    /// The polygon encoding type used for this area.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The vertices present in the polygon defining the area.
    #[serde(rename="vertexOffsets")]
    
    pub vertex_offsets: Option<Vertex2DList>,
    /// The z-ordering of this area. Areas with a lower z-order should be rendered beneath areas with a higher z-order. This z-ordering does not imply anything about the altitude of the line relative to the ground, but it can be used to prevent z-fighting during rendering on the client. This z-ordering can only be used to compare areas, and cannot be compared with the z_order field in the Line message. The z-order may be negative or zero. Prefer Area.basemap_z_order.
    #[serde(rename="zOrder")]
    
    pub z_order: Option<i32>,
}

impl client::Part for Area {}


/// Metadata necessary to determine the ordering of a particular basemap element relative to others. To render the basemap correctly, sort by z-plane, then z-grade, then z-within-grade.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BasemapZOrder {
    /// The second most significant component of the ordering of a component to be rendered onto the basemap.
    #[serde(rename="zGrade")]
    
    pub z_grade: Option<i32>,
    /// The most significant component of the ordering of a component to be rendered onto the basemap.
    #[serde(rename="zPlane")]
    
    pub z_plane: Option<i32>,
    /// The least significant component of the ordering of a component to be rendered onto the basemap.
    #[serde(rename="zWithinGrade")]
    
    pub z_within_grade: Option<i32>,
}

impl client::Part for BasemapZOrder {}


/// Represents a height-extruded area: a 3D prism with a constant X-Y plane cross section. Used to represent extruded buildings. A single building may consist of several extruded areas. The min_z and max_z fields are scaled to the size of the tile. An extruded area with a max_z value of 4096 has the same height as the width of the tile that it is on.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExtrudedArea {
    /// The area representing the footprint of the extruded area.
    
    pub area: Option<Area>,
    /// The z-value in local tile coordinates where the extruded area ends.
    #[serde(rename="maxZ")]
    
    pub max_z: Option<i32>,
    /// The z-value in local tile coordinates where the extruded area begins. This is non-zero for extruded areas that begin off the ground. For example, a building with a skybridge may have an extruded area component with a non-zero min_z.
    #[serde(rename="minZ")]
    
    pub min_z: Option<i32>,
}

impl client::Part for ExtrudedArea {}


/// A feature representing a single geographic entity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Feature {
    /// The localized name of this feature. Currently only returned for roads.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The geometry of this feature, representing the space that it occupies in the world.
    
    pub geometry: Option<Geometry>,
    /// Place ID of this feature, suitable for use in Places API details requests.
    #[serde(rename="placeId")]
    
    pub place_id: Option<String>,
    /// Relations to other features.
    
    pub relations: Option<Vec<Relation>>,
    /// Metadata for features with the SEGMENT FeatureType.
    #[serde(rename="segmentInfo")]
    
    pub segment_info: Option<SegmentInfo>,
    /// The type of this feature.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Feature {}


/// A tile containing information about the map features located in the region it covers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get featuretiles](FeaturetileGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FeatureTile {
    /// The global tile coordinates that uniquely identify this tile.
    
    pub coordinates: Option<TileCoordinates>,
    /// Features present on this map tile.
    
    pub features: Option<Vec<Feature>>,
    /// Resource name of the tile. The tile resource name is prefixed by its collection ID `tiles/` followed by the resource ID, which encodes the tile's global x and y coordinates and zoom level as `@,,z`. For example, `tiles/@1,2,3z`.
    
    pub name: Option<String>,
    /// Data providers for the data contained in this tile.
    
    pub providers: Option<Vec<ProviderInfo>>,
    /// Tile response status code to support tile caching.
    
    pub status: Option<String>,
    /// An opaque value, usually less than 30 characters, that contains version info about this tile and the data that was used to generate it. The client should store this value in its tile cache and pass it back to the API in the client_tile_version_id field of subsequent tile requests in order to enable the API to detect when the new tile would be the same as the one the client already has in its cache. Also see STATUS_OK_DATA_UNCHANGED.
    #[serde(rename="versionId")]
    
    pub version_id: Option<String>,
}

impl client::Resource for FeatureTile {}
impl client::ResponseResult for FeatureTile {}


/// A packed representation of a 2D grid of uniformly spaced points containing elevation data. Each point within the grid represents the altitude in meters above average sea level at that location within the tile. Elevations provided are (generally) relative to the EGM96 geoid, however some areas will be relative to NAVD88. EGM96 and NAVD88 are off by no more than 2 meters. The grid is oriented north-west to south-east, as illustrated: rows[0].a[0] rows[0].a[m] +-----------------+ | | | N | | ^ | | | | | W <-----> E | | | | | v | | S | | | +-----------------+ rows[n].a[0] rows[n].a[m] Rather than storing the altitudes directly, we store the diffs between them as integers at some requested level of precision to take advantage of integer packing. The actual altitude values a[] can be reconstructed using the scale and each row's first_altitude and altitude_diff fields.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FirstDerivativeElevationGrid {
    /// A multiplier applied to the altitude fields below to extract the actual altitudes in meters from the elevation grid.
    #[serde(rename="altitudeMultiplier")]
    
    pub altitude_multiplier: Option<f32>,
    /// Rows of points containing altitude data making up the elevation grid. Each row is the same length. Rows are ordered from north to south. E.g: rows[0] is the north-most row, and rows[n] is the south-most row.
    
    pub rows: Option<Vec<Row>>,
}

impl client::Part for FirstDerivativeElevationGrid {}


/// Represents the geometry of a feature, that is, the shape that it has on the map. The local tile coordinate system has the origin at the north-west (upper-left) corner of the tile, and is scaled to 4096 units across each edge. The height (Z) axis has the same scale factor: an extruded area with a max_z value of 4096 has the same height as the width of the tile that it is on. There is no clipping boundary, so it is possible that some coordinates will lie outside the tile boundaries.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Geometry {
    /// The areas present in this geometry.
    
    pub areas: Option<Vec<Area>>,
    /// The extruded areas present in this geometry. Not populated if modeled_volumes are included in this geometry unless always_include_building_footprints is set in GetFeatureTileRequest, in which case the client should decide which (extruded areas or modeled volumes) should be used (they should not be rendered together).
    #[serde(rename="extrudedAreas")]
    
    pub extruded_areas: Option<Vec<ExtrudedArea>>,
    /// The lines present in this geometry.
    
    pub lines: Option<Vec<Line>>,
    /// The modeled volumes present in this geometry. Not populated unless enable_modeled_volumes has been set in GetFeatureTileRequest.
    #[serde(rename="modeledVolumes")]
    
    pub modeled_volumes: Option<Vec<ModeledVolume>>,
}

impl client::Part for Geometry {}


/// Represents a 2D polyline. Used to represent segments such as roads, train tracks, etc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Line {
    /// The z-order of this geometry when rendered on a flat basemap. Geometry with a lower z-order should be rendered beneath geometry with a higher z-order. This z-ordering does not imply anything about the altitude of the area relative to the ground, but it can be used to prevent z-fighting. Unlike Line.z_order this can be used to compare with Area.basemap_z_order, and in fact may yield more accurate rendering (where a line may be rendered beneath an area).
    #[serde(rename="basemapZOrder")]
    
    pub basemap_z_order: Option<BasemapZOrder>,
    /// The vertices present in the polyline.
    #[serde(rename="vertexOffsets")]
    
    pub vertex_offsets: Option<Vertex2DList>,
    /// The z-order of the line. Lines with a lower z-order should be rendered beneath lines with a higher z-order. This z-ordering does not imply anything about the altitude of the area relative to the ground, but it can be used to prevent z-fighting during rendering on the client. In general, larger and more important road features will have a higher z-order line associated with them. This z-ordering can only be used to compare lines, and cannot be compared with the z_order field in the Area message. The z-order may be negative or zero. Prefer Line.basemap_z_order.
    #[serde(rename="zOrder")]
    
    pub z_order: Option<i32>,
}

impl client::Part for Line {}


/// Represents a modeled volume in 3D space. Used to represent 3D buildings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ModeledVolume {
    /// The triangle strips present in this mesh.
    
    pub strips: Option<Vec<TriangleStrip>>,
    /// The vertices present in the mesh defining the modeled volume.
    #[serde(rename="vertexOffsets")]
    
    pub vertex_offsets: Option<Vertex3DList>,
}

impl client::Part for ModeledVolume {}


/// Information about the data providers that should be included in the attribution string shown by the client.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProviderInfo {
    /// Attribution string for this provider. This string is not localized.
    
    pub description: Option<String>,
}

impl client::Part for ProviderInfo {}


/// Represents a relation to another feature in the tile. For example, a building might be occupied by a given POI. The related feature can be retrieved using the related feature index.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Relation {
    /// Zero-based index to look up the related feature from the list of features in the tile.
    #[serde(rename="relatedFeatureIndex")]
    
    pub related_feature_index: Option<i32>,
    /// Relation type between the origin feature to the related feature.
    #[serde(rename="relationType")]
    
    pub relation_type: Option<String>,
}

impl client::Part for Relation {}


/// Extra metadata relating to roads.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RoadInfo {
    /// Road has signage discouraging or prohibiting use by the general public. E.g., roads with signs that say "Private", or "No trespassing."
    #[serde(rename="isPrivate")]
    
    pub is_private: Option<bool>,
}

impl client::Part for RoadInfo {}


/// A row of altitude points in the elevation grid, ordered from west to east.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Row {
    /// The difference between each successive pair of altitudes, from west to east. The first, westmost point, is just the altitude rather than a diff. The units are specified by the altitude_multiplier parameter above; the value in meters is given by altitude_multiplier * altitude_diffs[n]. The altitude row (in metres above sea level) can be reconstructed with: a[0] = altitude_diffs[0] * altitude_multiplier when n > 0, a[n] = a[n-1] + altitude_diffs[n-1] * altitude_multiplier.
    #[serde(rename="altitudeDiffs")]
    
    pub altitude_diffs: Option<Vec<i32>>,
}

impl client::Part for Row {}


/// A packed representation of a 2D grid of uniformly spaced points containing elevation data. Each point within the grid represents the altitude in meters above average sea level at that location within the tile. Elevations provided are (generally) relative to the EGM96 geoid, however some areas will be relative to NAVD88. EGM96 and NAVD88 are off by no more than 2 meters. The grid is oriented north-west to south-east, as illustrated: rows[0].a[0] rows[0].a[m] +-----------------+ | | | N | | ^ | | | | | W <-----> E | | | | | v | | S | | | +-----------------+ rows[n].a[0] rows[n].a[m] Rather than storing the altitudes directly, we store the diffs of the diffs between them as integers at some requested level of precision to take advantage of integer packing. Note that the data is packed in such a way that is fast to decode in Unity and that further optimizes wire size.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SecondDerivativeElevationGrid {
    /// A multiplier applied to the elements in the encoded data to extract the actual altitudes in meters.
    #[serde(rename="altitudeMultiplier")]
    
    pub altitude_multiplier: Option<f32>,
    /// The number of columns included in the encoded elevation data (i.e. the horizontal resolution of the grid).
    #[serde(rename="columnCount")]
    
    pub column_count: Option<i32>,
    /// A stream of elements each representing a point on the tile running across each row from left to right, top to bottom. There will be precisely horizontal_resolution * vertical_resolution elements in the stream. The elements are not the heights, rather the second order derivative of the values one would expect in a stream of height data. Each element is a varint with the following encoding: ------------------------------------------------------------------------| | Head Nibble | ------------------------------------------------------------------------| | Bit 0 | Bit 1 | Bits 2-3 | | Terminator| Sign (1=neg) | Least significant 2 bits of absolute error | ------------------------------------------------------------------------| | Tail Nibble #1 | ------------------------------------------------------------------------| | Bit 0 | Bit 1-3 | | Terminator| Least significant 3 bits of absolute error | ------------------------------------------------------------------------| | ... | Tail Nibble #n | ------------------------------------------------------------------------| | Bit 0 | Bit 1-3 | | Terminator| Least significant 3 bits of absolute error | ------------------------------------------------------------------------|
    #[serde(rename="encodedData")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub encoded_data: Option<Vec<u8>>,
    /// The number of rows included in the encoded elevation data (i.e. the vertical resolution of the grid).
    #[serde(rename="rowCount")]
    
    pub row_count: Option<i32>,
}

impl client::Part for SecondDerivativeElevationGrid {}


/// Extra metadata relating to segments.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SegmentInfo {
    /// Metadata for features with the ROAD FeatureType.
    #[serde(rename="roadInfo")]
    
    pub road_info: Option<RoadInfo>,
}

impl client::Part for SegmentInfo {}


/// A tile containing information about the terrain located in the region it covers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get terraintiles](TerraintileGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TerrainTile {
    /// The global tile coordinates that uniquely identify this tile.
    
    pub coordinates: Option<TileCoordinates>,
    /// Terrain elevation data encoded as a FirstDerivativeElevationGrid.
    #[serde(rename="firstDerivative")]
    
    pub first_derivative: Option<FirstDerivativeElevationGrid>,
    /// Resource name of the tile. The tile resource name is prefixed by its collection ID `terrain/` followed by the resource ID, which encodes the tile's global x and y coordinates and zoom level as `@,,z`. For example, `terrain/@1,2,3z`.
    
    pub name: Option<String>,
    /// Terrain elevation data encoded as a SecondDerivativeElevationGrid. .
    #[serde(rename="secondDerivative")]
    
    pub second_derivative: Option<SecondDerivativeElevationGrid>,
}

impl client::Resource for TerrainTile {}
impl client::ResponseResult for TerrainTile {}


/// Global tile coordinates. Global tile coordinates reference a specific tile on the map at a specific zoom level. The origin of this coordinate system is always at the northwest corner of the map, with x values increasing from west to east and y values increasing from north to south. Tiles are indexed using x, y coordinates from that origin. The zoom level containing the entire world in a tile is 0, and it increases as you zoom in. Zoom level n + 1 will contain 4 times as many tiles as zoom level n. The zoom level controls the level of detail of the data that is returned. In particular, this affects the set of feature types returned, their density, and geometry simplification. The exact tile contents may change over time, but care will be taken to keep supporting the most important use cases. For example, zoom level 15 shows roads for orientation and planning in the local neighborhood and zoom level 17 shows buildings to give users on foot a sense of situational awareness.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TileCoordinates {
    /// Required. The x coordinate.
    
    pub x: Option<i32>,
    /// Required. The y coordinate.
    
    pub y: Option<i32>,
    /// Required. The Google Maps API zoom level.
    
    pub zoom: Option<i32>,
}

impl client::Part for TileCoordinates {}


/// Represents a strip of triangles. Each triangle uses the last edge of the previous one. The following diagram shows an example of a triangle strip, with each vertex labeled with its index in the vertex_index array. (1)-----(3) / \ / \ / \ / \ / \ / \ (0)-----(2)-----(4) Vertices may be in either clockwise or counter-clockwise order.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TriangleStrip {
    /// Index into the vertex_offset array representing the next vertex in the triangle strip.
    #[serde(rename="vertexIndices")]
    
    pub vertex_indices: Option<Vec<i32>>,
}

impl client::Part for TriangleStrip {}


/// 2D vertex list used for lines and areas. Each entry represents an offset from the previous one in local tile coordinates. The first entry is offset from (0, 0). For example, the list of vertices [(1,1), (2, 2), (1, 2)] would be encoded in vertex offsets as [(1, 1), (1, 1), (-1, 0)].
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Vertex2DList {
    /// List of x-offsets in local tile coordinates.
    #[serde(rename="xOffsets")]
    
    pub x_offsets: Option<Vec<i32>>,
    /// List of y-offsets in local tile coordinates.
    #[serde(rename="yOffsets")]
    
    pub y_offsets: Option<Vec<i32>>,
}

impl client::Part for Vertex2DList {}


/// 3D vertex list used for modeled volumes. Each entry represents an offset from the previous one in local tile coordinates. The first coordinate is offset from (0, 0, 0).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Vertex3DList {
    /// List of x-offsets in local tile coordinates.
    #[serde(rename="xOffsets")]
    
    pub x_offsets: Option<Vec<i32>>,
    /// List of y-offsets in local tile coordinates.
    #[serde(rename="yOffsets")]
    
    pub y_offsets: Option<Vec<i32>>,
    /// List of z-offsets in local tile coordinates.
    #[serde(rename="zOffsets")]
    
    pub z_offsets: Option<Vec<i32>>,
}

impl client::Part for Vertex3DList {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *featuretile* resources.
/// It is not used directly, but through the [`SemanticTile`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_vectortile1 as vectortile1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use vectortile1::{SemanticTile, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = SemanticTile::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.featuretiles();
/// # }
/// ```
pub struct FeaturetileMethods<'a, S>
    where S: 'a {

    hub: &'a SemanticTile<S>,
}

impl<'a, S> client::MethodsBuilder for FeaturetileMethods<'a, S> {}

impl<'a, S> FeaturetileMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a feature tile by its tile resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the tile. The tile resource name is prefixed by its collection ID `tiles/` followed by the resource ID, which encodes the tile's global x and y coordinates and zoom level as `@,,z`. For example, `tiles/@1,2,3z`.
    pub fn get(&self, name: &str) -> FeaturetileGetCall<'a, S> {
        FeaturetileGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _region_code: Default::default(),
            _language_code: Default::default(),
            _enable_unclipped_buildings: Default::default(),
            _enable_private_roads: Default::default(),
            _enable_political_features: Default::default(),
            _enable_modeled_volumes: Default::default(),
            _enable_feature_names: Default::default(),
            _enable_detailed_highway_types: Default::default(),
            _client_tile_version_id: Default::default(),
            _client_info_user_id: Default::default(),
            _client_info_platform: Default::default(),
            _client_info_operating_system: Default::default(),
            _client_info_device_model: Default::default(),
            _client_info_application_version: Default::default(),
            _client_info_application_id: Default::default(),
            _client_info_api_client: Default::default(),
            _always_include_building_footprints: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *terraintile* resources.
/// It is not used directly, but through the [`SemanticTile`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_vectortile1 as vectortile1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use vectortile1::{SemanticTile, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = SemanticTile::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.terraintiles();
/// # }
/// ```
pub struct TerraintileMethods<'a, S>
    where S: 'a {

    hub: &'a SemanticTile<S>,
}

impl<'a, S> client::MethodsBuilder for TerraintileMethods<'a, S> {}

impl<'a, S> TerraintileMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a terrain tile by its tile resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the tile. The tile resource name is prefixed by its collection ID `terraintiles/` followed by the resource ID, which encodes the tile's global x and y coordinates and zoom level as `@,,z`. For example, `terraintiles/@1,2,3z`.
    pub fn get(&self, name: &str) -> TerraintileGetCall<'a, S> {
        TerraintileGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _terrain_formats: Default::default(),
            _min_elevation_resolution_cells: Default::default(),
            _max_elevation_resolution_cells: Default::default(),
            _client_info_user_id: Default::default(),
            _client_info_platform: Default::default(),
            _client_info_operating_system: Default::default(),
            _client_info_device_model: Default::default(),
            _client_info_application_version: Default::default(),
            _client_info_application_id: Default::default(),
            _client_info_api_client: Default::default(),
            _altitude_precision_centimeters: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Gets a feature tile by its tile resource name.
///
/// A builder for the *get* method supported by a *featuretile* resource.
/// It is not used directly, but through a [`FeaturetileMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_vectortile1 as vectortile1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use vectortile1::{SemanticTile, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = SemanticTile::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.featuretiles().get("name")
///              .region_code("accusam")
///              .language_code("takimata")
///              .enable_unclipped_buildings(true)
///              .enable_private_roads(false)
///              .enable_political_features(false)
///              .enable_modeled_volumes(true)
///              .enable_feature_names(false)
///              .enable_detailed_highway_types(false)
///              .client_tile_version_id("dolore")
///              .client_info_user_id("dolore")
///              .client_info_platform("dolore")
///              .client_info_operating_system("voluptua.")
///              .client_info_device_model("amet.")
///              .client_info_application_version("ea")
///              .client_info_application_id("sadipscing")
///              .client_info_api_client("Lorem")
///              .always_include_building_footprints(true)
///              .doit().await;
/// # }
/// ```
pub struct FeaturetileGetCall<'a, S>
    where S: 'a {

    hub: &'a SemanticTile<S>,
    _name: String,
    _region_code: Option<String>,
    _language_code: Option<String>,
    _enable_unclipped_buildings: Option<bool>,
    _enable_private_roads: Option<bool>,
    _enable_political_features: Option<bool>,
    _enable_modeled_volumes: Option<bool>,
    _enable_feature_names: Option<bool>,
    _enable_detailed_highway_types: Option<bool>,
    _client_tile_version_id: Option<String>,
    _client_info_user_id: Option<String>,
    _client_info_platform: Option<String>,
    _client_info_operating_system: Option<String>,
    _client_info_device_model: Option<String>,
    _client_info_application_version: Option<String>,
    _client_info_application_id: Option<String>,
    _client_info_api_client: Option<String>,
    _always_include_building_footprints: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for FeaturetileGetCall<'a, S> {}

impl<'a, S> FeaturetileGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, FeatureTile)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "vectortile.featuretiles.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "name", "regionCode", "languageCode", "enableUnclippedBuildings", "enablePrivateRoads", "enablePoliticalFeatures", "enableModeledVolumes", "enableFeatureNames", "enableDetailedHighwayTypes", "clientTileVersionId", "clientInfo.userId", "clientInfo.platform", "clientInfo.operatingSystem", "clientInfo.deviceModel", "clientInfo.applicationVersion", "clientInfo.applicationId", "clientInfo.apiClient", "alwaysIncludeBuildingFootprints"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(20 + self._additional_params.len());
        params.push("name", self._name);
        if let Some(value) = self._region_code.as_ref() {
            params.push("regionCode", value);
        }
        if let Some(value) = self._language_code.as_ref() {
            params.push("languageCode", value);
        }
        if let Some(value) = self._enable_unclipped_buildings.as_ref() {
            params.push("enableUnclippedBuildings", value.to_string());
        }
        if let Some(value) = self._enable_private_roads.as_ref() {
            params.push("enablePrivateRoads", value.to_string());
        }
        if let Some(value) = self._enable_political_features.as_ref() {
            params.push("enablePoliticalFeatures", value.to_string());
        }
        if let Some(value) = self._enable_modeled_volumes.as_ref() {
            params.push("enableModeledVolumes", value.to_string());
        }
        if let Some(value) = self._enable_feature_names.as_ref() {
            params.push("enableFeatureNames", value.to_string());
        }
        if let Some(value) = self._enable_detailed_highway_types.as_ref() {
            params.push("enableDetailedHighwayTypes", value.to_string());
        }
        if let Some(value) = self._client_tile_version_id.as_ref() {
            params.push("clientTileVersionId", value);
        }
        if let Some(value) = self._client_info_user_id.as_ref() {
            params.push("clientInfo.userId", value);
        }
        if let Some(value) = self._client_info_platform.as_ref() {
            params.push("clientInfo.platform", value);
        }
        if let Some(value) = self._client_info_operating_system.as_ref() {
            params.push("clientInfo.operatingSystem", value);
        }
        if let Some(value) = self._client_info_device_model.as_ref() {
            params.push("clientInfo.deviceModel", value);
        }
        if let Some(value) = self._client_info_application_version.as_ref() {
            params.push("clientInfo.applicationVersion", value);
        }
        if let Some(value) = self._client_info_application_id.as_ref() {
            params.push("clientInfo.applicationId", value);
        }
        if let Some(value) = self._client_info_api_client.as_ref() {
            params.push("clientInfo.apiClient", value);
        }
        if let Some(value) = self._always_include_building_footprints.as_ref() {
            params.push("alwaysIncludeBuildingFootprints", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+name}";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());



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


    /// Required. Resource name of the tile. The tile resource name is prefixed by its collection ID `tiles/` followed by the resource ID, which encodes the tile's global x and y coordinates and zoom level as `@,,z`. For example, `tiles/@1,2,3z`.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> FeaturetileGetCall<'a, S> {
        self._name = new_value.to_string();
        self
    }
    /// Required. The Unicode country/region code (CLDR) of the location from which the request is coming from, such as "US" and "419". For more information, see http://www.unicode.org/reports/tr35/#unicode_region_subtag.
    ///
    /// Sets the *region code* query property to the given value.
    pub fn region_code(mut self, new_value: &str) -> FeaturetileGetCall<'a, S> {
        self._region_code = Some(new_value.to_string());
        self
    }
    /// Required. The BCP-47 language code corresponding to the language in which the name was requested, such as "en-US" or "sr-Latn". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
    ///
    /// Sets the *language code* query property to the given value.
    pub fn language_code(mut self, new_value: &str) -> FeaturetileGetCall<'a, S> {
        self._language_code = Some(new_value.to_string());
        self
    }
    /// Flag indicating whether unclipped buildings should be returned. If this is set, building render ops will extend beyond the tile boundary. Buildings will only be returned on the tile that contains their centroid.
    ///
    /// Sets the *enable unclipped buildings* query property to the given value.
    pub fn enable_unclipped_buildings(mut self, new_value: bool) -> FeaturetileGetCall<'a, S> {
        self._enable_unclipped_buildings = Some(new_value);
        self
    }
    /// Flag indicating whether the returned tile will contain road features that are marked private. Private roads are indicated by the Feature.segment_info.road_info.is_private field.
    ///
    /// Sets the *enable private roads* query property to the given value.
    pub fn enable_private_roads(mut self, new_value: bool) -> FeaturetileGetCall<'a, S> {
        self._enable_private_roads = Some(new_value);
        self
    }
    /// Flag indicating whether political features should be returned.
    ///
    /// Sets the *enable political features* query property to the given value.
    pub fn enable_political_features(mut self, new_value: bool) -> FeaturetileGetCall<'a, S> {
        self._enable_political_features = Some(new_value);
        self
    }
    /// Flag indicating whether 3D building models should be enabled. If this is set structures will be returned as 3D modeled volumes rather than 2.5D extruded areas where possible.
    ///
    /// Sets the *enable modeled volumes* query property to the given value.
    pub fn enable_modeled_volumes(mut self, new_value: bool) -> FeaturetileGetCall<'a, S> {
        self._enable_modeled_volumes = Some(new_value);
        self
    }
    /// Flag indicating whether human-readable names should be returned for features. If this is set, the display_name field on the feature will be filled out.
    ///
    /// Sets the *enable feature names* query property to the given value.
    pub fn enable_feature_names(mut self, new_value: bool) -> FeaturetileGetCall<'a, S> {
        self._enable_feature_names = Some(new_value);
        self
    }
    /// Flag indicating whether detailed highway types should be returned. If this is set, the CONTROLLED_ACCESS_HIGHWAY type may be returned. If not, then these highways will have the generic HIGHWAY type. This exists for backwards compatibility reasons.
    ///
    /// Sets the *enable detailed highway types* query property to the given value.
    pub fn enable_detailed_highway_types(mut self, new_value: bool) -> FeaturetileGetCall<'a, S> {
        self._enable_detailed_highway_types = Some(new_value);
        self
    }
    /// Optional version id identifying the tile that is already in the client's cache. This field should be populated with the most recent version_id value returned by the API for the requested tile. If the version id is empty the server always returns a newly rendered tile. If it is provided the server checks if the tile contents would be identical to one that's already on the client, and if so, returns a stripped-down response tile with STATUS_OK_DATA_UNCHANGED instead.
    ///
    /// Sets the *client tile version id* query property to the given value.
    pub fn client_tile_version_id(mut self, new_value: &str) -> FeaturetileGetCall<'a, S> {
        self._client_tile_version_id = Some(new_value.to_string());
        self
    }
    /// Required. A client-generated user ID. The ID should be generated and persisted during the first user session or whenever a pre-existing ID is not found. The exact format is up to the client. This must be non-empty in a GetFeatureTileRequest (whether via the header or GetFeatureTileRequest.client_info).
    ///
    /// Sets the *client info.user id* query property to the given value.
    pub fn client_info_user_id(mut self, new_value: &str) -> FeaturetileGetCall<'a, S> {
        self._client_info_user_id = Some(new_value.to_string());
        self
    }
    /// Platform where the application is running.
    ///
    /// Sets the *client info.platform* query property to the given value.
    pub fn client_info_platform(mut self, new_value: &str) -> FeaturetileGetCall<'a, S> {
        self._client_info_platform = Some(new_value.to_string());
        self
    }
    /// Operating system name and version as reported by the OS. For example, "Mac OS X 10.10.4". The exact format is platform-dependent.
    ///
    /// Sets the *client info.operating system* query property to the given value.
    pub fn client_info_operating_system(mut self, new_value: &str) -> FeaturetileGetCall<'a, S> {
        self._client_info_operating_system = Some(new_value.to_string());
        self
    }
    /// Device model as reported by the device. The exact format is platform-dependent.
    ///
    /// Sets the *client info.device model* query property to the given value.
    pub fn client_info_device_model(mut self, new_value: &str) -> FeaturetileGetCall<'a, S> {
        self._client_info_device_model = Some(new_value.to_string());
        self
    }
    /// Application version number, such as "1.2.3". The exact format is application-dependent.
    ///
    /// Sets the *client info.application version* query property to the given value.
    pub fn client_info_application_version(mut self, new_value: &str) -> FeaturetileGetCall<'a, S> {
        self._client_info_application_version = Some(new_value.to_string());
        self
    }
    /// Application ID, such as the package name on Android and the bundle identifier on iOS platforms.
    ///
    /// Sets the *client info.application id* query property to the given value.
    pub fn client_info_application_id(mut self, new_value: &str) -> FeaturetileGetCall<'a, S> {
        self._client_info_application_id = Some(new_value.to_string());
        self
    }
    /// API client name and version. For example, the SDK calling the API. The exact format is up to the client.
    ///
    /// Sets the *client info.api client* query property to the given value.
    pub fn client_info_api_client(mut self, new_value: &str) -> FeaturetileGetCall<'a, S> {
        self._client_info_api_client = Some(new_value.to_string());
        self
    }
    /// Flag indicating whether the returned tile will always contain 2.5D footprints for structures. If enabled_modeled_volumes is set, this will mean that structures will have both their 3D models and 2.5D footprints returned.
    ///
    /// Sets the *always include building footprints* query property to the given value.
    pub fn always_include_building_footprints(mut self, new_value: bool) -> FeaturetileGetCall<'a, S> {
        self._always_include_building_footprints = Some(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> FeaturetileGetCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> FeaturetileGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Gets a terrain tile by its tile resource name.
///
/// A builder for the *get* method supported by a *terraintile* resource.
/// It is not used directly, but through a [`TerraintileMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_vectortile1 as vectortile1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use vectortile1::{SemanticTile, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = SemanticTile::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.terraintiles().get("name")
///              .add_terrain_formats("est")
///              .min_elevation_resolution_cells(-27)
///              .max_elevation_resolution_cells(-43)
///              .client_info_user_id("sit")
///              .client_info_platform("et")
///              .client_info_operating_system("tempor")
///              .client_info_device_model("aliquyam")
///              .client_info_application_version("ipsum")
///              .client_info_application_id("et")
///              .client_info_api_client("sanctus")
///              .altitude_precision_centimeters(-56)
///              .doit().await;
/// # }
/// ```
pub struct TerraintileGetCall<'a, S>
    where S: 'a {

    hub: &'a SemanticTile<S>,
    _name: String,
    _terrain_formats: Vec<String>,
    _min_elevation_resolution_cells: Option<i32>,
    _max_elevation_resolution_cells: Option<i32>,
    _client_info_user_id: Option<String>,
    _client_info_platform: Option<String>,
    _client_info_operating_system: Option<String>,
    _client_info_device_model: Option<String>,
    _client_info_application_version: Option<String>,
    _client_info_application_id: Option<String>,
    _client_info_api_client: Option<String>,
    _altitude_precision_centimeters: Option<i32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for TerraintileGetCall<'a, S> {}

impl<'a, S> TerraintileGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, TerrainTile)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "vectortile.terraintiles.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "name", "terrainFormats", "minElevationResolutionCells", "maxElevationResolutionCells", "clientInfo.userId", "clientInfo.platform", "clientInfo.operatingSystem", "clientInfo.deviceModel", "clientInfo.applicationVersion", "clientInfo.applicationId", "clientInfo.apiClient", "altitudePrecisionCentimeters"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(14 + self._additional_params.len());
        params.push("name", self._name);
        if self._terrain_formats.len() > 0 {
            for f in self._terrain_formats.iter() {
                params.push("terrainFormats", f);
            }
        }
        if let Some(value) = self._min_elevation_resolution_cells.as_ref() {
            params.push("minElevationResolutionCells", value.to_string());
        }
        if let Some(value) = self._max_elevation_resolution_cells.as_ref() {
            params.push("maxElevationResolutionCells", value.to_string());
        }
        if let Some(value) = self._client_info_user_id.as_ref() {
            params.push("clientInfo.userId", value);
        }
        if let Some(value) = self._client_info_platform.as_ref() {
            params.push("clientInfo.platform", value);
        }
        if let Some(value) = self._client_info_operating_system.as_ref() {
            params.push("clientInfo.operatingSystem", value);
        }
        if let Some(value) = self._client_info_device_model.as_ref() {
            params.push("clientInfo.deviceModel", value);
        }
        if let Some(value) = self._client_info_application_version.as_ref() {
            params.push("clientInfo.applicationVersion", value);
        }
        if let Some(value) = self._client_info_application_id.as_ref() {
            params.push("clientInfo.applicationId", value);
        }
        if let Some(value) = self._client_info_api_client.as_ref() {
            params.push("clientInfo.apiClient", value);
        }
        if let Some(value) = self._altitude_precision_centimeters.as_ref() {
            params.push("altitudePrecisionCentimeters", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+name}";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());



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


    /// Required. Resource name of the tile. The tile resource name is prefixed by its collection ID `terraintiles/` followed by the resource ID, which encodes the tile's global x and y coordinates and zoom level as `@,,z`. For example, `terraintiles/@1,2,3z`.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> TerraintileGetCall<'a, S> {
        self._name = new_value.to_string();
        self
    }
    /// Terrain formats that the client understands.
    ///
    /// Append the given value to the *terrain formats* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_terrain_formats(mut self, new_value: &str) -> TerraintileGetCall<'a, S> {
        self._terrain_formats.push(new_value.to_string());
        self
    }
    /// The minimum allowed resolution for the returned elevation heightmap. Possible values: between 0 and 1024 (and not more than max_elevation_resolution_cells). Zero is supported for backward compatibility. Under-sized heightmaps will be non-uniformly up-sampled such that each edge is no shorter than this value. Non-uniformity is chosen to maximise the amount of preserved data. For example: Original resolution: 30px (width) * 10px (height) min_elevation_resolution: 30 New resolution: 30px (width) * 30px (height)
    ///
    /// Sets the *min elevation resolution cells* query property to the given value.
    pub fn min_elevation_resolution_cells(mut self, new_value: i32) -> TerraintileGetCall<'a, S> {
        self._min_elevation_resolution_cells = Some(new_value);
        self
    }
    /// The maximum allowed resolution for the returned elevation heightmap. Possible values: between 1 and 1024 (and not less than min_elevation_resolution_cells). Over-sized heightmaps will be non-uniformly down-sampled such that each edge is no longer than this value. Non-uniformity is chosen to maximise the amount of preserved data. For example: Original resolution: 100px (width) * 30px (height) max_elevation_resolution: 30 New resolution: 30px (width) * 30px (height)
    ///
    /// Sets the *max elevation resolution cells* query property to the given value.
    pub fn max_elevation_resolution_cells(mut self, new_value: i32) -> TerraintileGetCall<'a, S> {
        self._max_elevation_resolution_cells = Some(new_value);
        self
    }
    /// Required. A client-generated user ID. The ID should be generated and persisted during the first user session or whenever a pre-existing ID is not found. The exact format is up to the client. This must be non-empty in a GetFeatureTileRequest (whether via the header or GetFeatureTileRequest.client_info).
    ///
    /// Sets the *client info.user id* query property to the given value.
    pub fn client_info_user_id(mut self, new_value: &str) -> TerraintileGetCall<'a, S> {
        self._client_info_user_id = Some(new_value.to_string());
        self
    }
    /// Platform where the application is running.
    ///
    /// Sets the *client info.platform* query property to the given value.
    pub fn client_info_platform(mut self, new_value: &str) -> TerraintileGetCall<'a, S> {
        self._client_info_platform = Some(new_value.to_string());
        self
    }
    /// Operating system name and version as reported by the OS. For example, "Mac OS X 10.10.4". The exact format is platform-dependent.
    ///
    /// Sets the *client info.operating system* query property to the given value.
    pub fn client_info_operating_system(mut self, new_value: &str) -> TerraintileGetCall<'a, S> {
        self._client_info_operating_system = Some(new_value.to_string());
        self
    }
    /// Device model as reported by the device. The exact format is platform-dependent.
    ///
    /// Sets the *client info.device model* query property to the given value.
    pub fn client_info_device_model(mut self, new_value: &str) -> TerraintileGetCall<'a, S> {
        self._client_info_device_model = Some(new_value.to_string());
        self
    }
    /// Application version number, such as "1.2.3". The exact format is application-dependent.
    ///
    /// Sets the *client info.application version* query property to the given value.
    pub fn client_info_application_version(mut self, new_value: &str) -> TerraintileGetCall<'a, S> {
        self._client_info_application_version = Some(new_value.to_string());
        self
    }
    /// Application ID, such as the package name on Android and the bundle identifier on iOS platforms.
    ///
    /// Sets the *client info.application id* query property to the given value.
    pub fn client_info_application_id(mut self, new_value: &str) -> TerraintileGetCall<'a, S> {
        self._client_info_application_id = Some(new_value.to_string());
        self
    }
    /// API client name and version. For example, the SDK calling the API. The exact format is up to the client.
    ///
    /// Sets the *client info.api client* query property to the given value.
    pub fn client_info_api_client(mut self, new_value: &str) -> TerraintileGetCall<'a, S> {
        self._client_info_api_client = Some(new_value.to_string());
        self
    }
    /// The precision of terrain altitudes in centimeters. Possible values: between 1 (cm level precision) and 1,000,000 (10-kilometer level precision).
    ///
    /// Sets the *altitude precision centimeters* query property to the given value.
    pub fn altitude_precision_centimeters(mut self, new_value: i32) -> TerraintileGetCall<'a, S> {
        self._altitude_precision_centimeters = Some(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> TerraintileGetCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> TerraintileGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


