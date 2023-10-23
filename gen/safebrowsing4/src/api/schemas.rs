use super::*;
/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create threat hits](ThreatHitCreateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleProtobufEmpty { _never_set: Option<bool> }

impl client::ResponseResult for GoogleProtobufEmpty {}


/// The expected state of a client's local database.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSecuritySafebrowsingV4Checksum {
    /// The SHA256 hash of the client state; that is, of the sorted list of all hashes present in the database.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub sha256: Option<Vec<u8>>,
}

impl client::Part for GoogleSecuritySafebrowsingV4Checksum {}


/// The client metadata associated with Safe Browsing API requests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSecuritySafebrowsingV4ClientInfo {
    /// A client ID that (hopefully) uniquely identifies the client implementation of the Safe Browsing API.
    #[serde(rename="clientId")]
    
    pub client_id: Option<String>,
    /// The version of the client implementation.
    #[serde(rename="clientVersion")]
    
    pub client_version: Option<String>,
}

impl client::Part for GoogleSecuritySafebrowsingV4ClientInfo {}


/// Describes a Safe Browsing API update request. Clients can request updates for multiple lists in a single request. The server may not respond to all requests, if the server has no updates for that list. NOTE: Field index 2 is unused. NEXT: 5
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [fetch threat list updates](ThreatListUpdateFetchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequest {
    /// The client metadata.
    
    pub client: Option<GoogleSecuritySafebrowsingV4ClientInfo>,
    /// The requested threat list updates.
    #[serde(rename="listUpdateRequests")]
    
    pub list_update_requests: Option<Vec<GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequest>>,
}

impl client::RequestValue for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequest {}


/// A single list update request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequest {
    /// The constraints associated with this request.
    
    pub constraints: Option<GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraints>,
    /// The type of platform at risk by entries present in the list.
    #[serde(rename="platformType")]
    
    pub platform_type: Option<GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum>,
    /// The current state of the client for the requested list (the encrypted client state that was received from the last successful list update).
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub state: Option<Vec<u8>>,
    /// The types of entries present in the list.
    #[serde(rename="threatEntryType")]
    
    pub threat_entry_type: Option<GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryTypeEnum>,
    /// The type of threat posed by entries present in the list.
    #[serde(rename="threatType")]
    
    pub threat_type: Option<GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum>,
}

impl client::Part for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequest {}


/// The constraints for this update.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraints {
    /// A client's physical location, expressed as a ISO 31166-1 alpha-2 region code.
    #[serde(rename="deviceLocation")]
    
    pub device_location: Option<String>,
    /// Requests the lists for a specific language. Expects ISO 639 alpha-2 format.
    
    pub language: Option<String>,
    /// Sets the maximum number of entries that the client is willing to have in the local database for the specified list. This should be a power of 2 between 2**10 and 2**20. If zero, no database size limit is set.
    #[serde(rename="maxDatabaseEntries")]
    
    pub max_database_entries: Option<i32>,
    /// The maximum size in number of entries. The update will not contain more entries than this value. This should be a power of 2 between 2**10 and 2**20. If zero, no update size limit is set.
    #[serde(rename="maxUpdateEntries")]
    
    pub max_update_entries: Option<i32>,
    /// Requests the list for a specific geographic location. If not set the server may pick that value based on the user's IP address. Expects ISO 3166-1 alpha-2 format.
    
    pub region: Option<String>,
    /// The compression types supported by the client.
    #[serde(rename="supportedCompressions")]
    
    pub supported_compressions: Option<Vec<GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintSupportedCompressionsEnum>>,
}

impl client::Part for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraints {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get encoded updates](EncodedUpdateGetCall) (response)
/// * [fetch threat list updates](ThreatListUpdateFetchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponse {
    /// The list updates requested by the clients. The number of responses here may be less than the number of requests sent by clients. This is the case, for example, if the server has no updates for a particular list.
    #[serde(rename="listUpdateResponses")]
    
    pub list_update_responses: Option<Vec<GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponse>>,
    /// The minimum duration the client must wait before issuing any update request. If this field is not set clients may update as soon as they want.
    #[serde(rename="minimumWaitDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub minimum_wait_duration: Option<client::chrono::Duration>,
}

impl client::ResponseResult for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponse {}


/// An update to an individual list.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponse {
    /// A set of entries to add to a local threat type's list. Repeated to allow for a combination of compressed and raw data to be sent in a single response.
    
    pub additions: Option<Vec<GoogleSecuritySafebrowsingV4ThreatEntrySet>>,
    /// The expected SHA256 hash of the client state; that is, of the sorted list of all hashes present in the database after applying the provided update. If the client state doesn't match the expected state, the client must disregard this update and retry later.
    
    pub checksum: Option<GoogleSecuritySafebrowsingV4Checksum>,
    /// The new client state, in encrypted format. Opaque to clients.
    #[serde(rename="newClientState")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub new_client_state: Option<Vec<u8>>,
    /// The platform type for which data is returned.
    #[serde(rename="platformType")]
    
    pub platform_type: Option<GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum>,
    /// A set of entries to remove from a local threat type's list. In practice, this field is empty or contains exactly one ThreatEntrySet.
    
    pub removals: Option<Vec<GoogleSecuritySafebrowsingV4ThreatEntrySet>>,
    /// The type of response. This may indicate that an action is required by the client when the response is received.
    #[serde(rename="responseType")]
    
    pub response_type: Option<GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseTypeEnum>,
    /// The format of the threats.
    #[serde(rename="threatEntryType")]
    
    pub threat_entry_type: Option<GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryTypeEnum>,
    /// The threat type for which data is returned.
    #[serde(rename="threatType")]
    
    pub threat_type: Option<GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum>,
}

impl client::Part for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponse {}


/// Request to return full hashes matched by the provided hash prefixes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [find full hashes](FullHashFindCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSecuritySafebrowsingV4FindFullHashesRequest {
    /// Client metadata associated with callers of higher-level APIs built on top of the client's implementation.
    #[serde(rename="apiClient")]
    
    pub api_client: Option<GoogleSecuritySafebrowsingV4ClientInfo>,
    /// The client metadata.
    
    pub client: Option<GoogleSecuritySafebrowsingV4ClientInfo>,
    /// The current client states for each of the client's local threat lists.
    #[serde(rename="clientStates")]
    
    #[serde_as(as = "Option<Vec<::client::serde::urlsafe_base64::Wrapper>>")]
    pub client_states: Option<Vec<Vec<u8>>>,
    /// The lists and hashes to be checked.
    #[serde(rename="threatInfo")]
    
    pub threat_info: Option<GoogleSecuritySafebrowsingV4ThreatInfo>,
}

impl client::RequestValue for GoogleSecuritySafebrowsingV4FindFullHashesRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get encoded full hashes](EncodedFullHashGetCall) (response)
/// * [find full hashes](FullHashFindCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSecuritySafebrowsingV4FindFullHashesResponse {
    /// The full hashes that matched the requested prefixes.
    
    pub matches: Option<Vec<GoogleSecuritySafebrowsingV4ThreatMatch>>,
    /// The minimum duration the client must wait before issuing any find hashes request. If this field is not set, clients can issue a request as soon as they want.
    #[serde(rename="minimumWaitDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub minimum_wait_duration: Option<client::chrono::Duration>,
    /// For requested entities that did not match the threat list, how long to cache the response.
    #[serde(rename="negativeCacheDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub negative_cache_duration: Option<client::chrono::Duration>,
}

impl client::ResponseResult for GoogleSecuritySafebrowsingV4FindFullHashesResponse {}


/// Request to check entries against lists.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [find threat matches](ThreatMatchFindCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSecuritySafebrowsingV4FindThreatMatchesRequest {
    /// The client metadata.
    
    pub client: Option<GoogleSecuritySafebrowsingV4ClientInfo>,
    /// The lists and entries to be checked for matches.
    #[serde(rename="threatInfo")]
    
    pub threat_info: Option<GoogleSecuritySafebrowsingV4ThreatInfo>,
}

impl client::RequestValue for GoogleSecuritySafebrowsingV4FindThreatMatchesRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [find threat matches](ThreatMatchFindCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSecuritySafebrowsingV4FindThreatMatchesResponse {
    /// The threat list matches.
    
    pub matches: Option<Vec<GoogleSecuritySafebrowsingV4ThreatMatch>>,
}

impl client::ResponseResult for GoogleSecuritySafebrowsingV4FindThreatMatchesResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list threat lists](ThreatListListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSecuritySafebrowsingV4ListThreatListsResponse {
    /// The lists available for download by the client.
    #[serde(rename="threatLists")]
    
    pub threat_lists: Option<Vec<GoogleSecuritySafebrowsingV4ThreatListDescriptor>>,
}

impl client::ResponseResult for GoogleSecuritySafebrowsingV4ListThreatListsResponse {}


/// The uncompressed threat entries in hash format of a particular prefix length. Hashes can be anywhere from 4 to 32 bytes in size. A large majority are 4 bytes, but some hashes are lengthened if they collide with the hash of a popular URL. Used for sending ThreatEntrySet to clients that do not support compression, or when sending non-4-byte hashes to clients that do support compression.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSecuritySafebrowsingV4RawHashes {
    /// The number of bytes for each prefix encoded below. This field can be anywhere from 4 (shortest prefix) to 32 (full SHA256 hash).
    #[serde(rename="prefixSize")]
    
    pub prefix_size: Option<i32>,
    /// The hashes, in binary format, concatenated into one long string. Hashes are sorted in lexicographic order. For JSON API users, hashes are base64-encoded.
    #[serde(rename="rawHashes")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub raw_hashes: Option<Vec<u8>>,
}

impl client::Part for GoogleSecuritySafebrowsingV4RawHashes {}


/// A set of raw indices to remove from a local list.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSecuritySafebrowsingV4RawIndices {
    /// The indices to remove from a lexicographically-sorted local list.
    
    pub indices: Option<Vec<i32>>,
}

impl client::Part for GoogleSecuritySafebrowsingV4RawIndices {}


/// The Rice-Golomb encoded data. Used for sending compressed 4-byte hashes or compressed removal indices.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSecuritySafebrowsingV4RiceDeltaEncoding {
    /// The encoded deltas that are encoded using the Golomb-Rice coder.
    #[serde(rename="encodedData")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub encoded_data: Option<Vec<u8>>,
    /// The offset of the first entry in the encoded data, or, if only a single integer was encoded, that single integer's value. If the field is empty or missing, assume zero.
    #[serde(rename="firstValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub first_value: Option<i64>,
    /// The number of entries that are delta encoded in the encoded data. If only a single integer was encoded, this will be zero and the single value will be stored in `first_value`.
    #[serde(rename="numEntries")]
    
    pub num_entries: Option<i32>,
    /// The Golomb-Rice parameter, which is a number between 2 and 28. This field is missing (that is, zero) if `num_entries` is zero.
    #[serde(rename="riceParameter")]
    
    pub rice_parameter: Option<i32>,
}

impl client::Part for GoogleSecuritySafebrowsingV4RiceDeltaEncoding {}


/// An individual threat; for example, a malicious URL or its hash representation. Only one of these fields should be set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSecuritySafebrowsingV4ThreatEntry {
    /// The digest of an executable in SHA256 format. The API supports both binary and hex digests. For JSON requests, digests are base64-encoded.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub digest: Option<Vec<u8>>,
    /// A hash prefix, consisting of the most significant 4-32 bytes of a SHA256 hash. This field is in binary format. For JSON requests, hashes are base64-encoded.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub hash: Option<Vec<u8>>,
    /// A URL.
    
    pub url: Option<String>,
}

impl client::Part for GoogleSecuritySafebrowsingV4ThreatEntry {}


/// The metadata associated with a specific threat entry. The client is expected to know the metadata key/value pairs associated with each threat type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSecuritySafebrowsingV4ThreatEntryMetadata {
    /// The metadata entries.
    
    pub entries: Option<Vec<GoogleSecuritySafebrowsingV4ThreatEntryMetadataMetadataEntry>>,
}

impl client::Part for GoogleSecuritySafebrowsingV4ThreatEntryMetadata {}


/// A single metadata entry.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSecuritySafebrowsingV4ThreatEntryMetadataMetadataEntry {
    /// The metadata entry key. For JSON requests, the key is base64-encoded.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub key: Option<Vec<u8>>,
    /// The metadata entry value. For JSON requests, the value is base64-encoded.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub value: Option<Vec<u8>>,
}

impl client::Part for GoogleSecuritySafebrowsingV4ThreatEntryMetadataMetadataEntry {}


/// A set of threats that should be added or removed from a client's local database.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSecuritySafebrowsingV4ThreatEntrySet {
    /// The compression type for the entries in this set.
    #[serde(rename="compressionType")]
    
    pub compression_type: Option<GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionTypeEnum>,
    /// The raw SHA256-formatted entries.
    #[serde(rename="rawHashes")]
    
    pub raw_hashes: Option<GoogleSecuritySafebrowsingV4RawHashes>,
    /// The raw removal indices for a local list.
    #[serde(rename="rawIndices")]
    
    pub raw_indices: Option<GoogleSecuritySafebrowsingV4RawIndices>,
    /// The encoded 4-byte prefixes of SHA256-formatted entries, using a Golomb-Rice encoding. The hashes are converted to uint32, sorted in ascending order, then delta encoded and stored as encoded_data.
    #[serde(rename="riceHashes")]
    
    pub rice_hashes: Option<GoogleSecuritySafebrowsingV4RiceDeltaEncoding>,
    /// The encoded local, lexicographically-sorted list indices, using a Golomb-Rice encoding. Used for sending compressed removal indices. The removal indices (uint32) are sorted in ascending order, then delta encoded and stored as encoded_data.
    #[serde(rename="riceIndices")]
    
    pub rice_indices: Option<GoogleSecuritySafebrowsingV4RiceDeltaEncoding>,
}

impl client::Part for GoogleSecuritySafebrowsingV4ThreatEntrySet {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create threat hits](ThreatHitCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSecuritySafebrowsingV4ThreatHit {
    /// Client-reported identification.
    #[serde(rename="clientInfo")]
    
    pub client_info: Option<GoogleSecuritySafebrowsingV4ClientInfo>,
    /// The threat entry responsible for the hit. Full hash should be reported for hash-based hits.
    
    pub entry: Option<GoogleSecuritySafebrowsingV4ThreatEntry>,
    /// The platform type reported.
    #[serde(rename="platformType")]
    
    pub platform_type: Option<GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum>,
    /// The resources related to the threat hit.
    
    pub resources: Option<Vec<GoogleSecuritySafebrowsingV4ThreatHitThreatSource>>,
    /// The threat type reported.
    #[serde(rename="threatType")]
    
    pub threat_type: Option<GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum>,
    /// Details about the user that encountered the threat.
    #[serde(rename="userInfo")]
    
    pub user_info: Option<GoogleSecuritySafebrowsingV4ThreatHitUserInfo>,
}

impl client::RequestValue for GoogleSecuritySafebrowsingV4ThreatHit {}


/// A single resource related to a threat hit.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSecuritySafebrowsingV4ThreatHitThreatSource {
    /// Referrer of the resource. Only set if the referrer is available.
    
    pub referrer: Option<String>,
    /// The remote IP of the resource in ASCII format. Either IPv4 or IPv6.
    #[serde(rename="remoteIp")]
    
    pub remote_ip: Option<String>,
    /// The type of source reported.
    #[serde(rename="type")]
    
    pub type_: Option<GoogleSecuritySafebrowsingV4ThreatHitThreatSourceTypeEnum>,
    /// The URL of the resource.
    
    pub url: Option<String>,
}

impl client::Part for GoogleSecuritySafebrowsingV4ThreatHitThreatSource {}


/// Details about the user that encountered the threat.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSecuritySafebrowsingV4ThreatHitUserInfo {
    /// The UN M.49 region code associated with the user's location.
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
    /// Unique user identifier defined by the client.
    #[serde(rename="userId")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub user_id: Option<Vec<u8>>,
}

impl client::Part for GoogleSecuritySafebrowsingV4ThreatHitUserInfo {}


/// The information regarding one or more threats that a client submits when checking for matches in threat lists.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSecuritySafebrowsingV4ThreatInfo {
    /// The platform types to be checked.
    #[serde(rename="platformTypes")]
    
    pub platform_types: Option<Vec<GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum>>,
    /// The threat entries to be checked.
    #[serde(rename="threatEntries")]
    
    pub threat_entries: Option<Vec<GoogleSecuritySafebrowsingV4ThreatEntry>>,
    /// The entry types to be checked.
    #[serde(rename="threatEntryTypes")]
    
    pub threat_entry_types: Option<Vec<GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesEnum>>,
    /// The threat types to be checked.
    #[serde(rename="threatTypes")]
    
    pub threat_types: Option<Vec<GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum>>,
}

impl client::Part for GoogleSecuritySafebrowsingV4ThreatInfo {}


/// Describes an individual threat list. A list is defined by three parameters: the type of threat posed, the type of platform targeted by the threat, and the type of entries in the list.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSecuritySafebrowsingV4ThreatListDescriptor {
    /// The platform type targeted by the list's entries.
    #[serde(rename="platformType")]
    
    pub platform_type: Option<GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum>,
    /// The entry types contained in the list.
    #[serde(rename="threatEntryType")]
    
    pub threat_entry_type: Option<GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryTypeEnum>,
    /// The threat type posed by the list's entries.
    #[serde(rename="threatType")]
    
    pub threat_type: Option<GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum>,
}

impl client::Part for GoogleSecuritySafebrowsingV4ThreatListDescriptor {}


/// A match when checking a threat entry in the Safe Browsing threat lists.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSecuritySafebrowsingV4ThreatMatch {
    /// The cache lifetime for the returned match. Clients must not cache this response for more than this duration to avoid false positives.
    #[serde(rename="cacheDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub cache_duration: Option<client::chrono::Duration>,
    /// The platform type matching this threat.
    #[serde(rename="platformType")]
    
    pub platform_type: Option<GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum>,
    /// The threat matching this threat.
    
    pub threat: Option<GoogleSecuritySafebrowsingV4ThreatEntry>,
    /// Optional metadata associated with this threat.
    #[serde(rename="threatEntryMetadata")]
    
    pub threat_entry_metadata: Option<GoogleSecuritySafebrowsingV4ThreatEntryMetadata>,
    /// The threat entry type matching this threat.
    #[serde(rename="threatEntryType")]
    
    pub threat_entry_type: Option<GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryTypeEnum>,
    /// The threat type matching this threat.
    #[serde(rename="threatType")]
    
    pub threat_type: Option<GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum>,
}

impl client::Part for GoogleSecuritySafebrowsingV4ThreatMatch {}


