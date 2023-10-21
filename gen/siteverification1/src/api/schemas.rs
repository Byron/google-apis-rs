use super::*;
/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get token web resource](WebResourceGetTokenCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SiteVerificationWebResourceGettokenRequest {
    /// The site for which a verification token will be generated.
    
    pub site: Option<SiteVerificationWebResourceGettokenRequestSite>,
    /// The verification method that will be used to verify this site. For sites, 'FILE' or 'META' methods may be used. For domains, only 'DNS' may be used.
    #[serde(rename="verificationMethod")]
    
    pub verification_method: Option<String>,
}

impl client::RequestValue for SiteVerificationWebResourceGettokenRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get token web resource](WebResourceGetTokenCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SiteVerificationWebResourceGettokenResponse {
    /// The verification method to use in conjunction with this token. For FILE, the token should be placed in the top-level directory of the site, stored inside a file of the same name. For META, the token should be placed in the HEAD tag of the default page that is loaded for the site. For DNS, the token should be placed in a TXT record of the domain.
    
    pub method: Option<String>,
    /// The verification token. The token must be placed appropriately in order for verification to succeed.
    
    pub token: Option<String>,
}

impl client::ResponseResult for SiteVerificationWebResourceGettokenResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list web resource](WebResourceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SiteVerificationWebResourceListResponse {
    /// The list of sites that are owned by the authenticated user.
    
    pub items: Option<Vec<SiteVerificationWebResourceResource>>,
}

impl client::ResponseResult for SiteVerificationWebResourceListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get web resource](WebResourceGetCall) (response)
/// * [insert web resource](WebResourceInsertCall) (request|response)
/// * [patch web resource](WebResourcePatchCall) (request|response)
/// * [update web resource](WebResourceUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SiteVerificationWebResourceResource {
    /// The string used to identify this site. This value should be used in the "id" portion of the REST URL for the Get, Update, and Delete operations.
    
    pub id: Option<String>,
    /// The email addresses of all verified owners.
    
    pub owners: Option<Vec<String>>,
    /// The address and type of a site that is verified or will be verified.
    
    pub site: Option<SiteVerificationWebResourceResourceSite>,
}

impl client::RequestValue for SiteVerificationWebResourceResource {}
impl client::ResponseResult for SiteVerificationWebResourceResource {}


/// The site for which a verification token will be generated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SiteVerificationWebResourceGettokenRequestSite {
    /// The site identifier. If the type is set to SITE, the identifier is a URL. If the type is set to INET_DOMAIN, the site identifier is a domain name.
    
    pub identifier: Option<String>,
    /// The type of resource to be verified. Can be SITE or INET_DOMAIN (domain name).
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for SiteVerificationWebResourceGettokenRequestSite {}
impl client::Part for SiteVerificationWebResourceGettokenRequestSite {}


/// The address and type of a site that is verified or will be verified.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SiteVerificationWebResourceResourceSite {
    /// The site identifier. If the type is set to SITE, the identifier is a URL. If the type is set to INET_DOMAIN, the site identifier is a domain name.
    
    pub identifier: Option<String>,
    /// The site type. Can be SITE or INET_DOMAIN (domain name).
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for SiteVerificationWebResourceResourceSite {}
impl client::Part for SiteVerificationWebResourceResourceSite {}


