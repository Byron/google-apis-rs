// DO NOT EDIT !
// This file was generated automatically from 'src/mako/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *datastore* crate version *0.1.0+20140916*, where *20140916* is the exact revision of the *datastore:v1beta2* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.0*.
//! 
//! Everything else about the *datastore* *v1_beta2* API can be found at the
//! [official documentation site](https://developers.google.com/datastore/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/datastore1_beta2).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Datastore.html) ... 
//! 
//! * datasets
//!  * [*allocate ids*](struct.DatasetAllocateIdCall.html), [*begin transaction*](struct.DatasetBeginTransactionCall.html), [*commit*](struct.DatasetCommitCall.html), [*lookup*](struct.DatasetLookupCall.html), [*rollback*](struct.DatasetRollbackCall.html) and [*run query*](struct.DatasetRunQueryCall.html)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](../index.html).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Datastore.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
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
//! let r = hub.datasets().lookup(...).doit()
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
//! google-datastore1_beta2 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate "yup-oauth2" as oauth2;
//! extern crate "google-datastore1_beta2" as datastore1_beta2;
//! use datastore1_beta2::LookupRequest;
//! use datastore1_beta2::Result;
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use datastore1_beta2::Datastore;
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
//!                               hyper::Client::new(),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = Datastore::new(hyper::Client::new(), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req: LookupRequest = Default::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.datasets().lookup(&req, "datasetId")
//!              .doit();
//! 
//! match result {
//!     Result::HttpError(err) => println!("HTTPERROR: {:?}", err),
//!     Result::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
//!     Result::MissingToken => println!("OAuth2: Missing Token"),
//!     Result::Cancelled => println!("Operation cancelled by user"),
//!     Result::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
//!     Result::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
//!     Result::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
//!     Result::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
//!     Result::Success(_) => println!("Success (value doesn't print)"),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](../yup-oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downlods
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
//! [decodable](trait.ResponseResult.html) via json. Optionals are used to indicate that partial requests are responses are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifyable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are borrowed
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 
#![feature(core,io,thread_sleep)]
// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any 
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]
// Required for serde annotations
#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]

#[macro_use]
extern crate hyper;
extern crate serde;
extern crate "yup-oauth2" as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use serde::json;
use std::io;
use std::fs;
use std::thread::sleep;

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, CallBuilder, Hub, ReadSeek, Part, ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, ResourceMethodsBuilder, Resource, JsonServerError};


// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// View your email address
    UserinfoEmail,

    /// View and manage your Google Cloud Datastore data
    Full,

    /// View and manage your data across Google Cloud Platform services
    CloudPlatform,
}

impl Str for Scope {
    fn as_slice(&self) -> &str {
        match *self {
            Scope::UserinfoEmail => "https://www.googleapis.com/auth/userinfo.email",
            Scope::Full => "https://www.googleapis.com/auth/datastore",
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Full
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Datastore related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-datastore1_beta2" as datastore1_beta2;
/// use datastore1_beta2::LookupRequest;
/// use datastore1_beta2::Result;
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use datastore1_beta2::Datastore;
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
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Datastore::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: LookupRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.datasets().lookup(&req, "datasetId")
///              .doit();
/// 
/// match result {
///     Result::HttpError(err) => println!("HTTPERROR: {:?}", err),
///     Result::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
///     Result::MissingToken => println!("OAuth2: Missing Token"),
///     Result::Cancelled => println!("Operation cancelled by user"),
///     Result::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
///     Result::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
///     Result::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
///     Result::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
///     Result::Success(_) => println!("Success (value doesn't print)"),
/// }
/// # }
/// ```
pub struct Datastore<C, NC, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,

    _m: PhantomData<NC>
}

impl<'a, C, NC, A> Hub for Datastore<C, NC, A> {}

impl<'a, C, NC, A> Datastore<C, NC, A>
    where  NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Datastore<C, NC, A> {
        Datastore {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/0.1.0".to_string(),
            _m: PhantomData
        }
    }

    pub fn datasets(&'a self) -> DatasetMethods<'a, C, NC, A> {
        DatasetMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/0.1.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        let prev = self._user_agent.clone();
        self._user_agent = agent_name;
        prev
    }
}


// ############
// SCHEMAS ###
// ##########
/// An identifier for a particular subset of entities.
/// 
/// Entities are partitioned into various subsets, each used by different datasets and different namespaces within a dataset and so forth.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PartitionId {
    /// The namespace.    
    pub namespace: String,
    /// The dataset ID.    
    #[serde(alias="datasetId")]
    pub dataset_id: String,
}

impl Part for PartitionId {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [allocate ids datasets](struct.DatasetAllocateIdCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct AllocateIdsResponse {
    /// The keys specified in the request (in the same order), each with its key path completed with a newly allocated ID.    
    pub keys: Vec<Key>,
    /// no description provided    
    pub header: ResponseHeader,
}

impl ResponseResult for AllocateIdsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [lookup datasets](struct.DatasetLookupCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct LookupRequest {
    /// Keys of entities to look up from the datastore.    
    pub keys: Option<Vec<Key>>,
    /// Options for this lookup request. Optional.    
    #[serde(alias="readOptions")]
    pub read_options: Option<ReadOptions>,
}

impl RequestValue for LookupRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [begin transaction datasets](struct.DatasetBeginTransactionCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct BeginTransactionResponse {
    /// no description provided    
    pub header: ResponseHeader,
    /// The transaction identifier (always present).    
    pub transaction: String,
}

impl ResponseResult for BeginTransactionResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [allocate ids datasets](struct.DatasetAllocateIdCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct AllocateIdsRequest {
    /// A list of keys with incomplete key paths to allocate IDs for. No key may be reserved/read-only.    
    pub keys: Option<Vec<Key>>,
}

impl RequestValue for AllocateIdsRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [run query datasets](struct.DatasetRunQueryCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct RunQueryResponse {
    /// no description provided    
    pub header: ResponseHeader,
    /// A batch of query results (always present).    
    pub batch: QueryResultBatch,
}

impl ResponseResult for RunQueryResponse {}


/// A set of changes to apply.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct Mutation {
    /// Entities to insert. Each inserted entity's key must have a complete path and must not be reserved/read-only.    
    pub insert: Vec<Entity>,
    /// Ignore a user specified read-only period. Optional.    
    pub force: bool,
    /// Insert entities with a newly allocated ID. Each inserted entity's key must omit the final identifier in its path and must not be reserved/read-only.    
    #[serde(alias="insertAutoId")]
    pub insert_auto_id: Vec<Entity>,
    /// Keys of entities to delete. Each key must have a complete key path and must not be reserved/read-only.    
    pub delete: Vec<Key>,
    /// Entities to update. Each updated entity's key must have a complete path and must not be reserved/read-only.    
    pub update: Vec<Entity>,
    /// Entities to upsert. Each upserted entity's key must have a complete path and must not be reserved/read-only.    
    pub upsert: Vec<Entity>,
}

impl Part for Mutation {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [commit datasets](struct.DatasetCommitCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct CommitResponse {
    /// no description provided    
    pub header: ResponseHeader,
    /// The result of performing the mutation (if any).    
    #[serde(alias="mutationResult")]
    pub mutation_result: MutationResult,
}

impl ResponseResult for CommitResponse {}


/// A batch of results produced by a query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct QueryResultBatch {
    /// The state of the query after the current batch. One of notFinished, moreResultsAfterLimit, noMoreResults.    
    #[serde(alias="moreResults")]
    pub more_results: String,
    /// The number of results skipped because of Query.offset.    
    #[serde(alias="skippedResults")]
    pub skipped_results: i32,
    /// A cursor that points to the position after the last result in the batch. May be absent. TODO(arfuller): Once all plans produce cursors update documentation here.    
    #[serde(alias="endCursor")]
    pub end_cursor: String,
    /// The results for this batch.    
    #[serde(alias="entityResults")]
    pub entity_results: Vec<EntityResult>,
    /// The result type for every entity in entityResults. full for full entities, projection for entities with only projected properties, keyOnly for entities with only a key.    
    #[serde(alias="entityResultType")]
    pub entity_result_type: String,
}

impl Part for QueryResultBatch {}


/// A reference to a property relative to the kind expressions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct PropertyReference {
    /// The name of the property.    
    pub name: String,
}

impl Part for PropertyReference {}


/// The result of fetching an entity from the datastore.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct EntityResult {
    /// The resulting entity.    
    pub entity: Entity,
}

impl Part for EntityResult {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [rollback datasets](struct.DatasetRollbackCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct RollbackResponse {
    /// no description provided    
    pub header: ResponseHeader,
}

impl ResponseResult for RollbackResponse {}


/// A GQL query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct GqlQuery {
    /// When false, the query string must not contain a literal.    
    #[serde(alias="allowLiteral")]
    pub allow_literal: bool,
    /// A named argument must set field GqlQueryArg.name. No two named arguments may have the same name. For each non-reserved named binding site in the query string, there must be a named argument with that name, but not necessarily the inverse.    
    #[serde(alias="nameArgs")]
    pub name_args: Vec<GqlQueryArg>,
    /// The query string.    
    #[serde(alias="queryString")]
    pub query_string: String,
    /// Numbered binding site @1 references the first numbered argument, effectively using 1-based indexing, rather than the usual 0. A numbered argument must NOT set field GqlQueryArg.name. For each binding site numbered i in query_string, there must be an ith numbered argument. The inverse must also be true.    
    #[serde(alias="numberArgs")]
    pub number_args: Vec<GqlQueryArg>,
}

impl Part for GqlQuery {}


/// A unique identifier for an entity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Key {
    /// The entity path. An entity path consists of one or more elements composed of a kind and a string or numerical identifier, which identify entities. The first element identifies a root entity, the second element identifies a child of the root entity, the third element a child of the second entity, and so forth. The entities identified by all prefixes of the path are called the element's ancestors. An entity path is always fully complete: ALL of the entity's ancestors are required to be in the path along with the entity identifier itself. The only exception is that in some documented cases, the identifier in the last path element (for the entity) itself may be omitted. A path can never be empty. The path can have at most 100 elements.    
    pub path: Vec<KeyPathElement>,
    /// Entities are partitioned into subsets, currently identified by a dataset (usually implicitly specified by the project) and namespace ID. Queries are scoped to a single partition.    
    #[serde(alias="partitionId")]
    pub partition_id: PartitionId,
}

impl Part for Key {}


/// A representation of a property in a projection.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct PropertyExpression {
    /// The aggregation function to apply to the property. Optional. Can only be used when grouping by at least one property. Must then be set on all properties in the projection that are not being grouped by. Aggregation functions: first selects the first result as determined by the query's order.    
    #[serde(alias="aggregationFunction")]
    pub aggregation_function: String,
    /// The property to project.    
    pub property: PropertyReference,
}

impl Part for PropertyExpression {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct MutationResult {
    /// Keys for insertAutoId entities. One per entity from the request, in the same order.    
    #[serde(alias="insertAutoIdKeys")]
    pub insert_auto_id_keys: Vec<Key>,
    /// Number of index writes.    
    #[serde(alias="indexUpdates")]
    pub index_updates: i32,
}

impl Part for MutationResult {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct ReadOptions {
    /// The transaction to use. Optional.    
    pub transaction: String,
    /// The read consistency to use. One of default, strong, or eventual. Cannot be set when transaction is set. Lookup and ancestor queries default to strong, global queries default to eventual and cannot be set to strong. Optional. Default is default.    
    #[serde(alias="readConsistency")]
    pub read_consistency: String,
}

impl Part for ReadOptions {}


/// A filter on a specific property.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct PropertyFilter {
    /// The operator to filter by. One of lessThan, lessThanOrEqual, greaterThan, greaterThanOrEqual, equal, or hasAncestor.    
    pub operator: String,
    /// The property to filter by.    
    pub property: PropertyReference,
    /// The value to compare the property to.    
    pub value: Value,
}

impl Part for PropertyFilter {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ResponseHeader {
    /// Identifies what kind of resource this is. Value: the fixed string "datastore#responseHeader".    
    pub kind: String,
}

impl Part for ResponseHeader {}


/// A message that can hold any of the supported value types and associated metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Value {
    /// An entity value. May have no key. May have a key with an incomplete key path. May have a reserved/read-only key.    
    #[serde(alias="entityValue")]
    pub entity_value: Entity,
    /// A double value.    
    #[serde(alias="doubleValue")]
    pub double_value: f64,
    /// A blob key value.    
    #[serde(alias="blobKeyValue")]
    pub blob_key_value: String,
    /// The meaning field is reserved and should not be used.    
    pub meaning: i32,
    /// A timestamp value.    
    #[serde(alias="dateTimeValue")]
    pub date_time_value: String,
    /// A key value.    
    #[serde(alias="keyValue")]
    pub key_value: Key,
    /// A blob value. May be a maximum of 1,000,000 bytes.    
    #[serde(alias="blobValue")]
    pub blob_value: String,
    /// If the value should be indexed.
    /// 
    /// The indexed property may be set for a null value. When indexed is true, stringValue is limited to 500 characters and the blob value is limited to 500 bytes. Input values by default have indexed set to true; however, you can explicitly set indexed to true if you want. (An output value never has indexed explicitly set to true.) If a value is itself an entity, it cannot have indexed set to true.
    pub indexed: bool,
    /// A UTF-8 encoded string value.    
    #[serde(alias="stringValue")]
    pub string_value: String,
    /// A list value. Cannot contain another list value. Cannot also have a meaning and indexing set.    
    #[serde(alias="listValue")]
    pub list_value: Vec<Value>,
    /// A boolean value.    
    #[serde(alias="booleanValue")]
    pub boolean_value: bool,
    /// An integer value.    
    #[serde(alias="integerValue")]
    pub integer_value: String,
}

impl Part for Value {}


/// A holder for any type of filter. Exactly one field should be specified.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct Filter {
    /// A composite filter.    
    #[serde(alias="compositeFilter")]
    pub composite_filter: CompositeFilter,
    /// A filter on a property.    
    #[serde(alias="propertyFilter")]
    pub property_filter: PropertyFilter,
}

impl Part for Filter {}


/// A (kind, ID/name) pair used to construct a key path.
/// 
/// At most one of name or ID may be set. If either is set, the element is complete. If neither is set, the element is incomplete.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KeyPathElement {
    /// The kind of the entity. A kind matching regex "__.*__" is reserved/read-only. A kind must not contain more than 500 characters. Cannot be "".    
    pub kind: String,
    /// The ID of the entity. Never equal to zero. Values less than zero are discouraged and will not be supported in the future.    
    pub id: String,
    /// The name of the entity. A name matching regex "__.*__" is reserved/read-only. A name must not be more than 500 characters. Cannot be "".    
    pub name: String,
}

impl Part for KeyPathElement {}


/// An entity property.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Property {
    /// An entity value. May have no key. May have a key with an incomplete key path. May have a reserved/read-only key.    
    #[serde(alias="entityValue")]
    pub entity_value: Entity,
    /// A double value.    
    #[serde(alias="doubleValue")]
    pub double_value: f64,
    /// A blob key value.    
    #[serde(alias="blobKeyValue")]
    pub blob_key_value: String,
    /// The meaning field is reserved and should not be used.    
    pub meaning: i32,
    /// A timestamp value.    
    #[serde(alias="dateTimeValue")]
    pub date_time_value: String,
    /// A key value.    
    #[serde(alias="keyValue")]
    pub key_value: Key,
    /// A blob value. May be a maximum of 1,000,000 bytes.    
    #[serde(alias="blobValue")]
    pub blob_value: String,
    /// If the value should be indexed.
    /// 
    /// The indexed property may be set for a null value. When indexed is true, stringValue is limited to 500 characters and the blob value is limited to 500 bytes. Input values by default have indexed set to true; however, you can explicitly set indexed to true if you want. (An output value never has indexed explicitly set to true.) If a value is itself an entity, it cannot have indexed set to true.
    pub indexed: bool,
    /// A UTF-8 encoded string value.    
    #[serde(alias="stringValue")]
    pub string_value: String,
    /// A list value. Cannot contain another list value. Cannot also have a meaning and indexing set.    
    #[serde(alias="listValue")]
    pub list_value: Vec<Value>,
    /// A boolean value.    
    #[serde(alias="booleanValue")]
    pub boolean_value: bool,
    /// An integer value.    
    #[serde(alias="integerValue")]
    pub integer_value: String,
}

impl Part for Property {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [lookup datasets](struct.DatasetLookupCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct LookupResponse {
    /// Entities found.    
    pub found: Vec<EntityResult>,
    /// no description provided    
    pub header: ResponseHeader,
    /// A list of keys that were not looked up due to resource constraints.    
    pub deferred: Vec<Key>,
    /// Entities not found, with only the key populated.    
    pub missing: Vec<EntityResult>,
}

impl ResponseResult for LookupResponse {}


/// A filter that merges the multiple other filters using the given operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct CompositeFilter {
    /// The operator for combining multiple filters. Only "and" is currently supported.    
    pub operator: String,
    /// The list of filters to combine. Must contain at least one filter.    
    pub filters: Vec<Filter>,
}

impl Part for CompositeFilter {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [commit datasets](struct.DatasetCommitCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct CommitRequest {
    /// no description provided    
    #[serde(alias="ignoreReadOnly")]
    pub ignore_read_only: Option<bool>,
    /// The transaction identifier, returned by a call to beginTransaction. Must be set when mode is TRANSACTIONAL.    
    pub transaction: Option<String>,
    /// The type of commit to perform. Either TRANSACTIONAL or NON_TRANSACTIONAL.    
    pub mode: Option<String>,
    /// The mutation to perform. Optional.    
    pub mutation: Option<Mutation>,
}

impl RequestValue for CommitRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [run query datasets](struct.DatasetRunQueryCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct RunQueryRequest {
    /// The query to run. Either this field or field gql_query must be set, but not both.    
    pub query: Option<Query>,
    /// Entities are partitioned into subsets, identified by a dataset (usually implicitly specified by the project) and namespace ID. Queries are scoped to a single partition. This partition ID is normalized with the standard default context partition ID, but all other partition IDs in RunQueryRequest are normalized with this partition ID as the context partition ID.    
    #[serde(alias="partitionId")]
    pub partition_id: Option<PartitionId>,
    /// The GQL query to run. Either this field or field query must be set, but not both.    
    #[serde(alias="gqlQuery")]
    pub gql_query: Option<GqlQuery>,
    /// The options for this query.    
    #[serde(alias="readOptions")]
    pub read_options: Option<ReadOptions>,
}

impl RequestValue for RunQueryRequest {}


/// A representation of a kind.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct KindExpression {
    /// The name of the kind.    
    pub name: String,
}

impl Part for KindExpression {}


/// A binding argument for a GQL query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct GqlQueryArg {
    /// no description provided    
    pub cursor: String,
    /// Must match regex "[A-Za-z_$][A-Za-z_$0-9]*". Must not match regex "__.*__". Must not be "".    
    pub name: String,
    /// no description provided    
    pub value: Value,
}

impl Part for GqlQueryArg {}


/// An entity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    /// The entity's properties.    
    pub properties: HashMap<String, Property>,
    /// The entity's key.
    /// 
    /// An entity must have a key, unless otherwise documented (for example, an entity in Value.entityValue may have no key). An entity's kind is its key's path's last element's kind, or null if it has no key.
    pub key: Key,
}

impl Part for Entity {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [rollback datasets](struct.DatasetRollbackCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct RollbackRequest {
    /// The transaction identifier, returned by a call to beginTransaction.    
    pub transaction: Option<String>,
}

impl RequestValue for RollbackRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [begin transaction datasets](struct.DatasetBeginTransactionCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct BeginTransactionRequest {
    /// The transaction isolation level. Either snapshot or serializable. The default isolation level is snapshot isolation, which means that another transaction may not concurrently modify the data that is modified by this transaction. Optionally, a transaction can request to be made serializable which means that another transaction cannot concurrently modify the data that is read or modified by this transaction.    
    #[serde(alias="isolationLevel")]
    pub isolation_level: Option<String>,
}

impl RequestValue for BeginTransactionRequest {}


/// The desired order for a specific property.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct PropertyOrder {
    /// The direction to order by. One of ascending or descending. Optional, defaults to ascending.    
    pub direction: String,
    /// The property to order by.    
    pub property: PropertyReference,
}

impl Part for PropertyOrder {}


/// A query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct Query {
    /// A starting point for the query results. Optional. Query cursors are returned in query result batches.    
    #[serde(alias="startCursor")]
    pub start_cursor: String,
    /// The kinds to query (if empty, returns entities from all kinds).    
    pub kinds: Vec<KindExpression>,
    /// The projection to return. If not set the entire entity is returned.    
    pub projection: Vec<PropertyExpression>,
    /// The properties to group by (if empty, no grouping is applied to the result set).    
    #[serde(alias="groupBy")]
    pub group_by: Vec<PropertyReference>,
    /// The filter to apply (optional).    
    pub filter: Filter,
    /// The maximum number of results to return. Applies after all other constraints. Optional.    
    pub limit: i32,
    /// The number of results to skip. Applies before limit, but after all other constraints (optional, defaults to 0).    
    pub offset: i32,
    /// An ending point for the query results. Optional. Query cursors are returned in query result batches.    
    #[serde(alias="endCursor")]
    pub end_cursor: String,
    /// The order to apply to the query results (if empty, order is unspecified).    
    pub order: Vec<PropertyOrder>,
}

impl Part for Query {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *dataset* resources.
/// It is not used directly, but through the `Datastore` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-datastore1_beta2" as datastore1_beta2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use datastore1_beta2::Datastore;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Datastore::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `allocate_ids(...)`, `begin_transaction(...)`, `commit(...)`, `lookup(...)`, `rollback(...)` and `run_query(...)`
/// // to build up your call.
/// let rb = hub.datasets();
/// # }
/// ```
pub struct DatasetMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Datastore<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for DatasetMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> DatasetMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Commit a transaction, optionally creating, deleting or modifying some entities.    
    pub fn commit(&self, request: &CommitRequest, dataset_id: &str) -> DatasetCommitCall<'a, C, NC, A> {
        DatasetCommitCall {
            hub: self.hub,
            _request: request.clone(),
            _dataset_id: dataset_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Allocate IDs for incomplete keys (useful for referencing an entity before it is inserted).    
    pub fn allocate_ids(&self, request: &AllocateIdsRequest, dataset_id: &str) -> DatasetAllocateIdCall<'a, C, NC, A> {
        DatasetAllocateIdCall {
            hub: self.hub,
            _request: request.clone(),
            _dataset_id: dataset_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Roll back a transaction.    
    pub fn rollback(&self, request: &RollbackRequest, dataset_id: &str) -> DatasetRollbackCall<'a, C, NC, A> {
        DatasetRollbackCall {
            hub: self.hub,
            _request: request.clone(),
            _dataset_id: dataset_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Look up some entities by key.    
    pub fn lookup(&self, request: &LookupRequest, dataset_id: &str) -> DatasetLookupCall<'a, C, NC, A> {
        DatasetLookupCall {
            hub: self.hub,
            _request: request.clone(),
            _dataset_id: dataset_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Query for entities.    
    pub fn run_query(&self, request: &RunQueryRequest, dataset_id: &str) -> DatasetRunQueryCall<'a, C, NC, A> {
        DatasetRunQueryCall {
            hub: self.hub,
            _request: request.clone(),
            _dataset_id: dataset_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Begin a new transaction.    
    pub fn begin_transaction(&self, request: &BeginTransactionRequest, dataset_id: &str) -> DatasetBeginTransactionCall<'a, C, NC, A> {
        DatasetBeginTransactionCall {
            hub: self.hub,
            _request: request.clone(),
            _dataset_id: dataset_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Commit a transaction, optionally creating, deleting or modifying some entities.
///
/// A builder for the *commit* method supported by a *dataset* resource.
/// It is not used directly, but through a `DatasetMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-datastore1_beta2" as datastore1_beta2;
/// use datastore1_beta2::CommitRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use datastore1_beta2::Datastore;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Datastore::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: CommitRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.datasets().commit(&req, "datasetId")
///              .doit();
/// # }
/// ```
pub struct DatasetCommitCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Datastore<C, NC, A>,
    _request: CommitRequest,
    _dataset_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for DatasetCommitCall<'a, C, NC, A> {}

impl<'a, C, NC, A> DatasetCommitCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, CommitResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "datastore.datasets.commit", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("datasetId", self._dataset_id.to_string()));
        for &field in ["alt", "datasetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/datastore/v1beta2/datasets/{datasetId}/commit".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{datasetId}", "datasetId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["datasetId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
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
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &CommitRequest) -> DatasetCommitCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Identifies the dataset.    
    pub fn dataset_id(mut self, new_value: &str) -> DatasetCommitCall<'a, C, NC, A> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DatasetCommitCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> DatasetCommitCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> DatasetCommitCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Allocate IDs for incomplete keys (useful for referencing an entity before it is inserted).
///
/// A builder for the *allocateIds* method supported by a *dataset* resource.
/// It is not used directly, but through a `DatasetMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-datastore1_beta2" as datastore1_beta2;
/// use datastore1_beta2::AllocateIdsRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use datastore1_beta2::Datastore;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Datastore::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: AllocateIdsRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.datasets().allocate_ids(&req, "datasetId")
///              .doit();
/// # }
/// ```
pub struct DatasetAllocateIdCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Datastore<C, NC, A>,
    _request: AllocateIdsRequest,
    _dataset_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for DatasetAllocateIdCall<'a, C, NC, A> {}

impl<'a, C, NC, A> DatasetAllocateIdCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, AllocateIdsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "datastore.datasets.allocateIds", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("datasetId", self._dataset_id.to_string()));
        for &field in ["alt", "datasetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/datastore/v1beta2/datasets/{datasetId}/allocateIds".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{datasetId}", "datasetId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["datasetId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
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
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &AllocateIdsRequest) -> DatasetAllocateIdCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Identifies the dataset.    
    pub fn dataset_id(mut self, new_value: &str) -> DatasetAllocateIdCall<'a, C, NC, A> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DatasetAllocateIdCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> DatasetAllocateIdCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> DatasetAllocateIdCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Roll back a transaction.
///
/// A builder for the *rollback* method supported by a *dataset* resource.
/// It is not used directly, but through a `DatasetMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-datastore1_beta2" as datastore1_beta2;
/// use datastore1_beta2::RollbackRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use datastore1_beta2::Datastore;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Datastore::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: RollbackRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.datasets().rollback(&req, "datasetId")
///              .doit();
/// # }
/// ```
pub struct DatasetRollbackCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Datastore<C, NC, A>,
    _request: RollbackRequest,
    _dataset_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for DatasetRollbackCall<'a, C, NC, A> {}

impl<'a, C, NC, A> DatasetRollbackCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, RollbackResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "datastore.datasets.rollback", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("datasetId", self._dataset_id.to_string()));
        for &field in ["alt", "datasetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/datastore/v1beta2/datasets/{datasetId}/rollback".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{datasetId}", "datasetId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["datasetId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
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
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &RollbackRequest) -> DatasetRollbackCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Identifies the dataset.    
    pub fn dataset_id(mut self, new_value: &str) -> DatasetRollbackCall<'a, C, NC, A> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DatasetRollbackCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> DatasetRollbackCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> DatasetRollbackCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Look up some entities by key.
///
/// A builder for the *lookup* method supported by a *dataset* resource.
/// It is not used directly, but through a `DatasetMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-datastore1_beta2" as datastore1_beta2;
/// use datastore1_beta2::LookupRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use datastore1_beta2::Datastore;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Datastore::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: LookupRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.datasets().lookup(&req, "datasetId")
///              .doit();
/// # }
/// ```
pub struct DatasetLookupCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Datastore<C, NC, A>,
    _request: LookupRequest,
    _dataset_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for DatasetLookupCall<'a, C, NC, A> {}

impl<'a, C, NC, A> DatasetLookupCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, LookupResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "datastore.datasets.lookup", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("datasetId", self._dataset_id.to_string()));
        for &field in ["alt", "datasetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/datastore/v1beta2/datasets/{datasetId}/lookup".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{datasetId}", "datasetId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["datasetId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
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
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &LookupRequest) -> DatasetLookupCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Identifies the dataset.    
    pub fn dataset_id(mut self, new_value: &str) -> DatasetLookupCall<'a, C, NC, A> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DatasetLookupCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> DatasetLookupCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> DatasetLookupCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Query for entities.
///
/// A builder for the *runQuery* method supported by a *dataset* resource.
/// It is not used directly, but through a `DatasetMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-datastore1_beta2" as datastore1_beta2;
/// use datastore1_beta2::RunQueryRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use datastore1_beta2::Datastore;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Datastore::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: RunQueryRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.datasets().run_query(&req, "datasetId")
///              .doit();
/// # }
/// ```
pub struct DatasetRunQueryCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Datastore<C, NC, A>,
    _request: RunQueryRequest,
    _dataset_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for DatasetRunQueryCall<'a, C, NC, A> {}

impl<'a, C, NC, A> DatasetRunQueryCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, RunQueryResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "datastore.datasets.runQuery", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("datasetId", self._dataset_id.to_string()));
        for &field in ["alt", "datasetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/datastore/v1beta2/datasets/{datasetId}/runQuery".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{datasetId}", "datasetId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["datasetId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
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
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &RunQueryRequest) -> DatasetRunQueryCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Identifies the dataset.    
    pub fn dataset_id(mut self, new_value: &str) -> DatasetRunQueryCall<'a, C, NC, A> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DatasetRunQueryCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> DatasetRunQueryCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> DatasetRunQueryCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Begin a new transaction.
///
/// A builder for the *beginTransaction* method supported by a *dataset* resource.
/// It is not used directly, but through a `DatasetMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-datastore1_beta2" as datastore1_beta2;
/// use datastore1_beta2::BeginTransactionRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use datastore1_beta2::Datastore;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Datastore::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: BeginTransactionRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.datasets().begin_transaction(&req, "datasetId")
///              .doit();
/// # }
/// ```
pub struct DatasetBeginTransactionCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Datastore<C, NC, A>,
    _request: BeginTransactionRequest,
    _dataset_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for DatasetBeginTransactionCall<'a, C, NC, A> {}

impl<'a, C, NC, A> DatasetBeginTransactionCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BeginTransactionResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "datastore.datasets.beginTransaction", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("datasetId", self._dataset_id.to_string()));
        for &field in ["alt", "datasetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/datastore/v1beta2/datasets/{datasetId}/beginTransaction".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{datasetId}", "datasetId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["datasetId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
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
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &BeginTransactionRequest) -> DatasetBeginTransactionCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Identifies the dataset.    
    pub fn dataset_id(mut self, new_value: &str) -> DatasetBeginTransactionCall<'a, C, NC, A> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DatasetBeginTransactionCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> DatasetBeginTransactionCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> DatasetBeginTransactionCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


