use super::*;
/// Message that represents an arbitrary HTTP body. It should only be used for payload formats that can’t be represented as JSON, such as raw binary or an HTML page. This message can be used both in streaming and non-streaming API methods in the request as well as the response. It can be used as a top-level request field, which is convenient if one wants to extract parameters from either the URL or HTTP template into the request fields and also want access to the raw HTTP body. Example: message GetResourceRequest { // A unique request id. string request_id = 1; // The raw HTTP body is bound to this field. google.api.HttpBody http_body = 2; } service ResourceService { rpc GetResource(GetResourceRequest) returns (google.api.HttpBody); rpc UpdateResource(google.api.HttpBody) returns (google.protobuf.Empty); } Example with streaming methods: service CaldavService { rpc GetCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); rpc UpdateCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); } Use of this type only changes how the request and response bodies are handled, all other features will continue to work unchanged.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apis revisions get organizations](OrganizationApiRevisionGetCall) (response)
/// * [apis revisions update api proxy revision organizations](OrganizationApiRevisionUpdateApiProxyRevisionCall) (request)
/// * [apis create organizations](OrganizationApiCreateCall) (request)
/// * [environments keystores aliases create organizations](OrganizationEnvironmentKeystoreAliasCreateCall) (request)
/// * [environments keystores aliases csr organizations](OrganizationEnvironmentKeystoreAliasCsrCall) (response)
/// * [environments keystores aliases get certificate organizations](OrganizationEnvironmentKeystoreAliasGetCertificateCall) (response)
/// * [environments keystores aliases update organizations](OrganizationEnvironmentKeystoreAliasUpdateCall) (request)
/// * [environments queries get result organizations](OrganizationEnvironmentQueryGetResultCall) (response)
/// * [environments resourcefiles create organizations](OrganizationEnvironmentResourcefileCreateCall) (request)
/// * [environments resourcefiles get organizations](OrganizationEnvironmentResourcefileGetCall) (response)
/// * [environments resourcefiles update organizations](OrganizationEnvironmentResourcefileUpdateCall) (request)
/// * [environments security reports get result organizations](OrganizationEnvironmentSecurityReportGetResultCall) (response)
/// * [host queries get result organizations](OrganizationHostQueryGetResultCall) (response)
/// * [host security reports get result organizations](OrganizationHostSecurityReportGetResultCall) (response)
/// * [sharedflows revisions get organizations](OrganizationSharedflowRevisionGetCall) (response)
/// * [sharedflows revisions update shared flow revision organizations](OrganizationSharedflowRevisionUpdateSharedFlowRevisionCall) (request)
/// * [sharedflows create organizations](OrganizationSharedflowCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleApiHttpBody {
    /// The HTTP Content-Type header value specifying the content type of the body.
    #[serde(rename="contentType")]
    
    pub content_type: Option<String>,
    /// The HTTP request/response body as raw binary.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// Application specific response metadata. Must be set in the first response for streaming APIs.
    
    pub extensions: Option<Vec<HashMap<String, json::Value>>>,
}

impl client::RequestValue for GoogleApiHttpBody {}
impl client::ResponseResult for GoogleApiHttpBody {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Access {
    /// no description provided
    #[serde(rename="Get")]
    
    pub get: Option<GoogleCloudApigeeV1AccessGet>,
    /// no description provided
    #[serde(rename="Remove")]
    
    pub remove: Option<GoogleCloudApigeeV1AccessRemove>,
    /// no description provided
    #[serde(rename="Set")]
    
    pub set: Option<GoogleCloudApigeeV1AccessSet>,
}

impl client::Part for GoogleCloudApigeeV1Access {}


/// Get action. For example, "Get" : { "name" : "target.name", "value" : "default" }
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1AccessGet {
    /// no description provided
    
    pub name: Option<String>,
    /// no description provided
    
    pub value: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1AccessGet {}


/// Remove action. For example, "Remove" : { "name" : "target.name", "success" : true }
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1AccessRemove {
    /// no description provided
    
    pub name: Option<String>,
    /// no description provided
    
    pub success: Option<bool>,
}

impl client::Part for GoogleCloudApigeeV1AccessRemove {}


/// Set action. For example, "Set" : { "name" : "target.name", "success" : true, "value" : "default" }
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1AccessSet {
    /// no description provided
    
    pub name: Option<String>,
    /// no description provided
    
    pub success: Option<bool>,
    /// no description provided
    
    pub value: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1AccessSet {}


/// Request for ActivateNatAddressRequest. Activate the nat address request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances nat addresses activate organizations](OrganizationInstanceNatAddressActivateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ActivateNatAddressRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudApigeeV1ActivateNatAddressRequest {}


/// Add-on configurations for the Apigee organization.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1AddonsConfig {
    /// Configuration for the Advanced API Ops add-on.
    #[serde(rename="advancedApiOpsConfig")]
    
    pub advanced_api_ops_config: Option<GoogleCloudApigeeV1AdvancedApiOpsConfig>,
    /// Configuration for the API Security add-on.
    #[serde(rename="apiSecurityConfig")]
    
    pub api_security_config: Option<GoogleCloudApigeeV1ApiSecurityConfig>,
    /// Configuration for the Connectors Platform add-on.
    #[serde(rename="connectorsPlatformConfig")]
    
    pub connectors_platform_config: Option<GoogleCloudApigeeV1ConnectorsPlatformConfig>,
    /// Configuration for the Integration add-on.
    #[serde(rename="integrationConfig")]
    
    pub integration_config: Option<GoogleCloudApigeeV1IntegrationConfig>,
    /// Configuration for the Monetization add-on.
    #[serde(rename="monetizationConfig")]
    
    pub monetization_config: Option<GoogleCloudApigeeV1MonetizationConfig>,
}

impl client::Part for GoogleCloudApigeeV1AddonsConfig {}


/// Request for AdjustDeveloperBalance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [developers balance adjust organizations](OrganizationDeveloperBalanceAdjustCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1AdjustDeveloperBalanceRequest {
    /// * A positive value of `adjustment` means that that the API provider wants to adjust the balance for an under-charged developer i.e. the balance of the developer will decrease. * A negative value of `adjustment` means that that the API provider wants to adjust the balance for an over-charged developer i.e. the balance of the developer will increase.
    
    pub adjustment: Option<GoogleTypeMoney>,
}

impl client::RequestValue for GoogleCloudApigeeV1AdjustDeveloperBalanceRequest {}


/// Configuration for the Advanced API Ops add-on.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1AdvancedApiOpsConfig {
    /// Flag that specifies whether the Advanced API Ops add-on is enabled.
    
    pub enabled: Option<bool>,
}

impl client::Part for GoogleCloudApigeeV1AdvancedApiOpsConfig {}


/// Reference to a certificate or key/certificate pair.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments keystores aliases create organizations](OrganizationEnvironmentKeystoreAliasCreateCall) (response)
/// * [environments keystores aliases delete organizations](OrganizationEnvironmentKeystoreAliasDeleteCall) (response)
/// * [environments keystores aliases get organizations](OrganizationEnvironmentKeystoreAliasGetCall) (response)
/// * [environments keystores aliases update organizations](OrganizationEnvironmentKeystoreAliasUpdateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Alias {
    /// Resource ID for this alias. Values must match the regular expression `[^/]{1,255}`.
    
    pub alias: Option<String>,
    /// Chain of certificates under this alias.
    #[serde(rename="certsInfo")]
    
    pub certs_info: Option<GoogleCloudApigeeV1Certificate>,
    /// Type of alias.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1Alias {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1AliasRevisionConfig {
    /// Location of the alias file. For example, a Google Cloud Storage URI.
    
    pub location: Option<String>,
    /// Name of the alias revision included in the keystore in the following format: `organizations/{org}/environments/{env}/keystores/{keystore}/aliases/{alias}/revisions/{rev}`
    
    pub name: Option<String>,
    /// no description provided
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1AliasRevisionConfig {}


/// the Api category resource wrapped with response status, error_code etc.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sites apicategories create organizations](OrganizationSiteApicategoryCreateCall) (response)
/// * [sites apicategories get organizations](OrganizationSiteApicategoryGetCall) (response)
/// * [sites apicategories patch organizations](OrganizationSiteApicategoryPatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ApiCategory {
    /// Details of category.
    
    pub data: Option<GoogleCloudApigeeV1ApiCategoryData>,
    /// ID that can be used to find errors in the log files.
    #[serde(rename="errorCode")]
    
    pub error_code: Option<String>,
    /// Description of the operation.
    
    pub message: Option<String>,
    /// ID that can be used to find request details in the log files.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
    /// Status of the operation.
    
    pub status: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ApiCategory {}


/// the Api category resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sites apicategories create organizations](OrganizationSiteApicategoryCreateCall) (request)
/// * [sites apicategories patch organizations](OrganizationSiteApicategoryPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ApiCategoryData {
    /// ID of the category (a UUID).
    
    pub id: Option<String>,
    /// Name of the category.
    
    pub name: Option<String>,
    /// Name of the portal.
    #[serde(rename="siteId")]
    
    pub site_id: Option<String>,
    /// Time the category was last modified in milliseconds since epoch.
    #[serde(rename="updateTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub update_time: Option<i64>,
}

impl client::RequestValue for GoogleCloudApigeeV1ApiCategoryData {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apiproducts create organizations](OrganizationApiproductCreateCall) (request|response)
/// * [apiproducts delete organizations](OrganizationApiproductDeleteCall) (response)
/// * [apiproducts get organizations](OrganizationApiproductGetCall) (response)
/// * [apiproducts update organizations](OrganizationApiproductUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ApiProduct {
    /// Comma-separated list of API resources to be bundled in the API product. By default, the resource paths are mapped from the `proxy.pathsuffix` variable. The proxy path suffix is defined as the URI fragment following the ProxyEndpoint base path. For example, if the `apiResources` element is defined to be `/forecastrss` and the base path defined for the API proxy is `/weather`, then only requests to `/weather/forecastrss` are permitted by the API product. You can select a specific path, or you can select all subpaths with the following wildcard: - `/**`: Indicates that all sub-URIs are included. - `/*` : Indicates that only URIs one level down are included. By default, / supports the same resources as /** as well as the base path defined by the API proxy. For example, if the base path of the API proxy is `/v1/weatherapikey`, then the API product supports requests to `/v1/weatherapikey` and to any sub-URIs, such as `/v1/weatherapikey/forecastrss`, `/v1/weatherapikey/region/CA`, and so on. For more information, see Managing API products.
    #[serde(rename="apiResources")]
    
    pub api_resources: Option<Vec<String>>,
    /// Flag that specifies how API keys are approved to access the APIs defined by the API product. If set to `manual`, the consumer key is generated and returned in "pending" state. In this case, the API keys won't work until they have been explicitly approved. If set to `auto`, the consumer key is generated and returned in "approved" state and can be used immediately. **Note:** Typically, `auto` is used to provide access to free or trial API products that provide limited quota or capabilities.
    #[serde(rename="approvalType")]
    
    pub approval_type: Option<String>,
    /// Array of attributes that may be used to extend the default API product profile with customer-specific metadata. You can specify a maximum of 18 attributes. Use this property to specify the access level of the API product as either `public`, `private`, or `internal`. Only products marked `public` are available to developers in the Apigee developer portal. For example, you can set a product to `internal` while it is in development and then change access to `public` when it is ready to release on the portal. API products marked as `private` do not appear on the portal, but can be accessed by external developers.
    
    pub attributes: Option<Vec<GoogleCloudApigeeV1Attribute>>,
    /// Response only. Creation time of this environment as milliseconds since epoch.
    #[serde(rename="createdAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub created_at: Option<i64>,
    /// Description of the API product. Include key information about the API product that is not captured by other fields.
    
    pub description: Option<String>,
    /// Name displayed in the UI or developer portal to developers registering for API access.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Comma-separated list of environment names to which the API product is bound. Requests to environments that are not listed are rejected. By specifying one or more environments, you can bind the resources listed in the API product to a specific environment, preventing developers from accessing those resources through API proxies deployed in another environment. This setting is used, for example, to prevent resources associated with API proxies in `prod` from being accessed by API proxies deployed in `test`.
    
    pub environments: Option<Vec<String>>,
    /// Configuration used to group Apigee proxies or remote services with graphQL operation name, graphQL operation type and quotas. This grouping allows us to precisely set quota for a particular combination of graphQL name and operation type for a particular proxy request. If graphQL name is not set, this would imply quota will be applied on all graphQL requests matching the operation type.
    #[serde(rename="graphqlOperationGroup")]
    
    pub graphql_operation_group: Option<GoogleCloudApigeeV1GraphQLOperationGroup>,
    /// Response only. Modified time of this environment as milliseconds since epoch.
    #[serde(rename="lastModifiedAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_modified_at: Option<i64>,
    /// Internal name of the API product. Characters you can use in the name are restricted to: `A-Z0-9._\-$ %`. **Note:** The internal name cannot be edited when updating the API product.
    
    pub name: Option<String>,
    /// Configuration used to group Apigee proxies or remote services with resources, method types, and quotas. The resource refers to the resource URI (excluding the base path). With this grouping, the API product creator is able to fine-tune and give precise control over which REST methods have access to specific resources and how many calls can be made (using the `quota` setting). **Note:** The `api_resources` setting cannot be specified for both the API product and operation group; otherwise the call will fail.
    #[serde(rename="operationGroup")]
    
    pub operation_group: Option<GoogleCloudApigeeV1OperationGroup>,
    /// Comma-separated list of API proxy names to which this API product is bound. By specifying API proxies, you can associate resources in the API product with specific API proxies, preventing developers from accessing those resources through other API proxies. Apigee rejects requests to API proxies that are not listed. **Note:** The API proxy names must already exist in the specified environment as they will be validated upon creation.
    
    pub proxies: Option<Vec<String>>,
    /// Number of request messages permitted per app by this API product for the specified `quotaInterval` and `quotaTimeUnit`. For example, a `quota` of 50, for a `quotaInterval` of 12 and a `quotaTimeUnit` of hours means 50 requests are allowed every 12 hours.
    
    pub quota: Option<String>,
    /// Scope of the quota decides how the quota counter gets applied and evaluate for quota violation. If the Scope is set as PROXY, then all the operations defined for the APIproduct that are associated with the same proxy will share the same quota counter set at the APIproduct level, making it a global counter at a proxy level. If the Scope is set as OPERATION, then each operations get the counter set at the API product dedicated, making it a local counter. Note that, the QuotaCounterScope applies only when an operation does not have dedicated quota set for itself.
    #[serde(rename="quotaCounterScope")]
    
    pub quota_counter_scope: Option<String>,
    /// Time interval over which the number of request messages is calculated.
    #[serde(rename="quotaInterval")]
    
    pub quota_interval: Option<String>,
    /// Time unit defined for the `quotaInterval`. Valid values include `minute`, `hour`, `day`, or `month`.
    #[serde(rename="quotaTimeUnit")]
    
    pub quota_time_unit: Option<String>,
    /// Comma-separated list of OAuth scopes that are validated at runtime. Apigee validates that the scopes in any access token presented match the scopes defined in the OAuth policy associated with the API product.
    
    pub scopes: Option<Vec<String>>,
}

impl client::RequestValue for GoogleCloudApigeeV1ApiProduct {}
impl client::ResponseResult for GoogleCloudApigeeV1ApiProduct {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ApiProductRef {
    /// Name of the API product.
    
    pub apiproduct: Option<String>,
    /// Status of the API product. Valid values are `approved` or `revoked`.
    
    pub status: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1ApiProductRef {}


/// Metadata describing the API proxy
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apis delete organizations](OrganizationApiDeleteCall) (response)
/// * [apis get organizations](OrganizationApiGetCall) (response)
/// * [apis patch organizations](OrganizationApiPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ApiProxy {
    /// Output only. The type of the API proxy.
    #[serde(rename="apiProxyType")]
    
    pub api_proxy_type: Option<String>,
    /// User labels applied to this API Proxy.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The id of the most recently created revision for this api proxy.
    #[serde(rename="latestRevisionId")]
    
    pub latest_revision_id: Option<String>,
    /// Output only. Metadata describing the API proxy.
    #[serde(rename="metaData")]
    
    pub meta_data: Option<GoogleCloudApigeeV1EntityMetadata>,
    /// Output only. Name of the API proxy.
    
    pub name: Option<String>,
    /// Output only. Whether this proxy is read-only. A read-only proxy cannot have new revisions created through calls to CreateApiProxyRevision. A proxy is read-only if it was generated by an archive.
    #[serde(rename="readOnly")]
    
    pub read_only: Option<bool>,
    /// Output only. List of revisions defined for the API proxy.
    
    pub revision: Option<Vec<String>>,
}

impl client::RequestValue for GoogleCloudApigeeV1ApiProxy {}
impl client::ResponseResult for GoogleCloudApigeeV1ApiProxy {}


/// API proxy revision.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apis revisions delete organizations](OrganizationApiRevisionDeleteCall) (response)
/// * [apis revisions update api proxy revision organizations](OrganizationApiRevisionUpdateApiProxyRevisionCall) (response)
/// * [apis create organizations](OrganizationApiCreateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ApiProxyRevision {
    /// Output only. The archive that generated this proxy revision. This field is only present on proxy revisions that were generated by an archive. Proxies generated by archives cannot be updated, deleted, or deployed to other environments. Format: `organizations/*/environments/*/archiveDeployments/*`
    
    pub archive: Option<String>,
    /// Base URL of the API proxy.
    
    pub basepaths: Option<Vec<String>>,
    /// Version of the API proxy configuration schema to which the API proxy conforms. Currently, the only supported value is 4.0 (`majorVersion.minorVersion`). This setting may be used in the future to track the evolution of the API proxy format.
    #[serde(rename="configurationVersion")]
    
    pub configuration_version: Option<GoogleCloudApigeeV1ConfigVersion>,
    /// Revision number, app name, and organization for the API proxy.
    #[serde(rename="contextInfo")]
    
    pub context_info: Option<String>,
    /// Time that the API proxy revision was created in milliseconds since epoch.
    #[serde(rename="createdAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub created_at: Option<i64>,
    /// Description of the API proxy revision.
    
    pub description: Option<String>,
    /// Human-readable name of the API proxy.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Metadata describing the API proxy revision as a key-value map.
    #[serde(rename="entityMetaDataAsProperties")]
    
    pub entity_meta_data_as_properties: Option<HashMap<String, String>>,
    /// List of IntegrationEndpoints in the '/integration-endpoints' directory of the API proxy. This is a 'manifest' setting designed to provide visibility into the contents of the API proxy.
    #[serde(rename="integrationEndpoints")]
    
    pub integration_endpoints: Option<Vec<String>>,
    /// Time that the API proxy revision was last modified in milliseconds since epoch.
    #[serde(rename="lastModifiedAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_modified_at: Option<i64>,
    /// Name of the API proxy.
    
    pub name: Option<String>,
    /// List of policy names included in the API proxy revision..
    
    pub policies: Option<Vec<String>>,
    /// List of proxy names included in the API proxy revision.
    
    pub proxies: Option<Vec<String>>,
    /// List of ProxyEndpoints in the `/proxies` directory of the API proxy. Typically, this element is included only when the API proxy was created using the Edge UI. This is a 'manifest' setting designed to provide visibility into the contents of the API proxy.
    #[serde(rename="proxyEndpoints")]
    
    pub proxy_endpoints: Option<Vec<String>>,
    /// List of resource files included in the API proxy revision.
    #[serde(rename="resourceFiles")]
    
    pub resource_files: Option<GoogleCloudApigeeV1ResourceFiles>,
    /// List of the resources included in the API proxy revision formatted as "{type}://{name}".
    
    pub resources: Option<Vec<String>>,
    /// API proxy revision.
    
    pub revision: Option<String>,
    /// List of the shared flows included in the API proxy revision.
    #[serde(rename="sharedFlows")]
    
    pub shared_flows: Option<Vec<String>>,
    /// OpenAPI Specification that is associated with the API proxy. The value is set to a URL or to a path in the specification store.
    
    pub spec: Option<String>,
    /// List of TargetEndpoints in the `/targets` directory of the API proxy. Typically, this element is included only when the API proxy was created using the Edge UI. This is a 'manifest' setting designed to provide visibility into the contents of the API proxy.
    #[serde(rename="targetEndpoints")]
    
    pub target_endpoints: Option<Vec<String>>,
    /// List of TargetServers referenced in any TargetEndpoint in the API proxy. Typically, you will see this element only when the API proxy was created using the Edge UI. This is a 'manifest' setting designed to provide visibility into the contents of the API proxy.
    #[serde(rename="targetServers")]
    
    pub target_servers: Option<Vec<String>>,
    /// List of the targets included in the API proxy revision.
    
    pub targets: Option<Vec<String>>,
    /// List of the teams included in the API proxy revision.
    
    pub teams: Option<Vec<String>>,
    /// Type. Set to `Application`. Maintained for compatibility with the Apigee Edge API.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ApiProxyRevision {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sites apicategories delete organizations](OrganizationSiteApicategoryDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ApiResponseWrapper {
    /// ID that can be used to find errors in the log files.
    #[serde(rename="errorCode")]
    
    pub error_code: Option<String>,
    /// Description of the operation.
    
    pub message: Option<String>,
    /// ID that can be used to find request details in the log files.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
    /// Status of the operation.
    
    pub status: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ApiResponseWrapper {}


/// Configurations of the API Security add-on.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ApiSecurityConfig {
    /// Flag that specifies whether the API security add-on is enabled.
    
    pub enabled: Option<bool>,
    /// Output only. Time at which the API Security add-on expires in in milliseconds since epoch. If unspecified, the add-on will never expire.
    #[serde(rename="expiresAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expires_at: Option<i64>,
}

impl client::Part for GoogleCloudApigeeV1ApiSecurityConfig {}


/// Response for GetApiSecurityRuntimeConfig\[EnvironmentService.GetApiSecurityRuntimeConfig\].
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments get api security runtime config organizations](OrganizationEnvironmentGetApiSecurityRuntimeConfigCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ApiSecurityRuntimeConfig {
    /// A list of up to 5 Cloud Storage Blobs that contain SecurityActions.
    
    pub location: Option<Vec<String>>,
    /// Name of the environment API Security Runtime configuration resource. Format: `organizations/{org}/environments/{env}/apiSecurityRuntimeConfig`
    
    pub name: Option<String>,
    /// Revision ID of the API Security Runtime configuration. The higher the value, the more recently the configuration was deployed.
    #[serde(rename="revisionId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub revision_id: Option<i64>,
    /// Unique ID for the API Security Runtime configuration. The ID will only change if the environment is deleted and recreated.
    
    pub uid: Option<String>,
    /// Time that the API Security Runtime configuration was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ApiSecurityRuntimeConfig {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps get organizations](OrganizationAppGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1App {
    /// List of API products associated with the app.
    #[serde(rename="apiProducts")]
    
    pub api_products: Option<Vec<GoogleCloudApigeeV1ApiProductRef>>,
    /// ID of the app.
    #[serde(rename="appId")]
    
    pub app_id: Option<String>,
    /// List of attributes.
    
    pub attributes: Option<Vec<GoogleCloudApigeeV1Attribute>>,
    /// Callback URL used by OAuth 2.0 authorization servers to communicate authorization codes back to apps.
    #[serde(rename="callbackUrl")]
    
    pub callback_url: Option<String>,
    /// Name of the company that owns the app.
    #[serde(rename="companyName")]
    
    pub company_name: Option<String>,
    /// Output only. Unix time when the app was created.
    #[serde(rename="createdAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub created_at: Option<i64>,
    /// Output only. Set of credentials for the app. Credentials are API key/secret pairs associated with API products.
    
    pub credentials: Option<Vec<GoogleCloudApigeeV1Credential>>,
    /// ID of the developer.
    #[serde(rename="developerId")]
    
    pub developer_id: Option<String>,
    /// Duration, in milliseconds, of the consumer key that will be generated for the app. The default value, -1, indicates an infinite validity period. Once set, the expiration can't be updated. json key: keyExpiresIn
    #[serde(rename="keyExpiresIn")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub key_expires_in: Option<i64>,
    /// Output only. Last modified time as milliseconds since epoch.
    #[serde(rename="lastModifiedAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_modified_at: Option<i64>,
    /// Name of the app.
    
    pub name: Option<String>,
    /// Scopes to apply to the app. The specified scope names must already exist on the API product that you associate with the app.
    
    pub scopes: Option<Vec<String>>,
    /// Status of the credential.
    
    pub status: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1App {}


/// Archive Deployment information.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments archive deployments create organizations](OrganizationEnvironmentArchiveDeploymentCreateCall) (request)
/// * [environments archive deployments get organizations](OrganizationEnvironmentArchiveDeploymentGetCall) (response)
/// * [environments archive deployments patch organizations](OrganizationEnvironmentArchiveDeploymentPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ArchiveDeployment {
    /// Output only. The time at which the Archive Deployment was created in milliseconds since the epoch.
    #[serde(rename="createdAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub created_at: Option<i64>,
    /// Input only. The Google Cloud Storage signed URL returned from GenerateUploadUrl and used to upload the Archive zip file.
    #[serde(rename="gcsUri")]
    
    pub gcs_uri: Option<String>,
    /// User-supplied key-value pairs used to organize ArchiveDeployments. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62} Label values must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63} No more than 64 labels can be associated with a given store.
    
    pub labels: Option<HashMap<String, String>>,
    /// Name of the Archive Deployment in the following format: `organizations/{org}/environments/{env}/archiveDeployments/{id}`.
    
    pub name: Option<String>,
    /// Output only. A reference to the LRO that created this Archive Deployment in the following format: `organizations/{org}/operations/{id}`
    
    pub operation: Option<String>,
    /// Output only. The time at which the Archive Deployment was updated in milliseconds since the epoch.
    #[serde(rename="updatedAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub updated_at: Option<i64>,
}

impl client::RequestValue for GoogleCloudApigeeV1ArchiveDeployment {}
impl client::ResponseResult for GoogleCloudApigeeV1ArchiveDeployment {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments queries create organizations](OrganizationEnvironmentQueryCreateCall) (response)
/// * [environments queries get organizations](OrganizationEnvironmentQueryGetCall) (response)
/// * [host queries create organizations](OrganizationHostQueryCreateCall) (response)
/// * [host queries get organizations](OrganizationHostQueryGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1AsyncQuery {
    /// Creation time of the query.
    
    pub created: Option<String>,
    /// Hostname is available only when query is executed at host level.
    #[serde(rename="envgroupHostname")]
    
    pub envgroup_hostname: Option<String>,
    /// Error is set when query fails.
    
    pub error: Option<String>,
    /// ExecutionTime is available only after the query is completed.
    #[serde(rename="executionTime")]
    
    pub execution_time: Option<String>,
    /// Asynchronous Query Name.
    
    pub name: Option<String>,
    /// Contains information like metrics, dimenstions etc of the AsyncQuery.
    #[serde(rename="queryParams")]
    
    pub query_params: Option<GoogleCloudApigeeV1QueryMetadata>,
    /// Asynchronous Report ID.
    #[serde(rename="reportDefinitionId")]
    
    pub report_definition_id: Option<String>,
    /// Result is available only after the query is completed.
    
    pub result: Option<GoogleCloudApigeeV1AsyncQueryResult>,
    /// ResultFileSize is available only after the query is completed.
    #[serde(rename="resultFileSize")]
    
    pub result_file_size: Option<String>,
    /// ResultRows is available only after the query is completed.
    #[serde(rename="resultRows")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub result_rows: Option<i64>,
    /// Self link of the query. Example: `/organizations/myorg/environments/myenv/queries/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd` or following format if query is running at host level: `/organizations/myorg/hostQueries/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd`
    #[serde(rename="self")]
    
    pub self_: Option<String>,
    /// Query state could be "enqueued", "running", "completed", "failed".
    
    pub state: Option<String>,
    /// Last updated timestamp for the query.
    
    pub updated: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1AsyncQuery {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1AsyncQueryResult {
    /// Query result will be unaccessable after this time.
    
    pub expires: Option<String>,
    /// Self link of the query results. Example: `/organizations/myorg/environments/myenv/queries/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd/result` or following format if query is running at host level: `/organizations/myorg/hostQueries/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd/result`
    #[serde(rename="self")]
    
    pub self_: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1AsyncQueryResult {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [host queries get result view organizations](OrganizationHostQueryGetResultViewCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1AsyncQueryResultView {
    /// Error code when there is a failure.
    
    pub code: Option<i32>,
    /// Error message when there is a failure.
    
    pub error: Option<String>,
    /// Metadata contains information like metrics, dimenstions etc of the AsyncQuery.
    
    pub metadata: Option<GoogleCloudApigeeV1QueryMetadata>,
    /// Rows of query result. Each row is a JSON object. Example: {sum(message_count): 1, developer_app: "(not set)",…}
    
    pub rows: Option<Vec<json::Value>>,
    /// State of retrieving ResultView.
    
    pub state: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1AsyncQueryResultView {}


/// Key-value pair to store extra metadata.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apiproducts attributes delete organizations](OrganizationApiproductAttributeDeleteCall) (response)
/// * [apiproducts attributes get organizations](OrganizationApiproductAttributeGetCall) (response)
/// * [apiproducts attributes update api product attribute organizations](OrganizationApiproductAttributeUpdateApiProductAttributeCall) (request|response)
/// * [developers apps attributes delete organizations](OrganizationDeveloperAppAttributeDeleteCall) (response)
/// * [developers apps attributes get organizations](OrganizationDeveloperAppAttributeGetCall) (response)
/// * [developers apps attributes update developer app attribute organizations](OrganizationDeveloperAppAttributeUpdateDeveloperAppAttributeCall) (request|response)
/// * [developers attributes delete organizations](OrganizationDeveloperAttributeDeleteCall) (response)
/// * [developers attributes get organizations](OrganizationDeveloperAttributeGetCall) (response)
/// * [developers attributes update developer attribute organizations](OrganizationDeveloperAttributeUpdateDeveloperAttributeCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Attribute {
    /// API key of the attribute.
    
    pub name: Option<String>,
    /// Value of the attribute.
    
    pub value: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1Attribute {}
impl client::ResponseResult for GoogleCloudApigeeV1Attribute {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apiproducts attributes list organizations](OrganizationApiproductAttributeListCall) (response)
/// * [apiproducts attributes organizations](OrganizationApiproductAttributeCall) (request|response)
/// * [developers apps attributes list organizations](OrganizationDeveloperAppAttributeListCall) (response)
/// * [developers apps attributes organizations](OrganizationDeveloperAppAttributeCall) (request|response)
/// * [developers attributes list organizations](OrganizationDeveloperAttributeListCall) (response)
/// * [developers attributes organizations](OrganizationDeveloperAttributeCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Attributes {
    /// List of attributes.
    
    pub attribute: Option<Vec<GoogleCloudApigeeV1Attribute>>,
}

impl client::RequestValue for GoogleCloudApigeeV1Attributes {}
impl client::ResponseResult for GoogleCloudApigeeV1Attributes {}


/// CanaryEvaluation represents the canary analysis between two versions of the runtime that is serving requests.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances canaryevaluations create organizations](OrganizationInstanceCanaryevaluationCreateCall) (request)
/// * [instances canaryevaluations get organizations](OrganizationInstanceCanaryevaluationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1CanaryEvaluation {
    /// Required. The stable version that is serving requests.
    
    pub control: Option<String>,
    /// Output only. Create time of the canary evaluation.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. End time for the evaluation's analysis.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. Labels used to filter the metrics used for a canary evaluation.
    #[serde(rename="metricLabels")]
    
    pub metric_labels: Option<GoogleCloudApigeeV1CanaryEvaluationMetricLabels>,
    /// Output only. Name of the canary evalution.
    
    pub name: Option<String>,
    /// Required. Start time for the canary evaluation's analysis.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The current state of the canary evaluation.
    
    pub state: Option<String>,
    /// Required. The newer version that is serving requests.
    
    pub treatment: Option<String>,
    /// Output only. The resulting verdict of the canary evaluations: NONE, PASS, or FAIL.
    
    pub verdict: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1CanaryEvaluation {}
impl client::ResponseResult for GoogleCloudApigeeV1CanaryEvaluation {}


/// Labels that can be used to filter Apigee metrics.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1CanaryEvaluationMetricLabels {
    /// The environment ID associated with the metrics.
    
    pub env: Option<String>,
    /// Required. The instance ID associated with the metrics. In Apigee Hybrid, the value is configured during installation.
    
    pub instance_id: Option<String>,
    /// Required. The location associated with the metrics.
    
    pub location: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1CanaryEvaluationMetricLabels {}


/// X.509 certificate as defined in RFC 5280.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1CertInfo {
    /// X.509 basic constraints extension.
    #[serde(rename="basicConstraints")]
    
    pub basic_constraints: Option<String>,
    /// X.509 `notAfter` validity period in milliseconds since epoch.
    #[serde(rename="expiryDate")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expiry_date: Option<i64>,
    /// Flag that specifies whether the certificate is valid. Flag is set to `Yes` if the certificate is valid, `No` if expired, or `Not yet` if not yet valid.
    #[serde(rename="isValid")]
    
    pub is_valid: Option<String>,
    /// X.509 issuer.
    
    pub issuer: Option<String>,
    /// Public key component of the X.509 subject public key info.
    #[serde(rename="publicKey")]
    
    pub public_key: Option<String>,
    /// X.509 serial number.
    #[serde(rename="serialNumber")]
    
    pub serial_number: Option<String>,
    /// X.509 signatureAlgorithm.
    #[serde(rename="sigAlgName")]
    
    pub sig_alg_name: Option<String>,
    /// X.509 subject.
    
    pub subject: Option<String>,
    /// X.509 subject alternative names (SANs) extension.
    #[serde(rename="subjectAlternativeNames")]
    
    pub subject_alternative_names: Option<Vec<String>>,
    /// X.509 `notBefore` validity period in milliseconds since epoch.
    #[serde(rename="validFrom")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub valid_from: Option<i64>,
    /// X.509 version.
    
    pub version: Option<i32>,
}

impl client::Part for GoogleCloudApigeeV1CertInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Certificate {
    /// Chain of certificates under this name.
    #[serde(rename="certInfo")]
    
    pub cert_info: Option<Vec<GoogleCloudApigeeV1CertInfo>>,
}

impl client::Part for GoogleCloudApigeeV1Certificate {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1CommonNameConfig {
    /// no description provided
    #[serde(rename="matchWildCards")]
    
    pub match_wild_cards: Option<bool>,
    /// no description provided
    
    pub name: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1CommonNameConfig {}


/// Request for ComputeEnvironmentScores.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [security profiles environments compute environment scores organizations](OrganizationSecurityProfileEnvironmentComputeEnvironmentScoreCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ComputeEnvironmentScoresRequest {
    /// Optional. Filters are used to filter scored components. Return all the components if no filter is mentioned. Example: [{ "scorePath": "/org@myorg/envgroup@myenvgroup/env@myenv/proxies/proxy@myproxy/source" }, { "scorePath": "/org@myorg/envgroup@myenvgroup/env@myenv/proxies/proxy@myproxy/target", }] This will return components with path: "/org@myorg/envgroup@myenvgroup/env@myenv/proxies/proxy@myproxy/source" OR "/org@myorg/envgroup@myenvgroup/env@myenv/proxies/proxy@myproxy/target"
    
    pub filters: Option<Vec<GoogleCloudApigeeV1ComputeEnvironmentScoresRequestFilter>>,
    /// Optional. The maximum number of subcomponents to be returned in a single page. The service may return fewer than this value. If unspecified, at most 100 subcomponents will be returned in a single page.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// Optional. A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
    /// Required. Time range for score calculation. At most 14 days of scores will be returned.
    #[serde(rename="timeRange")]
    
    pub time_range: Option<GoogleTypeInterval>,
}

impl client::RequestValue for GoogleCloudApigeeV1ComputeEnvironmentScoresRequest {}


/// Filter scores by component path. Used custom filter instead of AIP-160 as the use cases are highly constrained and predictable.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ComputeEnvironmentScoresRequestFilter {
    /// Optional. Return scores for this component. Example: "/org@myorg/envgroup@myenvgroup/env@myenv/proxies/proxy@myproxy/source"
    #[serde(rename="scorePath")]
    
    pub score_path: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1ComputeEnvironmentScoresRequestFilter {}


/// Response for ComputeEnvironmentScores.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [security profiles environments compute environment scores organizations](OrganizationSecurityProfileEnvironmentComputeEnvironmentScoreCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ComputeEnvironmentScoresResponse {
    /// A page token, received from a previous `ComputeScore` call. Provide this to retrieve the subsequent page.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of scores. One score per day.
    
    pub scores: Option<Vec<GoogleCloudApigeeV1Score>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ComputeEnvironmentScoresResponse {}


/// Version of the API proxy configuration schema. Currently, only 4.0 is supported.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ConfigVersion {
    /// Major version of the API proxy configuration schema.
    #[serde(rename="majorVersion")]
    
    pub major_version: Option<i32>,
    /// Minor version of the API proxy configuration schema.
    #[serde(rename="minorVersion")]
    
    pub minor_version: Option<i32>,
}

impl client::Part for GoogleCloudApigeeV1ConfigVersion {}


/// Configuration for the Connectors Platform add-on.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ConnectorsPlatformConfig {
    /// Flag that specifies whether the Connectors Platform add-on is enabled.
    
    pub enabled: Option<bool>,
    /// Output only. Time at which the Connectors Platform add-on expires in milliseconds since epoch. If unspecified, the add-on will never expire.
    #[serde(rename="expiresAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expires_at: Option<i64>,
}

impl client::Part for GoogleCloudApigeeV1ConnectorsPlatformConfig {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Credential {
    /// List of API products this credential can be used for.
    #[serde(rename="apiProducts")]
    
    pub api_products: Option<Vec<GoogleCloudApigeeV1ApiProductRef>>,
    /// List of attributes associated with this credential.
    
    pub attributes: Option<Vec<GoogleCloudApigeeV1Attribute>>,
    /// Consumer key.
    #[serde(rename="consumerKey")]
    
    pub consumer_key: Option<String>,
    /// Secret key.
    #[serde(rename="consumerSecret")]
    
    pub consumer_secret: Option<String>,
    /// Time the credential will expire in milliseconds since epoch.
    #[serde(rename="expiresAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expires_at: Option<i64>,
    /// Time the credential was issued in milliseconds since epoch.
    #[serde(rename="issuedAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub issued_at: Option<i64>,
    /// List of scopes to apply to the app. Specified scopes must already exist on the API product that you associate with the app.
    
    pub scopes: Option<Vec<String>>,
    /// Status of the credential. Valid values include `approved` or `revoked`.
    
    pub status: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1Credential {}


/// Request for CreditDeveloperBalance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [developers balance credit organizations](OrganizationDeveloperBalanceCreditCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1CreditDeveloperBalanceRequest {
    /// The amount of money to be credited. The wallet corresponding to the currency specified within `transaction_amount` will be updated. For example, if you specified `currency_code` within `transaction_amount` as "USD", then the amount would be added to the wallet which has the "USD" currency or if no such wallet exists, a new wallet will be created with the "USD" currency.
    #[serde(rename="transactionAmount")]
    
    pub transaction_amount: Option<GoogleTypeMoney>,
    /// Each transaction_id uniquely identifies a credit balance request. If multiple requests are received with the same transaction_id, only one of them will be considered.
    #[serde(rename="transactionId")]
    
    pub transaction_id: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1CreditDeveloperBalanceRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports create organizations](OrganizationReportCreateCall) (request|response)
/// * [reports get organizations](OrganizationReportGetCall) (response)
/// * [reports update organizations](OrganizationReportUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1CustomReport {
    /// This field contains the chart type for the report
    #[serde(rename="chartType")]
    
    pub chart_type: Option<String>,
    /// Legacy field: not used. This field contains a list of comments associated with custom report
    
    pub comments: Option<Vec<String>>,
    /// Output only. Unix time when the app was created json key: createdAt
    #[serde(rename="createdAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub created_at: Option<i64>,
    /// This contains the list of dimensions for the report
    
    pub dimensions: Option<Vec<String>>,
    /// This is the display name for the report
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Environment name
    
    pub environment: Option<String>,
    /// This field contains the filter expression
    
    pub filter: Option<String>,
    /// Legacy field: not used. Contains the from time for the report
    #[serde(rename="fromTime")]
    
    pub from_time: Option<String>,
    /// Output only. Modified time of this entity as milliseconds since epoch. json key: lastModifiedAt
    #[serde(rename="lastModifiedAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_modified_at: Option<i64>,
    /// Output only. Last viewed time of this entity as milliseconds since epoch
    #[serde(rename="lastViewedAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_viewed_at: Option<i64>,
    /// Legacy field: not used This field contains the limit for the result retrieved
    
    pub limit: Option<String>,
    /// Required. This contains the list of metrics
    
    pub metrics: Option<Vec<GoogleCloudApigeeV1CustomReportMetric>>,
    /// Required. Unique identifier for the report T his is a legacy field used to encode custom report unique id
    
    pub name: Option<String>,
    /// Legacy field: not used. This field contains the offset for the data
    
    pub offset: Option<String>,
    /// Output only. Organization name
    
    pub organization: Option<String>,
    /// This field contains report properties such as ui metadata etc.
    
    pub properties: Option<Vec<GoogleCloudApigeeV1ReportProperty>>,
    /// Legacy field: not used much. Contains the list of sort by columns
    #[serde(rename="sortByCols")]
    
    pub sort_by_cols: Option<Vec<String>>,
    /// Legacy field: not used much. Contains the sort order for the sort columns
    #[serde(rename="sortOrder")]
    
    pub sort_order: Option<String>,
    /// Legacy field: not used. This field contains a list of tags associated with custom report
    
    pub tags: Option<Vec<String>>,
    /// This field contains the time unit of aggregation for the report
    #[serde(rename="timeUnit")]
    
    pub time_unit: Option<String>,
    /// Legacy field: not used. Contains the end time for the report
    #[serde(rename="toTime")]
    
    pub to_time: Option<String>,
    /// Legacy field: not used. This field contains the top k parameter value for restricting the result
    
    pub topk: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1CustomReport {}
impl client::ResponseResult for GoogleCloudApigeeV1CustomReport {}


/// This encapsulates a metric property of the form sum(message_count) where name is message_count and function is sum
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1CustomReportMetric {
    /// aggregate function
    
    pub function: Option<String>,
    /// name of the metric
    
    pub name: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1CustomReportMetric {}


/// Data collector configuration.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [datacollectors create organizations](OrganizationDatacollectorCreateCall) (request|response)
/// * [datacollectors get organizations](OrganizationDatacollectorGetCall) (response)
/// * [datacollectors patch organizations](OrganizationDatacollectorPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1DataCollector {
    /// Output only. The time at which the data collector was created in milliseconds since the epoch.
    #[serde(rename="createdAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub created_at: Option<i64>,
    /// A description of the data collector.
    
    pub description: Option<String>,
    /// Output only. The time at which the Data Collector was last updated in milliseconds since the epoch.
    #[serde(rename="lastModifiedAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_modified_at: Option<i64>,
    /// ID of the data collector. Must begin with `dc_`.
    
    pub name: Option<String>,
    /// Immutable. The type of data this data collector will collect.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1DataCollector {}
impl client::ResponseResult for GoogleCloudApigeeV1DataCollector {}


/// Data collector and its configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1DataCollectorConfig {
    /// Name of the data collector in the following format: `organizations/{org}/datacollectors/{datacollector}`
    
    pub name: Option<String>,
    /// Data type accepted by the data collector.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1DataCollectorConfig {}


/// The data store defines the connection to export data repository (Cloud Storage, BigQuery), including the credentials used to access the data repository.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [analytics datastores create organizations](OrganizationAnalyticDatastoreCreateCall) (request|response)
/// * [analytics datastores get organizations](OrganizationAnalyticDatastoreGetCall) (response)
/// * [analytics datastores test organizations](OrganizationAnalyticDatastoreTestCall) (request)
/// * [analytics datastores update organizations](OrganizationAnalyticDatastoreUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Datastore {
    /// Output only. Datastore create time, in milliseconds since the epoch of 1970-01-01T00:00:00Z
    #[serde(rename="createTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub create_time: Option<i64>,
    /// Datastore Configurations.
    #[serde(rename="datastoreConfig")]
    
    pub datastore_config: Option<GoogleCloudApigeeV1DatastoreConfig>,
    /// Required. Display name in UI
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Datastore last update time, in milliseconds since the epoch of 1970-01-01T00:00:00Z
    #[serde(rename="lastUpdateTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_update_time: Option<i64>,
    /// Output only. Organization that the datastore belongs to
    
    pub org: Option<String>,
    /// Output only. Resource link of Datastore. Example: `/organizations/{org}/analytics/datastores/{uuid}`
    #[serde(rename="self")]
    
    pub self_: Option<String>,
    /// Destination storage type. Supported types `gcs` or `bigquery`.
    #[serde(rename="targetType")]
    
    pub target_type: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1Datastore {}
impl client::ResponseResult for GoogleCloudApigeeV1Datastore {}


/// Configuration detail for datastore
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1DatastoreConfig {
    /// Name of the Cloud Storage bucket. Required for `gcs` target_type.
    #[serde(rename="bucketName")]
    
    pub bucket_name: Option<String>,
    /// BigQuery dataset name Required for `bigquery` target_type.
    #[serde(rename="datasetName")]
    
    pub dataset_name: Option<String>,
    /// Path of Cloud Storage bucket Required for `gcs` target_type.
    
    pub path: Option<String>,
    /// Required. GCP project in which the datastore exists
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Prefix of BigQuery table Required for `bigquery` target_type.
    #[serde(rename="tablePrefix")]
    
    pub table_prefix: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1DatastoreConfig {}


/// Date range of the data to export.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1DateRange {
    /// Required. End date (exclusive) of the data to export in the format `yyyy-mm-dd`. The date range ends at 00:00:00 UTC on the end date- which will not be in the output.
    
    pub end: Option<String>,
    /// Required. Start date of the data to export in the format `yyyy-mm-dd`. The date range begins at 00:00:00 UTC on the start date.
    
    pub start: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1DateRange {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments get debugmask organizations](OrganizationEnvironmentGetDebugmaskCall) (response)
/// * [environments update debugmask organizations](OrganizationEnvironmentUpdateDebugmaskCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1DebugMask {
    /// List of JSON paths that specify the JSON elements to be filtered from JSON payloads in error flows.
    #[serde(rename="faultJSONPaths")]
    
    pub fault_json_paths: Option<Vec<String>>,
    /// List of XPaths that specify the XML elements to be filtered from XML payloads in error flows.
    #[serde(rename="faultXPaths")]
    
    pub fault_x_paths: Option<Vec<String>>,
    /// Name of the debug mask.
    
    pub name: Option<String>,
    /// Map of namespaces to URIs.
    
    pub namespaces: Option<HashMap<String, String>>,
    /// List of JSON paths that specify the JSON elements to be filtered from JSON request message payloads.
    #[serde(rename="requestJSONPaths")]
    
    pub request_json_paths: Option<Vec<String>>,
    /// List of XPaths that specify the XML elements to be filtered from XML request message payloads.
    #[serde(rename="requestXPaths")]
    
    pub request_x_paths: Option<Vec<String>>,
    /// List of JSON paths that specify the JSON elements to be filtered from JSON response message payloads.
    #[serde(rename="responseJSONPaths")]
    
    pub response_json_paths: Option<Vec<String>>,
    /// List of XPaths that specify the XML elements to be filtered from XML response message payloads.
    #[serde(rename="responseXPaths")]
    
    pub response_x_paths: Option<Vec<String>>,
    /// List of variables that should be masked from the debug output.
    
    pub variables: Option<Vec<String>>,
}

impl client::RequestValue for GoogleCloudApigeeV1DebugMask {}
impl client::ResponseResult for GoogleCloudApigeeV1DebugMask {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments apis revisions debugsessions create organizations](OrganizationEnvironmentApiRevisionDebugsessionCreateCall) (request|response)
/// * [environments apis revisions debugsessions get organizations](OrganizationEnvironmentApiRevisionDebugsessionGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1DebugSession {
    /// Optional. The number of request to be traced. Min = 1, Max = 15, Default = 10.
    
    pub count: Option<i32>,
    /// Output only. The first transaction creation timestamp, recorded by UAP.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. A conditional statement which is evaluated against the request message to determine if it should be traced. Syntax matches that of on API Proxy bundle flow Condition.
    
    pub filter: Option<String>,
    /// A unique ID for this DebugSession.
    
    pub name: Option<String>,
    /// Optional. The time in seconds after which this DebugSession should end. This value will override the value in query param, if both are provided.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub timeout: Option<i64>,
    /// Optional. The maximum number of bytes captured from the response payload. Min = 0, Max = 5120, Default = 5120.
    
    pub tracesize: Option<i32>,
    /// Optional. The length of time, in seconds, that this debug session is valid, starting from when it's received in the control plane. Min = 1, Max = 15, Default = 10.
    
    pub validity: Option<i32>,
}

impl client::RequestValue for GoogleCloudApigeeV1DebugSession {}
impl client::ResponseResult for GoogleCloudApigeeV1DebugSession {}


/// A transaction contains all of the debug information of the entire message flow of an API call processed by the runtime plane. The information is collected and recorded at critical points of the message flow in the runtime apiproxy.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments apis revisions debugsessions data get organizations](OrganizationEnvironmentApiRevisionDebugsessionDataGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1DebugSessionTransaction {
    /// Flag indicating whether a transaction is completed or not
    
    pub completed: Option<bool>,
    /// List of debug data collected by runtime plane at various defined points in the flow.
    
    pub point: Option<Vec<GoogleCloudApigeeV1Point>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1DebugSessionTransaction {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports delete organizations](OrganizationReportDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1DeleteCustomReportResponse {
    /// The response contains only a message field.
    
    pub message: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1DeleteCustomReportResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments apis revisions deploy organizations](OrganizationEnvironmentApiRevisionDeployCall) (response)
/// * [environments apis revisions get deployments organizations](OrganizationEnvironmentApiRevisionGetDeploymentCall) (response)
/// * [environments sharedflows revisions deploy organizations](OrganizationEnvironmentSharedflowRevisionDeployCall) (response)
/// * [environments sharedflows revisions get deployments organizations](OrganizationEnvironmentSharedflowRevisionGetDeploymentCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Deployment {
    /// API proxy.
    #[serde(rename="apiProxy")]
    
    pub api_proxy: Option<String>,
    /// Time the API proxy was marked `deployed` in the control plane in millisconds since epoch.
    #[serde(rename="deployStartTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub deploy_start_time: Option<i64>,
    /// Environment.
    
    pub environment: Option<String>,
    /// Errors reported for this deployment. Populated only when state == ERROR. **Note**: This field is displayed only when viewing deployment status.
    
    pub errors: Option<Vec<GoogleRpcStatus>>,
    /// Status reported by each runtime instance. **Note**: This field is displayed only when viewing deployment status.
    
    pub instances: Option<Vec<GoogleCloudApigeeV1InstanceDeploymentStatus>>,
    /// Status reported by runtime pods. **Note**: **This field is deprecated**. Runtime versions 1.3 and above report instance level status rather than pod status.
    
    pub pods: Option<Vec<GoogleCloudApigeeV1PodStatus>>,
    /// API proxy revision.
    
    pub revision: Option<String>,
    /// Conflicts in the desired state routing configuration. The presence of conflicts does not cause the state to be `ERROR`, but it will mean that some of the deployment's base paths are not routed to its environment. If the conflicts change, the state will transition to `PROGRESSING` until the latest configuration is rolled out to all instances. **Note**: This field is displayed only when viewing deployment status.
    #[serde(rename="routeConflicts")]
    
    pub route_conflicts: Option<Vec<GoogleCloudApigeeV1DeploymentChangeReportRoutingConflict>>,
    /// The full resource name of Cloud IAM Service Account that this deployment is using, eg, `projects/-/serviceAccounts/{email}`.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
    /// Current state of the deployment. **Note**: This field is displayed only when viewing deployment status.
    
    pub state: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1Deployment {}


/// Response for GenerateDeployChangeReport and GenerateUndeployChangeReport. This report contains any validation failures that would cause the deployment to be rejected, as well changes and conflicts in routing that may occur due to the new deployment. The existence of a routing warning does not necessarily imply that the deployment request is bad, if the desired state of the deployment request is to effect a routing change. The primary purposes of the routing messages are: 1) To inform users of routing changes that may have an effect on traffic currently being routed to other existing deployments. 2) To warn users if some base path in the proxy will not receive traffic due to an existing deployment having already claimed that base path. The presence of routing conflicts/changes will not cause non-dry-run DeployApiProxy/UndeployApiProxy requests to be rejected.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments apis revisions deployments generate deploy change report organizations](OrganizationEnvironmentApiRevisionDeploymentGenerateDeployChangeReportCall) (response)
/// * [environments apis revisions deployments generate undeploy change report organizations](OrganizationEnvironmentApiRevisionDeploymentGenerateUndeployChangeReportCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1DeploymentChangeReport {
    /// All routing changes that may result from a deployment request.
    #[serde(rename="routingChanges")]
    
    pub routing_changes: Option<Vec<GoogleCloudApigeeV1DeploymentChangeReportRoutingChange>>,
    /// All base path conflicts detected for a deployment request.
    #[serde(rename="routingConflicts")]
    
    pub routing_conflicts: Option<Vec<GoogleCloudApigeeV1DeploymentChangeReportRoutingConflict>>,
    /// Validation errors that would cause the deployment change request to be rejected.
    #[serde(rename="validationErrors")]
    
    pub validation_errors: Option<GoogleRpcPreconditionFailure>,
}

impl client::ResponseResult for GoogleCloudApigeeV1DeploymentChangeReport {}


/// Describes a potential routing change that may occur as a result of some deployment operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1DeploymentChangeReportRoutingChange {
    /// Human-readable description of this routing change.
    
    pub description: Option<String>,
    /// Name of the environment group affected by this routing change.
    #[serde(rename="environmentGroup")]
    
    pub environment_group: Option<String>,
    /// Base path/deployment that may stop receiving some traffic.
    #[serde(rename="fromDeployment")]
    
    pub from_deployment: Option<GoogleCloudApigeeV1DeploymentChangeReportRoutingDeployment>,
    /// Set to `true` if using sequenced rollout would make this routing change safer. **Note**: This does not necessarily imply that automated sequenced rollout mode is supported for the operation.
    #[serde(rename="shouldSequenceRollout")]
    
    pub should_sequence_rollout: Option<bool>,
    /// Base path/deployment that may start receiving that traffic. May be null if no deployment is able to receive the traffic.
    #[serde(rename="toDeployment")]
    
    pub to_deployment: Option<GoogleCloudApigeeV1DeploymentChangeReportRoutingDeployment>,
}

impl client::Part for GoogleCloudApigeeV1DeploymentChangeReportRoutingChange {}


/// Describes a routing conflict that may cause a deployment not to receive traffic at some base path.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1DeploymentChangeReportRoutingConflict {
    /// Existing base path/deployment causing the conflict.
    #[serde(rename="conflictingDeployment")]
    
    pub conflicting_deployment: Option<GoogleCloudApigeeV1DeploymentChangeReportRoutingDeployment>,
    /// Human-readable description of this conflict.
    
    pub description: Option<String>,
    /// Name of the environment group in which this conflict exists.
    #[serde(rename="environmentGroup")]
    
    pub environment_group: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1DeploymentChangeReportRoutingConflict {}


/// Tuple representing a base path and the deployment containing it.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1DeploymentChangeReportRoutingDeployment {
    /// Name of the deployed API proxy revision containing the base path.
    #[serde(rename="apiProxy")]
    
    pub api_proxy: Option<String>,
    /// Base path receiving traffic.
    
    pub basepath: Option<String>,
    /// Name of the environment in which the proxy is deployed.
    
    pub environment: Option<String>,
    /// Name of the deployed API proxy revision containing the base path.
    
    pub revision: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1DeploymentChangeReportRoutingDeployment {}


/// NEXT ID: 11
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1DeploymentConfig {
    /// Additional key-value metadata for the deployment.
    
    pub attributes: Option<HashMap<String, String>>,
    /// Base path where the application will be hosted. Defaults to "/".
    #[serde(rename="basePath")]
    
    pub base_path: Option<String>,
    /// The list of deployment groups in which this proxy should be deployed. Not currently populated for shared flows.
    #[serde(rename="deploymentGroups")]
    
    pub deployment_groups: Option<Vec<String>>,
    /// A mapping from basepaths to proxy endpoint names in this proxy. Not populated for shared flows.
    
    pub endpoints: Option<HashMap<String, String>>,
    /// Location of the API proxy bundle as a URI.
    
    pub location: Option<String>,
    /// Name of the API or shared flow revision to be deployed in the following format: `organizations/{org}/apis/{api}/revisions/{rev}` or `organizations/{org}/sharedflows/{sharedflow}/revisions/{rev}`
    
    pub name: Option<String>,
    /// Unique ID of the API proxy revision.
    #[serde(rename="proxyUid")]
    
    pub proxy_uid: Option<String>,
    /// The service account identity associated with this deployment. If non-empty, will be in the following format: `projects/-/serviceAccounts/{account_email}`
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
    /// Unique ID. The ID will only change if the deployment is deleted and recreated.
    
    pub uid: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1DeploymentConfig {}


/// DeploymentGroupConfig represents a deployment group that should be present in a particular environment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1DeploymentGroupConfig {
    /// Name of the deployment group in the following format: `organizations/{org}/environments/{env}/deploymentGroups/{group}`.
    
    pub name: Option<String>,
    /// Revision number which can be used by the runtime to detect if the deployment group has changed between two versions.
    #[serde(rename="revisionId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub revision_id: Option<i64>,
    /// Unique ID. The ID will only change if the deployment group is deleted and recreated.
    
    pub uid: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1DeploymentGroupConfig {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [developers create organizations](OrganizationDeveloperCreateCall) (request|response)
/// * [developers delete organizations](OrganizationDeveloperDeleteCall) (response)
/// * [developers get organizations](OrganizationDeveloperGetCall) (response)
/// * [developers update organizations](OrganizationDeveloperUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Developer {
    /// Access type.
    #[serde(rename="accessType")]
    
    pub access_type: Option<String>,
    /// Developer app family.
    #[serde(rename="appFamily")]
    
    pub app_family: Option<String>,
    /// List of apps associated with the developer.
    
    pub apps: Option<Vec<String>>,
    /// Optional. Developer attributes (name/value pairs). The custom attribute limit is 18.
    
    pub attributes: Option<Vec<GoogleCloudApigeeV1Attribute>>,
    /// List of companies associated with the developer.
    
    pub companies: Option<Vec<String>>,
    /// Output only. Time at which the developer was created in milliseconds since epoch.
    #[serde(rename="createdAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub created_at: Option<i64>,
    /// ID of the developer. **Note**: IDs are generated internally by Apigee and are not guaranteed to stay the same over time.
    #[serde(rename="developerId")]
    
    pub developer_id: Option<String>,
    /// Required. Email address of the developer. This value is used to uniquely identify the developer in Apigee hybrid. Note that the email address has to be in lowercase only.
    
    pub email: Option<String>,
    /// Required. First name of the developer.
    #[serde(rename="firstName")]
    
    pub first_name: Option<String>,
    /// Output only. Time at which the developer was last modified in milliseconds since epoch.
    #[serde(rename="lastModifiedAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_modified_at: Option<i64>,
    /// Required. Last name of the developer.
    #[serde(rename="lastName")]
    
    pub last_name: Option<String>,
    /// Output only. Name of the Apigee organization in which the developer resides.
    #[serde(rename="organizationName")]
    
    pub organization_name: Option<String>,
    /// Output only. Status of the developer. Valid values are `active` and `inactive`.
    
    pub status: Option<String>,
    /// Required. User name of the developer. Not used by Apigee hybrid.
    #[serde(rename="userName")]
    
    pub user_name: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1Developer {}
impl client::ResponseResult for GoogleCloudApigeeV1Developer {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [developers apps create organizations](OrganizationDeveloperAppCreateCall) (request|response)
/// * [developers apps delete organizations](OrganizationDeveloperAppDeleteCall) (response)
/// * [developers apps generate key pair or update developer app status organizations](OrganizationDeveloperAppGenerateKeyPairOrUpdateDeveloperAppStatuCall) (request|response)
/// * [developers apps get organizations](OrganizationDeveloperAppGetCall) (response)
/// * [developers apps update organizations](OrganizationDeveloperAppUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1DeveloperApp {
    /// List of API products associated with the developer app.
    #[serde(rename="apiProducts")]
    
    pub api_products: Option<Vec<String>>,
    /// Developer app family.
    #[serde(rename="appFamily")]
    
    pub app_family: Option<String>,
    /// ID of the developer app.
    #[serde(rename="appId")]
    
    pub app_id: Option<String>,
    /// List of attributes for the developer app.
    
    pub attributes: Option<Vec<GoogleCloudApigeeV1Attribute>>,
    /// Callback URL used by OAuth 2.0 authorization servers to communicate authorization codes back to developer apps.
    #[serde(rename="callbackUrl")]
    
    pub callback_url: Option<String>,
    /// Output only. Time the developer app was created in milliseconds since epoch.
    #[serde(rename="createdAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub created_at: Option<i64>,
    /// Output only. Set of credentials for the developer app consisting of the consumer key/secret pairs associated with the API products.
    
    pub credentials: Option<Vec<GoogleCloudApigeeV1Credential>>,
    /// ID of the developer.
    #[serde(rename="developerId")]
    
    pub developer_id: Option<String>,
    /// Expiration time, in milliseconds, for the consumer key that is generated for the developer app. If not set or left to the default value of `-1`, the API key never expires. The expiration time can't be updated after it is set.
    #[serde(rename="keyExpiresIn")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub key_expires_in: Option<i64>,
    /// Output only. Time the developer app was modified in milliseconds since epoch.
    #[serde(rename="lastModifiedAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_modified_at: Option<i64>,
    /// Name of the developer app.
    
    pub name: Option<String>,
    /// Scopes to apply to the developer app. The specified scopes must already exist for the API product that you associate with the developer app.
    
    pub scopes: Option<Vec<String>>,
    /// Status of the credential. Valid values include `approved` or `revoked`.
    
    pub status: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1DeveloperApp {}
impl client::ResponseResult for GoogleCloudApigeeV1DeveloperApp {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [developers apps keys apiproducts delete organizations](OrganizationDeveloperAppKeyApiproductDeleteCall) (response)
/// * [developers apps keys create create organizations](OrganizationDeveloperAppKeyCreateCreateCall) (request|response)
/// * [developers apps keys create organizations](OrganizationDeveloperAppKeyCreateCall) (request|response)
/// * [developers apps keys delete organizations](OrganizationDeveloperAppKeyDeleteCall) (response)
/// * [developers apps keys get organizations](OrganizationDeveloperAppKeyGetCall) (response)
/// * [developers apps keys replace developer app key organizations](OrganizationDeveloperAppKeyReplaceDeveloperAppKeyCall) (request|response)
/// * [developers apps keys update developer app key organizations](OrganizationDeveloperAppKeyUpdateDeveloperAppKeyCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1DeveloperAppKey {
    /// List of API products for which the credential can be used. **Note**: Do not specify the list of API products when creating a consumer key and secret for a developer app. Instead, use the UpdateDeveloperAppKey API to make the association after the consumer key and secret are created.
    #[serde(rename="apiProducts")]
    
    pub api_products: Option<Vec<json::Value>>,
    /// List of attributes associated with the credential.
    
    pub attributes: Option<Vec<GoogleCloudApigeeV1Attribute>>,
    /// Consumer key.
    #[serde(rename="consumerKey")]
    
    pub consumer_key: Option<String>,
    /// Secret key.
    #[serde(rename="consumerSecret")]
    
    pub consumer_secret: Option<String>,
    /// Time the developer app expires in milliseconds since epoch.
    #[serde(rename="expiresAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expires_at: Option<i64>,
    /// Input only. Expiration time, in seconds, for the consumer key. If not set or left to the default value of `-1`, the API key never expires. The expiration time can't be updated after it is set.
    #[serde(rename="expiresInSeconds")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expires_in_seconds: Option<i64>,
    /// Time the developer app was created in milliseconds since epoch.
    #[serde(rename="issuedAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub issued_at: Option<i64>,
    /// Scopes to apply to the app. The specified scope names must already be defined for the API product that you associate with the app.
    
    pub scopes: Option<Vec<String>>,
    /// Status of the credential. Valid values include `approved` or `revoked`.
    
    pub status: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1DeveloperAppKey {}
impl client::ResponseResult for GoogleCloudApigeeV1DeveloperAppKey {}


/// Account balance for the developer.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [developers balance adjust organizations](OrganizationDeveloperBalanceAdjustCall) (response)
/// * [developers balance credit organizations](OrganizationDeveloperBalanceCreditCall) (response)
/// * [developers get balance organizations](OrganizationDeveloperGetBalanceCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1DeveloperBalance {
    /// Output only. List of all wallets. Each individual wallet stores the account balance for a particular currency.
    
    pub wallets: Option<Vec<GoogleCloudApigeeV1DeveloperBalanceWallet>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1DeveloperBalance {}


/// Wallet used to manage an account balance for a particular currency.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1DeveloperBalanceWallet {
    /// Current remaining balance of the developer for a particular currency.
    
    pub balance: Option<GoogleTypeMoney>,
    /// Output only. Time at which the developer last added credit to the account in milliseconds since epoch.
    #[serde(rename="lastCreditTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_credit_time: Option<i64>,
}

impl client::Part for GoogleCloudApigeeV1DeveloperBalanceWallet {}


/// Monetization configuration for the developer.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [developers get monetization config organizations](OrganizationDeveloperGetMonetizationConfigCall) (response)
/// * [developers update monetization config organizations](OrganizationDeveloperUpdateMonetizationConfigCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1DeveloperMonetizationConfig {
    /// Billing type.
    #[serde(rename="billingType")]
    
    pub billing_type: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1DeveloperMonetizationConfig {}
impl client::ResponseResult for GoogleCloudApigeeV1DeveloperMonetizationConfig {}


/// Structure of a DeveloperSubscription.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [developers subscriptions create organizations](OrganizationDeveloperSubscriptionCreateCall) (request|response)
/// * [developers subscriptions expire organizations](OrganizationDeveloperSubscriptionExpireCall) (response)
/// * [developers subscriptions get organizations](OrganizationDeveloperSubscriptionGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1DeveloperSubscription {
    /// Name of the API product for which the developer is purchasing a subscription.
    
    pub apiproduct: Option<String>,
    /// Output only. Time when the API product subscription was created in milliseconds since epoch.
    #[serde(rename="createdAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub created_at: Option<i64>,
    /// Time when the API product subscription ends in milliseconds since epoch.
    #[serde(rename="endTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub end_time: Option<i64>,
    /// Output only. Time when the API product subscription was last modified in milliseconds since epoch.
    #[serde(rename="lastModifiedAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_modified_at: Option<i64>,
    /// Output only. Name of the API product subscription.
    
    pub name: Option<String>,
    /// Time when the API product subscription starts in milliseconds since epoch.
    #[serde(rename="startTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start_time: Option<i64>,
}

impl client::RequestValue for GoogleCloudApigeeV1DeveloperSubscription {}
impl client::ResponseResult for GoogleCloudApigeeV1DeveloperSubscription {}


/// Encapsulates a metric grouped by dimension.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1DimensionMetric {
    /// List of metrics.
    
    pub metrics: Option<Vec<GoogleCloudApigeeV1Metric>>,
    /// Name of the dimension.
    
    pub name: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1DimensionMetric {}


/// Apigee endpoint attachment. For more information, see \[Southbound networking patterns\] (https://cloud.google.com/apigee/docs/api-platform/architecture/southbound-networking-patterns-endpoints).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [endpoint attachments create organizations](OrganizationEndpointAttachmentCreateCall) (request)
/// * [endpoint attachments get organizations](OrganizationEndpointAttachmentGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1EndpointAttachment {
    /// Output only. State of the endpoint attachment connection to the service attachment.
    #[serde(rename="connectionState")]
    
    pub connection_state: Option<String>,
    /// Output only. Host that can be used in either the HTTP target endpoint directly or as the host in target server.
    
    pub host: Option<String>,
    /// Required. Location of the endpoint attachment.
    
    pub location: Option<String>,
    /// Name of the endpoint attachment. Use the following structure in your request: `organizations/{org}/endpointAttachments/{endpoint_attachment}`
    
    pub name: Option<String>,
    /// Format: projects/*/regions/*/serviceAttachments/*
    #[serde(rename="serviceAttachment")]
    
    pub service_attachment: Option<String>,
    /// Output only. State of the endpoint attachment. Values other than `ACTIVE` mean the resource is not ready to use.
    
    pub state: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1EndpointAttachment {}
impl client::ResponseResult for GoogleCloudApigeeV1EndpointAttachment {}


/// EndpointChainingRule specifies the proxies contained in a particular deployment group, so that other deployment groups can find them in chaining calls.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1EndpointChainingRule {
    /// The deployment group to target for cross-shard chaining calls to these proxies.
    #[serde(rename="deploymentGroup")]
    
    pub deployment_group: Option<String>,
    /// List of proxy ids which may be found in the given deployment group.
    #[serde(rename="proxyIds")]
    
    pub proxy_ids: Option<Vec<String>>,
}

impl client::Part for GoogleCloudApigeeV1EndpointChainingRule {}


/// Metadata common to many entities in this API.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1EntityMetadata {
    /// Time at which the API proxy was created, in milliseconds since epoch.
    #[serde(rename="createdAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub created_at: Option<i64>,
    /// Time at which the API proxy was most recently modified, in milliseconds since epoch.
    #[serde(rename="lastModifiedAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_modified_at: Option<i64>,
    /// The type of entity described
    #[serde(rename="subType")]
    
    pub sub_type: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1EntityMetadata {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments create organizations](OrganizationEnvironmentCreateCall) (request)
/// * [environments get organizations](OrganizationEnvironmentGetCall) (response)
/// * [environments modify environment organizations](OrganizationEnvironmentModifyEnvironmentCall) (request)
/// * [environments update organizations](OrganizationEnvironmentUpdateCall) (request|response)
/// * [environments update environment organizations](OrganizationEnvironmentUpdateEnvironmentCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Environment {
    /// Optional. API Proxy type supported by the environment. The type can be set when creating the Environment and cannot be changed.
    #[serde(rename="apiProxyType")]
    
    pub api_proxy_type: Option<String>,
    /// Output only. Creation time of this environment as milliseconds since epoch.
    #[serde(rename="createdAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub created_at: Option<i64>,
    /// Optional. Deployment type supported by the environment. The deployment type can be set when creating the environment and cannot be changed. When you enable archive deployment, you will be **prevented from performing** a [subset of actions](https://cloud.google.com/apigee/docs/api-platform/local-development/overview#prevented-actions) within the environment, including: * Managing the deployment of API proxy or shared flow revisions * Creating, updating, or deleting resource files * Creating, updating, or deleting target servers
    #[serde(rename="deploymentType")]
    
    pub deployment_type: Option<String>,
    /// Optional. Description of the environment.
    
    pub description: Option<String>,
    /// Optional. Display name for this environment.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Optional. Url of the forward proxy to be applied to the runtime instances in this environment. Must be in the format of {scheme}://{hostname}:{port}. Note that scheme must be one of "http" or "https", and port must be supplied.
    #[serde(rename="forwardProxyUri")]
    
    pub forward_proxy_uri: Option<String>,
    /// Output only. Last modification time of this environment as milliseconds since epoch.
    #[serde(rename="lastModifiedAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_modified_at: Option<i64>,
    /// Required. Name of the environment. Values must match the regular expression `^[.\\p{Alnum}-_]{1,255}$`
    
    pub name: Option<String>,
    /// Optional. NodeConfig of the environment.
    #[serde(rename="nodeConfig")]
    
    pub node_config: Option<GoogleCloudApigeeV1NodeConfig>,
    /// Optional. Key-value pairs that may be used for customizing the environment.
    
    pub properties: Option<GoogleCloudApigeeV1Properties>,
    /// Output only. State of the environment. Values other than ACTIVE means the resource is not ready to use.
    
    pub state: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1Environment {}
impl client::ResponseResult for GoogleCloudApigeeV1Environment {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments get deployed config organizations](OrganizationEnvironmentGetDeployedConfigCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1EnvironmentConfig {
    /// The location for the config blob of API Runtime Control, aka Envoy Adapter, for op-based authentication as a URI, e.g. a Cloud Storage URI. This is only used by Envoy-based gateways.
    #[serde(rename="arcConfigLocation")]
    
    pub arc_config_location: Option<String>,
    /// Time that the environment configuration was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// List of data collectors used by the deployments in the environment.
    #[serde(rename="dataCollectors")]
    
    pub data_collectors: Option<Vec<GoogleCloudApigeeV1DataCollectorConfig>>,
    /// Debug mask that applies to all deployments in the environment.
    #[serde(rename="debugMask")]
    
    pub debug_mask: Option<GoogleCloudApigeeV1DebugMask>,
    /// List of deployment groups in the environment.
    #[serde(rename="deploymentGroups")]
    
    pub deployment_groups: Option<Vec<GoogleCloudApigeeV1DeploymentGroupConfig>>,
    /// List of deployments in the environment.
    
    pub deployments: Option<Vec<GoogleCloudApigeeV1DeploymentConfig>>,
    /// Revision ID for environment-scoped resources (e.g. target servers, keystores) in this config. This ID will increment any time a resource not scoped to a deployment group changes.
    #[serde(rename="envScopedRevisionId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub env_scoped_revision_id: Option<i64>,
    /// Feature flags inherited from the organization and environment.
    #[serde(rename="featureFlags")]
    
    pub feature_flags: Option<HashMap<String, String>>,
    /// List of flow hooks in the environment.
    
    pub flowhooks: Option<Vec<GoogleCloudApigeeV1FlowHookConfig>>,
    /// The forward proxy's url to be used by the runtime. When set, runtime will send requests to the target via the given forward proxy. This is only used by programmable gateways.
    #[serde(rename="forwardProxyUri")]
    
    pub forward_proxy_uri: Option<String>,
    /// The location for the gateway config blob as a URI, e.g. a Cloud Storage URI. This is only used by Envoy-based gateways.
    #[serde(rename="gatewayConfigLocation")]
    
    pub gateway_config_location: Option<String>,
    /// List of keystores in the environment.
    
    pub keystores: Option<Vec<GoogleCloudApigeeV1KeystoreConfig>>,
    /// Name of the environment configuration in the following format: `organizations/{org}/environments/{env}/configs/{config}`
    
    pub name: Option<String>,
    /// Used by the Control plane to add context information to help detect the source of the document during diagnostics and debugging.
    
    pub provider: Option<String>,
    /// Name of the PubSub topic for the environment.
    #[serde(rename="pubsubTopic")]
    
    pub pubsub_topic: Option<String>,
    /// List of resource references in the environment.
    #[serde(rename="resourceReferences")]
    
    pub resource_references: Option<Vec<GoogleCloudApigeeV1ReferenceConfig>>,
    /// List of resource versions in the environment.
    
    pub resources: Option<Vec<GoogleCloudApigeeV1ResourceConfig>>,
    /// Revision ID of the environment configuration. The higher the value, the more recently the configuration was deployed.
    #[serde(rename="revisionId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub revision_id: Option<i64>,
    /// DEPRECATED: Use revision_id.
    #[serde(rename="sequenceNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub sequence_number: Option<i64>,
    /// List of target servers in the environment. Disabled target servers are not displayed.
    
    pub targets: Option<Vec<GoogleCloudApigeeV1TargetServerConfig>>,
    /// Trace configurations. Contains config for the environment and config overrides for specific API proxies.
    #[serde(rename="traceConfig")]
    
    pub trace_config: Option<GoogleCloudApigeeV1RuntimeTraceConfig>,
    /// Unique ID for the environment configuration. The ID will only change if the environment is deleted and recreated.
    
    pub uid: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1EnvironmentConfig {}


/// EnvironmentGroup configuration. An environment group is used to group one or more Apigee environments under a single host name.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [envgroups create organizations](OrganizationEnvgroupCreateCall) (request)
/// * [envgroups get organizations](OrganizationEnvgroupGetCall) (response)
/// * [envgroups patch organizations](OrganizationEnvgroupPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1EnvironmentGroup {
    /// Output only. The time at which the environment group was created as milliseconds since epoch.
    #[serde(rename="createdAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub created_at: Option<i64>,
    /// Required. Host names for this environment group.
    
    pub hostnames: Option<Vec<String>>,
    /// Output only. The time at which the environment group was last updated as milliseconds since epoch.
    #[serde(rename="lastModifiedAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_modified_at: Option<i64>,
    /// ID of the environment group.
    
    pub name: Option<String>,
    /// Output only. State of the environment group. Values other than ACTIVE means the resource is not ready to use.
    
    pub state: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1EnvironmentGroup {}
impl client::ResponseResult for GoogleCloudApigeeV1EnvironmentGroup {}


/// EnvironmentGroupAttachment is a resource which defines an attachment of an environment to an environment group.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [envgroups attachments create organizations](OrganizationEnvgroupAttachmentCreateCall) (request)
/// * [envgroups attachments get organizations](OrganizationEnvgroupAttachmentGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1EnvironmentGroupAttachment {
    /// Output only. The time at which the environment group attachment was created as milliseconds since epoch.
    #[serde(rename="createdAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub created_at: Option<i64>,
    /// Required. ID of the attached environment.
    
    pub environment: Option<String>,
    /// Output only. ID of the environment group.
    #[serde(rename="environmentGroupId")]
    
    pub environment_group_id: Option<String>,
    /// ID of the environment group attachment.
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1EnvironmentGroupAttachment {}
impl client::ResponseResult for GoogleCloudApigeeV1EnvironmentGroupAttachment {}


/// EnvironmentGroupConfig is a revisioned snapshot of an EnvironmentGroup and its associated routing rules.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [envgroups get deployed ingress config organizations](OrganizationEnvgroupGetDeployedIngressConfigCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1EnvironmentGroupConfig {
    /// A list of proxies in each deployment group for proxy chaining calls.
    #[serde(rename="endpointChainingRules")]
    
    pub endpoint_chaining_rules: Option<Vec<GoogleCloudApigeeV1EndpointChainingRule>>,
    /// Host names for the environment group.
    
    pub hostnames: Option<Vec<String>>,
    /// When this message appears in the top-level IngressConfig, this field will be populated in lieu of the inlined routing_rules and hostnames fields. Some URL for downloading the full EnvironmentGroupConfig for this group.
    
    pub location: Option<String>,
    /// Name of the environment group in the following format: `organizations/{org}/envgroups/{envgroup}`.
    
    pub name: Option<String>,
    /// Revision id that defines the ordering of the EnvironmentGroupConfig resource. The higher the revision, the more recently the configuration was deployed.
    #[serde(rename="revisionId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub revision_id: Option<i64>,
    /// Ordered list of routing rules defining how traffic to this environment group's hostnames should be routed to different environments.
    #[serde(rename="routingRules")]
    
    pub routing_rules: Option<Vec<GoogleCloudApigeeV1RoutingRule>>,
    /// A unique id for the environment group config that will only change if the environment group is deleted and recreated.
    
    pub uid: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1EnvironmentGroupConfig {}


/// Request for ExpireDeveloperSubscription.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [developers subscriptions expire organizations](OrganizationDeveloperSubscriptionExpireCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ExpireDeveloperSubscriptionRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudApigeeV1ExpireDeveloperSubscriptionRequest {}


/// Details of an export job.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments analytics exports create organizations](OrganizationEnvironmentAnalyticExportCreateCall) (response)
/// * [environments analytics exports get organizations](OrganizationEnvironmentAnalyticExportGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Export {
    /// Output only. Time the export job was created.
    
    pub created: Option<String>,
    /// Name of the datastore that is the destination of the export job [datastore]
    #[serde(rename="datastoreName")]
    
    pub datastore_name: Option<String>,
    /// Description of the export job.
    
    pub description: Option<String>,
    /// Output only. Error is set when export fails
    
    pub error: Option<String>,
    /// Output only. Execution time for this export job. If the job is still in progress, it will be set to the amount of time that has elapsed since`created`, in seconds. Else, it will set to (`updated` - `created`), in seconds.
    #[serde(rename="executionTime")]
    
    pub execution_time: Option<String>,
    /// Display name of the export job.
    
    pub name: Option<String>,
    /// Output only. Self link of the export job. A URI that can be used to retrieve the status of an export job. Example: `/organizations/myorg/environments/myenv/analytics/exports/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd`
    #[serde(rename="self")]
    
    pub self_: Option<String>,
    /// Output only. Status of the export job. Valid values include `enqueued`, `running`, `completed`, and `failed`.
    
    pub state: Option<String>,
    /// Output only. Time the export job was last updated.
    
    pub updated: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1Export {}


/// Request body for \[CreateExportRequest\]
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments analytics exports create organizations](OrganizationEnvironmentAnalyticExportCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ExportRequest {
    /// Optional. Delimiter used in the CSV file, if `outputFormat` is set to `csv`. Defaults to the `,` (comma) character. Supported delimiter characters include comma (`,`), pipe (`|`), and tab (`\t`).
    #[serde(rename="csvDelimiter")]
    
    pub csv_delimiter: Option<String>,
    /// Required. Name of the preconfigured datastore.
    #[serde(rename="datastoreName")]
    
    pub datastore_name: Option<String>,
    /// Required. Date range of the data to export.
    #[serde(rename="dateRange")]
    
    pub date_range: Option<GoogleCloudApigeeV1DateRange>,
    /// Optional. Description of the export job.
    
    pub description: Option<String>,
    /// Required. Display name of the export job.
    
    pub name: Option<String>,
    /// Optional. Output format of the export. Valid values include: `csv` or `json`. Defaults to `json`. Note: Configure the delimiter for CSV output using the `csvDelimiter` property.
    #[serde(rename="outputFormat")]
    
    pub output_format: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1ExportRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments flowhooks attach shared flow to flow hook organizations](OrganizationEnvironmentFlowhookAttachSharedFlowToFlowHookCall) (request|response)
/// * [environments flowhooks detach shared flow from flow hook organizations](OrganizationEnvironmentFlowhookDetachSharedFlowFromFlowHookCall) (response)
/// * [environments flowhooks get organizations](OrganizationEnvironmentFlowhookGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1FlowHook {
    /// Optional. Flag that specifies whether execution should continue if the flow hook throws an exception. Set to `true` to continue execution. Set to `false` to stop execution if the flow hook throws an exception. Defaults to `true`.
    #[serde(rename="continueOnError")]
    
    pub continue_on_error: Option<bool>,
    /// Description of the flow hook.
    
    pub description: Option<String>,
    /// Output only. Where in the API call flow the flow hook is invoked. Must be one of `PreProxyFlowHook`, `PostProxyFlowHook`, `PreTargetFlowHook`, or `PostTargetFlowHook`.
    #[serde(rename="flowHookPoint")]
    
    pub flow_hook_point: Option<String>,
    /// Shared flow attached to this flow hook, or empty if there is none attached.
    #[serde(rename="sharedFlow")]
    
    pub shared_flow: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1FlowHook {}
impl client::ResponseResult for GoogleCloudApigeeV1FlowHook {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1FlowHookConfig {
    /// Flag that specifies whether the flow should abort after an error in the flow hook. Defaults to `true` (continue on error).
    #[serde(rename="continueOnError")]
    
    pub continue_on_error: Option<bool>,
    /// Name of the flow hook in the following format: `organizations/{org}/environments/{env}/flowhooks/{point}`. Valid `point` values include: `PreProxyFlowHook`, `PostProxyFlowHook`, `PreTargetFlowHook`, and `PostTargetFlowHook`
    
    pub name: Option<String>,
    /// Name of the shared flow to invoke in the following format: `organizations/{org}/sharedflows/{sharedflow}`
    #[serde(rename="sharedFlowName")]
    
    pub shared_flow_name: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1FlowHookConfig {}


/// Request for GenerateDownloadUrl method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments archive deployments generate download url organizations](OrganizationEnvironmentArchiveDeploymentGenerateDownloadUrlCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1GenerateDownloadUrlRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudApigeeV1GenerateDownloadUrlRequest {}


/// Response for GenerateDownloadUrl method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments archive deployments generate download url organizations](OrganizationEnvironmentArchiveDeploymentGenerateDownloadUrlCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1GenerateDownloadUrlResponse {
    /// The Google Cloud Storage signed URL that can be used to download the Archive zip file.
    #[serde(rename="downloadUri")]
    
    pub download_uri: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1GenerateDownloadUrlResponse {}


/// Request for GenerateUploadUrl method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments archive deployments generate upload url organizations](OrganizationEnvironmentArchiveDeploymentGenerateUploadUrlCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1GenerateUploadUrlRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudApigeeV1GenerateUploadUrlRequest {}


/// Response for GenerateUploadUrl method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments archive deployments generate upload url organizations](OrganizationEnvironmentArchiveDeploymentGenerateUploadUrlCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1GenerateUploadUrlResponse {
    /// The Google Cloud Storage signed URL that can be used to upload a new Archive zip file.
    #[serde(rename="uploadUri")]
    
    pub upload_uri: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1GenerateUploadUrlResponse {}


/// The response for GetAsyncQueryResultUrl
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments queries get resulturl organizations](OrganizationEnvironmentQueryGetResulturlCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1GetAsyncQueryResultUrlResponse {
    /// The list of Signed URLs generated by the CreateAsyncQuery request
    
    pub urls: Option<Vec<GoogleCloudApigeeV1GetAsyncQueryResultUrlResponseURLInfo>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1GetAsyncQueryResultUrlResponse {}


/// A Signed URL and the relevant metadata associated with it.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1GetAsyncQueryResultUrlResponseURLInfo {
    /// The MD5 Hash of the JSON data
    
    pub md5: Option<String>,
    /// The size of the returned file in bytes
    #[serde(rename="sizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub size_bytes: Option<i64>,
    /// The signed URL of the JSON data. Will be of the form `https://storage.googleapis.com/example-bucket/cat.jpeg?X-Goog-Algorithm= GOOG4-RSA-SHA256&X-Goog-Credential=example%40example-project.iam.gserviceaccount .com%2F20181026%2Fus-central1%2Fstorage%2Fgoog4_request&X-Goog-Date=20181026T18 1309Z&X-Goog-Expires=900&X-Goog-SignedHeaders=host&X-Goog-Signature=247a2aa45f16 9edf4d187d54e7cc46e4731b1e6273242c4f4c39a1d2507a0e58706e25e3a85a7dbb891d62afa849 6def8e260c1db863d9ace85ff0a184b894b117fe46d1225c82f2aa19efd52cf21d3e2022b3b868dc c1aca2741951ed5bf3bb25a34f5e9316a2841e8ff4c530b22ceaa1c5ce09c7cbb5732631510c2058 0e61723f5594de3aea497f195456a2ff2bdd0d13bad47289d8611b6f9cfeef0c46c91a455b94e90a 66924f722292d21e24d31dcfb38ce0c0f353ffa5a9756fc2a9f2b40bc2113206a81e324fc4fd6823 a29163fa845c8ae7eca1fcf6e5bb48b3200983c56c5ca81fffb151cca7402beddfc4a76b13344703 2ea7abedc098d2eb14a7`
    
    pub uri: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1GetAsyncQueryResultUrlResponseURLInfo {}


/// Request for GetSyncAuthorization.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get sync authorization organizations](OrganizationGetSyncAuthorizationCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1GetSyncAuthorizationRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudApigeeV1GetSyncAuthorizationRequest {}


/// Represents the pairing of GraphQL operation types and the GraphQL operation name.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1GraphQLOperation {
    /// GraphQL operation name. The name and operation type will be used to apply quotas. If no name is specified, the quota will be applied to all GraphQL operations irrespective of their operation names in the payload.
    
    pub operation: Option<String>,
    /// Required. GraphQL operation types. Valid values include `query` or `mutation`. **Note**: Apigee does not currently support `subscription` types.
    #[serde(rename="operationTypes")]
    
    pub operation_types: Option<Vec<String>>,
}

impl client::Part for GoogleCloudApigeeV1GraphQLOperation {}


/// Binds the resources in a proxy or remote service with the GraphQL operation and its associated quota enforcement.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1GraphQLOperationConfig {
    /// Required. Name of the API proxy endpoint or remote service with which the GraphQL operation and quota are associated.
    #[serde(rename="apiSource")]
    
    pub api_source: Option<String>,
    /// Custom attributes associated with the operation.
    
    pub attributes: Option<Vec<GoogleCloudApigeeV1Attribute>>,
    /// Required. List of GraphQL name/operation type pairs for the proxy or remote service to which quota will be applied. If only operation types are specified, the quota will be applied to all GraphQL requests irrespective of the GraphQL name. **Note**: Currently, you can specify only a single GraphQLOperation. Specifying more than one will cause the operation to fail.
    
    pub operations: Option<Vec<GoogleCloudApigeeV1GraphQLOperation>>,
    /// Quota parameters to be enforced for the resources, methods, and API source combination. If none are specified, quota enforcement will not be done.
    
    pub quota: Option<GoogleCloudApigeeV1Quota>,
}

impl client::Part for GoogleCloudApigeeV1GraphQLOperationConfig {}


/// List of graphQL operation configuration details associated with Apigee API proxies or remote services. Remote services are non-Apigee proxies, such as Istio-Envoy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1GraphQLOperationGroup {
    /// Flag that specifies whether the configuration is for Apigee API proxy or a remote service. Valid values include `proxy` or `remoteservice`. Defaults to `proxy`. Set to `proxy` when Apigee API proxies are associated with the API product. Set to `remoteservice` when non-Apigee proxies like Istio-Envoy are associated with the API product.
    #[serde(rename="operationConfigType")]
    
    pub operation_config_type: Option<String>,
    /// Required. List of operation configurations for either Apigee API proxies or other remote services that are associated with this API product.
    #[serde(rename="operationConfigs")]
    
    pub operation_configs: Option<Vec<GoogleCloudApigeeV1GraphQLOperationConfig>>,
}

impl client::Part for GoogleCloudApigeeV1GraphQLOperationGroup {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get deployed ingress config organizations](OrganizationGetDeployedIngressConfigCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1IngressConfig {
    /// List of environment groups in the organization.
    #[serde(rename="environmentGroups")]
    
    pub environment_groups: Option<Vec<GoogleCloudApigeeV1EnvironmentGroupConfig>>,
    /// Name of the resource in the following format: `organizations/{org}/deployedIngressConfig`.
    
    pub name: Option<String>,
    /// Time at which the IngressConfig revision was created.
    #[serde(rename="revisionCreateTime")]
    
    pub revision_create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Revision id that defines the ordering on IngressConfig resources. The higher the revision, the more recently the configuration was deployed.
    #[serde(rename="revisionId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub revision_id: Option<i64>,
    /// A unique id for the ingress config that will only change if the organization is deleted and recreated.
    
    pub uid: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1IngressConfig {}


/// Apigee runtime instance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances create organizations](OrganizationInstanceCreateCall) (request)
/// * [instances get organizations](OrganizationInstanceGetCall) (response)
/// * [instances patch organizations](OrganizationInstancePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Instance {
    /// Optional. Customer accept list represents the list of projects (id/number) on customer side that can privately connect to the service attachment. It is an optional field which the customers can provide during the instance creation. By default, the customer project associated with the Apigee organization will be included to the list.
    #[serde(rename="consumerAcceptList")]
    
    pub consumer_accept_list: Option<Vec<String>>,
    /// Output only. Time the instance was created in milliseconds since epoch.
    #[serde(rename="createdAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub created_at: Option<i64>,
    /// Optional. Description of the instance.
    
    pub description: Option<String>,
    /// Customer Managed Encryption Key (CMEK) used for disk and volume encryption. Required for Apigee paid subscriptions only. Use the following format: `projects/([^/]+)/locations/([^/]+)/keyRings/([^/]+)/cryptoKeys/([^/]+)`
    #[serde(rename="diskEncryptionKeyName")]
    
    pub disk_encryption_key_name: Option<String>,
    /// Optional. Display name for the instance.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Internal hostname or IP address of the Apigee endpoint used by clients to connect to the service.
    
    pub host: Option<String>,
    /// Optional. Comma-separated list of CIDR blocks of length 22 and/or 28 used to create the Apigee instance. Providing CIDR ranges is optional. You can provide just /22 or /28 or both (or neither). Ranges you provide should be freely available as part of a larger named range you have allocated to the Service Networking peering. If this parameter is not provided, Apigee automatically requests an available /22 and /28 CIDR block from Service Networking. Use the /22 CIDR block for configuring your firewall needs to allow traffic from Apigee. Input formats: `a.b.c.d/22` or `e.f.g.h/28` or `a.b.c.d/22,e.f.g.h/28`
    #[serde(rename="ipRange")]
    
    pub ip_range: Option<String>,
    /// Output only. Time the instance was last modified in milliseconds since epoch.
    #[serde(rename="lastModifiedAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_modified_at: Option<i64>,
    /// Required. Compute Engine location where the instance resides.
    
    pub location: Option<String>,
    /// Required. Resource ID of the instance. Values must match the regular expression `^a-z{0,30}[a-z\d]$`.
    
    pub name: Option<String>,
    /// Optional. Size of the CIDR block range that will be reserved by the instance. PAID organizations support `SLASH_16` to `SLASH_20` and defaults to `SLASH_16`. Evaluation organizations support only `SLASH_23`.
    #[serde(rename="peeringCidrRange")]
    
    pub peering_cidr_range: Option<String>,
    /// Output only. Port number of the exposed Apigee endpoint.
    
    pub port: Option<String>,
    /// Output only. Version of the runtime system running in the instance. The runtime system is the set of components that serve the API Proxy traffic in your Environments.
    #[serde(rename="runtimeVersion")]
    
    pub runtime_version: Option<String>,
    /// Output only. Resource name of the service attachment created for the instance in the format: `projects/*/regions/*/serviceAttachments/*` Apigee customers can privately forward traffic to this service attachment using the PSC endpoints.
    #[serde(rename="serviceAttachment")]
    
    pub service_attachment: Option<String>,
    /// Output only. State of the instance. Values other than `ACTIVE` means the resource is not ready to use.
    
    pub state: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1Instance {}
impl client::ResponseResult for GoogleCloudApigeeV1Instance {}


/// InstanceAttachment represents the installation of an environment onto an instance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances attachments create organizations](OrganizationInstanceAttachmentCreateCall) (request)
/// * [instances attachments get organizations](OrganizationInstanceAttachmentGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1InstanceAttachment {
    /// Output only. Time the attachment was created in milliseconds since epoch.
    #[serde(rename="createdAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub created_at: Option<i64>,
    /// ID of the attached environment.
    
    pub environment: Option<String>,
    /// Output only. ID of the attachment.
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1InstanceAttachment {}
impl client::ResponseResult for GoogleCloudApigeeV1InstanceAttachment {}


/// The status of a deployment as reported by a single instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1InstanceDeploymentStatus {
    /// Revisions currently deployed in MPs.
    #[serde(rename="deployedRevisions")]
    
    pub deployed_revisions: Option<Vec<GoogleCloudApigeeV1InstanceDeploymentStatusDeployedRevision>>,
    /// Current routes deployed in the ingress routing table. A route which is missing will appear in `missing_routes`.
    #[serde(rename="deployedRoutes")]
    
    pub deployed_routes: Option<Vec<GoogleCloudApigeeV1InstanceDeploymentStatusDeployedRoute>>,
    /// ID of the instance reporting the status.
    
    pub instance: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1InstanceDeploymentStatus {}


/// Revisions deployed in the MPs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1InstanceDeploymentStatusDeployedRevision {
    /// Percentage of MP replicas reporting this revision.
    
    pub percentage: Option<i32>,
    /// API proxy revision reported as deployed.
    
    pub revision: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1InstanceDeploymentStatusDeployedRevision {}


/// Route deployed in the ingress routing table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1InstanceDeploymentStatusDeployedRoute {
    /// Base path in the routing table.
    
    pub basepath: Option<String>,
    /// Environment group where this route is installed.
    
    pub envgroup: Option<String>,
    /// Destination environment. This will be empty if the route is not yet reported.
    
    pub environment: Option<String>,
    /// Percentage of ingress replicas reporting this route.
    
    pub percentage: Option<i32>,
}

impl client::Part for GoogleCloudApigeeV1InstanceDeploymentStatusDeployedRoute {}


/// Configuration for the Integration add-on.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1IntegrationConfig {
    /// Flag that specifies whether the Integration add-on is enabled.
    
    pub enabled: Option<bool>,
}

impl client::Part for GoogleCloudApigeeV1IntegrationConfig {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1KeyAliasReference {
    /// Alias ID. Must exist in the keystore referred to by the reference.
    #[serde(rename="aliasId")]
    
    pub alias_id: Option<String>,
    /// Reference name in the following format: `organizations/{org}/environments/{env}/references/{reference}`
    
    pub reference: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1KeyAliasReference {}


/// Key value map pair where the value represents the data associated with the corresponding key. **Note**: Supported for Apigee hybrid 1.8.x and higher.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apis keyvaluemaps entries create organizations](OrganizationApiKeyvaluemapEntryCreateCall) (request|response)
/// * [apis keyvaluemaps entries delete organizations](OrganizationApiKeyvaluemapEntryDeleteCall) (response)
/// * [apis keyvaluemaps entries get organizations](OrganizationApiKeyvaluemapEntryGetCall) (response)
/// * [environments keyvaluemaps entries create organizations](OrganizationEnvironmentKeyvaluemapEntryCreateCall) (request|response)
/// * [environments keyvaluemaps entries delete organizations](OrganizationEnvironmentKeyvaluemapEntryDeleteCall) (response)
/// * [environments keyvaluemaps entries get organizations](OrganizationEnvironmentKeyvaluemapEntryGetCall) (response)
/// * [keyvaluemaps entries create organizations](OrganizationKeyvaluemapEntryCreateCall) (request|response)
/// * [keyvaluemaps entries delete organizations](OrganizationKeyvaluemapEntryDeleteCall) (response)
/// * [keyvaluemaps entries get organizations](OrganizationKeyvaluemapEntryGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1KeyValueEntry {
    /// Resource URI that can be used to identify the scope of the key value map entries.
    
    pub name: Option<String>,
    /// Required. Data or payload that is being retrieved and associated with the unique key.
    
    pub value: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1KeyValueEntry {}
impl client::ResponseResult for GoogleCloudApigeeV1KeyValueEntry {}


/// Collection of key/value string pairs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apis keyvaluemaps create organizations](OrganizationApiKeyvaluemapCreateCall) (request|response)
/// * [apis keyvaluemaps delete organizations](OrganizationApiKeyvaluemapDeleteCall) (response)
/// * [environments keyvaluemaps create organizations](OrganizationEnvironmentKeyvaluemapCreateCall) (request|response)
/// * [environments keyvaluemaps delete organizations](OrganizationEnvironmentKeyvaluemapDeleteCall) (response)
/// * [keyvaluemaps create organizations](OrganizationKeyvaluemapCreateCall) (request|response)
/// * [keyvaluemaps delete organizations](OrganizationKeyvaluemapDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1KeyValueMap {
    /// Optional. Flag that specifies whether entry values will be encrypted. You must set this value to `true`. Apigee X and hybrid do not support unencrytped key value maps.
    
    pub encrypted: Option<bool>,
    /// Required. ID of the key value map.
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1KeyValueMap {}
impl client::ResponseResult for GoogleCloudApigeeV1KeyValueMap {}


/// Datastore for Certificates and Aliases.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments keystores create organizations](OrganizationEnvironmentKeystoreCreateCall) (request|response)
/// * [environments keystores delete organizations](OrganizationEnvironmentKeystoreDeleteCall) (response)
/// * [environments keystores get organizations](OrganizationEnvironmentKeystoreGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Keystore {
    /// Output only. Aliases in this keystore.
    
    pub aliases: Option<Vec<String>>,
    /// Required. Resource ID for this keystore. Values must match the regular expression `[\w[:space:].-]{1,255}`.
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1Keystore {}
impl client::ResponseResult for GoogleCloudApigeeV1Keystore {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1KeystoreConfig {
    /// Aliases in the keystore.
    
    pub aliases: Option<Vec<GoogleCloudApigeeV1AliasRevisionConfig>>,
    /// Resource name in the following format: `organizations/{org}/environments/{env}/keystores/{keystore}`
    
    pub name: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1KeystoreConfig {}


/// the response for ListApiCategoriesRequest.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sites apicategories list organizations](OrganizationSiteApicategoryListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListApiCategoriesResponse {
    /// Details of categories.
    
    pub data: Option<Vec<GoogleCloudApigeeV1ApiCategoryData>>,
    /// ID that can be used to find errors in the log files.
    #[serde(rename="errorCode")]
    
    pub error_code: Option<String>,
    /// Description of the operation.
    
    pub message: Option<String>,
    /// ID that can be used to find request details in the log files.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
    /// Status of the operation.
    
    pub status: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListApiCategoriesResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apiproducts list organizations](OrganizationApiproductListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListApiProductsResponse {
    /// Lists all API product names defined for an organization.
    #[serde(rename="apiProduct")]
    
    pub api_product: Option<Vec<GoogleCloudApigeeV1ApiProduct>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListApiProductsResponse {}


/// To change this message, in the same CL add a change log in go/changing-api-proto-breaks-ui
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apis list organizations](OrganizationApiListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListApiProxiesResponse {
    /// no description provided
    
    pub proxies: Option<Vec<GoogleCloudApigeeV1ApiProxy>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListApiProxiesResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps list organizations](OrganizationAppListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListAppsResponse {
    /// no description provided
    
    pub app: Option<Vec<GoogleCloudApigeeV1App>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListAppsResponse {}


/// Response for ListArchiveDeployments method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments archive deployments list organizations](OrganizationEnvironmentArchiveDeploymentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListArchiveDeploymentsResponse {
    /// Archive Deployments in the specified environment.
    #[serde(rename="archiveDeployments")]
    
    pub archive_deployments: Option<Vec<GoogleCloudApigeeV1ArchiveDeployment>>,
    /// Page token that you can include in a ListArchiveDeployments request to retrieve the next page. If omitted, no subsequent pages exist.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListArchiveDeploymentsResponse {}


/// The response for ListAsyncQueries.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments queries list organizations](OrganizationEnvironmentQueryListCall) (response)
/// * [host queries list organizations](OrganizationHostQueryListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListAsyncQueriesResponse {
    /// The asynchronous queries belong to requested resource name.
    
    pub queries: Option<Vec<GoogleCloudApigeeV1AsyncQuery>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListAsyncQueriesResponse {}


/// This message encapsulates a list of custom report definitions
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports list organizations](OrganizationReportListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListCustomReportsResponse {
    /// no description provided
    
    pub qualifier: Option<Vec<GoogleCloudApigeeV1CustomReport>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListCustomReportsResponse {}


/// Response for ListDataCollectors.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [datacollectors list organizations](OrganizationDatacollectorListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListDataCollectorsResponse {
    /// Data collectors in the specified organization.
    #[serde(rename="dataCollectors")]
    
    pub data_collectors: Option<Vec<GoogleCloudApigeeV1DataCollector>>,
    /// Page token that you can include in a ListDataCollectors request to retrieve the next page. If omitted, no subsequent pages exist.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListDataCollectorsResponse {}


/// The response for ListDatastores
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [analytics datastores list organizations](OrganizationAnalyticDatastoreListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListDatastoresResponse {
    /// A list of datastores
    
    pub datastores: Option<Vec<GoogleCloudApigeeV1Datastore>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListDatastoresResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments apis revisions debugsessions list organizations](OrganizationEnvironmentApiRevisionDebugsessionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListDebugSessionsResponse {
    /// Page token that you can include in a ListDebugSessionsRequest to retrieve the next page. If omitted, no subsequent pages exist.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Session info that includes debug session ID and the first transaction creation timestamp.
    
    pub sessions: Option<Vec<GoogleCloudApigeeV1Session>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListDebugSessionsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apis deployments list organizations](OrganizationApiDeploymentListCall) (response)
/// * [apis revisions deployments list organizations](OrganizationApiRevisionDeploymentListCall) (response)
/// * [deployments list organizations](OrganizationDeploymentListCall) (response)
/// * [environments apis deployments list organizations](OrganizationEnvironmentApiDeploymentListCall) (response)
/// * [environments deployments list organizations](OrganizationEnvironmentDeploymentListCall) (response)
/// * [environments sharedflows deployments list organizations](OrganizationEnvironmentSharedflowDeploymentListCall) (response)
/// * [sharedflows deployments list organizations](OrganizationSharedflowDeploymentListCall) (response)
/// * [sharedflows revisions deployments list organizations](OrganizationSharedflowRevisionDeploymentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListDeploymentsResponse {
    /// List of deployments.
    
    pub deployments: Option<Vec<GoogleCloudApigeeV1Deployment>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListDeploymentsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [developers apps list organizations](OrganizationDeveloperAppListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListDeveloperAppsResponse {
    /// List of developer apps and their credentials.
    
    pub app: Option<Vec<GoogleCloudApigeeV1DeveloperApp>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListDeveloperAppsResponse {}


/// Response for ListDeveloperSubscriptions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [developers subscriptions list organizations](OrganizationDeveloperSubscriptionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListDeveloperSubscriptionsResponse {
    /// List of all subscriptions.
    #[serde(rename="developerSubscriptions")]
    
    pub developer_subscriptions: Option<Vec<GoogleCloudApigeeV1DeveloperSubscription>>,
    /// Value that can be sent as `startKey` to retrieve the next page of content. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextStartKey")]
    
    pub next_start_key: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListDeveloperSubscriptionsResponse {}


/// Response for ListEndpointAttachments method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [endpoint attachments list organizations](OrganizationEndpointAttachmentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListEndpointAttachmentsResponse {
    /// Endpoint attachments in the specified organization.
    #[serde(rename="endpointAttachments")]
    
    pub endpoint_attachments: Option<Vec<GoogleCloudApigeeV1EndpointAttachment>>,
    /// Page token that you can include in an `ListEndpointAttachments` request to retrieve the next page. If omitted, no subsequent pages exist.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListEndpointAttachmentsResponse {}


/// Response for ListEnvironmentGroupAttachments.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [envgroups attachments list organizations](OrganizationEnvgroupAttachmentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListEnvironmentGroupAttachmentsResponse {
    /// EnvironmentGroupAttachments for the specified environment group.
    #[serde(rename="environmentGroupAttachments")]
    
    pub environment_group_attachments: Option<Vec<GoogleCloudApigeeV1EnvironmentGroupAttachment>>,
    /// Page token that you can include in a ListEnvironmentGroupAttachments request to retrieve the next page. If omitted, no subsequent pages exist.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListEnvironmentGroupAttachmentsResponse {}


/// Response for ListEnvironmentGroups.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [envgroups list organizations](OrganizationEnvgroupListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListEnvironmentGroupsResponse {
    /// EnvironmentGroups in the specified organization.
    #[serde(rename="environmentGroups")]
    
    pub environment_groups: Option<Vec<GoogleCloudApigeeV1EnvironmentGroup>>,
    /// Page token that you can include in a ListEnvironmentGroups request to retrieve the next page. If omitted, no subsequent pages exist.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListEnvironmentGroupsResponse {}


/// Response for ListEnvironmentResources
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments resourcefiles list organizations](OrganizationEnvironmentResourcefileListCall) (response)
/// * [environments resourcefiles list environment resources organizations](OrganizationEnvironmentResourcefileListEnvironmentResourceCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListEnvironmentResourcesResponse {
    /// List of resources files.
    #[serde(rename="resourceFile")]
    
    pub resource_file: Option<Vec<GoogleCloudApigeeV1ResourceFile>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListEnvironmentResourcesResponse {}


/// The response for ListExports
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments analytics exports list organizations](OrganizationEnvironmentAnalyticExportListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListExportsResponse {
    /// Details of the export jobs.
    
    pub exports: Option<Vec<GoogleCloudApigeeV1Export>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListExportsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [issuers list hybrid](HybridIssuerListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListHybridIssuersResponse {
    /// Lists of hybrid services and its trusted issuer email ids.
    
    pub issuers: Option<Vec<GoogleCloudApigeeV1ServiceIssuersMapping>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListHybridIssuersResponse {}


/// Response for ListInstanceAttachments.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances attachments list organizations](OrganizationInstanceAttachmentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListInstanceAttachmentsResponse {
    /// Attachments for the instance.
    
    pub attachments: Option<Vec<GoogleCloudApigeeV1InstanceAttachment>>,
    /// Page token that you can include in a ListInstanceAttachments request to retrieve the next page of content. If omitted, no subsequent pages exist.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListInstanceAttachmentsResponse {}


/// Response for ListInstances.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances list organizations](OrganizationInstanceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListInstancesResponse {
    /// Instances in the specified organization.
    
    pub instances: Option<Vec<GoogleCloudApigeeV1Instance>>,
    /// Page token that you can include in a ListInstance request to retrieve the next page of content. If omitted, no subsequent pages exist.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListInstancesResponse {}


/// The request structure for listing key value map keys and its corresponding values.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apis keyvaluemaps entries list organizations](OrganizationApiKeyvaluemapEntryListCall) (response)
/// * [environments keyvaluemaps entries list organizations](OrganizationEnvironmentKeyvaluemapEntryListCall) (response)
/// * [keyvaluemaps entries list organizations](OrganizationKeyvaluemapEntryListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListKeyValueEntriesResponse {
    /// One or more key value map keys and values.
    #[serde(rename="keyValueEntries")]
    
    pub key_value_entries: Option<Vec<GoogleCloudApigeeV1KeyValueEntry>>,
    /// Token that can be sent as `next_page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListKeyValueEntriesResponse {}


/// Response for ListNatAddresses.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances nat addresses list organizations](OrganizationInstanceNatAddressListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListNatAddressesResponse {
    /// List of NAT Addresses for the instance.
    #[serde(rename="natAddresses")]
    
    pub nat_addresses: Option<Vec<GoogleCloudApigeeV1NatAddress>>,
    /// Page token that you can include in a ListNatAddresses request to retrieve the next page of content. If omitted, no subsequent pages exist.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListNatAddressesResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [developers list organizations](OrganizationDeveloperListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListOfDevelopersResponse {
    /// List of developers.
    
    pub developer: Option<Vec<GoogleCloudApigeeV1Developer>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListOfDevelopersResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list organizations](OrganizationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListOrganizationsResponse {
    /// List of Apigee organizations and associated Google Cloud projects.
    
    pub organizations: Option<Vec<GoogleCloudApigeeV1OrganizationProjectMapping>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListOrganizationsResponse {}


/// Response for ListRatePlans.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apiproducts rateplans list organizations](OrganizationApiproductRateplanListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListRatePlansResponse {
    /// Value that can be sent as `startKey` to retrieve the next page of content. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextStartKey")]
    
    pub next_start_key: Option<String>,
    /// List of rate plans in an organization.
    #[serde(rename="ratePlans")]
    
    pub rate_plans: Option<Vec<GoogleCloudApigeeV1RatePlan>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListRatePlansResponse {}


/// Response for ListSecurityProfileRevisions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [security profiles list revisions organizations](OrganizationSecurityProfileListRevisionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListSecurityProfileRevisionsResponse {
    /// A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of security profile revisions. The revisions may be attached or unattached to any environment.
    #[serde(rename="securityProfiles")]
    
    pub security_profiles: Option<Vec<GoogleCloudApigeeV1SecurityProfile>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListSecurityProfileRevisionsResponse {}


/// Response for ListSecurityProfiles.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [security profiles list organizations](OrganizationSecurityProfileListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListSecurityProfilesResponse {
    /// A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of security profiles in the organization. The profiles may be attached or unattached to any environment. This will return latest revision of each profile.
    #[serde(rename="securityProfiles")]
    
    pub security_profiles: Option<Vec<GoogleCloudApigeeV1SecurityProfile>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListSecurityProfilesResponse {}


/// The response for SecurityReports.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments security reports list organizations](OrganizationEnvironmentSecurityReportListCall) (response)
/// * [host security reports list organizations](OrganizationHostSecurityReportListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListSecurityReportsResponse {
    /// If the number of security reports exceeded the page size requested, the token can be used to fetch the next page in a subsequent call. If the response is the last page and there are no more reports to return this field is left empty.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The security reports belong to requested resource name.
    #[serde(rename="securityReports")]
    
    pub security_reports: Option<Vec<GoogleCloudApigeeV1SecurityReport>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListSecurityReportsResponse {}


/// To change this message, in the same CL add a change log in go/changing-api-proto-breaks-ui
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sharedflows list organizations](OrganizationSharedflowListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListSharedFlowsResponse {
    /// no description provided
    #[serde(rename="sharedFlows")]
    
    pub shared_flows: Option<Vec<GoogleCloudApigeeV1SharedFlow>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListSharedFlowsResponse {}


/// Response for ListTraceConfigOverrides.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments trace config overrides list organizations](OrganizationEnvironmentTraceConfigOverrideListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ListTraceConfigOverridesResponse {
    /// Token value that can be passed as `page_token` to retrieve the next page of content.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List all trace configuration overrides in an environment.
    #[serde(rename="traceConfigOverrides")]
    
    pub trace_config_overrides: Option<Vec<GoogleCloudApigeeV1TraceConfigOverride>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ListTraceConfigOverridesResponse {}


/// Encapsulates additional information about query execution.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Metadata {
    /// List of error messages as strings.
    
    pub errors: Option<Vec<String>>,
    /// List of additional information such as data source, if result was truncated. For example: ``` "notices": [ "Source:Postgres", "PG Host:uappg0rw.e2e.apigeeks.net", "query served by:4b64601e-40de-4eb1-bfb9-eeee7ac929ed", "Table used: edge.api.uapgroup2.agg_api" ]```
    
    pub notices: Option<Vec<String>>,
}

impl client::Part for GoogleCloudApigeeV1Metadata {}


/// Encapsulates the metric data point. For example: ```{ "name": "sum(message_count)", "values" : [ { "timestamp": 1549004400000, "value": "39.0" }, { "timestamp" : 1548997200000, "value" : "0.0" } ] }``` or ```{ "name": "sum(message_count)", "values" : ["39.0"] }```
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Metric {
    /// Metric name.
    
    pub name: Option<String>,
    /// List of metric values. Possible value formats include: `"values":["39.0"]` or `"values":[ { "value": "39.0", "timestamp": 1232434354} ]`
    
    pub values: Option<Vec<json::Value>>,
}

impl client::Part for GoogleCloudApigeeV1Metric {}


/// The optionally aggregated metric to query with its ordering.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1MetricAggregation {
    /// Aggregation function associated with the metric.
    
    pub aggregation: Option<String>,
    /// Name of the metric
    
    pub name: Option<String>,
    /// Ordering for this aggregation in the result. For time series this is ignored since the ordering of points depends only on the timestamp, not the values.
    
    pub order: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1MetricAggregation {}


/// Configuration for the Monetization add-on.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1MonetizationConfig {
    /// Flag that specifies whether the Monetization add-on is enabled.
    
    pub enabled: Option<bool>,
}

impl client::Part for GoogleCloudApigeeV1MonetizationConfig {}


/// Apigee NAT(network address translation) address. A NAT address is a static external IP address used for Internet egress traffic.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances nat addresses create organizations](OrganizationInstanceNatAddressCreateCall) (request)
/// * [instances nat addresses get organizations](OrganizationInstanceNatAddressGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1NatAddress {
    /// Output only. The static IPV4 address.
    #[serde(rename="ipAddress")]
    
    pub ip_address: Option<String>,
    /// Required. Resource ID of the NAT address.
    
    pub name: Option<String>,
    /// Output only. State of the nat address.
    
    pub state: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1NatAddress {}
impl client::ResponseResult for GoogleCloudApigeeV1NatAddress {}


/// NodeConfig for setting the min/max number of nodes associated with the environment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1NodeConfig {
    /// Output only. The current total number of gateway nodes that each environment currently has across all instances.
    #[serde(rename="currentAggregateNodeCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub current_aggregate_node_count: Option<i64>,
    /// Optional. The maximum total number of gateway nodes that the is reserved for all instances that has the specified environment. If not specified, the default is determined by the recommended maximum number of nodes for that gateway.
    #[serde(rename="maxNodeCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_node_count: Option<i64>,
    /// Optional. The minimum total number of gateway nodes that the is reserved for all instances that has the specified environment. If not specified, the default is determined by the recommended minimum number of nodes for that gateway.
    #[serde(rename="minNodeCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub min_node_count: Option<i64>,
}

impl client::Part for GoogleCloudApigeeV1NodeConfig {}


/// Represents the pairing of REST resource path and the actions (verbs) allowed on the resource path.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Operation {
    /// methods refers to the REST verbs as in https://www.w3.org/Protocols/rfc2616/rfc2616-sec9.html. When none specified, all verb types are allowed.
    
    pub methods: Option<Vec<String>>,
    /// Required. REST resource path associated with the API proxy or remote service.
    
    pub resource: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1Operation {}


/// Binds the resources in an API proxy or remote service with the allowed REST methods and associated quota enforcement.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1OperationConfig {
    /// Required. Name of the API proxy or remote service with which the resources, methods, and quota are associated.
    #[serde(rename="apiSource")]
    
    pub api_source: Option<String>,
    /// Custom attributes associated with the operation.
    
    pub attributes: Option<Vec<GoogleCloudApigeeV1Attribute>>,
    /// List of resource/method pairs for the API proxy or remote service to which quota will applied. **Note**: Currently, you can specify only a single resource/method pair. The call will fail if more than one resource/method pair is provided.
    
    pub operations: Option<Vec<GoogleCloudApigeeV1Operation>>,
    /// Quota parameters to be enforced for the resources, methods, and API source combination. If none are specified, quota enforcement will not be done.
    
    pub quota: Option<GoogleCloudApigeeV1Quota>,
}

impl client::Part for GoogleCloudApigeeV1OperationConfig {}


/// List of operation configuration details associated with Apigee API proxies or remote services. Remote services are non-Apigee proxies, such as Istio-Envoy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1OperationGroup {
    /// Flag that specifes whether the configuration is for Apigee API proxy or a remote service. Valid values include `proxy` or `remoteservice`. Defaults to `proxy`. Set to `proxy` when Apigee API proxies are associated with the API product. Set to `remoteservice` when non-Apigee proxies like Istio-Envoy are associated with the API product.
    #[serde(rename="operationConfigType")]
    
    pub operation_config_type: Option<String>,
    /// Required. List of operation configurations for either Apigee API proxies or other remote services that are associated with this API product.
    #[serde(rename="operationConfigs")]
    
    pub operation_configs: Option<Vec<GoogleCloudApigeeV1OperationConfig>>,
}

impl client::Part for GoogleCloudApigeeV1OperationGroup {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments optimized stats get organizations](OrganizationEnvironmentOptimizedStatGetCall) (response)
/// * [optimized host stats get organizations](OrganizationOptimizedHostStatGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1OptimizedStats {
    /// Wraps the `stats` response for JavaScript Optimized Scenario with a response key. For example: ```{ "Response": { "TimeUnit": [], "metaData": { "errors": [], "notices": [ "Source:Postgres", "Table used: edge.api.aaxgroup001.agg_api", "PG Host:ruappg08-ro.production.apigeeks.net", "query served by:80c4ebca-6a10-4a2e-8faf-c60c1ee306ca" ] }, "resultTruncated": false, "stats": { "data": [ { "identifier": { "names": [ "apiproxy" ], "values": [ "sirjee" ] }, "metric": [ { "env": "prod", "name": "sum(message_count)", "values": [ 36.0 ] }, { "env": "prod", "name": "sum(is_error)", "values": [ 36.0 ] } ] } ] } } }```
    #[serde(rename="Response")]
    
    pub response: Option<GoogleCloudApigeeV1OptimizedStatsResponse>,
}

impl client::ResponseResult for GoogleCloudApigeeV1OptimizedStats {}


/// Encapsulates a data node as represented below: ``` { "identifier": { "names": [ "apiproxy" ], "values": [ "sirjee" ] }, "metric": [ { "env": "prod", "name": "sum(message_count)", "values": [ 36.0 ] } ] }``` or ``` { "env": "prod", "name": "sum(message_count)", "values": [ 36.0 ] }``` Depending on whether a dimension is present in the query or not the data node type can be a simple metric value or dimension identifier with list of metrics.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1OptimizedStatsNode {
    /// no description provided
    
    pub data: Option<Vec<json::Value>>,
}

impl client::Part for GoogleCloudApigeeV1OptimizedStatsNode {}


/// Encapsulates a response format for JavaScript Optimized Scenario.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1OptimizedStatsResponse {
    /// List of time unit values. Time unit refers to an epoch timestamp value.
    #[serde(rename="TimeUnit")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub time_unit: Option<Vec<i64>>,
    /// Metadata information about the query executed.
    #[serde(rename="metaData")]
    
    pub meta_data: Option<GoogleCloudApigeeV1Metadata>,
    /// Boolean flag that indicates whether the results were truncated based on the limit parameter.
    #[serde(rename="resultTruncated")]
    
    pub result_truncated: Option<bool>,
    /// `stats` results.
    
    pub stats: Option<GoogleCloudApigeeV1OptimizedStatsNode>,
}

impl client::Part for GoogleCloudApigeeV1OptimizedStatsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create organizations](OrganizationCreateCall) (request)
/// * [get organizations](OrganizationGetCall) (response)
/// * [update organizations](OrganizationUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Organization {
    /// Addon configurations of the Apigee organization.
    #[serde(rename="addonsConfig")]
    
    pub addons_config: Option<GoogleCloudApigeeV1AddonsConfig>,
    /// Required. DEPRECATED: This field will be deprecated once Apigee supports DRZ. Primary Google Cloud region for analytics data storage. For valid values, see [Create an Apigee organization](https://cloud.google.com/apigee/docs/api-platform/get-started/create-org).
    #[serde(rename="analyticsRegion")]
    
    pub analytics_region: Option<String>,
    /// Output only. Apigee Project ID associated with the organization. Use this project to allowlist Apigee in the Service Attachment when using private service connect with Apigee.
    #[serde(rename="apigeeProjectId")]
    
    pub apigee_project_id: Option<String>,
    /// Not used by Apigee.
    
    pub attributes: Option<Vec<String>>,
    /// Compute Engine network used for Service Networking to be peered with Apigee runtime instances. See [Getting started with the Service Networking API](https://cloud.google.com/service-infrastructure/docs/service-networking/getting-started). Valid only when [RuntimeType](#RuntimeType) is set to `CLOUD`. The value must be set before the creation of a runtime instance and can be updated only when there are no runtime instances. For example: `default`. Apigee also supports shared VPC (that is, the host network project is not the same as the one that is peering with Apigee). See [Shared VPC overview](https://cloud.google.com/vpc/docs/shared-vpc). To use a shared VPC network, use the following format: `projects/{host-project-id}/{region}/networks/{network-name}`. For example: `projects/my-sharedvpc-host/global/networks/mynetwork` **Note:** Not supported for Apigee hybrid.
    #[serde(rename="authorizedNetwork")]
    
    pub authorized_network: Option<String>,
    /// Billing type of the Apigee organization. See [Apigee pricing](https://cloud.google.com/apigee/pricing).
    #[serde(rename="billingType")]
    
    pub billing_type: Option<String>,
    /// Output only. Base64-encoded public certificate for the root CA of the Apigee organization. Valid only when [RuntimeType](#RuntimeType) is `CLOUD`.
    #[serde(rename="caCertificate")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub ca_certificate: Option<Vec<u8>>,
    /// Output only. Time that the Apigee organization was created in milliseconds since epoch.
    #[serde(rename="createdAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub created_at: Option<i64>,
    /// Not used by Apigee.
    #[serde(rename="customerName")]
    
    pub customer_name: Option<String>,
    /// Description of the Apigee organization.
    
    pub description: Option<String>,
    /// Display name for the Apigee organization. Unused, but reserved for future use.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. List of environments in the Apigee organization.
    
    pub environments: Option<Vec<String>>,
    /// Output only. Time that the Apigee organization is scheduled for deletion.
    #[serde(rename="expiresAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expires_at: Option<i64>,
    /// Output only. Time that the Apigee organization was last modified in milliseconds since epoch.
    #[serde(rename="lastModifiedAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_modified_at: Option<i64>,
    /// Output only. Name of the Apigee organization.
    
    pub name: Option<String>,
    /// Configuration for the Portals settings.
    #[serde(rename="portalDisabled")]
    
    pub portal_disabled: Option<bool>,
    /// Output only. Project ID associated with the Apigee organization.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Properties defined in the Apigee organization profile.
    
    pub properties: Option<GoogleCloudApigeeV1Properties>,
    /// Cloud KMS key name used for encrypting the data that is stored and replicated across runtime instances. Update is not allowed after the organization is created. Required when [RuntimeType](#RuntimeType) is `CLOUD`. If not specified when [RuntimeType](#RuntimeType) is `TRIAL`, a Google-Managed encryption key will be used. For example: “projects/foo/locations/us/keyRings/bar/cryptoKeys/baz”. **Note:** Not supported for Apigee hybrid.
    #[serde(rename="runtimeDatabaseEncryptionKeyName")]
    
    pub runtime_database_encryption_key_name: Option<String>,
    /// Required. Runtime type of the Apigee organization based on the Apigee subscription purchased.
    #[serde(rename="runtimeType")]
    
    pub runtime_type: Option<String>,
    /// Output only. State of the organization. Values other than ACTIVE means the resource is not ready to use.
    
    pub state: Option<String>,
    /// Output only. DEPRECATED: This will eventually be replaced by BillingType. Subscription type of the Apigee organization. Valid values include trial (free, limited, and for evaluation purposes only) or paid (full subscription has been purchased). See [Apigee pricing](https://cloud.google.com/apigee/pricing/).
    #[serde(rename="subscriptionType")]
    
    pub subscription_type: Option<String>,
    /// Not used by Apigee.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1Organization {}
impl client::ResponseResult for GoogleCloudApigeeV1Organization {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get project mapping organizations](OrganizationGetProjectMappingCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1OrganizationProjectMapping {
    /// Output only. The Google Cloud region where control plane data is located. For more information, see https://cloud.google.com/about/locations/.
    
    pub location: Option<String>,
    /// Name of the Apigee organization.
    
    pub organization: Option<String>,
    /// Google Cloud project associated with the Apigee organization
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// DEPRECATED: Use `project_id`. An Apigee Organization is mapped to a single project.
    #[serde(rename="projectIds")]
    
    pub project_ids: Option<Vec<String>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1OrganizationProjectMapping {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1PodStatus {
    /// Version of the application running in the pod.
    #[serde(rename="appVersion")]
    
    pub app_version: Option<String>,
    /// Status of the deployment. Valid values include: - `deployed`: Successful. - `error` : Failed. - `pending` : Pod has not yet reported on the deployment.
    #[serde(rename="deploymentStatus")]
    
    pub deployment_status: Option<String>,
    /// Time the deployment status was reported in milliseconds since epoch.
    #[serde(rename="deploymentStatusTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub deployment_status_time: Option<i64>,
    /// Time the proxy was deployed in milliseconds since epoch.
    #[serde(rename="deploymentTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub deployment_time: Option<i64>,
    /// Name of the pod which is reporting the status.
    #[serde(rename="podName")]
    
    pub pod_name: Option<String>,
    /// Overall status of the pod (not this specific deployment). Valid values include: - `active`: Up to date. - `stale` : Recently out of date. Pods that have not reported status in a long time are excluded from the output.
    #[serde(rename="podStatus")]
    
    pub pod_status: Option<String>,
    /// Time the pod status was reported in milliseconds since epoch.
    #[serde(rename="podStatusTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub pod_status_time: Option<i64>,
    /// Code associated with the deployment status.
    #[serde(rename="statusCode")]
    
    pub status_code: Option<String>,
    /// Human-readable message associated with the status code.
    #[serde(rename="statusCodeDetails")]
    
    pub status_code_details: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1PodStatus {}


/// Point is a group of information collected by runtime plane at critical points of the message flow of the processed API request. This is a list of supported point IDs, categorized to three major buckets. For each category, debug points that we are currently supporting are listed below: - Flow status debug points: StateChange FlowInfo Condition Execution DebugMask Error - Flow control debug points: FlowCallout Paused Resumed FlowReturn BreakFlow Error - Runtime debug points: ScriptExecutor FlowCalloutStepDefinition CustomTarget StepDefinition Oauth2ServicePoint RaiseFault NodeJS The detail information of the given debug point is stored in a list of results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Point {
    /// Name of a step in the transaction.
    
    pub id: Option<String>,
    /// List of results extracted from a given debug point.
    
    pub results: Option<Vec<GoogleCloudApigeeV1Result>>,
}

impl client::Part for GoogleCloudApigeeV1Point {}


/// Message for compatibility with legacy Edge specification for Java Properties object in JSON.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Properties {
    /// List of all properties in the object
    
    pub property: Option<Vec<GoogleCloudApigeeV1Property>>,
}

impl client::Part for GoogleCloudApigeeV1Properties {}


/// A single property entry in the Properties message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Property {
    /// The property key
    
    pub name: Option<String>,
    /// The property value
    
    pub value: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1Property {}


/// Request for ProvisionOrganization.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [provision organization projects](ProjectProvisionOrganizationCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ProvisionOrganizationRequest {
    /// Primary Cloud Platform region for analytics data storage. For valid values, see [Create an organization](https://cloud.google.com/apigee/docs/hybrid/latest/precog-provision). Defaults to `us-west1`.
    #[serde(rename="analyticsRegion")]
    
    pub analytics_region: Option<String>,
    /// Name of the customer project's VPC network. If provided, the network needs to be peered through Service Networking. If none is provided, the organization will have access only to the public internet.
    #[serde(rename="authorizedNetwork")]
    
    pub authorized_network: Option<String>,
    /// Cloud Platform location for the runtime instance. Defaults to zone `us-west1-a`. If a region is provided, `EVAL` organizations will use the region for automatically selecting a zone for the runtime instance.
    #[serde(rename="runtimeLocation")]
    
    pub runtime_location: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1ProvisionOrganizationRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments queries create organizations](OrganizationEnvironmentQueryCreateCall) (request)
/// * [host queries create organizations](OrganizationHostQueryCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Query {
    /// Delimiter used in the CSV file, if `outputFormat` is set to `csv`. Defaults to the `,` (comma) character. Supported delimiter characters include comma (`,`), pipe (`|`), and tab (`\t`).
    #[serde(rename="csvDelimiter")]
    
    pub csv_delimiter: Option<String>,
    /// A list of dimensions. https://docs.apigee.com/api-platform/analytics/analytics-reference#dimensions
    
    pub dimensions: Option<Vec<String>>,
    /// Hostname needs to be specified if query intends to run at host level. This field is only allowed when query is submitted by CreateHostAsyncQuery where analytics data will be grouped by organization and hostname.
    #[serde(rename="envgroupHostname")]
    
    pub envgroup_hostname: Option<String>,
    /// Boolean expression that can be used to filter data. Filter expressions can be combined using AND/OR terms and should be fully parenthesized to avoid ambiguity. See Analytics metrics, dimensions, and filters reference https://docs.apigee.com/api-platform/analytics/analytics-reference for more information on the fields available to filter on. For more information on the tokens that you use to build filter expressions, see Filter expression syntax. https://docs.apigee.com/api-platform/analytics/asynch-reports-api#filter-expression-syntax
    
    pub filter: Option<String>,
    /// Time unit used to group the result set. Valid values include: second, minute, hour, day, week, or month. If a query includes groupByTimeUnit, then the result is an aggregation based on the specified time unit and the resultant timestamp does not include milliseconds precision. If a query omits groupByTimeUnit, then the resultant timestamp includes milliseconds precision.
    #[serde(rename="groupByTimeUnit")]
    
    pub group_by_time_unit: Option<String>,
    /// Maximum number of rows that can be returned in the result.
    
    pub limit: Option<i32>,
    /// A list of Metrics.
    
    pub metrics: Option<Vec<GoogleCloudApigeeV1QueryMetric>>,
    /// Asynchronous Query Name.
    
    pub name: Option<String>,
    /// Valid values include: `csv` or `json`. Defaults to `json`. Note: Configure the delimiter for CSV output using the csvDelimiter property.
    #[serde(rename="outputFormat")]
    
    pub output_format: Option<String>,
    /// Asynchronous Report ID.
    #[serde(rename="reportDefinitionId")]
    
    pub report_definition_id: Option<String>,
    /// Required. Time range for the query. Can use the following predefined strings to specify the time range: `last60minutes` `last24hours` `last7days` Or, specify the timeRange as a structure describing start and end timestamps in the ISO format: yyyy-mm-ddThh:mm:ssZ. Example: "timeRange": { "start": "2018-07-29T00:13:00Z", "end": "2018-08-01T00:18:00Z" }
    #[serde(rename="timeRange")]
    
    pub time_range: Option<json::Value>,
}

impl client::RequestValue for GoogleCloudApigeeV1Query {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1QueryMetadata {
    /// Dimensions of the AsyncQuery.
    
    pub dimensions: Option<Vec<String>>,
    /// End timestamp of the query range.
    #[serde(rename="endTimestamp")]
    
    pub end_timestamp: Option<String>,
    /// Metrics of the AsyncQuery. Example: ["name:message_count,func:sum,alias:sum_message_count"]
    
    pub metrics: Option<Vec<String>>,
    /// Output format.
    #[serde(rename="outputFormat")]
    
    pub output_format: Option<String>,
    /// Start timestamp of the query range.
    #[serde(rename="startTimestamp")]
    
    pub start_timestamp: Option<String>,
    /// Query GroupBy time unit.
    #[serde(rename="timeUnit")]
    
    pub time_unit: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1QueryMetadata {}


/// More info about Metric: https://docs.apigee.com/api-platform/analytics/analytics-reference#metrics
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1QueryMetric {
    /// Alias for the metric. Alias will be used to replace metric name in query results.
    
    pub alias: Option<String>,
    /// Aggregation function: avg, min, max, or sum.
    
    pub function: Option<String>,
    /// Required. Metric name.
    
    pub name: Option<String>,
    /// One of `+`, `-`, `/`, `%`, `*`.
    
    pub operator: Option<String>,
    /// Operand value should be provided when operator is set.
    
    pub value: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1QueryMetric {}


/// Request payload representing the query to be run for fetching security statistics as rows.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments security stats query tabular stats organizations](OrganizationEnvironmentSecurityStatQueryTabularStatCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1QueryTabularStatsRequest {
    /// Required. List of dimension names to group the aggregations by.
    
    pub dimensions: Option<Vec<String>>,
    /// Filter further on specific dimension values. Follows the same grammar as custom report's filter expressions. Example, apiproxy eq 'foobar'. https://cloud.google.com/apigee/docs/api-platform/analytics/analytics-reference#filters
    
    pub filter: Option<String>,
    /// Required. List of metrics and their aggregations.
    
    pub metrics: Option<Vec<GoogleCloudApigeeV1MetricAggregation>>,
    /// Page size represents the number of rows.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// Identifies a sequence of rows.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
    /// Time range for the stats.
    #[serde(rename="timeRange")]
    
    pub time_range: Option<GoogleTypeInterval>,
}

impl client::RequestValue for GoogleCloudApigeeV1QueryTabularStatsRequest {}


/// Encapsulates two kinds of stats that are results of the dimensions and aggregations requested. - Tabular rows. - Time series data. Example of tabular rows, Represents security stats results as a row of flat values.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments security stats query tabular stats organizations](OrganizationEnvironmentSecurityStatQueryTabularStatCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1QueryTabularStatsResponse {
    /// Column names corresponding to the same order as the inner values in the stats field.
    
    pub columns: Option<Vec<String>>,
    /// Next page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Resultant rows from the executed query.
    
    pub values: Option<Vec<Vec<json::Value>>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1QueryTabularStatsResponse {}


/// QueryTimeSeriesStatsRequest represents a query that returns a collection of time series sequences grouped by their values.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments security stats query time series stats organizations](OrganizationEnvironmentSecurityStatQueryTimeSeriesStatCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1QueryTimeSeriesStatsRequest {
    /// List of dimension names to group the aggregations by. If no dimensions are passed, a single trend line representing the requested metric aggregations grouped by environment is returned.
    
    pub dimensions: Option<Vec<String>>,
    /// Filter further on specific dimension values. Follows the same grammar as custom report's filter expressions. Example, apiproxy eq 'foobar'. https://cloud.google.com/apigee/docs/api-platform/analytics/analytics-reference#filters
    
    pub filter: Option<String>,
    /// Required. List of metrics and their aggregations.
    
    pub metrics: Option<Vec<GoogleCloudApigeeV1MetricAggregation>>,
    /// Page size represents the number of time series sequences, one per unique set of dimensions and their values.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// Page token stands for a specific collection of time series sequences.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
    /// Required. Time range for the stats.
    #[serde(rename="timeRange")]
    
    pub time_range: Option<GoogleTypeInterval>,
    /// Order the sequences in increasing or decreasing order of timestamps. Default is descending order of timestamps (latest first).
    #[serde(rename="timestampOrder")]
    
    pub timestamp_order: Option<String>,
    /// Time buckets to group the stats by.
    #[serde(rename="windowSize")]
    
    pub window_size: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1QueryTimeSeriesStatsRequest {}


/// Represents security stats result as a collection of time series sequences.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments security stats query time series stats organizations](OrganizationEnvironmentSecurityStatQueryTimeSeriesStatCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1QueryTimeSeriesStatsResponse {
    /// Column names corresponding to the same order as the inner values in the stats field.
    
    pub columns: Option<Vec<String>>,
    /// Next page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Results of the query returned as a JSON array.
    
    pub values: Option<Vec<GoogleCloudApigeeV1QueryTimeSeriesStatsResponseSequence>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1QueryTimeSeriesStatsResponse {}


/// A sequence of time series.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1QueryTimeSeriesStatsResponseSequence {
    /// Map of dimensions and their values that uniquely identifies a time series sequence.
    
    pub dimensions: Option<HashMap<String, String>>,
    /// List of points. First value of each inner list is a timestamp.
    
    pub points: Option<Vec<Vec<json::Value>>>,
}

impl client::Part for GoogleCloudApigeeV1QueryTimeSeriesStatsResponseSequence {}


/// Quota contains the essential parameters needed that can be applied on the resources, methods, API source combination associated with this API product. While Quota is optional, setting it prevents requests from exceeding the provisioned parameters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Quota {
    /// Required. Time interval over which the number of request messages is calculated.
    
    pub interval: Option<String>,
    /// Required. Upper limit allowed for the time interval and time unit specified. Requests exceeding this limit will be rejected.
    
    pub limit: Option<String>,
    /// Time unit defined for the `interval`. Valid values include `minute`, `hour`, `day`, or `month`. If `limit` and `interval` are valid, the default value is `hour`; otherwise, the default is null.
    #[serde(rename="timeUnit")]
    
    pub time_unit: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1Quota {}


/// Rate plan details.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apiproducts rateplans create organizations](OrganizationApiproductRateplanCreateCall) (request|response)
/// * [apiproducts rateplans delete organizations](OrganizationApiproductRateplanDeleteCall) (response)
/// * [apiproducts rateplans get organizations](OrganizationApiproductRateplanGetCall) (response)
/// * [apiproducts rateplans update organizations](OrganizationApiproductRateplanUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1RatePlan {
    /// Name of the API product that the rate plan is associated with.
    
    pub apiproduct: Option<String>,
    /// Frequency at which the customer will be billed.
    #[serde(rename="billingPeriod")]
    
    pub billing_period: Option<String>,
    /// API call volume ranges and the fees charged when the total number of API calls is within a given range. The method used to calculate the final fee depends on the selected pricing model. For example, if the pricing model is `STAIRSTEP` and the ranges are defined as follows: ``` { "start": 1, "end": 100, "fee": 75 }, { "start": 101, "end": 200, "fee": 100 }, } ``` Then the following fees would be charged based on the total number of API calls (assuming the currency selected is `USD`): * 1 call costs $75 * 50 calls cost $75 * 150 calls cost $100 The number of API calls cannot exceed 200.
    #[serde(rename="consumptionPricingRates")]
    
    pub consumption_pricing_rates: Option<Vec<GoogleCloudApigeeV1RateRange>>,
    /// Pricing model used for consumption-based charges.
    #[serde(rename="consumptionPricingType")]
    
    pub consumption_pricing_type: Option<String>,
    /// Output only. Time that the rate plan was created in milliseconds since epoch.
    #[serde(rename="createdAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub created_at: Option<i64>,
    /// Currency to be used for billing. Consists of a three-letter code as defined by the [ISO 4217](https://en.wikipedia.org/wiki/ISO_4217) standard.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Description of the rate plan.
    
    pub description: Option<String>,
    /// Display name of the rate plan.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Time when the rate plan will expire in milliseconds since epoch. Set to 0 or `null` to indicate that the rate plan should never expire.
    #[serde(rename="endTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub end_time: Option<i64>,
    /// Frequency at which the fixed fee is charged.
    #[serde(rename="fixedFeeFrequency")]
    
    pub fixed_fee_frequency: Option<i32>,
    /// Fixed amount that is charged at a defined interval and billed in advance of use of the API product. The fee will be prorated for the first billing period.
    #[serde(rename="fixedRecurringFee")]
    
    pub fixed_recurring_fee: Option<GoogleTypeMoney>,
    /// Output only. Time the rate plan was last modified in milliseconds since epoch.
    #[serde(rename="lastModifiedAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_modified_at: Option<i64>,
    /// Output only. Name of the rate plan.
    
    pub name: Option<String>,
    /// DEPRECATED: This field is no longer supported and will eventually be removed when Apigee Hybrid 1.5/1.6 is no longer supported. Instead, use the `billingType` field inside `DeveloperMonetizationConfig` resource. Flag that specifies the billing account type, prepaid or postpaid.
    #[serde(rename="paymentFundingModel")]
    
    pub payment_funding_model: Option<String>,
    /// Details of the revenue sharing model.
    #[serde(rename="revenueShareRates")]
    
    pub revenue_share_rates: Option<Vec<GoogleCloudApigeeV1RevenueShareRange>>,
    /// Method used to calculate the revenue that is shared with developers.
    #[serde(rename="revenueShareType")]
    
    pub revenue_share_type: Option<String>,
    /// Initial, one-time fee paid when purchasing the API product.
    #[serde(rename="setupFee")]
    
    pub setup_fee: Option<GoogleTypeMoney>,
    /// Time when the rate plan becomes active in milliseconds since epoch.
    #[serde(rename="startTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start_time: Option<i64>,
    /// Current state of the rate plan (draft or published).
    
    pub state: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1RatePlan {}
impl client::ResponseResult for GoogleCloudApigeeV1RatePlan {}


/// API call volume range and the fees charged when the total number of API calls is within the range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1RateRange {
    /// Ending value of the range. Set to 0 or `null` for the last range of values.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub end: Option<i64>,
    /// Fee to charge when total number of API calls falls within this range.
    
    pub fee: Option<GoogleTypeMoney>,
    /// Starting value of the range. Set to 0 or `null` for the initial range of values.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start: Option<i64>,
}

impl client::Part for GoogleCloudApigeeV1RateRange {}


/// A Reference configuration. References must refer to a keystore that also exists in the parent environment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments references create organizations](OrganizationEnvironmentReferenceCreateCall) (request|response)
/// * [environments references delete organizations](OrganizationEnvironmentReferenceDeleteCall) (response)
/// * [environments references get organizations](OrganizationEnvironmentReferenceGetCall) (response)
/// * [environments references update organizations](OrganizationEnvironmentReferenceUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Reference {
    /// Optional. A human-readable description of this reference.
    
    pub description: Option<String>,
    /// Required. The resource id of this reference. Values must match the regular expression [\w\s\-.]+.
    
    pub name: Option<String>,
    /// Required. The id of the resource to which this reference refers. Must be the id of a resource that exists in the parent environment and is of the given resource_type.
    
    pub refers: Option<String>,
    /// The type of resource referred to by this reference. Valid values are 'KeyStore' or 'TrustStore'.
    #[serde(rename="resourceType")]
    
    pub resource_type: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1Reference {}
impl client::ResponseResult for GoogleCloudApigeeV1Reference {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ReferenceConfig {
    /// Name of the reference in the following format: `organizations/{org}/environments/{env}/references/{reference}`
    
    pub name: Option<String>,
    /// Name of the referenced resource in the following format: `organizations/{org}/environments/{env}/keystores/{keystore}` Only references to keystore resources are supported.
    #[serde(rename="resourceName")]
    
    pub resource_name: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1ReferenceConfig {}


/// Request for ReportInstanceStatus.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances report status organizations](OrganizationInstanceReportStatuCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ReportInstanceStatusRequest {
    /// A unique ID for the instance which is guaranteed to be unique in case the user installs multiple hybrid runtimes with the same instance ID.
    #[serde(rename="instanceUid")]
    
    pub instance_uid: Option<String>,
    /// The time the report was generated in the runtime. Used to prevent an old status from overwriting a newer one. An instance should space out it's status reports so that clock skew does not play a factor.
    #[serde(rename="reportTime")]
    
    pub report_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Status for config resources
    
    pub resources: Option<Vec<GoogleCloudApigeeV1ResourceStatus>>,
}

impl client::RequestValue for GoogleCloudApigeeV1ReportInstanceStatusRequest {}


/// Placeholder for future enhancements to status reporting protocol
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances report status organizations](OrganizationInstanceReportStatuCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ReportInstanceStatusResponse { _never_set: Option<bool> }

impl client::ResponseResult for GoogleCloudApigeeV1ReportInstanceStatusResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ReportProperty {
    /// name of the property
    
    pub property: Option<String>,
    /// property values
    
    pub value: Option<Vec<GoogleCloudApigeeV1Attribute>>,
}

impl client::Part for GoogleCloudApigeeV1ReportProperty {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ResourceConfig {
    /// Location of the resource as a URI.
    
    pub location: Option<String>,
    /// Resource name in the following format: `organizations/{org}/environments/{env}/resourcefiles/{type}/{file}/revisions/{rev}` Only environment-scoped resource files are supported.
    
    pub name: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1ResourceConfig {}


/// Metadata about a resource file.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments resourcefiles create organizations](OrganizationEnvironmentResourcefileCreateCall) (response)
/// * [environments resourcefiles delete organizations](OrganizationEnvironmentResourcefileDeleteCall) (response)
/// * [environments resourcefiles update organizations](OrganizationEnvironmentResourcefileUpdateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ResourceFile {
    /// ID of the resource file.
    
    pub name: Option<String>,
    /// Resource file type. {{ resource_file_type }}
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1ResourceFile {}


/// List of resource files.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ResourceFiles {
    /// List of resource files.
    #[serde(rename="resourceFile")]
    
    pub resource_file: Option<Vec<GoogleCloudApigeeV1ResourceFile>>,
}

impl client::Part for GoogleCloudApigeeV1ResourceFiles {}


/// The status of a resource loaded in the runtime.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ResourceStatus {
    /// The resource name. Currently only two resources are supported: EnvironmentGroup - organizations/{org}/envgroups/{envgroup} EnvironmentConfig - organizations/{org}/environments/{environment}/deployedConfig
    
    pub resource: Option<String>,
    /// Revisions of the resource currently deployed in the instance.
    
    pub revisions: Option<Vec<GoogleCloudApigeeV1RevisionStatus>>,
    /// The total number of replicas that should have this resource.
    #[serde(rename="totalReplicas")]
    
    pub total_replicas: Option<i32>,
    /// The uid of the resource. In the unexpected case that the instance has multiple uids for the same name, they should be reported under separate ResourceStatuses.
    
    pub uid: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1ResourceStatus {}


/// Result is short for "action result", could be different types identified by "action_result" field. Supported types: 1. DebugInfo : generic debug info collected by runtime recorded as a list of properties. For example, the contents could be virtual host info, state change result, or execution metadata. Required fields : properties, timestamp 2. RequestMessage: information of a http request. Contains headers, request URI and http methods type.Required fields : headers, uri, verb 3. ResponseMessage: information of a http response. Contains headers, reason phrase and http status code. Required fields : headers, reasonPhrase, statusCode 4. ErrorMessage: information of a http error message. Contains detail error message, reason phrase and status code. Required fields : content, headers, reasonPhrase, statusCode 5. VariableAccess: a list of variable access actions, can be Get, Set and Remove. Required fields : accessList
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Result {
    /// Type of the action result. Can be one of the five: DebugInfo, RequestMessage, ResponseMessage, ErrorMessage, VariableAccess
    #[serde(rename="ActionResult")]
    
    pub action_result: Option<String>,
    /// A list of variable access actions agaist the api proxy. Supported values: Get, Set, Remove.
    #[serde(rename="accessList")]
    
    pub access_list: Option<Vec<GoogleCloudApigeeV1Access>>,
    /// Error message content. for example, "content" : "{\"fault\":{\"faultstring\":\"API timed out\",\"detail\":{\"errorcode\":\"flow.APITimedOut\"}}}"
    
    pub content: Option<String>,
    /// A list of HTTP headers. for example, '"headers" : [ { "name" : "Content-Length", "value" : "83" }, { "name" : "Content-Type", "value" : "application/json" } ]'
    
    pub headers: Option<Vec<GoogleCloudApigeeV1Property>>,
    /// Name value pairs used for DebugInfo ActionResult.
    
    pub properties: Option<GoogleCloudApigeeV1Properties>,
    /// HTTP response phrase
    #[serde(rename="reasonPhrase")]
    
    pub reason_phrase: Option<String>,
    /// HTTP response code
    #[serde(rename="statusCode")]
    
    pub status_code: Option<String>,
    /// Timestamp of when the result is recorded. Its format is dd-mm-yy hh:mm:ss:xxx. For example, `"timestamp" : "12-08-19 00:31:59:960"`
    
    pub timestamp: Option<String>,
    /// The relative path of the api proxy. for example, `"uRI" : "/iloveapis"`
    #[serde(rename="uRI")]
    
    pub u_ri: Option<String>,
    /// HTTP method verb
    
    pub verb: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1Result {}


/// API call volume range and the percentage of revenue to share with the developer when the total number of API calls is within the range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1RevenueShareRange {
    /// Ending value of the range. Set to 0 or `null` for the last range of values.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub end: Option<i64>,
    /// Percentage of the revenue to be shared with the developer. For example, to share 21 percent of the total revenue with the developer, set this value to 21. Specify a decimal number with a maximum of two digits following the decimal point.
    #[serde(rename="sharePercentage")]
    
    pub share_percentage: Option<f64>,
    /// Starting value of the range. Set to 0 or `null` for the initial range of values.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start: Option<i64>,
}

impl client::Part for GoogleCloudApigeeV1RevenueShareRange {}


/// The status of a specific resource revision.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1RevisionStatus {
    /// Errors reported when attempting to load this revision.
    
    pub errors: Option<Vec<GoogleCloudApigeeV1UpdateError>>,
    /// The json content of the resource revision. Large specs should be sent individually via the spec field to avoid hitting request size limits.
    #[serde(rename="jsonSpec")]
    
    pub json_spec: Option<String>,
    /// The number of replicas that have successfully loaded this revision.
    
    pub replicas: Option<i32>,
    /// The revision of the resource.
    #[serde(rename="revisionId")]
    
    pub revision_id: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1RevisionStatus {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1RoutingRule {
    /// URI path prefix used to route to the specified environment. May contain one or more wildcards. For example, path segments consisting of a single `*` character will match any string.
    
    pub basepath: Option<String>,
    /// Name of a deployment group in an environment bound to the environment group in the following format: `organizations/{org}/environment/{env}/deploymentGroups/{group}` Only one of environment or deployment_group will be set.
    #[serde(rename="deploymentGroup")]
    
    pub deployment_group: Option<String>,
    /// The env group config revision_id when this rule was added or last updated. This value is set when the rule is created and will only update if the the environment_id changes. It is used to determine if the runtime is up to date with respect to this rule. This field is omitted from the IngressConfig unless the GetDeployedIngressConfig API is called with view=FULL.
    #[serde(rename="envGroupRevision")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub env_group_revision: Option<i64>,
    /// Name of an environment bound to the environment group in the following format: `organizations/{org}/environments/{env}`. Only one of environment or deployment_group will be set.
    
    pub environment: Option<String>,
    /// Conflicting targets, which will be resource names specifying either deployment groups or environments.
    #[serde(rename="otherTargets")]
    
    pub other_targets: Option<Vec<String>>,
    /// The resource name of the proxy revision that is receiving this basepath in the following format: `organizations/{org}/apis/{api}/revisions/{rev}`. This field is omitted from the IngressConfig unless the GetDeployedIngressConfig API is called with view=FULL.
    
    pub receiver: Option<String>,
    /// The unix timestamp when this rule was updated. This is updated whenever env_group_revision is updated. This field is omitted from the IngressConfig unless the GetDeployedIngressConfig API is called with view=FULL.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleCloudApigeeV1RoutingRule {}


/// Runtime configuration for the organization. Response for GetRuntimeConfig.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get runtime config organizations](OrganizationGetRuntimeConfigCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1RuntimeConfig {
    /// Cloud Storage bucket used for uploading Analytics records.
    #[serde(rename="analyticsBucket")]
    
    pub analytics_bucket: Option<String>,
    /// Name of the resource in the following format: `organizations/{org}/runtimeConfig`.
    
    pub name: Option<String>,
    /// Output only. Tenant project ID associated with the Apigee organization. The tenant project is used to host Google-managed resources that are dedicated to this Apigee organization. Clients have limited access to resources within the tenant project used to support Apigee runtime instances. Access to the tenant project is managed using SetSyncAuthorization. It can be empty if the tenant project hasn't been created yet.
    #[serde(rename="tenantProjectId")]
    
    pub tenant_project_id: Option<String>,
    /// Cloud Storage bucket used for uploading Trace records.
    #[serde(rename="traceBucket")]
    
    pub trace_bucket: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1RuntimeConfig {}


/// NEXT ID: 8 RuntimeTraceConfig defines the configurations for distributed trace in an environment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1RuntimeTraceConfig {
    /// Endpoint of the exporter.
    
    pub endpoint: Option<String>,
    /// Exporter that is used to view the distributed trace captured using OpenCensus. An exporter sends traces to any backend that is capable of consuming them. Recorded spans can be exported by registered exporters.
    
    pub exporter: Option<String>,
    /// Name of the trace config in the following format: `organizations/{org}/environment/{env}/traceConfig`
    
    pub name: Option<String>,
    /// List of trace configuration overrides for spicific API proxies.
    
    pub overrides: Option<Vec<GoogleCloudApigeeV1RuntimeTraceConfigOverride>>,
    /// The timestamp that the revision was created or updated.
    #[serde(rename="revisionCreateTime")]
    
    pub revision_create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Revision number which can be used by the runtime to detect if the trace config has changed between two versions.
    #[serde(rename="revisionId")]
    
    pub revision_id: Option<String>,
    /// Trace configuration for all API proxies in an environment.
    #[serde(rename="samplingConfig")]
    
    pub sampling_config: Option<GoogleCloudApigeeV1RuntimeTraceSamplingConfig>,
}

impl client::Part for GoogleCloudApigeeV1RuntimeTraceConfig {}


/// NEXT ID: 7 Trace configuration override for a specific API proxy in an environment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1RuntimeTraceConfigOverride {
    /// Name of the API proxy that will have its trace configuration overridden following format: `organizations/{org}/apis/{api}`
    #[serde(rename="apiProxy")]
    
    pub api_proxy: Option<String>,
    /// Name of the trace config override in the following format: `organizations/{org}/environment/{env}/traceConfig/overrides/{override}`
    
    pub name: Option<String>,
    /// The timestamp that the revision was created or updated.
    #[serde(rename="revisionCreateTime")]
    
    pub revision_create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Revision number which can be used by the runtime to detect if the trace config override has changed between two versions.
    #[serde(rename="revisionId")]
    
    pub revision_id: Option<String>,
    /// Trace configuration override for a specific API proxy in an environment.
    #[serde(rename="samplingConfig")]
    
    pub sampling_config: Option<GoogleCloudApigeeV1RuntimeTraceSamplingConfig>,
    /// Unique ID for the configuration override. The ID will only change if the override is deleted and recreated. Corresponds to name's "override" field.
    
    pub uid: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1RuntimeTraceConfigOverride {}


/// NEXT ID: 3 RuntimeTraceSamplingConfig represents the detail settings of distributed tracing. Only the fields that are defined in the distributed trace configuration can be overridden using the distribute trace configuration override APIs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1RuntimeTraceSamplingConfig {
    /// Sampler of distributed tracing. OFF is the default value.
    
    pub sampler: Option<String>,
    /// Field sampling rate. This value is only applicable when using the PROBABILITY sampler. The supported values are > 0 and <= 0.5.
    #[serde(rename="samplingRate")]
    
    pub sampling_rate: Option<f32>,
}

impl client::Part for GoogleCloudApigeeV1RuntimeTraceSamplingConfig {}


/// Response for Schema call
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments analytics admin get schemav2 organizations](OrganizationEnvironmentAnalyticAdminGetSchemav2Call) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Schema {
    /// List of schema fields grouped as dimensions.
    
    pub dimensions: Option<Vec<GoogleCloudApigeeV1SchemaSchemaElement>>,
    /// Additional metadata associated with schema. This is a legacy field and usually consists of an empty array of strings.
    
    pub meta: Option<Vec<String>>,
    /// List of schema fields grouped as dimensions that can be used with an aggregate function such as `sum`, `avg`, `min`, and `max`.
    
    pub metrics: Option<Vec<GoogleCloudApigeeV1SchemaSchemaElement>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1Schema {}


/// Message type for the schema element
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1SchemaSchemaElement {
    /// Name of the field.
    
    pub name: Option<String>,
    /// Properties for the schema field. For example: { "createTime": "2016-02-26T10:23:09.592Z", "custom": "false", "type": "string" }
    
    pub properties: Option<GoogleCloudApigeeV1SchemaSchemaProperty>,
}

impl client::Part for GoogleCloudApigeeV1SchemaSchemaElement {}


/// Properties for the schema field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1SchemaSchemaProperty {
    /// Time the field was created in RFC3339 string form. For example: `2016-02-26T10:23:09.592Z`.
    #[serde(rename="createTime")]
    
    pub create_time: Option<String>,
    /// Flag that specifies whether the field is standard in the dataset or a custom field created by the customer. `true` indicates that it is a custom field.
    
    pub custom: Option<String>,
    /// Data type of the field.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1SchemaSchemaProperty {}


/// Represents Security Score.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Score {
    /// Component containing score, recommendations and actions.
    
    pub component: Option<GoogleCloudApigeeV1ScoreComponent>,
    /// List of all the drilldown score components.
    
    pub subcomponents: Option<Vec<GoogleCloudApigeeV1ScoreComponent>>,
    /// Start and end time for the score.
    #[serde(rename="timeRange")]
    
    pub time_range: Option<GoogleTypeInterval>,
}

impl client::Part for GoogleCloudApigeeV1Score {}


/// Component is an individual security element that is scored.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ScoreComponent {
    /// Time when score was calculated.
    #[serde(rename="calculateTime")]
    
    pub calculate_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Time in the requested time period when data was last captured to compute the score.
    #[serde(rename="dataCaptureTime")]
    
    pub data_capture_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// List of paths for next components.
    #[serde(rename="drilldownPaths")]
    
    pub drilldown_paths: Option<Vec<String>>,
    /// List of recommendations to improve API security.
    
    pub recommendations: Option<Vec<GoogleCloudApigeeV1ScoreComponentRecommendation>>,
    /// Score for the component.
    
    pub score: Option<i32>,
    /// Path of the component. Example: /org@myorg/envgroup@myenvgroup/proxies/proxy@myproxy
    #[serde(rename="scorePath")]
    
    pub score_path: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1ScoreComponent {}


/// Recommendation based on security concerns and score.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ScoreComponentRecommendation {
    /// Actions for the recommendation to improve the security score.
    
    pub actions: Option<Vec<GoogleCloudApigeeV1ScoreComponentRecommendationAction>>,
    /// Description of the recommendation.
    
    pub description: Option<String>,
    /// Potential impact of this recommendation on the overall score. This denotes how important this recommendation is to improve the score.
    
    pub impact: Option<i32>,
    /// Title represents recommendation title.
    
    pub title: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1ScoreComponentRecommendation {}


/// Action to improve security score.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ScoreComponentRecommendationAction {
    /// Action context for the action.
    #[serde(rename="actionContext")]
    
    pub action_context: Option<GoogleCloudApigeeV1ScoreComponentRecommendationActionActionContext>,
    /// Description of the action.
    
    pub description: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1ScoreComponentRecommendationAction {}


/// Action context are all the relevant details for the action.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ScoreComponentRecommendationActionActionContext {
    /// Documentation link for the action.
    #[serde(rename="documentationLink")]
    
    pub documentation_link: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1ScoreComponentRecommendationActionActionContext {}


/// Represents a SecurityProfile resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [security profiles get organizations](OrganizationSecurityProfileGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1SecurityProfile {
    /// Display name of the security profile.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// List of environments attached to security profile.
    
    pub environments: Option<Vec<GoogleCloudApigeeV1SecurityProfileEnvironment>>,
    /// Output only. Maximum security score that can be generated by this profile.
    #[serde(rename="maxScore")]
    
    pub max_score: Option<i32>,
    /// Output only. Minimum security score that can be generated by this profile.
    #[serde(rename="minScore")]
    
    pub min_score: Option<i32>,
    /// Immutable. Name of the security profile resource. Format: organizations/{org}/securityProfiles/{profile}
    
    pub name: Option<String>,
    /// Output only. The time when revision was created.
    #[serde(rename="revisionCreateTime")]
    
    pub revision_create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Revision ID of the security profile.
    #[serde(rename="revisionId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub revision_id: Option<i64>,
    /// Output only. The time when revision was published. Once published, the security profile revision cannot be updated further and can be attached to environments.
    #[serde(rename="revisionPublishTime")]
    
    pub revision_publish_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The time when revision was updated.
    #[serde(rename="revisionUpdateTime")]
    
    pub revision_update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// List of profile scoring configs in this revision.
    #[serde(rename="scoringConfigs")]
    
    pub scoring_configs: Option<Vec<GoogleCloudApigeeV1SecurityProfileScoringConfig>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1SecurityProfile {}


/// Environment information of attached environments. Scoring an environment is enabled only if it is attached to a security profile.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1SecurityProfileEnvironment {
    /// Output only. Time at which environment was attached to the security profile.
    #[serde(rename="attachTime")]
    
    pub attach_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Name of the environment.
    
    pub environment: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1SecurityProfileEnvironment {}


/// Represents a SecurityProfileEnvironmentAssociation resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [security profiles environments create organizations](OrganizationSecurityProfileEnvironmentCreateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1SecurityProfileEnvironmentAssociation {
    /// Output only. The time when environment was attached to the security profile.
    #[serde(rename="attachTime")]
    
    pub attach_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Immutable. Name of the profile-environment association resource. Format: organizations/{org}/securityProfiles/{profile}/environments/{env}
    
    pub name: Option<String>,
    /// Revision ID of the security profile.
    #[serde(rename="securityProfileRevisionId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub security_profile_revision_id: Option<i64>,
}

impl client::RequestValue for GoogleCloudApigeeV1SecurityProfileEnvironmentAssociation {}
impl client::ResponseResult for GoogleCloudApigeeV1SecurityProfileEnvironmentAssociation {}


/// Security configurations to manage scoring.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1SecurityProfileScoringConfig {
    /// Description of the config.
    
    pub description: Option<String>,
    /// Path of the component config used for scoring.
    #[serde(rename="scorePath")]
    
    pub score_path: Option<String>,
    /// Title of the config.
    
    pub title: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1SecurityProfileScoringConfig {}


/// SecurityReport saves all the information about the created security report.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments security reports create organizations](OrganizationEnvironmentSecurityReportCreateCall) (response)
/// * [environments security reports get organizations](OrganizationEnvironmentSecurityReportGetCall) (response)
/// * [host security reports create organizations](OrganizationHostSecurityReportCreateCall) (response)
/// * [host security reports get organizations](OrganizationHostSecurityReportGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1SecurityReport {
    /// Creation time of the query.
    
    pub created: Option<String>,
    /// Display Name specified by the user.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Hostname is available only when query is executed at host level.
    #[serde(rename="envgroupHostname")]
    
    pub envgroup_hostname: Option<String>,
    /// Error is set when query fails.
    
    pub error: Option<String>,
    /// ExecutionTime is available only after the query is completed.
    #[serde(rename="executionTime")]
    
    pub execution_time: Option<String>,
    /// Contains information like metrics, dimenstions etc of the Security Report.
    #[serde(rename="queryParams")]
    
    pub query_params: Option<GoogleCloudApigeeV1SecurityReportMetadata>,
    /// Report Definition ID.
    #[serde(rename="reportDefinitionId")]
    
    pub report_definition_id: Option<String>,
    /// Result is available only after the query is completed.
    
    pub result: Option<GoogleCloudApigeeV1SecurityReportResultMetadata>,
    /// ResultFileSize is available only after the query is completed.
    #[serde(rename="resultFileSize")]
    
    pub result_file_size: Option<String>,
    /// ResultRows is available only after the query is completed.
    #[serde(rename="resultRows")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub result_rows: Option<i64>,
    /// Self link of the query. Example: `/organizations/myorg/environments/myenv/securityReports/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd` or following format if query is running at host level: `/organizations/myorg/hostSecurityReports/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd`
    #[serde(rename="self")]
    
    pub self_: Option<String>,
    /// Query state could be "enqueued", "running", "completed", "expired" and "failed".
    
    pub state: Option<String>,
    /// Output only. Last updated timestamp for the query.
    
    pub updated: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1SecurityReport {}


/// Metadata for the security report.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1SecurityReportMetadata {
    /// Dimensions of the SecurityReport.
    
    pub dimensions: Option<Vec<String>>,
    /// End timestamp of the query range.
    #[serde(rename="endTimestamp")]
    
    pub end_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Metrics of the SecurityReport. Example: ["name:bot_count,func:sum,alias:sum_bot_count"]
    
    pub metrics: Option<Vec<String>>,
    /// MIME type / Output format.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// Start timestamp of the query range.
    #[serde(rename="startTimestamp")]
    
    pub start_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Query GroupBy time unit. Example: "seconds", "minute", "hour"
    #[serde(rename="timeUnit")]
    
    pub time_unit: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1SecurityReportMetadata {}


/// Body structure when user makes a request to create a security report.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments security reports create organizations](OrganizationEnvironmentSecurityReportCreateCall) (request)
/// * [host security reports create organizations](OrganizationHostSecurityReportCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1SecurityReportQuery {
    /// Delimiter used in the CSV file, if `outputFormat` is set to `csv`. Defaults to the `,` (comma) character. Supported delimiter characters include comma (`,`), pipe (`|`), and tab (`\t`).
    #[serde(rename="csvDelimiter")]
    
    pub csv_delimiter: Option<String>,
    /// A list of dimensions. https://docs.apigee.com/api-platform/analytics/analytics-reference#dimensions
    
    pub dimensions: Option<Vec<String>>,
    /// Security Report display name which users can specify.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Hostname needs to be specified if query intends to run at host level. This field is only allowed when query is submitted by CreateHostSecurityReport where analytics data will be grouped by organization and hostname.
    #[serde(rename="envgroupHostname")]
    
    pub envgroup_hostname: Option<String>,
    /// Boolean expression that can be used to filter data. Filter expressions can be combined using AND/OR terms and should be fully parenthesized to avoid ambiguity. See Analytics metrics, dimensions, and filters reference https://docs.apigee.com/api-platform/analytics/analytics-reference for more information on the fields available to filter on. For more information on the tokens that you use to build filter expressions, see Filter expression syntax. https://docs.apigee.com/api-platform/analytics/asynch-reports-api#filter-expression-syntax
    
    pub filter: Option<String>,
    /// Time unit used to group the result set. Valid values include: second, minute, hour, day, week, or month. If a query includes groupByTimeUnit, then the result is an aggregation based on the specified time unit and the resultant timestamp does not include milliseconds precision. If a query omits groupByTimeUnit, then the resultant timestamp includes milliseconds precision.
    #[serde(rename="groupByTimeUnit")]
    
    pub group_by_time_unit: Option<String>,
    /// Maximum number of rows that can be returned in the result.
    
    pub limit: Option<i32>,
    /// A list of Metrics.
    
    pub metrics: Option<Vec<GoogleCloudApigeeV1SecurityReportQueryMetric>>,
    /// Valid values include: `csv` or `json`. Defaults to `json`. Note: Configure the delimiter for CSV output using the csvDelimiter property.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// Report Definition ID.
    #[serde(rename="reportDefinitionId")]
    
    pub report_definition_id: Option<String>,
    /// Required. Time range for the query. Can use the following predefined strings to specify the time range: `last60minutes` `last24hours` `last7days` Or, specify the timeRange as a structure describing start and end timestamps in the ISO format: yyyy-mm-ddThh:mm:ssZ. Example: "timeRange": { "start": "2018-07-29T00:13:00Z", "end": "2018-08-01T00:18:00Z" }
    #[serde(rename="timeRange")]
    
    pub time_range: Option<json::Value>,
}

impl client::RequestValue for GoogleCloudApigeeV1SecurityReportQuery {}


/// Metric of the Query
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1SecurityReportQueryMetric {
    /// Aggregation function: avg, min, max, or sum.
    #[serde(rename="aggregationFunction")]
    
    pub aggregation_function: Option<String>,
    /// Alias for the metric. Alias will be used to replace metric name in query results.
    
    pub alias: Option<String>,
    /// Required. Metric name.
    
    pub name: Option<String>,
    /// One of `+`, `-`, `/`, `%`, `*`.
    
    pub operator: Option<String>,
    /// Operand value should be provided when operator is set.
    
    pub value: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1SecurityReportQueryMetric {}


/// Contains informations about the security report results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1SecurityReportResultMetadata {
    /// Output only. Expire_time is set to 7 days after report creation. Query result will be unaccessable after this time. Example: "2021-05-04T13:38:52-07:00"
    
    pub expires: Option<String>,
    /// Self link of the query results. Example: `/organizations/myorg/environments/myenv/securityReports/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd/result` or following format if query is running at host level: `/organizations/myorg/hostSecurityReports/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd/result`
    #[serde(rename="self")]
    
    pub self_: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1SecurityReportResultMetadata {}


/// The response for security report result view APIs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments security reports get result view organizations](OrganizationEnvironmentSecurityReportGetResultViewCall) (response)
/// * [host security reports get result view organizations](OrganizationHostSecurityReportGetResultViewCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1SecurityReportResultView {
    /// Error code when there is a failure.
    
    pub code: Option<i32>,
    /// Error message when there is a failure.
    
    pub error: Option<String>,
    /// Metadata contains information like metrics, dimenstions etc of the security report.
    
    pub metadata: Option<GoogleCloudApigeeV1SecurityReportMetadata>,
    /// Rows of security report result. Each row is a JSON object. Example: {sum(message_count): 1, developer_app: "(not set)",…}
    
    pub rows: Option<Vec<json::Value>>,
    /// State of retrieving ResultView.
    
    pub state: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1SecurityReportResultView {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1ServiceIssuersMapping {
    /// List of trusted issuer email ids.
    #[serde(rename="emailIds")]
    
    pub email_ids: Option<Vec<String>>,
    /// String indicating the Apigee service name.
    
    pub service: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1ServiceIssuersMapping {}


/// Session carries the debug session id and its creation time.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Session {
    /// The debug session ID.
    
    pub id: Option<String>,
    /// The first transaction creation timestamp in millisecond, recorded by UAP.
    #[serde(rename="timestampMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub timestamp_ms: Option<i64>,
}

impl client::Part for GoogleCloudApigeeV1Session {}


/// Request for SetAddons.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set addons organizations](OrganizationSetAddonCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1SetAddonsRequest {
    /// Required. Add-on configurations.
    #[serde(rename="addonsConfig")]
    
    pub addons_config: Option<GoogleCloudApigeeV1AddonsConfig>,
}

impl client::RequestValue for GoogleCloudApigeeV1SetAddonsRequest {}


/// The metadata describing a shared flow
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sharedflows delete organizations](OrganizationSharedflowDeleteCall) (response)
/// * [sharedflows get organizations](OrganizationSharedflowGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1SharedFlow {
    /// The id of the most recently created revision for this shared flow.
    #[serde(rename="latestRevisionId")]
    
    pub latest_revision_id: Option<String>,
    /// Metadata describing the shared flow.
    #[serde(rename="metaData")]
    
    pub meta_data: Option<GoogleCloudApigeeV1EntityMetadata>,
    /// The ID of the shared flow.
    
    pub name: Option<String>,
    /// A list of revisions of this shared flow.
    
    pub revision: Option<Vec<String>>,
}

impl client::ResponseResult for GoogleCloudApigeeV1SharedFlow {}


/// The metadata describing a shared flow revision.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sharedflows revisions delete organizations](OrganizationSharedflowRevisionDeleteCall) (response)
/// * [sharedflows revisions update shared flow revision organizations](OrganizationSharedflowRevisionUpdateSharedFlowRevisionCall) (response)
/// * [sharedflows create organizations](OrganizationSharedflowCreateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1SharedFlowRevision {
    /// The version of the configuration schema to which this shared flow conforms. The only supported value currently is majorVersion 4 and minorVersion 0. This setting may be used in the future to enable evolution of the shared flow format.
    #[serde(rename="configurationVersion")]
    
    pub configuration_version: Option<GoogleCloudApigeeV1ConfigVersion>,
    /// A textual description of the shared flow revision.
    #[serde(rename="contextInfo")]
    
    pub context_info: Option<String>,
    /// Time at which this shared flow revision was created, in milliseconds since epoch.
    #[serde(rename="createdAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub created_at: Option<i64>,
    /// Description of the shared flow revision.
    
    pub description: Option<String>,
    /// The human readable name of this shared flow.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// A Key-Value map of metadata about this shared flow revision.
    #[serde(rename="entityMetaDataAsProperties")]
    
    pub entity_meta_data_as_properties: Option<HashMap<String, String>>,
    /// Time at which this shared flow revision was most recently modified, in milliseconds since epoch.
    #[serde(rename="lastModifiedAt")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_modified_at: Option<i64>,
    /// The resource ID of the parent shared flow.
    
    pub name: Option<String>,
    /// A list of policy names included in this shared flow revision.
    
    pub policies: Option<Vec<String>>,
    /// The resource files included in this shared flow revision.
    #[serde(rename="resourceFiles")]
    
    pub resource_files: Option<GoogleCloudApigeeV1ResourceFiles>,
    /// A list of the resources included in this shared flow revision formatted as "{type}://{name}".
    
    pub resources: Option<Vec<String>>,
    /// The resource ID of this revision.
    
    pub revision: Option<String>,
    /// A list of the shared flow names included in this shared flow revision.
    #[serde(rename="sharedFlows")]
    
    pub shared_flows: Option<Vec<String>>,
    /// The string "Application"
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1SharedFlowRevision {}


/// Encapsulates a `stats` response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments stats get organizations](OrganizationEnvironmentStatGetCall) (response)
/// * [host stats get organizations](OrganizationHostStatGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Stats {
    /// List of query results on the environment level.
    
    pub environments: Option<Vec<GoogleCloudApigeeV1StatsEnvironmentStats>>,
    /// List of query results grouped by host.
    
    pub hosts: Option<Vec<GoogleCloudApigeeV1StatsHostStats>>,
    /// Metadata information.
    #[serde(rename="metaData")]
    
    pub meta_data: Option<GoogleCloudApigeeV1Metadata>,
}

impl client::ResponseResult for GoogleCloudApigeeV1Stats {}


/// Encapsulates the environment wrapper: ``` "environments": [ { "metrics": [ { "name": "sum(message_count)", "values": [ "2.52056245E8" ] } ], "name": "prod" } ]```
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1StatsEnvironmentStats {
    /// List of metrics grouped under dimensions.
    
    pub dimensions: Option<Vec<GoogleCloudApigeeV1DimensionMetric>>,
    /// In the final response, only one of the following fields will be present based on the dimensions provided. If no dimensions are provided, then only top-level metrics is provided. If dimensions are included, then there will be a top-level dimensions field under environments which will contain metrics values and the dimension name. Example: ``` "environments": [ { "dimensions": [ { "metrics": [ { "name": "sum(message_count)", "values": [ "2.14049521E8" ] } ], "name": "nit_proxy" } ], "name": "prod" } ]``` or ```"environments": [ { "metrics": [ { "name": "sum(message_count)", "values": [ "2.19026331E8" ] } ], "name": "prod" } ]``` List of metric values.
    
    pub metrics: Option<Vec<GoogleCloudApigeeV1Metric>>,
    /// Name of the environment.
    
    pub name: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1StatsEnvironmentStats {}


/// Encapsulates the hostname wrapper: ``` "hosts": [ { "metrics": [ { "name": "sum(message_count)", "values": [ "2.52056245E8" ] } ], "name": "example.com" } ]```
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1StatsHostStats {
    /// List of metrics grouped under dimensions.
    
    pub dimensions: Option<Vec<GoogleCloudApigeeV1DimensionMetric>>,
    /// In the final response, only one of the following fields will be present based on the dimensions provided. If no dimensions are provided, then only the top-level metrics are provided. If dimensions are included, then there will be a top-level dimensions field under hostnames which will contain metrics values and the dimension name. Example: ``` "hosts": [ { "dimensions": [ { "metrics": [ { "name": "sum(message_count)", "values": [ "2.14049521E8" ] } ], "name": "nit_proxy" } ], "name": "example.com" } ]``` OR ```"hosts": [ { "metrics": [ { "name": "sum(message_count)", "values": [ "2.19026331E8" ] } ], "name": "example.com" } ]``` List of metric values.
    
    pub metrics: Option<Vec<GoogleCloudApigeeV1Metric>>,
    /// Hostname used in query.
    
    pub name: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1StatsHostStats {}


/// Pub/Sub subscription of an environment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments subscribe organizations](OrganizationEnvironmentSubscribeCall) (response)
/// * [environments unsubscribe organizations](OrganizationEnvironmentUnsubscribeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1Subscription {
    /// Full name of the Pub/Sub subcription. Use the following structure in your request: `subscription "projects/foo/subscription/bar"`
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleCloudApigeeV1Subscription {}
impl client::ResponseResult for GoogleCloudApigeeV1Subscription {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get sync authorization organizations](OrganizationGetSyncAuthorizationCall) (response)
/// * [set sync authorization organizations](OrganizationSetSyncAuthorizationCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1SyncAuthorization {
    /// Entity tag (ETag) used for optimistic concurrency control as a way to help prevent simultaneous updates from overwriting each other. For example, when you call [getSyncAuthorization](organizations/getSyncAuthorization) an ETag is returned in the response. Pass that ETag when calling the [setSyncAuthorization](organizations/setSyncAuthorization) to ensure that you are updating the correct version. If you don’t pass the ETag in the call to `setSyncAuthorization`, then the existing authorization is overwritten indiscriminately. **Note**: We strongly recommend that you use the ETag in the read-modify-write cycle to avoid race conditions.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Required. Array of service accounts to grant access to control plane resources, each specified using the following format: `serviceAccount:` service-account-name. The service-account-name is formatted like an email address. For example: `my-synchronizer-manager-service_account@my_project_id.iam.gserviceaccount.com` You might specify multiple service accounts, for example, if you have multiple environments and wish to assign a unique service account to each one. The service accounts must have **Apigee Synchronizer Manager** role. See also [Create service accounts](https://cloud.google.com/apigee/docs/hybrid/latest/sa-about#create-the-service-accounts).
    
    pub identities: Option<Vec<String>>,
}

impl client::RequestValue for GoogleCloudApigeeV1SyncAuthorization {}
impl client::ResponseResult for GoogleCloudApigeeV1SyncAuthorization {}


/// TargetServer configuration. TargetServers are used to decouple a proxy TargetEndpoint HTTPTargetConnections from concrete URLs for backend services.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments targetservers create organizations](OrganizationEnvironmentTargetserverCreateCall) (request|response)
/// * [environments targetservers delete organizations](OrganizationEnvironmentTargetserverDeleteCall) (response)
/// * [environments targetservers get organizations](OrganizationEnvironmentTargetserverGetCall) (response)
/// * [environments targetservers update organizations](OrganizationEnvironmentTargetserverUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1TargetServer {
    /// Optional. A human-readable description of this TargetServer.
    
    pub description: Option<String>,
    /// Required. The host name this target connects to. Value must be a valid hostname as described by RFC-1123.
    
    pub host: Option<String>,
    /// Optional. Enabling/disabling a TargetServer is useful when TargetServers are used in load balancing configurations, and one or more TargetServers need to taken out of rotation periodically. Defaults to true.
    #[serde(rename="isEnabled")]
    
    pub is_enabled: Option<bool>,
    /// Required. The resource id of this target server. Values must match the regular expression 
    
    pub name: Option<String>,
    /// Required. The port number this target connects to on the given host. Value must be between 1 and 65535, inclusive.
    
    pub port: Option<i32>,
    /// Immutable. The protocol used by this TargetServer.
    
    pub protocol: Option<String>,
    /// Optional. Specifies TLS configuration info for this TargetServer. The JSON name is `sSLInfo` for legacy/backwards compatibility reasons -- Edge originally supported SSL, and the name is still used for TLS configuration.
    #[serde(rename="sSLInfo")]
    
    pub s_sl_info: Option<GoogleCloudApigeeV1TlsInfo>,
}

impl client::RequestValue for GoogleCloudApigeeV1TargetServer {}
impl client::ResponseResult for GoogleCloudApigeeV1TargetServer {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1TargetServerConfig {
    /// Whether the target server is enabled. An empty/omitted value for this field should be interpreted as true.
    
    pub enabled: Option<bool>,
    /// Host name of the target server.
    
    pub host: Option<String>,
    /// Target server revision name in the following format: `organizations/{org}/environments/{env}/targetservers/{targetserver}/revisions/{rev}`
    
    pub name: Option<String>,
    /// Port number for the target server.
    
    pub port: Option<i32>,
    /// The protocol used by this target server.
    
    pub protocol: Option<String>,
    /// TLS settings for the target server.
    #[serde(rename="tlsInfo")]
    
    pub tls_info: Option<GoogleCloudApigeeV1TlsInfoConfig>,
}

impl client::Part for GoogleCloudApigeeV1TargetServerConfig {}


/// The response for TestDatastore
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [analytics datastores test organizations](OrganizationAnalyticDatastoreTestCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1TestDatastoreResponse {
    /// Output only. Error message of test connection failure
    
    pub error: Option<String>,
    /// Output only. It could be `completed` or `failed`
    
    pub state: Option<String>,
}

impl client::ResponseResult for GoogleCloudApigeeV1TestDatastoreResponse {}


/// TLS configuration information for virtual hosts and TargetServers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1TlsInfo {
    /// The SSL/TLS cipher suites to be used. For programmable proxies, it must be one of the cipher suite names listed in: http://docs.oracle.com/javase/8/docs/technotes/guides/security/StandardNames.html#ciphersuites. For configurable proxies, it must follow the configuration specified in: https://commondatastorage.googleapis.com/chromium-boringssl-docs/ssl.h.html#Cipher-suite-configuration. This setting has no effect for configurable proxies when negotiating TLS 1.3.
    
    pub ciphers: Option<Vec<String>>,
    /// Optional. Enables two-way TLS.
    #[serde(rename="clientAuthEnabled")]
    
    pub client_auth_enabled: Option<bool>,
    /// The TLS Common Name of the certificate.
    #[serde(rename="commonName")]
    
    pub common_name: Option<GoogleCloudApigeeV1TlsInfoCommonName>,
    /// Required. Enables TLS. If false, neither one-way nor two-way TLS will be enabled.
    
    pub enabled: Option<bool>,
    /// If true, Edge ignores TLS certificate errors. Valid when configuring TLS for target servers and target endpoints, and when configuring virtual hosts that use 2-way TLS. When used with a target endpoint/target server, if the backend system uses SNI and returns a cert with a subject Distinguished Name (DN) that does not match the hostname, there is no way to ignore the error and the connection fails.
    #[serde(rename="ignoreValidationErrors")]
    
    pub ignore_validation_errors: Option<bool>,
    /// Required if `client_auth_enabled` is true. The resource ID for the alias containing the private key and cert.
    #[serde(rename="keyAlias")]
    
    pub key_alias: Option<String>,
    /// Required if `client_auth_enabled` is true. The resource ID of the keystore.
    #[serde(rename="keyStore")]
    
    pub key_store: Option<String>,
    /// The TLS versioins to be used.
    
    pub protocols: Option<Vec<String>>,
    /// The resource ID of the truststore.
    #[serde(rename="trustStore")]
    
    pub trust_store: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1TlsInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1TlsInfoCommonName {
    /// The TLS Common Name string of the certificate.
    
    pub value: Option<String>,
    /// Indicates whether the cert should be matched against as a wildcard cert.
    #[serde(rename="wildcardMatch")]
    
    pub wildcard_match: Option<bool>,
}

impl client::Part for GoogleCloudApigeeV1TlsInfoCommonName {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1TlsInfoConfig {
    /// List of ciphers that are granted access.
    
    pub ciphers: Option<Vec<String>>,
    /// Flag that specifies whether client-side authentication is enabled for the target server. Enables two-way TLS.
    #[serde(rename="clientAuthEnabled")]
    
    pub client_auth_enabled: Option<bool>,
    /// Common name to validate the target server against.
    #[serde(rename="commonName")]
    
    pub common_name: Option<GoogleCloudApigeeV1CommonNameConfig>,
    /// Flag that specifies whether one-way TLS is enabled. Set to `true` to enable one-way TLS.
    
    pub enabled: Option<bool>,
    /// Flag that specifies whether to ignore TLS certificate validation errors. Set to `true` to ignore errors.
    #[serde(rename="ignoreValidationErrors")]
    
    pub ignore_validation_errors: Option<bool>,
    /// Name of the alias used for client-side authentication in the following format: `organizations/{org}/environments/{env}/keystores/{keystore}/aliases/{alias}`
    #[serde(rename="keyAlias")]
    
    pub key_alias: Option<String>,
    /// Reference name and alias pair to use for client-side authentication.
    #[serde(rename="keyAliasReference")]
    
    pub key_alias_reference: Option<GoogleCloudApigeeV1KeyAliasReference>,
    /// List of TLS protocols that are granted access.
    
    pub protocols: Option<Vec<String>>,
    /// Name of the keystore or keystore reference containing trusted certificates for the server in the following format: `organizations/{org}/environments/{env}/keystores/{keystore}` or `organizations/{org}/environments/{env}/references/{reference}`
    #[serde(rename="trustStore")]
    
    pub trust_store: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1TlsInfoConfig {}


/// TraceConfig defines the configurations in an environment of distributed trace.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments get trace config organizations](OrganizationEnvironmentGetTraceConfigCall) (response)
/// * [environments update trace config organizations](OrganizationEnvironmentUpdateTraceConfigCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1TraceConfig {
    /// Required. Endpoint of the exporter.
    
    pub endpoint: Option<String>,
    /// Required. Exporter that is used to view the distributed trace captured using OpenCensus. An exporter sends traces to any backend that is capable of consuming them. Recorded spans can be exported by registered exporters.
    
    pub exporter: Option<String>,
    /// Distributed trace configuration for all API proxies in an environment. You can also override the configuration for a specific API proxy using the distributed trace configuration overrides API.
    #[serde(rename="samplingConfig")]
    
    pub sampling_config: Option<GoogleCloudApigeeV1TraceSamplingConfig>,
}

impl client::RequestValue for GoogleCloudApigeeV1TraceConfig {}
impl client::ResponseResult for GoogleCloudApigeeV1TraceConfig {}


/// A representation of a configuration override.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments trace config overrides create organizations](OrganizationEnvironmentTraceConfigOverrideCreateCall) (request|response)
/// * [environments trace config overrides get organizations](OrganizationEnvironmentTraceConfigOverrideGetCall) (response)
/// * [environments trace config overrides patch organizations](OrganizationEnvironmentTraceConfigOverridePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1TraceConfigOverride {
    /// ID of the API proxy that will have its trace configuration overridden.
    #[serde(rename="apiProxy")]
    
    pub api_proxy: Option<String>,
    /// ID of the trace configuration override specified as a system-generated UUID.
    
    pub name: Option<String>,
    /// Trace configuration to override.
    #[serde(rename="samplingConfig")]
    
    pub sampling_config: Option<GoogleCloudApigeeV1TraceSamplingConfig>,
}

impl client::RequestValue for GoogleCloudApigeeV1TraceConfigOverride {}
impl client::ResponseResult for GoogleCloudApigeeV1TraceConfigOverride {}


/// TraceSamplingConfig represents the detail settings of distributed tracing. Only the fields that are defined in the distributed trace configuration can be overridden using the distribute trace configuration override APIs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1TraceSamplingConfig {
    /// Sampler of distributed tracing. OFF is the default value.
    
    pub sampler: Option<String>,
    /// Field sampling rate. This value is only applicable when using the PROBABILITY sampler. The supported values are > 0 and <= 0.5.
    #[serde(rename="samplingRate")]
    
    pub sampling_rate: Option<f32>,
}

impl client::Part for GoogleCloudApigeeV1TraceSamplingConfig {}


/// Details on why a resource update failed in the runtime.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudApigeeV1UpdateError {
    /// Status code.
    
    pub code: Option<String>,
    /// User-friendly error message.
    
    pub message: Option<String>,
    /// The sub resource specific to this error (e.g. a proxy deployed within the EnvironmentConfig). If empty the error refers to the top level resource.
    
    pub resource: Option<String>,
    /// A string that uniquely identifies the type of error. This provides a more reliable means to deduplicate errors across revisions and instances.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudApigeeV1UpdateError {}


/// Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { "audit_configs": [ { "service": "allServices", "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" }, { "log_type": "ADMIN_READ" } ] }, { "service": "sampleservice.googleapis.com", "audit_log_configs": [ { "log_type": "DATA_READ" }, { "log_type": "DATA_WRITE", "exempted_members": [ "user:aliya@example.com" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts `jose@example.com` from DATA_READ logging, and `aliya@example.com` from DATA_WRITE logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1AuditConfig {
    /// The configuration for logging of each type of permission.
    #[serde(rename="auditLogConfigs")]
    
    pub audit_log_configs: Option<Vec<GoogleIamV1AuditLogConfig>>,
    /// Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
    
    pub service: Option<String>,
}

impl client::Part for GoogleIamV1AuditConfig {}


/// Provides the configuration for logging a type of permissions. Example: { "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1AuditLogConfig {
    /// Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members.
    #[serde(rename="exemptedMembers")]
    
    pub exempted_members: Option<Vec<String>>,
    /// The log type that this config enables.
    #[serde(rename="logType")]
    
    pub log_type: Option<String>,
}

impl client::Part for GoogleIamV1AuditLogConfig {}


/// Associates `members`, or principals, with a `role`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1Binding {
    /// The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the principals in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub condition: Option<GoogleTypeExpr>,
    /// Specifies the principals requesting access for a Google Cloud resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. Does not include identities that come from external identity providers (IdPs) through identity federation. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a Google service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `serviceAccount:{projectid}.svc.id.goog[{namespace}/{kubernetes-sa}]`: An identifier for a [Kubernetes service account](https://cloud.google.com/kubernetes-engine/docs/how-to/kubernetes-service-accounts). For example, `my-project.svc.id.goog[my-namespace/my-kubernetes-sa]`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. 
    
    pub members: Option<Vec<String>>,
    /// Role that is assigned to the list of `members`, or principals. For example, `roles/viewer`, `roles/editor`, or `roles/owner`.
    
    pub role: Option<String>,
}

impl client::Part for GoogleIamV1Binding {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { “bindings”: \[ { “role”: “roles/resourcemanager.organizationAdmin”, “members”: \[ “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com” \] }, { “role”: “roles/resourcemanager.organizationViewer”, “members”: \[ “user:eve@example.com” \], “condition”: { “title”: “expirable access”, “description”: “Does not grant access after Sep 2020”, “expression”: “request.time \< timestamp(‘2020-10-01T00:00:00.000Z’)”, } } \], “etag”: “BwWWja0YfJA=”, “version”: 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time \< timestamp(‘2020-10-01T00:00:00.000Z’) etag: BwWWja0YfJA= version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments get iam policy organizations](OrganizationEnvironmentGetIamPolicyCall) (response)
/// * [environments set iam policy organizations](OrganizationEnvironmentSetIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1Policy {
    /// Specifies cloud audit logging configuration for this policy.
    #[serde(rename="auditConfigs")]
    
    pub audit_configs: Option<Vec<GoogleIamV1AuditConfig>>,
    /// Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`.
    
    pub bindings: Option<Vec<GoogleIamV1Binding>>,
    /// `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub version: Option<i32>,
}

impl client::ResponseResult for GoogleIamV1Policy {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments set iam policy organizations](OrganizationEnvironmentSetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them.
    
    pub policy: Option<GoogleIamV1Policy>,
    /// OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"`
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for GoogleIamV1SetIamPolicyRequest {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments test iam permissions organizations](OrganizationEnvironmentTestIamPermissionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1TestIamPermissionsRequest {
    /// The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions).
    
    pub permissions: Option<Vec<String>>,
}

impl client::RequestValue for GoogleIamV1TestIamPermissionsRequest {}


/// Response message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [environments test iam permissions organizations](OrganizationEnvironmentTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for GoogleIamV1TestIamPermissionsResponse {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [operations list organizations](OrganizationOperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningListOperationsResponse {
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of operations that matches the specified filter in the request.
    
    pub operations: Option<Vec<GoogleLongrunningOperation>>,
}

impl client::ResponseResult for GoogleLongrunningListOperationsResponse {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [endpoint attachments create organizations](OrganizationEndpointAttachmentCreateCall) (response)
/// * [endpoint attachments delete organizations](OrganizationEndpointAttachmentDeleteCall) (response)
/// * [envgroups attachments create organizations](OrganizationEnvgroupAttachmentCreateCall) (response)
/// * [envgroups attachments delete organizations](OrganizationEnvgroupAttachmentDeleteCall) (response)
/// * [envgroups create organizations](OrganizationEnvgroupCreateCall) (response)
/// * [envgroups delete organizations](OrganizationEnvgroupDeleteCall) (response)
/// * [envgroups patch organizations](OrganizationEnvgroupPatchCall) (response)
/// * [environments archive deployments create organizations](OrganizationEnvironmentArchiveDeploymentCreateCall) (response)
/// * [environments create organizations](OrganizationEnvironmentCreateCall) (response)
/// * [environments delete organizations](OrganizationEnvironmentDeleteCall) (response)
/// * [environments modify environment organizations](OrganizationEnvironmentModifyEnvironmentCall) (response)
/// * [instances attachments create organizations](OrganizationInstanceAttachmentCreateCall) (response)
/// * [instances attachments delete organizations](OrganizationInstanceAttachmentDeleteCall) (response)
/// * [instances canaryevaluations create organizations](OrganizationInstanceCanaryevaluationCreateCall) (response)
/// * [instances nat addresses activate organizations](OrganizationInstanceNatAddressActivateCall) (response)
/// * [instances nat addresses create organizations](OrganizationInstanceNatAddressCreateCall) (response)
/// * [instances nat addresses delete organizations](OrganizationInstanceNatAddressDeleteCall) (response)
/// * [instances create organizations](OrganizationInstanceCreateCall) (response)
/// * [instances delete organizations](OrganizationInstanceDeleteCall) (response)
/// * [instances patch organizations](OrganizationInstancePatchCall) (response)
/// * [operations get organizations](OrganizationOperationGetCall) (response)
/// * [create organizations](OrganizationCreateCall) (response)
/// * [delete organizations](OrganizationDeleteCall) (response)
/// * [set addons organizations](OrganizationSetAddonCall) (response)
/// * [provision organization projects](ProjectProvisionOrganizationCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningOperation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<GoogleRpcStatus>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for GoogleLongrunningOperation {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [analytics datastores delete organizations](OrganizationAnalyticDatastoreDeleteCall) (response)
/// * [datacollectors delete organizations](OrganizationDatacollectorDeleteCall) (response)
/// * [developers apps keys apiproducts update developer app key api product organizations](OrganizationDeveloperAppKeyApiproductUpdateDeveloperAppKeyApiProductCall) (response)
/// * [developers set developer status organizations](OrganizationDeveloperSetDeveloperStatuCall) (response)
/// * [environments apis revisions debugsessions delete data organizations](OrganizationEnvironmentApiRevisionDebugsessionDeleteDataCall) (response)
/// * [environments apis revisions undeploy organizations](OrganizationEnvironmentApiRevisionUndeployCall) (response)
/// * [environments archive deployments delete organizations](OrganizationEnvironmentArchiveDeploymentDeleteCall) (response)
/// * [environments caches delete organizations](OrganizationEnvironmentCacheDeleteCall) (response)
/// * [environments sharedflows revisions undeploy organizations](OrganizationEnvironmentSharedflowRevisionUndeployCall) (response)
/// * [environments trace config overrides delete organizations](OrganizationEnvironmentTraceConfigOverrideDeleteCall) (response)
/// * [environments unsubscribe organizations](OrganizationEnvironmentUnsubscribeCall) (response)
/// * [security profiles environments delete organizations](OrganizationSecurityProfileEnvironmentDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleProtobufEmpty { _never_set: Option<bool> }

impl client::ResponseResult for GoogleProtobufEmpty {}


/// Describes what preconditions have failed. For example, if an RPC failed because it required the Terms of Service to be acknowledged, it could list the terms of service violation in the PreconditionFailure message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleRpcPreconditionFailure {
    /// Describes all precondition violations.
    
    pub violations: Option<Vec<GoogleRpcPreconditionFailureViolation>>,
}

impl client::Part for GoogleRpcPreconditionFailure {}


/// A message type used to describe a single precondition failure.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleRpcPreconditionFailureViolation {
    /// A description of how the precondition failed. Developers can use this description to understand how to fix the failure. For example: "Terms of service not accepted".
    
    pub description: Option<String>,
    /// The subject, relative to the type, that failed. For example, "google.com/cloud" relative to the "TOS" type would indicate which terms of service is being referenced.
    
    pub subject: Option<String>,
    /// The type of PreconditionFailure. We recommend using a service-specific enum type to define the supported precondition violation subjects. For example, "TOS" for "Terms of Service violation".
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleRpcPreconditionFailureViolation {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleRpcStatus {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for GoogleRpcStatus {}


/// Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: "Summary size limit" description: "Determines if a summary is less than 100 chars" expression: "document.summary.size() < 100" Example (Equality): title: "Requestor is owner" description: "Determines if requestor is the document owner" expression: "document.owner == request.auth.claims.email" Example (Logic): title: "Public documents" description: "Determine whether the document should be publicly visible" expression: "document.type != 'private' && document.type != 'internal'" Example (Data Manipulation): title: "Notification string" description: "Create a notification string with a timestamp." expression: "'New message received at ' + string(document.create_time)" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeExpr {
    /// Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI.
    
    pub description: Option<String>,
    /// Textual representation of an expression in Common Expression Language syntax.
    
    pub expression: Option<String>,
    /// Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file.
    
    pub location: Option<String>,
    /// Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression.
    
    pub title: Option<String>,
}

impl client::Part for GoogleTypeExpr {}


/// Represents a time interval, encoded as a Timestamp start (inclusive) and a Timestamp end (exclusive). The start must be less than or equal to the end. When the start equals the end, the interval is empty (matches no time). When both start and end are unspecified, the interval matches any time.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeInterval {
    /// Optional. Exclusive end of the interval. If specified, a Timestamp matching this interval will have to be before the end.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Inclusive start of the interval. If specified, a Timestamp matching this interval will have to be the same or after the start.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleTypeInterval {}


/// Represents an amount of money with its currency type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeMoney {
    /// The three-letter currency code defined in ISO 4217.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    
    pub nanos: Option<i32>,
    /// The whole units of the amount. For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub units: Option<i64>,
}

impl client::Part for GoogleTypeMoney {}


