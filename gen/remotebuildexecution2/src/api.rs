#![allow(clippy::ptr_arg)]

use std::collections::{BTreeSet, HashMap};

use tokio::time::sleep;

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Clone, Copy)]
pub enum Scope {
    /// See, edit, configure, and delete your Google Cloud Platform data
    CloudPlatform,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for Scope {
    fn default() -> Scope {
        Scope::CloudPlatform
    }
}

// ########
// HUB ###
// ######

/// Central instance to access all RemoteBuildExecution related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_remotebuildexecution2 as remotebuildexecution2;
/// use remotebuildexecution2::{Result, Error};
/// # async fn dox() {
/// use remotebuildexecution2::{RemoteBuildExecution, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// let mut hub = RemoteBuildExecution::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.action_results().get("instanceName", "hash", -4)
///              .inline_stdout(true)
///              .inline_stderr(false)
///              .add_inline_output_files("amet")
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
pub struct RemoteBuildExecution<C> {
    pub client: common::Client<C>,
    pub auth: Box<dyn common::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<C> common::Hub for RemoteBuildExecution<C> {}

impl<'a, C> RemoteBuildExecution<C> {
    pub fn new<A: 'static + common::GetToken>(
        client: common::Client<C>,
        auth: A,
    ) -> RemoteBuildExecution<C> {
        RemoteBuildExecution {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/6.0.0".to_string(),
            _base_url: "https://remotebuildexecution.googleapis.com/".to_string(),
            _root_url: "https://remotebuildexecution.googleapis.com/".to_string(),
        }
    }

    pub fn action_results(&'a self) -> ActionResultMethods<'a, C> {
        ActionResultMethods { hub: self }
    }
    pub fn actions(&'a self) -> ActionMethods<'a, C> {
        ActionMethods { hub: self }
    }
    pub fn blobs(&'a self) -> BlobMethods<'a, C> {
        BlobMethods { hub: self }
    }
    pub fn methods(&'a self) -> MethodMethods<'a, C> {
        MethodMethods { hub: self }
    }
    pub fn operations(&'a self) -> OperationMethods<'a, C> {
        OperationMethods { hub: self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/6.0.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        std::mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://remotebuildexecution.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        std::mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://remotebuildexecution.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        std::mem::replace(&mut self._root_url, new_root_url)
    }
}

// ############
// SCHEMAS ###
// ##########
/// Describes the server/instance capabilities for updating the action cache.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2ActionCacheUpdateCapabilities {
    /// no description provided
    #[serde(rename = "updateEnabled")]
    pub update_enabled: Option<bool>,
}

impl common::Part for BuildBazelRemoteExecutionV2ActionCacheUpdateCapabilities {}

/// An ActionResult represents the result of an Action being run. It is advised that at least one field (for example `ActionResult.execution_metadata.Worker`) have a non-default value, to ensure that the serialized value is non-empty, which can then be used as a basic data sanity check.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [get action results](ActionResultGetCall) (response)
/// * [update action results](ActionResultUpdateCall) (request|response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2ActionResult {
    /// The details of the execution that originally produced this result.
    #[serde(rename = "executionMetadata")]
    pub execution_metadata: Option<BuildBazelRemoteExecutionV2ExecutedActionMetadata>,
    /// The exit code of the command.
    #[serde(rename = "exitCode")]
    pub exit_code: Option<i32>,
    /// The output directories of the action. For each output directory requested in the `output_directories` or `output_paths` field of the Action, if the corresponding directory existed after the action completed, a single entry will be present in the output list, which will contain the digest of a Tree message containing the directory tree, and the path equal exactly to the corresponding Action output_directories member. As an example, suppose the Action had an output directory `a/b/dir` and the execution produced the following contents in `a/b/dir`: a file named `bar` and a directory named `foo` with an executable file named `baz`. Then, output_directory will contain (hashes shortened for readability): ```json // OutputDirectory proto: { path: "a/b/dir" tree_digest: { hash: "4a73bc9d03...", size: 55 } } // Tree proto with hash "4a73bc9d03..." and size 55: { root: { files: [ { name: "bar", digest: { hash: "4a73bc9d03...", size: 65534 } } ], directories: [ { name: "foo", digest: { hash: "4cf2eda940...", size: 43 } } ] } children : { // (Directory proto with hash "4cf2eda940..." and size 43) files: [ { name: "baz", digest: { hash: "b2c941073e...", size: 1294, }, is_executable: true } ] } } ``` If an output of the same name as listed in `output_files` of the Command was found in `output_directories`, but was not a directory, the server will return a FAILED_PRECONDITION.
    #[serde(rename = "outputDirectories")]
    pub output_directories: Option<Vec<BuildBazelRemoteExecutionV2OutputDirectory>>,
    /// The output directories of the action that are symbolic links to other directories. Those may be links to other output directories, or input directories, or even absolute paths outside of the working directory, if the server supports SymlinkAbsolutePathStrategy.ALLOWED. For each output directory requested in the `output_directories` field of the Action, if the directory existed after the action completed, a single entry will be present either in this field, or in the `output_directories` field, if the directory was not a symbolic link. If an output of the same name was found, but was a symbolic link to a file instead of a directory, the server will return a FAILED_PRECONDITION. If the action does not produce the requested output, then that output will be omitted from the list. The server is free to arrange the output list as desired; clients MUST NOT assume that the output list is sorted. DEPRECATED as of v2.1. Servers that wish to be compatible with v2.0 API should still populate this field in addition to `output_symlinks`.
    #[serde(rename = "outputDirectorySymlinks")]
    pub output_directory_symlinks: Option<Vec<BuildBazelRemoteExecutionV2OutputSymlink>>,
    /// The output files of the action that are symbolic links to other files. Those may be links to other output files, or input files, or even absolute paths outside of the working directory, if the server supports SymlinkAbsolutePathStrategy.ALLOWED. For each output file requested in the `output_files` or `output_paths` field of the Action, if the corresponding file existed after the action completed, a single entry will be present either in this field, or in the `output_files` field, if the file was not a symbolic link. If an output symbolic link of the same name as listed in `output_files` of the Command was found, but its target type was not a regular file, the server will return a FAILED_PRECONDITION. If the action does not produce the requested output, then that output will be omitted from the list. The server is free to arrange the output list as desired; clients MUST NOT assume that the output list is sorted. DEPRECATED as of v2.1. Servers that wish to be compatible with v2.0 API should still populate this field in addition to `output_symlinks`.
    #[serde(rename = "outputFileSymlinks")]
    pub output_file_symlinks: Option<Vec<BuildBazelRemoteExecutionV2OutputSymlink>>,
    /// The output files of the action. For each output file requested in the `output_files` or `output_paths` field of the Action, if the corresponding file existed after the action completed, a single entry will be present either in this field, or the `output_file_symlinks` field if the file was a symbolic link to another file (`output_symlinks` field after v2.1). If an output listed in `output_files` was found, but was a directory rather than a regular file, the server will return a FAILED_PRECONDITION. If the action does not produce the requested output, then that output will be omitted from the list. The server is free to arrange the output list as desired; clients MUST NOT assume that the output list is sorted.
    #[serde(rename = "outputFiles")]
    pub output_files: Option<Vec<BuildBazelRemoteExecutionV2OutputFile>>,
    /// New in v2.1: this field will only be populated if the command `output_paths` field was used, and not the pre v2.1 `output_files` or `output_directories` fields. The output paths of the action that are symbolic links to other paths. Those may be links to other outputs, or inputs, or even absolute paths outside of the working directory, if the server supports SymlinkAbsolutePathStrategy.ALLOWED. A single entry for each output requested in `output_paths` field of the Action, if the corresponding path existed after the action completed and was a symbolic link. If the action does not produce a requested output, then that output will be omitted from the list. The server is free to arrange the output list as desired; clients MUST NOT assume that the output list is sorted.
    #[serde(rename = "outputSymlinks")]
    pub output_symlinks: Option<Vec<BuildBazelRemoteExecutionV2OutputSymlink>>,
    /// The digest for a blob containing the standard error of the action, which can be retrieved from the ContentAddressableStorage.
    #[serde(rename = "stderrDigest")]
    pub stderr_digest: Option<BuildBazelRemoteExecutionV2Digest>,
    /// The standard error buffer of the action. The server SHOULD NOT inline stderr unless requested by the client in the GetActionResultRequest message. The server MAY omit inlining, even if requested, and MUST do so if inlining would cause the response to exceed message size limits.
    #[serde(rename = "stderrRaw")]
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub stderr_raw: Option<Vec<u8>>,
    /// The digest for a blob containing the standard output of the action, which can be retrieved from the ContentAddressableStorage.
    #[serde(rename = "stdoutDigest")]
    pub stdout_digest: Option<BuildBazelRemoteExecutionV2Digest>,
    /// The standard output buffer of the action. The server SHOULD NOT inline stdout unless requested by the client in the GetActionResultRequest message. The server MAY omit inlining, even if requested, and MUST do so if inlining would cause the response to exceed message size limits.
    #[serde(rename = "stdoutRaw")]
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub stdout_raw: Option<Vec<u8>>,
}

impl common::RequestValue for BuildBazelRemoteExecutionV2ActionResult {}
impl common::ResponseResult for BuildBazelRemoteExecutionV2ActionResult {}

/// A request message for ContentAddressableStorage.BatchReadBlobs.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [batch read blobs](BlobBatchReadCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2BatchReadBlobsRequest {
    /// The individual blob digests.
    pub digests: Option<Vec<BuildBazelRemoteExecutionV2Digest>>,
}

impl common::RequestValue for BuildBazelRemoteExecutionV2BatchReadBlobsRequest {}

/// A response message for ContentAddressableStorage.BatchReadBlobs.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [batch read blobs](BlobBatchReadCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2BatchReadBlobsResponse {
    /// The responses to the requests.
    pub responses: Option<Vec<BuildBazelRemoteExecutionV2BatchReadBlobsResponseResponse>>,
}

impl common::ResponseResult for BuildBazelRemoteExecutionV2BatchReadBlobsResponse {}

/// A response corresponding to a single blob that the client tried to download.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2BatchReadBlobsResponseResponse {
    /// The raw binary data.
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// The digest to which this response corresponds.
    pub digest: Option<BuildBazelRemoteExecutionV2Digest>,
    /// The result of attempting to download that blob.
    pub status: Option<GoogleRpcStatus>,
}

impl common::Part for BuildBazelRemoteExecutionV2BatchReadBlobsResponseResponse {}

/// A request message for ContentAddressableStorage.BatchUpdateBlobs.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [batch update blobs](BlobBatchUpdateCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2BatchUpdateBlobsRequest {
    /// The individual upload requests.
    pub requests: Option<Vec<BuildBazelRemoteExecutionV2BatchUpdateBlobsRequestRequest>>,
}

impl common::RequestValue for BuildBazelRemoteExecutionV2BatchUpdateBlobsRequest {}

/// A request corresponding to a single blob that the client wants to upload.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2BatchUpdateBlobsRequestRequest {
    /// The raw binary data.
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// The digest of the blob. This MUST be the digest of `data`.
    pub digest: Option<BuildBazelRemoteExecutionV2Digest>,
}

impl common::Part for BuildBazelRemoteExecutionV2BatchUpdateBlobsRequestRequest {}

/// A response message for ContentAddressableStorage.BatchUpdateBlobs.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [batch update blobs](BlobBatchUpdateCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2BatchUpdateBlobsResponse {
    /// The responses to the requests.
    pub responses: Option<Vec<BuildBazelRemoteExecutionV2BatchUpdateBlobsResponseResponse>>,
}

impl common::ResponseResult for BuildBazelRemoteExecutionV2BatchUpdateBlobsResponse {}

/// A response corresponding to a single blob that the client tried to upload.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2BatchUpdateBlobsResponseResponse {
    /// The blob digest to which this response corresponds.
    pub digest: Option<BuildBazelRemoteExecutionV2Digest>,
    /// The result of attempting to upload that blob.
    pub status: Option<GoogleRpcStatus>,
}

impl common::Part for BuildBazelRemoteExecutionV2BatchUpdateBlobsResponseResponse {}

/// Capabilities of the remote cache system.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2CacheCapabilities {
    /// Capabilities for updating the action cache.
    #[serde(rename = "actionCacheUpdateCapabilities")]
    pub action_cache_update_capabilities:
        Option<BuildBazelRemoteExecutionV2ActionCacheUpdateCapabilities>,
    /// Supported cache priority range for both CAS and ActionCache.
    #[serde(rename = "cachePriorityCapabilities")]
    pub cache_priority_capabilities: Option<BuildBazelRemoteExecutionV2PriorityCapabilities>,
    /// All the digest functions supported by the remote cache. Remote cache may support multiple digest functions simultaneously.
    #[serde(rename = "digestFunction")]
    pub digest_function: Option<Vec<String>>,
    /// Maximum total size of blobs to be uploaded/downloaded using batch methods. A value of 0 means no limit is set, although in practice there will always be a message size limitation of the protocol in use, e.g. GRPC.
    #[serde(rename = "maxBatchTotalSizeBytes")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub max_batch_total_size_bytes: Option<i64>,
    /// Compressors supported by the "compressed-blobs" bytestream resources. Servers MUST support identity/no-compression, even if it is not listed here. Note that this does not imply which if any compressors are supported by the server at the gRPC level.
    #[serde(rename = "supportedCompressor")]
    pub supported_compressor: Option<Vec<String>>,
    /// Whether absolute symlink targets are supported.
    #[serde(rename = "symlinkAbsolutePathStrategy")]
    pub symlink_absolute_path_strategy: Option<String>,
}

impl common::Part for BuildBazelRemoteExecutionV2CacheCapabilities {}

/// A content digest. A digest for a given blob consists of the size of the blob and its hash. The hash algorithm to use is defined by the server. The size is considered to be an integral part of the digest and cannot be separated. That is, even if the `hash` field is correctly specified but `size_bytes` is not, the server MUST reject the request. The reason for including the size in the digest is as follows: in a great many cases, the server needs to know the size of the blob it is about to work with prior to starting an operation with it, such as flattening Merkle tree structures or streaming it to a worker. Technically, the server could implement a separate metadata store, but this results in a significantly more complicated implementation as opposed to having the client specify the size up-front (or storing the size along with the digest in every message where digests are embedded). This does mean that the API leaks some implementation details of (what we consider to be) a reasonable server implementation, but we consider this to be a worthwhile tradeoff. When a `Digest` is used to refer to a proto message, it always refers to the message in binary encoded form. To ensure consistent hashing, clients and servers MUST ensure that they serialize messages according to the following rules, even if there are alternate valid encodings for the same message: * Fields are serialized in tag order. * There are no unknown fields. * There are no duplicate fields. * Fields are serialized according to the default semantics for their type. Most protocol buffer implementations will always follow these rules when serializing, but care should be taken to avoid shortcuts. For instance, concatenating two messages to merge them may produce duplicate fields.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2Digest {
    /// The hash. In the case of SHA-256, it will always be a lowercase hex string exactly 64 characters long.
    pub hash: Option<String>,
    /// The size of the blob, in bytes.
    #[serde(rename = "sizeBytes")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub size_bytes: Option<i64>,
}

impl common::Part for BuildBazelRemoteExecutionV2Digest {}

/// A `Directory` represents a directory node in a file tree, containing zero or more children FileNodes, DirectoryNodes and SymlinkNodes. Each `Node` contains its name in the directory, either the digest of its content (either a file blob or a `Directory` proto) or a symlink target, as well as possibly some metadata about the file or directory. In order to ensure that two equivalent directory trees hash to the same value, the following restrictions MUST be obeyed when constructing a a `Directory`: * Every child in the directory must have a path of exactly one segment. Multiple levels of directory hierarchy may not be collapsed. * Each child in the directory must have a unique path segment (file name). Note that while the API itself is case-sensitive, the environment where the Action is executed may or may not be case-sensitive. That is, it is legal to call the API with a Directory that has both "Foo" and "foo" as children, but the Action may be rejected by the remote system upon execution. * The files, directories and symlinks in the directory must each be sorted in lexicographical order by path. The path strings must be sorted by code point, equivalently, by UTF-8 bytes. * The NodeProperties of files, directories, and symlinks must be sorted in lexicographical order by property name. A `Directory` that obeys the restrictions is said to be in canonical form. As an example, the following could be used for a file named `bar` and a directory named `foo` with an executable file named `baz` (hashes shortened for readability): ```json // (Directory proto) { files: [ { name: "bar", digest: { hash: "4a73bc9d03...", size: 65534 }, node_properties: [ { "name": "MTime", "value": "2017-01-15T01:30:15.01Z" } ] } ], directories: [ { name: "foo", digest: { hash: "4cf2eda940...", size: 43 } } ] } // (Directory proto with hash "4cf2eda940..." and size 43) { files: [ { name: "baz", digest: { hash: "b2c941073e...", size: 1294, }, is_executable: true } ] } ```
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2Directory {
    /// The subdirectories in the directory.
    pub directories: Option<Vec<BuildBazelRemoteExecutionV2DirectoryNode>>,
    /// The files in the directory.
    pub files: Option<Vec<BuildBazelRemoteExecutionV2FileNode>>,
    /// no description provided
    #[serde(rename = "nodeProperties")]
    pub node_properties: Option<BuildBazelRemoteExecutionV2NodeProperties>,
    /// The symlinks in the directory.
    pub symlinks: Option<Vec<BuildBazelRemoteExecutionV2SymlinkNode>>,
}

impl common::Part for BuildBazelRemoteExecutionV2Directory {}

/// A `DirectoryNode` represents a child of a Directory which is itself a `Directory` and its associated metadata.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2DirectoryNode {
    /// The digest of the Directory object represented. See Digest for information about how to take the digest of a proto message.
    pub digest: Option<BuildBazelRemoteExecutionV2Digest>,
    /// The name of the directory.
    pub name: Option<String>,
}

impl common::Part for BuildBazelRemoteExecutionV2DirectoryNode {}

/// A request message for Execution.Execute.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [execute actions](ActionExecuteCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2ExecuteRequest {
    /// The digest of the Action to execute.
    #[serde(rename = "actionDigest")]
    pub action_digest: Option<BuildBazelRemoteExecutionV2Digest>,
    /// An optional policy for execution of the action. The server will have a default policy if this is not provided.
    #[serde(rename = "executionPolicy")]
    pub execution_policy: Option<BuildBazelRemoteExecutionV2ExecutionPolicy>,
    /// An optional policy for the results of this execution in the remote cache. The server will have a default policy if this is not provided. This may be applied to both the ActionResult and the associated blobs.
    #[serde(rename = "resultsCachePolicy")]
    pub results_cache_policy: Option<BuildBazelRemoteExecutionV2ResultsCachePolicy>,
    /// If true, the action will be executed even if its result is already present in the ActionCache. The execution is still allowed to be merged with other in-flight executions of the same action, however - semantically, the service MUST only guarantee that the results of an execution with this field set were not visible before the corresponding execution request was sent. Note that actions from execution requests setting this field set are still eligible to be entered into the action cache upon completion, and services SHOULD overwrite any existing entries that may exist. This allows skip_cache_lookup requests to be used as a mechanism for replacing action cache entries that reference outputs no longer available or that are poisoned in any way. If false, the result may be served from the action cache.
    #[serde(rename = "skipCacheLookup")]
    pub skip_cache_lookup: Option<bool>,
}

impl common::RequestValue for BuildBazelRemoteExecutionV2ExecuteRequest {}

/// ExecutedActionMetadata contains details about a completed execution.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2ExecutedActionMetadata {
    /// Details that are specific to the kind of worker used. For example, on POSIX-like systems this could contain a message with getrusage(2) statistics.
    #[serde(rename = "auxiliaryMetadata")]
    pub auxiliary_metadata: Option<Vec<HashMap<String, serde_json::Value>>>,
    /// When the worker completed executing the action command.
    #[serde(rename = "executionCompletedTimestamp")]
    pub execution_completed_timestamp: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// When the worker started executing the action command.
    #[serde(rename = "executionStartTimestamp")]
    pub execution_start_timestamp: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// When the worker finished fetching action inputs.
    #[serde(rename = "inputFetchCompletedTimestamp")]
    pub input_fetch_completed_timestamp: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// When the worker started fetching action inputs.
    #[serde(rename = "inputFetchStartTimestamp")]
    pub input_fetch_start_timestamp: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// When the worker finished uploading action outputs.
    #[serde(rename = "outputUploadCompletedTimestamp")]
    pub output_upload_completed_timestamp: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// When the worker started uploading action outputs.
    #[serde(rename = "outputUploadStartTimestamp")]
    pub output_upload_start_timestamp: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// When was the action added to the queue.
    #[serde(rename = "queuedTimestamp")]
    pub queued_timestamp: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// The name of the worker which ran the execution.
    pub worker: Option<String>,
    /// When the worker completed the action, including all stages.
    #[serde(rename = "workerCompletedTimestamp")]
    pub worker_completed_timestamp: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// When the worker received the action.
    #[serde(rename = "workerStartTimestamp")]
    pub worker_start_timestamp: Option<chrono::DateTime<chrono::offset::Utc>>,
}

impl common::Part for BuildBazelRemoteExecutionV2ExecutedActionMetadata {}

/// Capabilities of the remote execution system.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2ExecutionCapabilities {
    /// Remote execution may only support a single digest function.
    #[serde(rename = "digestFunction")]
    pub digest_function: Option<String>,
    /// Whether remote execution is enabled for the particular server/instance.
    #[serde(rename = "execEnabled")]
    pub exec_enabled: Option<bool>,
    /// Supported execution priority range.
    #[serde(rename = "executionPriorityCapabilities")]
    pub execution_priority_capabilities: Option<BuildBazelRemoteExecutionV2PriorityCapabilities>,
    /// Supported node properties.
    #[serde(rename = "supportedNodeProperties")]
    pub supported_node_properties: Option<Vec<String>>,
}

impl common::Part for BuildBazelRemoteExecutionV2ExecutionCapabilities {}

/// An `ExecutionPolicy` can be used to control the scheduling of the action.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2ExecutionPolicy {
    /// The priority (relative importance) of this action. Generally, a lower value means that the action should be run sooner than actions having a greater priority value, but the interpretation of a given value is server- dependent. A priority of 0 means the *default* priority. Priorities may be positive or negative, and such actions should run later or sooner than actions having the default priority, respectively. The particular semantics of this field is up to the server. In particular, every server will have their own supported range of priorities, and will decide how these map into scheduling policy.
    pub priority: Option<i32>,
}

impl common::Part for BuildBazelRemoteExecutionV2ExecutionPolicy {}

/// A `FileNode` represents a single file and associated metadata.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2FileNode {
    /// The digest of the file's content.
    pub digest: Option<BuildBazelRemoteExecutionV2Digest>,
    /// True if file is executable, false otherwise.
    #[serde(rename = "isExecutable")]
    pub is_executable: Option<bool>,
    /// The name of the file.
    pub name: Option<String>,
    /// no description provided
    #[serde(rename = "nodeProperties")]
    pub node_properties: Option<BuildBazelRemoteExecutionV2NodeProperties>,
}

impl common::Part for BuildBazelRemoteExecutionV2FileNode {}

/// A request message for ContentAddressableStorage.FindMissingBlobs.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [find missing blobs](BlobFindMissingCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2FindMissingBlobsRequest {
    /// A list of the blobs to check.
    #[serde(rename = "blobDigests")]
    pub blob_digests: Option<Vec<BuildBazelRemoteExecutionV2Digest>>,
}

impl common::RequestValue for BuildBazelRemoteExecutionV2FindMissingBlobsRequest {}

/// A response message for ContentAddressableStorage.FindMissingBlobs.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [find missing blobs](BlobFindMissingCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2FindMissingBlobsResponse {
    /// A list of the blobs requested *not* present in the storage.
    #[serde(rename = "missingBlobDigests")]
    pub missing_blob_digests: Option<Vec<BuildBazelRemoteExecutionV2Digest>>,
}

impl common::ResponseResult for BuildBazelRemoteExecutionV2FindMissingBlobsResponse {}

/// A response message for ContentAddressableStorage.GetTree.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [get tree blobs](BlobGetTreeCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2GetTreeResponse {
    /// The directories descended from the requested root.
    pub directories: Option<Vec<BuildBazelRemoteExecutionV2Directory>>,
    /// If present, signifies that there are more results which the client can retrieve by passing this as the page_token in a subsequent request. If empty, signifies that this is the last page of results.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}

impl common::ResponseResult for BuildBazelRemoteExecutionV2GetTreeResponse {}

/// Node properties for FileNodes, DirectoryNodes, and SymlinkNodes. The server is responsible for specifying the properties that it accepts.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2NodeProperties {
    /// The file's last modification timestamp.
    pub mtime: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// A list of string-based NodeProperties.
    pub properties: Option<Vec<BuildBazelRemoteExecutionV2NodeProperty>>,
    /// The UNIX file mode, e.g., 0755.
    #[serde(rename = "unixMode")]
    pub unix_mode: Option<u32>,
}

impl common::Part for BuildBazelRemoteExecutionV2NodeProperties {}

/// A single property for FileNodes, DirectoryNodes, and SymlinkNodes. The server is responsible for specifying the property `name`s that it accepts. If permitted by the server, the same `name` may occur multiple times.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2NodeProperty {
    /// The property name.
    pub name: Option<String>,
    /// The property value.
    pub value: Option<String>,
}

impl common::Part for BuildBazelRemoteExecutionV2NodeProperty {}

/// An `OutputDirectory` is the output in an `ActionResult` corresponding to a directory's full contents rather than a single file.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2OutputDirectory {
    /// The full path of the directory relative to the working directory. The path separator is a forward slash `/`. Since this is a relative path, it MUST NOT begin with a leading forward slash. The empty string value is allowed, and it denotes the entire working directory.
    pub path: Option<String>,
    /// The digest of the encoded Tree proto containing the directory's contents.
    #[serde(rename = "treeDigest")]
    pub tree_digest: Option<BuildBazelRemoteExecutionV2Digest>,
}

impl common::Part for BuildBazelRemoteExecutionV2OutputDirectory {}

/// An `OutputFile` is similar to a FileNode, but it is used as an output in an `ActionResult`. It allows a full file path rather than only a name.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2OutputFile {
    /// The contents of the file if inlining was requested. The server SHOULD NOT inline file contents unless requested by the client in the GetActionResultRequest message. The server MAY omit inlining, even if requested, and MUST do so if inlining would cause the response to exceed message size limits.
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub contents: Option<Vec<u8>>,
    /// The digest of the file's content.
    pub digest: Option<BuildBazelRemoteExecutionV2Digest>,
    /// True if file is executable, false otherwise.
    #[serde(rename = "isExecutable")]
    pub is_executable: Option<bool>,
    /// no description provided
    #[serde(rename = "nodeProperties")]
    pub node_properties: Option<BuildBazelRemoteExecutionV2NodeProperties>,
    /// The full path of the file relative to the working directory, including the filename. The path separator is a forward slash `/`. Since this is a relative path, it MUST NOT begin with a leading forward slash.
    pub path: Option<String>,
}

impl common::Part for BuildBazelRemoteExecutionV2OutputFile {}

/// An `OutputSymlink` is similar to a Symlink, but it is used as an output in an `ActionResult`. `OutputSymlink` is binary-compatible with `SymlinkNode`.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2OutputSymlink {
    /// no description provided
    #[serde(rename = "nodeProperties")]
    pub node_properties: Option<BuildBazelRemoteExecutionV2NodeProperties>,
    /// The full path of the symlink relative to the working directory, including the filename. The path separator is a forward slash `/`. Since this is a relative path, it MUST NOT begin with a leading forward slash.
    pub path: Option<String>,
    /// The target path of the symlink. The path separator is a forward slash `/`. The target path can be relative to the parent directory of the symlink or it can be an absolute path starting with `/`. Support for absolute paths can be checked using the Capabilities API. `..` components are allowed anywhere in the target path.
    pub target: Option<String>,
}

impl common::Part for BuildBazelRemoteExecutionV2OutputSymlink {}

/// Allowed values for priority in ResultsCachePolicy and ExecutionPolicy Used for querying both cache and execution valid priority ranges.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2PriorityCapabilities {
    /// no description provided
    pub priorities: Option<Vec<BuildBazelRemoteExecutionV2PriorityCapabilitiesPriorityRange>>,
}

impl common::Part for BuildBazelRemoteExecutionV2PriorityCapabilities {}

/// Supported range of priorities, including boundaries.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2PriorityCapabilitiesPriorityRange {
    /// The maximum numeric value for this priority range, which represents the least urgent task or shortest retained item.
    #[serde(rename = "maxPriority")]
    pub max_priority: Option<i32>,
    /// The minimum numeric value for this priority range, which represents the most urgent task or longest retained item.
    #[serde(rename = "minPriority")]
    pub min_priority: Option<i32>,
}

impl common::Part for BuildBazelRemoteExecutionV2PriorityCapabilitiesPriorityRange {}

/// A `ResultsCachePolicy` is used for fine-grained control over how action outputs are stored in the CAS and Action Cache.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2ResultsCachePolicy {
    /// The priority (relative importance) of this content in the overall cache. Generally, a lower value means a longer retention time or other advantage, but the interpretation of a given value is server-dependent. A priority of 0 means a *default* value, decided by the server. The particular semantics of this field is up to the server. In particular, every server will have their own supported range of priorities, and will decide how these map into retention/eviction policy.
    pub priority: Option<i32>,
}

impl common::Part for BuildBazelRemoteExecutionV2ResultsCachePolicy {}

/// A response message for Capabilities.GetCapabilities.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [get capabilities](MethodGetCapabilityCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2ServerCapabilities {
    /// Capabilities of the remote cache system.
    #[serde(rename = "cacheCapabilities")]
    pub cache_capabilities: Option<BuildBazelRemoteExecutionV2CacheCapabilities>,
    /// Earliest RE API version supported, including deprecated versions.
    #[serde(rename = "deprecatedApiVersion")]
    pub deprecated_api_version: Option<BuildBazelSemverSemVer>,
    /// Capabilities of the remote execution system.
    #[serde(rename = "executionCapabilities")]
    pub execution_capabilities: Option<BuildBazelRemoteExecutionV2ExecutionCapabilities>,
    /// Latest RE API version supported.
    #[serde(rename = "highApiVersion")]
    pub high_api_version: Option<BuildBazelSemverSemVer>,
    /// Earliest non-deprecated RE API version supported.
    #[serde(rename = "lowApiVersion")]
    pub low_api_version: Option<BuildBazelSemverSemVer>,
}

impl common::ResponseResult for BuildBazelRemoteExecutionV2ServerCapabilities {}

/// A `SymlinkNode` represents a symbolic link.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2SymlinkNode {
    /// The name of the symlink.
    pub name: Option<String>,
    /// no description provided
    #[serde(rename = "nodeProperties")]
    pub node_properties: Option<BuildBazelRemoteExecutionV2NodeProperties>,
    /// The target path of the symlink. The path separator is a forward slash `/`. The target path can be relative to the parent directory of the symlink or it can be an absolute path starting with `/`. Support for absolute paths can be checked using the Capabilities API. `..` components are allowed anywhere in the target path as logical canonicalization may lead to different behavior in the presence of directory symlinks (e.g. `foo/../bar` may not be the same as `bar`). To reduce potential cache misses, canonicalization is still recommended where this is possible without impacting correctness.
    pub target: Option<String>,
}

impl common::Part for BuildBazelRemoteExecutionV2SymlinkNode {}

/// A request message for WaitExecution.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [wait execution operations](OperationWaitExecutionCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BuildBazelRemoteExecutionV2WaitExecutionRequest {
    _never_set: Option<bool>,
}

impl common::RequestValue for BuildBazelRemoteExecutionV2WaitExecutionRequest {}

/// The full version of a given tool.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
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

impl common::Part for BuildBazelSemverSemVer {}

/// This resource represents a long-running operation that is the result of a network API call.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [execute actions](ActionExecuteCall) (response)
/// * [wait execution operations](OperationWaitExecutionCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleLongrunningOperation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    pub error: Option<GoogleRpcStatus>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    pub response: Option<HashMap<String, serde_json::Value>>,
}

impl common::ResponseResult for GoogleLongrunningOperation {}

/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleRpcStatus {
    /// The status code, which should be an enum value of google.rpc.Code.
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    pub details: Option<Vec<HashMap<String, serde_json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    pub message: Option<String>,
}

impl common::Part for GoogleRpcStatus {}

// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *actionResult* resources.
/// It is not used directly, but through the [`RemoteBuildExecution`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_remotebuildexecution2 as remotebuildexecution2;
///
/// # async fn dox() {
/// use remotebuildexecution2::{RemoteBuildExecution, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// let mut hub = RemoteBuildExecution::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.action_results();
/// # }
/// ```
pub struct ActionResultMethods<'a, C>
where
    C: 'a,
{
    hub: &'a RemoteBuildExecution<C>,
}

impl<'a, C> common::MethodsBuilder for ActionResultMethods<'a, C> {}

impl<'a, C> ActionResultMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieve a cached execution result. Implementations SHOULD ensure that any blobs referenced from the ContentAddressableStorage are available at the time of returning the ActionResult and will be for some period of time afterwards. The lifetimes of the referenced blobs SHOULD be increased if necessary and applicable. Errors: * `NOT_FOUND`: The requested `ActionResult` is not in the cache.
    ///
    /// # Arguments
    ///
    /// * `instanceName` - The instance of the execution system to operate against. A server may support multiple instances of the execution system (with their own workers, storage, caches, etc.). The server MAY require use of this field to select between them in an implementation-defined fashion, otherwise it can be omitted.
    /// * `hash` - The hash. In the case of SHA-256, it will always be a lowercase hex string exactly 64 characters long.
    /// * `sizeBytes` - The size of the blob, in bytes.
    pub fn get(
        &self,
        instance_name: &str,
        hash: &str,
        size_bytes: i64,
    ) -> ActionResultGetCall<'a, C> {
        ActionResultGetCall {
            hub: self.hub,
            _instance_name: instance_name.to_string(),
            _hash: hash.to_string(),
            _size_bytes: size_bytes,
            _inline_stdout: Default::default(),
            _inline_stderr: Default::default(),
            _inline_output_files: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Upload a new execution result. In order to allow the server to perform access control based on the type of action, and to assist with client debugging, the client MUST first upload the Action that produced the result, along with its Command, into the `ContentAddressableStorage`. Server implementations MAY modify the `UpdateActionResultRequest.action_result` and return an equivalent value. Errors: * `INVALID_ARGUMENT`: One or more arguments are invalid. * `FAILED_PRECONDITION`: One or more errors occurred in updating the action result, such as a missing command or action. * `RESOURCE_EXHAUSTED`: There is insufficient storage space to add the entry to the cache.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `instanceName` - The instance of the execution system to operate against. A server may support multiple instances of the execution system (with their own workers, storage, caches, etc.). The server MAY require use of this field to select between them in an implementation-defined fashion, otherwise it can be omitted.
    /// * `hash` - The hash. In the case of SHA-256, it will always be a lowercase hex string exactly 64 characters long.
    /// * `sizeBytes` - The size of the blob, in bytes.
    pub fn update(
        &self,
        request: BuildBazelRemoteExecutionV2ActionResult,
        instance_name: &str,
        hash: &str,
        size_bytes: i64,
    ) -> ActionResultUpdateCall<'a, C> {
        ActionResultUpdateCall {
            hub: self.hub,
            _request: request,
            _instance_name: instance_name.to_string(),
            _hash: hash.to_string(),
            _size_bytes: size_bytes,
            _results_cache_policy_priority: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *action* resources.
/// It is not used directly, but through the [`RemoteBuildExecution`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_remotebuildexecution2 as remotebuildexecution2;
///
/// # async fn dox() {
/// use remotebuildexecution2::{RemoteBuildExecution, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// let mut hub = RemoteBuildExecution::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `execute(...)`
/// // to build up your call.
/// let rb = hub.actions();
/// # }
/// ```
pub struct ActionMethods<'a, C>
where
    C: 'a,
{
    hub: &'a RemoteBuildExecution<C>,
}

impl<'a, C> common::MethodsBuilder for ActionMethods<'a, C> {}

impl<'a, C> ActionMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Execute an action remotely. In order to execute an action, the client must first upload all of the inputs, the Command to run, and the Action into the ContentAddressableStorage. It then calls `Execute` with an `action_digest` referring to them. The server will run the action and eventually return the result. The input `Action`'s fields MUST meet the various canonicalization requirements specified in the documentation for their types so that it has the same digest as other logically equivalent `Action`s. The server MAY enforce the requirements and return errors if a non-canonical input is received. It MAY also proceed without verifying some or all of the requirements, such as for performance reasons. If the server does not verify the requirement, then it will treat the `Action` as distinct from another logically equivalent action if they hash differently. Returns a stream of google.longrunning.Operation messages describing the resulting execution, with eventual `response` ExecuteResponse. The `metadata` on the operation is of type ExecuteOperationMetadata. If the client remains connected after the first response is returned after the server, then updates are streamed as if the client had called WaitExecution until the execution completes or the request reaches an error. The operation can also be queried using Operations API. The server NEED NOT implement other methods or functionality of the Operations API. Errors discovered during creation of the `Operation` will be reported as gRPC Status errors, while errors that occurred while running the action will be reported in the `status` field of the `ExecuteResponse`. The server MUST NOT set the `error` field of the `Operation` proto. The possible errors include: * `INVALID_ARGUMENT`: One or more arguments are invalid. * `FAILED_PRECONDITION`: One or more errors occurred in setting up the action requested, such as a missing input or command or no worker being available. The client may be able to fix the errors and retry. * `RESOURCE_EXHAUSTED`: There is insufficient quota of some resource to run the action. * `UNAVAILABLE`: Due to a transient condition, such as all workers being occupied (and the server does not support a queue), the action could not be started. The client should retry. * `INTERNAL`: An internal error occurred in the execution engine or the worker. * `DEADLINE_EXCEEDED`: The execution timed out. * `CANCELLED`: The operation was cancelled by the client. This status is only possible if the server implements the Operations API CancelOperation method, and it was called for the current execution. In the case of a missing input or command, the server SHOULD additionally send a PreconditionFailure error detail where, for each requested blob not present in the CAS, there is a `Violation` with a `type` of `MISSING` and a `subject` of `"blobs/{hash}/{size}"` indicating the digest of the missing blob. The server does not need to guarantee that a call to this method leads to at most one execution of the action. The server MAY execute the action multiple times, potentially in parallel. These redundant executions MAY continue to run, even if the operation is completed.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `instanceName` - The instance of the execution system to operate against. A server may support multiple instances of the execution system (with their own workers, storage, caches, etc.). The server MAY require use of this field to select between them in an implementation-defined fashion, otherwise it can be omitted.
    pub fn execute(
        &self,
        request: BuildBazelRemoteExecutionV2ExecuteRequest,
        instance_name: &str,
    ) -> ActionExecuteCall<'a, C> {
        ActionExecuteCall {
            hub: self.hub,
            _request: request,
            _instance_name: instance_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *blob* resources.
/// It is not used directly, but through the [`RemoteBuildExecution`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_remotebuildexecution2 as remotebuildexecution2;
///
/// # async fn dox() {
/// use remotebuildexecution2::{RemoteBuildExecution, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// let mut hub = RemoteBuildExecution::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `batch_read(...)`, `batch_update(...)`, `find_missing(...)` and `get_tree(...)`
/// // to build up your call.
/// let rb = hub.blobs();
/// # }
/// ```
pub struct BlobMethods<'a, C>
where
    C: 'a,
{
    hub: &'a RemoteBuildExecution<C>,
}

impl<'a, C> common::MethodsBuilder for BlobMethods<'a, C> {}

impl<'a, C> BlobMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Download many blobs at once. The server may enforce a limit of the combined total size of blobs to be downloaded using this API. This limit may be obtained using the Capabilities API. Requests exceeding the limit should either be split into smaller chunks or downloaded using the ByteStream API, as appropriate. This request is equivalent to calling a Bytestream `Read` request on each individual blob, in parallel. The requests may succeed or fail independently. Errors: * `INVALID_ARGUMENT`: The client attempted to read more than the server supported limit. Every error on individual read will be returned in the corresponding digest status.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `instanceName` - The instance of the execution system to operate against. A server may support multiple instances of the execution system (with their own workers, storage, caches, etc.). The server MAY require use of this field to select between them in an implementation-defined fashion, otherwise it can be omitted.
    pub fn batch_read(
        &self,
        request: BuildBazelRemoteExecutionV2BatchReadBlobsRequest,
        instance_name: &str,
    ) -> BlobBatchReadCall<'a, C> {
        BlobBatchReadCall {
            hub: self.hub,
            _request: request,
            _instance_name: instance_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Upload many blobs at once. The server may enforce a limit of the combined total size of blobs to be uploaded using this API. This limit may be obtained using the Capabilities API. Requests exceeding the limit should either be split into smaller chunks or uploaded using the ByteStream API, as appropriate. This request is equivalent to calling a Bytestream `Write` request on each individual blob, in parallel. The requests may succeed or fail independently. Errors: * `INVALID_ARGUMENT`: The client attempted to upload more than the server supported limit. Individual requests may return the following errors, additionally: * `RESOURCE_EXHAUSTED`: There is insufficient disk quota to store the blob. * `INVALID_ARGUMENT`: The Digest does not match the provided data.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `instanceName` - The instance of the execution system to operate against. A server may support multiple instances of the execution system (with their own workers, storage, caches, etc.). The server MAY require use of this field to select between them in an implementation-defined fashion, otherwise it can be omitted.
    pub fn batch_update(
        &self,
        request: BuildBazelRemoteExecutionV2BatchUpdateBlobsRequest,
        instance_name: &str,
    ) -> BlobBatchUpdateCall<'a, C> {
        BlobBatchUpdateCall {
            hub: self.hub,
            _request: request,
            _instance_name: instance_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Determine if blobs are present in the CAS. Clients can use this API before uploading blobs to determine which ones are already present in the CAS and do not need to be uploaded again. Servers SHOULD increase the lifetimes of the referenced blobs if necessary and applicable. There are no method-specific errors.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `instanceName` - The instance of the execution system to operate against. A server may support multiple instances of the execution system (with their own workers, storage, caches, etc.). The server MAY require use of this field to select between them in an implementation-defined fashion, otherwise it can be omitted.
    pub fn find_missing(
        &self,
        request: BuildBazelRemoteExecutionV2FindMissingBlobsRequest,
        instance_name: &str,
    ) -> BlobFindMissingCall<'a, C> {
        BlobFindMissingCall {
            hub: self.hub,
            _request: request,
            _instance_name: instance_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Fetch the entire directory tree rooted at a node. This request must be targeted at a Directory stored in the ContentAddressableStorage (CAS). The server will enumerate the `Directory` tree recursively and return every node descended from the root. The GetTreeRequest.page_token parameter can be used to skip ahead in the stream (e.g. when retrying a partially completed and aborted request), by setting it to a value taken from GetTreeResponse.next_page_token of the last successfully processed GetTreeResponse). The exact traversal order is unspecified and, unless retrieving subsequent pages from an earlier request, is not guaranteed to be stable across multiple invocations of `GetTree`. If part of the tree is missing from the CAS, the server will return the portion present and omit the rest. Errors: * `NOT_FOUND`: The requested tree root is not present in the CAS.
    ///
    /// # Arguments
    ///
    /// * `instanceName` - The instance of the execution system to operate against. A server may support multiple instances of the execution system (with their own workers, storage, caches, etc.). The server MAY require use of this field to select between them in an implementation-defined fashion, otherwise it can be omitted.
    /// * `hash` - The hash. In the case of SHA-256, it will always be a lowercase hex string exactly 64 characters long.
    /// * `sizeBytes` - The size of the blob, in bytes.
    pub fn get_tree(
        &self,
        instance_name: &str,
        hash: &str,
        size_bytes: i64,
    ) -> BlobGetTreeCall<'a, C> {
        BlobGetTreeCall {
            hub: self.hub,
            _instance_name: instance_name.to_string(),
            _hash: hash.to_string(),
            _size_bytes: size_bytes,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *operation* resources.
/// It is not used directly, but through the [`RemoteBuildExecution`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_remotebuildexecution2 as remotebuildexecution2;
///
/// # async fn dox() {
/// use remotebuildexecution2::{RemoteBuildExecution, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// let mut hub = RemoteBuildExecution::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `wait_execution(...)`
/// // to build up your call.
/// let rb = hub.operations();
/// # }
/// ```
pub struct OperationMethods<'a, C>
where
    C: 'a,
{
    hub: &'a RemoteBuildExecution<C>,
}

impl<'a, C> common::MethodsBuilder for OperationMethods<'a, C> {}

impl<'a, C> OperationMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Wait for an execution operation to complete. When the client initially makes the request, the server immediately responds with the current status of the execution. The server will leave the request stream open until the operation completes, and then respond with the completed operation. The server MAY choose to stream additional updates as execution progresses, such as to provide an update as to the state of the execution.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the Operation returned by Execute.
    pub fn wait_execution(
        &self,
        request: BuildBazelRemoteExecutionV2WaitExecutionRequest,
        name: &str,
    ) -> OperationWaitExecutionCall<'a, C> {
        OperationWaitExecutionCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

/// A builder providing access to all free methods, which are not associated with a particular resource.
/// It is not used directly, but through the [`RemoteBuildExecution`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_remotebuildexecution2 as remotebuildexecution2;
///
/// # async fn dox() {
/// use remotebuildexecution2::{RemoteBuildExecution, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// let mut hub = RemoteBuildExecution::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_capabilities(...)`
/// // to build up your call.
/// let rb = hub.methods();
/// # }
/// ```
pub struct MethodMethods<'a, C>
where
    C: 'a,
{
    hub: &'a RemoteBuildExecution<C>,
}

impl<'a, C> common::MethodsBuilder for MethodMethods<'a, C> {}

impl<'a, C> MethodMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// GetCapabilities returns the server capabilities configuration of the remote endpoint. Only the capabilities of the services supported by the endpoint will be returned: * Execution + CAS + Action Cache endpoints should return both CacheCapabilities and ExecutionCapabilities. * Execution only endpoints should return ExecutionCapabilities. * CAS + Action Cache only endpoints should return CacheCapabilities.
    ///
    /// # Arguments
    ///
    /// * `instanceName` - The instance of the execution system to operate against. A server may support multiple instances of the execution system (with their own workers, storage, caches, etc.). The server MAY require use of this field to select between them in an implementation-defined fashion, otherwise it can be omitted.
    pub fn get_capabilities(&self, instance_name: &str) -> MethodGetCapabilityCall<'a, C> {
        MethodGetCapabilityCall {
            hub: self.hub,
            _instance_name: instance_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

// ###################
// CallBuilders   ###
// #################

/// Retrieve a cached execution result. Implementations SHOULD ensure that any blobs referenced from the ContentAddressableStorage are available at the time of returning the ActionResult and will be for some period of time afterwards. The lifetimes of the referenced blobs SHOULD be increased if necessary and applicable. Errors: * `NOT_FOUND`: The requested `ActionResult` is not in the cache.
///
/// A builder for the *get* method supported by a *actionResult* resource.
/// It is not used directly, but through a [`ActionResultMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_remotebuildexecution2 as remotebuildexecution2;
/// # async fn dox() {
/// # use remotebuildexecution2::{RemoteBuildExecution, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = RemoteBuildExecution::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.action_results().get("instanceName", "hash", -93)
///              .inline_stdout(true)
///              .inline_stderr(true)
///              .add_inline_output_files("ipsum")
///              .doit().await;
/// # }
/// ```
pub struct ActionResultGetCall<'a, C>
where
    C: 'a,
{
    hub: &'a RemoteBuildExecution<C>,
    _instance_name: String,
    _hash: String,
    _size_bytes: i64,
    _inline_stdout: Option<bool>,
    _inline_stderr: Option<bool>,
    _inline_output_files: Vec<String>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ActionResultGetCall<'a, C> {}

impl<'a, C> ActionResultGetCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(
        mut self,
    ) -> common::Result<(common::Response, BuildBazelRemoteExecutionV2ActionResult)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "remotebuildexecution.actionResults.get",
            http_method: hyper::Method::GET,
        });

        for &field in [
            "alt",
            "instanceName",
            "hash",
            "sizeBytes",
            "inlineStdout",
            "inlineStderr",
            "inlineOutputFiles",
        ]
        .iter()
        {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(8 + self._additional_params.len());
        params.push("instanceName", self._instance_name);
        params.push("hash", self._hash);
        params.push("sizeBytes", self._size_bytes.to_string());
        if let Some(value) = self._inline_stdout.as_ref() {
            params.push("inlineStdout", value.to_string());
        }
        if let Some(value) = self._inline_stderr.as_ref() {
            params.push("inlineStderr", value.to_string());
        }
        if !self._inline_output_files.is_empty() {
            for f in self._inline_output_files.iter() {
                params.push("inlineOutputFiles", f);
            }
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url =
            self.hub._base_url.clone() + "v2/{+instanceName}/actionResults/{hash}/{sizeBytes}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [
            ("{+instanceName}", "instanceName"),
            ("{hash}", "hash"),
            ("{sizeBytes}", "sizeBytes"),
        ]
        .iter()
        {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["sizeBytes", "hash", "instanceName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

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

    /// The instance of the execution system to operate against. A server may support multiple instances of the execution system (with their own workers, storage, caches, etc.). The server MAY require use of this field to select between them in an implementation-defined fashion, otherwise it can be omitted.
    ///
    /// Sets the *instance name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn instance_name(mut self, new_value: &str) -> ActionResultGetCall<'a, C> {
        self._instance_name = new_value.to_string();
        self
    }
    /// The hash. In the case of SHA-256, it will always be a lowercase hex string exactly 64 characters long.
    ///
    /// Sets the *hash* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn hash(mut self, new_value: &str) -> ActionResultGetCall<'a, C> {
        self._hash = new_value.to_string();
        self
    }
    /// The size of the blob, in bytes.
    ///
    /// Sets the *size bytes* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn size_bytes(mut self, new_value: i64) -> ActionResultGetCall<'a, C> {
        self._size_bytes = new_value;
        self
    }
    /// A hint to the server to request inlining stdout in the ActionResult message.
    ///
    /// Sets the *inline stdout* query property to the given value.
    pub fn inline_stdout(mut self, new_value: bool) -> ActionResultGetCall<'a, C> {
        self._inline_stdout = Some(new_value);
        self
    }
    /// A hint to the server to request inlining stderr in the ActionResult message.
    ///
    /// Sets the *inline stderr* query property to the given value.
    pub fn inline_stderr(mut self, new_value: bool) -> ActionResultGetCall<'a, C> {
        self._inline_stderr = Some(new_value);
        self
    }
    /// A hint to the server to inline the contents of the listed output files. Each path needs to exactly match one file path in either `output_paths` or `output_files` (DEPRECATED since v2.1) in the Command message.
    ///
    /// Append the given value to the *inline output files* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_inline_output_files(mut self, new_value: &str) -> ActionResultGetCall<'a, C> {
        self._inline_output_files.push(new_value.to_string());
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
    ) -> ActionResultGetCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ActionResultGetCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ActionResultGetCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ActionResultGetCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ActionResultGetCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Upload a new execution result. In order to allow the server to perform access control based on the type of action, and to assist with client debugging, the client MUST first upload the Action that produced the result, along with its Command, into the `ContentAddressableStorage`. Server implementations MAY modify the `UpdateActionResultRequest.action_result` and return an equivalent value. Errors: * `INVALID_ARGUMENT`: One or more arguments are invalid. * `FAILED_PRECONDITION`: One or more errors occurred in updating the action result, such as a missing command or action. * `RESOURCE_EXHAUSTED`: There is insufficient storage space to add the entry to the cache.
///
/// A builder for the *update* method supported by a *actionResult* resource.
/// It is not used directly, but through a [`ActionResultMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_remotebuildexecution2 as remotebuildexecution2;
/// use remotebuildexecution2::api::BuildBazelRemoteExecutionV2ActionResult;
/// # async fn dox() {
/// # use remotebuildexecution2::{RemoteBuildExecution, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = RemoteBuildExecution::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BuildBazelRemoteExecutionV2ActionResult::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.action_results().update(req, "instanceName", "hash", -17)
///              .results_cache_policy_priority(-99)
///              .doit().await;
/// # }
/// ```
pub struct ActionResultUpdateCall<'a, C>
where
    C: 'a,
{
    hub: &'a RemoteBuildExecution<C>,
    _request: BuildBazelRemoteExecutionV2ActionResult,
    _instance_name: String,
    _hash: String,
    _size_bytes: i64,
    _results_cache_policy_priority: Option<i32>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ActionResultUpdateCall<'a, C> {}

impl<'a, C> ActionResultUpdateCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(
        mut self,
    ) -> common::Result<(common::Response, BuildBazelRemoteExecutionV2ActionResult)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "remotebuildexecution.actionResults.update",
            http_method: hyper::Method::PUT,
        });

        for &field in [
            "alt",
            "instanceName",
            "hash",
            "sizeBytes",
            "resultsCachePolicy.priority",
        ]
        .iter()
        {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("instanceName", self._instance_name);
        params.push("hash", self._hash);
        params.push("sizeBytes", self._size_bytes.to_string());
        if let Some(value) = self._results_cache_policy_priority.as_ref() {
            params.push("resultsCachePolicy.priority", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url =
            self.hub._base_url.clone() + "v2/{+instanceName}/actionResults/{hash}/{sizeBytes}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [
            ("{+instanceName}", "instanceName"),
            ("{hash}", "hash"),
            ("{sizeBytes}", "sizeBytes"),
        ]
        .iter()
        {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["sizeBytes", "hash", "instanceName"];
            params.remove_params(&to_remove);
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
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::PUT)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

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
        new_value: BuildBazelRemoteExecutionV2ActionResult,
    ) -> ActionResultUpdateCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The instance of the execution system to operate against. A server may support multiple instances of the execution system (with their own workers, storage, caches, etc.). The server MAY require use of this field to select between them in an implementation-defined fashion, otherwise it can be omitted.
    ///
    /// Sets the *instance name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn instance_name(mut self, new_value: &str) -> ActionResultUpdateCall<'a, C> {
        self._instance_name = new_value.to_string();
        self
    }
    /// The hash. In the case of SHA-256, it will always be a lowercase hex string exactly 64 characters long.
    ///
    /// Sets the *hash* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn hash(mut self, new_value: &str) -> ActionResultUpdateCall<'a, C> {
        self._hash = new_value.to_string();
        self
    }
    /// The size of the blob, in bytes.
    ///
    /// Sets the *size bytes* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn size_bytes(mut self, new_value: i64) -> ActionResultUpdateCall<'a, C> {
        self._size_bytes = new_value;
        self
    }
    /// The priority (relative importance) of this content in the overall cache. Generally, a lower value means a longer retention time or other advantage, but the interpretation of a given value is server-dependent. A priority of 0 means a *default* value, decided by the server. The particular semantics of this field is up to the server. In particular, every server will have their own supported range of priorities, and will decide how these map into retention/eviction policy.
    ///
    /// Sets the *results cache policy.priority* query property to the given value.
    pub fn results_cache_policy_priority(
        mut self,
        new_value: i32,
    ) -> ActionResultUpdateCall<'a, C> {
        self._results_cache_policy_priority = Some(new_value);
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
    ) -> ActionResultUpdateCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ActionResultUpdateCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ActionResultUpdateCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ActionResultUpdateCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ActionResultUpdateCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Execute an action remotely. In order to execute an action, the client must first upload all of the inputs, the Command to run, and the Action into the ContentAddressableStorage. It then calls `Execute` with an `action_digest` referring to them. The server will run the action and eventually return the result. The input `Action`'s fields MUST meet the various canonicalization requirements specified in the documentation for their types so that it has the same digest as other logically equivalent `Action`s. The server MAY enforce the requirements and return errors if a non-canonical input is received. It MAY also proceed without verifying some or all of the requirements, such as for performance reasons. If the server does not verify the requirement, then it will treat the `Action` as distinct from another logically equivalent action if they hash differently. Returns a stream of google.longrunning.Operation messages describing the resulting execution, with eventual `response` ExecuteResponse. The `metadata` on the operation is of type ExecuteOperationMetadata. If the client remains connected after the first response is returned after the server, then updates are streamed as if the client had called WaitExecution until the execution completes or the request reaches an error. The operation can also be queried using Operations API. The server NEED NOT implement other methods or functionality of the Operations API. Errors discovered during creation of the `Operation` will be reported as gRPC Status errors, while errors that occurred while running the action will be reported in the `status` field of the `ExecuteResponse`. The server MUST NOT set the `error` field of the `Operation` proto. The possible errors include: * `INVALID_ARGUMENT`: One or more arguments are invalid. * `FAILED_PRECONDITION`: One or more errors occurred in setting up the action requested, such as a missing input or command or no worker being available. The client may be able to fix the errors and retry. * `RESOURCE_EXHAUSTED`: There is insufficient quota of some resource to run the action. * `UNAVAILABLE`: Due to a transient condition, such as all workers being occupied (and the server does not support a queue), the action could not be started. The client should retry. * `INTERNAL`: An internal error occurred in the execution engine or the worker. * `DEADLINE_EXCEEDED`: The execution timed out. * `CANCELLED`: The operation was cancelled by the client. This status is only possible if the server implements the Operations API CancelOperation method, and it was called for the current execution. In the case of a missing input or command, the server SHOULD additionally send a PreconditionFailure error detail where, for each requested blob not present in the CAS, there is a `Violation` with a `type` of `MISSING` and a `subject` of `"blobs/{hash}/{size}"` indicating the digest of the missing blob. The server does not need to guarantee that a call to this method leads to at most one execution of the action. The server MAY execute the action multiple times, potentially in parallel. These redundant executions MAY continue to run, even if the operation is completed.
///
/// A builder for the *execute* method supported by a *action* resource.
/// It is not used directly, but through a [`ActionMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_remotebuildexecution2 as remotebuildexecution2;
/// use remotebuildexecution2::api::BuildBazelRemoteExecutionV2ExecuteRequest;
/// # async fn dox() {
/// # use remotebuildexecution2::{RemoteBuildExecution, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = RemoteBuildExecution::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BuildBazelRemoteExecutionV2ExecuteRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.actions().execute(req, "instanceName")
///              .doit().await;
/// # }
/// ```
pub struct ActionExecuteCall<'a, C>
where
    C: 'a,
{
    hub: &'a RemoteBuildExecution<C>,
    _request: BuildBazelRemoteExecutionV2ExecuteRequest,
    _instance_name: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ActionExecuteCall<'a, C> {}

impl<'a, C> ActionExecuteCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, GoogleLongrunningOperation)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "remotebuildexecution.actions.execute",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt", "instanceName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("instanceName", self._instance_name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v2/{+instanceName}/actions:execute";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+instanceName}", "instanceName")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["instanceName"];
            params.remove_params(&to_remove);
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
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
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

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

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
        new_value: BuildBazelRemoteExecutionV2ExecuteRequest,
    ) -> ActionExecuteCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The instance of the execution system to operate against. A server may support multiple instances of the execution system (with their own workers, storage, caches, etc.). The server MAY require use of this field to select between them in an implementation-defined fashion, otherwise it can be omitted.
    ///
    /// Sets the *instance name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn instance_name(mut self, new_value: &str) -> ActionExecuteCall<'a, C> {
        self._instance_name = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> ActionExecuteCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ActionExecuteCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ActionExecuteCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ActionExecuteCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ActionExecuteCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Download many blobs at once. The server may enforce a limit of the combined total size of blobs to be downloaded using this API. This limit may be obtained using the Capabilities API. Requests exceeding the limit should either be split into smaller chunks or downloaded using the ByteStream API, as appropriate. This request is equivalent to calling a Bytestream `Read` request on each individual blob, in parallel. The requests may succeed or fail independently. Errors: * `INVALID_ARGUMENT`: The client attempted to read more than the server supported limit. Every error on individual read will be returned in the corresponding digest status.
///
/// A builder for the *batchRead* method supported by a *blob* resource.
/// It is not used directly, but through a [`BlobMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_remotebuildexecution2 as remotebuildexecution2;
/// use remotebuildexecution2::api::BuildBazelRemoteExecutionV2BatchReadBlobsRequest;
/// # async fn dox() {
/// # use remotebuildexecution2::{RemoteBuildExecution, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = RemoteBuildExecution::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BuildBazelRemoteExecutionV2BatchReadBlobsRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.blobs().batch_read(req, "instanceName")
///              .doit().await;
/// # }
/// ```
pub struct BlobBatchReadCall<'a, C>
where
    C: 'a,
{
    hub: &'a RemoteBuildExecution<C>,
    _request: BuildBazelRemoteExecutionV2BatchReadBlobsRequest,
    _instance_name: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for BlobBatchReadCall<'a, C> {}

impl<'a, C> BlobBatchReadCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(
        mut self,
    ) -> common::Result<(
        common::Response,
        BuildBazelRemoteExecutionV2BatchReadBlobsResponse,
    )> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "remotebuildexecution.blobs.batchRead",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt", "instanceName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("instanceName", self._instance_name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v2/{+instanceName}/blobs:batchRead";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+instanceName}", "instanceName")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["instanceName"];
            params.remove_params(&to_remove);
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
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
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

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

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
        new_value: BuildBazelRemoteExecutionV2BatchReadBlobsRequest,
    ) -> BlobBatchReadCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The instance of the execution system to operate against. A server may support multiple instances of the execution system (with their own workers, storage, caches, etc.). The server MAY require use of this field to select between them in an implementation-defined fashion, otherwise it can be omitted.
    ///
    /// Sets the *instance name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn instance_name(mut self, new_value: &str) -> BlobBatchReadCall<'a, C> {
        self._instance_name = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> BlobBatchReadCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BlobBatchReadCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> BlobBatchReadCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> BlobBatchReadCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> BlobBatchReadCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Upload many blobs at once. The server may enforce a limit of the combined total size of blobs to be uploaded using this API. This limit may be obtained using the Capabilities API. Requests exceeding the limit should either be split into smaller chunks or uploaded using the ByteStream API, as appropriate. This request is equivalent to calling a Bytestream `Write` request on each individual blob, in parallel. The requests may succeed or fail independently. Errors: * `INVALID_ARGUMENT`: The client attempted to upload more than the server supported limit. Individual requests may return the following errors, additionally: * `RESOURCE_EXHAUSTED`: There is insufficient disk quota to store the blob. * `INVALID_ARGUMENT`: The Digest does not match the provided data.
///
/// A builder for the *batchUpdate* method supported by a *blob* resource.
/// It is not used directly, but through a [`BlobMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_remotebuildexecution2 as remotebuildexecution2;
/// use remotebuildexecution2::api::BuildBazelRemoteExecutionV2BatchUpdateBlobsRequest;
/// # async fn dox() {
/// # use remotebuildexecution2::{RemoteBuildExecution, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = RemoteBuildExecution::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BuildBazelRemoteExecutionV2BatchUpdateBlobsRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.blobs().batch_update(req, "instanceName")
///              .doit().await;
/// # }
/// ```
pub struct BlobBatchUpdateCall<'a, C>
where
    C: 'a,
{
    hub: &'a RemoteBuildExecution<C>,
    _request: BuildBazelRemoteExecutionV2BatchUpdateBlobsRequest,
    _instance_name: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for BlobBatchUpdateCall<'a, C> {}

impl<'a, C> BlobBatchUpdateCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(
        mut self,
    ) -> common::Result<(
        common::Response,
        BuildBazelRemoteExecutionV2BatchUpdateBlobsResponse,
    )> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "remotebuildexecution.blobs.batchUpdate",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt", "instanceName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("instanceName", self._instance_name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v2/{+instanceName}/blobs:batchUpdate";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+instanceName}", "instanceName")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["instanceName"];
            params.remove_params(&to_remove);
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
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
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

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

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
        new_value: BuildBazelRemoteExecutionV2BatchUpdateBlobsRequest,
    ) -> BlobBatchUpdateCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The instance of the execution system to operate against. A server may support multiple instances of the execution system (with their own workers, storage, caches, etc.). The server MAY require use of this field to select between them in an implementation-defined fashion, otherwise it can be omitted.
    ///
    /// Sets the *instance name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn instance_name(mut self, new_value: &str) -> BlobBatchUpdateCall<'a, C> {
        self._instance_name = new_value.to_string();
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
    ) -> BlobBatchUpdateCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BlobBatchUpdateCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> BlobBatchUpdateCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> BlobBatchUpdateCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> BlobBatchUpdateCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Determine if blobs are present in the CAS. Clients can use this API before uploading blobs to determine which ones are already present in the CAS and do not need to be uploaded again. Servers SHOULD increase the lifetimes of the referenced blobs if necessary and applicable. There are no method-specific errors.
///
/// A builder for the *findMissing* method supported by a *blob* resource.
/// It is not used directly, but through a [`BlobMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_remotebuildexecution2 as remotebuildexecution2;
/// use remotebuildexecution2::api::BuildBazelRemoteExecutionV2FindMissingBlobsRequest;
/// # async fn dox() {
/// # use remotebuildexecution2::{RemoteBuildExecution, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = RemoteBuildExecution::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BuildBazelRemoteExecutionV2FindMissingBlobsRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.blobs().find_missing(req, "instanceName")
///              .doit().await;
/// # }
/// ```
pub struct BlobFindMissingCall<'a, C>
where
    C: 'a,
{
    hub: &'a RemoteBuildExecution<C>,
    _request: BuildBazelRemoteExecutionV2FindMissingBlobsRequest,
    _instance_name: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for BlobFindMissingCall<'a, C> {}

impl<'a, C> BlobFindMissingCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(
        mut self,
    ) -> common::Result<(
        common::Response,
        BuildBazelRemoteExecutionV2FindMissingBlobsResponse,
    )> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "remotebuildexecution.blobs.findMissing",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt", "instanceName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("instanceName", self._instance_name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v2/{+instanceName}/blobs:findMissing";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+instanceName}", "instanceName")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["instanceName"];
            params.remove_params(&to_remove);
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
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
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

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

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
        new_value: BuildBazelRemoteExecutionV2FindMissingBlobsRequest,
    ) -> BlobFindMissingCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The instance of the execution system to operate against. A server may support multiple instances of the execution system (with their own workers, storage, caches, etc.). The server MAY require use of this field to select between them in an implementation-defined fashion, otherwise it can be omitted.
    ///
    /// Sets the *instance name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn instance_name(mut self, new_value: &str) -> BlobFindMissingCall<'a, C> {
        self._instance_name = new_value.to_string();
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
    ) -> BlobFindMissingCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BlobFindMissingCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> BlobFindMissingCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> BlobFindMissingCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> BlobFindMissingCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Fetch the entire directory tree rooted at a node. This request must be targeted at a Directory stored in the ContentAddressableStorage (CAS). The server will enumerate the `Directory` tree recursively and return every node descended from the root. The GetTreeRequest.page_token parameter can be used to skip ahead in the stream (e.g. when retrying a partially completed and aborted request), by setting it to a value taken from GetTreeResponse.next_page_token of the last successfully processed GetTreeResponse). The exact traversal order is unspecified and, unless retrieving subsequent pages from an earlier request, is not guaranteed to be stable across multiple invocations of `GetTree`. If part of the tree is missing from the CAS, the server will return the portion present and omit the rest. Errors: * `NOT_FOUND`: The requested tree root is not present in the CAS.
///
/// A builder for the *getTree* method supported by a *blob* resource.
/// It is not used directly, but through a [`BlobMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_remotebuildexecution2 as remotebuildexecution2;
/// # async fn dox() {
/// # use remotebuildexecution2::{RemoteBuildExecution, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = RemoteBuildExecution::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.blobs().get_tree("instanceName", "hash", -61)
///              .page_token("Stet")
///              .page_size(-13)
///              .doit().await;
/// # }
/// ```
pub struct BlobGetTreeCall<'a, C>
where
    C: 'a,
{
    hub: &'a RemoteBuildExecution<C>,
    _instance_name: String,
    _hash: String,
    _size_bytes: i64,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for BlobGetTreeCall<'a, C> {}

impl<'a, C> BlobGetTreeCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(
        mut self,
    ) -> common::Result<(common::Response, BuildBazelRemoteExecutionV2GetTreeResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "remotebuildexecution.blobs.getTree",
            http_method: hyper::Method::GET,
        });

        for &field in [
            "alt",
            "instanceName",
            "hash",
            "sizeBytes",
            "pageToken",
            "pageSize",
        ]
        .iter()
        {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("instanceName", self._instance_name);
        params.push("hash", self._hash);
        params.push("sizeBytes", self._size_bytes.to_string());
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url =
            self.hub._base_url.clone() + "v2/{+instanceName}/blobs/{hash}/{sizeBytes}:getTree";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [
            ("{+instanceName}", "instanceName"),
            ("{hash}", "hash"),
            ("{sizeBytes}", "sizeBytes"),
        ]
        .iter()
        {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["sizeBytes", "hash", "instanceName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

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

    /// The instance of the execution system to operate against. A server may support multiple instances of the execution system (with their own workers, storage, caches, etc.). The server MAY require use of this field to select between them in an implementation-defined fashion, otherwise it can be omitted.
    ///
    /// Sets the *instance name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn instance_name(mut self, new_value: &str) -> BlobGetTreeCall<'a, C> {
        self._instance_name = new_value.to_string();
        self
    }
    /// The hash. In the case of SHA-256, it will always be a lowercase hex string exactly 64 characters long.
    ///
    /// Sets the *hash* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn hash(mut self, new_value: &str) -> BlobGetTreeCall<'a, C> {
        self._hash = new_value.to_string();
        self
    }
    /// The size of the blob, in bytes.
    ///
    /// Sets the *size bytes* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn size_bytes(mut self, new_value: i64) -> BlobGetTreeCall<'a, C> {
        self._size_bytes = new_value;
        self
    }
    /// A page token, which must be a value received in a previous GetTreeResponse. If present, the server will use that token as an offset, returning only that page and the ones that succeed it.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> BlobGetTreeCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// A maximum page size to request. If present, the server will request no more than this many items. Regardless of whether a page size is specified, the server may place its own limit on the number of items to be returned and require the client to retrieve more items using a subsequent request.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> BlobGetTreeCall<'a, C> {
        self._page_size = Some(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> BlobGetTreeCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BlobGetTreeCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> BlobGetTreeCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> BlobGetTreeCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> BlobGetTreeCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Wait for an execution operation to complete. When the client initially makes the request, the server immediately responds with the current status of the execution. The server will leave the request stream open until the operation completes, and then respond with the completed operation. The server MAY choose to stream additional updates as execution progresses, such as to provide an update as to the state of the execution.
///
/// A builder for the *waitExecution* method supported by a *operation* resource.
/// It is not used directly, but through a [`OperationMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_remotebuildexecution2 as remotebuildexecution2;
/// use remotebuildexecution2::api::BuildBazelRemoteExecutionV2WaitExecutionRequest;
/// # async fn dox() {
/// # use remotebuildexecution2::{RemoteBuildExecution, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = RemoteBuildExecution::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BuildBazelRemoteExecutionV2WaitExecutionRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.operations().wait_execution(req, "name")
///              .doit().await;
/// # }
/// ```
pub struct OperationWaitExecutionCall<'a, C>
where
    C: 'a,
{
    hub: &'a RemoteBuildExecution<C>,
    _request: BuildBazelRemoteExecutionV2WaitExecutionRequest,
    _name: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for OperationWaitExecutionCall<'a, C> {}

impl<'a, C> OperationWaitExecutionCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, GoogleLongrunningOperation)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "remotebuildexecution.operations.waitExecution",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("name", self._name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v2/{+name}:waitExecution";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
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
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
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

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

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
        new_value: BuildBazelRemoteExecutionV2WaitExecutionRequest,
    ) -> OperationWaitExecutionCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The name of the Operation returned by Execute.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> OperationWaitExecutionCall<'a, C> {
        self._name = new_value.to_string();
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
    ) -> OperationWaitExecutionCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> OperationWaitExecutionCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OperationWaitExecutionCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OperationWaitExecutionCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OperationWaitExecutionCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// GetCapabilities returns the server capabilities configuration of the remote endpoint. Only the capabilities of the services supported by the endpoint will be returned: * Execution + CAS + Action Cache endpoints should return both CacheCapabilities and ExecutionCapabilities. * Execution only endpoints should return ExecutionCapabilities. * CAS + Action Cache only endpoints should return CacheCapabilities.
///
/// A builder for the *getCapabilities* method.
/// It is not used directly, but through a [`MethodMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_remotebuildexecution2 as remotebuildexecution2;
/// # async fn dox() {
/// # use remotebuildexecution2::{RemoteBuildExecution, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = RemoteBuildExecution::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().get_capabilities("instanceName")
///              .doit().await;
/// # }
/// ```
pub struct MethodGetCapabilityCall<'a, C>
where
    C: 'a,
{
    hub: &'a RemoteBuildExecution<C>,
    _instance_name: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for MethodGetCapabilityCall<'a, C> {}

impl<'a, C> MethodGetCapabilityCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(
        mut self,
    ) -> common::Result<(
        common::Response,
        BuildBazelRemoteExecutionV2ServerCapabilities,
    )> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "remotebuildexecution.getCapabilities",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "instanceName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("instanceName", self._instance_name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v2/{+instanceName}/capabilities";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+instanceName}", "instanceName")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["instanceName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

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

    /// The instance of the execution system to operate against. A server may support multiple instances of the execution system (with their own workers, storage, caches, etc.). The server MAY require use of this field to select between them in an implementation-defined fashion, otherwise it can be omitted.
    ///
    /// Sets the *instance name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn instance_name(mut self, new_value: &str) -> MethodGetCapabilityCall<'a, C> {
        self._instance_name = new_value.to_string();
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
    ) -> MethodGetCapabilityCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> MethodGetCapabilityCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MethodGetCapabilityCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MethodGetCapabilityCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MethodGetCapabilityCall<'a, C> {
        self._scopes.clear();
        self
    }
}
