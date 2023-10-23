use super::*;
/// Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { "audit_configs": [ { "service": "allServices", "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" }, { "log_type": "ADMIN_READ" } ] }, { "service": "sampleservice.googleapis.com", "audit_log_configs": [ { "log_type": "DATA_READ" }, { "log_type": "DATA_WRITE", "exempted_members": [ "user:aliya@example.com" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts `jose@example.com` from DATA_READ logging, and `aliya@example.com` from DATA_WRITE logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditConfig {
    /// The configuration for logging of each type of permission.
    #[serde(rename="auditLogConfigs")]
    
    pub audit_log_configs: Option<Vec<AuditLogConfig>>,
    /// Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
    
    pub service: Option<String>,
}

impl client::Part for AuditConfig {}


/// Provides the configuration for logging a type of permissions. Example: { "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditLogConfig {
    /// Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members.
    #[serde(rename="exemptedMembers")]
    
    pub exempted_members: Option<Vec<String>>,
    /// The log type that this config enables.
    #[serde(rename="logType")]
    
    pub log_type: Option<AuditLogConfigLogTypeEnum>,
}

impl client::Part for AuditLogConfig {}


/// AuthConfig defines details of a authentication type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuthConfig {
    /// List containing additional auth configs.
    #[serde(rename="additionalVariables")]
    
    pub additional_variables: Option<Vec<ConfigVariable>>,
    /// The type of authentication configured.
    #[serde(rename="authType")]
    
    pub auth_type: Option<AuthConfigAuthTypeEnum>,
    /// Oauth2ClientCredentials.
    #[serde(rename="oauth2ClientCredentials")]
    
    pub oauth2_client_credentials: Option<Oauth2ClientCredentials>,
    /// Oauth2JwtBearer.
    #[serde(rename="oauth2JwtBearer")]
    
    pub oauth2_jwt_bearer: Option<Oauth2JwtBearer>,
    /// SSH Public Key.
    #[serde(rename="sshPublicKey")]
    
    pub ssh_public_key: Option<SshPublicKey>,
    /// UserPassword.
    #[serde(rename="userPassword")]
    
    pub user_password: Option<UserPassword>,
}

impl client::Part for AuthConfig {}


/// AuthConfigTemplate defines required field over an authentication type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuthConfigTemplate {
    /// The type of authentication configured.
    #[serde(rename="authType")]
    
    pub auth_type: Option<AuthConfigTemplateAuthTypeEnum>,
    /// Config variables to describe an `AuthConfig` for a `Connection`.
    #[serde(rename="configVariableTemplates")]
    
    pub config_variable_templates: Option<Vec<ConfigVariableTemplate>>,
}

impl client::Part for AuthConfigTemplate {}


/// This configuration captures the details required to render an authorization link for the OAuth Authorization Code Flow.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuthorizationCodeLink {
    /// The client ID assigned to the GCP Connectors OAuth app for the connector data source.
    #[serde(rename="clientId")]
    
    pub client_id: Option<String>,
    /// Whether to enable PKCE for the auth code flow.
    #[serde(rename="enablePkce")]
    
    pub enable_pkce: Option<bool>,
    /// The scopes for which the user will authorize GCP Connectors on the connector data source.
    
    pub scopes: Option<Vec<String>>,
    /// The base URI the user must click to trigger the authorization code login flow.
    
    pub uri: Option<String>,
}

impl client::Part for AuthorizationCodeLink {}


/// Associates `members`, or principals, with a `role`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Binding {
    /// The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the principals in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub condition: Option<Expr>,
    /// Specifies the principals requesting access for a Google Cloud resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. Does not include identities that come from external identity providers (IdPs) through identity federation. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a Google service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `serviceAccount:{projectid}.svc.id.goog[{namespace}/{kubernetes-sa}]`: An identifier for a [Kubernetes service account](https://cloud.google.com/kubernetes-engine/docs/how-to/kubernetes-service-accounts). For example, `my-project.svc.id.goog[my-namespace/my-kubernetes-sa]`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. 
    
    pub members: Option<Vec<String>>,
    /// Role that is assigned to the list of `members`, or principals. For example, `roles/viewer`, `roles/editor`, or `roles/owner`.
    
    pub role: Option<String>,
}

impl client::Part for Binding {}


/// The request message for Operations.CancelOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelOperationRequest {}


/// ConfigVariable represents a configuration variable present in a Connection. or AuthConfig.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigVariable {
    /// Value is a bool.
    #[serde(rename="boolValue")]
    
    pub bool_value: Option<bool>,
    /// Value is an integer
    #[serde(rename="intValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub int_value: Option<i64>,
    /// Key of the config variable.
    
    pub key: Option<String>,
    /// Value is a secret.
    #[serde(rename="secretValue")]
    
    pub secret_value: Option<Secret>,
    /// Value is a string.
    #[serde(rename="stringValue")]
    
    pub string_value: Option<String>,
}

impl client::Part for ConfigVariable {}


/// ConfigVariableTemplate provides metadata about a `ConfigVariable` that is used in a Connection.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigVariableTemplate {
    /// Authorization code link options. To be populated if `ValueType` is `AUTHORIZATION_CODE`
    #[serde(rename="authorizationCodeLink")]
    
    pub authorization_code_link: Option<AuthorizationCodeLink>,
    /// Description.
    
    pub description: Option<String>,
    /// Display name of the parameter.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Enum options. To be populated if `ValueType` is `ENUM`
    #[serde(rename="enumOptions")]
    
    pub enum_options: Option<Vec<EnumOption>>,
    /// Key of the config variable.
    
    pub key: Option<String>,
    /// Flag represents that this `ConfigVariable` must be provided for a connection.
    
    pub required: Option<bool>,
    /// Role grant configuration for the config variable.
    #[serde(rename="roleGrant")]
    
    pub role_grant: Option<RoleGrant>,
    /// State of the config variable.
    
    pub state: Option<ConfigVariableTemplateStateEnum>,
    /// Regular expression in RE2 syntax used for validating the `value` of a `ConfigVariable`.
    #[serde(rename="validationRegex")]
    
    pub validation_regex: Option<String>,
    /// Type of the parameter: string, int, bool etc. consider custom type for the benefit for the validation.
    #[serde(rename="valueType")]
    
    pub value_type: Option<ConfigVariableTemplateValueTypeEnum>,
}

impl client::Part for ConfigVariableTemplate {}


/// Connection represents an instance of connector.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations connections create projects](ProjectLocationConnectionCreateCall) (request)
/// * [locations connections get projects](ProjectLocationConnectionGetCall) (response)
/// * [locations connections patch projects](ProjectLocationConnectionPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Connection {
    /// Optional. Configuration for establishing the connection's authentication with an external system.
    #[serde(rename="authConfig")]
    
    pub auth_config: Option<AuthConfig>,
    /// Optional. Configuration for configuring the connection with an external system.
    #[serde(rename="configVariables")]
    
    pub config_variables: Option<Vec<ConfigVariable>>,
    /// Required. Connector version on which the connection is created. The format is: projects/*/locations/*/providers/*/connectors/*/versions/* Only global location is supported for ConnectorVersion resource.
    #[serde(rename="connectorVersion")]
    
    pub connector_version: Option<String>,
    /// Output only. Created time.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Description of the resource.
    
    pub description: Option<String>,
    /// Optional. Configuration of the Connector's destination. Only accepted for Connectors that accepts user defined destination(s).
    #[serde(rename="destinationConfigs")]
    
    pub destination_configs: Option<Vec<DestinationConfig>>,
    /// Output only. GCR location where the envoy image is stored. formatted like: gcr.io/{bucketName}/{imageName}
    #[serde(rename="envoyImageLocation")]
    
    pub envoy_image_location: Option<String>,
    /// Output only. GCR location where the runtime image is stored. formatted like: gcr.io/{bucketName}/{imageName}
    #[serde(rename="imageLocation")]
    
    pub image_location: Option<String>,
    /// Optional. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources
    
    pub labels: Option<HashMap<String, String>>,
    /// Optional. Configuration that indicates whether or not the Connection can be edited.
    #[serde(rename="lockConfig")]
    
    pub lock_config: Option<LockConfig>,
    /// Output only. Resource name of the Connection. Format: projects/{project}/locations/{location}/connections/{connection}
    
    pub name: Option<String>,
    /// Optional. Node configuration for the connection.
    #[serde(rename="nodeConfig")]
    
    pub node_config: Option<NodeConfig>,
    /// Optional. Service account needed for runtime plane to access GCP resources.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
    /// Output only. The name of the Service Directory service name. Used for Private Harpoon to resolve the ILB address. e.g. "projects/cloud-connectors-e2e-testing/locations/us-central1/namespaces/istio-system/services/istio-ingressgateway-connectors"
    #[serde(rename="serviceDirectory")]
    
    pub service_directory: Option<String>,
    /// Output only. Current status of the connection.
    
    pub status: Option<ConnectionStatus>,
    /// Optional. Suspended indicates if a user has suspended a connection or not.
    
    pub suspended: Option<bool>,
    /// Output only. Updated time.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Connection {}
impl client::ResponseResult for Connection {}


/// ConnectionSchemaMetadata is the singleton resource of each connection. It includes the entity and action names of runtime resources exposed by a connection backend.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations connections get connection schema metadata projects](ProjectLocationConnectionGetConnectionSchemaMetadataCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConnectionSchemaMetadata {
    /// Output only. List of actions.
    
    pub actions: Option<Vec<String>>,
    /// Output only. List of entity names.
    
    pub entities: Option<Vec<String>>,
    /// Output only. Resource name. Format: projects/{project}/locations/{location}/connections/{connection}/connectionSchemaMetadata
    
    pub name: Option<String>,
    /// Output only. Timestamp when the connection runtime schema refresh was triggered.
    #[serde(rename="refreshTime")]
    
    pub refresh_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The current state of runtime schema.
    
    pub state: Option<ConnectionSchemaMetadataStateEnum>,
    /// Output only. Timestamp when the connection runtime schema was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for ConnectionSchemaMetadata {}


/// ConnectionStatus indicates the state of the connection.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConnectionStatus {
    /// Description.
    
    pub description: Option<String>,
    /// State.
    
    pub state: Option<ConnectionStatuStateEnum>,
    /// Status provides detailed information for the state.
    
    pub status: Option<String>,
}

impl client::Part for ConnectionStatus {}


/// Connectors indicates a specific connector type, e.x. Salesforce, SAP etc.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations providers connectors get projects](ProjectLocationProviderConnectorGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Connector {
    /// Output only. Created time.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Description of the resource.
    
    pub description: Option<String>,
    /// Output only. Display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Link to documentation page.
    #[serde(rename="documentationUri")]
    
    pub documentation_uri: Option<String>,
    /// Output only. Link to external page.
    #[serde(rename="externalUri")]
    
    pub external_uri: Option<String>,
    /// Output only. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. Flag to mark the version indicating the launch stage.
    #[serde(rename="launchStage")]
    
    pub launch_stage: Option<ConnectorLaunchStageEnum>,
    /// Output only. Resource name of the Connector. Format: projects/{project}/locations/{location}/providers/{provider}/connectors/{connector} Only global location is supported for Connector resource.
    
    pub name: Option<String>,
    /// Output only. Updated time.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Cloud storage location of icons etc consumed by UI.
    #[serde(rename="webAssetsLocation")]
    
    pub web_assets_location: Option<String>,
}

impl client::ResponseResult for Connector {}


/// ConnectorVersion indicates a specific version of a connector.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations providers connectors versions get projects](ProjectLocationProviderConnectorVersionGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConnectorVersion {
    /// Output only. List of auth configs supported by the Connector Version.
    #[serde(rename="authConfigTemplates")]
    
    pub auth_config_templates: Option<Vec<AuthConfigTemplate>>,
    /// Output only. List of config variables needed to create a connection.
    #[serde(rename="configVariableTemplates")]
    
    pub config_variable_templates: Option<Vec<ConfigVariableTemplate>>,
    /// Output only. Created time.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Configuration for Egress Control.
    #[serde(rename="egressControlConfig")]
    
    pub egress_control_config: Option<EgressControlConfig>,
    /// Output only. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. Flag to mark the version indicating the launch stage.
    #[serde(rename="launchStage")]
    
    pub launch_stage: Option<ConnectorVersionLaunchStageEnum>,
    /// Output only. Resource name of the Version. Format: projects/{project}/locations/{location}/providers/{provider}/connectors/{connector}/versions/{version} Only global location is supported for Connector resource.
    
    pub name: Option<String>,
    /// Output only. ReleaseVersion of the connector, for example: "1.0.1-alpha".
    #[serde(rename="releaseVersion")]
    
    pub release_version: Option<String>,
    /// Output only. Role grant configuration for this config variable. It will be DEPRECATED soon.
    #[serde(rename="roleGrant")]
    
    pub role_grant: Option<RoleGrant>,
    /// Output only. Role grant configurations for this connector version.
    #[serde(rename="roleGrants")]
    
    pub role_grants: Option<Vec<RoleGrant>>,
    /// Output only. Information about the runtime features supported by the Connector.
    #[serde(rename="supportedRuntimeFeatures")]
    
    pub supported_runtime_features: Option<SupportedRuntimeFeatures>,
    /// Output only. Updated time.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for ConnectorVersion {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Destination {
    /// For publicly routable host.
    
    pub host: Option<String>,
    /// The port is the target port number that is accepted by the destination.
    
    pub port: Option<i32>,
    /// PSC service attachments. Format: projects/*/regions/*/serviceAttachments/*
    #[serde(rename="serviceAttachment")]
    
    pub service_attachment: Option<String>,
}

impl client::Part for Destination {}


/// Define the Connectors target endpoint.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DestinationConfig {
    /// The destinations for the key.
    
    pub destinations: Option<Vec<Destination>>,
    /// The key is the destination identifier that is supported by the Connector.
    
    pub key: Option<String>,
}

impl client::Part for DestinationConfig {}


/// Egress control config for connector runtime. These configurations define the rules to identify which outbound domains/hosts needs to be whitelisted. It may be a static information for a particular connector version or it is derived from the configurations provided by the customer in Connection resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EgressControlConfig {
    /// Static Comma separated backends which are common for all Connection resources. Supported formats for each backend are host:port or just host (host can be ip address or domain name).
    
    pub backends: Option<String>,
    /// Extractions Rules to extract the backends from customer provided configuration.
    #[serde(rename="extractionRules")]
    
    pub extraction_rules: Option<ExtractionRules>,
}

impl client::Part for EgressControlConfig {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (response)
/// * [locations operations delete projects](ProjectLocationOperationDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// EnumOption definition
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnumOption {
    /// Display name of the option.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Id of the option.
    
    pub id: Option<String>,
}

impl client::Part for EnumOption {}


/// Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: "Summary size limit" description: "Determines if a summary is less than 100 chars" expression: "document.summary.size() < 100" Example (Equality): title: "Requestor is owner" description: "Determines if requestor is the document owner" expression: "document.owner == request.auth.claims.email" Example (Logic): title: "Public documents" description: "Determine whether the document should be publicly visible" expression: "document.type != 'private' && document.type != 'internal'" Example (Data Manipulation): title: "Notification string" description: "Create a notification string with a timestamp." expression: "'New message received at ' + string(document.create_time)" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Expr {
    /// Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI.
    
    pub description: Option<String>,
    /// Textual representation of an expression in Common Expression Language syntax.
    
    pub expression: Option<String>,
    /// Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file.
    
    pub location: Option<String>,
    /// Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression.
    
    pub title: Option<String>,
}

impl client::Part for Expr {}


/// Extraction Rule.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExtractionRule {
    /// Regex used to extract backend details from source. If empty, whole source value will be used.
    #[serde(rename="extractionRegex")]
    
    pub extraction_regex: Option<String>,
    /// Source on which the rule is applied.
    
    pub source: Option<Source>,
}

impl client::Part for ExtractionRule {}


/// Extraction Rules to identity the backends from customer provided configuration in Connection resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExtractionRules {
    /// Collection of Extraction Rule.
    #[serde(rename="extractionRule")]
    
    pub extraction_rule: Option<Vec<ExtractionRule>>,
}

impl client::Part for ExtractionRules {}


/// Metadata of an entity field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Field {
    /// The following map contains fields that are not explicitly mentioned above,this give connectors the flexibility to add new metadata fields.
    #[serde(rename="additionalDetails")]
    
    pub additional_details: Option<HashMap<String, json::Value>>,
    /// The data type of the Field.
    #[serde(rename="dataType")]
    
    pub data_type: Option<FieldDataTypeEnum>,
    /// The following field specifies the default value of the Field provided by the external system if a value is not provided.
    #[serde(rename="defaultValue")]
    
    pub default_value: Option<json::Value>,
    /// A brief description of the Field.
    
    pub description: Option<String>,
    /// Name of the Field.
    
    pub field: Option<String>,
    /// The following boolean field specifies if the current Field acts as a primary key or id if the parent is of type entity.
    
    pub key: Option<bool>,
    /// Specifies whether a null value is allowed.
    
    pub nullable: Option<bool>,
    /// Specifies if the Field is readonly.
    
    pub readonly: Option<bool>,
}

impl client::Part for Field {}


/// Metadata of an input parameter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InputParameter {
    /// The data type of the Parameter.
    #[serde(rename="dataType")]
    
    pub data_type: Option<InputParameterDataTypeEnum>,
    /// The following field specifies the default value of the Parameter provided by the external system if a value is not provided.
    #[serde(rename="defaultValue")]
    
    pub default_value: Option<json::Value>,
    /// A brief description of the Parameter.
    
    pub description: Option<String>,
    /// Specifies whether a null value is allowed.
    
    pub nullable: Option<bool>,
    /// Name of the Parameter.
    
    pub parameter: Option<String>,
}

impl client::Part for InputParameter {}


/// JWT claims used for the jwt-bearer authorization grant.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    /// Value for the "aud" claim.
    
    pub audience: Option<String>,
    /// Value for the "iss" claim.
    
    pub issuer: Option<String>,
    /// Value for the "sub" claim.
    
    pub subject: Option<String>,
}

impl client::Part for JwtClaims {}


/// Response message for ConnectorsService.ListConnections
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations connections list projects](ProjectLocationConnectionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListConnectionsResponse {
    /// Connections.
    
    pub connections: Option<Vec<Connection>>,
    /// Next page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListConnectionsResponse {}


/// Response message for Connectors.ListConnectorVersions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations providers connectors versions list projects](ProjectLocationProviderConnectorVersionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListConnectorVersionsResponse {
    /// A list of connector versions.
    #[serde(rename="connectorVersions")]
    
    pub connector_versions: Option<Vec<ConnectorVersion>>,
    /// Next page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListConnectorVersionsResponse {}


/// Response message for Connectors.ListConnectors.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations providers connectors list projects](ProjectLocationProviderConnectorListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListConnectorsResponse {
    /// A list of connectors.
    
    pub connectors: Option<Vec<Connector>>,
    /// Next page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListConnectorsResponse {}


/// The response message for Locations.ListLocations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations list projects](ProjectLocationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLocationsResponse {
    /// A list of locations that matches the specified filter in the request.
    
    pub locations: Option<Vec<Location>>,
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListLocationsResponse {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations list projects](ProjectLocationOperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOperationsResponse {
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of operations that matches the specified filter in the request.
    
    pub operations: Option<Vec<Operation>>,
}

impl client::ResponseResult for ListOperationsResponse {}


/// Response message for Connectors.ListProviders.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations providers list projects](ProjectLocationProviderListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListProvidersResponse {
    /// Next page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of providers.
    
    pub providers: Option<Vec<Provider>>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListProvidersResponse {}


/// Response message for ConnectorsService.ListRuntimeActionSchemas.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations connections runtime action schemas list projects](ProjectLocationConnectionRuntimeActionSchemaListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListRuntimeActionSchemasResponse {
    /// Next page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Runtime action schemas.
    #[serde(rename="runtimeActionSchemas")]
    
    pub runtime_action_schemas: Option<Vec<RuntimeActionSchema>>,
}

impl client::ResponseResult for ListRuntimeActionSchemasResponse {}


/// Response message for ConnectorsService.ListRuntimeEntitySchemas.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations connections runtime entity schemas list projects](ProjectLocationConnectionRuntimeEntitySchemaListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListRuntimeEntitySchemasResponse {
    /// Next page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Runtime entity schemas.
    #[serde(rename="runtimeEntitySchemas")]
    
    pub runtime_entity_schemas: Option<Vec<RuntimeEntitySchema>>,
}

impl client::ResponseResult for ListRuntimeEntitySchemasResponse {}


/// A resource that represents Google Cloud Platform location.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations get projects](ProjectLocationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// The friendly name for this location, typically a nearby city name. For example, "Tokyo".
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"}
    
    pub labels: Option<HashMap<String, String>>,
    /// The canonical id for this location. For example: `"us-east1"`.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// Service-specific metadata. For example the available capacity at the given location.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"`
    
    pub name: Option<String>,
}

impl client::ResponseResult for Location {}


/// Determines whether or no a connection is locked. If locked, a reason must be specified.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LockConfig {
    /// Indicates whether or not the connection is locked.
    
    pub locked: Option<bool>,
    /// Describes why a connection is locked.
    
    pub reason: Option<String>,
}

impl client::Part for LockConfig {}


/// Node configuration for the connection.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Maximum number of nodes in the runtime nodes.
    #[serde(rename="maxNodeCount")]
    
    pub max_node_count: Option<i32>,
    /// Minimum number of nodes in the runtime nodes.
    #[serde(rename="minNodeCount")]
    
    pub min_node_count: Option<i32>,
}

impl client::Part for NodeConfig {}


/// Parameters to support Oauth 2.0 Client Credentials Grant Authentication. See https://tools.ietf.org/html/rfc6749#section-1.3.4 for more details.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Oauth2ClientCredentials {
    /// The client identifier.
    #[serde(rename="clientId")]
    
    pub client_id: Option<String>,
    /// Secret version reference containing the client secret.
    #[serde(rename="clientSecret")]
    
    pub client_secret: Option<Secret>,
}

impl client::Part for Oauth2ClientCredentials {}


/// Parameters to support JSON Web Token (JWT) Profile for Oauth 2.0 Authorization Grant based authentication. See https://tools.ietf.org/html/rfc7523 for more details.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Oauth2JwtBearer {
    /// Secret version reference containing a PKCS#8 PEM-encoded private key associated with the Client Certificate. This private key will be used to sign JWTs used for the jwt-bearer authorization grant. Specified in the form as: `projects/*/secrets/*/versions/*`.
    #[serde(rename="clientKey")]
    
    pub client_key: Option<Secret>,
    /// JwtClaims providers fields to generate the token.
    #[serde(rename="jwtClaims")]
    
    pub jwt_claims: Option<JwtClaims>,
}

impl client::Part for Oauth2JwtBearer {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations connections connection schema metadata refresh projects](ProjectLocationConnectionConnectionSchemaMetadataRefreshCall) (response)
/// * [locations connections create projects](ProjectLocationConnectionCreateCall) (response)
/// * [locations connections delete projects](ProjectLocationConnectionDeleteCall) (response)
/// * [locations connections patch projects](ProjectLocationConnectionPatchCall) (response)
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<Status>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for Operation {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { “bindings”: \[ { “role”: “roles/resourcemanager.organizationAdmin”, “members”: \[ “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com” \] }, { “role”: “roles/resourcemanager.organizationViewer”, “members”: \[ “user:eve@example.com” \], “condition”: { “title”: “expirable access”, “description”: “Does not grant access after Sep 2020”, “expression”: “request.time \< timestamp(‘2020-10-01T00:00:00.000Z’)”, } } \], “etag”: “BwWWja0YfJA=”, “version”: 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time \< timestamp(‘2020-10-01T00:00:00.000Z’) etag: BwWWja0YfJA= version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations connections get iam policy projects](ProjectLocationConnectionGetIamPolicyCall) (response)
/// * [locations connections set iam policy projects](ProjectLocationConnectionSetIamPolicyCall) (response)
/// * [locations providers get iam policy projects](ProjectLocationProviderGetIamPolicyCall) (response)
/// * [locations providers set iam policy projects](ProjectLocationProviderSetIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// Specifies cloud audit logging configuration for this policy.
    #[serde(rename="auditConfigs")]
    
    pub audit_configs: Option<Vec<AuditConfig>>,
    /// Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`.
    
    pub bindings: Option<Vec<Binding>>,
    /// `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub version: Option<i32>,
}

impl client::ResponseResult for Policy {}


/// Provider indicates the owner who provides the connectors.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations providers get projects](ProjectLocationProviderGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Provider {
    /// Output only. Created time.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Description of the resource.
    
    pub description: Option<String>,
    /// Output only. Display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Link to documentation page.
    #[serde(rename="documentationUri")]
    
    pub documentation_uri: Option<String>,
    /// Output only. Link to external page.
    #[serde(rename="externalUri")]
    
    pub external_uri: Option<String>,
    /// Output only. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. Flag to mark the version indicating the launch stage.
    #[serde(rename="launchStage")]
    
    pub launch_stage: Option<ProviderLaunchStageEnum>,
    /// Output only. Resource name of the Provider. Format: projects/{project}/locations/{location}/providers/{provider} Only global location is supported for Provider resource.
    
    pub name: Option<String>,
    /// Output only. Updated time.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Cloud storage location of icons etc consumed by UI.
    #[serde(rename="webAssetsLocation")]
    
    pub web_assets_location: Option<String>,
}

impl client::ResponseResult for Provider {}


/// Request message for ConnectorsService.RefreshConnectionSchemaMetadata.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations connections connection schema metadata refresh projects](ProjectLocationConnectionConnectionSchemaMetadataRefreshCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RefreshConnectionSchemaMetadataRequest { _never_set: Option<bool> }

impl client::RequestValue for RefreshConnectionSchemaMetadataRequest {}


/// Resource definition
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Resource {
    /// Template to uniquely represent a GCP resource in a format IAM expects This is a template that can have references to other values provided in the config variable template.
    #[serde(rename="pathTemplate")]
    
    pub path_template: Option<String>,
    /// Different types of resource supported.
    #[serde(rename="type")]
    
    pub type_: Option<ResourceTypeEnum>,
}

impl client::Part for Resource {}


/// Metadata of result field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResultMetadata {
    /// The data type of the field.
    #[serde(rename="dataType")]
    
    pub data_type: Option<ResultMetadataDataTypeEnum>,
    /// A brief description of the field.
    
    pub description: Option<String>,
    /// Name of the result field.
    
    pub field: Option<String>,
}

impl client::Part for ResultMetadata {}


/// This configuration defines all the Cloud IAM roles that needs to be granted to a particular GCP resource for the selected prinicpal like service account. These configurations will let UI display to customers what IAM roles need to be granted by them. Or these configurations can be used by the UI to render a 'grant' button to do the same on behalf of the user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RoleGrant {
    /// Template that UI can use to provide helper text to customers.
    #[serde(rename="helperTextTemplate")]
    
    pub helper_text_template: Option<String>,
    /// Prinicipal/Identity for whom the role need to assigned.
    
    pub principal: Option<RoleGrantPrincipalEnum>,
    /// Resource on which the roles needs to be granted for the principal.
    
    pub resource: Option<Resource>,
    /// List of roles that need to be granted.
    
    pub roles: Option<Vec<String>>,
}

impl client::Part for RoleGrant {}


/// Schema of a runtime action.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeActionSchema {
    /// Output only. Name of the action.
    
    pub action: Option<String>,
    /// Output only. List of input parameter metadata for the action.
    #[serde(rename="inputParameters")]
    
    pub input_parameters: Option<Vec<InputParameter>>,
    /// Output only. List of result field metadata.
    #[serde(rename="resultMetadata")]
    
    pub result_metadata: Option<Vec<ResultMetadata>>,
}

impl client::Part for RuntimeActionSchema {}


/// RuntimeConfig is the singleton resource of each location. It includes generic resource configs consumed by control plane and runtime plane like: pub/sub topic/subscription resource name, Cloud Storage location storing schema etc.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations get runtime config projects](ProjectLocationGetRuntimeConfigCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeConfig {
    /// Output only. Pub/Sub subscription for connd to receive message. E.g. projects/{project-id}/subscriptions/{topic-id}
    #[serde(rename="conndSubscription")]
    
    pub connd_subscription: Option<String>,
    /// Output only. Pub/Sub topic for connd to send message. E.g. projects/{project-id}/topics/{topic-id}
    #[serde(rename="conndTopic")]
    
    pub connd_topic: Option<String>,
    /// Output only. Pub/Sub subscription for control plane to receive message. E.g. projects/{project-id}/subscriptions/{topic-id}
    #[serde(rename="controlPlaneSubscription")]
    
    pub control_plane_subscription: Option<String>,
    /// Output only. Pub/Sub topic for control plne to send message. communication. E.g. projects/{project-id}/topics/{topic-id}
    #[serde(rename="controlPlaneTopic")]
    
    pub control_plane_topic: Option<String>,
    /// Output only. location_id of the runtime location. E.g. "us-west1".
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// Output only. Name of the runtimeConfig resource. Format: projects/{project}/locations/{location}/runtimeConfig
    
    pub name: Option<String>,
    /// Output only. The endpoint of the connectors runtime ingress.
    #[serde(rename="runtimeEndpoint")]
    
    pub runtime_endpoint: Option<String>,
    /// Output only. The Cloud Storage bucket that stores connector's schema reports.
    #[serde(rename="schemaGcsBucket")]
    
    pub schema_gcs_bucket: Option<String>,
    /// Output only. The name of the Service Directory service name.
    #[serde(rename="serviceDirectory")]
    
    pub service_directory: Option<String>,
    /// Output only. The state of the location.
    
    pub state: Option<RuntimeConfigStateEnum>,
}

impl client::ResponseResult for RuntimeConfig {}


/// Schema of a runtime entity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeEntitySchema {
    /// Output only. Name of the entity.
    
    pub entity: Option<String>,
    /// Output only. List of fields in the entity.
    
    pub fields: Option<Vec<Field>>,
}

impl client::Part for RuntimeEntitySchema {}


/// Secret provides a reference to entries in Secret Manager.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Secret {
    /// The resource name of the secret version in the format, format as: `projects/*/secrets/*/versions/*`.
    #[serde(rename="secretVersion")]
    
    pub secret_version: Option<String>,
}

impl client::Part for Secret {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations connections set iam policy projects](ProjectLocationConnectionSetIamPolicyCall) (request)
/// * [locations providers set iam policy projects](ProjectLocationProviderSetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them.
    
    pub policy: Option<Policy>,
    /// OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"`
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for SetIamPolicyRequest {}


/// Source to extract the backend from.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Source {
    /// Field identifier. For example config vaiable name.
    #[serde(rename="fieldId")]
    
    pub field_id: Option<String>,
    /// Type of the source.
    #[serde(rename="sourceType")]
    
    pub source_type: Option<SourceSourceTypeEnum>,
}

impl client::Part for Source {}


/// Parameters to support Ssh public key Authentication.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SshPublicKey {
    /// Format of SSH Client cert.
    #[serde(rename="certType")]
    
    pub cert_type: Option<String>,
    /// SSH Client Cert. It should contain both public and private key.
    #[serde(rename="sshClientCert")]
    
    pub ssh_client_cert: Option<Secret>,
    /// Password (passphrase) for ssh client certificate if it has one.
    #[serde(rename="sshClientCertPass")]
    
    pub ssh_client_cert_pass: Option<Secret>,
    /// The user account used to authenticate.
    
    pub username: Option<String>,
}

impl client::Part for SshPublicKey {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for Status {}


/// Supported runtime features of a connector version. This is passed to the management layer to add a new connector version by the connector developer. Details about how this proto is passed to the management layer is covered in this doc - go/runtime-manifest.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SupportedRuntimeFeatures {
    /// Specifies if the connector supports action apis like 'executeAction'.
    #[serde(rename="actionApis")]
    
    pub action_apis: Option<bool>,
    /// Specifies if the connector supports entity apis like 'createEntity'.
    #[serde(rename="entityApis")]
    
    pub entity_apis: Option<bool>,
    /// Specifies if the connector supports 'ExecuteSqlQuery' operation.
    #[serde(rename="sqlQuery")]
    
    pub sql_query: Option<bool>,
}

impl client::Part for SupportedRuntimeFeatures {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations connections test iam permissions projects](ProjectLocationConnectionTestIamPermissionCall) (request)
/// * [locations providers test iam permissions projects](ProjectLocationProviderTestIamPermissionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsRequest {
    /// The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions).
    
    pub permissions: Option<Vec<String>>,
}

impl client::RequestValue for TestIamPermissionsRequest {}


/// Response message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations connections test iam permissions projects](ProjectLocationConnectionTestIamPermissionCall) (response)
/// * [locations providers test iam permissions projects](ProjectLocationProviderTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// Parameters to support Username and Password Authentication.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserPassword {
    /// Secret version reference containing the password.
    
    pub password: Option<Secret>,
    /// Username.
    
    pub username: Option<String>,
}

impl client::Part for UserPassword {}


