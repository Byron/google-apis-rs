use super::*;
/// AMP URL response for a requested URL.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch get amp urls](AmpUrlBatchGetCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AmpUrl {
    /// The AMP URL pointing to the publisher's web server.
    #[serde(rename="ampUrl")]
    
    pub amp_url: Option<String>,
    /// The [AMP Cache URL](https://developers.google.com/amp/cache/overview#amp-cache-url-format) pointing to the cached document in the Google AMP Cache.
    #[serde(rename="cdnAmpUrl")]
    
    pub cdn_amp_url: Option<String>,
    /// The original non-AMP URL.
    #[serde(rename="originalUrl")]
    
    pub original_url: Option<String>,
}

impl client::Resource for AmpUrl {}


/// AMP URL Error resource for a requested URL that couldn't be found.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AmpUrlError {
    /// The error code of an API call.
    #[serde(rename="errorCode")]
    
    pub error_code: Option<AmpUrlErrorErrorCodeEnum>,
    /// An optional descriptive error message.
    #[serde(rename="errorMessage")]
    
    pub error_message: Option<String>,
    /// The original non-AMP URL.
    #[serde(rename="originalUrl")]
    
    pub original_url: Option<String>,
}

impl client::Part for AmpUrlError {}


/// AMP URL request for a batch of URLs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch get amp urls](AmpUrlBatchGetCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchGetAmpUrlsRequest {
    /// The lookup_strategy being requested.
    #[serde(rename="lookupStrategy")]
    
    pub lookup_strategy: Option<BatchGetAmpUrlsRequestLookupStrategyEnum>,
    /// List of URLs to look up for the paired AMP URLs. The URLs are case-sensitive. Up to 50 URLs per lookup (see [Usage Limits](https://developers.google.com/amp/cache/reference/limits)).
    
    pub urls: Option<Vec<String>>,
}

impl client::RequestValue for BatchGetAmpUrlsRequest {}


/// Batch AMP URL response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch get amp urls](AmpUrlBatchGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchGetAmpUrlsResponse {
    /// For each URL in BatchAmpUrlsRequest, the URL response. The response might not be in the same order as URLs in the batch request. If BatchAmpUrlsRequest contains duplicate URLs, AmpUrl is generated only once.
    #[serde(rename="ampUrls")]
    
    pub amp_urls: Option<Vec<AmpUrl>>,
    /// The errors for requested URLs that have no AMP URL.
    #[serde(rename="urlErrors")]
    
    pub url_errors: Option<Vec<AmpUrlError>>,
}

impl client::ResponseResult for BatchGetAmpUrlsResponse {}


