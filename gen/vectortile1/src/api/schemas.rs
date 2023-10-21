use super::*;
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
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
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


