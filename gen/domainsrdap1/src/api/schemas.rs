use super::*;
/// Message that represents an arbitrary HTTP body. It should only be used for payload formats that can’t be represented as JSON, such as raw binary or an HTML page. This message can be used both in streaming and non-streaming API methods in the request as well as the response. It can be used as a top-level request field, which is convenient if one wants to extract parameters from either the URL or HTTP template into the request fields and also want access to the raw HTTP body. Example: message GetResourceRequest { // A unique request id. string request_id = 1; // The raw HTTP body is bound to this field. google.api.HttpBody http_body = 2; } service ResourceService { rpc GetResource(GetResourceRequest) returns (google.api.HttpBody); rpc UpdateResource(google.api.HttpBody) returns (google.protobuf.Empty); } Example with streaming methods: service CaldavService { rpc GetCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); rpc UpdateCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); } Use of this type only changes how the request and response bodies are handled, all other features will continue to work unchanged.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get domain](DomainGetCall) (response)
/// * [get help](MethodGetHelpCall) (response)
/// * [get ip](MethodGetIpCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpBody {
    /// The HTTP Content-Type header value specifying the content type of the body.
    #[serde(rename="contentType")]
    
    pub content_type: Option<String>,
    /// The HTTP request/response body as raw binary.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// Application specific response metadata. Must be set in the first response for streaming APIs.
    
    pub extensions: Option<Vec<HashMap<String, json::Value>>>,
}

impl client::ResponseResult for HttpBody {}


/// Links object defined in [section 4.2 of RFC 7483](https://tools.ietf.org/html/rfc7483#section-4.2).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    /// Target URL of a link. Example: "http://example.com/previous".
    
    pub href: Option<String>,
    /// Language code of a link. Example: "en".
    
    pub hreflang: Option<String>,
    /// Media type of the link destination. Example: "screen".
    
    pub media: Option<String>,
    /// Relation type of a link. Example: "previous".
    
    pub rel: Option<String>,
    /// Title of this link. Example: "title".
    
    pub title: Option<String>,
    /// Content type of the link. Example: "application/json".
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// URL giving context for the link. Example: "http://example.com/current".
    
    pub value: Option<String>,
}

impl client::Part for Link {}


/// Notices object defined in [section 4.3 of RFC 7483](https://tools.ietf.org/html/rfc7483#section-4.3).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Notice {
    /// Description of the notice.
    
    pub description: Option<Vec<String>>,
    /// Link to a document containing more information.
    
    pub links: Option<Vec<Link>>,
    /// Title of a notice. Example: "Terms of Service".
    
    pub title: Option<String>,
    /// Type values defined in [section 10.2.1 of RFC 7483](https://tools.ietf.org/html/rfc7483#section-10.2.1) specific to a whole response: "result set truncated due to authorization", "result set truncated due to excessive load", "result set truncated due to unexplainable reasons".
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Notice {}


/// Response to a general RDAP query.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get autnum](AutnumGetCall) (response)
/// * [get entity](EntityGetCall) (response)
/// * [get ip](IpGetCall) (response)
/// * [get nameserver](NameserverGetCall) (response)
/// * [get domains](MethodGetDomainCall) (response)
/// * [get entities](MethodGetEntityCall) (response)
/// * [get nameservers](MethodGetNameserverCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RdapResponse {
    /// Error description.
    
    pub description: Option<Vec<String>>,
    /// Error HTTP code. Example: "501".
    #[serde(rename="errorCode")]
    
    pub error_code: Option<i32>,
    /// HTTP response with content type set to "application/json+rdap".
    #[serde(rename="jsonResponse")]
    
    pub json_response: Option<HttpBody>,
    /// Error language code. Error response info fields are defined in [section 6 of RFC 7483](https://tools.ietf.org/html/rfc7483#section-6).
    
    pub lang: Option<String>,
    /// Notices applying to this response.
    
    pub notices: Option<Vec<Notice>>,
    /// RDAP conformance level.
    #[serde(rename="rdapConformance")]
    
    pub rdap_conformance: Option<Vec<String>>,
    /// Error title.
    
    pub title: Option<String>,
}

impl client::ResponseResult for RdapResponse {}


