use super::*;
/// Describes an android app asset.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidAppAsset {
    /// Because there is no global enforcement of package name uniqueness, we also require a signing certificate, which in combination with the package name uniquely identifies an app. Some apps' signing keys are rotated, so they may be signed by different keys over time. We treat these as distinct assets, since we use (package name, cert) as the unique ID. This should not normally pose any problems as both versions of the app will make the same or similar statements. Other assets making statements about the app will have to be updated when a key is rotated, however. (Note that the syntaxes for publishing and querying for statements contain syntactic sugar to easily let you specify apps that are known by multiple certificates.) REQUIRED
    
    pub certificate: Option<CertificateInfo>,
    /// Android App assets are naturally identified by their Java package name. For example, the Google Maps app uses the package name `com.google.android.apps.maps`. REQUIRED
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
}

impl client::Part for AndroidAppAsset {}


/// Uniquely identifies an asset. A digital asset is an identifiable and addressable online entity that typically provides some service or content. Examples of assets are websites, Android apps, Twitter feeds, and Plus Pages.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Asset {
    /// Set if this is an Android App asset.
    #[serde(rename="androidApp")]
    
    pub android_app: Option<AndroidAppAsset>,
    /// Set if this is a web asset.
    
    pub web: Option<WebAsset>,
}

impl client::Part for Asset {}


/// Message used to check for the existence of multiple digital asset links within a single RPC.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [bulk check assetlinks](AssetlinkBulkCheckCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BulkCheckRequest {
    /// Same configuration as in Check request, all statements checks will use same configurations.
    #[serde(rename="allowGoogleInternalDataSources")]
    
    pub allow_google_internal_data_sources: Option<bool>,
    /// If specified, will be used in any given template statement that doesn’t specify a relation.
    #[serde(rename="defaultRelation")]
    
    pub default_relation: Option<String>,
    /// If specified, will be used in any given template statement that doesn’t specify a source.
    #[serde(rename="defaultSource")]
    
    pub default_source: Option<Asset>,
    /// If specified, will be used in any given template statement that doesn’t specify a target.
    #[serde(rename="defaultTarget")]
    
    pub default_target: Option<Asset>,
    /// Same configuration as in Check request, all statements checks will use same configurations.
    #[serde(rename="skipCacheLookup")]
    
    pub skip_cache_lookup: Option<bool>,
    /// List of statements to check. For each statement, you can omit a field if the corresponding default_* field below was supplied. Minimum 1 statement; maximum 1,000 statements. Any additional statements will be ignored.
    
    pub statements: Option<Vec<StatementTemplate>>,
}

impl client::RequestValue for BulkCheckRequest {}


/// Response for BulkCheck call. Results are sent in a list in the same order in which they were sent. Individual check errors are described in the appropriate check_results entry. If the entire call fails, the response will include a bulk_error_code field describing the error.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [bulk check assetlinks](AssetlinkBulkCheckCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BulkCheckResponse {
    /// Error code for the entire request. Present only if the entire request failed. Individual check errors will not trigger the presence of this field.
    #[serde(rename="bulkErrorCode")]
    
    pub bulk_error_code: Option<BulkCheckResponseBulkErrorCodeEnum>,
    /// List of results for each check request. Results are returned in the same order in which they were sent in the request.
    #[serde(rename="checkResults")]
    
    pub check_results: Option<Vec<CheckResponse>>,
}

impl client::ResponseResult for BulkCheckResponse {}


/// Describes an X509 certificate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CertificateInfo {
    /// The uppercase SHA-265 fingerprint of the certificate. From the PEM certificate, it can be acquired like this: $ keytool -printcert -file $CERTFILE | grep SHA256: SHA256: 14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83: \ 42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5 or like this: $ openssl x509 -in $CERTFILE -noout -fingerprint -sha256 SHA256 Fingerprint=14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64: \ 16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5 In this example, the contents of this field would be `14:6D:E9:83:C5:73: 06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF: 44:E5`. If these tools are not available to you, you can convert the PEM certificate into the DER format, compute the SHA-256 hash of that string and represent the result as a hexstring (that is, uppercase hexadecimal representations of each octet, separated by colons).
    #[serde(rename="sha256Fingerprint")]
    
    pub sha256_fingerprint: Option<String>,
}

impl client::Part for CertificateInfo {}


/// Response message for the CheckAssetLinks call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [check assetlinks](AssetlinkCheckCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CheckResponse {
    /// Human-readable message containing information intended to help end users understand, reproduce and debug the result. The message will be in English and we are currently not planning to offer any translations. Please note that no guarantees are made about the contents or format of this string. Any aspect of it may be subject to change without notice. You should not attempt to programmatically parse this data. For programmatic access, use the error_code field below.
    #[serde(rename="debugString")]
    
    pub debug_string: Option<String>,
    /// Error codes that describe the result of the Check operation.
    #[serde(rename="errorCode")]
    
    pub error_code: Option<Vec<CheckResponseErrorCodeEnum>>,
    /// Set to true if the assets specified in the request are linked by the relation specified in the request.
    
    pub linked: Option<bool>,
    /// From serving time, how much longer the response should be considered valid barring further updates. REQUIRED
    #[serde(rename="maxAge")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub max_age: Option<client::chrono::Duration>,
}

impl client::ResponseResult for CheckResponse {}


/// Response message for the List call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list statements](StatementListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListResponse {
    /// Human-readable message containing information intended to help end users understand, reproduce and debug the result. The message will be in English and we are currently not planning to offer any translations. Please note that no guarantees are made about the contents or format of this string. Any aspect of it may be subject to change without notice. You should not attempt to programmatically parse this data. For programmatic access, use the error_code field below.
    #[serde(rename="debugString")]
    
    pub debug_string: Option<String>,
    /// Error codes that describe the result of the List operation.
    #[serde(rename="errorCode")]
    
    pub error_code: Option<Vec<ListResponseErrorCodeEnum>>,
    /// From serving time, how much longer the response should be considered valid barring further updates. REQUIRED
    #[serde(rename="maxAge")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub max_age: Option<client::chrono::Duration>,
    /// A list of all the matching statements that have been found.
    
    pub statements: Option<Vec<Statement>>,
}

impl client::ResponseResult for ListResponse {}


/// Describes a reliable statement that has been made about the relationship between a source asset and a target asset. Statements are always made by the source asset, either directly or by delegating to a statement list that is stored elsewhere. For more detailed definitions of statements and assets, please refer to our [API documentation landing page](https://developers.google.com/digital-asset-links/v1/getting-started).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list statements](StatementListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Statement {
    /// The relation identifies the use of the statement as intended by the source asset’s owner (that is, the person or entity who issued the statement). Every complete statement has a relation. We identify relations with strings of the format `/`, where `must be one of a set of pre-defined purpose categories, and` is a free-form lowercase alphanumeric string that describes the specific use case of the statement. Refer to [our API documentation](https://developers.google.com/digital-asset-links/v1/relation-strings) for the current list of supported relations. Example: `delegate_permission/common.handle_all_urls` REQUIRED
    
    pub relation: Option<String>,
    /// Every statement has a source asset. REQUIRED
    
    pub source: Option<Asset>,
    /// Every statement has a target asset. REQUIRED
    
    pub target: Option<Asset>,
}

impl client::Resource for Statement {}


/// A single statement to check in a bulk call using BulkCheck. See CheckRequest for details about each field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StatementTemplate {
    /// The relationship being asserted between the source and target. If omitted, you must specify a BulkCheckRequest.default_relation value to use here.
    
    pub relation: Option<String>,
    /// The source asset that is asserting the statement. If omitted, you must specify a BulkCheckRequest.default_source value to use here.
    
    pub source: Option<Asset>,
    /// The target that the source is declaring the relationship with. If omitted, you must specify a BulkCheckRequest.default_target to use here.
    
    pub target: Option<Asset>,
}

impl client::Part for StatementTemplate {}


/// Describes a web asset.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebAsset {
    /// Web assets are identified by a URL that contains only the scheme, hostname and port parts. The format is http[s]://[:] Hostnames must be fully qualified: they must end in a single period ("`.`"). Only the schemes "http" and "https" are currently allowed. Port numbers are given as a decimal number, and they must be omitted if the standard port numbers are used: 80 for http and 443 for https. We call this limited URL the "site". All URLs that share the same scheme, hostname and port are considered to be a part of the site and thus belong to the web asset. Example: the asset with the site `https://www.google.com` contains all these URLs: * `https://www.google.com/` * `https://www.google.com:443/` * `https://www.google.com/foo` * `https://www.google.com/foo?bar` * `https://www.google.com/foo#bar` * `https://user@password:www.google.com/` But it does not contain these URLs: * `http://www.google.com/` (wrong scheme) * `https://google.com/` (hostname does not match) * `https://www.google.com:444/` (port does not match) REQUIRED
    
    pub site: Option<String>,
}

impl client::Part for WebAsset {}


