use super::*;
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
/// use std::default::Default;
/// use remotebuildexecution2::{RemoteBuildExecution, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = RemoteBuildExecution::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.action_results();
/// # }
/// ```
pub struct ActionResultMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a RemoteBuildExecution<S>,
}

impl<'a, S> client::MethodsBuilder for ActionResultMethods<'a, S> {}

impl<'a, S> ActionResultMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieve a cached execution result. Implementations SHOULD ensure that any blobs referenced from the ContentAddressableStorage are available at the time of returning the ActionResult and will be for some period of time afterwards. The lifetimes of the referenced blobs SHOULD be increased if necessary and applicable. Errors: * `NOT_FOUND`: The requested `ActionResult` is not in the cache.
    /// 
    /// # Arguments
    ///
    /// * `instanceName` - The instance of the execution system to operate against. A server may support multiple instances of the execution system (with their own workers, storage, caches, etc.). The server MAY require use of this field to select between them in an implementation-defined fashion, otherwise it can be omitted.
    /// * `hash` - The hash. In the case of SHA-256, it will always be a lowercase hex string exactly 64 characters long.
    /// * `sizeBytes` - The size of the blob, in bytes.
    pub fn get(&self, instance_name: &str, hash: &str, size_bytes: i64) -> ActionResultGetCall<'a, S> {
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
    pub fn update(&self, request: BuildBazelRemoteExecutionV2ActionResult, instance_name: &str, hash: &str, size_bytes: i64) -> ActionResultUpdateCall<'a, S> {
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
/// use std::default::Default;
/// use remotebuildexecution2::{RemoteBuildExecution, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = RemoteBuildExecution::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `execute(...)`
/// // to build up your call.
/// let rb = hub.actions();
/// # }
/// ```
pub struct ActionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a RemoteBuildExecution<S>,
}

impl<'a, S> client::MethodsBuilder for ActionMethods<'a, S> {}

impl<'a, S> ActionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Execute an action remotely. In order to execute an action, the client must first upload all of the inputs, the Command to run, and the Action into the ContentAddressableStorage. It then calls `Execute` with an `action_digest` referring to them. The server will run the action and eventually return the result. The input `Action`'s fields MUST meet the various canonicalization requirements specified in the documentation for their types so that it has the same digest as other logically equivalent `Action`s. The server MAY enforce the requirements and return errors if a non-canonical input is received. It MAY also proceed without verifying some or all of the requirements, such as for performance reasons. If the server does not verify the requirement, then it will treat the `Action` as distinct from another logically equivalent action if they hash differently. Returns a stream of google.longrunning.Operation messages describing the resulting execution, with eventual `response` ExecuteResponse. The `metadata` on the operation is of type ExecuteOperationMetadata. If the client remains connected after the first response is returned after the server, then updates are streamed as if the client had called WaitExecution until the execution completes or the request reaches an error. The operation can also be queried using Operations API. The server NEED NOT implement other methods or functionality of the Operations API. Errors discovered during creation of the `Operation` will be reported as gRPC Status errors, while errors that occurred while running the action will be reported in the `status` field of the `ExecuteResponse`. The server MUST NOT set the `error` field of the `Operation` proto. The possible errors include: * `INVALID_ARGUMENT`: One or more arguments are invalid. * `FAILED_PRECONDITION`: One or more errors occurred in setting up the action requested, such as a missing input or command or no worker being available. The client may be able to fix the errors and retry. * `RESOURCE_EXHAUSTED`: There is insufficient quota of some resource to run the action. * `UNAVAILABLE`: Due to a transient condition, such as all workers being occupied (and the server does not support a queue), the action could not be started. The client should retry. * `INTERNAL`: An internal error occurred in the execution engine or the worker. * `DEADLINE_EXCEEDED`: The execution timed out. * `CANCELLED`: The operation was cancelled by the client. This status is only possible if the server implements the Operations API CancelOperation method, and it was called for the current execution. In the case of a missing input or command, the server SHOULD additionally send a PreconditionFailure error detail where, for each requested blob not present in the CAS, there is a `Violation` with a `type` of `MISSING` and a `subject` of `"blobs/{hash}/{size}"` indicating the digest of the missing blob. The server does not need to guarantee that a call to this method leads to at most one execution of the action. The server MAY execute the action multiple times, potentially in parallel. These redundant executions MAY continue to run, even if the operation is completed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `instanceName` - The instance of the execution system to operate against. A server may support multiple instances of the execution system (with their own workers, storage, caches, etc.). The server MAY require use of this field to select between them in an implementation-defined fashion, otherwise it can be omitted.
    pub fn execute(&self, request: BuildBazelRemoteExecutionV2ExecuteRequest, instance_name: &str) -> ActionExecuteCall<'a, S> {
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
/// use std::default::Default;
/// use remotebuildexecution2::{RemoteBuildExecution, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = RemoteBuildExecution::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `batch_read(...)`, `batch_update(...)`, `find_missing(...)` and `get_tree(...)`
/// // to build up your call.
/// let rb = hub.blobs();
/// # }
/// ```
pub struct BlobMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a RemoteBuildExecution<S>,
}

impl<'a, S> client::MethodsBuilder for BlobMethods<'a, S> {}

impl<'a, S> BlobMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Download many blobs at once. The server may enforce a limit of the combined total size of blobs to be downloaded using this API. This limit may be obtained using the Capabilities API. Requests exceeding the limit should either be split into smaller chunks or downloaded using the ByteStream API, as appropriate. This request is equivalent to calling a Bytestream `Read` request on each individual blob, in parallel. The requests may succeed or fail independently. Errors: * `INVALID_ARGUMENT`: The client attempted to read more than the server supported limit. Every error on individual read will be returned in the corresponding digest status.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `instanceName` - The instance of the execution system to operate against. A server may support multiple instances of the execution system (with their own workers, storage, caches, etc.). The server MAY require use of this field to select between them in an implementation-defined fashion, otherwise it can be omitted.
    pub fn batch_read(&self, request: BuildBazelRemoteExecutionV2BatchReadBlobsRequest, instance_name: &str) -> BlobBatchReadCall<'a, S> {
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
    pub fn batch_update(&self, request: BuildBazelRemoteExecutionV2BatchUpdateBlobsRequest, instance_name: &str) -> BlobBatchUpdateCall<'a, S> {
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
    pub fn find_missing(&self, request: BuildBazelRemoteExecutionV2FindMissingBlobsRequest, instance_name: &str) -> BlobFindMissingCall<'a, S> {
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
    pub fn get_tree(&self, instance_name: &str, hash: &str, size_bytes: i64) -> BlobGetTreeCall<'a, S> {
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
/// use std::default::Default;
/// use remotebuildexecution2::{RemoteBuildExecution, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = RemoteBuildExecution::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `wait_execution(...)`
/// // to build up your call.
/// let rb = hub.operations();
/// # }
/// ```
pub struct OperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a RemoteBuildExecution<S>,
}

impl<'a, S> client::MethodsBuilder for OperationMethods<'a, S> {}

impl<'a, S> OperationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Wait for an execution operation to complete. When the client initially makes the request, the server immediately responds with the current status of the execution. The server will leave the request stream open until the operation completes, and then respond with the completed operation. The server MAY choose to stream additional updates as execution progresses, such as to provide an update as to the state of the execution.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the Operation returned by Execute.
    pub fn wait_execution(&self, request: BuildBazelRemoteExecutionV2WaitExecutionRequest, name: &str) -> OperationWaitExecutionCall<'a, S> {
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
/// use std::default::Default;
/// use remotebuildexecution2::{RemoteBuildExecution, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = RemoteBuildExecution::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_capabilities(...)`
/// // to build up your call.
/// let rb = hub.methods();
/// # }
/// ```
pub struct MethodMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a RemoteBuildExecution<S>,
}

impl<'a, S> client::MethodsBuilder for MethodMethods<'a, S> {}

impl<'a, S> MethodMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// GetCapabilities returns the server capabilities configuration of the remote endpoint. Only the capabilities of the services supported by the endpoint will be returned: * Execution + CAS + Action Cache endpoints should return both CacheCapabilities and ExecutionCapabilities. * Execution only endpoints should return ExecutionCapabilities. * CAS + Action Cache only endpoints should return CacheCapabilities.
    /// 
    /// # Arguments
    ///
    /// * `instanceName` - The instance of the execution system to operate against. A server may support multiple instances of the execution system (with their own workers, storage, caches, etc.). The server MAY require use of this field to select between them in an implementation-defined fashion, otherwise it can be omitted.
    pub fn get_capabilities(&self, instance_name: &str) -> MethodGetCapabilityCall<'a, S> {
        MethodGetCapabilityCall {
            hub: self.hub,
            _instance_name: instance_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



