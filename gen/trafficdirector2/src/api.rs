use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
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
    /// See, edit, configure, and delete your Google Cloud Platform data
    CloudPlatform,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::CloudPlatform
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all TrafficDirectorService related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_trafficdirector2 as trafficdirector2;
/// use trafficdirector2::api::ClientStatusRequest;
/// use trafficdirector2::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use trafficdirector2::TrafficDirectorService;
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
/// let mut hub = TrafficDirectorService::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ClientStatusRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.discovery().client_status(req)
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
pub struct TrafficDirectorService<C> {
    client: RefCell<C>,
    auth: RefCell<oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C> client::Hub for TrafficDirectorService<C> {}

impl<'a, C> TrafficDirectorService<C>
    where  C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {

    pub fn new(client: C, authenticator: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>) -> TrafficDirectorService<C> {
        TrafficDirectorService {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/2.0.0".to_string(),
            _base_url: "https://trafficdirector.googleapis.com/".to_string(),
            _root_url: "https://trafficdirector.googleapis.com/".to_string(),
        }
    }

    pub fn discovery(&'a self) -> DiscoveryMethods<'a, C> {
        DiscoveryMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/2.0.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://trafficdirector.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://trafficdirector.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Addresses specify either a logical or physical address and port, which are used to tell Envoy where to bind/listen, connect to upstream and find management servers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Address {
    /// no description provided
    pub pipe: Option<Pipe>,
    /// no description provided
    #[serde(rename="socketAddress")]
    pub socket_address: Option<SocketAddress>,
}

impl client::Part for Address {}


/// BuildVersion combines SemVer version of extension with free-form build information (i.e. 'alpha', 'private-build') as a set of strings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildVersion {
    /// Free-form build information. Envoy defines several well known keys in the source/common/version/version.h file
    pub metadata: Option<HashMap<String, String>>,
    /// SemVer version of extension.
    pub version: Option<SemanticVersion>,
}

impl client::Part for BuildVersion {}


/// All xds configs for a particular client.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClientConfig {
    /// Node for a particular client.
    pub node: Option<Node>,
    /// no description provided
    #[serde(rename="xdsConfig")]
    pub xds_config: Option<Vec<PerXdsConfig>>,
}

impl client::Part for ClientConfig {}


/// Request for client status of clients identified by a list of NodeMatchers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [client_status discovery](DiscoveryClientStatuCall) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClientStatusRequest {
    /// Management server can use these match criteria to identify clients. The match follows OR semantics.
    #[serde(rename="nodeMatchers")]
    pub node_matchers: Option<Vec<NodeMatcher>>,
}

impl client::RequestValue for ClientStatusRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [client_status discovery](DiscoveryClientStatuCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClientStatusResponse {
    /// Client configs for the clients specified in the ClientStatusRequest.
    pub config: Option<Vec<ClientConfig>>,
}

impl client::ResponseResult for ClientStatusResponse {}


/// Envoy's cluster manager fills this message with all currently known clusters. Cluster configuration information can be used to recreate an Envoy configuration by populating all clusters as static clusters or by returning them in a CDS response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClustersConfigDump {
    /// The dynamically loaded active clusters. These are clusters that are available to service data plane traffic.
    #[serde(rename="dynamicActiveClusters")]
    pub dynamic_active_clusters: Option<Vec<DynamicCluster>>,
    /// The dynamically loaded warming clusters. These are clusters that are currently undergoing warming in preparation to service data plane traffic. Note that if attempting to recreate an Envoy configuration from a configuration dump, the warming clusters should generally be discarded.
    #[serde(rename="dynamicWarmingClusters")]
    pub dynamic_warming_clusters: Option<Vec<DynamicCluster>>,
    /// The statically loaded cluster configs.
    #[serde(rename="staticClusters")]
    pub static_clusters: Option<Vec<StaticCluster>>,
    /// This is the :ref:`version_info ` in the last processed CDS discovery response. If there are only static bootstrap clusters, this field will be "".
    #[serde(rename="versionInfo")]
    pub version_info: Option<String>,
}

impl client::Part for ClustersConfigDump {}


/// Specifies the way to match a double value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DoubleMatcher {
    /// If specified, the input double value must be equal to the value specified here.
    pub exact: Option<f64>,
    /// If specified, the input double value must be in the range specified here. Note: The range is using half-open interval semantics [start, end).
    pub range: Option<DoubleRange>,
}

impl client::Part for DoubleMatcher {}


/// Specifies the double start and end of the range using half-open interval semantics [start, end).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DoubleRange {
    /// end of the range (exclusive)
    pub end: Option<f64>,
    /// start of the range (inclusive)
    pub start: Option<f64>,
}

impl client::Part for DoubleRange {}


/// Describes a dynamically loaded cluster via the CDS API.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicCluster {
    /// The cluster config.
    pub cluster: Option<HashMap<String, String>>,
    /// The timestamp when the Cluster was last updated.
    #[serde(rename="lastUpdated")]
    pub last_updated: Option<String>,
    /// This is the per-resource version information. This version is currently taken from the :ref:`version_info ` field at the time that the cluster was loaded. In the future, discrete per-cluster versions may be supported by the API.
    #[serde(rename="versionInfo")]
    pub version_info: Option<String>,
}

impl client::Part for DynamicCluster {}


/// Describes a dynamically loaded listener via the LDS API. [#next-free-field: 6]
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicListener {
    /// The listener state for any active listener by this name. These are listeners that are available to service data plane traffic.
    #[serde(rename="activeState")]
    pub active_state: Option<DynamicListenerState>,
    /// The listener state for any draining listener by this name. These are listeners that are currently undergoing draining in preparation to stop servicing data plane traffic. Note that if attempting to recreate an Envoy configuration from a configuration dump, the draining listeners should generally be discarded.
    #[serde(rename="drainingState")]
    pub draining_state: Option<DynamicListenerState>,
    /// Set if the last update failed, cleared after the next successful update.
    #[serde(rename="errorState")]
    pub error_state: Option<UpdateFailureState>,
    /// The name or unique id of this listener, pulled from the DynamicListenerState config.
    pub name: Option<String>,
    /// The listener state for any warming listener by this name. These are listeners that are currently undergoing warming in preparation to service data plane traffic. Note that if attempting to recreate an Envoy configuration from a configuration dump, the warming listeners should generally be discarded.
    #[serde(rename="warmingState")]
    pub warming_state: Option<DynamicListenerState>,
}

impl client::Part for DynamicListener {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicListenerState {
    /// The timestamp when the Listener was last successfully updated.
    #[serde(rename="lastUpdated")]
    pub last_updated: Option<String>,
    /// The listener config.
    pub listener: Option<HashMap<String, String>>,
    /// This is the per-resource version information. This version is currently taken from the :ref:`version_info ` field at the time that the listener was loaded. In the future, discrete per-listener versions may be supported by the API.
    #[serde(rename="versionInfo")]
    pub version_info: Option<String>,
}

impl client::Part for DynamicListenerState {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicRouteConfig {
    /// The timestamp when the Route was last updated.
    #[serde(rename="lastUpdated")]
    pub last_updated: Option<String>,
    /// The route config.
    #[serde(rename="routeConfig")]
    pub route_config: Option<HashMap<String, String>>,
    /// This is the per-resource version information. This version is currently taken from the :ref:`version_info ` field at the time that the route configuration was loaded.
    #[serde(rename="versionInfo")]
    pub version_info: Option<String>,
}

impl client::Part for DynamicRouteConfig {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicScopedRouteConfigs {
    /// The timestamp when the scoped route config set was last updated.
    #[serde(rename="lastUpdated")]
    pub last_updated: Option<String>,
    /// The name assigned to the scoped route configurations.
    pub name: Option<String>,
    /// The scoped route configurations.
    #[serde(rename="scopedRouteConfigs")]
    pub scoped_route_configs: Option<Vec<HashMap<String, String>>>,
    /// This is the per-resource version information. This version is currently taken from the :ref:`version_info ` field at the time that the scoped routes configuration was loaded.
    #[serde(rename="versionInfo")]
    pub version_info: Option<String>,
}

impl client::Part for DynamicScopedRouteConfigs {}


/// Version and identification for an Envoy extension. [#next-free-field: 6]
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Extension {
    /// Category of the extension. Extension category names use reverse DNS notation. For instance "envoy.filters.listener" for Envoy's built-in listener filters or "com.acme.filters.http" for HTTP filters from acme.com vendor. [#comment:
    pub category: Option<String>,
    /// Indicates that the extension is present but was disabled via dynamic configuration.
    pub disabled: Option<bool>,
    /// This is the name of the Envoy filter as specified in the Envoy configuration, e.g. envoy.filters.http.router, com.acme.widget.
    pub name: Option<String>,
    /// [#not-implemented-hide:] Type descriptor of extension configuration proto. [#comment:
    #[serde(rename="typeDescriptor")]
    pub type_descriptor: Option<String>,
    /// The version is a property of the extension and maintained independently of other extensions and the Envoy API. This field is not set when extension did not provide version information.
    pub version: Option<BuildVersion>,
}

impl client::Part for Extension {}


/// Google's `RE2 `_ regex engine. The regex string must adhere to the documented `syntax `_. The engine is designed to complete execution in linear time as well as limit the amount of memory used. Envoy supports program size checking via runtime. The runtime keys `re2.max_program_size.error_level` and `re2.max_program_size.warn_level` can be set to integers as the maximum program size or complexity that a compiled regex can have before an exception is thrown or a warning is logged, respectively. `re2.max_program_size.error_level` defaults to 100, and `re2.max_program_size.warn_level` has no default if unset (will not check/log a warning). Envoy emits two stats for tracking the program size of regexes: the histogram `re2.program_size`, which records the program size, and the counter `re2.exceeded_warn_level`, which is incremented each time the program size exceeds the warn level threshold.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleRE2 {
    /// This field controls the RE2 "program size" which is a rough estimate of how complex a compiled regex is to evaluate. A regex that has a program size greater than the configured value will fail to compile. In this case, the configured max program size can be increased or the regex can be simplified. If not specified, the default is 100. This field is deprecated; regexp validation should be performed on the management server instead of being done by each individual client.
    #[serde(rename="maxProgramSize")]
    pub max_program_size: Option<u32>,
}

impl client::Part for GoogleRE2 {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InlineScopedRouteConfigs {
    /// The timestamp when the scoped route config set was last updated.
    #[serde(rename="lastUpdated")]
    pub last_updated: Option<String>,
    /// The name assigned to the scoped route configurations.
    pub name: Option<String>,
    /// The scoped route configurations.
    #[serde(rename="scopedRouteConfigs")]
    pub scoped_route_configs: Option<Vec<HashMap<String, String>>>,
}

impl client::Part for InlineScopedRouteConfigs {}


/// Specifies the way to match a list value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMatcher {
    /// If specified, at least one of the values in the list must match the value specified.
    #[serde(rename="oneOf")]
    pub one_of: Option<ValueMatcher>,
}

impl client::Part for ListMatcher {}


/// Envoy's listener manager fills this message with all currently known listeners. Listener configuration information can be used to recreate an Envoy configuration by populating all listeners as static listeners or by returning them in a LDS response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListenersConfigDump {
    /// State for any warming, active, or draining listeners.
    #[serde(rename="dynamicListeners")]
    pub dynamic_listeners: Option<Vec<DynamicListener>>,
    /// The statically loaded listener configs.
    #[serde(rename="staticListeners")]
    pub static_listeners: Option<Vec<StaticListener>>,
    /// This is the :ref:`version_info ` in the last processed LDS discovery response. If there are only static bootstrap listeners, this field will be "".
    #[serde(rename="versionInfo")]
    pub version_info: Option<String>,
}

impl client::Part for ListenersConfigDump {}


/// Identifies location of where either Envoy runs or where upstream hosts run.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Locality {
    /// Region this :ref:`zone ` belongs to.
    pub region: Option<String>,
    /// When used for locality of upstream hosts, this field further splits zone into smaller chunks of sub-zones so they can be load balanced independently.
    #[serde(rename="subZone")]
    pub sub_zone: Option<String>,
    /// Defines the local service zone where Envoy is running. Though optional, it should be set if discovery service routing is used and the discovery service exposes :ref:`zone data `, either in this message or via :option:`--service-zone`. The meaning of zone is context dependent, e.g. `Availability Zone (AZ) `_ on AWS, `Zone `_ on GCP, etc.
    pub zone: Option<String>,
}

impl client::Part for Locality {}


/// Identifies a specific Envoy instance. The node identifier is presented to the management server, which may use this identifier to distinguish per Envoy configuration for serving. [#next-free-field: 12]
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    /// This is motivated by informing a management server during canary which version of Envoy is being tested in a heterogeneous fleet. This will be set by Envoy in management server RPCs. This field is deprecated in favor of the user_agent_name and user_agent_version values.
    #[serde(rename="buildVersion")]
    pub build_version: Option<String>,
    /// Client feature support list. These are well known features described in the Envoy API repository for a given major version of an API. Client features use reverse DNS naming scheme, for example `com.acme.feature`. See :ref:`the list of features ` that xDS client may support.
    #[serde(rename="clientFeatures")]
    pub client_features: Option<Vec<String>>,
    /// Defines the local service cluster name where Envoy is running. Though optional, it should be set if any of the following features are used: :ref:`statsd `, :ref:`health check cluster verification `, :ref:`runtime override directory `, :ref:`user agent addition `, :ref:`HTTP global rate limiting `, :ref:`CDS `, and :ref:`HTTP tracing `, either in this message or via :option:`--service-cluster`.
    pub cluster: Option<String>,
    /// List of extensions and their versions supported by the node.
    pub extensions: Option<Vec<Extension>>,
    /// An opaque node identifier for the Envoy node. This also provides the local service node name. It should be set if any of the following features are used: :ref:`statsd `, :ref:`CDS `, and :ref:`HTTP tracing `, either in this message or via :option:`--service-node`.
    pub id: Option<String>,
    /// Known listening ports on the node as a generic hint to the management server for filtering :ref:`listeners ` to be returned. For example, if there is a listener bound to port 80, the list can optionally contain the SocketAddress `(0.0.0.0,80)`. The field is optional and just a hint.
    #[serde(rename="listeningAddresses")]
    pub listening_addresses: Option<Vec<Address>>,
    /// Locality specifying where the Envoy instance is running.
    pub locality: Option<Locality>,
    /// Opaque metadata extending the node identifier. Envoy will pass this directly to the management server.
    pub metadata: Option<HashMap<String, String>>,
    /// Structured version of the entity requesting config.
    #[serde(rename="userAgentBuildVersion")]
    pub user_agent_build_version: Option<BuildVersion>,
    /// Free-form string that identifies the entity requesting config. E.g. "envoy" or "grpc"
    #[serde(rename="userAgentName")]
    pub user_agent_name: Option<String>,
    /// Free-form string that identifies the version of the entity requesting config. E.g. "1.12.2" or "abcd1234", or "SpecialEnvoyBuild"
    #[serde(rename="userAgentVersion")]
    pub user_agent_version: Option<String>,
}

impl client::Part for Node {}


/// Specifies the way to match a Node. The match follows AND semantics.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NodeMatcher {
    /// Specifies match criteria on the node id.
    #[serde(rename="nodeId")]
    pub node_id: Option<StringMatcher>,
    /// Specifies match criteria on the node metadata.
    #[serde(rename="nodeMetadatas")]
    pub node_metadatas: Option<Vec<StructMatcher>>,
}

impl client::Part for NodeMatcher {}


/// NullMatch is an empty message to specify a null value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NullMatch { _never_set: Option<bool> }

impl client::Part for NullMatch {}


/// Specifies the segment in a path to retrieve value from Struct.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PathSegment {
    /// If specified, use the key to retrieve the value in a Struct.
    pub key: Option<String>,
}

impl client::Part for PathSegment {}


/// Detailed config (per xDS) with status. [#next-free-field: 6]
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PerXdsConfig {
    /// no description provided
    #[serde(rename="clusterConfig")]
    pub cluster_config: Option<ClustersConfigDump>,
    /// no description provided
    #[serde(rename="listenerConfig")]
    pub listener_config: Option<ListenersConfigDump>,
    /// no description provided
    #[serde(rename="routeConfig")]
    pub route_config: Option<RoutesConfigDump>,
    /// no description provided
    #[serde(rename="scopedRouteConfig")]
    pub scoped_route_config: Option<ScopedRoutesConfigDump>,
    /// no description provided
    pub status: Option<String>,
}

impl client::Part for PerXdsConfig {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Pipe {
    /// The mode for the Pipe. Not applicable for abstract sockets.
    pub mode: Option<u32>,
    /// Unix Domain Socket path. On Linux, paths starting with '@' will use the abstract namespace. The starting '@' is replaced by a null byte by Envoy. Paths starting with '@' will result in an error in environments other than Linux.
    pub path: Option<String>,
}

impl client::Part for Pipe {}


/// A regex matcher designed for safety when used with untrusted input.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RegexMatcher {
    /// Google's RE2 regex engine.
    #[serde(rename="googleRe2")]
    pub google_re2: Option<GoogleRE2>,
    /// The regex match string. The string must be supported by the configured engine.
    pub regex: Option<String>,
}

impl client::Part for RegexMatcher {}


/// Envoy's RDS implementation fills this message with all currently loaded routes, as described by their RouteConfiguration objects. Static routes that are either defined in the bootstrap configuration or defined inline while configuring listeners are separated from those configured dynamically via RDS. Route configuration information can be used to recreate an Envoy configuration by populating all routes as static routes or by returning them in RDS responses.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RoutesConfigDump {
    /// The dynamically loaded route configs.
    #[serde(rename="dynamicRouteConfigs")]
    pub dynamic_route_configs: Option<Vec<DynamicRouteConfig>>,
    /// The statically loaded route configs.
    #[serde(rename="staticRouteConfigs")]
    pub static_route_configs: Option<Vec<StaticRouteConfig>>,
}

impl client::Part for RoutesConfigDump {}


/// Envoy's scoped RDS implementation fills this message with all currently loaded route configuration scopes (defined via ScopedRouteConfigurationsSet protos). This message lists both the scopes defined inline with the higher order object (i.e., the HttpConnectionManager) and the dynamically obtained scopes via the SRDS API.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ScopedRoutesConfigDump {
    /// The dynamically loaded scoped route configs.
    #[serde(rename="dynamicScopedRouteConfigs")]
    pub dynamic_scoped_route_configs: Option<Vec<DynamicScopedRouteConfigs>>,
    /// The statically loaded scoped route configs.
    #[serde(rename="inlineScopedRouteConfigs")]
    pub inline_scoped_route_configs: Option<Vec<InlineScopedRouteConfigs>>,
}

impl client::Part for ScopedRoutesConfigDump {}


/// Envoy uses SemVer (https://semver.org/). Major/minor versions indicate expected behaviors and APIs, the patch version field is used only for security fixes and can be generally ignored.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SemanticVersion {
    /// no description provided
    #[serde(rename="majorNumber")]
    pub major_number: Option<u32>,
    /// no description provided
    #[serde(rename="minorNumber")]
    pub minor_number: Option<u32>,
    /// no description provided
    pub patch: Option<u32>,
}

impl client::Part for SemanticVersion {}


/// [#next-free-field: 7]
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SocketAddress {
    /// The address for this socket. :ref:`Listeners ` will bind to the address. An empty address is not allowed. Specify ``0.0.0.0`` or ``::`` to bind to any address. [#comment:TODO(zuercher) reinstate when implemented: It is possible to distinguish a Listener address via the prefix/suffix matching in :ref:`FilterChainMatch `.] When used within an upstream :ref:`BindConfig `, the address controls the source address of outbound connections. For :ref:`clusters `, the cluster type determines whether the address must be an IP (*STATIC* or *EDS* clusters) or a hostname resolved by DNS (*STRICT_DNS* or *LOGICAL_DNS* clusters). Address resolution can be customized via :ref:`resolver_name `.
    pub address: Option<String>,
    /// When binding to an IPv6 address above, this enables `IPv4 compatibility `_. Binding to ``::`` will allow both IPv4 and IPv6 connections, with peer IPv4 addresses mapped into IPv6 space as ``::FFFF:``.
    #[serde(rename="ipv4Compat")]
    pub ipv4_compat: Option<bool>,
    /// This is only valid if :ref:`resolver_name ` is specified below and the named resolver is capable of named port resolution.
    #[serde(rename="namedPort")]
    pub named_port: Option<String>,
    /// no description provided
    #[serde(rename="portValue")]
    pub port_value: Option<u32>,
    /// no description provided
    pub protocol: Option<String>,
    /// The name of the custom resolver. This must have been registered with Envoy. If this is empty, a context dependent default applies. If the address is a concrete IP address, no resolution will occur. If address is a hostname this should be set for resolution other than DNS. Specifying a custom resolver with *STRICT_DNS* or *LOGICAL_DNS* will generate an error at runtime.
    #[serde(rename="resolverName")]
    pub resolver_name: Option<String>,
}

impl client::Part for SocketAddress {}


/// Describes a statically loaded cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StaticCluster {
    /// The cluster config.
    pub cluster: Option<HashMap<String, String>>,
    /// The timestamp when the Cluster was last updated.
    #[serde(rename="lastUpdated")]
    pub last_updated: Option<String>,
}

impl client::Part for StaticCluster {}


/// Describes a statically loaded listener.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StaticListener {
    /// The timestamp when the Listener was last successfully updated.
    #[serde(rename="lastUpdated")]
    pub last_updated: Option<String>,
    /// The listener config.
    pub listener: Option<HashMap<String, String>>,
}

impl client::Part for StaticListener {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StaticRouteConfig {
    /// The timestamp when the Route was last updated.
    #[serde(rename="lastUpdated")]
    pub last_updated: Option<String>,
    /// The route config.
    #[serde(rename="routeConfig")]
    pub route_config: Option<HashMap<String, String>>,
}

impl client::Part for StaticRouteConfig {}


/// Specifies the way to match a string. [#next-free-field: 7]
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StringMatcher {
    /// The input string must match exactly the string specified here. Examples: * *abc* only matches the value *abc*.
    pub exact: Option<String>,
    /// If true, indicates the exact/prefix/suffix matching should be case insensitive. This has no effect for the safe_regex match. For example, the matcher *data* will match both input string *Data* and *data* if set to true.
    #[serde(rename="ignoreCase")]
    pub ignore_case: Option<bool>,
    /// The input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead. Examples: * *abc* matches the value *abc.xyz*
    pub prefix: Option<String>,
    /// The input string must match the regular expression specified here. The regex grammar is defined `here `_. Examples: * The regex ``\d{3}`` matches the value *123* * The regex ``\d{3}`` does not match the value *1234* * The regex ``\d{3}`` does not match the value *123.456* .. attention:: This field has been deprecated in favor of `safe_regex` as it is not safe for use with untrusted input in all cases.
    pub regex: Option<String>,
    /// The input string must match the regular expression specified here.
    #[serde(rename="safeRegex")]
    pub safe_regex: Option<RegexMatcher>,
    /// The input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead. Examples: * *abc* matches the value *xyz.abc*
    pub suffix: Option<String>,
}

impl client::Part for StringMatcher {}


/// StructMatcher provides a general interface to check if a given value is matched in google.protobuf.Struct. It uses `path` to retrieve the value from the struct and then check if it's matched to the specified value. For example, for the following Struct: .. code-block:: yaml fields: a: struct_value: fields: b: struct_value: fields: c: string_value: pro t: list_value: values: - string_value: m - string_value: n The following MetadataMatcher is matched as the path [a, b, c] will retrieve a string value "pro" from the Metadata which is matched to the specified prefix match. .. code-block:: yaml path: - key: a - key: b - key: c value: string_match: prefix: pr The following StructMatcher is matched as the code will match one of the string values in the list at the path [a, t]. .. code-block:: yaml path: - key: a - key: t value: list_match: one_of: string_match: exact: m An example use of StructMatcher is to match metadata in envoy.v*.core.Node.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StructMatcher {
    /// The path to retrieve the Value from the Struct.
    pub path: Option<Vec<PathSegment>>,
    /// The StructMatcher is matched if the value retrieved by path is matched to this value.
    pub value: Option<ValueMatcher>,
}

impl client::Part for StructMatcher {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateFailureState {
    /// Details about the last failed update attempt.
    pub details: Option<String>,
    /// What the component configuration would have been if the update had succeeded.
    #[serde(rename="failedConfiguration")]
    pub failed_configuration: Option<HashMap<String, String>>,
    /// Time of the latest failed update attempt.
    #[serde(rename="lastUpdateAttempt")]
    pub last_update_attempt: Option<String>,
}

impl client::Part for UpdateFailureState {}


/// Specifies the way to match a ProtobufWkt::Value. Primitive values and ListValue are supported. StructValue is not supported and is always not matched. [#next-free-field: 7]
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ValueMatcher {
    /// If specified, a match occurs if and only if the target value is a bool value and is equal to this field.
    #[serde(rename="boolMatch")]
    pub bool_match: Option<bool>,
    /// If specified, a match occurs if and only if the target value is a double value and is matched to this field.
    #[serde(rename="doubleMatch")]
    pub double_match: Option<DoubleMatcher>,
    /// If specified, a match occurs if and only if the target value is a list value and is matched to this field.
    #[serde(rename="listMatch")]
    pub list_match: Option<ListMatcher>,
    /// If specified, a match occurs if and only if the target value is a NullValue.
    #[serde(rename="nullMatch")]
    pub null_match: Option<NullMatch>,
    /// If specified, value match will be performed based on whether the path is referring to a valid primitive value in the metadata. If the path is referring to a non-primitive value, the result is always not matched.
    #[serde(rename="presentMatch")]
    pub present_match: Option<bool>,
    /// If specified, a match occurs if and only if the target value is a string value and is matched to this field.
    #[serde(rename="stringMatch")]
    pub string_match: Option<StringMatcher>,
}

impl client::Part for ValueMatcher {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *discovery* resources.
/// It is not used directly, but through the `TrafficDirectorService` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_trafficdirector2 as trafficdirector2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use trafficdirector2::TrafficDirectorService;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = TrafficDirectorService::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `client_status(...)`
/// // to build up your call.
/// let rb = hub.discovery();
/// # }
/// ```
pub struct DiscoveryMethods<'a, C>
    where C: 'a {

    hub: &'a TrafficDirectorService<C>,
}

impl<'a, C> client::MethodsBuilder for DiscoveryMethods<'a, C> {}

impl<'a, C> DiscoveryMethods<'a, C> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// 
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn client_status(&self, request: ClientStatusRequest) -> DiscoveryClientStatuCall<'a, C> {
        DiscoveryClientStatuCall {
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

/// 
///
/// A builder for the *client_status* method supported by a *discovery* resource.
/// It is not used directly, but through a `DiscoveryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_trafficdirector2 as trafficdirector2;
/// use trafficdirector2::api::ClientStatusRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use trafficdirector2::TrafficDirectorService;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = TrafficDirectorService::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ClientStatusRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.discovery().client_status(req)
///              .doit().await;
/// # }
/// ```
pub struct DiscoveryClientStatuCall<'a, C>
    where C: 'a {

    hub: &'a TrafficDirectorService<C>,
    _request: ClientStatusRequest,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for DiscoveryClientStatuCall<'a, C> {}

impl<'a, C> DiscoveryClientStatuCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ClientStatusResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "trafficdirector.discovery.client_status",
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

        let mut url = self.hub._base_url.clone() + "v2/discovery:client_status";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
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
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
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
                    let (res_parts, res_body) = res.into_parts();
                    let res_body_string: String = String::from_utf8(
                        hyper::body::to_bytes(res_body)
                            .await
                            .unwrap()
                            .into_iter()
                            .collect(),
                    )
                    .unwrap();
                    let reconstructed_result =
                        hyper::Response::from_parts(res_parts, res_body_string.clone().into());

                    if !reconstructed_result.status().is_success() {
                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&reconstructed_result,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(reconstructed_result)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (reconstructed_result, decoded),
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
    pub fn request(mut self, new_value: ClientStatusRequest) -> DiscoveryClientStatuCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DiscoveryClientStatuCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DiscoveryClientStatuCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DiscoveryClientStatuCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


