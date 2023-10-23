use super::*;
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
///              .client_info_platform(&Default::default())
///              .client_info_operating_system("dolore")
///              .client_info_device_model("voluptua.")
///              .client_info_application_version("amet.")
///              .client_info_application_id("ea")
///              .client_info_api_client("sadipscing")
///              .always_include_building_footprints(true)
///              .doit().await;
/// # }
/// ```
pub struct FeaturetileGetCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a SemanticTile<S>,
   pub(super) _name: String,
   pub(super) _region_code: Option<String>,
   pub(super) _language_code: Option<String>,
   pub(super) _enable_unclipped_buildings: Option<bool>,
   pub(super) _enable_private_roads: Option<bool>,
   pub(super) _enable_political_features: Option<bool>,
   pub(super) _enable_modeled_volumes: Option<bool>,
   pub(super) _enable_feature_names: Option<bool>,
   pub(super) _enable_detailed_highway_types: Option<bool>,
   pub(super) _client_tile_version_id: Option<String>,
   pub(super) _client_info_user_id: Option<String>,
   pub(super) _client_info_platform: Option<FeaturetileClientInfoPlatformEnum>,
   pub(super) _client_info_operating_system: Option<String>,
   pub(super) _client_info_device_model: Option<String>,
   pub(super) _client_info_application_version: Option<String>,
   pub(super) _client_info_application_id: Option<String>,
   pub(super) _client_info_api_client: Option<String>,
   pub(super) _always_include_building_footprints: Option<bool>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
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
        params.push("name", &self._name);
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
    pub fn client_info_platform(mut self, new_value: &FeaturetileClientInfoPlatformEnum) -> FeaturetileGetCall<'a, S> {
        self._client_info_platform = Some(new_value.clone());
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
///              .add_terrain_formats(&Default::default())
///              .min_elevation_resolution_cells(-7)
///              .max_elevation_resolution_cells(-27)
///              .client_info_user_id("sed")
///              .client_info_platform(&Default::default())
///              .client_info_operating_system("sit")
///              .client_info_device_model("et")
///              .client_info_application_version("tempor")
///              .client_info_application_id("aliquyam")
///              .client_info_api_client("ipsum")
///              .altitude_precision_centimeters(-18)
///              .doit().await;
/// # }
/// ```
pub struct TerraintileGetCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a SemanticTile<S>,
   pub(super) _name: String,
   pub(super) _terrain_formats: Option<TerraintileTerrainFormatsEnum>,
   pub(super) _min_elevation_resolution_cells: Option<i32>,
   pub(super) _max_elevation_resolution_cells: Option<i32>,
   pub(super) _client_info_user_id: Option<String>,
   pub(super) _client_info_platform: Option<TerraintileClientInfoPlatformEnum>,
   pub(super) _client_info_operating_system: Option<String>,
   pub(super) _client_info_device_model: Option<String>,
   pub(super) _client_info_application_version: Option<String>,
   pub(super) _client_info_application_id: Option<String>,
   pub(super) _client_info_api_client: Option<String>,
   pub(super) _altitude_precision_centimeters: Option<i32>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
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
        params.push("name", &self._name);
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
    pub fn add_terrain_formats(mut self, new_value: &TerraintileTerrainFormatsEnum) -> TerraintileGetCall<'a, S> {
        self._terrain_formats.push(new_value.clone());
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
    pub fn client_info_platform(mut self, new_value: &TerraintileClientInfoPlatformEnum) -> TerraintileGetCall<'a, S> {
        self._client_info_platform = Some(new_value.clone());
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


