// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Remote Build Execution* crate version *1.0.9+20190702*, where *20190702* is the exact revision of the *remotebuildexecution:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.9*.
//! 
//! Everything else about the *Remote Build Execution* *v2* API can be found at the
//! [official documentation site](https://cloud.google.com/remote-build-execution/docs/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/remotebuildexecution2).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.RemoteBuildExecution.html) ... 
//! 
//! * action results
//!  * [*get*](struct.ActionResultGetCall.html) and [*update*](struct.ActionResultUpdateCall.html)
//! * actions
//!  * [*execute*](struct.ActionExecuteCall.html)
//! * blobs
//!  * [*batch read*](struct.BlobBatchReadCall.html), [*batch update*](struct.BlobBatchUpdateCall.html), [*find missing*](struct.BlobFindMissingCall.html) and [*get tree*](struct.BlobGetTreeCall.html)
//! * operations
//!  * [*wait execution*](struct.OperationWaitExecutionCall.html)
//! 
//! Other activities are ...
//! 
//! * [get capabilities](struct.MethodGetCapabilityCall.html)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.RemoteBuildExecution.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](trait.MethodsBuilder.html) which in turn
//!       allow access to individual [*Call Builders*](trait.CallBuilder.html)
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit()
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.action_results().get(...).doit()
//! let r = hub.action_results().update(...).doit()
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-remotebuildexecution2 = "*"
//! # This project intentionally uses an old version of Hyper. See
//! # https://github.com/Byron/google-apis-rs/issues/173 for more
//! # information.
//! hyper = "^0.10"
//! hyper-rustls = "^0.6"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! yup-oauth2 = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_remotebuildexecution2 as remotebuildexecution2;
//! use remotebuildexecution2::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use remotebuildexecution2::RemoteBuildExecution;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
//!                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = RemoteBuildExecution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.action_results().get("instanceName", "hash", "sizeBytes")
//!              .inline_stdout(false)
//!              .inline_stderr(true)
//!              .add_inline_output_files("erat")
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](enum.Result.html), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](trait.ResponseResult.html), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](trait.Delegate.html) to the 
//! [Method Builder](trait.CallBuilder.html) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [enocodable](trait.RequestValue.html) and 
//! [decodable](trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;
use std::time::Duration;

pub use cmn::*;


// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// View and manage your data across Google Cloud Platform services
    CloudPlatform,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
        }
    }
}

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
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_remotebuildexecution2 as remotebuildexecution2;
/// use remotebuildexecution2::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use remotebuildexecution2::RemoteBuildExecution;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = RemoteBuildExecution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.action_results().get("instanceName", "hash", "sizeBytes")
///              .inline_stdout(true)
///              .inline_stderr(false)
///              .add_inline_output_files("sadipscing")
///              .doit();
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
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
pub struct RemoteBuildExecution<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for RemoteBuildExecution<C, A> {}

impl<'a, C, A> RemoteBuildExecution<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> RemoteBuildExecution<C, A> {
        RemoteBuildExecution {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.9".to_string(),
            _base_url: "https://remotebuildexecution.googleapis.com/".to_string(),
            _root_url: "https://remotebuildexecution.googleapis.com/".to_string(),
        }
    }

    pub fn action_results(&'a self) -> ActionResultMethods<'a, C, A> {
        ActionResultMethods { hub: &self }
    }
    pub fn actions(&'a self) -> ActionMethods<'a, C, A> {
        ActionMethods { hub: &self }
    }
    pub fn blobs(&'a self) -> BlobMethods<'a, C, A> {
        BlobMethods { hub: &self }
    }
    pub fn methods(&'a self) -> MethodMethods<'a, C, A> {
        MethodMethods { hub: &self }
    }
    pub fn operations(&'a self) -> OperationMethods<'a, C, A> {
        OperationMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.9`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://remotebuildexecution.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://remotebuildexecution.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Supported range of priorities, including boundaries.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2PriorityCapabilitiesPriorityRange {
    /// no description provided
    #[serde(rename="maxPriority")]
    pub max_priority: Option<i32>,
    /// no description provided
    #[serde(rename="minPriority")]
    pub min_priority: Option<i32>,
}

impl Part for BuildBazelRemoteExecutionV2PriorityCapabilitiesPriorityRange {}


/// A `SymlinkNode` represents a symbolic link.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2SymlinkNode {
    /// The name of the symlink.
    pub name: Option<String>,
    /// The target path of the symlink. The path separator is a forward slash `/`.
    /// The target path can be relative to the parent directory of the symlink or
    /// it can be an absolute path starting with `/`. Support for absolute paths
    /// can be checked using the Capabilities
    /// API. The canonical form forbids the substrings `/./` and `//` in the target
    /// path. `..` components are allowed anywhere in the target path.
    pub target: Option<String>,
}

impl Part for BuildBazelRemoteExecutionV2SymlinkNode {}


/// A `Directory` represents a directory node in a file tree, containing zero or
/// more children FileNodes,
/// DirectoryNodes and
/// SymlinkNodes.
/// Each `Node` contains its name in the directory, either the digest of its
/// content (either a file blob or a `Directory` proto) or a symlink target, as
/// well as possibly some metadata about the file or directory.
/// 
/// In order to ensure that two equivalent directory trees hash to the same
/// value, the following restrictions MUST be obeyed when constructing a
/// a `Directory`:
/// 
/// * Every child in the directory must have a path of exactly one segment.
///   Multiple levels of directory hierarchy may not be collapsed.
/// * Each child in the directory must have a unique path segment (file name).
///   Note that while the API itself is case-sensitive, the environment where
///   the Action is executed may or may not be case-sensitive. That is, it is
///   legal to call the API with a Directory that has both "Foo" and "foo" as
///   children, but the Action may be rejected by the remote system upon
///   execution.
/// * The files, directories and symlinks in the directory must each be sorted
///   in lexicographical order by path. The path strings must be sorted by code
///   point, equivalently, by UTF-8 bytes.
/// 
/// A `Directory` that obeys the restrictions is said to be in canonical form.
/// 
/// As an example, the following could be used for a file named `bar` and a
/// directory named `foo` with an executable file named `baz` (hashes shortened
/// for readability):
/// 
/// ```json
/// // (Directory proto)
/// {
///   files: [
///     {
///       name: "bar",
///       digest: {
///         hash: "4a73bc9d03...",
///         size: 65534
///       }
///     }
///   ],
///   directories: [
///     {
///       name: "foo",
///       digest: {
///         hash: "4cf2eda940...",
///         size: 43
///       }
///     }
///   ]
/// }
/// 
/// // (Directory proto with hash "4cf2eda940..." and size 43)
/// {
///   files: [
///     {
///       name: "baz",
///       digest: {
///         hash: "b2c941073e...",
///         size: 1294,
///       },
///       is_executable: true
///     }
///   ]
/// }
/// ```
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2Directory {
    /// The files in the directory.
    pub files: Option<Vec<BuildBazelRemoteExecutionV2FileNode>>,
    /// The symlinks in the directory.
    pub symlinks: Option<Vec<BuildBazelRemoteExecutionV2SymlinkNode>>,
    /// The subdirectories in the directory.
    pub directories: Option<Vec<BuildBazelRemoteExecutionV2DirectoryNode>>,
}

impl Part for BuildBazelRemoteExecutionV2Directory {}


/// A response message for
/// ContentAddressableStorage.FindMissingBlobs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [find missing blobs](struct.BlobFindMissingCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2FindMissingBlobsResponse {
    /// A list of the blobs requested *not* present in the storage.
    #[serde(rename="missingBlobDigests")]
    pub missing_blob_digests: Option<Vec<BuildBazelRemoteExecutionV2Digest>>,
}

impl ResponseResult for BuildBazelRemoteExecutionV2FindMissingBlobsResponse {}


/// A response message for
/// ContentAddressableStorage.GetTree.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get tree blobs](struct.BlobGetTreeCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2GetTreeResponse {
    /// If present, signifies that there are more results which the client can
    /// retrieve by passing this as the page_token in a subsequent
    /// request.
    /// If empty, signifies that this is the last page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The directories descended from the requested root.
    pub directories: Option<Vec<BuildBazelRemoteExecutionV2Directory>>,
}

impl ResponseResult for BuildBazelRemoteExecutionV2GetTreeResponse {}


/// A request message for
/// ContentAddressableStorage.BatchReadBlobs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch read blobs](struct.BlobBatchReadCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2BatchReadBlobsRequest {
    /// The individual blob digests.
    pub digests: Option<Vec<BuildBazelRemoteExecutionV2Digest>>,
}

impl RequestValue for BuildBazelRemoteExecutionV2BatchReadBlobsRequest {}


/// Allowed values for priority in
/// ResultsCachePolicy
/// Used for querying both cache and execution valid priority ranges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2PriorityCapabilities {
    /// no description provided
    pub priorities: Option<Vec<BuildBazelRemoteExecutionV2PriorityCapabilitiesPriorityRange>>,
}

impl Part for BuildBazelRemoteExecutionV2PriorityCapabilities {}


/// A request message for
/// WaitExecution.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [wait execution operations](struct.OperationWaitExecutionCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2WaitExecutionRequest { _never_set: Option<bool> }

impl RequestValue for BuildBazelRemoteExecutionV2WaitExecutionRequest {}


/// A request corresponding to a single blob that the client wants to upload.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2BatchUpdateBlobsRequestRequest {
    /// The raw binary data.
    pub data: Option<String>,
    /// The digest of the blob. This MUST be the digest of `data`.
    pub digest: Option<BuildBazelRemoteExecutionV2Digest>,
}

impl Part for BuildBazelRemoteExecutionV2BatchUpdateBlobsRequestRequest {}


/// An ActionResult represents the result of an
/// Action being run.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get action results](struct.ActionResultGetCall.html) (response)
/// * [update action results](struct.ActionResultUpdateCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2ActionResult {
    /// The details of the execution that originally produced this result.
    #[serde(rename="executionMetadata")]
    pub execution_metadata: Option<BuildBazelRemoteExecutionV2ExecutedActionMetadata>,
    /// The output files of the action that are symbolic links to other files. Those
    /// may be links to other output files, or input files, or even absolute paths
    /// outside of the working directory, if the server supports
    /// SymlinkAbsolutePathStrategy.ALLOWED.
    /// For each output file requested in the `output_files` field of the Action,
    /// if the corresponding file existed after
    /// the action completed, a single entry will be present either in this field,
    /// or in the `output_files` field, if the file was not a symbolic link.
    /// 
    /// If an output symbolic link of the same name was found, but its target
    /// type was not a regular file, the server will return a FAILED_PRECONDITION.
    /// If the action does not produce the requested output, then that output
    /// will be omitted from the list. The server is free to arrange the output
    /// list as desired; clients MUST NOT assume that the output list is sorted.
    #[serde(rename="outputFileSymlinks")]
    pub output_file_symlinks: Option<Vec<BuildBazelRemoteExecutionV2OutputSymlink>>,
    /// The digest for a blob containing the standard error of the action, which
    /// can be retrieved from the
    /// ContentAddressableStorage.
    #[serde(rename="stderrDigest")]
    pub stderr_digest: Option<BuildBazelRemoteExecutionV2Digest>,
    /// The standard output buffer of the action. The server SHOULD NOT inline
    /// stdout unless requested by the client in the
    /// GetActionResultRequest
    /// message. The server MAY omit inlining, even if requested, and MUST do so if inlining
    /// would cause the response to exceed message size limits.
    #[serde(rename="stdoutRaw")]
    pub stdout_raw: Option<String>,
    /// The standard error buffer of the action. The server SHOULD NOT inline
    /// stderr unless requested by the client in the
    /// GetActionResultRequest
    /// message. The server MAY omit inlining, even if requested, and MUST do so if inlining
    /// would cause the response to exceed message size limits.
    #[serde(rename="stderrRaw")]
    pub stderr_raw: Option<String>,
    /// The digest for a blob containing the standard output of the action, which
    /// can be retrieved from the
    /// ContentAddressableStorage.
    #[serde(rename="stdoutDigest")]
    pub stdout_digest: Option<BuildBazelRemoteExecutionV2Digest>,
    /// The output files of the action. For each output file requested in the
    /// `output_files` field of the Action, if the corresponding file existed after
    /// the action completed, a single entry will be present either in this field,
    /// or the `output_file_symlinks` field if the file was a symbolic link to
    /// another file.
    /// 
    /// If an output of the same name was found, but was a directory rather
    /// than a regular file, the server will return a FAILED_PRECONDITION.
    /// If the action does not produce the requested output, then that output
    /// will be omitted from the list. The server is free to arrange the output
    /// list as desired; clients MUST NOT assume that the output list is sorted.
    #[serde(rename="outputFiles")]
    pub output_files: Option<Vec<BuildBazelRemoteExecutionV2OutputFile>>,
    /// The output directories of the action that are symbolic links to other
    /// directories. Those may be links to other output directories, or input
    /// directories, or even absolute paths outside of the working directory,
    /// if the server supports
    /// SymlinkAbsolutePathStrategy.ALLOWED.
    /// For each output directory requested in the `output_directories` field of
    /// the Action, if the directory existed after the action completed, a
    /// single entry will be present either in this field, or in the
    /// `output_directories` field, if the directory was not a symbolic link.
    /// 
    /// If an output of the same name was found, but was a symbolic link to a file
    /// instead of a directory, the server will return a FAILED_PRECONDITION.
    /// If the action does not produce the requested output, then that output
    /// will be omitted from the list. The server is free to arrange the output
    /// list as desired; clients MUST NOT assume that the output list is sorted.
    #[serde(rename="outputDirectorySymlinks")]
    pub output_directory_symlinks: Option<Vec<BuildBazelRemoteExecutionV2OutputSymlink>>,
    /// The output directories of the action. For each output directory requested
    /// in the `output_directories` field of the Action, if the corresponding
    /// directory existed after the action completed, a single entry will be
    /// present in the output list, which will contain the digest of a
    /// Tree message containing the
    /// directory tree, and the path equal exactly to the corresponding Action
    /// output_directories member.
    /// 
    /// As an example, suppose the Action had an output directory `a/b/dir` and the
    /// execution produced the following contents in `a/b/dir`: a file named `bar`
    /// and a directory named `foo` with an executable file named `baz`. Then,
    /// output_directory will contain (hashes shortened for readability):
    /// 
    /// ```json
    /// // OutputDirectory proto:
    /// {
    ///   path: "a/b/dir"
    ///   tree_digest: {
    ///     hash: "4a73bc9d03...",
    ///     size: 55
    ///   }
    /// }
    /// // Tree proto with hash "4a73bc9d03..." and size 55:
    /// {
    ///   root: {
    ///     files: [
    ///       {
    ///         name: "bar",
    ///         digest: {
    ///           hash: "4a73bc9d03...",
    ///           size: 65534
    ///         }
    ///       }
    ///     ],
    ///     directories: [
    ///       {
    ///         name: "foo",
    ///         digest: {
    ///           hash: "4cf2eda940...",
    ///           size: 43
    ///         }
    ///       }
    ///     ]
    ///   }
    ///   children : {
    ///     // (Directory proto with hash "4cf2eda940..." and size 43)
    ///     files: [
    ///       {
    ///         name: "baz",
    ///         digest: {
    ///           hash: "b2c941073e...",
    ///           size: 1294,
    ///         },
    ///         is_executable: true
    ///       }
    ///     ]
    ///   }
    /// }
    /// ```
    /// If an output of the same name was found, but was not a directory, the
    /// server will return a FAILED_PRECONDITION.
    #[serde(rename="outputDirectories")]
    pub output_directories: Option<Vec<BuildBazelRemoteExecutionV2OutputDirectory>>,
    /// The exit code of the command.
    #[serde(rename="exitCode")]
    pub exit_code: Option<i32>,
}

impl RequestValue for BuildBazelRemoteExecutionV2ActionResult {}
impl ResponseResult for BuildBazelRemoteExecutionV2ActionResult {}


/// A response message for
/// Capabilities.GetCapabilities.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get capabilities](struct.MethodGetCapabilityCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2ServerCapabilities {
    /// Earliest RE API version supported, including deprecated versions.
    #[serde(rename="deprecatedApiVersion")]
    pub deprecated_api_version: Option<BuildBazelSemverSemVer>,
    /// Earliest non-deprecated RE API version supported.
    #[serde(rename="lowApiVersion")]
    pub low_api_version: Option<BuildBazelSemverSemVer>,
    /// Capabilities of the remote cache system.
    #[serde(rename="cacheCapabilities")]
    pub cache_capabilities: Option<BuildBazelRemoteExecutionV2CacheCapabilities>,
    /// Latest RE API version supported.
    #[serde(rename="highApiVersion")]
    pub high_api_version: Option<BuildBazelSemverSemVer>,
    /// Capabilities of the remote execution system.
    #[serde(rename="executionCapabilities")]
    pub execution_capabilities: Option<BuildBazelRemoteExecutionV2ExecutionCapabilities>,
}

impl ResponseResult for BuildBazelRemoteExecutionV2ServerCapabilities {}


/// A response message for
/// ContentAddressableStorage.BatchReadBlobs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch read blobs](struct.BlobBatchReadCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2BatchReadBlobsResponse {
    /// The responses to the requests.
    pub responses: Option<Vec<BuildBazelRemoteExecutionV2BatchReadBlobsResponseResponse>>,
}

impl ResponseResult for BuildBazelRemoteExecutionV2BatchReadBlobsResponse {}


/// A `DirectoryNode` represents a child of a
/// Directory which is itself
/// a `Directory` and its associated metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2DirectoryNode {
    /// The name of the directory.
    pub name: Option<String>,
    /// The digest of the
    /// Directory object
    /// represented. See Digest
    /// for information about how to take the digest of a proto message.
    pub digest: Option<BuildBazelRemoteExecutionV2Digest>,
}

impl Part for BuildBazelRemoteExecutionV2DirectoryNode {}


/// An `OutputDirectory` is the output in an `ActionResult` corresponding to a
/// directory's full contents rather than a single file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2OutputDirectory {
    /// The full path of the directory relative to the working directory. The path
    /// separator is a forward slash `/`. Since this is a relative path, it MUST
    /// NOT begin with a leading forward slash. The empty string value is allowed,
    /// and it denotes the entire working directory.
    pub path: Option<String>,
    /// The digest of the encoded
    /// Tree proto containing the
    /// directory's contents.
    #[serde(rename="treeDigest")]
    pub tree_digest: Option<BuildBazelRemoteExecutionV2Digest>,
}

impl Part for BuildBazelRemoteExecutionV2OutputDirectory {}


/// A request message for
/// Execution.Execute.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [execute actions](struct.ActionExecuteCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2ExecuteRequest {
    /// An optional policy for the results of this execution in the remote cache.
    /// The server will have a default policy if this is not provided.
    /// This may be applied to both the ActionResult and the associated blobs.
    #[serde(rename="resultsCachePolicy")]
    pub results_cache_policy: Option<BuildBazelRemoteExecutionV2ResultsCachePolicy>,
    /// If true, the action will be executed even if its result is already
    /// present in the ActionCache.
    /// The execution is still allowed to be merged with other in-flight executions
    /// of the same action, however - semantically, the service MUST only guarantee
    /// that the results of an execution with this field set were not visible
    /// before the corresponding execution request was sent.
    /// Note that actions from execution requests setting this field set are still
    /// eligible to be entered into the action cache upon completion, and services
    /// SHOULD overwrite any existing entries that may exist. This allows
    /// skip_cache_lookup requests to be used as a mechanism for replacing action
    /// cache entries that reference outputs no longer available or that are
    /// poisoned in any way.
    /// If false, the result may be served from the action cache.
    #[serde(rename="skipCacheLookup")]
    pub skip_cache_lookup: Option<bool>,
    /// The digest of the Action to
    /// execute.
    #[serde(rename="actionDigest")]
    pub action_digest: Option<BuildBazelRemoteExecutionV2Digest>,
    /// An optional policy for execution of the action.
    /// The server will have a default policy if this is not provided.
    #[serde(rename="executionPolicy")]
    pub execution_policy: Option<BuildBazelRemoteExecutionV2ExecutionPolicy>,
}

impl RequestValue for BuildBazelRemoteExecutionV2ExecuteRequest {}


/// Describes the server/instance capabilities for updating the action cache.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2ActionCacheUpdateCapabilities {
    /// no description provided
    #[serde(rename="updateEnabled")]
    pub update_enabled: Option<bool>,
}

impl Part for BuildBazelRemoteExecutionV2ActionCacheUpdateCapabilities {}


/// Capabilities of the remote cache system.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2CacheCapabilities {
    /// Supported cache priority range for both CAS and ActionCache.
    #[serde(rename="cachePriorityCapabilities")]
    pub cache_priority_capabilities: Option<BuildBazelRemoteExecutionV2PriorityCapabilities>,
    /// Maximum total size of blobs to be uploaded/downloaded using
    /// batch methods. A value of 0 means no limit is set, although
    /// in practice there will always be a message size limitation
    /// of the protocol in use, e.g. GRPC.
    #[serde(rename="maxBatchTotalSizeBytes")]
    pub max_batch_total_size_bytes: Option<String>,
    /// Capabilities for updating the action cache.
    #[serde(rename="actionCacheUpdateCapabilities")]
    pub action_cache_update_capabilities: Option<BuildBazelRemoteExecutionV2ActionCacheUpdateCapabilities>,
    /// All the digest functions supported by the remote cache.
    /// Remote cache may support multiple digest functions simultaneously.
    #[serde(rename="digestFunction")]
    pub digest_function: Option<Vec<String>>,
    /// Whether absolute symlink targets are supported.
    #[serde(rename="symlinkAbsolutePathStrategy")]
    pub symlink_absolute_path_strategy: Option<String>,
}

impl Part for BuildBazelRemoteExecutionV2CacheCapabilities {}


/// The full version of a given tool.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelSemverSemVer {
    /// The major version, e.g 10 for 10.2.3.
    pub major: Option<i32>,
    /// The pre-release version. Either this field or major/minor/patch fields
    /// must be filled. They are mutually exclusive. Pre-release versions are
    /// assumed to be earlier than any released versions.
    pub prerelease: Option<String>,
    /// The minor version, e.g. 2 for 10.2.3.
    pub minor: Option<i32>,
    /// The patch version, e.g 3 for 10.2.3.
    pub patch: Option<i32>,
}

impl Part for BuildBazelSemverSemVer {}


/// A request message for
/// ContentAddressableStorage.FindMissingBlobs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [find missing blobs](struct.BlobFindMissingCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2FindMissingBlobsRequest {
    /// A list of the blobs to check.
    #[serde(rename="blobDigests")]
    pub blob_digests: Option<Vec<BuildBazelRemoteExecutionV2Digest>>,
}

impl RequestValue for BuildBazelRemoteExecutionV2FindMissingBlobsRequest {}


/// A response corresponding to a single blob that the client tried to download.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2BatchReadBlobsResponseResponse {
    /// The result of attempting to download that blob.
    pub status: Option<GoogleRpcStatus>,
    /// The raw binary data.
    pub data: Option<String>,
    /// The digest to which this response corresponds.
    pub digest: Option<BuildBazelRemoteExecutionV2Digest>,
}

impl Part for BuildBazelRemoteExecutionV2BatchReadBlobsResponseResponse {}


/// An `OutputSymlink` is similar to a
/// Symlink, but it is used as an
/// output in an `ActionResult`.
/// 
/// `OutputSymlink` is binary-compatible with `SymlinkNode`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2OutputSymlink {
    /// The full path of the symlink relative to the working directory, including the
    /// filename. The path separator is a forward slash `/`. Since this is a
    /// relative path, it MUST NOT begin with a leading forward slash.
    pub path: Option<String>,
    /// The target path of the symlink. The path separator is a forward slash `/`.
    /// The target path can be relative to the parent directory of the symlink or
    /// it can be an absolute path starting with `/`. Support for absolute paths
    /// can be checked using the Capabilities
    /// API. The canonical form forbids the substrings `/./` and `//` in the target
    /// path. `..` components are allowed anywhere in the target path.
    pub target: Option<String>,
}

impl Part for BuildBazelRemoteExecutionV2OutputSymlink {}


/// A response corresponding to a single blob that the client tried to upload.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2BatchUpdateBlobsResponseResponse {
    /// The result of attempting to upload that blob.
    pub status: Option<GoogleRpcStatus>,
    /// The blob digest to which this response corresponds.
    pub digest: Option<BuildBazelRemoteExecutionV2Digest>,
}

impl Part for BuildBazelRemoteExecutionV2BatchUpdateBlobsResponseResponse {}


/// The `Status` type defines a logical error model that is suitable for
/// different programming environments, including REST APIs and RPC APIs. It is
/// used by [gRPC](https://github.com/grpc). Each `Status` message contains
/// three pieces of data: error code, error message, and error details.
/// 
/// You can find out more about this error model and how to work with it in the
/// [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleRpcStatus {
    /// A developer-facing error message, which should be in English. Any
    /// user-facing error message should be localized and sent in the
    /// google.rpc.Status.details field, or localized by the client.
    pub message: Option<String>,
    /// The status code, which should be an enum value of google.rpc.Code.
    pub code: Option<i32>,
    /// A list of messages that carry the error details.  There is a common set of
    /// message types for APIs to use.
    pub details: Option<Vec<HashMap<String, String>>>,
}

impl Part for GoogleRpcStatus {}


/// A `FileNode` represents a single file and associated metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2FileNode {
    /// True if file is executable, false otherwise.
    #[serde(rename="isExecutable")]
    pub is_executable: Option<bool>,
    /// The name of the file.
    pub name: Option<String>,
    /// The digest of the file's content.
    pub digest: Option<BuildBazelRemoteExecutionV2Digest>,
}

impl Part for BuildBazelRemoteExecutionV2FileNode {}


/// An `OutputFile` is similar to a
/// FileNode, but it is used as an
/// output in an `ActionResult`. It allows a full file path rather than
/// only a name.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2OutputFile {
    /// The full path of the file relative to the working directory, including the
    /// filename. The path separator is a forward slash `/`. Since this is a
    /// relative path, it MUST NOT begin with a leading forward slash.
    pub path: Option<String>,
    /// True if file is executable, false otherwise.
    #[serde(rename="isExecutable")]
    pub is_executable: Option<bool>,
    /// The contents of the file if inlining was requested. The server SHOULD NOT inline
    /// file contents unless requested by the client in the
    /// GetActionResultRequest
    /// message. The server MAY omit inlining, even if requested, and MUST do so if inlining
    /// would cause the response to exceed message size limits.
    pub contents: Option<String>,
    /// The digest of the file's content.
    pub digest: Option<BuildBazelRemoteExecutionV2Digest>,
}

impl Part for BuildBazelRemoteExecutionV2OutputFile {}


/// A response message for
/// ContentAddressableStorage.BatchUpdateBlobs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch update blobs](struct.BlobBatchUpdateCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2BatchUpdateBlobsResponse {
    /// The responses to the requests.
    pub responses: Option<Vec<BuildBazelRemoteExecutionV2BatchUpdateBlobsResponseResponse>>,
}

impl ResponseResult for BuildBazelRemoteExecutionV2BatchUpdateBlobsResponse {}


/// A content digest. A digest for a given blob consists of the size of the blob
/// and its hash. The hash algorithm to use is defined by the server, but servers
/// SHOULD use SHA-256.
/// 
/// The size is considered to be an integral part of the digest and cannot be
/// separated. That is, even if the `hash` field is correctly specified but
/// `size_bytes` is not, the server MUST reject the request.
/// 
/// The reason for including the size in the digest is as follows: in a great
/// many cases, the server needs to know the size of the blob it is about to work
/// with prior to starting an operation with it, such as flattening Merkle tree
/// structures or streaming it to a worker. Technically, the server could
/// implement a separate metadata store, but this results in a significantly more
/// complicated implementation as opposed to having the client specify the size
/// up-front (or storing the size along with the digest in every message where
/// digests are embedded). This does mean that the API leaks some implementation
/// details of (what we consider to be) a reasonable server implementation, but
/// we consider this to be a worthwhile tradeoff.
/// 
/// When a `Digest` is used to refer to a proto message, it always refers to the
/// message in binary encoded form. To ensure consistent hashing, clients and
/// servers MUST ensure that they serialize messages according to the following
/// rules, even if there are alternate valid encodings for the same message:
/// 
/// * Fields are serialized in tag order.
/// * There are no unknown fields.
/// * There are no duplicate fields.
/// * Fields are serialized according to the default semantics for their type.
/// 
/// Most protocol buffer implementations will always follow these rules when
/// serializing, but care should be taken to avoid shortcuts. For instance,
/// concatenating two messages to merge them may produce duplicate fields.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2Digest {
    /// The size of the blob, in bytes.
    #[serde(rename="sizeBytes")]
    pub size_bytes: Option<String>,
    /// The hash. In the case of SHA-256, it will always be a lowercase hex string
    /// exactly 64 characters long.
    pub hash: Option<String>,
}

impl Part for BuildBazelRemoteExecutionV2Digest {}


/// A request message for
/// ContentAddressableStorage.BatchUpdateBlobs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch update blobs](struct.BlobBatchUpdateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2BatchUpdateBlobsRequest {
    /// The individual upload requests.
    pub requests: Option<Vec<BuildBazelRemoteExecutionV2BatchUpdateBlobsRequestRequest>>,
}

impl RequestValue for BuildBazelRemoteExecutionV2BatchUpdateBlobsRequest {}


/// A `ResultsCachePolicy` is used for fine-grained control over how action
/// outputs are stored in the CAS and Action Cache.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2ResultsCachePolicy {
    /// The priority (relative importance) of this content in the overall cache.
    /// Generally, a lower value means a longer retention time or other advantage,
    /// but the interpretation of a given value is server-dependent. A priority of
    /// 0 means a *default* value, decided by the server.
    /// 
    /// The particular semantics of this field is up to the server. In particular,
    /// every server will have their own supported range of priorities, and will
    /// decide how these map into retention/eviction policy.
    pub priority: Option<i32>,
}

impl Part for BuildBazelRemoteExecutionV2ResultsCachePolicy {}


/// An `ExecutionPolicy` can be used to control the scheduling of the action.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2ExecutionPolicy {
    /// The priority (relative importance) of this action. Generally, a lower value
    /// means that the action should be run sooner than actions having a greater
    /// priority value, but the interpretation of a given value is server-
    /// dependent. A priority of 0 means the *default* priority. Priorities may be
    /// positive or negative, and such actions should run later or sooner than
    /// actions having the default priority, respectively. The particular semantics
    /// of this field is up to the server. In particular, every server will have
    /// their own supported range of priorities, and will decide how these map into
    /// scheduling policy.
    pub priority: Option<i32>,
}

impl Part for BuildBazelRemoteExecutionV2ExecutionPolicy {}


/// Capabilities of the remote execution system.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2ExecutionCapabilities {
    /// Supported execution priority range.
    #[serde(rename="executionPriorityCapabilities")]
    pub execution_priority_capabilities: Option<BuildBazelRemoteExecutionV2PriorityCapabilities>,
    /// Whether remote execution is enabled for the particular server/instance.
    #[serde(rename="execEnabled")]
    pub exec_enabled: Option<bool>,
    /// Remote execution may only support a single digest function.
    #[serde(rename="digestFunction")]
    pub digest_function: Option<String>,
}

impl Part for BuildBazelRemoteExecutionV2ExecutionCapabilities {}


/// ExecutedActionMetadata contains details about a completed execution.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildBazelRemoteExecutionV2ExecutedActionMetadata {
    /// When the worker started uploading action outputs.
    #[serde(rename="outputUploadStartTimestamp")]
    pub output_upload_start_timestamp: Option<String>,
    /// When the worker completed the action, including all stages.
    #[serde(rename="workerCompletedTimestamp")]
    pub worker_completed_timestamp: Option<String>,
    /// When was the action added to the queue.
    #[serde(rename="queuedTimestamp")]
    pub queued_timestamp: Option<String>,
    /// The name of the worker which ran the execution.
    pub worker: Option<String>,
    /// When the worker started executing the action command.
    #[serde(rename="executionStartTimestamp")]
    pub execution_start_timestamp: Option<String>,
    /// When the worker completed executing the action command.
    #[serde(rename="executionCompletedTimestamp")]
    pub execution_completed_timestamp: Option<String>,
    /// When the worker received the action.
    #[serde(rename="workerStartTimestamp")]
    pub worker_start_timestamp: Option<String>,
    /// When the worker finished uploading action outputs.
    #[serde(rename="outputUploadCompletedTimestamp")]
    pub output_upload_completed_timestamp: Option<String>,
    /// When the worker started fetching action inputs.
    #[serde(rename="inputFetchStartTimestamp")]
    pub input_fetch_start_timestamp: Option<String>,
    /// When the worker finished fetching action inputs.
    #[serde(rename="inputFetchCompletedTimestamp")]
    pub input_fetch_completed_timestamp: Option<String>,
}

impl Part for BuildBazelRemoteExecutionV2ExecutedActionMetadata {}


/// This resource represents a long-running operation that is the result of a
/// network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [wait execution operations](struct.OperationWaitExecutionCall.html) (response)
/// * [execute actions](struct.ActionExecuteCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningOperation {
    /// The error result of the operation in case of failure or cancellation.
    pub error: Option<GoogleRpcStatus>,
    /// If the value is `false`, it means the operation is still in progress.
    /// If `true`, the operation is completed, and either `error` or `response` is
    /// available.
    pub done: Option<bool>,
    /// The normal response of the operation in case of success.  If the original
    /// method returns no data on success, such as `Delete`, the response is
    /// `google.protobuf.Empty`.  If the original method is standard
    /// `Get`/`Create`/`Update`, the response should be the resource.  For other
    /// methods, the response should have the type `XxxResponse`, where `Xxx`
    /// is the original method name.  For example, if the original method name
    /// is `TakeSnapshot()`, the inferred response type is
    /// `TakeSnapshotResponse`.
    pub response: Option<HashMap<String, String>>,
    /// The server-assigned name, which is only unique within the same service that
    /// originally returns it. If you use the default HTTP mapping, the
    /// `name` should be a resource name ending with `operations/{unique_id}`.
    pub name: Option<String>,
    /// Service-specific metadata associated with the operation.  It typically
    /// contains progress information and common metadata such as create time.
    /// Some services might not provide such metadata.  Any method that returns a
    /// long-running operation should document the metadata type, if any.
    pub metadata: Option<HashMap<String, String>>,
}

impl ResponseResult for GoogleLongrunningOperation {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *operation* resources.
/// It is not used directly, but through the `RemoteBuildExecution` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_remotebuildexecution2 as remotebuildexecution2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use remotebuildexecution2::RemoteBuildExecution;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = RemoteBuildExecution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `wait_execution(...)`
/// // to build up your call.
/// let rb = hub.operations();
/// # }
/// ```
pub struct OperationMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a RemoteBuildExecution<C, A>,
}

impl<'a, C, A> MethodsBuilder for OperationMethods<'a, C, A> {}

impl<'a, C, A> OperationMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Wait for an execution operation to complete. When the client initially
    /// makes the request, the server immediately responds with the current status
    /// of the execution. The server will leave the request stream open until the
    /// operation completes, and then respond with the completed operation. The
    /// server MAY choose to stream additional updates as execution progresses,
    /// such as to provide an update as to the state of the execution.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the Operation
    ///            returned by Execute.
    pub fn wait_execution(&self, request: BuildBazelRemoteExecutionV2WaitExecutionRequest, name: &str) -> OperationWaitExecutionCall<'a, C, A> {
        OperationWaitExecutionCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *actionResult* resources.
/// It is not used directly, but through the `RemoteBuildExecution` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_remotebuildexecution2 as remotebuildexecution2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use remotebuildexecution2::RemoteBuildExecution;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = RemoteBuildExecution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.action_results();
/// # }
/// ```
pub struct ActionResultMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a RemoteBuildExecution<C, A>,
}

impl<'a, C, A> MethodsBuilder for ActionResultMethods<'a, C, A> {}

impl<'a, C, A> ActionResultMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieve a cached execution result.
    /// 
    /// Implementations SHOULD ensure that any blobs referenced from the
    /// ContentAddressableStorage
    /// are available at the time of returning the
    /// ActionResult and will be
    /// for some period of time afterwards. The TTLs of the referenced blobs SHOULD be increased
    /// if necessary and applicable.
    /// 
    /// Errors:
    /// 
    /// * `NOT_FOUND`: The requested `ActionResult` is not in the cache.
    /// 
    /// # Arguments
    ///
    /// * `instanceName` - The instance of the execution system to operate against. A server may
    ///                    support multiple instances of the execution system (with their own workers,
    ///                    storage, caches, etc.). The server MAY require use of this field to select
    ///                    between them in an implementation-defined fashion, otherwise it can be
    ///                    omitted.
    /// * `hash` - The hash. In the case of SHA-256, it will always be a lowercase hex string
    ///            exactly 64 characters long.
    /// * `sizeBytes` - The size of the blob, in bytes.
    pub fn get(&self, instance_name: &str, hash: &str, size_bytes: &str) -> ActionResultGetCall<'a, C, A> {
        ActionResultGetCall {
            hub: self.hub,
            _instance_name: instance_name.to_string(),
            _hash: hash.to_string(),
            _size_bytes: size_bytes.to_string(),
            _inline_stdout: Default::default(),
            _inline_stderr: Default::default(),
            _inline_output_files: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Upload a new execution result.
    /// 
    /// In order to allow the server to perform access control based on the type of
    /// action, and to assist with client debugging, the client MUST first upload
    /// the Action that produced the
    /// result, along with its
    /// Command, into the
    /// `ContentAddressableStorage`.
    /// 
    /// Errors:
    /// 
    /// * `INVALID_ARGUMENT`: One or more arguments are invalid.
    /// * `FAILED_PRECONDITION`: One or more errors occurred in updating the
    ///   action result, such as a missing command or action.
    /// * `RESOURCE_EXHAUSTED`: There is insufficient storage space to add the
    ///   entry to the cache.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `instanceName` - The instance of the execution system to operate against. A server may
    ///                    support multiple instances of the execution system (with their own workers,
    ///                    storage, caches, etc.). The server MAY require use of this field to select
    ///                    between them in an implementation-defined fashion, otherwise it can be
    ///                    omitted.
    /// * `hash` - The hash. In the case of SHA-256, it will always be a lowercase hex string
    ///            exactly 64 characters long.
    /// * `sizeBytes` - The size of the blob, in bytes.
    pub fn update(&self, request: BuildBazelRemoteExecutionV2ActionResult, instance_name: &str, hash: &str, size_bytes: &str) -> ActionResultUpdateCall<'a, C, A> {
        ActionResultUpdateCall {
            hub: self.hub,
            _request: request,
            _instance_name: instance_name.to_string(),
            _hash: hash.to_string(),
            _size_bytes: size_bytes.to_string(),
            _results_cache_policy_priority: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *blob* resources.
/// It is not used directly, but through the `RemoteBuildExecution` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_remotebuildexecution2 as remotebuildexecution2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use remotebuildexecution2::RemoteBuildExecution;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = RemoteBuildExecution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `batch_read(...)`, `batch_update(...)`, `find_missing(...)` and `get_tree(...)`
/// // to build up your call.
/// let rb = hub.blobs();
/// # }
/// ```
pub struct BlobMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a RemoteBuildExecution<C, A>,
}

impl<'a, C, A> MethodsBuilder for BlobMethods<'a, C, A> {}

impl<'a, C, A> BlobMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Determine if blobs are present in the CAS.
    /// 
    /// Clients can use this API before uploading blobs to determine which ones are
    /// already present in the CAS and do not need to be uploaded again.
    /// 
    /// There are no method-specific errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `instanceName` - The instance of the execution system to operate against. A server may
    ///                    support multiple instances of the execution system (with their own workers,
    ///                    storage, caches, etc.). The server MAY require use of this field to select
    ///                    between them in an implementation-defined fashion, otherwise it can be
    ///                    omitted.
    pub fn find_missing(&self, request: BuildBazelRemoteExecutionV2FindMissingBlobsRequest, instance_name: &str) -> BlobFindMissingCall<'a, C, A> {
        BlobFindMissingCall {
            hub: self.hub,
            _request: request,
            _instance_name: instance_name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Upload many blobs at once.
    /// 
    /// The server may enforce a limit of the combined total size of blobs
    /// to be uploaded using this API. This limit may be obtained using the
    /// Capabilities API.
    /// Requests exceeding the limit should either be split into smaller
    /// chunks or uploaded using the
    /// ByteStream API, as appropriate.
    /// 
    /// This request is equivalent to calling a Bytestream `Write` request
    /// on each individual blob, in parallel. The requests may succeed or fail
    /// independently.
    /// 
    /// Errors:
    /// 
    /// * `INVALID_ARGUMENT`: The client attempted to upload more than the
    ///   server supported limit.
    /// 
    /// Individual requests may return the following errors, additionally:
    /// 
    /// * `RESOURCE_EXHAUSTED`: There is insufficient disk quota to store the blob.
    /// * `INVALID_ARGUMENT`: The
    /// Digest does not match the
    /// provided data.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `instanceName` - The instance of the execution system to operate against. A server may
    ///                    support multiple instances of the execution system (with their own workers,
    ///                    storage, caches, etc.). The server MAY require use of this field to select
    ///                    between them in an implementation-defined fashion, otherwise it can be
    ///                    omitted.
    pub fn batch_update(&self, request: BuildBazelRemoteExecutionV2BatchUpdateBlobsRequest, instance_name: &str) -> BlobBatchUpdateCall<'a, C, A> {
        BlobBatchUpdateCall {
            hub: self.hub,
            _request: request,
            _instance_name: instance_name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Download many blobs at once.
    /// 
    /// The server may enforce a limit of the combined total size of blobs
    /// to be downloaded using this API. This limit may be obtained using the
    /// Capabilities API.
    /// Requests exceeding the limit should either be split into smaller
    /// chunks or downloaded using the
    /// ByteStream API, as appropriate.
    /// 
    /// This request is equivalent to calling a Bytestream `Read` request
    /// on each individual blob, in parallel. The requests may succeed or fail
    /// independently.
    /// 
    /// Errors:
    /// 
    /// * `INVALID_ARGUMENT`: The client attempted to read more than the
    ///   server supported limit.
    /// 
    /// Every error on individual read will be returned in the corresponding digest
    /// status.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `instanceName` - The instance of the execution system to operate against. A server may
    ///                    support multiple instances of the execution system (with their own workers,
    ///                    storage, caches, etc.). The server MAY require use of this field to select
    ///                    between them in an implementation-defined fashion, otherwise it can be
    ///                    omitted.
    pub fn batch_read(&self, request: BuildBazelRemoteExecutionV2BatchReadBlobsRequest, instance_name: &str) -> BlobBatchReadCall<'a, C, A> {
        BlobBatchReadCall {
            hub: self.hub,
            _request: request,
            _instance_name: instance_name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetch the entire directory tree rooted at a node.
    /// 
    /// This request must be targeted at a
    /// Directory stored in the
    /// ContentAddressableStorage
    /// (CAS). The server will enumerate the `Directory` tree recursively and
    /// return every node descended from the root.
    /// 
    /// The GetTreeRequest.page_token parameter can be used to skip ahead in
    /// the stream (e.g. when retrying a partially completed and aborted request),
    /// by setting it to a value taken from GetTreeResponse.next_page_token of the
    /// last successfully processed GetTreeResponse).
    /// 
    /// The exact traversal order is unspecified and, unless retrieving subsequent
    /// pages from an earlier request, is not guaranteed to be stable across
    /// multiple invocations of `GetTree`.
    /// 
    /// If part of the tree is missing from the CAS, the server will return the
    /// portion present and omit the rest.
    /// 
    /// * `NOT_FOUND`: The requested tree root is not present in the CAS.
    /// 
    /// # Arguments
    ///
    /// * `instanceName` - The instance of the execution system to operate against. A server may
    ///                    support multiple instances of the execution system (with their own workers,
    ///                    storage, caches, etc.). The server MAY require use of this field to select
    ///                    between them in an implementation-defined fashion, otherwise it can be
    ///                    omitted.
    /// * `hash` - The hash. In the case of SHA-256, it will always be a lowercase hex string
    ///            exactly 64 characters long.
    /// * `sizeBytes` - The size of the blob, in bytes.
    pub fn get_tree(&self, instance_name: &str, hash: &str, size_bytes: &str) -> BlobGetTreeCall<'a, C, A> {
        BlobGetTreeCall {
            hub: self.hub,
            _instance_name: instance_name.to_string(),
            _hash: hash.to_string(),
            _size_bytes: size_bytes.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *action* resources.
/// It is not used directly, but through the `RemoteBuildExecution` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_remotebuildexecution2 as remotebuildexecution2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use remotebuildexecution2::RemoteBuildExecution;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = RemoteBuildExecution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `execute(...)`
/// // to build up your call.
/// let rb = hub.actions();
/// # }
/// ```
pub struct ActionMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a RemoteBuildExecution<C, A>,
}

impl<'a, C, A> MethodsBuilder for ActionMethods<'a, C, A> {}

impl<'a, C, A> ActionMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Execute an action remotely.
    /// 
    /// In order to execute an action, the client must first upload all of the
    /// inputs, the
    /// Command to run, and the
    /// Action into the
    /// ContentAddressableStorage.
    /// It then calls `Execute` with an `action_digest` referring to them. The
    /// server will run the action and eventually return the result.
    /// 
    /// The input `Action`'s fields MUST meet the various canonicalization
    /// requirements specified in the documentation for their types so that it has
    /// the same digest as other logically equivalent `Action`s. The server MAY
    /// enforce the requirements and return errors if a non-canonical input is
    /// received. It MAY also proceed without verifying some or all of the
    /// requirements, such as for performance reasons. If the server does not
    /// verify the requirement, then it will treat the `Action` as distinct from
    /// another logically equivalent action if they hash differently.
    /// 
    /// Returns a stream of
    /// google.longrunning.Operation messages
    /// describing the resulting execution, with eventual `response`
    /// ExecuteResponse. The
    /// `metadata` on the operation is of type
    /// ExecuteOperationMetadata.
    /// 
    /// If the client remains connected after the first response is returned after
    /// the server, then updates are streamed as if the client had called
    /// WaitExecution
    /// until the execution completes or the request reaches an error. The
    /// operation can also be queried using Operations
    /// API.
    /// 
    /// The server NEED NOT implement other methods or functionality of the
    /// Operations API.
    /// 
    /// Errors discovered during creation of the `Operation` will be reported
    /// as gRPC Status errors, while errors that occurred while running the
    /// action will be reported in the `status` field of the `ExecuteResponse`. The
    /// server MUST NOT set the `error` field of the `Operation` proto.
    /// The possible errors include:
    /// 
    /// * `INVALID_ARGUMENT`: One or more arguments are invalid.
    /// * `FAILED_PRECONDITION`: One or more errors occurred in setting up the
    ///   action requested, such as a missing input or command or no worker being
    ///   available. The client may be able to fix the errors and retry.
    /// * `RESOURCE_EXHAUSTED`: There is insufficient quota of some resource to run
    ///   the action.
    /// * `UNAVAILABLE`: Due to a transient condition, such as all workers being
    ///   occupied (and the server does not support a queue), the action could not
    ///   be started. The client should retry.
    /// * `INTERNAL`: An internal error occurred in the execution engine or the
    ///   worker.
    /// * `DEADLINE_EXCEEDED`: The execution timed out.
    /// * `CANCELLED`: The operation was cancelled by the client. This status is
    ///   only possible if the server implements the Operations API CancelOperation
    ///   method, and it was called for the current execution.
    /// 
    /// In the case of a missing input or command, the server SHOULD additionally
    /// send a PreconditionFailure error detail
    /// where, for each requested blob not present in the CAS, there is a
    /// `Violation` with a `type` of `MISSING` and a `subject` of
    /// `"blobs/{hash}/{size}"` indicating the digest of the missing blob.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `instanceName` - The instance of the execution system to operate against. A server may
    ///                    support multiple instances of the execution system (with their own workers,
    ///                    storage, caches, etc.). The server MAY require use of this field to select
    ///                    between them in an implementation-defined fashion, otherwise it can be
    ///                    omitted.
    pub fn execute(&self, request: BuildBazelRemoteExecutionV2ExecuteRequest, instance_name: &str) -> ActionExecuteCall<'a, C, A> {
        ActionExecuteCall {
            hub: self.hub,
            _request: request,
            _instance_name: instance_name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all free methods, which are not associated with a particular resource.
/// It is not used directly, but through the `RemoteBuildExecution` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_remotebuildexecution2 as remotebuildexecution2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use remotebuildexecution2::RemoteBuildExecution;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = RemoteBuildExecution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_capabilities(...)`
/// // to build up your call.
/// let rb = hub.methods();
/// # }
/// ```
pub struct MethodMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a RemoteBuildExecution<C, A>,
}

impl<'a, C, A> MethodsBuilder for MethodMethods<'a, C, A> {}

impl<'a, C, A> MethodMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// GetCapabilities returns the server capabilities configuration of the
    /// remote endpoint.
    /// Only the capabilities of the services supported by the endpoint will
    /// be returned:
    /// * Execution + CAS + Action Cache endpoints should return both
    ///   CacheCapabilities and ExecutionCapabilities.
    /// * Execution only endpoints should return ExecutionCapabilities.
    /// * CAS + Action Cache only endpoints should return CacheCapabilities.
    /// 
    /// # Arguments
    ///
    /// * `instanceName` - The instance of the execution system to operate against. A server may
    ///                    support multiple instances of the execution system (with their own workers,
    ///                    storage, caches, etc.). The server MAY require use of this field to select
    ///                    between them in an implementation-defined fashion, otherwise it can be
    ///                    omitted.
    pub fn get_capabilities(&self, instance_name: &str) -> MethodGetCapabilityCall<'a, C, A> {
        MethodGetCapabilityCall {
            hub: self.hub,
            _instance_name: instance_name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Wait for an execution operation to complete. When the client initially
/// makes the request, the server immediately responds with the current status
/// of the execution. The server will leave the request stream open until the
/// operation completes, and then respond with the completed operation. The
/// server MAY choose to stream additional updates as execution progresses,
/// such as to provide an update as to the state of the execution.
///
/// A builder for the *waitExecution* method supported by a *operation* resource.
/// It is not used directly, but through a `OperationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_remotebuildexecution2 as remotebuildexecution2;
/// use remotebuildexecution2::BuildBazelRemoteExecutionV2WaitExecutionRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use remotebuildexecution2::RemoteBuildExecution;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = RemoteBuildExecution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BuildBazelRemoteExecutionV2WaitExecutionRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.operations().wait_execution(req, "name")
///              .doit();
/// # }
/// ```
pub struct OperationWaitExecutionCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a RemoteBuildExecution<C, A>,
    _request: BuildBazelRemoteExecutionV2WaitExecutionRequest,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OperationWaitExecutionCall<'a, C, A> {}

impl<'a, C, A> OperationWaitExecutionCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GoogleLongrunningOperation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "remotebuildexecution.operations.waitExecution",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/{+name}:waitExecution";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["name"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: BuildBazelRemoteExecutionV2WaitExecutionRequest) -> OperationWaitExecutionCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The name of the Operation
    /// returned by Execute.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> OperationWaitExecutionCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OperationWaitExecutionCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> OperationWaitExecutionCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OperationWaitExecutionCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieve a cached execution result.
/// 
/// Implementations SHOULD ensure that any blobs referenced from the
/// ContentAddressableStorage
/// are available at the time of returning the
/// ActionResult and will be
/// for some period of time afterwards. The TTLs of the referenced blobs SHOULD be increased
/// if necessary and applicable.
/// 
/// Errors:
/// 
/// * `NOT_FOUND`: The requested `ActionResult` is not in the cache.
///
/// A builder for the *get* method supported by a *actionResult* resource.
/// It is not used directly, but through a `ActionResultMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_remotebuildexecution2 as remotebuildexecution2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use remotebuildexecution2::RemoteBuildExecution;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = RemoteBuildExecution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.action_results().get("instanceName", "hash", "sizeBytes")
///              .inline_stdout(true)
///              .inline_stderr(true)
///              .add_inline_output_files("et")
///              .doit();
/// # }
/// ```
pub struct ActionResultGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a RemoteBuildExecution<C, A>,
    _instance_name: String,
    _hash: String,
    _size_bytes: String,
    _inline_stdout: Option<bool>,
    _inline_stderr: Option<bool>,
    _inline_output_files: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ActionResultGetCall<'a, C, A> {}

impl<'a, C, A> ActionResultGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BuildBazelRemoteExecutionV2ActionResult)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "remotebuildexecution.actionResults.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(8 + self._additional_params.len());
        params.push(("instanceName", self._instance_name.to_string()));
        params.push(("hash", self._hash.to_string()));
        params.push(("sizeBytes", self._size_bytes.to_string()));
        if let Some(value) = self._inline_stdout {
            params.push(("inlineStdout", value.to_string()));
        }
        if let Some(value) = self._inline_stderr {
            params.push(("inlineStderr", value.to_string()));
        }
        if self._inline_output_files.len() > 0 {
            for f in self._inline_output_files.iter() {
                params.push(("inlineOutputFiles", f.to_string()));
            }
        }
        for &field in ["alt", "instanceName", "hash", "sizeBytes", "inlineStdout", "inlineStderr", "inlineOutputFiles"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/{+instanceName}/actionResults/{hash}/{sizeBytes}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+instanceName}", "instanceName"), ("{hash}", "hash"), ("{sizeBytes}", "sizeBytes")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["sizeBytes", "hash", "instanceName"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The instance of the execution system to operate against. A server may
    /// support multiple instances of the execution system (with their own workers,
    /// storage, caches, etc.). The server MAY require use of this field to select
    /// between them in an implementation-defined fashion, otherwise it can be
    /// omitted.
    ///
    /// Sets the *instance name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn instance_name(mut self, new_value: &str) -> ActionResultGetCall<'a, C, A> {
        self._instance_name = new_value.to_string();
        self
    }
    /// The hash. In the case of SHA-256, it will always be a lowercase hex string
    /// exactly 64 characters long.
    ///
    /// Sets the *hash* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn hash(mut self, new_value: &str) -> ActionResultGetCall<'a, C, A> {
        self._hash = new_value.to_string();
        self
    }
    /// The size of the blob, in bytes.
    ///
    /// Sets the *size bytes* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn size_bytes(mut self, new_value: &str) -> ActionResultGetCall<'a, C, A> {
        self._size_bytes = new_value.to_string();
        self
    }
    /// A hint to the server to request inlining stdout in the
    /// ActionResult message.
    ///
    /// Sets the *inline stdout* query property to the given value.
    pub fn inline_stdout(mut self, new_value: bool) -> ActionResultGetCall<'a, C, A> {
        self._inline_stdout = Some(new_value);
        self
    }
    /// A hint to the server to request inlining stderr in the
    /// ActionResult message.
    ///
    /// Sets the *inline stderr* query property to the given value.
    pub fn inline_stderr(mut self, new_value: bool) -> ActionResultGetCall<'a, C, A> {
        self._inline_stderr = Some(new_value);
        self
    }
    /// A hint to the server to inline the contents of the listed output files.
    /// Each path needs to exactly match one path in `output_files` in the
    /// Command message.
    ///
    /// Append the given value to the *inline output files* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_inline_output_files(mut self, new_value: &str) -> ActionResultGetCall<'a, C, A> {
        self._inline_output_files.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ActionResultGetCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ActionResultGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ActionResultGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Upload a new execution result.
/// 
/// In order to allow the server to perform access control based on the type of
/// action, and to assist with client debugging, the client MUST first upload
/// the Action that produced the
/// result, along with its
/// Command, into the
/// `ContentAddressableStorage`.
/// 
/// Errors:
/// 
/// * `INVALID_ARGUMENT`: One or more arguments are invalid.
/// * `FAILED_PRECONDITION`: One or more errors occurred in updating the
///   action result, such as a missing command or action.
/// * `RESOURCE_EXHAUSTED`: There is insufficient storage space to add the
///   entry to the cache.
///
/// A builder for the *update* method supported by a *actionResult* resource.
/// It is not used directly, but through a `ActionResultMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_remotebuildexecution2 as remotebuildexecution2;
/// use remotebuildexecution2::BuildBazelRemoteExecutionV2ActionResult;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use remotebuildexecution2::RemoteBuildExecution;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = RemoteBuildExecution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BuildBazelRemoteExecutionV2ActionResult::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.action_results().update(req, "instanceName", "hash", "sizeBytes")
///              .results_cache_policy_priority(-21)
///              .doit();
/// # }
/// ```
pub struct ActionResultUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a RemoteBuildExecution<C, A>,
    _request: BuildBazelRemoteExecutionV2ActionResult,
    _instance_name: String,
    _hash: String,
    _size_bytes: String,
    _results_cache_policy_priority: Option<i32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ActionResultUpdateCall<'a, C, A> {}

impl<'a, C, A> ActionResultUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BuildBazelRemoteExecutionV2ActionResult)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "remotebuildexecution.actionResults.update",
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("instanceName", self._instance_name.to_string()));
        params.push(("hash", self._hash.to_string()));
        params.push(("sizeBytes", self._size_bytes.to_string()));
        if let Some(value) = self._results_cache_policy_priority {
            params.push(("resultsCachePolicy.priority", value.to_string()));
        }
        for &field in ["alt", "instanceName", "hash", "sizeBytes", "resultsCachePolicy.priority"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/{+instanceName}/actionResults/{hash}/{sizeBytes}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+instanceName}", "instanceName"), ("{hash}", "hash"), ("{sizeBytes}", "sizeBytes")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["sizeBytes", "hash", "instanceName"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: BuildBazelRemoteExecutionV2ActionResult) -> ActionResultUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The instance of the execution system to operate against. A server may
    /// support multiple instances of the execution system (with their own workers,
    /// storage, caches, etc.). The server MAY require use of this field to select
    /// between them in an implementation-defined fashion, otherwise it can be
    /// omitted.
    ///
    /// Sets the *instance name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn instance_name(mut self, new_value: &str) -> ActionResultUpdateCall<'a, C, A> {
        self._instance_name = new_value.to_string();
        self
    }
    /// The hash. In the case of SHA-256, it will always be a lowercase hex string
    /// exactly 64 characters long.
    ///
    /// Sets the *hash* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn hash(mut self, new_value: &str) -> ActionResultUpdateCall<'a, C, A> {
        self._hash = new_value.to_string();
        self
    }
    /// The size of the blob, in bytes.
    ///
    /// Sets the *size bytes* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn size_bytes(mut self, new_value: &str) -> ActionResultUpdateCall<'a, C, A> {
        self._size_bytes = new_value.to_string();
        self
    }
    /// The priority (relative importance) of this content in the overall cache.
    /// Generally, a lower value means a longer retention time or other advantage,
    /// but the interpretation of a given value is server-dependent. A priority of
    /// 0 means a *default* value, decided by the server.
    /// 
    /// The particular semantics of this field is up to the server. In particular,
    /// every server will have their own supported range of priorities, and will
    /// decide how these map into retention/eviction policy.
    ///
    /// Sets the *results cache policy.priority* query property to the given value.
    pub fn results_cache_policy_priority(mut self, new_value: i32) -> ActionResultUpdateCall<'a, C, A> {
        self._results_cache_policy_priority = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ActionResultUpdateCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ActionResultUpdateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ActionResultUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Determine if blobs are present in the CAS.
/// 
/// Clients can use this API before uploading blobs to determine which ones are
/// already present in the CAS and do not need to be uploaded again.
/// 
/// There are no method-specific errors.
///
/// A builder for the *findMissing* method supported by a *blob* resource.
/// It is not used directly, but through a `BlobMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_remotebuildexecution2 as remotebuildexecution2;
/// use remotebuildexecution2::BuildBazelRemoteExecutionV2FindMissingBlobsRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use remotebuildexecution2::RemoteBuildExecution;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = RemoteBuildExecution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BuildBazelRemoteExecutionV2FindMissingBlobsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.blobs().find_missing(req, "instanceName")
///              .doit();
/// # }
/// ```
pub struct BlobFindMissingCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a RemoteBuildExecution<C, A>,
    _request: BuildBazelRemoteExecutionV2FindMissingBlobsRequest,
    _instance_name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for BlobFindMissingCall<'a, C, A> {}

impl<'a, C, A> BlobFindMissingCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BuildBazelRemoteExecutionV2FindMissingBlobsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "remotebuildexecution.blobs.findMissing",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("instanceName", self._instance_name.to_string()));
        for &field in ["alt", "instanceName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/{+instanceName}/blobs:findMissing";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+instanceName}", "instanceName")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["instanceName"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: BuildBazelRemoteExecutionV2FindMissingBlobsRequest) -> BlobFindMissingCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The instance of the execution system to operate against. A server may
    /// support multiple instances of the execution system (with their own workers,
    /// storage, caches, etc.). The server MAY require use of this field to select
    /// between them in an implementation-defined fashion, otherwise it can be
    /// omitted.
    ///
    /// Sets the *instance name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn instance_name(mut self, new_value: &str) -> BlobFindMissingCall<'a, C, A> {
        self._instance_name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BlobFindMissingCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> BlobFindMissingCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> BlobFindMissingCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Upload many blobs at once.
/// 
/// The server may enforce a limit of the combined total size of blobs
/// to be uploaded using this API. This limit may be obtained using the
/// Capabilities API.
/// Requests exceeding the limit should either be split into smaller
/// chunks or uploaded using the
/// ByteStream API, as appropriate.
/// 
/// This request is equivalent to calling a Bytestream `Write` request
/// on each individual blob, in parallel. The requests may succeed or fail
/// independently.
/// 
/// Errors:
/// 
/// * `INVALID_ARGUMENT`: The client attempted to upload more than the
///   server supported limit.
/// 
/// Individual requests may return the following errors, additionally:
/// 
/// * `RESOURCE_EXHAUSTED`: There is insufficient disk quota to store the blob.
/// * `INVALID_ARGUMENT`: The
/// Digest does not match the
/// provided data.
///
/// A builder for the *batchUpdate* method supported by a *blob* resource.
/// It is not used directly, but through a `BlobMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_remotebuildexecution2 as remotebuildexecution2;
/// use remotebuildexecution2::BuildBazelRemoteExecutionV2BatchUpdateBlobsRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use remotebuildexecution2::RemoteBuildExecution;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = RemoteBuildExecution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BuildBazelRemoteExecutionV2BatchUpdateBlobsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.blobs().batch_update(req, "instanceName")
///              .doit();
/// # }
/// ```
pub struct BlobBatchUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a RemoteBuildExecution<C, A>,
    _request: BuildBazelRemoteExecutionV2BatchUpdateBlobsRequest,
    _instance_name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for BlobBatchUpdateCall<'a, C, A> {}

impl<'a, C, A> BlobBatchUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BuildBazelRemoteExecutionV2BatchUpdateBlobsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "remotebuildexecution.blobs.batchUpdate",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("instanceName", self._instance_name.to_string()));
        for &field in ["alt", "instanceName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/{+instanceName}/blobs:batchUpdate";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+instanceName}", "instanceName")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["instanceName"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: BuildBazelRemoteExecutionV2BatchUpdateBlobsRequest) -> BlobBatchUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The instance of the execution system to operate against. A server may
    /// support multiple instances of the execution system (with their own workers,
    /// storage, caches, etc.). The server MAY require use of this field to select
    /// between them in an implementation-defined fashion, otherwise it can be
    /// omitted.
    ///
    /// Sets the *instance name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn instance_name(mut self, new_value: &str) -> BlobBatchUpdateCall<'a, C, A> {
        self._instance_name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BlobBatchUpdateCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> BlobBatchUpdateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> BlobBatchUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Download many blobs at once.
/// 
/// The server may enforce a limit of the combined total size of blobs
/// to be downloaded using this API. This limit may be obtained using the
/// Capabilities API.
/// Requests exceeding the limit should either be split into smaller
/// chunks or downloaded using the
/// ByteStream API, as appropriate.
/// 
/// This request is equivalent to calling a Bytestream `Read` request
/// on each individual blob, in parallel. The requests may succeed or fail
/// independently.
/// 
/// Errors:
/// 
/// * `INVALID_ARGUMENT`: The client attempted to read more than the
///   server supported limit.
/// 
/// Every error on individual read will be returned in the corresponding digest
/// status.
///
/// A builder for the *batchRead* method supported by a *blob* resource.
/// It is not used directly, but through a `BlobMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_remotebuildexecution2 as remotebuildexecution2;
/// use remotebuildexecution2::BuildBazelRemoteExecutionV2BatchReadBlobsRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use remotebuildexecution2::RemoteBuildExecution;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = RemoteBuildExecution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BuildBazelRemoteExecutionV2BatchReadBlobsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.blobs().batch_read(req, "instanceName")
///              .doit();
/// # }
/// ```
pub struct BlobBatchReadCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a RemoteBuildExecution<C, A>,
    _request: BuildBazelRemoteExecutionV2BatchReadBlobsRequest,
    _instance_name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for BlobBatchReadCall<'a, C, A> {}

impl<'a, C, A> BlobBatchReadCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BuildBazelRemoteExecutionV2BatchReadBlobsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "remotebuildexecution.blobs.batchRead",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("instanceName", self._instance_name.to_string()));
        for &field in ["alt", "instanceName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/{+instanceName}/blobs:batchRead";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+instanceName}", "instanceName")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["instanceName"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: BuildBazelRemoteExecutionV2BatchReadBlobsRequest) -> BlobBatchReadCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The instance of the execution system to operate against. A server may
    /// support multiple instances of the execution system (with their own workers,
    /// storage, caches, etc.). The server MAY require use of this field to select
    /// between them in an implementation-defined fashion, otherwise it can be
    /// omitted.
    ///
    /// Sets the *instance name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn instance_name(mut self, new_value: &str) -> BlobBatchReadCall<'a, C, A> {
        self._instance_name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BlobBatchReadCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> BlobBatchReadCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> BlobBatchReadCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Fetch the entire directory tree rooted at a node.
/// 
/// This request must be targeted at a
/// Directory stored in the
/// ContentAddressableStorage
/// (CAS). The server will enumerate the `Directory` tree recursively and
/// return every node descended from the root.
/// 
/// The GetTreeRequest.page_token parameter can be used to skip ahead in
/// the stream (e.g. when retrying a partially completed and aborted request),
/// by setting it to a value taken from GetTreeResponse.next_page_token of the
/// last successfully processed GetTreeResponse).
/// 
/// The exact traversal order is unspecified and, unless retrieving subsequent
/// pages from an earlier request, is not guaranteed to be stable across
/// multiple invocations of `GetTree`.
/// 
/// If part of the tree is missing from the CAS, the server will return the
/// portion present and omit the rest.
/// 
/// * `NOT_FOUND`: The requested tree root is not present in the CAS.
///
/// A builder for the *getTree* method supported by a *blob* resource.
/// It is not used directly, but through a `BlobMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_remotebuildexecution2 as remotebuildexecution2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use remotebuildexecution2::RemoteBuildExecution;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = RemoteBuildExecution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.blobs().get_tree("instanceName", "hash", "sizeBytes")
///              .page_token("sadipscing")
///              .page_size(-48)
///              .doit();
/// # }
/// ```
pub struct BlobGetTreeCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a RemoteBuildExecution<C, A>,
    _instance_name: String,
    _hash: String,
    _size_bytes: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for BlobGetTreeCall<'a, C, A> {}

impl<'a, C, A> BlobGetTreeCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BuildBazelRemoteExecutionV2GetTreeResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "remotebuildexecution.blobs.getTree",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("instanceName", self._instance_name.to_string()));
        params.push(("hash", self._hash.to_string()));
        params.push(("sizeBytes", self._size_bytes.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        for &field in ["alt", "instanceName", "hash", "sizeBytes", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/{+instanceName}/blobs/{hash}/{sizeBytes}:getTree";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+instanceName}", "instanceName"), ("{hash}", "hash"), ("{sizeBytes}", "sizeBytes")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["sizeBytes", "hash", "instanceName"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The instance of the execution system to operate against. A server may
    /// support multiple instances of the execution system (with their own workers,
    /// storage, caches, etc.). The server MAY require use of this field to select
    /// between them in an implementation-defined fashion, otherwise it can be
    /// omitted.
    ///
    /// Sets the *instance name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn instance_name(mut self, new_value: &str) -> BlobGetTreeCall<'a, C, A> {
        self._instance_name = new_value.to_string();
        self
    }
    /// The hash. In the case of SHA-256, it will always be a lowercase hex string
    /// exactly 64 characters long.
    ///
    /// Sets the *hash* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn hash(mut self, new_value: &str) -> BlobGetTreeCall<'a, C, A> {
        self._hash = new_value.to_string();
        self
    }
    /// The size of the blob, in bytes.
    ///
    /// Sets the *size bytes* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn size_bytes(mut self, new_value: &str) -> BlobGetTreeCall<'a, C, A> {
        self._size_bytes = new_value.to_string();
        self
    }
    /// A page token, which must be a value received in a previous
    /// GetTreeResponse.
    /// If present, the server will use it to return the following page of results.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> BlobGetTreeCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// A maximum page size to request. If present, the server will request no more
    /// than this many items. Regardless of whether a page size is specified, the
    /// server may place its own limit on the number of items to be returned and
    /// require the client to retrieve more items using a subsequent request.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> BlobGetTreeCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BlobGetTreeCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> BlobGetTreeCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> BlobGetTreeCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Execute an action remotely.
/// 
/// In order to execute an action, the client must first upload all of the
/// inputs, the
/// Command to run, and the
/// Action into the
/// ContentAddressableStorage.
/// It then calls `Execute` with an `action_digest` referring to them. The
/// server will run the action and eventually return the result.
/// 
/// The input `Action`'s fields MUST meet the various canonicalization
/// requirements specified in the documentation for their types so that it has
/// the same digest as other logically equivalent `Action`s. The server MAY
/// enforce the requirements and return errors if a non-canonical input is
/// received. It MAY also proceed without verifying some or all of the
/// requirements, such as for performance reasons. If the server does not
/// verify the requirement, then it will treat the `Action` as distinct from
/// another logically equivalent action if they hash differently.
/// 
/// Returns a stream of
/// google.longrunning.Operation messages
/// describing the resulting execution, with eventual `response`
/// ExecuteResponse. The
/// `metadata` on the operation is of type
/// ExecuteOperationMetadata.
/// 
/// If the client remains connected after the first response is returned after
/// the server, then updates are streamed as if the client had called
/// WaitExecution
/// until the execution completes or the request reaches an error. The
/// operation can also be queried using Operations
/// API.
/// 
/// The server NEED NOT implement other methods or functionality of the
/// Operations API.
/// 
/// Errors discovered during creation of the `Operation` will be reported
/// as gRPC Status errors, while errors that occurred while running the
/// action will be reported in the `status` field of the `ExecuteResponse`. The
/// server MUST NOT set the `error` field of the `Operation` proto.
/// The possible errors include:
/// 
/// * `INVALID_ARGUMENT`: One or more arguments are invalid.
/// * `FAILED_PRECONDITION`: One or more errors occurred in setting up the
///   action requested, such as a missing input or command or no worker being
///   available. The client may be able to fix the errors and retry.
/// * `RESOURCE_EXHAUSTED`: There is insufficient quota of some resource to run
///   the action.
/// * `UNAVAILABLE`: Due to a transient condition, such as all workers being
///   occupied (and the server does not support a queue), the action could not
///   be started. The client should retry.
/// * `INTERNAL`: An internal error occurred in the execution engine or the
///   worker.
/// * `DEADLINE_EXCEEDED`: The execution timed out.
/// * `CANCELLED`: The operation was cancelled by the client. This status is
///   only possible if the server implements the Operations API CancelOperation
///   method, and it was called for the current execution.
/// 
/// In the case of a missing input or command, the server SHOULD additionally
/// send a PreconditionFailure error detail
/// where, for each requested blob not present in the CAS, there is a
/// `Violation` with a `type` of `MISSING` and a `subject` of
/// `"blobs/{hash}/{size}"` indicating the digest of the missing blob.
///
/// A builder for the *execute* method supported by a *action* resource.
/// It is not used directly, but through a `ActionMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_remotebuildexecution2 as remotebuildexecution2;
/// use remotebuildexecution2::BuildBazelRemoteExecutionV2ExecuteRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use remotebuildexecution2::RemoteBuildExecution;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = RemoteBuildExecution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BuildBazelRemoteExecutionV2ExecuteRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.actions().execute(req, "instanceName")
///              .doit();
/// # }
/// ```
pub struct ActionExecuteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a RemoteBuildExecution<C, A>,
    _request: BuildBazelRemoteExecutionV2ExecuteRequest,
    _instance_name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ActionExecuteCall<'a, C, A> {}

impl<'a, C, A> ActionExecuteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GoogleLongrunningOperation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "remotebuildexecution.actions.execute",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("instanceName", self._instance_name.to_string()));
        for &field in ["alt", "instanceName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/{+instanceName}/actions:execute";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+instanceName}", "instanceName")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["instanceName"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: BuildBazelRemoteExecutionV2ExecuteRequest) -> ActionExecuteCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The instance of the execution system to operate against. A server may
    /// support multiple instances of the execution system (with their own workers,
    /// storage, caches, etc.). The server MAY require use of this field to select
    /// between them in an implementation-defined fashion, otherwise it can be
    /// omitted.
    ///
    /// Sets the *instance name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn instance_name(mut self, new_value: &str) -> ActionExecuteCall<'a, C, A> {
        self._instance_name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ActionExecuteCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ActionExecuteCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ActionExecuteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// GetCapabilities returns the server capabilities configuration of the
/// remote endpoint.
/// Only the capabilities of the services supported by the endpoint will
/// be returned:
/// * Execution + CAS + Action Cache endpoints should return both
///   CacheCapabilities and ExecutionCapabilities.
/// * Execution only endpoints should return ExecutionCapabilities.
/// * CAS + Action Cache only endpoints should return CacheCapabilities.
///
/// A builder for the *getCapabilities* method.
/// It is not used directly, but through a `MethodMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_remotebuildexecution2 as remotebuildexecution2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use remotebuildexecution2::RemoteBuildExecution;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = RemoteBuildExecution::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().get_capabilities("instanceName")
///              .doit();
/// # }
/// ```
pub struct MethodGetCapabilityCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a RemoteBuildExecution<C, A>,
    _instance_name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MethodGetCapabilityCall<'a, C, A> {}

impl<'a, C, A> MethodGetCapabilityCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BuildBazelRemoteExecutionV2ServerCapabilities)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "remotebuildexecution.getCapabilities",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("instanceName", self._instance_name.to_string()));
        for &field in ["alt", "instanceName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/{+instanceName}/capabilities";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+instanceName}", "instanceName")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["instanceName"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The instance of the execution system to operate against. A server may
    /// support multiple instances of the execution system (with their own workers,
    /// storage, caches, etc.). The server MAY require use of this field to select
    /// between them in an implementation-defined fashion, otherwise it can be
    /// omitted.
    ///
    /// Sets the *instance name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn instance_name(mut self, new_value: &str) -> MethodGetCapabilityCall<'a, C, A> {
        self._instance_name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MethodGetCapabilityCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> MethodGetCapabilityCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> MethodGetCapabilityCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


