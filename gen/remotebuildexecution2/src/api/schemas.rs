use super::*;
/// Describes the server/instance capabilities for updating the action cache.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2ActionCacheUpdateCapabilities {
    /// no description provided
    #[serde(rename="updateEnabled")]
    
    pub update_enabled: Option<bool>,
}

impl client::Part for BuildBazelRemoteExecutionV2ActionCacheUpdateCapabilities {}


/// An ActionResult represents the result of an Action being run. It is advised that at least one field (for example `ActionResult.execution_metadata.Worker`) have a non-default value, to ensure that the serialized value is non-empty, which can then be used as a basic data sanity check.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get action results](ActionResultGetCall) (response)
/// * [update action results](ActionResultUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2ActionResult {
    /// The details of the execution that originally produced this result.
    #[serde(rename="executionMetadata")]
    
    pub execution_metadata: Option<BuildBazelRemoteExecutionV2ExecutedActionMetadata>,
    /// The exit code of the command.
    #[serde(rename="exitCode")]
    
    pub exit_code: Option<i32>,
    /// The output directories of the action. For each output directory requested in the `output_directories` or `output_paths` field of the Action, if the corresponding directory existed after the action completed, a single entry will be present in the output list, which will contain the digest of a Tree message containing the directory tree, and the path equal exactly to the corresponding Action output_directories member. As an example, suppose the Action had an output directory `a/b/dir` and the execution produced the following contents in `a/b/dir`: a file named `bar` and a directory named `foo` with an executable file named `baz`. Then, output_directory will contain (hashes shortened for readability): ```json // OutputDirectory proto: { path: "a/b/dir" tree_digest: { hash: "4a73bc9d03...", size: 55 } } // Tree proto with hash "4a73bc9d03..." and size 55: { root: { files: [ { name: "bar", digest: { hash: "4a73bc9d03...", size: 65534 } } ], directories: [ { name: "foo", digest: { hash: "4cf2eda940...", size: 43 } } ] } children : { // (Directory proto with hash "4cf2eda940..." and size 43) files: [ { name: "baz", digest: { hash: "b2c941073e...", size: 1294, }, is_executable: true } ] } } ``` If an output of the same name as listed in `output_files` of the Command was found in `output_directories`, but was not a directory, the server will return a FAILED_PRECONDITION.
    #[serde(rename="outputDirectories")]
    
    pub output_directories: Option<Vec<BuildBazelRemoteExecutionV2OutputDirectory>>,
    /// The output directories of the action that are symbolic links to other directories. Those may be links to other output directories, or input directories, or even absolute paths outside of the working directory, if the server supports SymlinkAbsolutePathStrategy.ALLOWED. For each output directory requested in the `output_directories` field of the Action, if the directory existed after the action completed, a single entry will be present either in this field, or in the `output_directories` field, if the directory was not a symbolic link. If an output of the same name was found, but was a symbolic link to a file instead of a directory, the server will return a FAILED_PRECONDITION. If the action does not produce the requested output, then that output will be omitted from the list. The server is free to arrange the output list as desired; clients MUST NOT assume that the output list is sorted. DEPRECATED as of v2.1. Servers that wish to be compatible with v2.0 API should still populate this field in addition to `output_symlinks`.
    #[serde(rename="outputDirectorySymlinks")]
    
    pub output_directory_symlinks: Option<Vec<BuildBazelRemoteExecutionV2OutputSymlink>>,
    /// The output files of the action that are symbolic links to other files. Those may be links to other output files, or input files, or even absolute paths outside of the working directory, if the server supports SymlinkAbsolutePathStrategy.ALLOWED. For each output file requested in the `output_files` or `output_paths` field of the Action, if the corresponding file existed after the action completed, a single entry will be present either in this field, or in the `output_files` field, if the file was not a symbolic link. If an output symbolic link of the same name as listed in `output_files` of the Command was found, but its target type was not a regular file, the server will return a FAILED_PRECONDITION. If the action does not produce the requested output, then that output will be omitted from the list. The server is free to arrange the output list as desired; clients MUST NOT assume that the output list is sorted. DEPRECATED as of v2.1. Servers that wish to be compatible with v2.0 API should still populate this field in addition to `output_symlinks`.
    #[serde(rename="outputFileSymlinks")]
    
    pub output_file_symlinks: Option<Vec<BuildBazelRemoteExecutionV2OutputSymlink>>,
    /// The output files of the action. For each output file requested in the `output_files` or `output_paths` field of the Action, if the corresponding file existed after the action completed, a single entry will be present either in this field, or the `output_file_symlinks` field if the file was a symbolic link to another file (`output_symlinks` field after v2.1). If an output listed in `output_files` was found, but was a directory rather than a regular file, the server will return a FAILED_PRECONDITION. If the action does not produce the requested output, then that output will be omitted from the list. The server is free to arrange the output list as desired; clients MUST NOT assume that the output list is sorted.
    #[serde(rename="outputFiles")]
    
    pub output_files: Option<Vec<BuildBazelRemoteExecutionV2OutputFile>>,
    /// New in v2.1: this field will only be populated if the command `output_paths` field was used, and not the pre v2.1 `output_files` or `output_directories` fields. The output paths of the action that are symbolic links to other paths. Those may be links to other outputs, or inputs, or even absolute paths outside of the working directory, if the server supports SymlinkAbsolutePathStrategy.ALLOWED. A single entry for each output requested in `output_paths` field of the Action, if the corresponding path existed after the action completed and was a symbolic link. If the action does not produce a requested output, then that output will be omitted from the list. The server is free to arrange the output list as desired; clients MUST NOT assume that the output list is sorted.
    #[serde(rename="outputSymlinks")]
    
    pub output_symlinks: Option<Vec<BuildBazelRemoteExecutionV2OutputSymlink>>,
    /// The digest for a blob containing the standard error of the action, which can be retrieved from the ContentAddressableStorage.
    #[serde(rename="stderrDigest")]
    
    pub stderr_digest: Option<BuildBazelRemoteExecutionV2Digest>,
    /// The standard error buffer of the action. The server SHOULD NOT inline stderr unless requested by the client in the GetActionResultRequest message. The server MAY omit inlining, even if requested, and MUST do so if inlining would cause the response to exceed message size limits.
    #[serde(rename="stderrRaw")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub stderr_raw: Option<Vec<u8>>,
    /// The digest for a blob containing the standard output of the action, which can be retrieved from the ContentAddressableStorage.
    #[serde(rename="stdoutDigest")]
    
    pub stdout_digest: Option<BuildBazelRemoteExecutionV2Digest>,
    /// The standard output buffer of the action. The server SHOULD NOT inline stdout unless requested by the client in the GetActionResultRequest message. The server MAY omit inlining, even if requested, and MUST do so if inlining would cause the response to exceed message size limits.
    #[serde(rename="stdoutRaw")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub stdout_raw: Option<Vec<u8>>,
}

impl client::RequestValue for BuildBazelRemoteExecutionV2ActionResult {}
impl client::ResponseResult for BuildBazelRemoteExecutionV2ActionResult {}


/// A request message for ContentAddressableStorage.BatchReadBlobs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch read blobs](BlobBatchReadCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2BatchReadBlobsRequest {
    /// The individual blob digests.
    
    pub digests: Option<Vec<BuildBazelRemoteExecutionV2Digest>>,
}

impl client::RequestValue for BuildBazelRemoteExecutionV2BatchReadBlobsRequest {}


/// A response message for ContentAddressableStorage.BatchReadBlobs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch read blobs](BlobBatchReadCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2BatchReadBlobsResponse {
    /// The responses to the requests.
    
    pub responses: Option<Vec<BuildBazelRemoteExecutionV2BatchReadBlobsResponseResponse>>,
}

impl client::ResponseResult for BuildBazelRemoteExecutionV2BatchReadBlobsResponse {}


/// A response corresponding to a single blob that the client tried to download.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2BatchReadBlobsResponseResponse {
    /// The raw binary data.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// The digest to which this response corresponds.
    
    pub digest: Option<BuildBazelRemoteExecutionV2Digest>,
    /// The result of attempting to download that blob.
    
    pub status: Option<GoogleRpcStatus>,
}

impl client::Part for BuildBazelRemoteExecutionV2BatchReadBlobsResponseResponse {}


/// A request message for ContentAddressableStorage.BatchUpdateBlobs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch update blobs](BlobBatchUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2BatchUpdateBlobsRequest {
    /// The individual upload requests.
    
    pub requests: Option<Vec<BuildBazelRemoteExecutionV2BatchUpdateBlobsRequestRequest>>,
}

impl client::RequestValue for BuildBazelRemoteExecutionV2BatchUpdateBlobsRequest {}


/// A request corresponding to a single blob that the client wants to upload.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2BatchUpdateBlobsRequestRequest {
    /// The raw binary data.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// The digest of the blob. This MUST be the digest of `data`.
    
    pub digest: Option<BuildBazelRemoteExecutionV2Digest>,
}

impl client::Part for BuildBazelRemoteExecutionV2BatchUpdateBlobsRequestRequest {}


/// A response message for ContentAddressableStorage.BatchUpdateBlobs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch update blobs](BlobBatchUpdateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2BatchUpdateBlobsResponse {
    /// The responses to the requests.
    
    pub responses: Option<Vec<BuildBazelRemoteExecutionV2BatchUpdateBlobsResponseResponse>>,
}

impl client::ResponseResult for BuildBazelRemoteExecutionV2BatchUpdateBlobsResponse {}


/// A response corresponding to a single blob that the client tried to upload.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2BatchUpdateBlobsResponseResponse {
    /// The blob digest to which this response corresponds.
    
    pub digest: Option<BuildBazelRemoteExecutionV2Digest>,
    /// The result of attempting to upload that blob.
    
    pub status: Option<GoogleRpcStatus>,
}

impl client::Part for BuildBazelRemoteExecutionV2BatchUpdateBlobsResponseResponse {}


/// Capabilities of the remote cache system.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2CacheCapabilities {
    /// Capabilities for updating the action cache.
    #[serde(rename="actionCacheUpdateCapabilities")]
    
    pub action_cache_update_capabilities: Option<BuildBazelRemoteExecutionV2ActionCacheUpdateCapabilities>,
    /// Supported cache priority range for both CAS and ActionCache.
    #[serde(rename="cachePriorityCapabilities")]
    
    pub cache_priority_capabilities: Option<BuildBazelRemoteExecutionV2PriorityCapabilities>,
    /// All the digest functions supported by the remote cache. Remote cache may support multiple digest functions simultaneously.
    #[serde(rename="digestFunction")]
    
    pub digest_function: Option<Vec<BuildBazelRemoteExecutionV2CacheCapabilityDigestFunctionEnum>>,
    /// Maximum total size of blobs to be uploaded/downloaded using batch methods. A value of 0 means no limit is set, although in practice there will always be a message size limitation of the protocol in use, e.g. GRPC.
    #[serde(rename="maxBatchTotalSizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_batch_total_size_bytes: Option<i64>,
    /// Compressors supported by the "compressed-blobs" bytestream resources. Servers MUST support identity/no-compression, even if it is not listed here. Note that this does not imply which if any compressors are supported by the server at the gRPC level.
    #[serde(rename="supportedCompressor")]
    
    pub supported_compressor: Option<Vec<BuildBazelRemoteExecutionV2CacheCapabilitySupportedCompressorEnum>>,
    /// Whether absolute symlink targets are supported.
    #[serde(rename="symlinkAbsolutePathStrategy")]
    
    pub symlink_absolute_path_strategy: Option<BuildBazelRemoteExecutionV2CacheCapabilitySymlinkAbsolutePathStrategyEnum>,
}

impl client::Part for BuildBazelRemoteExecutionV2CacheCapabilities {}


/// A content digest. A digest for a given blob consists of the size of the blob and its hash. The hash algorithm to use is defined by the server. The size is considered to be an integral part of the digest and cannot be separated. That is, even if the `hash` field is correctly specified but `size_bytes` is not, the server MUST reject the request. The reason for including the size in the digest is as follows: in a great many cases, the server needs to know the size of the blob it is about to work with prior to starting an operation with it, such as flattening Merkle tree structures or streaming it to a worker. Technically, the server could implement a separate metadata store, but this results in a significantly more complicated implementation as opposed to having the client specify the size up-front (or storing the size along with the digest in every message where digests are embedded). This does mean that the API leaks some implementation details of (what we consider to be) a reasonable server implementation, but we consider this to be a worthwhile tradeoff. When a `Digest` is used to refer to a proto message, it always refers to the message in binary encoded form. To ensure consistent hashing, clients and servers MUST ensure that they serialize messages according to the following rules, even if there are alternate valid encodings for the same message: * Fields are serialized in tag order. * There are no unknown fields. * There are no duplicate fields. * Fields are serialized according to the default semantics for their type. Most protocol buffer implementations will always follow these rules when serializing, but care should be taken to avoid shortcuts. For instance, concatenating two messages to merge them may produce duplicate fields.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2Digest {
    /// The hash. In the case of SHA-256, it will always be a lowercase hex string exactly 64 characters long.
    
    pub hash: Option<String>,
    /// The size of the blob, in bytes.
    #[serde(rename="sizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub size_bytes: Option<i64>,
}

impl client::Part for BuildBazelRemoteExecutionV2Digest {}


/// A `Directory` represents a directory node in a file tree, containing zero or more children FileNodes, DirectoryNodes and SymlinkNodes. Each `Node` contains its name in the directory, either the digest of its content (either a file blob or a `Directory` proto) or a symlink target, as well as possibly some metadata about the file or directory. In order to ensure that two equivalent directory trees hash to the same value, the following restrictions MUST be obeyed when constructing a a `Directory`: * Every child in the directory must have a path of exactly one segment. Multiple levels of directory hierarchy may not be collapsed. * Each child in the directory must have a unique path segment (file name). Note that while the API itself is case-sensitive, the environment where the Action is executed may or may not be case-sensitive. That is, it is legal to call the API with a Directory that has both "Foo" and "foo" as children, but the Action may be rejected by the remote system upon execution. * The files, directories and symlinks in the directory must each be sorted in lexicographical order by path. The path strings must be sorted by code point, equivalently, by UTF-8 bytes. * The NodeProperties of files, directories, and symlinks must be sorted in lexicographical order by property name. A `Directory` that obeys the restrictions is said to be in canonical form. As an example, the following could be used for a file named `bar` and a directory named `foo` with an executable file named `baz` (hashes shortened for readability): ```json // (Directory proto) { files: [ { name: "bar", digest: { hash: "4a73bc9d03...", size: 65534 }, node_properties: [ { "name": "MTime", "value": "2017-01-15T01:30:15.01Z" } ] } ], directories: [ { name: "foo", digest: { hash: "4cf2eda940...", size: 43 } } ] } // (Directory proto with hash "4cf2eda940..." and size 43) { files: [ { name: "baz", digest: { hash: "b2c941073e...", size: 1294, }, is_executable: true } ] } ```
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2Directory {
    /// The subdirectories in the directory.
    
    pub directories: Option<Vec<BuildBazelRemoteExecutionV2DirectoryNode>>,
    /// The files in the directory.
    
    pub files: Option<Vec<BuildBazelRemoteExecutionV2FileNode>>,
    /// no description provided
    #[serde(rename="nodeProperties")]
    
    pub node_properties: Option<BuildBazelRemoteExecutionV2NodeProperties>,
    /// The symlinks in the directory.
    
    pub symlinks: Option<Vec<BuildBazelRemoteExecutionV2SymlinkNode>>,
}

impl client::Part for BuildBazelRemoteExecutionV2Directory {}


/// A `DirectoryNode` represents a child of a Directory which is itself a `Directory` and its associated metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2DirectoryNode {
    /// The digest of the Directory object represented. See Digest for information about how to take the digest of a proto message.
    
    pub digest: Option<BuildBazelRemoteExecutionV2Digest>,
    /// The name of the directory.
    
    pub name: Option<String>,
}

impl client::Part for BuildBazelRemoteExecutionV2DirectoryNode {}


/// A request message for Execution.Execute.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [execute actions](ActionExecuteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2ExecuteRequest {
    /// The digest of the Action to execute.
    #[serde(rename="actionDigest")]
    
    pub action_digest: Option<BuildBazelRemoteExecutionV2Digest>,
    /// An optional policy for execution of the action. The server will have a default policy if this is not provided.
    #[serde(rename="executionPolicy")]
    
    pub execution_policy: Option<BuildBazelRemoteExecutionV2ExecutionPolicy>,
    /// An optional policy for the results of this execution in the remote cache. The server will have a default policy if this is not provided. This may be applied to both the ActionResult and the associated blobs.
    #[serde(rename="resultsCachePolicy")]
    
    pub results_cache_policy: Option<BuildBazelRemoteExecutionV2ResultsCachePolicy>,
    /// If true, the action will be executed even if its result is already present in the ActionCache. The execution is still allowed to be merged with other in-flight executions of the same action, however - semantically, the service MUST only guarantee that the results of an execution with this field set were not visible before the corresponding execution request was sent. Note that actions from execution requests setting this field set are still eligible to be entered into the action cache upon completion, and services SHOULD overwrite any existing entries that may exist. This allows skip_cache_lookup requests to be used as a mechanism for replacing action cache entries that reference outputs no longer available or that are poisoned in any way. If false, the result may be served from the action cache.
    #[serde(rename="skipCacheLookup")]
    
    pub skip_cache_lookup: Option<bool>,
}

impl client::RequestValue for BuildBazelRemoteExecutionV2ExecuteRequest {}


/// ExecutedActionMetadata contains details about a completed execution.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2ExecutedActionMetadata {
    /// Details that are specific to the kind of worker used. For example, on POSIX-like systems this could contain a message with getrusage(2) statistics.
    #[serde(rename="auxiliaryMetadata")]
    
    pub auxiliary_metadata: Option<Vec<HashMap<String, json::Value>>>,
    /// When the worker completed executing the action command.
    #[serde(rename="executionCompletedTimestamp")]
    
    pub execution_completed_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// When the worker started executing the action command.
    #[serde(rename="executionStartTimestamp")]
    
    pub execution_start_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// When the worker finished fetching action inputs.
    #[serde(rename="inputFetchCompletedTimestamp")]
    
    pub input_fetch_completed_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// When the worker started fetching action inputs.
    #[serde(rename="inputFetchStartTimestamp")]
    
    pub input_fetch_start_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// When the worker finished uploading action outputs.
    #[serde(rename="outputUploadCompletedTimestamp")]
    
    pub output_upload_completed_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// When the worker started uploading action outputs.
    #[serde(rename="outputUploadStartTimestamp")]
    
    pub output_upload_start_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// When was the action added to the queue.
    #[serde(rename="queuedTimestamp")]
    
    pub queued_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The name of the worker which ran the execution.
    
    pub worker: Option<String>,
    /// When the worker completed the action, including all stages.
    #[serde(rename="workerCompletedTimestamp")]
    
    pub worker_completed_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// When the worker received the action.
    #[serde(rename="workerStartTimestamp")]
    
    pub worker_start_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for BuildBazelRemoteExecutionV2ExecutedActionMetadata {}


/// Capabilities of the remote execution system.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2ExecutionCapabilities {
    /// Remote execution may only support a single digest function.
    #[serde(rename="digestFunction")]
    
    pub digest_function: Option<BuildBazelRemoteExecutionV2ExecutionCapabilityDigestFunctionEnum>,
    /// Whether remote execution is enabled for the particular server/instance.
    #[serde(rename="execEnabled")]
    
    pub exec_enabled: Option<bool>,
    /// Supported execution priority range.
    #[serde(rename="executionPriorityCapabilities")]
    
    pub execution_priority_capabilities: Option<BuildBazelRemoteExecutionV2PriorityCapabilities>,
    /// Supported node properties.
    #[serde(rename="supportedNodeProperties")]
    
    pub supported_node_properties: Option<Vec<String>>,
}

impl client::Part for BuildBazelRemoteExecutionV2ExecutionCapabilities {}


/// An `ExecutionPolicy` can be used to control the scheduling of the action.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2ExecutionPolicy {
    /// The priority (relative importance) of this action. Generally, a lower value means that the action should be run sooner than actions having a greater priority value, but the interpretation of a given value is server- dependent. A priority of 0 means the *default* priority. Priorities may be positive or negative, and such actions should run later or sooner than actions having the default priority, respectively. The particular semantics of this field is up to the server. In particular, every server will have their own supported range of priorities, and will decide how these map into scheduling policy.
    
    pub priority: Option<i32>,
}

impl client::Part for BuildBazelRemoteExecutionV2ExecutionPolicy {}


/// A `FileNode` represents a single file and associated metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2FileNode {
    /// The digest of the file's content.
    
    pub digest: Option<BuildBazelRemoteExecutionV2Digest>,
    /// True if file is executable, false otherwise.
    #[serde(rename="isExecutable")]
    
    pub is_executable: Option<bool>,
    /// The name of the file.
    
    pub name: Option<String>,
    /// no description provided
    #[serde(rename="nodeProperties")]
    
    pub node_properties: Option<BuildBazelRemoteExecutionV2NodeProperties>,
}

impl client::Part for BuildBazelRemoteExecutionV2FileNode {}


/// A request message for ContentAddressableStorage.FindMissingBlobs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [find missing blobs](BlobFindMissingCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2FindMissingBlobsRequest {
    /// A list of the blobs to check.
    #[serde(rename="blobDigests")]
    
    pub blob_digests: Option<Vec<BuildBazelRemoteExecutionV2Digest>>,
}

impl client::RequestValue for BuildBazelRemoteExecutionV2FindMissingBlobsRequest {}


/// A response message for ContentAddressableStorage.FindMissingBlobs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [find missing blobs](BlobFindMissingCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2FindMissingBlobsResponse {
    /// A list of the blobs requested *not* present in the storage.
    #[serde(rename="missingBlobDigests")]
    
    pub missing_blob_digests: Option<Vec<BuildBazelRemoteExecutionV2Digest>>,
}

impl client::ResponseResult for BuildBazelRemoteExecutionV2FindMissingBlobsResponse {}


/// A response message for ContentAddressableStorage.GetTree.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get tree blobs](BlobGetTreeCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2GetTreeResponse {
    /// The directories descended from the requested root.
    
    pub directories: Option<Vec<BuildBazelRemoteExecutionV2Directory>>,
    /// If present, signifies that there are more results which the client can retrieve by passing this as the page_token in a subsequent request. If empty, signifies that this is the last page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for BuildBazelRemoteExecutionV2GetTreeResponse {}


/// Node properties for FileNodes, DirectoryNodes, and SymlinkNodes. The server is responsible for specifying the properties that it accepts.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2NodeProperties {
    /// The file's last modification timestamp.
    
    pub mtime: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A list of string-based NodeProperties.
    
    pub properties: Option<Vec<BuildBazelRemoteExecutionV2NodeProperty>>,
    /// The UNIX file mode, e.g., 0755.
    #[serde(rename="unixMode")]
    
    pub unix_mode: Option<u32>,
}

impl client::Part for BuildBazelRemoteExecutionV2NodeProperties {}


/// A single property for FileNodes, DirectoryNodes, and SymlinkNodes. The server is responsible for specifying the property `name`s that it accepts. If permitted by the server, the same `name` may occur multiple times.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2NodeProperty {
    /// The property name.
    
    pub name: Option<String>,
    /// The property value.
    
    pub value: Option<String>,
}

impl client::Part for BuildBazelRemoteExecutionV2NodeProperty {}


/// An `OutputDirectory` is the output in an `ActionResult` corresponding to a directory's full contents rather than a single file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2OutputDirectory {
    /// The full path of the directory relative to the working directory. The path separator is a forward slash `/`. Since this is a relative path, it MUST NOT begin with a leading forward slash. The empty string value is allowed, and it denotes the entire working directory.
    
    pub path: Option<String>,
    /// The digest of the encoded Tree proto containing the directory's contents.
    #[serde(rename="treeDigest")]
    
    pub tree_digest: Option<BuildBazelRemoteExecutionV2Digest>,
}

impl client::Part for BuildBazelRemoteExecutionV2OutputDirectory {}


/// An `OutputFile` is similar to a FileNode, but it is used as an output in an `ActionResult`. It allows a full file path rather than only a name.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2OutputFile {
    /// The contents of the file if inlining was requested. The server SHOULD NOT inline file contents unless requested by the client in the GetActionResultRequest message. The server MAY omit inlining, even if requested, and MUST do so if inlining would cause the response to exceed message size limits.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub contents: Option<Vec<u8>>,
    /// The digest of the file's content.
    
    pub digest: Option<BuildBazelRemoteExecutionV2Digest>,
    /// True if file is executable, false otherwise.
    #[serde(rename="isExecutable")]
    
    pub is_executable: Option<bool>,
    /// no description provided
    #[serde(rename="nodeProperties")]
    
    pub node_properties: Option<BuildBazelRemoteExecutionV2NodeProperties>,
    /// The full path of the file relative to the working directory, including the filename. The path separator is a forward slash `/`. Since this is a relative path, it MUST NOT begin with a leading forward slash.
    
    pub path: Option<String>,
}

impl client::Part for BuildBazelRemoteExecutionV2OutputFile {}


/// An `OutputSymlink` is similar to a Symlink, but it is used as an output in an `ActionResult`. `OutputSymlink` is binary-compatible with `SymlinkNode`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2OutputSymlink {
    /// no description provided
    #[serde(rename="nodeProperties")]
    
    pub node_properties: Option<BuildBazelRemoteExecutionV2NodeProperties>,
    /// The full path of the symlink relative to the working directory, including the filename. The path separator is a forward slash `/`. Since this is a relative path, it MUST NOT begin with a leading forward slash.
    
    pub path: Option<String>,
    /// The target path of the symlink. The path separator is a forward slash `/`. The target path can be relative to the parent directory of the symlink or it can be an absolute path starting with `/`. Support for absolute paths can be checked using the Capabilities API. `..` components are allowed anywhere in the target path.
    
    pub target: Option<String>,
}

impl client::Part for BuildBazelRemoteExecutionV2OutputSymlink {}


/// Allowed values for priority in ResultsCachePolicy and ExecutionPolicy Used for querying both cache and execution valid priority ranges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2PriorityCapabilities {
    /// no description provided
    
    pub priorities: Option<Vec<BuildBazelRemoteExecutionV2PriorityCapabilitiesPriorityRange>>,
}

impl client::Part for BuildBazelRemoteExecutionV2PriorityCapabilities {}


/// Supported range of priorities, including boundaries.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2PriorityCapabilitiesPriorityRange {
    /// The maximum numeric value for this priority range, which represents the least urgent task or shortest retained item.
    #[serde(rename="maxPriority")]
    
    pub max_priority: Option<i32>,
    /// The minimum numeric value for this priority range, which represents the most urgent task or longest retained item.
    #[serde(rename="minPriority")]
    
    pub min_priority: Option<i32>,
}

impl client::Part for BuildBazelRemoteExecutionV2PriorityCapabilitiesPriorityRange {}


/// A `ResultsCachePolicy` is used for fine-grained control over how action outputs are stored in the CAS and Action Cache.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2ResultsCachePolicy {
    /// The priority (relative importance) of this content in the overall cache. Generally, a lower value means a longer retention time or other advantage, but the interpretation of a given value is server-dependent. A priority of 0 means a *default* value, decided by the server. The particular semantics of this field is up to the server. In particular, every server will have their own supported range of priorities, and will decide how these map into retention/eviction policy.
    
    pub priority: Option<i32>,
}

impl client::Part for BuildBazelRemoteExecutionV2ResultsCachePolicy {}


/// A response message for Capabilities.GetCapabilities.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get capabilities](MethodGetCapabilityCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2ServerCapabilities {
    /// Capabilities of the remote cache system.
    #[serde(rename="cacheCapabilities")]
    
    pub cache_capabilities: Option<BuildBazelRemoteExecutionV2CacheCapabilities>,
    /// Earliest RE API version supported, including deprecated versions.
    #[serde(rename="deprecatedApiVersion")]
    
    pub deprecated_api_version: Option<BuildBazelSemverSemVer>,
    /// Capabilities of the remote execution system.
    #[serde(rename="executionCapabilities")]
    
    pub execution_capabilities: Option<BuildBazelRemoteExecutionV2ExecutionCapabilities>,
    /// Latest RE API version supported.
    #[serde(rename="highApiVersion")]
    
    pub high_api_version: Option<BuildBazelSemverSemVer>,
    /// Earliest non-deprecated RE API version supported.
    #[serde(rename="lowApiVersion")]
    
    pub low_api_version: Option<BuildBazelSemverSemVer>,
}

impl client::ResponseResult for BuildBazelRemoteExecutionV2ServerCapabilities {}


/// A `SymlinkNode` represents a symbolic link.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2SymlinkNode {
    /// The name of the symlink.
    
    pub name: Option<String>,
    /// no description provided
    #[serde(rename="nodeProperties")]
    
    pub node_properties: Option<BuildBazelRemoteExecutionV2NodeProperties>,
    /// The target path of the symlink. The path separator is a forward slash `/`. The target path can be relative to the parent directory of the symlink or it can be an absolute path starting with `/`. Support for absolute paths can be checked using the Capabilities API. `..` components are allowed anywhere in the target path as logical canonicalization may lead to different behavior in the presence of directory symlinks (e.g. `foo/../bar` may not be the same as `bar`). To reduce potential cache misses, canonicalization is still recommended where this is possible without impacting correctness.
    
    pub target: Option<String>,
}

impl client::Part for BuildBazelRemoteExecutionV2SymlinkNode {}


/// A request message for WaitExecution.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [wait execution operations](OperationWaitExecutionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2WaitExecutionRequest { _never_set: Option<bool> }

impl client::RequestValue for BuildBazelRemoteExecutionV2WaitExecutionRequest {}


/// The full version of a given tool.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelSemverSemVer {
    /// The major version, e.g 10 for 10.2.3.
    
    pub major: Option<i32>,
    /// The minor version, e.g. 2 for 10.2.3.
    
    pub minor: Option<i32>,
    /// The patch version, e.g 3 for 10.2.3.
    
    pub patch: Option<i32>,
    /// The pre-release version. Either this field or major/minor/patch fields must be filled. They are mutually exclusive. Pre-release versions are assumed to be earlier than any released versions.
    
    pub prerelease: Option<String>,
}

impl client::Part for BuildBazelSemverSemVer {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [execute actions](ActionExecuteCall) (response)
/// * [wait execution operations](OperationWaitExecutionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningOperation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<GoogleRpcStatus>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for GoogleLongrunningOperation {}


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


