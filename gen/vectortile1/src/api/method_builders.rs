use super::*;
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

   pub(super) hub: &'a SemanticTile<S>,
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

   pub(super) hub: &'a SemanticTile<S>,
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



