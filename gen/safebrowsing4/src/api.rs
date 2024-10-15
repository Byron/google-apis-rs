#![allow(clippy::ptr_arg)]

use std::collections::{BTreeSet, HashMap};

use tokio::time::sleep;

// ##############
// UTILITIES ###
// ############

// ########
// HUB ###
// ######

/// Central instance to access all Safebrowsing related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_safebrowsing4 as safebrowsing4;
/// use safebrowsing4::{Result, Error};
/// # async fn dox() {
/// use safebrowsing4::{Safebrowsing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and
/// // `client_secret`, among other things.
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you,
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http1()
///         .build()
/// );
/// let mut hub = Safebrowsing::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.encoded_full_hashes().get(vec![0, 1, 2, 3])
///              .client_version("voluptua.")
///              .client_id("At")
///              .doit().await;
///
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::Io(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
#[derive(Clone)]
pub struct Safebrowsing<C> {
    pub client: common::Client<C>,
    pub auth: Box<dyn common::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<C> common::Hub for Safebrowsing<C> {}

impl<'a, C> Safebrowsing<C> {
    pub fn new<A: 'static + common::GetToken>(
        client: common::Client<C>,
        auth: A,
    ) -> Safebrowsing<C> {
        Safebrowsing {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/6.0.0".to_string(),
            _base_url: "https://safebrowsing.googleapis.com/".to_string(),
            _root_url: "https://safebrowsing.googleapis.com/".to_string(),
        }
    }

    pub fn encoded_full_hashes(&'a self) -> EncodedFullHashMethods<'a, C> {
        EncodedFullHashMethods { hub: self }
    }
    pub fn encoded_updates(&'a self) -> EncodedUpdateMethods<'a, C> {
        EncodedUpdateMethods { hub: self }
    }
    pub fn full_hashes(&'a self) -> FullHashMethods<'a, C> {
        FullHashMethods { hub: self }
    }
    pub fn threat_hits(&'a self) -> ThreatHitMethods<'a, C> {
        ThreatHitMethods { hub: self }
    }
    pub fn threat_list_updates(&'a self) -> ThreatListUpdateMethods<'a, C> {
        ThreatListUpdateMethods { hub: self }
    }
    pub fn threat_lists(&'a self) -> ThreatListMethods<'a, C> {
        ThreatListMethods { hub: self }
    }
    pub fn threat_matches(&'a self) -> ThreatMatchMethods<'a, C> {
        ThreatMatchMethods { hub: self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/6.0.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        std::mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://safebrowsing.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        std::mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://safebrowsing.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        std::mem::replace(&mut self._root_url, new_root_url)
    }
}

// ############
// SCHEMAS ###
// ##########
/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [create threat hits](ThreatHitCreateCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleProtobufEmpty {
    _never_set: Option<bool>,
}

impl common::ResponseResult for GoogleProtobufEmpty {}

/// The expected state of a client's local database.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleSecuritySafebrowsingV4Checksum {
    /// The SHA256 hash of the client state; that is, of the sorted list of all hashes present in the database.
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub sha256: Option<Vec<u8>>,
}

impl common::Part for GoogleSecuritySafebrowsingV4Checksum {}

/// The client metadata associated with Safe Browsing API requests.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleSecuritySafebrowsingV4ClientInfo {
    /// A client ID that (hopefully) uniquely identifies the client implementation of the Safe Browsing API.
    #[serde(rename = "clientId")]
    pub client_id: Option<String>,
    /// The version of the client implementation.
    #[serde(rename = "clientVersion")]
    pub client_version: Option<String>,
}

impl common::Part for GoogleSecuritySafebrowsingV4ClientInfo {}

/// Describes a Safe Browsing API update request. Clients can request updates for multiple lists in a single request. The server may not respond to all requests, if the server has no updates for that list. NOTE: Field index 2 is unused. NEXT: 5
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [fetch threat list updates](ThreatListUpdateFetchCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequest {
    /// The client metadata.
    pub client: Option<GoogleSecuritySafebrowsingV4ClientInfo>,
    /// The requested threat list updates.
    #[serde(rename = "listUpdateRequests")]
    pub list_update_requests:
        Option<Vec<GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequest>>,
}

impl common::RequestValue for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequest {}

/// A single list update request.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequest {
    /// The constraints associated with this request.
    pub constraints: Option<
        GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraints,
    >,
    /// The type of platform at risk by entries present in the list.
    #[serde(rename = "platformType")]
    pub platform_type: Option<String>,
    /// The current state of the client for the requested list (the encrypted client state that was received from the last successful list update).
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub state: Option<Vec<u8>>,
    /// The types of entries present in the list.
    #[serde(rename = "threatEntryType")]
    pub threat_entry_type: Option<String>,
    /// The type of threat posed by entries present in the list.
    #[serde(rename = "threatType")]
    pub threat_type: Option<String>,
}

impl common::Part for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequest {}

/// The constraints for this update.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraints {
    /// A client's physical location, expressed as a ISO 31166-1 alpha-2 region code.
    #[serde(rename = "deviceLocation")]
    pub device_location: Option<String>,
    /// Requests the lists for a specific language. Expects ISO 639 alpha-2 format.
    pub language: Option<String>,
    /// Sets the maximum number of entries that the client is willing to have in the local database for the specified list. This should be a power of 2 between 2**10 and 2**20. If zero, no database size limit is set.
    #[serde(rename = "maxDatabaseEntries")]
    pub max_database_entries: Option<i32>,
    /// The maximum size in number of entries. The update will not contain more entries than this value. This should be a power of 2 between 2**10 and 2**20. If zero, no update size limit is set.
    #[serde(rename = "maxUpdateEntries")]
    pub max_update_entries: Option<i32>,
    /// Requests the list for a specific geographic location. If not set the server may pick that value based on the user's IP address. Expects ISO 3166-1 alpha-2 format.
    pub region: Option<String>,
    /// The compression types supported by the client.
    #[serde(rename = "supportedCompressions")]
    pub supported_compressions: Option<Vec<String>>,
}

impl common::Part
    for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraints
{
}

/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [get encoded updates](EncodedUpdateGetCall) (response)
/// * [fetch threat list updates](ThreatListUpdateFetchCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponse {
    /// The list updates requested by the clients. The number of responses here may be less than the number of requests sent by clients. This is the case, for example, if the server has no updates for a particular list.
    #[serde(rename = "listUpdateResponses")]
    pub list_update_responses:
        Option<Vec<GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponse>>,
    /// The minimum duration the client must wait before issuing any update request. If this field is not set clients may update as soon as they want.
    #[serde(rename = "minimumWaitDuration")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub minimum_wait_duration: Option<chrono::Duration>,
}

impl common::ResponseResult for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponse {}

/// An update to an individual list.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponse {
    /// A set of entries to add to a local threat type's list. Repeated to allow for a combination of compressed and raw data to be sent in a single response.
    pub additions: Option<Vec<GoogleSecuritySafebrowsingV4ThreatEntrySet>>,
    /// The expected SHA256 hash of the client state; that is, of the sorted list of all hashes present in the database after applying the provided update. If the client state doesn't match the expected state, the client must disregard this update and retry later.
    pub checksum: Option<GoogleSecuritySafebrowsingV4Checksum>,
    /// The new client state, in encrypted format. Opaque to clients.
    #[serde(rename = "newClientState")]
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub new_client_state: Option<Vec<u8>>,
    /// The platform type for which data is returned.
    #[serde(rename = "platformType")]
    pub platform_type: Option<String>,
    /// A set of entries to remove from a local threat type's list. In practice, this field is empty or contains exactly one ThreatEntrySet.
    pub removals: Option<Vec<GoogleSecuritySafebrowsingV4ThreatEntrySet>>,
    /// The type of response. This may indicate that an action is required by the client when the response is received.
    #[serde(rename = "responseType")]
    pub response_type: Option<String>,
    /// The format of the threats.
    #[serde(rename = "threatEntryType")]
    pub threat_entry_type: Option<String>,
    /// The threat type for which data is returned.
    #[serde(rename = "threatType")]
    pub threat_type: Option<String>,
}

impl common::Part for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponse {}

/// Request to return full hashes matched by the provided hash prefixes.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [find full hashes](FullHashFindCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleSecuritySafebrowsingV4FindFullHashesRequest {
    /// Client metadata associated with callers of higher-level APIs built on top of the client's implementation.
    #[serde(rename = "apiClient")]
    pub api_client: Option<GoogleSecuritySafebrowsingV4ClientInfo>,
    /// The client metadata.
    pub client: Option<GoogleSecuritySafebrowsingV4ClientInfo>,
    /// The current client states for each of the client's local threat lists.
    #[serde(rename = "clientStates")]
    #[serde_as(as = "Option<Vec<common::serde::standard_base64::Wrapper>>")]
    pub client_states: Option<Vec<Vec<u8>>>,
    /// The lists and hashes to be checked.
    #[serde(rename = "threatInfo")]
    pub threat_info: Option<GoogleSecuritySafebrowsingV4ThreatInfo>,
}

impl common::RequestValue for GoogleSecuritySafebrowsingV4FindFullHashesRequest {}

/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [get encoded full hashes](EncodedFullHashGetCall) (response)
/// * [find full hashes](FullHashFindCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleSecuritySafebrowsingV4FindFullHashesResponse {
    /// The full hashes that matched the requested prefixes.
    pub matches: Option<Vec<GoogleSecuritySafebrowsingV4ThreatMatch>>,
    /// The minimum duration the client must wait before issuing any find hashes request. If this field is not set, clients can issue a request as soon as they want.
    #[serde(rename = "minimumWaitDuration")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub minimum_wait_duration: Option<chrono::Duration>,
    /// For requested entities that did not match the threat list, how long to cache the response.
    #[serde(rename = "negativeCacheDuration")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub negative_cache_duration: Option<chrono::Duration>,
}

impl common::ResponseResult for GoogleSecuritySafebrowsingV4FindFullHashesResponse {}

/// Request to check entries against lists.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [find threat matches](ThreatMatchFindCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleSecuritySafebrowsingV4FindThreatMatchesRequest {
    /// The client metadata.
    pub client: Option<GoogleSecuritySafebrowsingV4ClientInfo>,
    /// The lists and entries to be checked for matches.
    #[serde(rename = "threatInfo")]
    pub threat_info: Option<GoogleSecuritySafebrowsingV4ThreatInfo>,
}

impl common::RequestValue for GoogleSecuritySafebrowsingV4FindThreatMatchesRequest {}

/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [find threat matches](ThreatMatchFindCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleSecuritySafebrowsingV4FindThreatMatchesResponse {
    /// The threat list matches.
    pub matches: Option<Vec<GoogleSecuritySafebrowsingV4ThreatMatch>>,
}

impl common::ResponseResult for GoogleSecuritySafebrowsingV4FindThreatMatchesResponse {}

/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list threat lists](ThreatListListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleSecuritySafebrowsingV4ListThreatListsResponse {
    /// The lists available for download by the client.
    #[serde(rename = "threatLists")]
    pub threat_lists: Option<Vec<GoogleSecuritySafebrowsingV4ThreatListDescriptor>>,
}

impl common::ResponseResult for GoogleSecuritySafebrowsingV4ListThreatListsResponse {}

/// The uncompressed threat entries in hash format of a particular prefix length. Hashes can be anywhere from 4 to 32 bytes in size. A large majority are 4 bytes, but some hashes are lengthened if they collide with the hash of a popular URL. Used for sending ThreatEntrySet to clients that do not support compression, or when sending non-4-byte hashes to clients that do support compression.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleSecuritySafebrowsingV4RawHashes {
    /// The number of bytes for each prefix encoded below. This field can be anywhere from 4 (shortest prefix) to 32 (full SHA256 hash).
    #[serde(rename = "prefixSize")]
    pub prefix_size: Option<i32>,
    /// The hashes, in binary format, concatenated into one long string. Hashes are sorted in lexicographic order. For JSON API users, hashes are base64-encoded.
    #[serde(rename = "rawHashes")]
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub raw_hashes: Option<Vec<u8>>,
}

impl common::Part for GoogleSecuritySafebrowsingV4RawHashes {}

/// A set of raw indices to remove from a local list.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleSecuritySafebrowsingV4RawIndices {
    /// The indices to remove from a lexicographically-sorted local list.
    pub indices: Option<Vec<i32>>,
}

impl common::Part for GoogleSecuritySafebrowsingV4RawIndices {}

/// The Rice-Golomb encoded data. Used for sending compressed 4-byte hashes or compressed removal indices.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleSecuritySafebrowsingV4RiceDeltaEncoding {
    /// The encoded deltas that are encoded using the Golomb-Rice coder.
    #[serde(rename = "encodedData")]
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub encoded_data: Option<Vec<u8>>,
    /// The offset of the first entry in the encoded data, or, if only a single integer was encoded, that single integer's value. If the field is empty or missing, assume zero.
    #[serde(rename = "firstValue")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub first_value: Option<i64>,
    /// The number of entries that are delta encoded in the encoded data. If only a single integer was encoded, this will be zero and the single value will be stored in `first_value`.
    #[serde(rename = "numEntries")]
    pub num_entries: Option<i32>,
    /// The Golomb-Rice parameter, which is a number between 2 and 28. This field is missing (that is, zero) if `num_entries` is zero.
    #[serde(rename = "riceParameter")]
    pub rice_parameter: Option<i32>,
}

impl common::Part for GoogleSecuritySafebrowsingV4RiceDeltaEncoding {}

/// An individual threat; for example, a malicious URL or its hash representation. Only one of these fields should be set.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleSecuritySafebrowsingV4ThreatEntry {
    /// The digest of an executable in SHA256 format. The API supports both binary and hex digests. For JSON requests, digests are base64-encoded.
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub digest: Option<Vec<u8>>,
    /// A hash prefix, consisting of the most significant 4-32 bytes of a SHA256 hash. This field is in binary format. For JSON requests, hashes are base64-encoded.
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub hash: Option<Vec<u8>>,
    /// A URL.
    pub url: Option<String>,
}

impl common::Part for GoogleSecuritySafebrowsingV4ThreatEntry {}

/// The metadata associated with a specific threat entry. The client is expected to know the metadata key/value pairs associated with each threat type.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleSecuritySafebrowsingV4ThreatEntryMetadata {
    /// The metadata entries.
    pub entries: Option<Vec<GoogleSecuritySafebrowsingV4ThreatEntryMetadataMetadataEntry>>,
}

impl common::Part for GoogleSecuritySafebrowsingV4ThreatEntryMetadata {}

/// A single metadata entry.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleSecuritySafebrowsingV4ThreatEntryMetadataMetadataEntry {
    /// The metadata entry key. For JSON requests, the key is base64-encoded.
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub key: Option<Vec<u8>>,
    /// The metadata entry value. For JSON requests, the value is base64-encoded.
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub value: Option<Vec<u8>>,
}

impl common::Part for GoogleSecuritySafebrowsingV4ThreatEntryMetadataMetadataEntry {}

/// A set of threats that should be added or removed from a client's local database.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleSecuritySafebrowsingV4ThreatEntrySet {
    /// The compression type for the entries in this set.
    #[serde(rename = "compressionType")]
    pub compression_type: Option<String>,
    /// The raw SHA256-formatted entries.
    #[serde(rename = "rawHashes")]
    pub raw_hashes: Option<GoogleSecuritySafebrowsingV4RawHashes>,
    /// The raw removal indices for a local list.
    #[serde(rename = "rawIndices")]
    pub raw_indices: Option<GoogleSecuritySafebrowsingV4RawIndices>,
    /// The encoded 4-byte prefixes of SHA256-formatted entries, using a Golomb-Rice encoding. The hashes are converted to uint32, sorted in ascending order, then delta encoded and stored as encoded_data.
    #[serde(rename = "riceHashes")]
    pub rice_hashes: Option<GoogleSecuritySafebrowsingV4RiceDeltaEncoding>,
    /// The encoded local, lexicographically-sorted list indices, using a Golomb-Rice encoding. Used for sending compressed removal indices. The removal indices (uint32) are sorted in ascending order, then delta encoded and stored as encoded_data.
    #[serde(rename = "riceIndices")]
    pub rice_indices: Option<GoogleSecuritySafebrowsingV4RiceDeltaEncoding>,
}

impl common::Part for GoogleSecuritySafebrowsingV4ThreatEntrySet {}

/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [create threat hits](ThreatHitCreateCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleSecuritySafebrowsingV4ThreatHit {
    /// Client-reported identification.
    #[serde(rename = "clientInfo")]
    pub client_info: Option<GoogleSecuritySafebrowsingV4ClientInfo>,
    /// The threat entry responsible for the hit. Full hash should be reported for hash-based hits.
    pub entry: Option<GoogleSecuritySafebrowsingV4ThreatEntry>,
    /// The platform type reported.
    #[serde(rename = "platformType")]
    pub platform_type: Option<String>,
    /// The resources related to the threat hit.
    pub resources: Option<Vec<GoogleSecuritySafebrowsingV4ThreatHitThreatSource>>,
    /// The threat type reported.
    #[serde(rename = "threatType")]
    pub threat_type: Option<String>,
    /// Details about the user that encountered the threat.
    #[serde(rename = "userInfo")]
    pub user_info: Option<GoogleSecuritySafebrowsingV4ThreatHitUserInfo>,
}

impl common::RequestValue for GoogleSecuritySafebrowsingV4ThreatHit {}

/// A single resource related to a threat hit.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleSecuritySafebrowsingV4ThreatHitThreatSource {
    /// Referrer of the resource. Only set if the referrer is available.
    pub referrer: Option<String>,
    /// The remote IP of the resource in ASCII format. Either IPv4 or IPv6.
    #[serde(rename = "remoteIp")]
    pub remote_ip: Option<String>,
    /// The type of source reported.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// The URL of the resource.
    pub url: Option<String>,
}

impl common::Part for GoogleSecuritySafebrowsingV4ThreatHitThreatSource {}

/// Details about the user that encountered the threat.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleSecuritySafebrowsingV4ThreatHitUserInfo {
    /// The UN M.49 region code associated with the user's location.
    #[serde(rename = "regionCode")]
    pub region_code: Option<String>,
    /// Unique user identifier defined by the client.
    #[serde(rename = "userId")]
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub user_id: Option<Vec<u8>>,
}

impl common::Part for GoogleSecuritySafebrowsingV4ThreatHitUserInfo {}

/// The information regarding one or more threats that a client submits when checking for matches in threat lists.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleSecuritySafebrowsingV4ThreatInfo {
    /// The platform types to be checked.
    #[serde(rename = "platformTypes")]
    pub platform_types: Option<Vec<String>>,
    /// The threat entries to be checked.
    #[serde(rename = "threatEntries")]
    pub threat_entries: Option<Vec<GoogleSecuritySafebrowsingV4ThreatEntry>>,
    /// The entry types to be checked.
    #[serde(rename = "threatEntryTypes")]
    pub threat_entry_types: Option<Vec<String>>,
    /// The threat types to be checked.
    #[serde(rename = "threatTypes")]
    pub threat_types: Option<Vec<String>>,
}

impl common::Part for GoogleSecuritySafebrowsingV4ThreatInfo {}

/// Describes an individual threat list. A list is defined by three parameters: the type of threat posed, the type of platform targeted by the threat, and the type of entries in the list.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleSecuritySafebrowsingV4ThreatListDescriptor {
    /// The platform type targeted by the list's entries.
    #[serde(rename = "platformType")]
    pub platform_type: Option<String>,
    /// The entry types contained in the list.
    #[serde(rename = "threatEntryType")]
    pub threat_entry_type: Option<String>,
    /// The threat type posed by the list's entries.
    #[serde(rename = "threatType")]
    pub threat_type: Option<String>,
}

impl common::Part for GoogleSecuritySafebrowsingV4ThreatListDescriptor {}

/// A match when checking a threat entry in the Safe Browsing threat lists.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleSecuritySafebrowsingV4ThreatMatch {
    /// The cache lifetime for the returned match. Clients must not cache this response for more than this duration to avoid false positives.
    #[serde(rename = "cacheDuration")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub cache_duration: Option<chrono::Duration>,
    /// The platform type matching this threat.
    #[serde(rename = "platformType")]
    pub platform_type: Option<String>,
    /// The threat matching this threat.
    pub threat: Option<GoogleSecuritySafebrowsingV4ThreatEntry>,
    /// Optional metadata associated with this threat.
    #[serde(rename = "threatEntryMetadata")]
    pub threat_entry_metadata: Option<GoogleSecuritySafebrowsingV4ThreatEntryMetadata>,
    /// The threat entry type matching this threat.
    #[serde(rename = "threatEntryType")]
    pub threat_entry_type: Option<String>,
    /// The threat type matching this threat.
    #[serde(rename = "threatType")]
    pub threat_type: Option<String>,
}

impl common::Part for GoogleSecuritySafebrowsingV4ThreatMatch {}

// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *encodedFullHash* resources.
/// It is not used directly, but through the [`Safebrowsing`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_safebrowsing4 as safebrowsing4;
///
/// # async fn dox() {
/// use safebrowsing4::{Safebrowsing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http1()
///         .build()
/// );
/// let mut hub = Safebrowsing::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.encoded_full_hashes();
/// # }
/// ```
pub struct EncodedFullHashMethods<'a, C>
where
    C: 'a,
{
    hub: &'a Safebrowsing<C>,
}

impl<'a, C> common::MethodsBuilder for EncodedFullHashMethods<'a, C> {}

impl<'a, C> EncodedFullHashMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    ///
    ///
    /// # Arguments
    ///
    /// * `encodedRequest` - A serialized FindFullHashesRequest proto.
    pub fn get(&self, encoded_request: Vec<u8>) -> EncodedFullHashGetCall<'a, C> {
        EncodedFullHashGetCall {
            hub: self.hub,
            _encoded_request: encoded_request,
            _client_version: Default::default(),
            _client_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *encodedUpdate* resources.
/// It is not used directly, but through the [`Safebrowsing`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_safebrowsing4 as safebrowsing4;
///
/// # async fn dox() {
/// use safebrowsing4::{Safebrowsing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http1()
///         .build()
/// );
/// let mut hub = Safebrowsing::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.encoded_updates();
/// # }
/// ```
pub struct EncodedUpdateMethods<'a, C>
where
    C: 'a,
{
    hub: &'a Safebrowsing<C>,
}

impl<'a, C> common::MethodsBuilder for EncodedUpdateMethods<'a, C> {}

impl<'a, C> EncodedUpdateMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    ///
    ///
    /// # Arguments
    ///
    /// * `encodedRequest` - A serialized FetchThreatListUpdatesRequest proto.
    pub fn get(&self, encoded_request: Vec<u8>) -> EncodedUpdateGetCall<'a, C> {
        EncodedUpdateGetCall {
            hub: self.hub,
            _encoded_request: encoded_request,
            _client_version: Default::default(),
            _client_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *fullHash* resources.
/// It is not used directly, but through the [`Safebrowsing`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_safebrowsing4 as safebrowsing4;
///
/// # async fn dox() {
/// use safebrowsing4::{Safebrowsing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http1()
///         .build()
/// );
/// let mut hub = Safebrowsing::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `find(...)`
/// // to build up your call.
/// let rb = hub.full_hashes();
/// # }
/// ```
pub struct FullHashMethods<'a, C>
where
    C: 'a,
{
    hub: &'a Safebrowsing<C>,
}

impl<'a, C> common::MethodsBuilder for FullHashMethods<'a, C> {}

impl<'a, C> FullHashMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Finds the full hashes that match the requested hash prefixes.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn find(
        &self,
        request: GoogleSecuritySafebrowsingV4FindFullHashesRequest,
    ) -> FullHashFindCall<'a, C> {
        FullHashFindCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *threatHit* resources.
/// It is not used directly, but through the [`Safebrowsing`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_safebrowsing4 as safebrowsing4;
///
/// # async fn dox() {
/// use safebrowsing4::{Safebrowsing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http1()
///         .build()
/// );
/// let mut hub = Safebrowsing::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`
/// // to build up your call.
/// let rb = hub.threat_hits();
/// # }
/// ```
pub struct ThreatHitMethods<'a, C>
where
    C: 'a,
{
    hub: &'a Safebrowsing<C>,
}

impl<'a, C> common::MethodsBuilder for ThreatHitMethods<'a, C> {}

impl<'a, C> ThreatHitMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Reports a Safe Browsing threat list hit to Google. Only projects with TRUSTED_REPORTER visibility can use this method.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(
        &self,
        request: GoogleSecuritySafebrowsingV4ThreatHit,
    ) -> ThreatHitCreateCall<'a, C> {
        ThreatHitCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *threatListUpdate* resources.
/// It is not used directly, but through the [`Safebrowsing`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_safebrowsing4 as safebrowsing4;
///
/// # async fn dox() {
/// use safebrowsing4::{Safebrowsing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http1()
///         .build()
/// );
/// let mut hub = Safebrowsing::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `fetch(...)`
/// // to build up your call.
/// let rb = hub.threat_list_updates();
/// # }
/// ```
pub struct ThreatListUpdateMethods<'a, C>
where
    C: 'a,
{
    hub: &'a Safebrowsing<C>,
}

impl<'a, C> common::MethodsBuilder for ThreatListUpdateMethods<'a, C> {}

impl<'a, C> ThreatListUpdateMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches the most recent threat list updates. A client can request updates for multiple lists at once.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn fetch(
        &self,
        request: GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequest,
    ) -> ThreatListUpdateFetchCall<'a, C> {
        ThreatListUpdateFetchCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *threatList* resources.
/// It is not used directly, but through the [`Safebrowsing`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_safebrowsing4 as safebrowsing4;
///
/// # async fn dox() {
/// use safebrowsing4::{Safebrowsing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http1()
///         .build()
/// );
/// let mut hub = Safebrowsing::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.threat_lists();
/// # }
/// ```
pub struct ThreatListMethods<'a, C>
where
    C: 'a,
{
    hub: &'a Safebrowsing<C>,
}

impl<'a, C> common::MethodsBuilder for ThreatListMethods<'a, C> {}

impl<'a, C> ThreatListMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the Safe Browsing threat lists available for download.
    pub fn list(&self) -> ThreatListListCall<'a, C> {
        ThreatListListCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *threatMatch* resources.
/// It is not used directly, but through the [`Safebrowsing`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_safebrowsing4 as safebrowsing4;
///
/// # async fn dox() {
/// use safebrowsing4::{Safebrowsing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http1()
///         .build()
/// );
/// let mut hub = Safebrowsing::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `find(...)`
/// // to build up your call.
/// let rb = hub.threat_matches();
/// # }
/// ```
pub struct ThreatMatchMethods<'a, C>
where
    C: 'a,
{
    hub: &'a Safebrowsing<C>,
}

impl<'a, C> common::MethodsBuilder for ThreatMatchMethods<'a, C> {}

impl<'a, C> ThreatMatchMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Finds the threat entries that match the Safe Browsing lists.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn find(
        &self,
        request: GoogleSecuritySafebrowsingV4FindThreatMatchesRequest,
    ) -> ThreatMatchFindCall<'a, C> {
        ThreatMatchFindCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}

// ###################
// CallBuilders   ###
// #################

///
///
/// A builder for the *get* method supported by a *encodedFullHash* resource.
/// It is not used directly, but through a [`EncodedFullHashMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_safebrowsing4 as safebrowsing4;
/// # async fn dox() {
/// # use safebrowsing4::{Safebrowsing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = Safebrowsing::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.encoded_full_hashes().get(vec![0, 1, 2, 3])
///              .client_version("sanctus")
///              .client_id("sed")
///              .doit().await;
/// # }
/// ```
pub struct EncodedFullHashGetCall<'a, C>
where
    C: 'a,
{
    hub: &'a Safebrowsing<C>,
    _encoded_request: Vec<u8>,
    _client_version: Option<String>,
    _client_id: Option<String>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C> common::CallBuilder for EncodedFullHashGetCall<'a, C> {}

impl<'a, C> EncodedFullHashGetCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(
        mut self,
    ) -> common::Result<(
        common::Response,
        GoogleSecuritySafebrowsingV4FindFullHashesResponse,
    )> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "safebrowsing.encodedFullHashes.get",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "encodedRequest", "clientVersion", "clientId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push(
            "encodedRequest",
            common::serde::standard_base64::to_string(&self._encoded_request),
        );
        if let Some(value) = self._client_version.as_ref() {
            params.push("clientVersion", value);
        }
        if let Some(value) = self._client_id.as_ref() {
            params.push("clientId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/encodedFullHashes/{encodedRequest}";

        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(common::Error::MissingAPIKey);
            }
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{encodedRequest}", "encodedRequest")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["encodedRequest"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// A serialized FindFullHashesRequest proto.
    ///
    /// Sets the *encoded request* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn encoded_request(mut self, new_value: Vec<u8>) -> EncodedFullHashGetCall<'a, C> {
        self._encoded_request = new_value;
        self
    }
    /// The version of the client implementation.
    ///
    /// Sets the *client version* query property to the given value.
    pub fn client_version(mut self, new_value: &str) -> EncodedFullHashGetCall<'a, C> {
        self._client_version = Some(new_value.to_string());
        self
    }
    /// A client ID that (hopefully) uniquely identifies the client implementation of the Safe Browsing API.
    ///
    /// Sets the *client id* query property to the given value.
    pub fn client_id(mut self, new_value: &str) -> EncodedFullHashGetCall<'a, C> {
        self._client_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> EncodedFullHashGetCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> EncodedFullHashGetCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }
}

///
///
/// A builder for the *get* method supported by a *encodedUpdate* resource.
/// It is not used directly, but through a [`EncodedUpdateMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_safebrowsing4 as safebrowsing4;
/// # async fn dox() {
/// # use safebrowsing4::{Safebrowsing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = Safebrowsing::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.encoded_updates().get(vec![0, 1, 2, 3])
///              .client_version("amet.")
///              .client_id("takimata")
///              .doit().await;
/// # }
/// ```
pub struct EncodedUpdateGetCall<'a, C>
where
    C: 'a,
{
    hub: &'a Safebrowsing<C>,
    _encoded_request: Vec<u8>,
    _client_version: Option<String>,
    _client_id: Option<String>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C> common::CallBuilder for EncodedUpdateGetCall<'a, C> {}

impl<'a, C> EncodedUpdateGetCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(
        mut self,
    ) -> common::Result<(
        common::Response,
        GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponse,
    )> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "safebrowsing.encodedUpdates.get",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "encodedRequest", "clientVersion", "clientId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push(
            "encodedRequest",
            common::serde::standard_base64::to_string(&self._encoded_request),
        );
        if let Some(value) = self._client_version.as_ref() {
            params.push("clientVersion", value);
        }
        if let Some(value) = self._client_id.as_ref() {
            params.push("clientId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/encodedUpdates/{encodedRequest}";

        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(common::Error::MissingAPIKey);
            }
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{encodedRequest}", "encodedRequest")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["encodedRequest"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// A serialized FetchThreatListUpdatesRequest proto.
    ///
    /// Sets the *encoded request* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn encoded_request(mut self, new_value: Vec<u8>) -> EncodedUpdateGetCall<'a, C> {
        self._encoded_request = new_value;
        self
    }
    /// The version of the client implementation.
    ///
    /// Sets the *client version* query property to the given value.
    pub fn client_version(mut self, new_value: &str) -> EncodedUpdateGetCall<'a, C> {
        self._client_version = Some(new_value.to_string());
        self
    }
    /// A client ID that uniquely identifies the client implementation of the Safe Browsing API.
    ///
    /// Sets the *client id* query property to the given value.
    pub fn client_id(mut self, new_value: &str) -> EncodedUpdateGetCall<'a, C> {
        self._client_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> EncodedUpdateGetCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> EncodedUpdateGetCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }
}

/// Finds the full hashes that match the requested hash prefixes.
///
/// A builder for the *find* method supported by a *fullHash* resource.
/// It is not used directly, but through a [`FullHashMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_safebrowsing4 as safebrowsing4;
/// use safebrowsing4::api::GoogleSecuritySafebrowsingV4FindFullHashesRequest;
/// # async fn dox() {
/// # use safebrowsing4::{Safebrowsing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = Safebrowsing::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleSecuritySafebrowsingV4FindFullHashesRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.full_hashes().find(req)
///              .doit().await;
/// # }
/// ```
pub struct FullHashFindCall<'a, C>
where
    C: 'a,
{
    hub: &'a Safebrowsing<C>,
    _request: GoogleSecuritySafebrowsingV4FindFullHashesRequest,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C> common::CallBuilder for FullHashFindCall<'a, C> {}

impl<'a, C> FullHashFindCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(
        mut self,
    ) -> common::Result<(
        common::Response,
        GoogleSecuritySafebrowsingV4FindFullHashesResponse,
    )> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "safebrowsing.fullHashes.find",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/fullHashes:find";

        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(common::Error::MissingAPIKey);
            }
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(
        mut self,
        new_value: GoogleSecuritySafebrowsingV4FindFullHashesRequest,
    ) -> FullHashFindCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> FullHashFindCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> FullHashFindCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }
}

/// Reports a Safe Browsing threat list hit to Google. Only projects with TRUSTED_REPORTER visibility can use this method.
///
/// A builder for the *create* method supported by a *threatHit* resource.
/// It is not used directly, but through a [`ThreatHitMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_safebrowsing4 as safebrowsing4;
/// use safebrowsing4::api::GoogleSecuritySafebrowsingV4ThreatHit;
/// # async fn dox() {
/// # use safebrowsing4::{Safebrowsing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = Safebrowsing::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleSecuritySafebrowsingV4ThreatHit::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.threat_hits().create(req)
///              .doit().await;
/// # }
/// ```
pub struct ThreatHitCreateCall<'a, C>
where
    C: 'a,
{
    hub: &'a Safebrowsing<C>,
    _request: GoogleSecuritySafebrowsingV4ThreatHit,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C> common::CallBuilder for ThreatHitCreateCall<'a, C> {}

impl<'a, C> ThreatHitCreateCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, GoogleProtobufEmpty)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "safebrowsing.threatHits.create",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/threatHits";

        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(common::Error::MissingAPIKey);
            }
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(
        mut self,
        new_value: GoogleSecuritySafebrowsingV4ThreatHit,
    ) -> ThreatHitCreateCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ThreatHitCreateCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ThreatHitCreateCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }
}

/// Fetches the most recent threat list updates. A client can request updates for multiple lists at once.
///
/// A builder for the *fetch* method supported by a *threatListUpdate* resource.
/// It is not used directly, but through a [`ThreatListUpdateMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_safebrowsing4 as safebrowsing4;
/// use safebrowsing4::api::GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequest;
/// # async fn dox() {
/// # use safebrowsing4::{Safebrowsing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = Safebrowsing::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.threat_list_updates().fetch(req)
///              .doit().await;
/// # }
/// ```
pub struct ThreatListUpdateFetchCall<'a, C>
where
    C: 'a,
{
    hub: &'a Safebrowsing<C>,
    _request: GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequest,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C> common::CallBuilder for ThreatListUpdateFetchCall<'a, C> {}

impl<'a, C> ThreatListUpdateFetchCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(
        mut self,
    ) -> common::Result<(
        common::Response,
        GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponse,
    )> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "safebrowsing.threatListUpdates.fetch",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/threatListUpdates:fetch";

        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(common::Error::MissingAPIKey);
            }
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(
        mut self,
        new_value: GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequest,
    ) -> ThreatListUpdateFetchCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ThreatListUpdateFetchCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ThreatListUpdateFetchCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }
}

/// Lists the Safe Browsing threat lists available for download.
///
/// A builder for the *list* method supported by a *threatList* resource.
/// It is not used directly, but through a [`ThreatListMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_safebrowsing4 as safebrowsing4;
/// # async fn dox() {
/// # use safebrowsing4::{Safebrowsing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = Safebrowsing::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.threat_lists().list()
///              .doit().await;
/// # }
/// ```
pub struct ThreatListListCall<'a, C>
where
    C: 'a,
{
    hub: &'a Safebrowsing<C>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C> common::CallBuilder for ThreatListListCall<'a, C> {}

impl<'a, C> ThreatListListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(
        mut self,
    ) -> common::Result<(
        common::Response,
        GoogleSecuritySafebrowsingV4ListThreatListsResponse,
    )> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "safebrowsing.threatLists.list",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(2 + self._additional_params.len());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/threatLists";

        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(common::Error::MissingAPIKey);
            }
        }

        let url = params.parse_with_url(&url);

        loop {
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ThreatListListCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ThreatListListCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }
}

/// Finds the threat entries that match the Safe Browsing lists.
///
/// A builder for the *find* method supported by a *threatMatch* resource.
/// It is not used directly, but through a [`ThreatMatchMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_safebrowsing4 as safebrowsing4;
/// use safebrowsing4::api::GoogleSecuritySafebrowsingV4FindThreatMatchesRequest;
/// # async fn dox() {
/// # use safebrowsing4::{Safebrowsing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = Safebrowsing::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleSecuritySafebrowsingV4FindThreatMatchesRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.threat_matches().find(req)
///              .doit().await;
/// # }
/// ```
pub struct ThreatMatchFindCall<'a, C>
where
    C: 'a,
{
    hub: &'a Safebrowsing<C>,
    _request: GoogleSecuritySafebrowsingV4FindThreatMatchesRequest,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C> common::CallBuilder for ThreatMatchFindCall<'a, C> {}

impl<'a, C> ThreatMatchFindCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(
        mut self,
    ) -> common::Result<(
        common::Response,
        GoogleSecuritySafebrowsingV4FindThreatMatchesResponse,
    )> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "safebrowsing.threatMatches.find",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/threatMatches:find";

        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(common::Error::MissingAPIKey);
            }
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(
        mut self,
        new_value: GoogleSecuritySafebrowsingV4FindThreatMatchesRequest,
    ) -> ThreatMatchFindCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ThreatMatchFindCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ThreatMatchFindCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }
}
