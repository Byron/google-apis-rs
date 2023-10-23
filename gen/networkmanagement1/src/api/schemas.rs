use super::*;
/// Details of the final state "abort" and associated resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AbortInfo {
    /// Causes that the analysis is aborted.
    
    pub cause: Option<AbortInfoCauseEnum>,
    /// List of project IDs that the user has specified in the request but does not have permission to access network configs. Analysis is aborted in this case with the PERMISSION_DENIED cause.
    #[serde(rename="projectsMissingPermission")]
    
    pub projects_missing_permission: Option<Vec<String>>,
    /// URI of the resource that caused the abort.
    #[serde(rename="resourceUri")]
    
    pub resource_uri: Option<String>,
}

impl client::Part for AbortInfo {}


/// Wrapper for app engine service version attributes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppEngineVersionEndpoint {
    /// An [App Engine](https://cloud.google.com/appengine) [service version](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions) name.
    
    pub uri: Option<String>,
}

impl client::Part for AppEngineVersionEndpoint {}


/// For display only. Metadata associated with an App Engine version.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppEngineVersionInfo {
    /// Name of an App Engine version.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// App Engine execution environment for a version.
    
    pub environment: Option<String>,
    /// Runtime of the App Engine version.
    
    pub runtime: Option<String>,
    /// URI of an App Engine version.
    
    pub uri: Option<String>,
}

impl client::Part for AppEngineVersionInfo {}


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
/// * [locations global operations cancel projects](ProjectLocationGlobalOperationCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelOperationRequest {}


/// Wrapper for Cloud Function attributes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudFunctionEndpoint {
    /// A [Cloud Function](https://cloud.google.com/functions) name.
    
    pub uri: Option<String>,
}

impl client::Part for CloudFunctionEndpoint {}


/// For display only. Metadata associated with a Cloud Function.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudFunctionInfo {
    /// Name of a Cloud Function.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Location in which the Cloud Function is deployed.
    
    pub location: Option<String>,
    /// URI of a Cloud Function.
    
    pub uri: Option<String>,
    /// Latest successfully deployed version id of the Cloud Function.
    #[serde(rename="versionId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version_id: Option<i64>,
}

impl client::Part for CloudFunctionInfo {}


/// Wrapper for Cloud Run revision attributes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudRunRevisionEndpoint {
    /// A [Cloud Run](https://cloud.google.com/run) [revision](https://cloud.google.com/run/docs/reference/rest/v1/namespaces.revisions/get) URI. The format is: projects/{project}/locations/{location}/revisions/{revision}
    
    pub uri: Option<String>,
}

impl client::Part for CloudRunRevisionEndpoint {}


/// For display only. Metadata associated with a Cloud Run revision.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudRunRevisionInfo {
    /// Name of a Cloud Run revision.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Location in which this revision is deployed.
    
    pub location: Option<String>,
    /// URI of Cloud Run service this revision belongs to.
    #[serde(rename="serviceUri")]
    
    pub service_uri: Option<String>,
    /// URI of a Cloud Run revision.
    
    pub uri: Option<String>,
}

impl client::Part for CloudRunRevisionInfo {}


/// For display only. Metadata associated with a Cloud SQL instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudSQLInstanceInfo {
    /// Name of a Cloud SQL instance.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// External IP address of a Cloud SQL instance.
    #[serde(rename="externalIp")]
    
    pub external_ip: Option<String>,
    /// Internal IP address of a Cloud SQL instance.
    #[serde(rename="internalIp")]
    
    pub internal_ip: Option<String>,
    /// URI of a Cloud SQL instance network or empty string if the instance does not have one.
    #[serde(rename="networkUri")]
    
    pub network_uri: Option<String>,
    /// Region in which the Cloud SQL instance is running.
    
    pub region: Option<String>,
    /// URI of a Cloud SQL instance.
    
    pub uri: Option<String>,
}

impl client::Part for CloudSQLInstanceInfo {}


/// A Connectivity Test for a network reachability analysis.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global connectivity tests create projects](ProjectLocationGlobalConnectivityTestCreateCall) (request)
/// * [locations global connectivity tests get projects](ProjectLocationGlobalConnectivityTestGetCall) (response)
/// * [locations global connectivity tests patch projects](ProjectLocationGlobalConnectivityTestPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConnectivityTest {
    /// Output only. The time the test was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The user-supplied description of the Connectivity Test. Maximum of 512 characters.
    
    pub description: Option<String>,
    /// Required. Destination specification of the Connectivity Test. You can use a combination of destination IP address, Compute Engine VM instance, or VPC network to uniquely identify the destination location. Even if the destination IP address is not unique, the source IP location is unique. Usually, the analysis can infer the destination endpoint from route information. If the destination you specify is a VM instance and the instance has multiple network interfaces, then you must also specify either a destination IP address or VPC network to identify the destination interface. A reachability analysis proceeds even if the destination location is ambiguous. However, the result can include endpoints that you don't intend to test.
    
    pub destination: Option<Endpoint>,
    /// Output only. The display name of a Connectivity Test.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Resource labels to represent user-provided metadata.
    
    pub labels: Option<HashMap<String, String>>,
    /// Required. Unique name of the resource using the form: `projects/{project_id}/locations/global/connectivityTests/{test_id}`
    
    pub name: Option<String>,
    /// IP Protocol of the test. When not provided, "TCP" is assumed.
    
    pub protocol: Option<String>,
    /// Output only. The reachability details of this test from the latest run. The details are updated when creating a new test, updating an existing test, or triggering a one-time rerun of an existing test.
    #[serde(rename="reachabilityDetails")]
    
    pub reachability_details: Option<ReachabilityDetails>,
    /// Other projects that may be relevant for reachability analysis. This is applicable to scenarios where a test can cross project boundaries.
    #[serde(rename="relatedProjects")]
    
    pub related_projects: Option<Vec<String>>,
    /// Required. Source specification of the Connectivity Test. You can use a combination of source IP address, virtual machine (VM) instance, or Compute Engine network to uniquely identify the source location. Examples: If the source IP address is an internal IP address within a Google Cloud Virtual Private Cloud (VPC) network, then you must also specify the VPC network. Otherwise, specify the VM instance, which already contains its internal IP address and VPC network information. If the source of the test is within an on-premises network, then you must provide the destination VPC network. If the source endpoint is a Compute Engine VM instance with multiple network interfaces, the instance itself is not sufficient to identify the endpoint. So, you must also specify the source IP address or VPC network. A reachability analysis proceeds even if the source location is ambiguous. However, the test result may include endpoints that you don't intend to test.
    
    pub source: Option<Endpoint>,
    /// Output only. The time the test's configuration was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for ConnectivityTest {}
impl client::ResponseResult for ConnectivityTest {}


/// Details of the final state "deliver" and associated resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeliverInfo {
    /// URI of the resource that the packet is delivered to.
    #[serde(rename="resourceUri")]
    
    pub resource_uri: Option<String>,
    /// Target type where the packet is delivered to.
    
    pub target: Option<DeliverInfoTargetEnum>,
}

impl client::Part for DeliverInfo {}


/// Details of the final state "drop" and associated resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DropInfo {
    /// Cause that the packet is dropped.
    
    pub cause: Option<DropInfoCauseEnum>,
    /// URI of the resource that caused the drop.
    #[serde(rename="resourceUri")]
    
    pub resource_uri: Option<String>,
}

impl client::Part for DropInfo {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global operations cancel projects](ProjectLocationGlobalOperationCancelCall) (response)
/// * [locations global operations delete projects](ProjectLocationGlobalOperationDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Source or destination of the Connectivity Test.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Endpoint {
    /// An [App Engine](https://cloud.google.com/appengine) [service version](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions).
    #[serde(rename="appEngineVersion")]
    
    pub app_engine_version: Option<AppEngineVersionEndpoint>,
    /// A [Cloud Function](https://cloud.google.com/functions).
    #[serde(rename="cloudFunction")]
    
    pub cloud_function: Option<CloudFunctionEndpoint>,
    /// A [Cloud Run](https://cloud.google.com/run) [revision](https://cloud.google.com/run/docs/reference/rest/v1/namespaces.revisions/get)
    #[serde(rename="cloudRunRevision")]
    
    pub cloud_run_revision: Option<CloudRunRevisionEndpoint>,
    /// A [Cloud SQL](https://cloud.google.com/sql) instance URI.
    #[serde(rename="cloudSqlInstance")]
    
    pub cloud_sql_instance: Option<String>,
    /// A cluster URI for [Google Kubernetes Engine master](https://cloud.google.com/kubernetes-engine/docs/concepts/cluster-architecture).
    #[serde(rename="gkeMasterCluster")]
    
    pub gke_master_cluster: Option<String>,
    /// A Compute Engine instance URI.
    
    pub instance: Option<String>,
    /// The IP address of the endpoint, which can be an external or internal IP. An IPv6 address is only allowed when the test’s destination is a [global load balancer VIP](https://cloud.google.com/load-balancing/docs/load-balancing-overview).
    #[serde(rename="ipAddress")]
    
    pub ip_address: Option<String>,
    /// A Compute Engine network URI.
    
    pub network: Option<String>,
    /// Type of the network where the endpoint is located. Applicable only to source endpoint, as destination network type can be inferred from the source.
    #[serde(rename="networkType")]
    
    pub network_type: Option<EndpointNetworkTypeEnum>,
    /// The IP protocol port of the endpoint. Only applicable when protocol is TCP or UDP.
    
    pub port: Option<i32>,
    /// Project ID where the endpoint is located. The Project ID can be derived from the URI if you provide a VM instance or network URI. The following are two cases where you must provide the project ID: 1. Only the IP address is specified, and the IP address is within a GCP project. 2. When you are using Shared VPC and the IP address that you provide is from the service project. In this case, the network that the IP address resides in is defined in the host project.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
}

impl client::Part for Endpoint {}


/// For display only. The specification of the endpoints for the test. EndpointInfo is derived from source and destination Endpoint and validated by the backend data plane model.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EndpointInfo {
    /// Destination IP address.
    #[serde(rename="destinationIp")]
    
    pub destination_ip: Option<String>,
    /// URI of the network where this packet is sent to.
    #[serde(rename="destinationNetworkUri")]
    
    pub destination_network_uri: Option<String>,
    /// Destination port. Only valid when protocol is TCP or UDP.
    #[serde(rename="destinationPort")]
    
    pub destination_port: Option<i32>,
    /// IP protocol in string format, for example: "TCP", "UDP", "ICMP".
    
    pub protocol: Option<String>,
    /// Source IP address.
    #[serde(rename="sourceIp")]
    
    pub source_ip: Option<String>,
    /// URI of the network where this packet originates from.
    #[serde(rename="sourceNetworkUri")]
    
    pub source_network_uri: Option<String>,
    /// Source port. Only valid when protocol is TCP or UDP.
    #[serde(rename="sourcePort")]
    
    pub source_port: Option<i32>,
}

impl client::Part for EndpointInfo {}


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


/// For display only. Metadata associated with a VPC firewall rule, an implied VPC firewall rule, or a hierarchical firewall policy rule.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FirewallInfo {
    /// Possible values: ALLOW, DENY
    
    pub action: Option<String>,
    /// Possible values: INGRESS, EGRESS
    
    pub direction: Option<String>,
    /// The display name of the VPC firewall rule. This field is not applicable to hierarchical firewall policy rules.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The firewall rule's type.
    #[serde(rename="firewallRuleType")]
    
    pub firewall_rule_type: Option<FirewallInfoFirewallRuleTypeEnum>,
    /// The URI of the VPC network that the firewall rule is associated with. This field is not applicable to hierarchical firewall policy rules.
    #[serde(rename="networkUri")]
    
    pub network_uri: Option<String>,
    /// The hierarchical firewall policy that this rule is associated with. This field is not applicable to VPC firewall rules.
    
    pub policy: Option<String>,
    /// The priority of the firewall rule.
    
    pub priority: Option<i32>,
    /// The target service accounts specified by the firewall rule.
    #[serde(rename="targetServiceAccounts")]
    
    pub target_service_accounts: Option<Vec<String>>,
    /// The target tags defined by the VPC firewall rule. This field is not applicable to hierarchical firewall policy rules.
    #[serde(rename="targetTags")]
    
    pub target_tags: Option<Vec<String>>,
    /// The URI of the VPC firewall rule. This field is not applicable to implied firewall rules or hierarchical firewall policy rules.
    
    pub uri: Option<String>,
}

impl client::Part for FirewallInfo {}


/// Details of the final state "forward" and associated resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ForwardInfo {
    /// URI of the resource that the packet is forwarded to.
    #[serde(rename="resourceUri")]
    
    pub resource_uri: Option<String>,
    /// Target type where this packet is forwarded to.
    
    pub target: Option<ForwardInfoTargetEnum>,
}

impl client::Part for ForwardInfo {}


/// For display only. Metadata associated with a Compute Engine forwarding rule.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ForwardingRuleInfo {
    /// Name of a Compute Engine forwarding rule.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Port range defined in the forwarding rule that matches the test.
    #[serde(rename="matchedPortRange")]
    
    pub matched_port_range: Option<String>,
    /// Protocol defined in the forwarding rule that matches the test.
    #[serde(rename="matchedProtocol")]
    
    pub matched_protocol: Option<String>,
    /// Network URI. Only valid for Internal Load Balancer.
    #[serde(rename="networkUri")]
    
    pub network_uri: Option<String>,
    /// Target type of the forwarding rule.
    
    pub target: Option<String>,
    /// URI of a Compute Engine forwarding rule.
    
    pub uri: Option<String>,
    /// VIP of the forwarding rule.
    
    pub vip: Option<String>,
}

impl client::Part for ForwardingRuleInfo {}


/// For display only. Metadata associated with a Google Kubernetes Engine (GKE) cluster master.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GKEMasterInfo {
    /// URI of a GKE cluster network.
    #[serde(rename="clusterNetworkUri")]
    
    pub cluster_network_uri: Option<String>,
    /// URI of a GKE cluster.
    #[serde(rename="clusterUri")]
    
    pub cluster_uri: Option<String>,
    /// External IP address of a GKE cluster master.
    #[serde(rename="externalIp")]
    
    pub external_ip: Option<String>,
    /// Internal IP address of a GKE cluster master.
    #[serde(rename="internalIp")]
    
    pub internal_ip: Option<String>,
}

impl client::Part for GKEMasterInfo {}


/// For display only. Metadata associated with a Compute Engine instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceInfo {
    /// Name of a Compute Engine instance.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// External IP address of the network interface.
    #[serde(rename="externalIp")]
    
    pub external_ip: Option<String>,
    /// Name of the network interface of a Compute Engine instance.
    
    pub interface: Option<String>,
    /// Internal IP address of the network interface.
    #[serde(rename="internalIp")]
    
    pub internal_ip: Option<String>,
    /// Network tags configured on the instance.
    #[serde(rename="networkTags")]
    
    pub network_tags: Option<Vec<String>>,
    /// URI of a Compute Engine network.
    #[serde(rename="networkUri")]
    
    pub network_uri: Option<String>,
    /// Service account authorized for the instance.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
    /// URI of a Compute Engine instance.
    
    pub uri: Option<String>,
}

impl client::Part for InstanceInfo {}


/// Response for the `ListConnectivityTests` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global connectivity tests list projects](ProjectLocationGlobalConnectivityTestListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListConnectivityTestsResponse {
    /// Page token to fetch the next set of Connectivity Tests.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of Connectivity Tests.
    
    pub resources: Option<Vec<ConnectivityTest>>,
    /// Locations that could not be reached (when querying all locations with `-`).
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListConnectivityTestsResponse {}


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
/// * [locations global operations list projects](ProjectLocationGlobalOperationListCall) (response)
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


/// For display only. Metadata associated with a specific load balancer backend.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LoadBalancerBackend {
    /// Name of a Compute Engine instance or network endpoint.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// A list of firewall rule URIs allowing probes from health check IP ranges.
    #[serde(rename="healthCheckAllowingFirewallRules")]
    
    pub health_check_allowing_firewall_rules: Option<Vec<String>>,
    /// A list of firewall rule URIs blocking probes from health check IP ranges.
    #[serde(rename="healthCheckBlockingFirewallRules")]
    
    pub health_check_blocking_firewall_rules: Option<Vec<String>>,
    /// State of the health check firewall configuration.
    #[serde(rename="healthCheckFirewallState")]
    
    pub health_check_firewall_state: Option<LoadBalancerBackendHealthCheckFirewallStateEnum>,
    /// URI of a Compute Engine instance or network endpoint.
    
    pub uri: Option<String>,
}

impl client::Part for LoadBalancerBackend {}


/// For display only. Metadata associated with a load balancer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LoadBalancerInfo {
    /// Type of load balancer's backend configuration.
    #[serde(rename="backendType")]
    
    pub backend_type: Option<LoadBalancerInfoBackendTypeEnum>,
    /// Backend configuration URI.
    #[serde(rename="backendUri")]
    
    pub backend_uri: Option<String>,
    /// Information for the loadbalancer backends.
    
    pub backends: Option<Vec<LoadBalancerBackend>>,
    /// URI of the health check for the load balancer.
    #[serde(rename="healthCheckUri")]
    
    pub health_check_uri: Option<String>,
    /// Type of the load balancer.
    #[serde(rename="loadBalancerType")]
    
    pub load_balancer_type: Option<LoadBalancerInfoLoadBalancerTypeEnum>,
}

impl client::Part for LoadBalancerInfo {}


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


/// For display only. Metadata associated with a Compute Engine network.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkInfo {
    /// Name of a Compute Engine network.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The IP range that matches the test.
    #[serde(rename="matchedIpRange")]
    
    pub matched_ip_range: Option<String>,
    /// URI of a Compute Engine network.
    
    pub uri: Option<String>,
}

impl client::Part for NetworkInfo {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global connectivity tests create projects](ProjectLocationGlobalConnectivityTestCreateCall) (response)
/// * [locations global connectivity tests delete projects](ProjectLocationGlobalConnectivityTestDeleteCall) (response)
/// * [locations global connectivity tests patch projects](ProjectLocationGlobalConnectivityTestPatchCall) (response)
/// * [locations global connectivity tests rerun projects](ProjectLocationGlobalConnectivityTestRerunCall) (response)
/// * [locations global operations get projects](ProjectLocationGlobalOperationGetCall) (response)
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
/// * [locations global connectivity tests get iam policy projects](ProjectLocationGlobalConnectivityTestGetIamPolicyCall) (response)
/// * [locations global connectivity tests set iam policy projects](ProjectLocationGlobalConnectivityTestSetIamPolicyCall) (response)
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


/// Results of the configuration analysis from the last run of the test.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReachabilityDetails {
    /// The details of a failure or a cancellation of reachability analysis.
    
    pub error: Option<Status>,
    /// The overall result of the test's configuration analysis.
    
    pub result: Option<ReachabilityDetailResultEnum>,
    /// Result may contain a list of traces if a test has multiple possible paths in the network, such as when destination endpoint is a load balancer with multiple backends.
    
    pub traces: Option<Vec<Trace>>,
    /// The time of the configuration analysis.
    #[serde(rename="verifyTime")]
    
    pub verify_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for ReachabilityDetails {}


/// Request for the `RerunConnectivityTest` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global connectivity tests rerun projects](ProjectLocationGlobalConnectivityTestRerunCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RerunConnectivityTestRequest { _never_set: Option<bool> }

impl client::RequestValue for RerunConnectivityTestRequest {}


/// For display only. Metadata associated with a Compute Engine route.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RouteInfo {
    /// Destination IP range of the route.
    #[serde(rename="destIpRange")]
    
    pub dest_ip_range: Option<String>,
    /// Destination port ranges of the route. Policy based routes only.
    #[serde(rename="destPortRanges")]
    
    pub dest_port_ranges: Option<Vec<String>>,
    /// Name of a Compute Engine route.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Instance tags of the route.
    #[serde(rename="instanceTags")]
    
    pub instance_tags: Option<Vec<String>>,
    /// URI of a Compute Engine network.
    #[serde(rename="networkUri")]
    
    pub network_uri: Option<String>,
    /// Next hop of the route.
    #[serde(rename="nextHop")]
    
    pub next_hop: Option<String>,
    /// Type of next hop.
    #[serde(rename="nextHopType")]
    
    pub next_hop_type: Option<RouteInfoNextHopTypeEnum>,
    /// Priority of the route.
    
    pub priority: Option<i32>,
    /// Protocols of the route. Policy based routes only.
    
    pub protocols: Option<Vec<String>>,
    /// Type of route.
    #[serde(rename="routeType")]
    
    pub route_type: Option<RouteInfoRouteTypeEnum>,
    /// Source IP address range of the route. Policy based routes only.
    #[serde(rename="srcIpRange")]
    
    pub src_ip_range: Option<String>,
    /// Source port ranges of the route. Policy based routes only.
    #[serde(rename="srcPortRanges")]
    
    pub src_port_ranges: Option<Vec<String>>,
    /// URI of a Compute Engine route. Dynamic route from cloud router does not have a URI. Advertised route from Google Cloud VPC to on-premises network also does not have a URI.
    
    pub uri: Option<String>,
}

impl client::Part for RouteInfo {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global connectivity tests set iam policy projects](ProjectLocationGlobalConnectivityTestSetIamPolicyCall) (request)
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


/// A simulated forwarding path is composed of multiple steps. Each step has a well-defined state and an associated configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Step {
    /// Display information of the final state "abort" and reason.
    
    pub abort: Option<AbortInfo>,
    /// Display information of an App Engine service version.
    #[serde(rename="appEngineVersion")]
    
    pub app_engine_version: Option<AppEngineVersionInfo>,
    /// This is a step that leads to the final state Drop.
    #[serde(rename="causesDrop")]
    
    pub causes_drop: Option<bool>,
    /// Display information of a Cloud Function.
    #[serde(rename="cloudFunction")]
    
    pub cloud_function: Option<CloudFunctionInfo>,
    /// Display information of a Cloud Run revision.
    #[serde(rename="cloudRunRevision")]
    
    pub cloud_run_revision: Option<CloudRunRevisionInfo>,
    /// Display information of a Cloud SQL instance.
    #[serde(rename="cloudSqlInstance")]
    
    pub cloud_sql_instance: Option<CloudSQLInstanceInfo>,
    /// Display information of the final state "deliver" and reason.
    
    pub deliver: Option<DeliverInfo>,
    /// A description of the step. Usually this is a summary of the state.
    
    pub description: Option<String>,
    /// Display information of the final state "drop" and reason.
    
    pub drop: Option<DropInfo>,
    /// Display information of the source and destination under analysis. The endpoint information in an intermediate state may differ with the initial input, as it might be modified by state like NAT, or Connection Proxy.
    
    pub endpoint: Option<EndpointInfo>,
    /// Display information of a Compute Engine firewall rule.
    
    pub firewall: Option<FirewallInfo>,
    /// Display information of the final state "forward" and reason.
    
    pub forward: Option<ForwardInfo>,
    /// Display information of a Compute Engine forwarding rule.
    #[serde(rename="forwardingRule")]
    
    pub forwarding_rule: Option<ForwardingRuleInfo>,
    /// Display information of a Google Kubernetes Engine cluster master.
    #[serde(rename="gkeMaster")]
    
    pub gke_master: Option<GKEMasterInfo>,
    /// Display information of a Compute Engine instance.
    
    pub instance: Option<InstanceInfo>,
    /// Display information of the load balancers.
    #[serde(rename="loadBalancer")]
    
    pub load_balancer: Option<LoadBalancerInfo>,
    /// Display information of a Google Cloud network.
    
    pub network: Option<NetworkInfo>,
    /// Project ID that contains the configuration this step is validating.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Display information of a Compute Engine route.
    
    pub route: Option<RouteInfo>,
    /// Each step is in one of the pre-defined states.
    
    pub state: Option<StepStateEnum>,
    /// Display information of a VPC connector.
    #[serde(rename="vpcConnector")]
    
    pub vpc_connector: Option<VpcConnectorInfo>,
    /// Display information of a Compute Engine VPN gateway.
    #[serde(rename="vpnGateway")]
    
    pub vpn_gateway: Option<VpnGatewayInfo>,
    /// Display information of a Compute Engine VPN tunnel.
    #[serde(rename="vpnTunnel")]
    
    pub vpn_tunnel: Option<VpnTunnelInfo>,
}

impl client::Part for Step {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global connectivity tests test iam permissions projects](ProjectLocationGlobalConnectivityTestTestIamPermissionCall) (request)
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
/// * [locations global connectivity tests test iam permissions projects](ProjectLocationGlobalConnectivityTestTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// Trace represents one simulated packet forwarding path. * Each trace contains multiple ordered steps. * Each step is in a particular state with associated configuration. * State is categorized as final or non-final states. * Each final state has a reason associated. * Each trace must end with a final state (the last step). ``` |---------------------Trace----------------------| Step1(State) Step2(State) --- StepN(State(final)) ```
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Trace {
    /// Derived from the source and destination endpoints definition specified by user request, and validated by the data plane model. If there are multiple traces starting from different source locations, then the endpoint_info may be different between traces.
    #[serde(rename="endpointInfo")]
    
    pub endpoint_info: Option<EndpointInfo>,
    /// A trace of a test contains multiple steps from the initial state to the final state (delivered, dropped, forwarded, or aborted). The steps are ordered by the processing sequence within the simulated network state machine. It is critical to preserve the order of the steps and avoid reordering or sorting them.
    
    pub steps: Option<Vec<Step>>,
}

impl client::Part for Trace {}


/// For display only. Metadata associated with a VPC connector.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VpcConnectorInfo {
    /// Name of a VPC connector.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Location in which the VPC connector is deployed.
    
    pub location: Option<String>,
    /// URI of a VPC connector.
    
    pub uri: Option<String>,
}

impl client::Part for VpcConnectorInfo {}


/// For display only. Metadata associated with a Compute Engine VPN gateway.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VpnGatewayInfo {
    /// Name of a VPN gateway.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// IP address of the VPN gateway.
    #[serde(rename="ipAddress")]
    
    pub ip_address: Option<String>,
    /// URI of a Compute Engine network where the VPN gateway is configured.
    #[serde(rename="networkUri")]
    
    pub network_uri: Option<String>,
    /// Name of a Google Cloud region where this VPN gateway is configured.
    
    pub region: Option<String>,
    /// URI of a VPN gateway.
    
    pub uri: Option<String>,
    /// A VPN tunnel that is associated with this VPN gateway. There may be multiple VPN tunnels configured on a VPN gateway, and only the one relevant to the test is displayed.
    #[serde(rename="vpnTunnelUri")]
    
    pub vpn_tunnel_uri: Option<String>,
}

impl client::Part for VpnGatewayInfo {}


/// For display only. Metadata associated with a Compute Engine VPN tunnel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VpnTunnelInfo {
    /// Name of a VPN tunnel.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// URI of a Compute Engine network where the VPN tunnel is configured.
    #[serde(rename="networkUri")]
    
    pub network_uri: Option<String>,
    /// Name of a Google Cloud region where this VPN tunnel is configured.
    
    pub region: Option<String>,
    /// URI of a VPN gateway at remote end of the tunnel.
    #[serde(rename="remoteGateway")]
    
    pub remote_gateway: Option<String>,
    /// Remote VPN gateway's IP address.
    #[serde(rename="remoteGatewayIp")]
    
    pub remote_gateway_ip: Option<String>,
    /// Type of the routing policy.
    #[serde(rename="routingType")]
    
    pub routing_type: Option<VpnTunnelInfoRoutingTypeEnum>,
    /// URI of the VPN gateway at local end of the tunnel.
    #[serde(rename="sourceGateway")]
    
    pub source_gateway: Option<String>,
    /// Local VPN gateway's IP address.
    #[serde(rename="sourceGatewayIp")]
    
    pub source_gateway_ip: Option<String>,
    /// URI of a VPN tunnel.
    
    pub uri: Option<String>,
}

impl client::Part for VpnTunnelInfo {}


