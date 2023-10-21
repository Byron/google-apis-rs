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
    
    pub log_type: Option<String>,
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
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelOperationRequest {}


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


/// A definition of a matcher that selects endpoints to which the policies should be applied.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EndpointMatcher {
    /// The matcher is based on node metadata presented by xDS clients.
    #[serde(rename="metadataLabelMatcher")]
    
    pub metadata_label_matcher: Option<EndpointMatcherMetadataLabelMatcher>,
}

impl client::Part for EndpointMatcher {}


/// The matcher that is based on node metadata presented by xDS clients.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EndpointMatcherMetadataLabelMatcher {
    /// Specifies how matching should be done. Supported values are: MATCH_ANY: At least one of the Labels specified in the matcher should match the metadata presented by xDS client. MATCH_ALL: The metadata presented by the xDS client should contain all of the labels specified here. The selection is determined based on the best match. For example, suppose there are three EndpointPolicy resources P1, P2 and P3 and if P1 has a the matcher as MATCH_ANY , P2 has MATCH_ALL , and P3 has MATCH_ALL . If a client with label connects, the config from P1 will be selected. If a client with label connects, the config from P2 will be selected. If a client with label connects, the config from P3 will be selected. If there is more than one best match, (for example, if a config P4 with selector exists and if a client with label connects), an error will be thrown.
    #[serde(rename="metadataLabelMatchCriteria")]
    
    pub metadata_label_match_criteria: Option<String>,
    /// The list of label value pairs that must match labels in the provided metadata based on filterMatchCriteria This list can have at most 64 entries. The list can be empty if the match criteria is MATCH_ANY, to specify a wildcard match (i.e this matches any client).
    #[serde(rename="metadataLabels")]
    
    pub metadata_labels: Option<Vec<EndpointMatcherMetadataLabelMatcherMetadataLabels>>,
}

impl client::Part for EndpointMatcherMetadataLabelMatcher {}


/// Defines a name-pair value for a single label.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EndpointMatcherMetadataLabelMatcherMetadataLabels {
    /// Required. Label name presented as key in xDS Node Metadata.
    #[serde(rename="labelName")]
    
    pub label_name: Option<String>,
    /// Required. Label value presented as value corresponding to the above key, in xDS Node Metadata.
    #[serde(rename="labelValue")]
    
    pub label_value: Option<String>,
}

impl client::Part for EndpointMatcherMetadataLabelMatcherMetadataLabels {}


/// EndpointPolicy is a resource that helps apply desired configuration on the endpoints that match specific criteria. For example, this resource can be used to apply “authentication config” an all endpoints that serve on port 8080.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations endpoint policies create projects](ProjectLocationEndpointPolicyCreateCall) (request)
/// * [locations endpoint policies get projects](ProjectLocationEndpointPolicyGetCall) (response)
/// * [locations endpoint policies patch projects](ProjectLocationEndpointPolicyPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EndpointPolicy {
    /// Optional. This field specifies the URL of AuthorizationPolicy resource that applies authorization policies to the inbound traffic at the matched endpoints. Refer to Authorization. If this field is not specified, authorization is disabled(no authz checks) for this endpoint.
    #[serde(rename="authorizationPolicy")]
    
    pub authorization_policy: Option<String>,
    /// Optional. A URL referring to a ClientTlsPolicy resource. ClientTlsPolicy can be set to specify the authentication for traffic from the proxy to the actual endpoints. More specifically, it is applied to the outgoing traffic from the proxy to the endpoint. This is typically used for sidecar model where the proxy identifies itself as endpoint to the control plane, with the connection between sidecar and endpoint requiring authentication. If this field is not set, authentication is disabled(open). Applicable only when EndpointPolicyType is SIDECAR_PROXY.
    #[serde(rename="clientTlsPolicy")]
    
    pub client_tls_policy: Option<String>,
    /// Output only. The timestamp when the resource was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. A free-text description of the resource. Max length 1024 characters.
    
    pub description: Option<String>,
    /// Required. A matcher that selects endpoints to which the policies should be applied.
    #[serde(rename="endpointMatcher")]
    
    pub endpoint_matcher: Option<EndpointMatcher>,
    /// Optional. Set of label tags associated with the EndpointPolicy resource.
    
    pub labels: Option<HashMap<String, String>>,
    /// Required. Name of the EndpointPolicy resource. It matches pattern `projects/{project}/locations/global/endpointPolicies/{endpoint_policy}`.
    
    pub name: Option<String>,
    /// Optional. A URL referring to ServerTlsPolicy resource. ServerTlsPolicy is used to determine the authentication policy to be applied to terminate the inbound traffic at the identified backends. If this field is not set, authentication is disabled(open) for this endpoint.
    #[serde(rename="serverTlsPolicy")]
    
    pub server_tls_policy: Option<String>,
    /// Optional. Port selector for the (matched) endpoints. If no port selector is provided, the matched config is applied to all ports.
    #[serde(rename="trafficPortSelector")]
    
    pub traffic_port_selector: Option<TrafficPortSelector>,
    /// Required. The type of endpoint policy. This is primarily used to validate the configuration.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Output only. The timestamp when the resource was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for EndpointPolicy {}
impl client::ResponseResult for EndpointPolicy {}


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


/// Gateway represents the configuration for a proxy, typically a load balancer. It captures the ip:port over which the services are exposed by the proxy, along with any policy configurations. Routes have reference to to Gateways to dictate how requests should be routed by this Gateway.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations gateways create projects](ProjectLocationGatewayCreateCall) (request)
/// * [locations gateways get projects](ProjectLocationGatewayGetCall) (response)
/// * [locations gateways patch projects](ProjectLocationGatewayPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Gateway {
    /// Output only. The timestamp when the resource was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. A free-text description of the resource. Max length 1024 characters.
    
    pub description: Option<String>,
    /// Optional. Set of label tags associated with the Gateway resource.
    
    pub labels: Option<HashMap<String, String>>,
    /// Required. Name of the Gateway resource. It matches pattern `projects/*/locations/*/gateways/`.
    
    pub name: Option<String>,
    /// Required. One or more port numbers (1-65535), on which the Gateway will receive traffic. The proxy binds to the specified ports. Gateways of type 'SECURE_WEB_GATEWAY' are limited to 1 port. Gateways of type 'OPEN_MESH' listen on 0.0.0.0 and support multiple ports.
    
    pub ports: Option<Vec<i32>>,
    /// Required. Immutable. Scope determines how configuration across multiple Gateway instances are merged. The configuration for multiple Gateway instances with the same scope will be merged as presented as a single coniguration to the proxy/load balancer. Max length 64 characters. Scope should start with a letter and can only have letters, numbers, hyphens.
    
    pub scope: Option<String>,
    /// Output only. Server-defined URL of this resource
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Optional. A fully-qualified ServerTLSPolicy URL reference. Specifies how TLS traffic is terminated. If empty, TLS termination is disabled.
    #[serde(rename="serverTlsPolicy")]
    
    pub server_tls_policy: Option<String>,
    /// Immutable. The type of the customer managed gateway. This field is required. If unspecified, an error is returned.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Output only. The timestamp when the resource was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Gateway {}
impl client::ResponseResult for Gateway {}


/// GrpcRoute is the resource defining how gRPC traffic routed by a Mesh or Gateway resource is routed.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations grpc routes create projects](ProjectLocationGrpcRouteCreateCall) (request)
/// * [locations grpc routes get projects](ProjectLocationGrpcRouteGetCall) (response)
/// * [locations grpc routes patch projects](ProjectLocationGrpcRoutePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrpcRoute {
    /// Output only. The timestamp when the resource was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. A free-text description of the resource. Max length 1024 characters.
    
    pub description: Option<String>,
    /// Optional. Gateways defines a list of gateways this GrpcRoute is attached to, as one of the routing rules to route the requests served by the gateway. Each gateway reference should match the pattern: `projects/*/locations/global/gateways/`
    
    pub gateways: Option<Vec<String>>,
    /// Required. Service hostnames with an optional port for which this route describes traffic. Format: [:] Hostname is the fully qualified domain name of a network host. This matches the RFC 1123 definition of a hostname with 2 notable exceptions: - IPs are not allowed. - A hostname may be prefixed with a wildcard label (*.). The wildcard label must appear by itself as the first label. Hostname can be "precise" which is a domain name without the terminating dot of a network host (e.g. "foo.example.com") or "wildcard", which is a domain name prefixed with a single wildcard label (e.g. *.example.com). Note that as per RFC1035 and RFC1123, a label must consist of lower case alphanumeric characters or '-', and must start and end with an alphanumeric character. No other punctuation is allowed. The routes associated with a Mesh or Gateway must have unique hostnames. If you attempt to attach multiple routes with conflicting hostnames, the configuration will be rejected. For example, while it is acceptable for routes for the hostnames "*.foo.bar.com" and "*.bar.com" to be associated with the same route, it is not possible to associate two routes both with "*.bar.com" or both with "bar.com". If a port is specified, then gRPC clients must use the channel URI with the port to match this rule (i.e. "xds:///service:123"), otherwise they must supply the URI without a port (i.e. "xds:///service").
    
    pub hostnames: Option<Vec<String>>,
    /// Optional. Set of label tags associated with the GrpcRoute resource.
    
    pub labels: Option<HashMap<String, String>>,
    /// Optional. Meshes defines a list of meshes this GrpcRoute is attached to, as one of the routing rules to route the requests served by the mesh. Each mesh reference should match the pattern: `projects/*/locations/global/meshes/`
    
    pub meshes: Option<Vec<String>>,
    /// Required. Name of the GrpcRoute resource. It matches pattern `projects/*/locations/global/grpcRoutes/`
    
    pub name: Option<String>,
    /// Required. A list of detailed rules defining how to route traffic. Within a single GrpcRoute, the GrpcRoute.RouteAction associated with the first matching GrpcRoute.RouteRule will be executed. At least one rule must be supplied.
    
    pub rules: Option<Vec<GrpcRouteRouteRule>>,
    /// Output only. Server-defined URL of this resource
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Output only. The timestamp when the resource was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GrpcRoute {}
impl client::ResponseResult for GrpcRoute {}


/// The destination to which traffic will be routed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrpcRouteDestination {
    /// Required. The URL of a destination service to which to route traffic. Must refer to either a BackendService or ServiceDirectoryService.
    #[serde(rename="serviceName")]
    
    pub service_name: Option<String>,
    /// Optional. Specifies the proportion of requests forwarded to the backend referenced by the serviceName field. This is computed as: weight/Sum(weights in this destination list). For non-zero values, there may be some epsilon from the exact proportion defined here depending on the precision an implementation supports. If only one serviceName is specified and it has a weight greater than 0, 100% of the traffic is forwarded to that backend. If weights are specified for any one service name, they need to be specified for all of them. If weights are unspecified for all services, then, traffic is distributed in equal proportions to all of them.
    
    pub weight: Option<i32>,
}

impl client::Part for GrpcRouteDestination {}


/// The specification for fault injection introduced into traffic to test the resiliency of clients to destination service failure. As part of fault injection, when clients send requests to a destination, delays can be introduced on a percentage of requests before sending those requests to the destination service. Similarly requests from clients can be aborted by for a percentage of requests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrpcRouteFaultInjectionPolicy {
    /// The specification for aborting to client requests.
    
    pub abort: Option<GrpcRouteFaultInjectionPolicyAbort>,
    /// The specification for injecting delay to client requests.
    
    pub delay: Option<GrpcRouteFaultInjectionPolicyDelay>,
}

impl client::Part for GrpcRouteFaultInjectionPolicy {}


/// Specification of how client requests are aborted as part of fault injection before being sent to a destination.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrpcRouteFaultInjectionPolicyAbort {
    /// The HTTP status code used to abort the request. The value must be between 200 and 599 inclusive.
    #[serde(rename="httpStatus")]
    
    pub http_status: Option<i32>,
    /// The percentage of traffic which will be aborted. The value must be between [0, 100]
    
    pub percentage: Option<i32>,
}

impl client::Part for GrpcRouteFaultInjectionPolicyAbort {}


/// Specification of how client requests are delayed as part of fault injection before being sent to a destination.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrpcRouteFaultInjectionPolicyDelay {
    /// Specify a fixed delay before forwarding the request.
    #[serde(rename="fixedDelay")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub fixed_delay: Option<client::chrono::Duration>,
    /// The percentage of traffic on which delay will be injected. The value must be between [0, 100]
    
    pub percentage: Option<i32>,
}

impl client::Part for GrpcRouteFaultInjectionPolicyDelay {}


/// A match against a collection of headers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrpcRouteHeaderMatch {
    /// Required. The key of the header.
    
    pub key: Option<String>,
    /// Optional. Specifies how to match against the value of the header. If not specified, a default value of EXACT is used.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Required. The value of the header.
    
    pub value: Option<String>,
}

impl client::Part for GrpcRouteHeaderMatch {}


/// Specifies a match against a method.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrpcRouteMethodMatch {
    /// Optional. Specifies that matches are case sensitive. The default value is true. case_sensitive must not be used with a type of REGULAR_EXPRESSION.
    #[serde(rename="caseSensitive")]
    
    pub case_sensitive: Option<bool>,
    /// Required. Name of the method to match against. If unspecified, will match all methods.
    #[serde(rename="grpcMethod")]
    
    pub grpc_method: Option<String>,
    /// Required. Name of the service to match against. If unspecified, will match all services.
    #[serde(rename="grpcService")]
    
    pub grpc_service: Option<String>,
    /// Optional. Specifies how to match against the name. If not specified, a default value of "EXACT" is used.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GrpcRouteMethodMatch {}


/// The specifications for retries.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrpcRouteRetryPolicy {
    /// Specifies the allowed number of retries. This number must be > 0. If not specified, default to 1.
    #[serde(rename="numRetries")]
    
    pub num_retries: Option<u32>,
    /// - connect-failure: Router will retry on failures connecting to Backend Services, for example due to connection timeouts. - refused-stream: Router will retry if the backend service resets the stream with a REFUSED_STREAM error code. This reset type indicates that it is safe to retry. - cancelled: Router will retry if the gRPC status code in the response header is set to cancelled - deadline-exceeded: Router will retry if the gRPC status code in the response header is set to deadline-exceeded - resource-exhausted: Router will retry if the gRPC status code in the response header is set to resource-exhausted - unavailable: Router will retry if the gRPC status code in the response header is set to unavailable
    #[serde(rename="retryConditions")]
    
    pub retry_conditions: Option<Vec<String>>,
}

impl client::Part for GrpcRouteRetryPolicy {}


/// Specifies how to route matched traffic.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrpcRouteRouteAction {
    /// Optional. The destination services to which traffic should be forwarded. If multiple destinations are specified, traffic will be split between Backend Service(s) according to the weight field of these destinations.
    
    pub destinations: Option<Vec<GrpcRouteDestination>>,
    /// Optional. The specification for fault injection introduced into traffic to test the resiliency of clients to destination service failure. As part of fault injection, when clients send requests to a destination, delays can be introduced on a percentage of requests before sending those requests to the destination service. Similarly requests from clients can be aborted by for a percentage of requests. timeout and retry_policy will be ignored by clients that are configured with a fault_injection_policy
    #[serde(rename="faultInjectionPolicy")]
    
    pub fault_injection_policy: Option<GrpcRouteFaultInjectionPolicy>,
    /// Optional. Specifies the retry policy associated with this route.
    #[serde(rename="retryPolicy")]
    
    pub retry_policy: Option<GrpcRouteRetryPolicy>,
    /// Optional. Specifies the timeout for selected route. Timeout is computed from the time the request has been fully processed (i.e. end of stream) up until the response has been completely processed. Timeout includes all retries.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub timeout: Option<client::chrono::Duration>,
}

impl client::Part for GrpcRouteRouteAction {}


/// Criteria for matching traffic. A RouteMatch will be considered to match when all supplied fields match.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrpcRouteRouteMatch {
    /// Optional. Specifies a collection of headers to match.
    
    pub headers: Option<Vec<GrpcRouteHeaderMatch>>,
    /// Optional. A gRPC method to match against. If this field is empty or omitted, will match all methods.
    
    pub method: Option<GrpcRouteMethodMatch>,
}

impl client::Part for GrpcRouteRouteMatch {}


/// Describes how to route traffic.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrpcRouteRouteRule {
    /// Required. A detailed rule defining how to route traffic. This field is required.
    
    pub action: Option<GrpcRouteRouteAction>,
    /// Optional. Matches define conditions used for matching the rule against incoming gRPC requests. Each match is independent, i.e. this rule will be matched if ANY one of the matches is satisfied. If no matches field is specified, this rule will unconditionally match traffic.
    
    pub matches: Option<Vec<GrpcRouteRouteMatch>>,
}

impl client::Part for GrpcRouteRouteRule {}


/// HttpRoute is the resource defining how HTTP traffic should be routed by a Mesh or Gateway resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations http routes create projects](ProjectLocationHttpRouteCreateCall) (request)
/// * [locations http routes get projects](ProjectLocationHttpRouteGetCall) (response)
/// * [locations http routes patch projects](ProjectLocationHttpRoutePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpRoute {
    /// Output only. The timestamp when the resource was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. A free-text description of the resource. Max length 1024 characters.
    
    pub description: Option<String>,
    /// Optional. Gateways defines a list of gateways this HttpRoute is attached to, as one of the routing rules to route the requests served by the gateway. Each gateway reference should match the pattern: `projects/*/locations/global/gateways/`
    
    pub gateways: Option<Vec<String>>,
    /// Required. Hostnames define a set of hosts that should match against the HTTP host header to select a HttpRoute to process the request. Hostname is the fully qualified domain name of a network host, as defined by RFC 1123 with the exception that: - IPs are not allowed. - A hostname may be prefixed with a wildcard label (*.). The wildcard label must appear by itself as the first label. Hostname can be "precise" which is a domain name without the terminating dot of a network host (e.g. "foo.example.com") or "wildcard", which is a domain name prefixed with a single wildcard label (e.g. *.example.com). Note that as per RFC1035 and RFC1123, a label must consist of lower case alphanumeric characters or '-', and must start and end with an alphanumeric character. No other punctuation is allowed. The routes associated with a Mesh or Gateways must have unique hostnames. If you attempt to attach multiple routes with conflicting hostnames, the configuration will be rejected. For example, while it is acceptable for routes for the hostnames "*.foo.bar.com" and "*.bar.com" to be associated with the same Mesh (or Gateways under the same scope), it is not possible to associate two routes both with "*.bar.com" or both with "bar.com".
    
    pub hostnames: Option<Vec<String>>,
    /// Optional. Set of label tags associated with the HttpRoute resource.
    
    pub labels: Option<HashMap<String, String>>,
    /// Optional. Meshes defines a list of meshes this HttpRoute is attached to, as one of the routing rules to route the requests served by the mesh. Each mesh reference should match the pattern: `projects/*/locations/global/meshes/` The attached Mesh should be of a type SIDECAR
    
    pub meshes: Option<Vec<String>>,
    /// Required. Name of the HttpRoute resource. It matches pattern `projects/*/locations/global/httpRoutes/http_route_name>`.
    
    pub name: Option<String>,
    /// Required. Rules that define how traffic is routed and handled. Rules will be matched sequentially based on the RouteMatch specified for the rule.
    
    pub rules: Option<Vec<HttpRouteRouteRule>>,
    /// Output only. Server-defined URL of this resource
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Output only. The timestamp when the resource was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for HttpRoute {}
impl client::ResponseResult for HttpRoute {}


/// The Specification for allowing client side cross-origin requests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpRouteCorsPolicy {
    /// In response to a preflight request, setting this to true indicates that the actual request can include user credentials. This translates to the Access-Control-Allow-Credentials header. Default value is false.
    #[serde(rename="allowCredentials")]
    
    pub allow_credentials: Option<bool>,
    /// Specifies the content for Access-Control-Allow-Headers header.
    #[serde(rename="allowHeaders")]
    
    pub allow_headers: Option<Vec<String>>,
    /// Specifies the content for Access-Control-Allow-Methods header.
    #[serde(rename="allowMethods")]
    
    pub allow_methods: Option<Vec<String>>,
    /// Specifies the regular expression patterns that match allowed origins. For regular expression grammar, please see https://github.com/google/re2/wiki/Syntax.
    #[serde(rename="allowOriginRegexes")]
    
    pub allow_origin_regexes: Option<Vec<String>>,
    /// Specifies the list of origins that will be allowed to do CORS requests. An origin is allowed if it matches either an item in allow_origins or an item in allow_origin_regexes.
    #[serde(rename="allowOrigins")]
    
    pub allow_origins: Option<Vec<String>>,
    /// If true, the CORS policy is disabled. The default value is false, which indicates that the CORS policy is in effect.
    
    pub disabled: Option<bool>,
    /// Specifies the content for Access-Control-Expose-Headers header.
    #[serde(rename="exposeHeaders")]
    
    pub expose_headers: Option<Vec<String>>,
    /// Specifies how long result of a preflight request can be cached in seconds. This translates to the Access-Control-Max-Age header.
    #[serde(rename="maxAge")]
    
    pub max_age: Option<String>,
}

impl client::Part for HttpRouteCorsPolicy {}


/// Specifications of a destination to which the request should be routed to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpRouteDestination {
    /// The URL of a BackendService to route traffic to.
    #[serde(rename="serviceName")]
    
    pub service_name: Option<String>,
    /// Specifies the proportion of requests forwarded to the backend referenced by the serviceName field. This is computed as: weight/Sum(weights in this destination list). For non-zero values, there may be some epsilon from the exact proportion defined here depending on the precision an implementation supports. If only one serviceName is specified and it has a weight greater than 0, 100% of the traffic is forwarded to that backend. If weights are specified for any one service name, they need to be specified for all of them. If weights are unspecified for all services, then, traffic is distributed in equal proportions to all of them.
    
    pub weight: Option<i32>,
}

impl client::Part for HttpRouteDestination {}


/// The specification for fault injection introduced into traffic to test the resiliency of clients to destination service failure. As part of fault injection, when clients send requests to a destination, delays can be introduced by client proxy on a percentage of requests before sending those requests to the destination service. Similarly requests can be aborted by client proxy for a percentage of requests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpRouteFaultInjectionPolicy {
    /// The specification for aborting to client requests.
    
    pub abort: Option<HttpRouteFaultInjectionPolicyAbort>,
    /// The specification for injecting delay to client requests.
    
    pub delay: Option<HttpRouteFaultInjectionPolicyDelay>,
}

impl client::Part for HttpRouteFaultInjectionPolicy {}


/// Specification of how client requests are aborted as part of fault injection before being sent to a destination.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpRouteFaultInjectionPolicyAbort {
    /// The HTTP status code used to abort the request. The value must be between 200 and 599 inclusive.
    #[serde(rename="httpStatus")]
    
    pub http_status: Option<i32>,
    /// The percentage of traffic which will be aborted. The value must be between [0, 100]
    
    pub percentage: Option<i32>,
}

impl client::Part for HttpRouteFaultInjectionPolicyAbort {}


/// Specification of how client requests are delayed as part of fault injection before being sent to a destination.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpRouteFaultInjectionPolicyDelay {
    /// Specify a fixed delay before forwarding the request.
    #[serde(rename="fixedDelay")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub fixed_delay: Option<client::chrono::Duration>,
    /// The percentage of traffic on which delay will be injected. The value must be between [0, 100]
    
    pub percentage: Option<i32>,
}

impl client::Part for HttpRouteFaultInjectionPolicyDelay {}


/// Specifies how to select a route rule based on HTTP request headers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpRouteHeaderMatch {
    /// The value of the header should match exactly the content of exact_match.
    #[serde(rename="exactMatch")]
    
    pub exact_match: Option<String>,
    /// The name of the HTTP header to match against.
    
    pub header: Option<String>,
    /// If specified, the match result will be inverted before checking. Default value is set to false.
    #[serde(rename="invertMatch")]
    
    pub invert_match: Option<bool>,
    /// The value of the header must start with the contents of prefix_match.
    #[serde(rename="prefixMatch")]
    
    pub prefix_match: Option<String>,
    /// A header with header_name must exist. The match takes place whether or not the header has a value.
    #[serde(rename="presentMatch")]
    
    pub present_match: Option<bool>,
    /// If specified, the rule will match if the request header value is within the range.
    #[serde(rename="rangeMatch")]
    
    pub range_match: Option<HttpRouteHeaderMatchIntegerRange>,
    /// The value of the header must match the regular expression specified in regex_match. For regular expression grammar, please see: https://github.com/google/re2/wiki/Syntax
    #[serde(rename="regexMatch")]
    
    pub regex_match: Option<String>,
    /// The value of the header must end with the contents of suffix_match.
    #[serde(rename="suffixMatch")]
    
    pub suffix_match: Option<String>,
}

impl client::Part for HttpRouteHeaderMatch {}


/// Represents an integer value range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpRouteHeaderMatchIntegerRange {
    /// End of the range (exclusive)
    
    pub end: Option<i32>,
    /// Start of the range (inclusive)
    
    pub start: Option<i32>,
}

impl client::Part for HttpRouteHeaderMatchIntegerRange {}


/// The specification for modifying HTTP header in HTTP request and HTTP response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpRouteHeaderModifier {
    /// Add the headers with given map where key is the name of the header, value is the value of the header.
    
    pub add: Option<HashMap<String, String>>,
    /// Remove headers (matching by header names) specified in the list.
    
    pub remove: Option<Vec<String>>,
    /// Completely overwrite/replace the headers with given map where key is the name of the header, value is the value of the header.
    
    pub set: Option<HashMap<String, String>>,
}

impl client::Part for HttpRouteHeaderModifier {}


/// Specifications to match a query parameter in the request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpRouteQueryParameterMatch {
    /// The value of the query parameter must exactly match the contents of exact_match. Only one of exact_match, regex_match, or present_match must be set.
    #[serde(rename="exactMatch")]
    
    pub exact_match: Option<String>,
    /// Specifies that the QueryParameterMatcher matches if request contains query parameter, irrespective of whether the parameter has a value or not. Only one of exact_match, regex_match, or present_match must be set.
    #[serde(rename="presentMatch")]
    
    pub present_match: Option<bool>,
    /// The name of the query parameter to match.
    #[serde(rename="queryParameter")]
    
    pub query_parameter: Option<String>,
    /// The value of the query parameter must match the regular expression specified by regex_match. For regular expression grammar, please see https://github.com/google/re2/wiki/Syntax Only one of exact_match, regex_match, or present_match must be set.
    #[serde(rename="regexMatch")]
    
    pub regex_match: Option<String>,
}

impl client::Part for HttpRouteQueryParameterMatch {}


/// The specification for redirecting traffic.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpRouteRedirect {
    /// The host that will be used in the redirect response instead of the one that was supplied in the request.
    #[serde(rename="hostRedirect")]
    
    pub host_redirect: Option<String>,
    /// If set to true, the URL scheme in the redirected request is set to https. If set to false, the URL scheme of the redirected request will remain the same as that of the request. The default is set to false.
    #[serde(rename="httpsRedirect")]
    
    pub https_redirect: Option<bool>,
    /// The path that will be used in the redirect response instead of the one that was supplied in the request. path_redirect can not be supplied together with prefix_redirect. Supply one alone or neither. If neither is supplied, the path of the original request will be used for the redirect.
    #[serde(rename="pathRedirect")]
    
    pub path_redirect: Option<String>,
    /// The port that will be used in the redirected request instead of the one that was supplied in the request.
    #[serde(rename="portRedirect")]
    
    pub port_redirect: Option<i32>,
    /// Indicates that during redirection, the matched prefix (or path) should be swapped with this value. This option allows URLs be dynamically created based on the request.
    #[serde(rename="prefixRewrite")]
    
    pub prefix_rewrite: Option<String>,
    /// The HTTP Status code to use for the redirect.
    #[serde(rename="responseCode")]
    
    pub response_code: Option<String>,
    /// if set to true, any accompanying query portion of the original URL is removed prior to redirecting the request. If set to false, the query portion of the original URL is retained. The default is set to false.
    #[serde(rename="stripQuery")]
    
    pub strip_query: Option<bool>,
}

impl client::Part for HttpRouteRedirect {}


/// Specifies the policy on how requests are shadowed to a separate mirrored destination service. The proxy does not wait for responses from the shadow service. Prior to sending traffic to the shadow service, the host/authority header is suffixed with -shadow.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpRouteRequestMirrorPolicy {
    /// The destination the requests will be mirrored to. The weight of the destination will be ignored.
    
    pub destination: Option<HttpRouteDestination>,
}

impl client::Part for HttpRouteRequestMirrorPolicy {}


/// The specifications for retries.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpRouteRetryPolicy {
    /// Specifies the allowed number of retries. This number must be > 0. If not specified, default to 1.
    #[serde(rename="numRetries")]
    
    pub num_retries: Option<i32>,
    /// Specifies a non-zero timeout per retry attempt.
    #[serde(rename="perTryTimeout")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub per_try_timeout: Option<client::chrono::Duration>,
    /// Specifies one or more conditions when this retry policy applies. Valid values are: 5xx: Proxy will attempt a retry if the destination service responds with any 5xx response code, of if the destination service does not respond at all, example: disconnect, reset, read timeout, connection failure and refused streams. gateway-error: Similar to 5xx, but only applies to response codes 502, 503, 504. reset: Proxy will attempt a retry if the destination service does not respond at all (disconnect/reset/read timeout) connect-failure: Proxy will retry on failures connecting to destination for example due to connection timeouts. retriable-4xx: Proxy will retry fro retriable 4xx response codes. Currently the only retriable error supported is 409. refused-stream: Proxy will retry if the destination resets the stream with a REFUSED_STREAM error code. This reset type indicates that it is safe to retry.
    #[serde(rename="retryConditions")]
    
    pub retry_conditions: Option<Vec<String>>,
}

impl client::Part for HttpRouteRetryPolicy {}


/// The specifications for routing traffic and applying associated policies.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpRouteRouteAction {
    /// The specification for allowing client side cross-origin requests.
    #[serde(rename="corsPolicy")]
    
    pub cors_policy: Option<HttpRouteCorsPolicy>,
    /// The destination to which traffic should be forwarded.
    
    pub destinations: Option<Vec<HttpRouteDestination>>,
    /// The specification for fault injection introduced into traffic to test the resiliency of clients to backend service failure. As part of fault injection, when clients send requests to a backend service, delays can be introduced on a percentage of requests before sending those requests to the backend service. Similarly requests from clients can be aborted for a percentage of requests. timeout and retry_policy will be ignored by clients that are configured with a fault_injection_policy
    #[serde(rename="faultInjectionPolicy")]
    
    pub fault_injection_policy: Option<HttpRouteFaultInjectionPolicy>,
    /// If set, the request is directed as configured by this field.
    
    pub redirect: Option<HttpRouteRedirect>,
    /// The specification for modifying the headers of a matching request prior to delivery of the request to the destination.
    #[serde(rename="requestHeaderModifier")]
    
    pub request_header_modifier: Option<HttpRouteHeaderModifier>,
    /// Specifies the policy on how requests intended for the routes destination are shadowed to a separate mirrored destination. Proxy will not wait for the shadow destination to respond before returning the response. Prior to sending traffic to the shadow service, the host/authority header is suffixed with -shadow.
    #[serde(rename="requestMirrorPolicy")]
    
    pub request_mirror_policy: Option<HttpRouteRequestMirrorPolicy>,
    /// The specification for modifying the headers of a response prior to sending the response back to the client.
    #[serde(rename="responseHeaderModifier")]
    
    pub response_header_modifier: Option<HttpRouteHeaderModifier>,
    /// Specifies the retry policy associated with this route.
    #[serde(rename="retryPolicy")]
    
    pub retry_policy: Option<HttpRouteRetryPolicy>,
    /// Specifies the timeout for selected route. Timeout is computed from the time the request has been fully processed (i.e. end of stream) up until the response has been completely processed. Timeout includes all retries.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub timeout: Option<client::chrono::Duration>,
    /// The specification for rewrite URL before forwarding requests to the destination.
    #[serde(rename="urlRewrite")]
    
    pub url_rewrite: Option<HttpRouteURLRewrite>,
}

impl client::Part for HttpRouteRouteAction {}


/// RouteMatch defines specifications used to match requests. If multiple match types are set, this RouteMatch will match if ALL type of matches are matched.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpRouteRouteMatch {
    /// The HTTP request path value should exactly match this value. Only one of full_path_match, prefix_match, or regex_match should be used.
    #[serde(rename="fullPathMatch")]
    
    pub full_path_match: Option<String>,
    /// Specifies a list of HTTP request headers to match against. ALL of the supplied headers must be matched.
    
    pub headers: Option<Vec<HttpRouteHeaderMatch>>,
    /// Specifies if prefix_match and full_path_match matches are case sensitive. The default value is false.
    #[serde(rename="ignoreCase")]
    
    pub ignore_case: Option<bool>,
    /// The HTTP request path value must begin with specified prefix_match. prefix_match must begin with a /. Only one of full_path_match, prefix_match, or regex_match should be used.
    #[serde(rename="prefixMatch")]
    
    pub prefix_match: Option<String>,
    /// Specifies a list of query parameters to match against. ALL of the query parameters must be matched.
    #[serde(rename="queryParameters")]
    
    pub query_parameters: Option<Vec<HttpRouteQueryParameterMatch>>,
    /// The HTTP request path value must satisfy the regular expression specified by regex_match after removing any query parameters and anchor supplied with the original URL. For regular expression grammar, please see https://github.com/google/re2/wiki/Syntax Only one of full_path_match, prefix_match, or regex_match should be used.
    #[serde(rename="regexMatch")]
    
    pub regex_match: Option<String>,
}

impl client::Part for HttpRouteRouteMatch {}


/// Specifies how to match traffic and how to route traffic when traffic is matched.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpRouteRouteRule {
    /// The detailed rule defining how to route matched traffic.
    
    pub action: Option<HttpRouteRouteAction>,
    /// A list of matches define conditions used for matching the rule against incoming HTTP requests. Each match is independent, i.e. this rule will be matched if ANY one of the matches is satisfied. If no matches field is specified, this rule will unconditionally match traffic. If a default rule is desired to be configured, add a rule with no matches specified to the end of the rules list.
    
    pub matches: Option<Vec<HttpRouteRouteMatch>>,
}

impl client::Part for HttpRouteRouteRule {}


/// The specification for modifying the URL of the request, prior to forwarding the request to the destination.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpRouteURLRewrite {
    /// Prior to forwarding the request to the selected destination, the requests host header is replaced by this value.
    #[serde(rename="hostRewrite")]
    
    pub host_rewrite: Option<String>,
    /// Prior to forwarding the request to the selected destination, the matching portion of the requests path is replaced by this value.
    #[serde(rename="pathPrefixRewrite")]
    
    pub path_prefix_rewrite: Option<String>,
}

impl client::Part for HttpRouteURLRewrite {}


/// Response returned by the ListEndpointPolicies method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations endpoint policies list projects](ProjectLocationEndpointPolicyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListEndpointPoliciesResponse {
    /// List of EndpointPolicy resources.
    #[serde(rename="endpointPolicies")]
    
    pub endpoint_policies: Option<Vec<EndpointPolicy>>,
    /// If there might be more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListEndpointPoliciesResponse {}


/// Response returned by the ListGateways method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations gateways list projects](ProjectLocationGatewayListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListGatewaysResponse {
    /// List of Gateway resources.
    
    pub gateways: Option<Vec<Gateway>>,
    /// If there might be more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListGatewaysResponse {}


/// Response returned by the ListGrpcRoutes method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations grpc routes list projects](ProjectLocationGrpcRouteListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListGrpcRoutesResponse {
    /// List of GrpcRoute resources.
    #[serde(rename="grpcRoutes")]
    
    pub grpc_routes: Option<Vec<GrpcRoute>>,
    /// If there might be more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListGrpcRoutesResponse {}


/// Response returned by the ListHttpRoutes method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations http routes list projects](ProjectLocationHttpRouteListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListHttpRoutesResponse {
    /// List of HttpRoute resources.
    #[serde(rename="httpRoutes")]
    
    pub http_routes: Option<Vec<HttpRoute>>,
    /// If there might be more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListHttpRoutesResponse {}


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


/// Response returned by the ListMeshes method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations meshes list projects](ProjectLocationMeshListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMeshesResponse {
    /// List of Mesh resources.
    
    pub meshes: Option<Vec<Mesh>>,
    /// If there might be more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListMeshesResponse {}


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


/// Response returned by the ListServiceBindings method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations service bindings list projects](ProjectLocationServiceBindingListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListServiceBindingsResponse {
    /// If there might be more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of ServiceBinding resources.
    #[serde(rename="serviceBindings")]
    
    pub service_bindings: Option<Vec<ServiceBinding>>,
}

impl client::ResponseResult for ListServiceBindingsResponse {}


/// Response returned by the ListTcpRoutes method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations tcp routes list projects](ProjectLocationTcpRouteListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTcpRoutesResponse {
    /// If there might be more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of TcpRoute resources.
    #[serde(rename="tcpRoutes")]
    
    pub tcp_routes: Option<Vec<TcpRoute>>,
}

impl client::ResponseResult for ListTcpRoutesResponse {}


/// Response returned by the ListTlsRoutes method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations tls routes list projects](ProjectLocationTlsRouteListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTlsRoutesResponse {
    /// If there might be more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of TlsRoute resources.
    #[serde(rename="tlsRoutes")]
    
    pub tls_routes: Option<Vec<TlsRoute>>,
}

impl client::ResponseResult for ListTlsRoutesResponse {}


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


/// Mesh represents a logical configuration grouping for workload to workload communication within a service mesh. Routes that point to mesh dictate how requests are routed within this logical mesh boundary.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations meshes create projects](ProjectLocationMeshCreateCall) (request)
/// * [locations meshes get projects](ProjectLocationMeshGetCall) (response)
/// * [locations meshes patch projects](ProjectLocationMeshPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Mesh {
    /// Output only. The timestamp when the resource was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. A free-text description of the resource. Max length 1024 characters.
    
    pub description: Option<String>,
    /// Optional. If set to a valid TCP port (1-65535), instructs the SIDECAR proxy to listen on the specified port of localhost (127.0.0.1) address. The SIDECAR proxy will expect all traffic to be redirected to this port regardless of its actual ip:port destination. If unset, a port '15001' is used as the interception port. This will is applicable only for sidecar proxy deployments.
    #[serde(rename="interceptionPort")]
    
    pub interception_port: Option<i32>,
    /// Optional. Set of label tags associated with the Mesh resource.
    
    pub labels: Option<HashMap<String, String>>,
    /// Required. Name of the Mesh resource. It matches pattern `projects/*/locations/global/meshes/`.
    
    pub name: Option<String>,
    /// Output only. Server-defined URL of this resource
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Output only. The timestamp when the resource was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Mesh {}
impl client::ResponseResult for Mesh {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations endpoint policies create projects](ProjectLocationEndpointPolicyCreateCall) (response)
/// * [locations endpoint policies delete projects](ProjectLocationEndpointPolicyDeleteCall) (response)
/// * [locations endpoint policies patch projects](ProjectLocationEndpointPolicyPatchCall) (response)
/// * [locations gateways create projects](ProjectLocationGatewayCreateCall) (response)
/// * [locations gateways delete projects](ProjectLocationGatewayDeleteCall) (response)
/// * [locations gateways patch projects](ProjectLocationGatewayPatchCall) (response)
/// * [locations grpc routes create projects](ProjectLocationGrpcRouteCreateCall) (response)
/// * [locations grpc routes delete projects](ProjectLocationGrpcRouteDeleteCall) (response)
/// * [locations grpc routes patch projects](ProjectLocationGrpcRoutePatchCall) (response)
/// * [locations http routes create projects](ProjectLocationHttpRouteCreateCall) (response)
/// * [locations http routes delete projects](ProjectLocationHttpRouteDeleteCall) (response)
/// * [locations http routes patch projects](ProjectLocationHttpRoutePatchCall) (response)
/// * [locations meshes create projects](ProjectLocationMeshCreateCall) (response)
/// * [locations meshes delete projects](ProjectLocationMeshDeleteCall) (response)
/// * [locations meshes patch projects](ProjectLocationMeshPatchCall) (response)
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
/// * [locations service bindings create projects](ProjectLocationServiceBindingCreateCall) (response)
/// * [locations service bindings delete projects](ProjectLocationServiceBindingDeleteCall) (response)
/// * [locations tcp routes create projects](ProjectLocationTcpRouteCreateCall) (response)
/// * [locations tcp routes delete projects](ProjectLocationTcpRouteDeleteCall) (response)
/// * [locations tcp routes patch projects](ProjectLocationTcpRoutePatchCall) (response)
/// * [locations tls routes create projects](ProjectLocationTlsRouteCreateCall) (response)
/// * [locations tls routes delete projects](ProjectLocationTlsRouteDeleteCall) (response)
/// * [locations tls routes patch projects](ProjectLocationTlsRoutePatchCall) (response)
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
/// * [locations edge cache keysets get iam policy projects](ProjectLocationEdgeCacheKeysetGetIamPolicyCall) (response)
/// * [locations edge cache keysets set iam policy projects](ProjectLocationEdgeCacheKeysetSetIamPolicyCall) (response)
/// * [locations edge cache origins get iam policy projects](ProjectLocationEdgeCacheOriginGetIamPolicyCall) (response)
/// * [locations edge cache origins set iam policy projects](ProjectLocationEdgeCacheOriginSetIamPolicyCall) (response)
/// * [locations edge cache services get iam policy projects](ProjectLocationEdgeCacheServiceGetIamPolicyCall) (response)
/// * [locations edge cache services set iam policy projects](ProjectLocationEdgeCacheServiceSetIamPolicyCall) (response)
/// * [locations endpoint policies get iam policy projects](ProjectLocationEndpointPolicyGetIamPolicyCall) (response)
/// * [locations endpoint policies set iam policy projects](ProjectLocationEndpointPolicySetIamPolicyCall) (response)
/// * [locations gateways get iam policy projects](ProjectLocationGatewayGetIamPolicyCall) (response)
/// * [locations gateways set iam policy projects](ProjectLocationGatewaySetIamPolicyCall) (response)
/// * [locations meshes get iam policy projects](ProjectLocationMeshGetIamPolicyCall) (response)
/// * [locations meshes set iam policy projects](ProjectLocationMeshSetIamPolicyCall) (response)
/// * [locations service bindings get iam policy projects](ProjectLocationServiceBindingGetIamPolicyCall) (response)
/// * [locations service bindings set iam policy projects](ProjectLocationServiceBindingSetIamPolicyCall) (response)
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


/// ServiceBinding is the resource that defines a Service Directory Service to be used in a BackendService resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations service bindings create projects](ProjectLocationServiceBindingCreateCall) (request)
/// * [locations service bindings get projects](ProjectLocationServiceBindingGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceBinding {
    /// Output only. The timestamp when the resource was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. A free-text description of the resource. Max length 1024 characters.
    
    pub description: Option<String>,
    /// Optional. Set of label tags associated with the ServiceBinding resource.
    
    pub labels: Option<HashMap<String, String>>,
    /// Required. Name of the ServiceBinding resource. It matches pattern `projects/*/locations/global/serviceBindings/service_binding_name`.
    
    pub name: Option<String>,
    /// Required. The full Service Directory Service name of the format projects/*/locations/*/namespaces/*/services/*
    
    pub service: Option<String>,
    /// Output only. The timestamp when the resource was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for ServiceBinding {}
impl client::ResponseResult for ServiceBinding {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations edge cache keysets set iam policy projects](ProjectLocationEdgeCacheKeysetSetIamPolicyCall) (request)
/// * [locations edge cache origins set iam policy projects](ProjectLocationEdgeCacheOriginSetIamPolicyCall) (request)
/// * [locations edge cache services set iam policy projects](ProjectLocationEdgeCacheServiceSetIamPolicyCall) (request)
/// * [locations endpoint policies set iam policy projects](ProjectLocationEndpointPolicySetIamPolicyCall) (request)
/// * [locations gateways set iam policy projects](ProjectLocationGatewaySetIamPolicyCall) (request)
/// * [locations meshes set iam policy projects](ProjectLocationMeshSetIamPolicyCall) (request)
/// * [locations service bindings set iam policy projects](ProjectLocationServiceBindingSetIamPolicyCall) (request)
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


/// TcpRoute is the resource defining how TCP traffic should be routed by a Mesh/Gateway resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations tcp routes create projects](ProjectLocationTcpRouteCreateCall) (request)
/// * [locations tcp routes get projects](ProjectLocationTcpRouteGetCall) (response)
/// * [locations tcp routes patch projects](ProjectLocationTcpRoutePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TcpRoute {
    /// Output only. The timestamp when the resource was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. A free-text description of the resource. Max length 1024 characters.
    
    pub description: Option<String>,
    /// Optional. Gateways defines a list of gateways this TcpRoute is attached to, as one of the routing rules to route the requests served by the gateway. Each gateway reference should match the pattern: `projects/*/locations/global/gateways/`
    
    pub gateways: Option<Vec<String>>,
    /// Optional. Set of label tags associated with the TcpRoute resource.
    
    pub labels: Option<HashMap<String, String>>,
    /// Optional. Meshes defines a list of meshes this TcpRoute is attached to, as one of the routing rules to route the requests served by the mesh. Each mesh reference should match the pattern: `projects/*/locations/global/meshes/` The attached Mesh should be of a type SIDECAR
    
    pub meshes: Option<Vec<String>>,
    /// Required. Name of the TcpRoute resource. It matches pattern `projects/*/locations/global/tcpRoutes/tcp_route_name>`.
    
    pub name: Option<String>,
    /// Required. Rules that define how traffic is routed and handled. At least one RouteRule must be supplied. If there are multiple rules then the action taken will be the first rule to match.
    
    pub rules: Option<Vec<TcpRouteRouteRule>>,
    /// Output only. Server-defined URL of this resource
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Output only. The timestamp when the resource was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for TcpRoute {}
impl client::ResponseResult for TcpRoute {}


/// The specifications for routing traffic and applying associated policies.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TcpRouteRouteAction {
    /// Optional. The destination services to which traffic should be forwarded. At least one destination service is required.
    
    pub destinations: Option<Vec<TcpRouteRouteDestination>>,
    /// Optional. If true, Router will use the destination IP and port of the original connection as the destination of the request. Default is false.
    #[serde(rename="originalDestination")]
    
    pub original_destination: Option<bool>,
}

impl client::Part for TcpRouteRouteAction {}


/// Describe the destination for traffic to be routed to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TcpRouteRouteDestination {
    /// Required. The URL of a BackendService to route traffic to.
    #[serde(rename="serviceName")]
    
    pub service_name: Option<String>,
    /// Optional. Specifies the proportion of requests forwarded to the backend referenced by the serviceName field. This is computed as: weight/Sum(weights in this destination list). For non-zero values, there may be some epsilon from the exact proportion defined here depending on the precision an implementation supports. If only one serviceName is specified and it has a weight greater than 0, 100% of the traffic is forwarded to that backend. If weights are specified for any one service name, they need to be specified for all of them. If weights are unspecified for all services, then, traffic is distributed in equal proportions to all of them.
    
    pub weight: Option<i32>,
}

impl client::Part for TcpRouteRouteDestination {}


/// RouteMatch defines the predicate used to match requests to a given action. Multiple match types are "OR"ed for evaluation. If no routeMatch field is specified, this rule will unconditionally match traffic.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TcpRouteRouteMatch {
    /// Required. Must be specified in the CIDR range format. A CIDR range consists of an IP Address and a prefix length to construct the subnet mask. By default, the prefix length is 32 (i.e. matches a single IP address). Only IPV4 addresses are supported. Examples: "10.0.0.1" - matches against this exact IP address. "10.0.0.0/8" - matches against any IP address within the 10.0.0.0 subnet and 255.255.255.0 mask. "0.0.0.0/0" - matches against any IP address'.
    
    pub address: Option<String>,
    /// Required. Specifies the destination port to match against.
    
    pub port: Option<String>,
}

impl client::Part for TcpRouteRouteMatch {}


/// Specifies how to match traffic and how to route traffic when traffic is matched.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TcpRouteRouteRule {
    /// Required. The detailed rule defining how to route matched traffic.
    
    pub action: Option<TcpRouteRouteAction>,
    /// Optional. RouteMatch defines the predicate used to match requests to a given action. Multiple match types are "OR"ed for evaluation. If no routeMatch field is specified, this rule will unconditionally match traffic.
    
    pub matches: Option<Vec<TcpRouteRouteMatch>>,
}

impl client::Part for TcpRouteRouteRule {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations edge cache keysets test iam permissions projects](ProjectLocationEdgeCacheKeysetTestIamPermissionCall) (request)
/// * [locations edge cache origins test iam permissions projects](ProjectLocationEdgeCacheOriginTestIamPermissionCall) (request)
/// * [locations edge cache services test iam permissions projects](ProjectLocationEdgeCacheServiceTestIamPermissionCall) (request)
/// * [locations endpoint policies test iam permissions projects](ProjectLocationEndpointPolicyTestIamPermissionCall) (request)
/// * [locations gateways test iam permissions projects](ProjectLocationGatewayTestIamPermissionCall) (request)
/// * [locations meshes test iam permissions projects](ProjectLocationMeshTestIamPermissionCall) (request)
/// * [locations service bindings test iam permissions projects](ProjectLocationServiceBindingTestIamPermissionCall) (request)
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
/// * [locations edge cache keysets test iam permissions projects](ProjectLocationEdgeCacheKeysetTestIamPermissionCall) (response)
/// * [locations edge cache origins test iam permissions projects](ProjectLocationEdgeCacheOriginTestIamPermissionCall) (response)
/// * [locations edge cache services test iam permissions projects](ProjectLocationEdgeCacheServiceTestIamPermissionCall) (response)
/// * [locations endpoint policies test iam permissions projects](ProjectLocationEndpointPolicyTestIamPermissionCall) (response)
/// * [locations gateways test iam permissions projects](ProjectLocationGatewayTestIamPermissionCall) (response)
/// * [locations meshes test iam permissions projects](ProjectLocationMeshTestIamPermissionCall) (response)
/// * [locations service bindings test iam permissions projects](ProjectLocationServiceBindingTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// TlsRoute defines how traffic should be routed based on SNI and other matching L3 attributes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations tls routes create projects](ProjectLocationTlsRouteCreateCall) (request)
/// * [locations tls routes get projects](ProjectLocationTlsRouteGetCall) (response)
/// * [locations tls routes patch projects](ProjectLocationTlsRoutePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TlsRoute {
    /// Output only. The timestamp when the resource was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. A free-text description of the resource. Max length 1024 characters.
    
    pub description: Option<String>,
    /// Optional. Gateways defines a list of gateways this TlsRoute is attached to, as one of the routing rules to route the requests served by the gateway. Each gateway reference should match the pattern: `projects/*/locations/global/gateways/`
    
    pub gateways: Option<Vec<String>>,
    /// Optional. Meshes defines a list of meshes this TlsRoute is attached to, as one of the routing rules to route the requests served by the mesh. Each mesh reference should match the pattern: `projects/*/locations/global/meshes/` The attached Mesh should be of a type SIDECAR
    
    pub meshes: Option<Vec<String>>,
    /// Required. Name of the TlsRoute resource. It matches pattern `projects/*/locations/global/tlsRoutes/tls_route_name>`.
    
    pub name: Option<String>,
    /// Required. Rules that define how traffic is routed and handled. At least one RouteRule must be supplied. If there are multiple rules then the action taken will be the first rule to match.
    
    pub rules: Option<Vec<TlsRouteRouteRule>>,
    /// Output only. Server-defined URL of this resource
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Output only. The timestamp when the resource was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for TlsRoute {}
impl client::ResponseResult for TlsRoute {}


/// The specifications for routing traffic and applying associated policies.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TlsRouteRouteAction {
    /// Required. The destination services to which traffic should be forwarded. At least one destination service is required.
    
    pub destinations: Option<Vec<TlsRouteRouteDestination>>,
}

impl client::Part for TlsRouteRouteAction {}


/// Describe the destination for traffic to be routed to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TlsRouteRouteDestination {
    /// Required. The URL of a BackendService to route traffic to.
    #[serde(rename="serviceName")]
    
    pub service_name: Option<String>,
    /// Optional. Specifies the proportion of requests forwareded to the backend referenced by the service_name field. This is computed as: weight/Sum(weights in destinations) Weights in all destinations does not need to sum up to 100.
    
    pub weight: Option<i32>,
}

impl client::Part for TlsRouteRouteDestination {}


/// RouteMatch defines the predicate used to match requests to a given action. Multiple match types are "AND"ed for evaluation. If no routeMatch field is specified, this rule will unconditionally match traffic.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TlsRouteRouteMatch {
    /// Optional. ALPN (Application-Layer Protocol Negotiation) to match against. Examples: "http/1.1", "h2". At least one of sni_host and alpn is required. Up to 5 alpns across all matches can be set.
    
    pub alpn: Option<Vec<String>>,
    /// Optional. SNI (server name indicator) to match against. SNI will be matched against all wildcard domains, i.e. www.example.com will be first matched against www.example.com, then *.example.com, then *.com. Partial wildcards are not supported, and values like *w.example.com are invalid. At least one of sni_host and alpn is required. Up to 5 sni hosts across all matches can be set.
    #[serde(rename="sniHost")]
    
    pub sni_host: Option<Vec<String>>,
}

impl client::Part for TlsRouteRouteMatch {}


/// Specifies how to match traffic and how to route traffic when traffic is matched.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TlsRouteRouteRule {
    /// Required. The detailed rule defining how to route matched traffic.
    
    pub action: Option<TlsRouteRouteAction>,
    /// Required. RouteMatch defines the predicate used to match requests to a given action. Multiple match types are "OR"ed for evaluation.
    
    pub matches: Option<Vec<TlsRouteRouteMatch>>,
}

impl client::Part for TlsRouteRouteRule {}


/// Specification of a port-based selector.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TrafficPortSelector {
    /// Optional. A list of ports. Can be port numbers or port range (example, [80-90] specifies all ports from 80 to 90, including 80 and 90) or named ports or * to specify all ports. If the list is empty, all ports are selected.
    
    pub ports: Option<Vec<String>>,
}

impl client::Part for TrafficPortSelector {}


