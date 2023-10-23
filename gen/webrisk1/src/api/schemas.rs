use super::*;
/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [compute diff threat lists](ThreatListComputeDiffCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudWebriskV1ComputeThreatListDiffResponse {
    /// A set of entries to add to a local threat type's list.
    
    pub additions: Option<GoogleCloudWebriskV1ThreatEntryAdditions>,
    /// The expected SHA256 hash of the client state; that is, of the sorted list of all hashes present in the database after applying the provided diff. If the client state doesn't match the expected state, the client must discard this diff and retry later.
    
    pub checksum: Option<GoogleCloudWebriskV1ComputeThreatListDiffResponseChecksum>,
    /// The new opaque client version token. This should be retained by the client and passed into the next call of ComputeThreatListDiff as 'version_token'. A separate version token should be stored and used for each threatList.
    #[serde(rename="newVersionToken")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub new_version_token: Option<Vec<u8>>,
    /// The soonest the client should wait before issuing any diff request. Querying sooner is unlikely to produce a meaningful diff. Waiting longer is acceptable considering the use case. If this field is not set clients may update as soon as they want.
    #[serde(rename="recommendedNextDiff")]
    
    pub recommended_next_diff: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A set of entries to remove from a local threat type's list. This field may be empty.
    
    pub removals: Option<GoogleCloudWebriskV1ThreatEntryRemovals>,
    /// The type of response. This may indicate that an action must be taken by the client when the response is received.
    #[serde(rename="responseType")]
    
    pub response_type: Option<GoogleCloudWebriskV1ComputeThreatListDiffResponseResponseTypeEnum>,
}

impl client::ResponseResult for GoogleCloudWebriskV1ComputeThreatListDiffResponse {}


/// The expected state of a client's local database.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudWebriskV1ComputeThreatListDiffResponseChecksum {
    /// The SHA256 hash of the client state; that is, of the sorted list of all hashes present in the database.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub sha256: Option<Vec<u8>>,
}

impl client::Part for GoogleCloudWebriskV1ComputeThreatListDiffResponseChecksum {}


/// The uncompressed threat entries in hash format. Hashes can be anywhere from 4 to 32 bytes in size. A large majority are 4 bytes, but some hashes are lengthened if they collide with the hash of a popular URI. Used for sending ThreatEntryAdditons to clients that do not support compression, or when sending non-4-byte hashes to clients that do support compression.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudWebriskV1RawHashes {
    /// The number of bytes for each prefix encoded below. This field can be anywhere from 4 (shortest prefix) to 32 (full SHA256 hash). In practice this is almost always 4, except in exceptional circumstances.
    #[serde(rename="prefixSize")]
    
    pub prefix_size: Option<i32>,
    /// The hashes, in binary format, concatenated into one long string. Hashes are sorted in lexicographic order. For JSON API users, hashes are base64-encoded.
    #[serde(rename="rawHashes")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub raw_hashes: Option<Vec<u8>>,
}

impl client::Part for GoogleCloudWebriskV1RawHashes {}


/// A set of raw indices to remove from a local list.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudWebriskV1RawIndices {
    /// The indices to remove from a lexicographically-sorted local list.
    
    pub indices: Option<Vec<i32>>,
}

impl client::Part for GoogleCloudWebriskV1RawIndices {}


/// The Rice-Golomb encoded data. Used for sending compressed 4-byte hashes or compressed removal indices.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudWebriskV1RiceDeltaEncoding {
    /// The encoded deltas that are encoded using the Golomb-Rice coder.
    #[serde(rename="encodedData")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub encoded_data: Option<Vec<u8>>,
    /// The number of entries that are delta encoded in the encoded data. If only a single integer was encoded, this will be zero and the single value will be stored in `first_value`.
    #[serde(rename="entryCount")]
    
    pub entry_count: Option<i32>,
    /// The offset of the first entry in the encoded data, or, if only a single integer was encoded, that single integer's value. If the field is empty or missing, assume zero.
    #[serde(rename="firstValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub first_value: Option<i64>,
    /// The Golomb-Rice parameter, which is a number between 2 and 28. This field is missing (that is, zero) if `num_entries` is zero.
    #[serde(rename="riceParameter")]
    
    pub rice_parameter: Option<i32>,
}

impl client::Part for GoogleCloudWebriskV1RiceDeltaEncoding {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search hashes](HashSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudWebriskV1SearchHashesResponse {
    /// For requested entities that did not match the threat list, how long to cache the response until.
    #[serde(rename="negativeExpireTime")]
    
    pub negative_expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The full hashes that matched the requested prefixes. The hash will be populated in the key.
    
    pub threats: Option<Vec<GoogleCloudWebriskV1SearchHashesResponseThreatHash>>,
}

impl client::ResponseResult for GoogleCloudWebriskV1SearchHashesResponse {}


/// Contains threat information on a matching hash.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudWebriskV1SearchHashesResponseThreatHash {
    /// The cache lifetime for the returned match. Clients must not cache this response past this timestamp to avoid false positives.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A 32 byte SHA256 hash. This field is in binary format. For JSON requests, hashes are base64-encoded.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub hash: Option<Vec<u8>>,
    /// The ThreatList this threat belongs to. This must contain at least one entry.
    #[serde(rename="threatTypes")]
    
    pub threat_types: Option<Vec<GoogleCloudWebriskV1SearchHashesResponseThreatHashThreatTypesEnum>>,
}

impl client::Part for GoogleCloudWebriskV1SearchHashesResponseThreatHash {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search uris](UriSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudWebriskV1SearchUrisResponse {
    /// The threat list matches. This might be empty if the URI is on no list.
    
    pub threat: Option<GoogleCloudWebriskV1SearchUrisResponseThreatUri>,
}

impl client::ResponseResult for GoogleCloudWebriskV1SearchUrisResponse {}


/// Contains threat information on a matching uri.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudWebriskV1SearchUrisResponseThreatUri {
    /// The cache lifetime for the returned match. Clients must not cache this response past this timestamp to avoid false positives.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The ThreatList this threat belongs to.
    #[serde(rename="threatTypes")]
    
    pub threat_types: Option<Vec<GoogleCloudWebriskV1SearchUrisResponseThreatUriThreatTypesEnum>>,
}

impl client::Part for GoogleCloudWebriskV1SearchUrisResponseThreatUri {}


/// Wraps a URI that might be displaying malicious content.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [submissions create projects](ProjectSubmissionCreateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudWebriskV1Submission {
    /// ThreatTypes found to be associated with the submitted URI after reviewing it. This might be empty if the URI was not added to any list.
    #[serde(rename="threatTypes")]
    
    pub threat_types: Option<Vec<GoogleCloudWebriskV1SubmissionThreatTypesEnum>>,
    /// Required. The URI that is being reported for malicious content to be analyzed.
    
    pub uri: Option<String>,
}

impl client::RequestValue for GoogleCloudWebriskV1Submission {}
impl client::ResponseResult for GoogleCloudWebriskV1Submission {}


/// Request to send a potentially malicious URI to WebRisk.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [uris submit projects](ProjectUriSubmitCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudWebriskV1SubmitUriRequest {
    /// Required. The submission that contains the URI to be scanned.
    
    pub submission: Option<GoogleCloudWebriskV1Submission>,
}

impl client::RequestValue for GoogleCloudWebriskV1SubmitUriRequest {}


/// Contains the set of entries to add to a local database. May contain a combination of compressed and raw data in a single response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudWebriskV1ThreatEntryAdditions {
    /// The raw SHA256-formatted entries. Repeated to allow returning sets of hashes with different prefix sizes.
    #[serde(rename="rawHashes")]
    
    pub raw_hashes: Option<Vec<GoogleCloudWebriskV1RawHashes>>,
    /// The encoded 4-byte prefixes of SHA256-formatted entries, using a Golomb-Rice encoding. The hashes are converted to uint32, sorted in ascending order, then delta encoded and stored as encoded_data.
    #[serde(rename="riceHashes")]
    
    pub rice_hashes: Option<GoogleCloudWebriskV1RiceDeltaEncoding>,
}

impl client::Part for GoogleCloudWebriskV1ThreatEntryAdditions {}


/// Contains the set of entries to remove from a local database.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudWebriskV1ThreatEntryRemovals {
    /// The raw removal indices for a local list.
    #[serde(rename="rawIndices")]
    
    pub raw_indices: Option<GoogleCloudWebriskV1RawIndices>,
    /// The encoded local, lexicographically-sorted list indices, using a Golomb-Rice encoding. Used for sending compressed removal indices. The removal indices (uint32) are sorted in ascending order, then delta encoded and stored as encoded_data.
    #[serde(rename="riceIndices")]
    
    pub rice_indices: Option<GoogleCloudWebriskV1RiceDeltaEncoding>,
}

impl client::Part for GoogleCloudWebriskV1ThreatEntryRemovals {}


/// The request message for Operations.CancelOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [operations cancel projects](ProjectOperationCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningCancelOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleLongrunningCancelOperationRequest {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [operations list projects](ProjectOperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningListOperationsResponse {
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of operations that matches the specified filter in the request.
    
    pub operations: Option<Vec<GoogleLongrunningOperation>>,
}

impl client::ResponseResult for GoogleLongrunningListOperationsResponse {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [operations get projects](ProjectOperationGetCall) (response)
/// * [uris submit projects](ProjectUriSubmitCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningOperation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<GoogleRpcStatus>,
    /// Contains a `SubmitUriMetadata` object.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// Matches the `/v1/{project-name}/operations/{operation-id}` pattern.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for GoogleLongrunningOperation {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [operations cancel projects](ProjectOperationCancelCall) (response)
/// * [operations delete projects](ProjectOperationDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleProtobufEmpty { _never_set: Option<bool> }

impl client::ResponseResult for GoogleProtobufEmpty {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleRpcStatus {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for GoogleRpcStatus {}


