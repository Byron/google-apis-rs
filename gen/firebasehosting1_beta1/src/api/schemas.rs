use super::*;
/// Contains metadata about the user who performed an action, such as creating a release or finalizing a version.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActingUser {
    /// The email address of the user when the user performed the action.
    
    pub email: Option<String>,
    /// A profile image URL for the user. May not be present if the user has changed their email address or deleted their account.
    #[serde(rename="imageUrl")]
    
    pub image_url: Option<String>,
}

impl client::Part for ActingUser {}


/// Represents a DNS certificate challenge.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CertDnsChallenge {
    /// The domain name upon which the DNS challenge must be satisfied.
    #[serde(rename="domainName")]
    
    pub domain_name: Option<String>,
    /// The value that must be present as a TXT record on the domain name to satisfy the challenge.
    
    pub token: Option<String>,
}

impl client::Part for CertDnsChallenge {}


/// Represents an HTTP certificate challenge.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CertHttpChallenge {
    /// The URL path on which to serve the specified token to satisfy the certificate challenge.
    
    pub path: Option<String>,
    /// The token to serve at the specified URL path to satisfy the certificate challenge.
    
    pub token: Option<String>,
}

impl client::Part for CertHttpChallenge {}


/// A `Channel` represents a stream of releases for a site. All sites have a default `live` channel that serves content to the Firebase-provided subdomains and any connected custom domains.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sites channels create projects](ProjectSiteChannelCreateCall) (request|response)
/// * [sites channels get projects](ProjectSiteChannelGetCall) (response)
/// * [sites channels patch projects](ProjectSiteChannelPatchCall) (request|response)
/// * [channels create sites](SiteChannelCreateCall) (request|response)
/// * [channels get sites](SiteChannelGetCall) (response)
/// * [channels patch sites](SiteChannelPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    /// Output only. The time at which the channel was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The time at which the channel will be automatically deleted. If null, the channel will not be automatically deleted. This field is present in the output whether it's set directly or via the `ttl` field.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Text labels used for extra metadata and/or filtering.
    
    pub labels: Option<HashMap<String, String>>,
    /// The fully-qualified resource name for the channel, in the format: sites/ SITE_ID/channels/CHANNEL_ID
    
    pub name: Option<String>,
    /// Output only. The current release for the channel, if any.
    
    pub release: Option<Release>,
    /// The number of previous releases to retain on the channel for rollback or other purposes. Must be a number between 1-100. Defaults to 10 for new channels.
    #[serde(rename="retainedReleaseCount")]
    
    pub retained_release_count: Option<i32>,
    /// Input only. A time-to-live for this channel. Sets `expire_time` to the provided duration past the time of the request.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub ttl: Option<client::chrono::Duration>,
    /// Output only. The time at which the channel was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The URL at which the content of this channel's current release can be viewed. This URL is a Firebase-provided subdomain of `web.app`. The content of this channel's current release can also be viewed at the Firebase-provided subdomain of `firebaseapp.com`. If this channel is the `live` channel for the Hosting site, then the content of this channel's current release can also be viewed at any connected custom domains.
    
    pub url: Option<String>,
}

impl client::RequestValue for Channel {}
impl client::ResponseResult for Channel {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sites versions clone projects](ProjectSiteVersionCloneCall) (request)
/// * [versions clone sites](SiteVersionCloneCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloneVersionRequest {
    /// If provided, only paths that do not match any of the RegEx values in this list will be included in the new version.
    
    pub exclude: Option<PathFilter>,
    /// If true, the call to `CloneVersion` immediately finalizes the version after cloning is complete. If false, the cloned version will have a status of `CREATED`. Use [`UpdateVersion`](patch) to set the status of the version to `FINALIZED`.
    
    pub finalize: Option<bool>,
    /// If provided, only paths that match one or more RegEx values in this list will be included in the new version.
    
    pub include: Option<PathFilter>,
    /// Required. The unique identifier for the version to be cloned, in the format: sites/SITE_ID/versions/VERSION_ID
    #[serde(rename="sourceVersion")]
    
    pub source_version: Option<String>,
}

impl client::RequestValue for CloneVersionRequest {}


/// A configured rewrite that directs requests to a Cloud Run service. If the Cloud Run service does not exist when setting or updating your Firebase Hosting configuration, then the request fails. Any errors from the Cloud Run service are passed to the end user (for example, if you delete a service, any requests directed to that service receive a `404` error).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudRunRewrite {
    /// Optional. User-provided region where the Cloud Run service is hosted. Defaults to `us-central1` if not supplied.
    
    pub region: Option<String>,
    /// Required. User-defined ID of the Cloud Run service.
    #[serde(rename="serviceId")]
    
    pub service_id: Option<String>,
}

impl client::Part for CloudRunRewrite {}


/// The intended behavior and status information of a domain.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sites domains create projects](ProjectSiteDomainCreateCall) (request|response)
/// * [sites domains get projects](ProjectSiteDomainGetCall) (response)
/// * [sites domains update projects](ProjectSiteDomainUpdateCall) (request|response)
/// * [domains create sites](SiteDomainCreateCall) (request|response)
/// * [domains get sites](SiteDomainGetCall) (response)
/// * [domains update sites](SiteDomainUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Domain {
    /// Required. The domain name of the association.
    #[serde(rename="domainName")]
    
    pub domain_name: Option<String>,
    /// If set, the domain should redirect with the provided parameters.
    #[serde(rename="domainRedirect")]
    
    pub domain_redirect: Option<DomainRedirect>,
    /// Output only. Information about the provisioning of certificates and the health of the DNS resolution for the domain.
    
    pub provisioning: Option<DomainProvisioning>,
    /// Required. The site name of the association.
    
    pub site: Option<String>,
    /// Output only. Additional status of the domain association.
    
    pub status: Option<DomainStatusEnum>,
    /// Output only. The time at which the domain was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Domain {}
impl client::ResponseResult for Domain {}


/// The current certificate provisioning status information for a domain.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DomainProvisioning {
    /// The TXT records (for the certificate challenge) that were found at the last DNS fetch.
    #[serde(rename="certChallengeDiscoveredTxt")]
    
    pub cert_challenge_discovered_txt: Option<Vec<String>>,
    /// The DNS challenge for generating a certificate.
    #[serde(rename="certChallengeDns")]
    
    pub cert_challenge_dns: Option<CertDnsChallenge>,
    /// The HTTP challenge for generating a certificate.
    #[serde(rename="certChallengeHttp")]
    
    pub cert_challenge_http: Option<CertHttpChallenge>,
    /// The certificate provisioning status; updated when Firebase Hosting provisions an SSL certificate for the domain.
    #[serde(rename="certStatus")]
    
    pub cert_status: Option<DomainProvisioningCertStatusEnum>,
    /// The IPs found at the last DNS fetch.
    #[serde(rename="discoveredIps")]
    
    pub discovered_ips: Option<Vec<String>>,
    /// The time at which the last DNS fetch occurred.
    #[serde(rename="dnsFetchTime")]
    
    pub dns_fetch_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The DNS record match status as of the last DNS fetch.
    #[serde(rename="dnsStatus")]
    
    pub dns_status: Option<DomainProvisioningDnsStatusEnum>,
    /// The list of IPs to which the domain is expected to resolve.
    #[serde(rename="expectedIps")]
    
    pub expected_ips: Option<Vec<String>>,
}

impl client::Part for DomainProvisioning {}


/// Defines the behavior of a domain-level redirect. Domain redirects preserve the path of the redirect but replace the requested domain with the one specified in the redirect configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DomainRedirect {
    /// Required. The domain name to redirect to.
    #[serde(rename="domainName")]
    
    pub domain_name: Option<String>,
    /// Required. The redirect status code.
    #[serde(rename="type")]
    
    pub type_: Option<DomainRedirectTypeEnum>,
}

impl client::Part for DomainRedirect {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sites channels delete projects](ProjectSiteChannelDeleteCall) (response)
/// * [sites domains delete projects](ProjectSiteDomainDeleteCall) (response)
/// * [sites versions delete projects](ProjectSiteVersionDeleteCall) (response)
/// * [sites delete projects](ProjectSiteDeleteCall) (response)
/// * [channels delete sites](SiteChannelDeleteCall) (response)
/// * [domains delete sites](SiteDomainDeleteCall) (response)
/// * [versions delete sites](SiteVersionDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// A [`Header`](https://firebase.google.com/docs/hosting/full-config#headers) specifies a URL pattern that, if matched to the request URL path, triggers Hosting to apply the specified custom response headers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Header {
    /// The user-supplied [glob](https://firebase.google.com/docs/hosting/full-config#glob_pattern_matching) to match against the request URL path.
    
    pub glob: Option<String>,
    /// Required. The additional headers to add to the response.
    
    pub headers: Option<HashMap<String, String>>,
    /// The user-supplied RE2 regular expression to match against the request URL path.
    
    pub regex: Option<String>,
}

impl client::Part for Header {}


/// If provided, i18n rewrites are enabled.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct I18nConfig {
    /// Required. The user-supplied path where country and language specific content will be looked for within the public directory.
    
    pub root: Option<String>,
}

impl client::Part for I18nConfig {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sites channels list projects](ProjectSiteChannelListCall) (response)
/// * [channels list sites](SiteChannelListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListChannelsResponse {
    /// The list of channels.
    
    pub channels: Option<Vec<Channel>>,
    /// The pagination token, if more results exist beyond the ones in this response. Include this token in your next call to `ListChannels`. Page tokens are short-lived and should not be stored.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListChannelsResponse {}


/// The response to listing Domains.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sites domains list projects](ProjectSiteDomainListCall) (response)
/// * [domains list sites](SiteDomainListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDomainsResponse {
    /// The list of domains, if any exist.
    
    pub domains: Option<Vec<Domain>>,
    /// The pagination token, if more results exist.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListDomainsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sites channels releases list projects](ProjectSiteChannelReleaseListCall) (response)
/// * [sites releases list projects](ProjectSiteReleaseListCall) (response)
/// * [channels releases list sites](SiteChannelReleaseListCall) (response)
/// * [releases list sites](SiteReleaseListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListReleasesResponse {
    /// The pagination token, if more results exist beyond the ones in this response. Include this token in your next call to `ListReleases`. Page tokens are short-lived and should not be stored.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of hashes of files that still need to be uploaded, if any exist.
    
    pub releases: Option<Vec<Release>>,
}

impl client::ResponseResult for ListReleasesResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sites list projects](ProjectSiteListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSitesResponse {
    /// The pagination token, if more results exist beyond the ones in this response. Include this token in your next call to `ListSites`. Page tokens are short-lived and should not be stored.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of Site objects associated with the specified Firebase project.
    
    pub sites: Option<Vec<Site>>,
}

impl client::ResponseResult for ListSitesResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sites versions files list projects](ProjectSiteVersionFileListCall) (response)
/// * [versions files list sites](SiteVersionFileListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListVersionFilesResponse {
    ///  The list of paths to the hashes of the files in the specified version.
    
    pub files: Option<Vec<VersionFile>>,
    /// The pagination token, if more results exist beyond the ones in this response. Include this token in your next call to `ListVersionFiles`. Page tokens are short-lived and should not be stored.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListVersionFilesResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sites versions list projects](ProjectSiteVersionListCall) (response)
/// * [versions list sites](SiteVersionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListVersionsResponse {
    /// The pagination token, if more results exist beyond the ones in this response. Include this token in your next call to `ListVersions`. Page tokens are short-lived and should not be stored.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of versions, if any exist.
    
    pub versions: Option<Vec<Version>>,
}

impl client::ResponseResult for ListVersionsResponse {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [operations get projects](ProjectOperationGetCall) (response)
/// * [sites versions clone projects](ProjectSiteVersionCloneCall) (response)
/// * [versions clone sites](SiteVersionCloneCall) (response)
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


/// A representation of filter path.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PathFilter {
    /// An array of RegEx values by which to filter.
    
    pub regexes: Option<Vec<String>>,
}

impl client::Part for PathFilter {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sites versions populate files projects](ProjectSiteVersionPopulateFileCall) (request)
/// * [versions populate files sites](SiteVersionPopulateFileCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PopulateVersionFilesRequest {
    /// A set of file paths to the hashes corresponding to assets that should be added to the version. A file path to an empty hash will remove the path from the version. Calculate a hash by Gzipping the file then taking the SHA256 hash of the newly compressed file.
    
    pub files: Option<HashMap<String, String>>,
}

impl client::RequestValue for PopulateVersionFilesRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sites versions populate files projects](ProjectSiteVersionPopulateFileCall) (response)
/// * [versions populate files sites](SiteVersionPopulateFileCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PopulateVersionFilesResponse {
    /// The content hashes of the specified files that need to be uploaded to the specified URL.
    #[serde(rename="uploadRequiredHashes")]
    
    pub upload_required_hashes: Option<Vec<String>>,
    /// The URL to which the files should be uploaded, in the format: "https://upload-firebasehosting.googleapis.com/upload/sites/SITE_ID /versions/VERSION_ID/files" Perform a multipart `POST` of the Gzipped file contents to the URL using a forward slash and the hash of the file appended to the end.
    #[serde(rename="uploadUrl")]
    
    pub upload_url: Option<String>,
}

impl client::ResponseResult for PopulateVersionFilesResponse {}


/// A [`Redirect`](https://firebase.google.com/docs/hosting/full-config#redirects) specifies a URL pattern that, if matched to the request URL path, triggers Hosting to respond with a redirect to the specified destination path.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Redirect {
    /// The user-supplied [glob](https://firebase.google.com/docs/hosting/full-config#glob_pattern_matching) to match against the request URL path.
    
    pub glob: Option<String>,
    /// Required. The value to put in the HTTP location header of the response. The location can contain capture group values from the pattern using a `:` prefix to identify the segment and an optional `*` to capture the rest of the URL. For example: "glob": "/:capture*", "statusCode": 301, "location": "https://example.com/foo/:capture"
    
    pub location: Option<String>,
    /// The user-supplied RE2 regular expression to match against the request URL path.
    
    pub regex: Option<String>,
    /// Required. The status HTTP code to return in the response. It must be a valid 3xx status code.
    #[serde(rename="statusCode")]
    
    pub status_code: Option<i32>,
}

impl client::Part for Redirect {}


/// A `Release` is a particular [collection of configurations and files](sites.versions) that is set to be public at a particular time.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sites channels releases create projects](ProjectSiteChannelReleaseCreateCall) (request|response)
/// * [sites releases create projects](ProjectSiteReleaseCreateCall) (request|response)
/// * [channels releases create sites](SiteChannelReleaseCreateCall) (request|response)
/// * [releases create sites](SiteReleaseCreateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Release {
    /// The deploy description when the release was created. The value can be up to 512 characters.
    
    pub message: Option<String>,
    /// Output only. The unique identifier for the release, in either of the following formats: - sites/SITE_ID/releases/RELEASE_ID - sites/SITE_ID/channels/CHANNEL_ID/releases/RELEASE_ID This name is provided in the response body when you call [`releases.create`](sites.releases/create) or [`channels.releases.create`](sites.channels.releases/create).
    
    pub name: Option<String>,
    /// Output only. The time at which the version is set to be public.
    #[serde(rename="releaseTime")]
    
    pub release_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Identifies the user who created the release.
    #[serde(rename="releaseUser")]
    
    pub release_user: Option<ActingUser>,
    /// Explains the reason for the release. Specify a value for this field only when creating a `SITE_DISABLE` type release.
    #[serde(rename="type")]
    
    pub type_: Option<ReleaseTypeEnum>,
    /// Output only. The configuration and content that was released.
    
    pub version: Option<Version>,
}

impl client::RequestValue for Release {}
impl client::ResponseResult for Release {}


/// A [`Rewrite`](https://firebase.google.com/docs/hosting/full-config#rewrites) specifies a URL pattern that, if matched to the request URL path, triggers Hosting to respond as if the service were given the specified destination URL.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Rewrite {
    /// The request will be forwarded to Firebase Dynamic Links.
    #[serde(rename="dynamicLinks")]
    
    pub dynamic_links: Option<bool>,
    /// The function to proxy requests to. Must match the exported function name exactly.
    
    pub function: Option<String>,
    /// Optional. Specify a Cloud region for rewritten Functions invocations. If not provided, defaults to us-central1.
    #[serde(rename="functionRegion")]
    
    pub function_region: Option<String>,
    /// The user-supplied [glob](https://firebase.google.com/docs/hosting/full-config#glob_pattern_matching) to match against the request URL path.
    
    pub glob: Option<String>,
    /// The URL path to rewrite the request to.
    
    pub path: Option<String>,
    /// The user-supplied RE2 regular expression to match against the request URL path.
    
    pub regex: Option<String>,
    /// The request will be forwarded to Cloud Run.
    
    pub run: Option<CloudRunRewrite>,
}

impl client::Part for Rewrite {}


/// The configuration for how incoming requests to a site should be routed and processed before serving content. The URL request paths are matched against the specified URL patterns in the configuration, then Hosting applies the applicable configuration according to a specific [priority order](https://firebase.google.com/docs/hosting/full-config#hosting_priority_order).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServingConfig {
    /// How to handle well known App Association files.
    #[serde(rename="appAssociation")]
    
    pub app_association: Option<ServingConfigAppAssociationEnum>,
    /// Defines whether to drop the file extension from uploaded files.
    #[serde(rename="cleanUrls")]
    
    pub clean_urls: Option<bool>,
    /// An array of objects, where each object specifies a URL pattern that, if matched to the request URL path, triggers Hosting to apply the specified custom response headers.
    
    pub headers: Option<Vec<Header>>,
    /// Optional. Defines i18n rewrite behavior.
    
    pub i18n: Option<I18nConfig>,
    /// An array of objects (called redirect rules), where each rule specifies a URL pattern that, if matched to the request URL path, triggers Hosting to respond with a redirect to the specified destination path.
    
    pub redirects: Option<Vec<Redirect>>,
    /// An array of objects (called rewrite rules), where each rule specifies a URL pattern that, if matched to the request URL path, triggers Hosting to respond as if the service were given the specified destination URL.
    
    pub rewrites: Option<Vec<Rewrite>>,
    /// Defines how to handle a trailing slash in the URL path.
    #[serde(rename="trailingSlashBehavior")]
    
    pub trailing_slash_behavior: Option<ServingConfigTrailingSlashBehaviorEnum>,
}

impl client::Part for ServingConfig {}


/// A `Site` represents a Firebase Hosting site.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sites create projects](ProjectSiteCreateCall) (request|response)
/// * [sites get projects](ProjectSiteGetCall) (response)
/// * [sites patch projects](ProjectSitePatchCall) (request|response)
/// * [channels releases create sites](SiteChannelReleaseCreateCall) (none)
/// * [channels releases list sites](SiteChannelReleaseListCall) (none)
/// * [channels create sites](SiteChannelCreateCall) (none)
/// * [channels delete sites](SiteChannelDeleteCall) (none)
/// * [channels get sites](SiteChannelGetCall) (none)
/// * [channels list sites](SiteChannelListCall) (none)
/// * [channels patch sites](SiteChannelPatchCall) (none)
/// * [domains create sites](SiteDomainCreateCall) (none)
/// * [domains delete sites](SiteDomainDeleteCall) (none)
/// * [domains get sites](SiteDomainGetCall) (none)
/// * [domains list sites](SiteDomainListCall) (none)
/// * [domains update sites](SiteDomainUpdateCall) (none)
/// * [releases create sites](SiteReleaseCreateCall) (none)
/// * [releases list sites](SiteReleaseListCall) (none)
/// * [versions files list sites](SiteVersionFileListCall) (none)
/// * [versions clone sites](SiteVersionCloneCall) (none)
/// * [versions create sites](SiteVersionCreateCall) (none)
/// * [versions delete sites](SiteVersionDeleteCall) (none)
/// * [versions list sites](SiteVersionListCall) (none)
/// * [versions patch sites](SiteVersionPatchCall) (none)
/// * [versions populate files sites](SiteVersionPopulateFileCall) (none)
/// * [get config sites](SiteGetConfigCall) (none)
/// * [update config sites](SiteUpdateConfigCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Site {
    /// Optional. The [ID of a Web App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.webApps#WebApp.FIELDS.app_id) associated with the Hosting site.
    #[serde(rename="appId")]
    
    pub app_id: Option<String>,
    /// Output only. The default URL for the Hosting site.
    #[serde(rename="defaultUrl")]
    
    pub default_url: Option<String>,
    /// Optional. User-specified labels for the Hosting site.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The fully-qualified resource name of the Hosting site, in the format: projects/PROJECT_IDENTIFIER/sites/SITE_ID PROJECT_IDENTIFIER: the Firebase project's [`ProjectNumber`](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its [`ProjectId`](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects#FirebaseProject.FIELDS.project_id). Learn more about using project identifiers in Google's [AIP 2510 standard](https://google.aip.dev/cloud/2510).
    
    pub name: Option<String>,
    /// Output only. The type of Hosting site. Every Firebase project has a `DEFAULT_SITE`, which is created when Hosting is provisioned for the project. All additional sites are `USER_SITE`.
    #[serde(rename="type")]
    
    pub type_: Option<SiteTypeEnum>,
}

impl client::RequestValue for Site {}
impl client::Resource for Site {}
impl client::ResponseResult for Site {}


/// A `SiteConfig` contains metadata associated with a specific site that controls Firebase Hosting serving behavior
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sites get config projects](ProjectSiteGetConfigCall) (response)
/// * [sites update config projects](ProjectSiteUpdateConfigCall) (request|response)
/// * [get config sites](SiteGetConfigCall) (response)
/// * [update config sites](SiteUpdateConfigCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SiteConfig {
    /// Whether or not web requests made by site visitors are logged via Cloud Logging.
    #[serde(rename="cloudLoggingEnabled")]
    
    pub cloud_logging_enabled: Option<bool>,
    /// The number of FINALIZED versions that will be held for a site before automatic deletion. When a new version is deployed, content for versions in storage in excess of this number will be deleted, and will no longer be billed for storage usage. Oldest versions will be deleted first; sites are created with an unlimited number of max_versions by default.
    #[serde(rename="maxVersions")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_versions: Option<i64>,
}

impl client::RequestValue for SiteConfig {}
impl client::ResponseResult for SiteConfig {}


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


/// A `Version` is a configuration and a collection of static files which determine how a site is displayed.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sites versions create projects](ProjectSiteVersionCreateCall) (request|response)
/// * [sites versions patch projects](ProjectSiteVersionPatchCall) (request|response)
/// * [versions create sites](SiteVersionCreateCall) (request|response)
/// * [versions patch sites](SiteVersionPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Version {
    /// The configuration for the behavior of the site. This configuration exists in the [`firebase.json`](https://firebase.google.com/docs/cli/#the_firebasejson_file) file.
    
    pub config: Option<ServingConfig>,
    /// Output only. The time at which the version was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Identifies the user who created the version.
    #[serde(rename="createUser")]
    
    pub create_user: Option<ActingUser>,
    /// Output only. The time at which the version was `DELETED`.
    #[serde(rename="deleteTime")]
    
    pub delete_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Identifies the user who `DELETED` the version.
    #[serde(rename="deleteUser")]
    
    pub delete_user: Option<ActingUser>,
    /// Output only. The total number of files associated with the version. This value is calculated after a version is `FINALIZED`.
    #[serde(rename="fileCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub file_count: Option<i64>,
    /// Output only. The time at which the version was `FINALIZED`.
    #[serde(rename="finalizeTime")]
    
    pub finalize_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Identifies the user who `FINALIZED` the version.
    #[serde(rename="finalizeUser")]
    
    pub finalize_user: Option<ActingUser>,
    /// The labels used for extra metadata and/or filtering.
    
    pub labels: Option<HashMap<String, String>>,
    /// The fully-qualified resource name for the version, in the format: sites/ SITE_ID/versions/VERSION_ID This name is provided in the response body when you call [`CreateVersion`](sites.versions/create).
    
    pub name: Option<String>,
    /// The deploy status of the version. For a successful deploy, call [`CreateVersion`](sites.versions/create) to make a new version (`CREATED` status), [upload all desired files](sites.versions/populateFiles) to the version, then [update](sites.versions/patch) the version to the `FINALIZED` status. Note that if you leave the version in the `CREATED` state for more than 12 hours, the system will automatically mark the version as `ABANDONED`. You can also change the status of a version to `DELETED` by calling [`DeleteVersion`](sites.versions/delete).
    
    pub status: Option<VersionStatusEnum>,
    /// Output only. The total stored bytesize of the version. This value is calculated after a version is `FINALIZED`.
    #[serde(rename="versionBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version_bytes: Option<i64>,
}

impl client::RequestValue for Version {}
impl client::ResponseResult for Version {}


/// A static content file that is part of a version.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VersionFile {
    /// The SHA256 content hash of the file.
    
    pub hash: Option<String>,
    /// The URI at which the file's content should display.
    
    pub path: Option<String>,
    /// Output only. The current status of a particular file in the specified version. The value will be either `pending upload` or `uploaded`.
    
    pub status: Option<VersionFileStatusEnum>,
}

impl client::Part for VersionFile {}


