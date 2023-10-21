use super::*;
/// This message defines attributes associated with API operations, such as a network API request. The terminology is based on the conventions used by Google APIs, Istio, and OpenAPI.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Api {
    /// The API operation name. For gRPC requests, it is the fully qualified API method name, such as "google.pubsub.v1.Publisher.Publish". For OpenAPI requests, it is the `operationId`, such as "getPet".
    
    pub operation: Option<String>,
    /// The API protocol used for sending the request, such as "http", "https", "grpc", or "internal".
    
    pub protocol: Option<String>,
    /// The API service name. It is a logical identifier for a networked API, such as "pubsub.googleapis.com". The naming syntax depends on the API management system being used for handling the request.
    
    pub service: Option<String>,
    /// The API version associated with the API operation above, such as "v1" or "v1alpha1".
    
    pub version: Option<String>,
}

impl client::Part for Api {}


/// This message defines the standard attribute vocabulary for Google APIs. An attribute is a piece of metadata that describes an activity on a network service. For example, the size of an HTTP request, or the status code of an HTTP response. Each attribute has a type and a name, which is logically defined as a proto message field in `AttributeContext`. The field type becomes the attribute type, and the field path becomes the attribute name. For example, the attribute `source.ip` maps to field `AttributeContext.source.ip`. This message definition is guaranteed not to have any wire breaking change. So you can use it directly for passing attributes across different systems. NOTE: Different system may generate different subset of attributes. Please verify the system specification before relying on an attribute generated a system.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AttributeContext {
    /// Represents an API operation that is involved to a network activity.
    
    pub api: Option<Api>,
    /// The destination of a network activity, such as accepting a TCP connection. In a multi hop network activity, the destination represents the receiver of the last hop.
    
    pub destination: Option<Peer>,
    /// Supports extensions for advanced use cases, such as logs and metrics.
    
    pub extensions: Option<Vec<HashMap<String, json::Value>>>,
    /// The origin of a network activity. In a multi hop network activity, the origin represents the sender of the first hop. For the first hop, the `source` and the `origin` must have the same content.
    
    pub origin: Option<Peer>,
    /// Represents a network request, such as an HTTP request.
    
    pub request: Option<Request>,
    /// Represents a target resource that is involved with a network activity. If multiple resources are involved with an activity, this must be the primary one.
    
    pub resource: Option<Resource>,
    /// Represents a network response, such as an HTTP response.
    
    pub response: Option<Response>,
    /// The source of a network activity, such as starting a TCP connection. In a multi hop network activity, the source represents the sender of the last hop.
    
    pub source: Option<Peer>,
}

impl client::Part for AttributeContext {}


/// This message defines request authentication attributes. Terminology is based on the JSON Web Token (JWT) standard, but the terms also correlate to concepts in other standards.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Auth {
    /// A list of access level resource names that allow resources to be accessed by authenticated requester. It is part of Secure GCP processing for the incoming request. An access level string has the format: "//{api_service_name}/accessPolicies/{policy_id}/accessLevels/{short_name}" Example: "//accesscontextmanager.googleapis.com/accessPolicies/MY_POLICY_ID/accessLevels/MY_LEVEL"
    #[serde(rename="accessLevels")]
    
    pub access_levels: Option<Vec<String>>,
    /// The intended audience(s) for this authentication information. Reflects the audience (`aud`) claim within a JWT. The audience value(s) depends on the `issuer`, but typically include one or more of the following pieces of information: * The services intended to receive the credential. For example, ["https://pubsub.googleapis.com/", "https://storage.googleapis.com/"]. * A set of service-based scopes. For example, ["https://www.googleapis.com/auth/cloud-platform"]. * The client id of an app, such as the Firebase project id for JWTs from Firebase Auth. Consult the documentation for the credential issuer to determine the information provided.
    
    pub audiences: Option<Vec<String>>,
    /// Structured claims presented with the credential. JWTs include `{key: value}` pairs for standard and private claims. The following is a subset of the standard required and optional claims that would typically be presented for a Google-based JWT: {'iss': 'accounts.google.com', 'sub': '113289723416554971153', 'aud': ['123456789012', 'pubsub.googleapis.com'], 'azp': '123456789012.apps.googleusercontent.com', 'email': 'jsmith@example.com', 'iat': 1353601026, 'exp': 1353604926} SAML assertions are similarly specified, but with an identity provider dependent structure.
    
    pub claims: Option<HashMap<String, json::Value>>,
    /// The authorized presenter of the credential. Reflects the optional Authorized Presenter (`azp`) claim within a JWT or the OAuth client id. For example, a Google Cloud Platform client id looks as follows: "123456789012.apps.googleusercontent.com".
    
    pub presenter: Option<String>,
    /// The authenticated principal. Reflects the issuer (`iss`) and subject (`sub`) claims within a JWT. The issuer and subject should be `/` delimited, with `/` percent-encoded within the subject fragment. For Google accounts, the principal format is: "https://accounts.google.com/{id}"
    
    pub principal: Option<String>,
}

impl client::Part for Auth {}


/// Request message for the Check method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [check services](ServiceCheckCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CheckRequest {
    /// Describes attributes about the operation being executed by the service.
    
    pub attributes: Option<AttributeContext>,
    /// Optional. Contains a comma-separated list of flags.
    
    pub flags: Option<String>,
    /// Describes the resources and the policies applied to each resource.
    
    pub resources: Option<Vec<ResourceInfo>>,
    /// Specifies the version of the service configuration that should be used to process the request. Must not be empty. Set this field to 'latest' to specify using the latest configuration.
    #[serde(rename="serviceConfigId")]
    
    pub service_config_id: Option<String>,
}

impl client::RequestValue for CheckRequest {}


/// Response message for the Check method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [check services](ServiceCheckCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CheckResponse {
    /// Returns a set of request contexts generated from the `CheckRequest`.
    
    pub headers: Option<HashMap<String, String>>,
    /// Operation is allowed when this field is not set. Any non-'OK' status indicates a denial; google.rpc.Status.details would contain additional details about the denial.
    
    pub status: Option<Status>,
}

impl client::ResponseResult for CheckResponse {}


/// This message defines attributes for a node that handles a network request. The node can be either a service or an application that sends, forwards, or receives the request. Service peers should fill in `principal` and `labels` as appropriate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Peer {
    /// The IP address of the peer.
    
    pub ip: Option<String>,
    /// The labels associated with the peer.
    
    pub labels: Option<HashMap<String, String>>,
    /// The network port of the peer.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub port: Option<i64>,
    /// The identity of this peer. Similar to `Request.auth.principal`, but relative to the peer instead of the request. For example, the identity associated with a load balancer that forwarded the request.
    
    pub principal: Option<String>,
    /// The CLDR country/region code associated with the above IP address. If the IP address is private, the `region_code` should reflect the physical location where this peer is running.
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
}

impl client::Part for Peer {}


/// Request message for the Report method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [report services](ServiceReportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportRequest {
    /// Describes the list of operations to be reported. Each operation is represented as an AttributeContext, and contains all attributes around an API access.
    
    pub operations: Option<Vec<AttributeContext>>,
    /// Specifies the version of the service configuration that should be used to process the request. Must not be empty. Set this field to 'latest' to specify using the latest configuration.
    #[serde(rename="serviceConfigId")]
    
    pub service_config_id: Option<String>,
}

impl client::RequestValue for ReportRequest {}


/// Response message for the Report method. If the request contains any invalid data, the server returns an RPC error.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [report services](ServiceReportCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportResponse { _never_set: Option<bool> }

impl client::ResponseResult for ReportResponse {}


/// This message defines attributes for an HTTP request. If the actual request is not an HTTP request, the runtime system should try to map the actual request to an equivalent HTTP request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Request {
    /// The request authentication. May be absent for unauthenticated requests. Derived from the HTTP request `Authorization` header or equivalent.
    
    pub auth: Option<Auth>,
    /// The HTTP request headers. If multiple headers share the same key, they must be merged according to the HTTP spec. All header keys must be lowercased, because HTTP header keys are case-insensitive.
    
    pub headers: Option<HashMap<String, String>>,
    /// The HTTP request `Host` header value.
    
    pub host: Option<String>,
    /// The unique ID for a request, which can be propagated to downstream systems. The ID should have low probability of collision within a single day for a specific service.
    
    pub id: Option<String>,
    /// The HTTP request method, such as `GET`, `POST`.
    
    pub method: Option<String>,
    /// The HTTP URL path, excluding the query parameters.
    
    pub path: Option<String>,
    /// The network protocol used with the request, such as "http/1.1", "spdy/3", "h2", "h2c", "webrtc", "tcp", "udp", "quic". See https://www.iana.org/assignments/tls-extensiontype-values/tls-extensiontype-values.xhtml#alpn-protocol-ids for details.
    
    pub protocol: Option<String>,
    /// The HTTP URL query in the format of `name1=value1&name2=value2`, as it appears in the first line of the HTTP request. No decoding is performed.
    
    pub query: Option<String>,
    /// A special parameter for request reason. It is used by security systems to associate auditing information with a request.
    
    pub reason: Option<String>,
    /// The HTTP URL scheme, such as `http` and `https`.
    
    pub scheme: Option<String>,
    /// The HTTP request size in bytes. If unknown, it must be -1.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub size: Option<i64>,
    /// The timestamp when the `destination` service receives the last byte of the request.
    
    pub time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for Request {}


/// This message defines core attributes for a resource. A resource is an addressable (named) entity provided by the destination service. For example, a file stored on a network storage service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Resource {
    /// Annotations is an unstructured key-value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. More info: https://kubernetes.io/docs/user-guide/annotations
    
    pub annotations: Option<HashMap<String, String>>,
    /// Output only. The timestamp when the resource was created. This may be either the time creation was initiated or when it was completed.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The timestamp when the resource was deleted. If the resource is not deleted, this must be empty.
    #[serde(rename="deleteTime")]
    
    pub delete_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Mutable. The display name set by clients. Must be <= 63 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. An opaque value that uniquely identifies a version or generation of a resource. It can be used to confirm that the client and server agree on the ordering of a resource being written.
    
    pub etag: Option<String>,
    /// The labels or tags on the resource, such as AWS resource tags and Kubernetes resource labels.
    
    pub labels: Option<HashMap<String, String>>,
    /// Immutable. The location of the resource. The location encoding is specific to the service provider, and new encoding may be introduced as the service evolves. For Google Cloud products, the encoding is what is used by Google Cloud APIs, such as `us-east1`, `aws-us-east-1`, and `azure-eastus2`. The semantics of `location` is identical to the `cloud.googleapis.com/location` label used by some Google Cloud APIs.
    
    pub location: Option<String>,
    /// The stable identifier (name) of a resource on the `service`. A resource can be logically identified as "//{resource.service}/{resource.name}". The differences between a resource name and a URI are: * Resource name is a logical identifier, independent of network protocol and API version. For example, `//pubsub.googleapis.com/projects/123/topics/news-feed`. * URI often includes protocol and version information, so it can be used directly by applications. For example, `https://pubsub.googleapis.com/v1/projects/123/topics/news-feed`. See https://cloud.google.com/apis/design/resource_names for details.
    
    pub name: Option<String>,
    /// The name of the service that this resource belongs to, such as `pubsub.googleapis.com`. The service may be different from the DNS hostname that actually serves the request.
    
    pub service: Option<String>,
    /// The type of the resource. The syntax is platform-specific because different platforms define their resources differently. For Google APIs, the type format must be "{service}/{kind}", such as "pubsub.googleapis.com/Topic".
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The unique identifier of the resource. UID is unique in the time and space for this resource within the scope of the service. It is typically generated by the server on successful creation of a resource and must not be changed. UID is used to uniquely identify resources with resource name reuses. This should be a UUID4.
    
    pub uid: Option<String>,
    /// Output only. The timestamp when the resource was last updated. Any change to the resource made by users must refresh this value. Changes to a resource made by the service should refresh this value.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for Resource {}


/// Describes a resource referenced in the request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceInfo {
    /// Optional. The identifier of the container of this resource. For Google Cloud APIs, the resource container must be one of the following formats: - `projects/` - `folders/` - `organizations/` For the policy enforcement on the container level (VPCSC and Location Policy check), this field takes precedence on the container extracted from name when presents.
    
    pub container: Option<String>,
    /// Optional. The location of the resource. The value must be a valid zone, region or multiregion. For example: "europe-west4" or "northamerica-northeast1-a"
    
    pub location: Option<String>,
    /// The name of the resource referenced in the request.
    
    pub name: Option<String>,
    /// The resource permission needed for this request. The format must be "{service}/{plural}.{verb}".
    
    pub permission: Option<String>,
    /// The resource type in the format of "{service}/{kind}".
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for ResourceInfo {}


/// This message defines attributes for a typical network response. It generally models semantics of an HTTP response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    /// The amount of time it takes the backend service to fully respond to a request. Measured from when the destination service starts to send the request to the backend until when the destination service receives the complete response from the backend.
    #[serde(rename="backendLatency")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub backend_latency: Option<client::chrono::Duration>,
    /// The HTTP response status code, such as `200` and `404`.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub code: Option<i64>,
    /// The HTTP response headers. If multiple headers share the same key, they must be merged according to HTTP spec. All header keys must be lowercased, because HTTP header keys are case-insensitive.
    
    pub headers: Option<HashMap<String, String>>,
    /// The HTTP response size in bytes. If unknown, it must be -1.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub size: Option<i64>,
    /// The timestamp when the `destination` service sends the last byte of the response.
    
    pub time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for Response {}


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


