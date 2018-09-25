// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *bigquery* crate version *1.0.7+20171202*, where *20171202* is the exact revision of the *bigquery:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.7*.
//!
//! Everything else about the *bigquery* *v2* API can be found at the
//! [official documentation site](https://cloud.google.com/bigquery/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/bigquery2).
//! # Features
//!
//! Handle the following *Resources* with ease from the central [hub](struct.Bigquery.html) ...
//!
//! * [datasets](struct.Dataset.html)
//!  * [*delete*](struct.DatasetDeleteCall.html), [*get*](struct.DatasetGetCall.html), [*insert*](struct.DatasetInsertCall.html), [*list*](struct.DatasetListCall.html), [*patch*](struct.DatasetPatchCall.html) and [*update*](struct.DatasetUpdateCall.html)
//! * [jobs](struct.Job.html)
//!  * [*cancel*](struct.JobCancelCall.html), [*get*](struct.JobGetCall.html), [*get query results*](struct.JobGetQueryResultCall.html), [*insert*](struct.JobInsertCall.html), [*list*](struct.JobListCall.html) and [*query*](struct.JobQueryCall.html)
//! * projects
//!  * [*get service account*](struct.ProjectGetServiceAccountCall.html) and [*list*](struct.ProjectListCall.html)
//! * tabledata
//!  * [*insert all*](struct.TabledataInsertAllCall.html) and [*list*](struct.TabledataListCall.html)
//! * [tables](struct.Table.html)
//!  * [*delete*](struct.TableDeleteCall.html), [*get*](struct.TableGetCall.html), [*insert*](struct.TableInsertCall.html), [*list*](struct.TableListCall.html), [*patch*](struct.TablePatchCall.html) and [*update*](struct.TableUpdateCall.html)
//!
//!
//! Upload supported by ...
//!
//! * [*insert jobs*](struct.JobInsertCall.html)
//!
//!
//!
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//!
//! # Structure of this Library
//!
//! The API is structured into the following primary items:
//!
//! * **[Hub](struct.Bigquery.html)**
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
//! let r = hub.tables().update(...).doit()
//! let r = hub.tables().insert(...).doit()
//! let r = hub.tables().list(...).doit()
//! let r = hub.tables().delete(...).doit()
//! let r = hub.tables().get(...).doit()
//! let r = hub.tables().patch(...).doit()
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
//! google-bigquery2 = "*"
//! ```
//!
//! ## A complete example
//!
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_bigquery2 as bigquery2;
//! use bigquery2::Table;
//! use bigquery2::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use bigquery2::Bigquery;
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
//!                               hyper::client::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = Bigquery::new(hyper::client::Client::new(), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = Table::default();
//!
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.tables().update(req, "projectId", "datasetId", "tableId")
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
#![feature(extern_prelude)]
// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate futures;
extern crate serde;
extern crate serde_json;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::ops::DerefMut;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use json::Value;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;
use std::time::Duration;
use futures::Future;
pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, Error, CallBuilder, Hub, ReadSeek, Part,
              ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, MethodsBuilder,
              Resource, ErrorResponse, remove_json_null_values};


// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// Manage your data and permissions in Google Cloud Storage
    DevstorageFullControl,

    /// View your data across Google Cloud Platform services
    CloudPlatformReadOnly,

    /// View your data in Google Cloud Storage
    DevstorageReadOnly,

    /// Manage your data in Google Cloud Storage
    DevstorageReadWrite,

    /// View and manage your data across Google Cloud Platform services
    CloudPlatform,

    /// Insert data into Google BigQuery
    Insertdata,

    /// View and manage your data in Google BigQuery
    Full,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::DevstorageFullControl => "https://www.googleapis.com/auth/devstorage.full_control",
            Scope::CloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
            Scope::DevstorageReadOnly => "https://www.googleapis.com/auth/devstorage.read_only",
            Scope::DevstorageReadWrite => "https://www.googleapis.com/auth/devstorage.read_write",
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::Insertdata => "https://www.googleapis.com/auth/bigquery.insertdata",
            Scope::Full => "https://www.googleapis.com/auth/bigquery",
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

/// Central instance to access all Bigquery related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_bigquery2 as bigquery2;
/// use bigquery2::Table;
/// use bigquery2::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use bigquery2::Bigquery;
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
///                               hyper::client::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::)),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Bigquery::new(hyper::client::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::)), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Table::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.tables().update(req, "projectId", "datasetId", "tableId")
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
pub struct Bigquery<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for Bigquery<C, A> {}

impl<'a, C, A> Bigquery<C, A>
    where  C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Bigquery<C, A> {
        Bigquery {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.7".to_string(),
            _base_url: "https://www.googleapis.com/bigquery/v2/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn datasets(&'a self) -> DatasetMethods<'a, C, A> {
        DatasetMethods { hub: &self }
    }
    pub fn jobs(&'a self) -> JobMethods<'a, C, A> {
        JobMethods { hub: &self }
    }
    pub fn projects(&'a self) -> ProjectMethods<'a, C, A> {
        ProjectMethods { hub: &self }
    }
    pub fn tabledata(&'a self) -> TabledataMethods<'a, C, A> {
        TabledataMethods { hub: &self }
    }
    pub fn tables(&'a self) -> TableMethods<'a, C, A> {
        TableMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.7`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/bigquery/v2/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// The rows to insert.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableDataInsertAllRequestRows {
    /// [Optional] A unique ID for each row. BigQuery uses this property to detect duplicate insertion requests on a best-effort basis.
    #[serde(rename="insertId")]
    pub insert_id: Option<String>,
    /// [Required] A JSON object that contains a row of data. The object's properties and values must match the destination table's schema.
    pub json: Option<JsonObject>,
}

impl NestedType for TableDataInsertAllRequestRows {}
impl Part for TableDataInsertAllRequestRows {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobStatistics4 {
    /// [Output-only] Number of files per destination URI or URI pattern specified in the extract configuration. These values will be in the same order as the URIs specified in the 'destinationUris' field.
    #[serde(rename="destinationUriFileCounts")]
    pub destination_uri_file_counts: Option<Vec<i64>>,
}

impl Part for JobStatistics4 {}


/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [query jobs](struct.JobQueryCall.html) (request)
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryRequest {
    /// [Optional] How long to wait for the query to complete, in milliseconds, before the request times out and returns. Note that this is only a timeout for the request, not the query. If the query takes longer to run than the timeout value, the call returns without any results and with the 'jobComplete' flag set to false. You can call GetQueryResults() to wait for the query to complete and read the results. The default value is 10000 milliseconds (10 seconds).
    #[serde(rename="timeoutMs")]
    pub timeout_ms: Option<u32>,
    /// The resource type of the request.
    pub kind: Option<String>,
    /// [Optional] If set to true, BigQuery doesn't run the job. Instead, if the query is valid, BigQuery returns statistics about the job such as how many bytes would be processed. If the query is invalid, an error returns. The default value is false.
    #[serde(rename="dryRun")]
    pub dry_run: Option<bool>,
    /// Standard SQL only. Set to POSITIONAL to use positional (?) query parameters or to NAMED to use named (@myparam) query parameters in this query.
    #[serde(rename="parameterMode")]
    pub parameter_mode: Option<String>,
    /// [Optional] Whether to look for the result in the query cache. The query cache is a best-effort cache that will be flushed whenever tables in the query are modified. The default value is true.
    #[serde(rename="useQueryCache")]
    pub use_query_cache: Option<bool>,
    /// [Optional] Specifies the default datasetId and projectId to assume for any unqualified table names in the query. If not set, all table names in the query string must be qualified in the format 'datasetId.tableId'.
    #[serde(rename="defaultDataset")]
    pub default_dataset: Option<DatasetReference>,
    /// Specifies whether to use BigQuery's legacy SQL dialect for this query. The default value is true. If set to false, the query will use BigQuery's standard SQL: https://cloud.google.com/bigquery/sql-reference/ When useLegacySql is set to false, the value of flattenResults is ignored; query will be run as if flattenResults is false.
    #[serde(rename="useLegacySql")]
    pub use_legacy_sql: Option<bool>,
    /// [Optional] The maximum number of rows of data to return per page of results. Setting this flag to a small value such as 1000 and then paging through results might improve reliability when the query result set is large. In addition to this limit, responses are also limited to 10 MB. By default, there is no maximum row count, and only the byte limit applies.
    #[serde(rename="maxResults")]
    pub max_results: Option<u32>,
    /// Query parameters for Standard SQL queries.
    #[serde(rename="queryParameters")]
    pub query_parameters: Option<Vec<QueryParameter>>,
    /// [Required] A query string, following the BigQuery query syntax, of the query to execute. Example: "SELECT count(f1) FROM [myProjectId:myDatasetId.myTableId]".
    pub query: Option<String>,
    /// [Deprecated] This property is deprecated.
    #[serde(rename="preserveNulls")]
    pub preserve_nulls: Option<bool>,
}

impl RequestValue for QueryRequest {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobStatistics2 {
    /// [Output-only] Slot-milliseconds for the job.
    #[serde(rename="totalSlotMs")]
    pub total_slot_ms: Option<String>,
    /// [Output-only] Describes execution plan for the query.
    #[serde(rename="queryPlan")]
    pub query_plan: Option<Vec<ExplainQueryStage>>,
    /// [Output-only] The original estimate of bytes processed for the job.
    #[serde(rename="estimatedBytesProcessed")]
    pub estimated_bytes_processed: Option<String>,
    /// [Output-only, Experimental] The type of query statement, if valid. Possible values (new values might be added in the future): "SELECT": SELECT query. "INSERT": INSERT query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language "UPDATE": UPDATE query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language "DELETE": DELETE query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language "CREATE_TABLE": CREATE [OR REPLACE] TABLE without AS SELECT. "CREATE_TABLE_AS_SELECT": CREATE [OR REPLACE] TABLE ... AS SELECT ... "DROP_TABLE": DROP TABLE query. "CREATE_VIEW": CREATE [OR REPLACE] VIEW ... AS SELECT ... "DROP_VIEW": DROP VIEW query.
    #[serde(rename="statementType")]
    pub statement_type: Option<String>,
    /// [Output-only] Total bytes billed for the job.
    #[serde(rename="totalBytesBilled")]
    pub total_bytes_billed: Option<String>,
    /// [Output-only] Total bytes processed for the job.
    #[serde(rename="totalBytesProcessed")]
    pub total_bytes_processed: Option<String>,
    /// [Output-only] Whether the query result was fetched from the query cache.
    #[serde(rename="cacheHit")]
    pub cache_hit: Option<bool>,
    /// [Output-only, Experimental] The DDL operation performed, possibly dependent on the pre-existence of the DDL target. Possible values (new values might be added in the future): "CREATE": The query created the DDL target. "SKIP": No-op. Example cases: the query is CREATE TABLE IF NOT EXISTS while the table already exists, or the query is DROP TABLE IF EXISTS while the table does not exist. "REPLACE": The query replaced the DDL target. Example case: the query is CREATE OR REPLACE TABLE, and the table already exists. "DROP": The query deleted the DDL target.
    #[serde(rename="ddlOperationPerformed")]
    pub ddl_operation_performed: Option<String>,
    /// [Output-only] The number of rows affected by a DML statement. Present only for DML statements INSERT, UPDATE or DELETE.
    #[serde(rename="numDmlAffectedRows")]
    pub num_dml_affected_rows: Option<String>,
    /// [Output-only, Experimental] Standard SQL only: list of undeclared query parameters detected during a dry run validation.
    #[serde(rename="undeclaredQueryParameters")]
    pub undeclared_query_parameters: Option<Vec<QueryParameter>>,
    /// [Output-only] Billing tier for the job.
    #[serde(rename="billingTier")]
    pub billing_tier: Option<i32>,
    /// [Output-only, Experimental] The DDL target table. Present only for CREATE/DROP TABLE/VIEW queries.
    #[serde(rename="ddlTargetTable")]
    pub ddl_target_table: Option<TableReference>,
    /// [Output-only, Experimental] Referenced tables for the job. Queries that reference more than 50 tables will not have a complete list.
    #[serde(rename="referencedTables")]
    pub referenced_tables: Option<Vec<TableReference>>,
    /// [Output-only, Experimental] The schema of the results. Present only for successful dry run of non-legacy SQL queries.
    pub schema: Option<TableSchema>,
}

impl Part for JobStatistics2 {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobStatistics3 {
    /// [Output-only] Number of rows imported in a load job. Note that while an import job is in the running state, this value may change.
    #[serde(rename="outputRows")]
    pub output_rows: Option<String>,
    /// [Output-only] The number of bad records encountered. Note that if the job has failed because of more bad records encountered than the maximum allowed in the load job configuration, then this number can be less than the total number of bad records present in the input data.
    #[serde(rename="badRecords")]
    pub bad_records: Option<String>,
    /// [Output-only] Number of source files in a load job.
    #[serde(rename="inputFiles")]
    pub input_files: Option<String>,
    /// [Output-only] Size of the loaded data in bytes. Note that while a load job is in the running state, this value may change.
    #[serde(rename="outputBytes")]
    pub output_bytes: Option<String>,
    /// [Output-only] Number of bytes of source data in a load job.
    #[serde(rename="inputFileBytes")]
    pub input_file_bytes: Option<String>,
}

impl Part for JobStatistics3 {}


/// Additional details for a view.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableListTablesView {
    /// True if view is defined in legacy SQL dialect, false if in standard SQL.
    #[serde(rename="useLegacySql")]
    pub use_legacy_sql: Option<bool>,
}

impl NestedType for TableListTablesView {}
impl Part for TableListTablesView {}


/// Represents a single JSON object.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JsonObject(Option<HashMap<String, JsonValue>>);

impl Part for JsonObject {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EncryptionConfiguration {
    /// [Optional] Describes the Cloud KMS encryption key that will be used to protect destination BigQuery table. The BigQuery Service Account associated with your project requires access to this encryption key.
    #[serde(rename="kmsKeyName")]
    pub kms_key_name: Option<String>,
}

impl Part for EncryptionConfiguration {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableFieldSchema {
    /// [Optional] Describes the nested schema fields if the type property is set to RECORD.
    pub fields: Option<Vec<TableFieldSchema>>,
    /// [Optional] The field description. The maximum length is 1,024 characters.
    pub description: Option<String>,
    /// [Required] The field data type. Possible values include STRING, BYTES, INTEGER, INT64 (same as INTEGER), FLOAT, FLOAT64 (same as FLOAT), BOOLEAN, BOOL (same as BOOLEAN), TIMESTAMP, DATE, TIME, DATETIME, RECORD (where RECORD indicates that the field contains a nested schema) or STRUCT (same as RECORD).
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// [Optional] The field mode. Possible values include NULLABLE, REQUIRED and REPEATED. The default value is NULLABLE.
    pub mode: Option<String>,
    /// [Required] The field name. The name must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_), and must start with a letter or underscore. The maximum length is 128 characters.
    pub name: Option<String>,
}

impl Part for TableFieldSchema {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryParameter {
    /// [Required] The type of this parameter.
    #[serde(rename="parameterType")]
    pub parameter_type: Option<QueryParameterType>,
    /// [Required] The value of this parameter.
    #[serde(rename="parameterValue")]
    pub parameter_value: Option<QueryParameterValue>,
    /// [Optional] If unset, this is a positional parameter. Otherwise, should be unique within a query.
    pub name: Option<String>,
}

impl Part for QueryParameter {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BigtableColumnFamily {
    /// [Optional] The encoding of the values when the type is not STRING. Acceptable encoding values are: TEXT - indicates values are alphanumeric text strings. BINARY - indicates values are encoded using HBase Bytes.toBytes family of functions. This can be overridden for a specific column by listing that column in 'columns' and specifying an encoding for it.
    pub encoding: Option<String>,
    /// [Optional] The type to convert the value in cells of this column family. The values are expected to be encoded using HBase Bytes.toBytes function when using the BINARY encoding value. Following BigQuery types are allowed (case-sensitive) - BYTES STRING INTEGER FLOAT BOOLEAN Default type is BYTES. This can be overridden for a specific column by listing that column in 'columns' and specifying a type for it.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// [Optional] If this is set only the latest version of value are exposed for all columns in this column family. This can be overridden for a specific column by listing that column in 'columns' and specifying a different setting for that column.
    #[serde(rename="onlyReadLatest")]
    pub only_read_latest: Option<bool>,
    /// [Optional] Lists of columns that should be exposed as individual fields as opposed to a list of (column name, value) pairs. All columns whose qualifier matches a qualifier in this list can be accessed as .. Other columns can be accessed as a list through .Column field.
    pub columns: Option<Vec<BigtableColumn>>,
    /// Identifier of the column family.
    #[serde(rename="familyId")]
    pub family_id: Option<String>,
}

impl Part for BigtableColumnFamily {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BigtableOptions {
    /// [Optional] If field is true, then the rowkey column families will be read and converted to string. Otherwise they are read with BYTES type values and users need to manually cast them with CAST if necessary. The default value is false.
    #[serde(rename="readRowkeyAsString")]
    pub read_rowkey_as_string: Option<bool>,
    /// [Optional] If field is true, then the column families that are not specified in columnFamilies list are not exposed in the table schema. Otherwise, they are read with BYTES type values. The default value is false.
    #[serde(rename="ignoreUnspecifiedColumnFamilies")]
    pub ignore_unspecified_column_families: Option<bool>,
    /// [Optional] List of column families to expose in the table schema along with their types. This list restricts the column families that can be referenced in queries and specifies their value types. You can use this list to do type conversions - see the 'type' field for more details. If you leave this list empty, all column families are present in the table schema and their values are read as BYTES. During a query only the column families referenced in that query are read from Bigtable.
    #[serde(rename="columnFamilies")]
    pub column_families: Option<Vec<BigtableColumnFamily>>,
}

impl Part for BigtableOptions {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BigtableColumn {
    /// [Required] Qualifier of the column. Columns in the parent column family that has this exact qualifier are exposed as . field. If the qualifier is valid UTF-8 string, it can be specified in the qualifier_string field. Otherwise, a base-64 encoded value must be set to qualifier_encoded. The column field name is the same as the column qualifier. However, if the qualifier is not a valid BigQuery field identifier i.e. does not match [a-zA-Z][a-zA-Z0-9_]*, a valid identifier must be provided as field_name.
    #[serde(rename="qualifierEncoded")]
    pub qualifier_encoded: Option<String>,
    /// [Optional] If the qualifier is not a valid BigQuery field identifier i.e. does not match [a-zA-Z][a-zA-Z0-9_]*, a valid identifier must be provided as the column field name and is used as field name in queries.
    #[serde(rename="fieldName")]
    pub field_name: Option<String>,
    /// [Optional] If this is set, only the latest version of value in this column are exposed. 'onlyReadLatest' can also be set at the column family level. However, the setting at this level takes precedence if 'onlyReadLatest' is set at both levels.
    #[serde(rename="onlyReadLatest")]
    pub only_read_latest: Option<bool>,
    /// [Optional] The encoding of the values when the type is not STRING. Acceptable encoding values are: TEXT - indicates values are alphanumeric text strings. BINARY - indicates values are encoded using HBase Bytes.toBytes family of functions. 'encoding' can also be set at the column family level. However, the setting at this level takes precedence if 'encoding' is set at both levels.
    pub encoding: Option<String>,
    /// no description provided
    #[serde(rename="qualifierString")]
    pub qualifier_string: Option<String>,
    /// [Optional] The type to convert the value in cells of this column. The values are expected to be encoded using HBase Bytes.toBytes function when using the BINARY encoding value. Following BigQuery types are allowed (case-sensitive) - BYTES STRING INTEGER FLOAT BOOLEAN Default type is BYTES. 'type' can also be set at the column family level. However, the setting at this level takes precedence if 'type' is set at both levels.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for BigtableColumn {}


/// An array of errors for rows that were not inserted.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableDataInsertAllResponseInsertErrors {
    /// The index of the row that error applies to.
    pub index: Option<u32>,
    /// Error information for the row indicated by the index property.
    pub errors: Option<Vec<ErrorProto>>,
}

impl NestedType for TableDataInsertAllResponseInsertErrors {}
impl Part for TableDataInsertAllResponseInsertErrors {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableCell {
    /// no description provided
    pub v: Option<Value>,
}

impl Part for TableCell {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryParameterType {
    /// [Optional] The types of the fields of this struct, in order, if this is a struct.
    #[serde(rename="structTypes")]
    pub struct_types: Option<Vec<QueryParameterTypeStructTypes>>,
    /// [Required] The top level type of this field.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// [Optional] The type of the array's elements, if this is an array.
    #[serde(rename="arrayType")]
    pub array_type: Option<Option<Box<QueryParameterType>>>,
}

impl Part for QueryParameterType {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobReference {
    /// [Required] The ID of the project containing this job.
    #[serde(rename="projectId")]
    pub project_id: Option<String>,
    /// [Required] The ID of the job. The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-). The maximum length is 1,024 characters.
    #[serde(rename="jobId")]
    pub job_id: Option<String>,
}

impl Part for JobReference {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExplainQueryStage {
    /// Current status for the stage.
    pub status: Option<String>,
    /// Total number of bytes written to shuffle.
    #[serde(rename="shuffleOutputBytes")]
    pub shuffle_output_bytes: Option<String>,
    /// Milliseconds the slowest shard spent on CPU-bound tasks.
    #[serde(rename="computeMsMax")]
    pub compute_ms_max: Option<String>,
    /// Milliseconds the average shard spent on CPU-bound tasks.
    #[serde(rename="computeMsAvg")]
    pub compute_ms_avg: Option<String>,
    /// Total number of bytes written to shuffle and spilled to disk.
    #[serde(rename="shuffleOutputBytesSpilled")]
    pub shuffle_output_bytes_spilled: Option<String>,
    /// Milliseconds the average shard spent waiting to be scheduled.
    #[serde(rename="waitMsAvg")]
    pub wait_ms_avg: Option<String>,
    /// Milliseconds the average shard spent on writing output.
    #[serde(rename="writeMsAvg")]
    pub write_ms_avg: Option<String>,
    /// Milliseconds the average shard spent reading input.
    #[serde(rename="readMsAvg")]
    pub read_ms_avg: Option<String>,
    /// Relative amount of time the average shard spent waiting to be scheduled.
    #[serde(rename="waitRatioAvg")]
    pub wait_ratio_avg: Option<f64>,
    /// Unique ID for stage within plan.
    pub id: Option<String>,
    /// Number of parallel input segments completed.
    #[serde(rename="completedParallelInputs")]
    pub completed_parallel_inputs: Option<String>,
    /// Relative amount of time the slowest shard spent on CPU-bound tasks.
    #[serde(rename="computeRatioMax")]
    pub compute_ratio_max: Option<f64>,
    /// Human-readable name for stage.
    pub name: Option<String>,
    /// Milliseconds the slowest shard spent waiting to be scheduled.
    #[serde(rename="waitMsMax")]
    pub wait_ms_max: Option<String>,
    /// Milliseconds the slowest shard spent reading input.
    #[serde(rename="readMsMax")]
    pub read_ms_max: Option<String>,
    /// Number of records written by the stage.
    #[serde(rename="recordsWritten")]
    pub records_written: Option<String>,
    /// Relative amount of time the slowest shard spent waiting to be scheduled.
    #[serde(rename="waitRatioMax")]
    pub wait_ratio_max: Option<f64>,
    /// Relative amount of time the average shard spent reading input.
    #[serde(rename="readRatioAvg")]
    pub read_ratio_avg: Option<f64>,
    /// Number of parallel input segments to be processed.
    #[serde(rename="parallelInputs")]
    pub parallel_inputs: Option<String>,
    /// Relative amount of time the slowest shard spent reading input.
    #[serde(rename="readRatioMax")]
    pub read_ratio_max: Option<f64>,
    /// List of operations within the stage in dependency order (approximately chronological).
    pub steps: Option<Vec<ExplainQueryStep>>,
    /// Number of records read into the stage.
    #[serde(rename="recordsRead")]
    pub records_read: Option<String>,
    /// Relative amount of time the average shard spent on CPU-bound tasks.
    #[serde(rename="computeRatioAvg")]
    pub compute_ratio_avg: Option<f64>,
    /// Relative amount of time the slowest shard spent on writing output.
    #[serde(rename="writeRatioMax")]
    pub write_ratio_max: Option<f64>,
    /// Milliseconds the slowest shard spent on writing output.
    #[serde(rename="writeMsMax")]
    pub write_ms_max: Option<String>,
    /// Relative amount of time the average shard spent on writing output.
    #[serde(rename="writeRatioAvg")]
    pub write_ratio_avg: Option<f64>,
}

impl Part for ExplainQueryStage {}


/// An array of the dataset resources in the project. Each resource contains basic information. For full information about a particular dataset resource, use the Datasets: get method. This property is omitted when there are no datasets in the project.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatasetListDatasets {
    /// A descriptive name for the dataset, if one exists.
    #[serde(rename="friendlyName")]
    pub friendly_name: Option<String>,
    /// The resource type. This property always returns the value "bigquery#dataset".
    pub kind: Option<String>,
    /// The labels associated with this dataset. You can use these to organize and group your datasets.
    pub labels: Option<HashMap<String, String>>,
    /// The fully-qualified, unique, opaque ID of the dataset.
    pub id: Option<String>,
    /// The dataset reference. Use this property to access specific parts of the dataset's ID, such as project ID or dataset ID.
    #[serde(rename="datasetReference")]
    pub dataset_reference: Option<DatasetReference>,
}

impl NestedType for DatasetListDatasets {}
impl Part for DatasetListDatasets {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Streamingbuffer {
    /// [Output-only] A lower-bound estimate of the number of bytes currently in the streaming buffer.
    #[serde(rename="estimatedBytes")]
    pub estimated_bytes: Option<String>,
    /// [Output-only] A lower-bound estimate of the number of rows currently in the streaming buffer.
    #[serde(rename="estimatedRows")]
    pub estimated_rows: Option<String>,
    /// [Output-only] Contains the timestamp of the oldest entry in the streaming buffer, in milliseconds since the epoch, if the streaming buffer is available.
    #[serde(rename="oldestEntryTime")]
    pub oldest_entry_time: Option<String>,
}

impl Part for Streamingbuffer {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserDefinedFunctionResource {
    /// [Pick one] A code resource to load from a Google Cloud Storage URI (gs://bucket/path).
    #[serde(rename="resourceUri")]
    pub resource_uri: Option<String>,
    /// [Pick one] An inline resource that contains code for a user-defined function (UDF). Providing a inline code resource is equivalent to providing a URI for a file containing the same code.
    #[serde(rename="inlineCode")]
    pub inline_code: Option<String>,
}

impl Part for UserDefinedFunctionResource {}


/// Tables in the requested dataset.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableListTables {
    /// The resource type.
    pub kind: Option<String>,
    /// [Experimental] The labels associated with this table. You can use these to organize and group your tables.
    pub labels: Option<HashMap<String, String>>,
    /// The time when this table was created, in milliseconds since the epoch.
    #[serde(rename="creationTime")]
    pub creation_time: Option<String>,
    /// An opaque ID of the table
    pub id: Option<String>,
    /// The user-friendly name for this table.
    #[serde(rename="friendlyName")]
    pub friendly_name: Option<String>,
    /// The time-based partitioning for this table.
    #[serde(rename="timePartitioning")]
    pub time_partitioning: Option<TimePartitioning>,
    /// [Optional] The time when this table expires, in milliseconds since the epoch. If not present, the table will persist indefinitely. Expired tables will be deleted and their storage reclaimed.
    #[serde(rename="expirationTime")]
    pub expiration_time: Option<String>,
    /// The type of table. Possible values are: TABLE, VIEW.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// A reference uniquely identifying the table.
    #[serde(rename="tableReference")]
    pub table_reference: Option<TableReference>,
    /// Additional details for a view.
    pub view: Option<TableListTablesView>,
}

impl NestedType for TableListTables {}
impl Part for TableListTables {}


/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list tables](struct.TableListCall.html) (response)
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableList {
    /// A token to request the next page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Tables in the requested dataset.
    pub tables: Option<Vec<TableListTables>>,
    /// The type of list.
    pub kind: Option<String>,
    /// A hash of this page of results.
    pub etag: Option<String>,
    /// The total number of tables in the dataset.
    #[serde(rename="totalItems")]
    pub total_items: Option<i32>,
}

impl ResponseResult for TableList {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProjectReference {
    /// [Required] ID of the project. Can be either the numeric ID or the assigned ID of the project.
    #[serde(rename="projectId")]
    pub project_id: Option<String>,
}

impl Part for ProjectReference {}


/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list tabledata](struct.TabledataListCall.html) (response)
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableDataList {
    /// A token used for paging results. Providing this token instead of the startIndex parameter can help you retrieve stable results when an underlying table is changing.
    #[serde(rename="pageToken")]
    pub page_token: Option<String>,
    /// The resource type of the response.
    pub kind: Option<String>,
    /// A hash of this page of results.
    pub etag: Option<String>,
    /// Rows of results.
    pub rows: Option<Vec<TableRow>>,
    /// The total number of rows in the complete table.
    #[serde(rename="totalRows")]
    pub total_rows: Option<String>,
}

impl ResponseResult for TableDataList {}


/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [cancel jobs](struct.JobCancelCall.html) (none)
/// * [list jobs](struct.JobListCall.html) (none)
/// * [get query results jobs](struct.JobGetQueryResultCall.html) (none)
/// * [query jobs](struct.JobQueryCall.html) (none)
/// * [get jobs](struct.JobGetCall.html) (response)
/// * [insert jobs](struct.JobInsertCall.html) (request|response)
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    /// [Output-only] The status of this job. Examine this value when polling an asynchronous job to see if the job is complete.
    pub status: Option<JobStatus>,
    /// [Output-only] The type of the resource.
    pub kind: Option<String>,
    /// [Output-only] Information about the job, including starting time and ending time of the job.
    pub statistics: Option<JobStatistics>,
    /// [Optional] Reference describing the unique-per-user name of the job.
    #[serde(rename="jobReference")]
    pub job_reference: Option<JobReference>,
    /// [Output-only] A hash of this resource.
    pub etag: Option<String>,
    /// [Output-only] A URL that can be used to access this resource again.
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
    /// [Required] Describes the job configuration.
    pub configuration: Option<JobConfiguration>,
    /// [Output-only] Opaque ID field of the job
    pub id: Option<String>,
    /// [Output-only] Email address of the user who ran the job.
    pub user_email: Option<String>,
}

impl RequestValue for Job {}
impl Resource for Job {}
impl ResponseResult for Job {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimePartitioning {
    /// [Experimental] [Optional] If not set, the table is partitioned by pseudo column '_PARTITIONTIME'; if set, the table is partitioned by this field. The field must be a top-level TIMESTAMP or DATE field. Its mode must be NULLABLE or REQUIRED.
    pub field: Option<String>,
    /// [Required] The only type supported is DAY, which will generate one partition per day.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// [Optional] Number of milliseconds for which to keep the storage for a partition.
    #[serde(rename="expirationMs")]
    pub expiration_ms: Option<String>,
}

impl Part for TimePartitioning {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobConfigurationLoad {
    /// [Experimental] Custom encryption configuration (e.g., Cloud KMS keys).
    #[serde(rename="destinationEncryptionConfiguration")]
    pub destination_encryption_configuration: Option<EncryptionConfiguration>,
    /// [Optional] The character encoding of the data. The supported values are UTF-8 or ISO-8859-1. The default value is UTF-8. BigQuery decodes the data after the raw, binary data has been split using the values of the quote and fieldDelimiter properties.
    pub encoding: Option<String>,
    /// [Optional] The value that is used to quote data sections in a CSV file. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. The default value is a double-quote ('"'). If your data does not contain quoted sections, set the property value to an empty string. If your data contains quoted newline characters, you must also set the allowQuotedNewlines property to true.
    pub quote: Option<String>,
    /// Indicates if we should automatically infer the options and schema for CSV and JSON sources.
    pub autodetect: Option<bool>,
    /// [Required] The destination table to load the data into.
    #[serde(rename="destinationTable")]
    pub destination_table: Option<TableReference>,
    /// [Required] The fully-qualified URIs that point to your data in Google Cloud. For Google Cloud Storage URIs: Each URI can contain one '*' wildcard character and it must come after the 'bucket' name. Size limits related to load jobs apply to external data sources. For Google Cloud Bigtable URIs: Exactly one URI can be specified and it has be a fully specified and valid HTTPS URL for a Google Cloud Bigtable table. For Google Cloud Datastore backups: Exactly one URI can be specified. Also, the '*' wildcard character is not allowed.
    #[serde(rename="sourceUris")]
    pub source_uris: Option<Vec<String>>,
    /// [Optional] Specifies a string that represents a null value in a CSV file. For example, if you specify "\N", BigQuery interprets "\N" as a null value when loading a CSV file. The default value is the empty string. If you set this property to a custom value, BigQuery throws an error if an empty string is present for all data types except for STRING and BYTE. For STRING and BYTE columns, BigQuery interprets the empty string as an empty value.
    #[serde(rename="nullMarker")]
    pub null_marker: Option<String>,
    /// If specified, configures time-based partitioning for the destination table.
    #[serde(rename="timePartitioning")]
    pub time_partitioning: Option<TimePartitioning>,
    /// Indicates if BigQuery should allow quoted data sections that contain newline characters in a CSV file. The default value is false.
    #[serde(rename="allowQuotedNewlines")]
    pub allow_quoted_newlines: Option<bool>,
    /// If sourceFormat is set to "DATASTORE_BACKUP", indicates which entity properties to load into BigQuery from a Cloud Datastore backup. Property names are case sensitive and must be top-level properties. If no properties are specified, BigQuery loads all properties. If any named property isn't found in the Cloud Datastore backup, an invalid error is returned in the job result.
    #[serde(rename="projectionFields")]
    pub projection_fields: Option<Vec<String>>,
    /// [Optional] Accept rows that are missing trailing optional columns. The missing values are treated as nulls. If false, records with missing trailing columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false. Only applicable to CSV, ignored for other formats.
    #[serde(rename="allowJaggedRows")]
    pub allow_jagged_rows: Option<bool>,
    /// [Optional] The separator for fields in a CSV file. The separator can be any ISO-8859-1 single-byte character. To use a character in the range 128-255, you must encode the character as UTF8. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. BigQuery also supports the escape sequence "\t" to specify a tab separator. The default value is a comma (',').
    #[serde(rename="fieldDelimiter")]
    pub field_delimiter: Option<String>,
    /// [Optional] The format of the data files. For CSV files, specify "CSV". For datastore backups, specify "DATASTORE_BACKUP". For newline-delimited JSON, specify "NEWLINE_DELIMITED_JSON". For Avro, specify "AVRO". The default value is CSV.
    #[serde(rename="sourceFormat")]
    pub source_format: Option<String>,
    /// [Optional] The maximum number of bad records that BigQuery can ignore when running the job. If the number of bad records exceeds this value, an invalid error is returned in the job result. The default value is 0, which requires that all records are valid.
    #[serde(rename="maxBadRecords")]
    pub max_bad_records: Option<i32>,
    /// Allows the schema of the destination table to be updated as a side effect of the load job if a schema is autodetected or supplied in the job configuration. Schema update options are supported in two cases: when writeDisposition is WRITE_APPEND; when writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table, specified by partition decorators. For normal tables, WRITE_TRUNCATE will always overwrite the schema. One or more of the following values are specified: ALLOW_FIELD_ADDITION: allow adding a nullable field to the schema. ALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable.
    #[serde(rename="schemaUpdateOptions")]
    pub schema_update_options: Option<Vec<String>>,
    /// [Optional] Indicates if BigQuery should allow extra values that are not represented in the table schema. If true, the extra values are ignored. If false, records with extra columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false. The sourceFormat property determines what BigQuery treats as an extra value: CSV: Trailing columns JSON: Named values that don't match any column names
    #[serde(rename="ignoreUnknownValues")]
    pub ignore_unknown_values: Option<bool>,
    /// [Optional] Specifies the action that occurs if the destination table already exists. The following values are supported: WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data. WRITE_APPEND: If the table already exists, BigQuery appends the data to the table. WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result. The default value is WRITE_APPEND. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Creation, truncation and append actions occur as one atomic update upon job completion.
    #[serde(rename="writeDisposition")]
    pub write_disposition: Option<String>,
    /// [Optional] The number of rows at the top of a CSV file that BigQuery will skip when loading the data. The default value is 0. This property is useful if you have header rows in the file that should be skipped.
    #[serde(rename="skipLeadingRows")]
    pub skip_leading_rows: Option<i32>,
    /// [Optional] Specifies whether the job is allowed to create new tables. The following values are supported: CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table. CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result. The default value is CREATE_IF_NEEDED. Creation, truncation and append actions occur as one atomic update upon job completion.
    #[serde(rename="createDisposition")]
    pub create_disposition: Option<String>,
    /// [Deprecated] The format of the schemaInline property.
    #[serde(rename="schemaInlineFormat")]
    pub schema_inline_format: Option<String>,
    /// [Deprecated] The inline schema. For CSV schemas, specify as "Field1:Type1[,Field2:Type2]*". For example, "foo:STRING, bar:INTEGER, baz:FLOAT".
    #[serde(rename="schemaInline")]
    pub schema_inline: Option<String>,
    /// [Optional] The schema for the destination table. The schema can be omitted if the destination table already exists, or if you're loading data from Google Cloud Datastore.
    pub schema: Option<TableSchema>,
}

impl Part for JobConfigurationLoad {}


/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list jobs](struct.JobListCall.html) (response)
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobList {
    /// A token to request the next page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The resource type of the response.
    pub kind: Option<String>,
    /// A hash of this page of results.
    pub etag: Option<String>,
    /// List of jobs that were requested.
    pub jobs: Option<Vec<JobListJobs>>,
}

impl ResponseResult for JobList {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobConfigurationExtract {
    /// [Pick one] DEPRECATED: Use destinationUris instead, passing only one URI as necessary. The fully-qualified Google Cloud Storage URI where the extracted table should be written.
    #[serde(rename="destinationUri")]
    pub destination_uri: Option<String>,
    /// [Optional] The exported file format. Possible values include CSV, NEWLINE_DELIMITED_JSON and AVRO. The default value is CSV. Tables with nested or repeated fields cannot be exported as CSV.
    #[serde(rename="destinationFormat")]
    pub destination_format: Option<String>,
    /// [Optional] The compression type to use for exported files. Possible values include GZIP and NONE. The default value is NONE.
    pub compression: Option<String>,
    /// [Pick one] A list of fully-qualified Google Cloud Storage URIs where the extracted table should be written.
    #[serde(rename="destinationUris")]
    pub destination_uris: Option<Vec<String>>,
    /// [Optional] Whether to print out a header row in the results. Default is true.
    #[serde(rename="printHeader")]
    pub print_header: Option<bool>,
    /// [Optional] Delimiter to use between fields in the exported data. Default is ','
    #[serde(rename="fieldDelimiter")]
    pub field_delimiter: Option<String>,
    /// [Required] A reference to the table being exported.
    #[serde(rename="sourceTable")]
    pub source_table: Option<TableReference>,
}

impl Part for JobConfigurationExtract {}


/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [get query results jobs](struct.JobGetQueryResultCall.html) (response)
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetQueryResultsResponse {
    /// The resource type of the response.
    pub kind: Option<String>,
    /// [Output-only] The first errors or warnings encountered during the running of the job. The final message includes the number of errors that caused the process to stop. Errors here do not necessarily mean that the job has completed or was unsuccessful.
    pub errors: Option<Vec<ErrorProto>>,
    /// Reference to the BigQuery Job that was created to run the query. This field will be present even if the original request timed out, in which case GetQueryResults can be used to read the results once the query has completed. Since this API only returns the first page of results, subsequent pages can be fetched via the same mechanism (GetQueryResults).
    #[serde(rename="jobReference")]
    pub job_reference: Option<JobReference>,
    /// Whether the query result was fetched from the query cache.
    #[serde(rename="cacheHit")]
    pub cache_hit: Option<bool>,
    /// Whether the query has completed or not. If rows or totalRows are present, this will always be true. If this is false, totalRows will not be available.
    #[serde(rename="jobComplete")]
    pub job_complete: Option<bool>,
    /// The total number of rows in the complete query result set, which can be more than the number of rows in this single page of results. Present only when the query completes successfully.
    #[serde(rename="totalRows")]
    pub total_rows: Option<String>,
    /// The total number of bytes processed for this query.
    #[serde(rename="totalBytesProcessed")]
    pub total_bytes_processed: Option<String>,
    /// A token used for paging results.
    #[serde(rename="pageToken")]
    pub page_token: Option<String>,
    /// A hash of this response.
    pub etag: Option<String>,
    /// An object with as many results as can be contained within the maximum permitted reply size. To get any additional rows, you can call GetQueryResults and specify the jobReference returned above. Present only when the query completes successfully.
    pub rows: Option<Vec<TableRow>>,
    /// [Output-only] The number of rows affected by a DML statement. Present only for DML statements INSERT, UPDATE or DELETE.
    #[serde(rename="numDmlAffectedRows")]
    pub num_dml_affected_rows: Option<String>,
    /// The schema of the results. Present only when the query completes successfully.
    pub schema: Option<TableSchema>,
}

impl ResponseResult for GetQueryResultsResponse {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableReference {
    /// [Required] The ID of the project containing this table.
    #[serde(rename="projectId")]
    pub project_id: Option<String>,
    /// [Required] The ID of the table. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters.
    #[serde(rename="tableId")]
    pub table_id: Option<String>,
    /// [Required] The ID of the dataset containing this table.
    #[serde(rename="datasetId")]
    pub dataset_id: Option<String>,
}

impl Part for TableReference {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CsvOptions {
    /// [Optional] Indicates if BigQuery should accept rows that are missing trailing optional columns. If true, BigQuery treats missing trailing columns as null values. If false, records with missing trailing columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false.
    #[serde(rename="allowJaggedRows")]
    pub allow_jagged_rows: Option<bool>,
    /// [Optional] The number of rows at the top of a CSV file that BigQuery will skip when reading the data. The default value is 0. This property is useful if you have header rows in the file that should be skipped.
    #[serde(rename="skipLeadingRows")]
    pub skip_leading_rows: Option<String>,
    /// [Optional] The separator for fields in a CSV file. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. BigQuery also supports the escape sequence "\t" to specify a tab separator. The default value is a comma (',').
    #[serde(rename="fieldDelimiter")]
    pub field_delimiter: Option<String>,
    /// [Optional] The character encoding of the data. The supported values are UTF-8 or ISO-8859-1. The default value is UTF-8. BigQuery decodes the data after the raw, binary data has been split using the values of the quote and fieldDelimiter properties.
    pub encoding: Option<String>,
    /// [Optional] The value that is used to quote data sections in a CSV file. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. The default value is a double-quote ('"'). If your data does not contain quoted sections, set the property value to an empty string. If your data contains quoted newline characters, you must also set the allowQuotedNewlines property to true.
    pub quote: Option<String>,
    /// [Optional] Indicates if BigQuery should allow quoted data sections that contain newline characters in a CSV file. The default value is false.
    #[serde(rename="allowQuotedNewlines")]
    pub allow_quoted_newlines: Option<bool>,
}

impl Part for CsvOptions {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExternalDataConfiguration {
    /// [Optional] The compression type of the data source. Possible values include GZIP and NONE. The default value is NONE. This setting is ignored for Google Cloud Bigtable, Google Cloud Datastore backups and Avro formats.
    pub compression: Option<String>,
    /// Additional properties to set if sourceFormat is set to CSV.
    #[serde(rename="csvOptions")]
    pub csv_options: Option<CsvOptions>,
    /// Try to detect schema and format options automatically. Any option specified explicitly will be honored.
    pub autodetect: Option<bool>,
    /// [Optional] The maximum number of bad records that BigQuery can ignore when reading data. If the number of bad records exceeds this value, an invalid error is returned in the job result. The default value is 0, which requires that all records are valid. This setting is ignored for Google Cloud Bigtable, Google Cloud Datastore backups and Avro formats.
    #[serde(rename="maxBadRecords")]
    pub max_bad_records: Option<i32>,
    /// [Optional] Indicates if BigQuery should allow extra values that are not represented in the table schema. If true, the extra values are ignored. If false, records with extra columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false. The sourceFormat property determines what BigQuery treats as an extra value: CSV: Trailing columns JSON: Named values that don't match any column names Google Cloud Bigtable: This setting is ignored. Google Cloud Datastore backups: This setting is ignored. Avro: This setting is ignored.
    #[serde(rename="ignoreUnknownValues")]
    pub ignore_unknown_values: Option<bool>,
    /// [Required] The fully-qualified URIs that point to your data in Google Cloud. For Google Cloud Storage URIs: Each URI can contain one '*' wildcard character and it must come after the 'bucket' name. Size limits related to load jobs apply to external data sources. For Google Cloud Bigtable URIs: Exactly one URI can be specified and it has be a fully specified and valid HTTPS URL for a Google Cloud Bigtable table. For Google Cloud Datastore backups, exactly one URI can be specified. Also, the '*' wildcard character is not allowed.
    #[serde(rename="sourceUris")]
    pub source_uris: Option<Vec<String>>,
    /// [Optional] Additional options if sourceFormat is set to BIGTABLE.
    #[serde(rename="bigtableOptions")]
    pub bigtable_options: Option<BigtableOptions>,
    /// [Required] The data format. For CSV files, specify "CSV". For Google sheets, specify "GOOGLE_SHEETS". For newline-delimited JSON, specify "NEWLINE_DELIMITED_JSON". For Avro files, specify "AVRO". For Google Cloud Datastore backups, specify "DATASTORE_BACKUP". [Beta] For Google Cloud Bigtable, specify "BIGTABLE".
    #[serde(rename="sourceFormat")]
    pub source_format: Option<String>,
    /// [Optional] Additional options if sourceFormat is set to GOOGLE_SHEETS.
    #[serde(rename="googleSheetsOptions")]
    pub google_sheets_options: Option<GoogleSheetsOptions>,
    /// [Optional] The schema for the data. Schema is required for CSV and JSON formats. Schema is disallowed for Google Cloud Bigtable, Cloud Datastore backups, and Avro formats.
    pub schema: Option<TableSchema>,
}

impl Part for ExternalDataConfiguration {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExplainQueryStep {
    /// Machine-readable operation type.
    pub kind: Option<String>,
    /// Human-readable stage descriptions.
    pub substeps: Option<Vec<String>>,
}

impl Part for ExplainQueryStep {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ErrorProto {
    /// Debugging information. This property is internal to Google and should not be used.
    #[serde(rename="debugInfo")]
    pub debug_info: Option<String>,
    /// A human-readable description of the error.
    pub message: Option<String>,
    /// A short error code that summarizes the error.
    pub reason: Option<String>,
    /// Specifies where the error occurred, if present.
    pub location: Option<String>,
}

impl Part for ErrorProto {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ViewDefinition {
    /// [Required] A query that BigQuery executes when the view is referenced.
    pub query: Option<String>,
    /// Specifies whether to use BigQuery's legacy SQL for this view. The default value is true. If set to false, the view will use BigQuery's standard SQL: https://cloud.google.com/bigquery/sql-reference/ Queries and views that reference this view must use the same flag value.
    #[serde(rename="useLegacySql")]
    pub use_legacy_sql: Option<bool>,
    /// Describes user-defined function resources used in the query.
    #[serde(rename="userDefinedFunctionResources")]
    pub user_defined_function_resources: Option<Vec<UserDefinedFunctionResource>>,
}

impl Part for ViewDefinition {}


/// [Optional] An array of objects that define dataset access for one or more entities. You can set this property when inserting or updating a dataset in order to control who is allowed to access the data. If unspecified at dataset creation time, BigQuery adds default dataset access for the following entities: access.specialGroup: projectReaders; access.role: READER; access.specialGroup: projectWriters; access.role: WRITER; access.specialGroup: projectOwners; access.role: OWNER; access.userByEmail: [dataset creator email]; access.role: OWNER;
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatasetAccess {
    /// [Pick one] A domain to grant access to. Any users signed in with the domain specified will be granted the specified access. Example: "example.com".
    pub domain: Option<String>,
    /// [Required] Describes the rights granted to the user specified by the other member of the access object. The following string values are supported: READER, WRITER, OWNER.
    pub role: Option<String>,
    /// [Pick one] An email address of a user to grant access to. For example: fred@example.com.
    #[serde(rename="userByEmail")]
    pub user_by_email: Option<String>,
    /// [Pick one] A special group to grant access to. Possible values include: projectOwners: Owners of the enclosing project. projectReaders: Readers of the enclosing project. projectWriters: Writers of the enclosing project. allAuthenticatedUsers: All authenticated BigQuery users.
    #[serde(rename="specialGroup")]
    pub special_group: Option<String>,
    /// [Pick one] An email address of a Google Group to grant access to.
    #[serde(rename="groupByEmail")]
    pub group_by_email: Option<String>,
    /// [Pick one] A view from a different dataset to grant access to. Queries executed against that view will have read access to tables in this dataset. The role field is not required when this field is set. If that view is updated by any user, access to the view needs to be granted again via an update operation.
    pub view: Option<TableReference>,
}

impl NestedType for DatasetAccess {}
impl Part for DatasetAccess {}


/// Projects to which you have at least READ access.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProjectListProjects {
    /// A descriptive name for this project.
    #[serde(rename="friendlyName")]
    pub friendly_name: Option<String>,
    /// The resource type.
    pub kind: Option<String>,
    /// The numeric ID of this project.
    #[serde(rename="numericId")]
    pub numeric_id: Option<String>,
    /// An opaque ID of this project.
    pub id: Option<String>,
    /// A unique reference to this project.
    #[serde(rename="projectReference")]
    pub project_reference: Option<ProjectReference>,
}

impl NestedType for ProjectListProjects {}
impl Part for ProjectListProjects {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobStatus {
    /// [Output-only] Running state of the job.
    pub state: Option<String>,
    /// [Output-only] The first errors encountered during the running of the job. The final message includes the number of errors that caused the process to stop. Errors here do not necessarily mean that the job has completed or was unsuccessful.
    pub errors: Option<Vec<ErrorProto>>,
    /// [Output-only] Final error result of the job. If present, indicates that the job has completed and was unsuccessful.
    #[serde(rename="errorResult")]
    pub error_result: Option<ErrorProto>,
}

impl Part for JobStatus {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableSchema {
    /// Describes the fields in a table.
    pub fields: Option<Vec<TableFieldSchema>>,
}

impl Part for TableSchema {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
/// The contained type is `Option<String>`.
///
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonValue(json::Value);

impl Default for JsonValue {
    fn default() -> JsonValue {
        JsonValue(json::Value::Null)
    }
}

impl Part for JsonValue {}


/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list projects](struct.ProjectListCall.html) (response)
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProjectList {
    /// A token to request the next page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The total number of projects in the list.
    #[serde(rename="totalItems")]
    pub total_items: Option<i32>,
    /// The type of list.
    pub kind: Option<String>,
    /// A hash of the page of results
    pub etag: Option<String>,
    /// Projects to which you have at least READ access.
    pub projects: Option<Vec<ProjectListProjects>>,
}

impl ResponseResult for ProjectList {}


/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [insert all tabledata](struct.TabledataInsertAllCall.html) (request)
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableDataInsertAllRequest {
    /// [Optional] Accept rows that contain values that do not match the schema. The unknown values are ignored. Default is false, which treats unknown values as errors.
    #[serde(rename="ignoreUnknownValues")]
    pub ignore_unknown_values: Option<bool>,
    /// The resource type of the response.
    pub kind: Option<String>,
    /// The rows to insert.
    pub rows: Option<Vec<TableDataInsertAllRequestRows>>,
    /// [Optional] Insert all valid rows of a request, even if invalid rows exist. The default value is false, which causes the entire request to fail if any invalid rows exist.
    #[serde(rename="skipInvalidRows")]
    pub skip_invalid_rows: Option<bool>,
    /// [Experimental] If specified, treats the destination table as a base template, and inserts the rows into an instance table named "{destination}{templateSuffix}". BigQuery will manage creation of the instance table, using the schema of the base template table. See https://cloud.google.com/bigquery/streaming-data-into-bigquery#template-tables for considerations when working with templates tables.
    #[serde(rename="templateSuffix")]
    pub template_suffix: Option<String>,
}

impl RequestValue for TableDataInsertAllRequest {}


/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list datasets](struct.DatasetListCall.html) (response)
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatasetList {
    /// A token that can be used to request the next results page. This property is omitted on the final results page.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The list type. This property always returns the value "bigquery#datasetList".
    pub kind: Option<String>,
    /// An array of the dataset resources in the project. Each resource contains basic information. For full information about a particular dataset resource, use the Datasets: get method. This property is omitted when there are no datasets in the project.
    pub datasets: Option<Vec<DatasetListDatasets>>,
    /// A hash value of the results page. You can use this property to determine if the page has changed since the last request.
    pub etag: Option<String>,
}

impl ResponseResult for DatasetList {}


/// [Optional] The types of the fields of this struct, in order, if this is a struct.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryParameterTypeStructTypes {
    /// [Required] The type of this field.
    #[serde(rename="type")]
    pub type_: Option<QueryParameterType>,
    /// [Optional] Human-oriented description of the field.
    pub description: Option<String>,
    /// [Optional] The name of this field.
    pub name: Option<String>,
}

impl NestedType for QueryParameterTypeStructTypes {}
impl Part for QueryParameterTypeStructTypes {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobStatistics {
    /// [Output-only] Statistics for a load job.
    pub load: Option<JobStatistics3>,
    /// [Output-only] Statistics for an extract job.
    pub extract: Option<JobStatistics4>,
    /// [Output-only] Start time of this job, in milliseconds since the epoch. This field will be present when the job transitions from the PENDING state to either RUNNING or DONE.
    #[serde(rename="startTime")]
    pub start_time: Option<String>,
    /// [Output-only] Statistics for a query job.
    pub query: Option<JobStatistics2>,
    /// [Output-only] End time of this job, in milliseconds since the epoch. This field will be present whenever a job is in the DONE state.
    #[serde(rename="endTime")]
    pub end_time: Option<String>,
    /// [Output-only] Creation time of this job, in milliseconds since the epoch. This field will be present on all jobs.
    #[serde(rename="creationTime")]
    pub creation_time: Option<String>,
    /// [Output-only] [Deprecated] Use the bytes processed in the query statistics instead.
    #[serde(rename="totalBytesProcessed")]
    pub total_bytes_processed: Option<String>,
}

impl Part for JobStatistics {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobConfigurationQuery {
    /// [Optional] If true and query uses legacy SQL dialect, flattens all nested and repeated fields in the query results. allowLargeResults must be true if this is set to false. For standard SQL queries, this flag is ignored and results are never flattened.
    #[serde(rename="flattenResults")]
    pub flatten_results: Option<bool>,
    /// [Experimental] Custom encryption configuration (e.g., Cloud KMS keys).
    #[serde(rename="destinationEncryptionConfiguration")]
    pub destination_encryption_configuration: Option<EncryptionConfiguration>,
    /// [Optional] Whether to look for the result in the query cache. The query cache is a best-effort cache that will be flushed whenever tables in the query are modified. Moreover, the query cache is only available when a query does not have a destination table specified. The default value is true.
    #[serde(rename="useQueryCache")]
    pub use_query_cache: Option<bool>,
    /// [Optional] Describes the table where the query results should be stored. If not present, a new table will be created to store the results. This property must be set for large results that exceed the maximum response size.
    #[serde(rename="destinationTable")]
    pub destination_table: Option<TableReference>,
    /// Query parameters for standard SQL queries.
    #[serde(rename="queryParameters")]
    pub query_parameters: Option<Vec<QueryParameter>>,
    /// [Required] SQL query text to execute. The useLegacySql field can be used to indicate whether the query uses legacy SQL or standard SQL.
    pub query: Option<String>,
    /// [Deprecated] This property is deprecated.
    #[serde(rename="preserveNulls")]
    pub preserve_nulls: Option<bool>,
    /// [Optional] Limits the bytes billed for this job. Queries that will have bytes billed beyond this limit will fail (without incurring a charge). If unspecified, this will be set to your project default.
    #[serde(rename="maximumBytesBilled")]
    pub maximum_bytes_billed: Option<String>,
    /// [Optional] Limits the billing tier for this job. Queries that have resource usage beyond this tier will fail (without incurring a charge). If unspecified, this will be set to your project default.
    #[serde(rename="maximumBillingTier")]
    pub maximum_billing_tier: Option<i32>,
    /// Standard SQL only. Set to POSITIONAL to use positional (?) query parameters or to NAMED to use named (@myparam) query parameters in this query.
    #[serde(rename="parameterMode")]
    pub parameter_mode: Option<String>,
    /// [Optional] Specifies the default dataset to use for unqualified table names in the query.
    #[serde(rename="defaultDataset")]
    pub default_dataset: Option<DatasetReference>,
    /// Specifies whether to use BigQuery's legacy SQL dialect for this query. The default value is true. If set to false, the query will use BigQuery's standard SQL: https://cloud.google.com/bigquery/sql-reference/ When useLegacySql is set to false, the value of flattenResults is ignored; query will be run as if flattenResults is false.
    #[serde(rename="useLegacySql")]
    pub use_legacy_sql: Option<bool>,
    /// Allows the schema of the destination table to be updated as a side effect of the query job. Schema update options are supported in two cases: when writeDisposition is WRITE_APPEND; when writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table, specified by partition decorators. For normal tables, WRITE_TRUNCATE will always overwrite the schema. One or more of the following values are specified: ALLOW_FIELD_ADDITION: allow adding a nullable field to the schema. ALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable.
    #[serde(rename="schemaUpdateOptions")]
    pub schema_update_options: Option<Vec<String>>,
    /// [Optional] Specifies a priority for the query. Possible values include INTERACTIVE and BATCH. The default value is INTERACTIVE.
    pub priority: Option<String>,
    /// [Optional] Specifies the action that occurs if the destination table already exists. The following values are supported: WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data and uses the schema from the query result. WRITE_APPEND: If the table already exists, BigQuery appends the data to the table. WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result. The default value is WRITE_EMPTY. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Creation, truncation and append actions occur as one atomic update upon job completion.
    #[serde(rename="writeDisposition")]
    pub write_disposition: Option<String>,
    /// [Optional] If true and query uses legacy SQL dialect, allows the query to produce arbitrarily large result tables at a slight cost in performance. Requires destinationTable to be set. For standard SQL queries, this flag is ignored and large results are always allowed. However, you must still set destinationTable when result size exceeds the allowed maximum response size.
    #[serde(rename="allowLargeResults")]
    pub allow_large_results: Option<bool>,
    /// If specified, configures time-based partitioning for the destination table.
    #[serde(rename="timePartitioning")]
    pub time_partitioning: Option<TimePartitioning>,
    /// [Optional] Specifies whether the job is allowed to create new tables. The following values are supported: CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table. CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result. The default value is CREATE_IF_NEEDED. Creation, truncation and append actions occur as one atomic update upon job completion.
    #[serde(rename="createDisposition")]
    pub create_disposition: Option<String>,
    /// Describes user-defined function resources used in the query.
    #[serde(rename="userDefinedFunctionResources")]
    pub user_defined_function_resources: Option<Vec<UserDefinedFunctionResource>>,
    /// [Optional] If querying an external data source outside of BigQuery, describes the data format, location and other properties of the data source. By defining these properties, the data source can then be queried as if it were a standard BigQuery table.
    #[serde(rename="tableDefinitions")]
    pub table_definitions: Option<HashMap<String, ExternalDataConfiguration>>,
}

impl Part for JobConfigurationQuery {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatasetReference {
    /// [Optional] The ID of the project containing this dataset.
    #[serde(rename="projectId")]
    pub project_id: Option<String>,
    /// [Required] A unique ID for this dataset, without the project name. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters.
    #[serde(rename="datasetId")]
    pub dataset_id: Option<String>,
}

impl Part for DatasetReference {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableRow {
    /// Represents a single row in the result set, consisting of one or more fields.
    pub f: Option<Vec<TableCell>>,
}

impl Part for TableRow {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobConfiguration {
    /// [Pick one] Configures a load job.
    pub load: Option<JobConfigurationLoad>,
    /// [Pick one] Copies a table.
    pub copy: Option<JobConfigurationTableCopy>,
    /// [Optional] If set, don't actually run this job. A valid query will return a mostly empty response with some processing statistics, while an invalid query will return the same error it would if it wasn't a dry run. Behavior of non-query jobs is undefined.
    #[serde(rename="dryRun")]
    pub dry_run: Option<bool>,
    /// [Pick one] Configures a query job.
    pub query: Option<JobConfigurationQuery>,
    /// [Experimental] The labels associated with this job. You can use these to organize and group your jobs. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter and each label in the list must have a different key.
    pub labels: Option<HashMap<String, String>>,
    /// [Pick one] Configures an extract job.
    pub extract: Option<JobConfigurationExtract>,
}

impl Part for JobConfiguration {}


/// List of jobs that were requested.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobListJobs {
    /// [Full-projection-only] Describes the state of the job.
    pub status: Option<JobStatus>,
    /// The resource type.
    pub kind: Option<String>,
    /// [Output-only] Information about the job, including starting time and ending time of the job.
    pub statistics: Option<JobStatistics>,
    /// Job reference uniquely identifying the job.
    #[serde(rename="jobReference")]
    pub job_reference: Option<JobReference>,
    /// Running state of the job. When the state is DONE, errorResult can be checked to determine whether the job succeeded or failed.
    pub state: Option<String>,
    /// A result object that will be present only if the job has failed.
    #[serde(rename="errorResult")]
    pub error_result: Option<ErrorProto>,
    /// [Full-projection-only] Specifies the job configuration.
    pub configuration: Option<JobConfiguration>,
    /// Unique opaque ID of the job.
    pub id: Option<String>,
    /// [Full-projection-only] Email address of the user who ran the job.
    pub user_email: Option<String>,
}

impl NestedType for JobListJobs {}
impl Part for JobListJobs {}


/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [cancel jobs](struct.JobCancelCall.html) (response)
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobCancelResponse {
    /// The final state of the job.
    pub job: Option<Job>,
    /// The resource type of the response.
    pub kind: Option<String>,
}

impl ResponseResult for JobCancelResponse {}


/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [get datasets](struct.DatasetGetCall.html) (response)
/// * [list datasets](struct.DatasetListCall.html) (none)
/// * [patch datasets](struct.DatasetPatchCall.html) (request|response)
/// * [update datasets](struct.DatasetUpdateCall.html) (request|response)
/// * [delete datasets](struct.DatasetDeleteCall.html) (none)
/// * [insert datasets](struct.DatasetInsertCall.html) (request|response)
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Dataset {
    /// [Output-only] The resource type.
    pub kind: Option<String>,
    /// [Optional] A user-friendly description of the dataset.
    pub description: Option<String>,
    /// [Required] A reference that identifies the dataset.
    #[serde(rename="datasetReference")]
    pub dataset_reference: Option<DatasetReference>,
    /// The labels associated with this dataset. You can use these to organize and group your datasets. You can set this property when inserting or updating a dataset. See Labeling Datasets for more information.
    pub labels: Option<HashMap<String, String>>,
    /// [Output-only] The time when this dataset was created, in milliseconds since the epoch.
    #[serde(rename="creationTime")]
    pub creation_time: Option<String>,
    /// [Optional] An array of objects that define dataset access for one or more entities. You can set this property when inserting or updating a dataset in order to control who is allowed to access the data. If unspecified at dataset creation time, BigQuery adds default dataset access for the following entities: access.specialGroup: projectReaders; access.role: READER; access.specialGroup: projectWriters; access.role: WRITER; access.specialGroup: projectOwners; access.role: OWNER; access.userByEmail: [dataset creator email]; access.role: OWNER;
    pub access: Option<Vec<DatasetAccess>>,
    /// [Optional] The default lifetime of all tables in the dataset, in milliseconds. The minimum value is 3600000 milliseconds (one hour). Once this property is set, all newly-created tables in the dataset will have an expirationTime property set to the creation time plus the value in this property, and changing the value will only affect new tables, not existing ones. When the expirationTime for a given table is reached, that table will be deleted automatically. If a table's expirationTime is modified or removed before the table expires, or if you provide an explicit expirationTime when creating a table, that value takes precedence over the default expiration time indicated by this property.
    #[serde(rename="defaultTableExpirationMs")]
    pub default_table_expiration_ms: Option<String>,
    /// [Output-only] A hash of the resource.
    pub etag: Option<String>,
    /// The geographic location where the dataset should reside. Possible values include EU and US. The default value is US.
    pub location: Option<String>,
    /// [Optional] A descriptive name for the dataset.
    #[serde(rename="friendlyName")]
    pub friendly_name: Option<String>,
    /// [Output-only] The date when this dataset or any of its tables was last modified, in milliseconds since the epoch.
    #[serde(rename="lastModifiedTime")]
    pub last_modified_time: Option<String>,
    /// [Output-only] The fully-qualified unique name of the dataset in the format projectId:datasetId. The dataset name without the project name is given in the datasetId field. When creating a new dataset, leave this field blank, and instead specify the datasetId field.
    pub id: Option<String>,
    /// [Output-only] A URL that can be used to access the resource again. You can use this URL in Get or Update requests to the resource.
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
}

impl RequestValue for Dataset {}
impl Resource for Dataset {}
impl ResponseResult for Dataset {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryParameterValue {
    /// [Optional] The struct field values, in order of the struct type's declaration.
    #[serde(rename="structValues")]
    pub struct_values: Option<HashMap<String, QueryParameterValue>>,
    /// [Optional] The array values, if this is an array type.
    #[serde(rename="arrayValues")]
    pub array_values: Option<Vec<QueryParameterValue>>,
    /// [Optional] The value of this value, if a simple scalar type.
    pub value: Option<String>,
}

impl Part for QueryParameterValue {}


/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [get service account projects](struct.ProjectGetServiceAccountCall.html) (response)
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetServiceAccountResponse {
    /// The resource type of the response.
    pub kind: Option<String>,
    /// The service account email address.
    pub email: Option<String>,
}

impl ResponseResult for GetServiceAccountResponse {}


/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [update tables](struct.TableUpdateCall.html) (request|response)
/// * [insert tables](struct.TableInsertCall.html) (request|response)
/// * [list tables](struct.TableListCall.html) (none)
/// * [delete tables](struct.TableDeleteCall.html) (none)
/// * [get tables](struct.TableGetCall.html) (response)
/// * [patch tables](struct.TablePatchCall.html) (request|response)
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Table {
    /// [Optional] A user-friendly description of this table.
    pub description: Option<String>,
    /// [Output-only] The time when this table was created, in milliseconds since the epoch.
    #[serde(rename="creationTime")]
    pub creation_time: Option<String>,
    /// [Experimental] The labels associated with this table. You can use these to organize and group your tables. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter and each label in the list must have a different key.
    pub labels: Option<HashMap<String, String>>,
    /// [Output-only] The size of this table in bytes, excluding any data in the streaming buffer.
    #[serde(rename="numBytes")]
    pub num_bytes: Option<String>,
    /// If specified, configures time-based partitioning for this table.
    #[serde(rename="timePartitioning")]
    pub time_partitioning: Option<TimePartitioning>,
    /// [Output-only] The time when this table was last modified, in milliseconds since the epoch.
    #[serde(rename="lastModifiedTime")]
    pub last_modified_time: Option<String>,
    /// [Output-only] The number of bytes in the table that are considered "long-term storage".
    #[serde(rename="numLongTermBytes")]
    pub num_long_term_bytes: Option<String>,
    /// [Output-only] An opaque ID uniquely identifying the table.
    pub id: Option<String>,
    /// [Experimental] Custom encryption configuration (e.g., Cloud KMS keys).
    #[serde(rename="encryptionConfiguration")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    /// [Output-only] The type of the resource.
    pub kind: Option<String>,
    /// [Output-only] Contains information regarding this table's streaming buffer, if one is present. This field will be absent if the table is not being streamed to or if there is no data in the streaming buffer.
    #[serde(rename="streamingBuffer")]
    pub streaming_buffer: Option<Streamingbuffer>,
    /// [Optional] Describes the data format, location, and other properties of a table stored outside of BigQuery. By defining these properties, the data source can then be queried as if it were a standard BigQuery table.
    #[serde(rename="externalDataConfiguration")]
    pub external_data_configuration: Option<ExternalDataConfiguration>,
    /// [Required] Reference describing the ID of this table.
    #[serde(rename="tableReference")]
    pub table_reference: Option<TableReference>,
    /// [Output-only] The number of rows of data in this table, excluding any data in the streaming buffer.
    #[serde(rename="numRows")]
    pub num_rows: Option<String>,
    /// [Output-only] A hash of this resource.
    pub etag: Option<String>,
    /// [Output-only] The geographic location where the table resides. This value is inherited from the dataset.
    pub location: Option<String>,
    /// [Optional] A descriptive name for this table.
    #[serde(rename="friendlyName")]
    pub friendly_name: Option<String>,
    /// [Optional] The time when this table expires, in milliseconds since the epoch. If not present, the table will persist indefinitely. Expired tables will be deleted and their storage reclaimed.
    #[serde(rename="expirationTime")]
    pub expiration_time: Option<String>,
    /// [Optional] The view definition.
    pub view: Option<ViewDefinition>,
    /// [Output-only] Describes the table type. The following values are supported: TABLE: A normal BigQuery table. VIEW: A virtual table defined by a SQL query. EXTERNAL: A table that references data stored in an external storage system, such as Google Cloud Storage. The default value is TABLE.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// [Output-only] A URL that can be used to access this resource again.
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
    /// [Optional] Describes the schema of this table.
    pub schema: Option<TableSchema>,
}

impl RequestValue for Table {}
impl Resource for Table {}
impl ResponseResult for Table {}


/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [insert all tabledata](struct.TabledataInsertAllCall.html) (response)
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableDataInsertAllResponse {
    /// The resource type of the response.
    pub kind: Option<String>,
    /// An array of errors for rows that were not inserted.
    #[serde(rename="insertErrors")]
    pub insert_errors: Option<Vec<TableDataInsertAllResponseInsertErrors>>,
}

impl ResponseResult for TableDataInsertAllResponse {}


/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [query jobs](struct.JobQueryCall.html) (response)
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryResponse {
    /// The resource type.
    pub kind: Option<String>,
    /// [Output-only] The first errors or warnings encountered during the running of the job. The final message includes the number of errors that caused the process to stop. Errors here do not necessarily mean that the job has completed or was unsuccessful.
    pub errors: Option<Vec<ErrorProto>>,
    /// Reference to the Job that was created to run the query. This field will be present even if the original request timed out, in which case GetQueryResults can be used to read the results once the query has completed. Since this API only returns the first page of results, subsequent pages can be fetched via the same mechanism (GetQueryResults).
    #[serde(rename="jobReference")]
    pub job_reference: Option<JobReference>,
    /// Whether the query result was fetched from the query cache.
    #[serde(rename="cacheHit")]
    pub cache_hit: Option<bool>,
    /// Whether the query has completed or not. If rows or totalRows are present, this will always be true. If this is false, totalRows will not be available.
    #[serde(rename="jobComplete")]
    pub job_complete: Option<bool>,
    /// The total number of rows in the complete query result set, which can be more than the number of rows in this single page of results.
    #[serde(rename="totalRows")]
    pub total_rows: Option<String>,
    /// The total number of bytes processed for this query. If this query was a dry run, this is the number of bytes that would be processed if the query were run.
    #[serde(rename="totalBytesProcessed")]
    pub total_bytes_processed: Option<String>,
    /// A token used for paging results.
    #[serde(rename="pageToken")]
    pub page_token: Option<String>,
    /// An object with as many results as can be contained within the maximum permitted reply size. To get any additional rows, you can call GetQueryResults and specify the jobReference returned above.
    pub rows: Option<Vec<TableRow>>,
    /// [Output-only] The number of rows affected by a DML statement. Present only for DML statements INSERT, UPDATE or DELETE.
    #[serde(rename="numDmlAffectedRows")]
    pub num_dml_affected_rows: Option<String>,
    /// The schema of the results. Present only when the query completes successfully.
    pub schema: Option<TableSchema>,
}

impl ResponseResult for QueryResponse {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobConfigurationTableCopy {
    /// [Pick one] Source tables to copy.
    #[serde(rename="sourceTables")]
    pub source_tables: Option<Vec<TableReference>>,
    /// [Optional] Specifies the action that occurs if the destination table already exists. The following values are supported: WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data. WRITE_APPEND: If the table already exists, BigQuery appends the data to the table. WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result. The default value is WRITE_EMPTY. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Creation, truncation and append actions occur as one atomic update upon job completion.
    #[serde(rename="writeDisposition")]
    pub write_disposition: Option<String>,
    /// [Experimental] Custom encryption configuration (e.g., Cloud KMS keys).
    #[serde(rename="destinationEncryptionConfiguration")]
    pub destination_encryption_configuration: Option<EncryptionConfiguration>,
    /// [Optional] Specifies whether the job is allowed to create new tables. The following values are supported: CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table. CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result. The default value is CREATE_IF_NEEDED. Creation, truncation and append actions occur as one atomic update upon job completion.
    #[serde(rename="createDisposition")]
    pub create_disposition: Option<String>,
    /// [Required] The destination table
    #[serde(rename="destinationTable")]
    pub destination_table: Option<TableReference>,
    /// [Pick one] Source table to copy.
    #[serde(rename="sourceTable")]
    pub source_table: Option<TableReference>,
}

impl Part for JobConfigurationTableCopy {}


/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSheetsOptions {
    /// [Optional] The number of rows at the top of a sheet that BigQuery will skip when reading the data. The default value is 0. This property is useful if you have header rows that should be skipped. When autodetect is on, behavior is the following: * skipLeadingRows unspecified - Autodetect tries to detect headers in the first row. If they are not detected, the row is read as data. Otherwise data is read starting from the second row. * skipLeadingRows is 0 - Instructs autodetect that there are no headers and data should be read starting from the first row. * skipLeadingRows = N > 0 - Autodetect skips N-1 rows and tries to detect headers in row N. If headers are not detected, row N is just skipped. Otherwise row N is used to extract column names for the detected schema.
    #[serde(rename="skipLeadingRows")]
    pub skip_leading_rows: Option<String>,
}

impl Part for GoogleSheetsOptions {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *table* resources.
/// It is not used directly, but through the `Bigquery` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_bigquery2 as bigquery2;
///
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use bigquery2::Bigquery;
///
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::client::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::)),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Bigquery::new(hyper::client::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::)), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.tables();
/// # }
/// ```
pub struct TableMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
}

impl<'a, C, A> MethodsBuilder for TableMethods<'a, C, A> {}

impl<'a, C, A> TableMethods<'a, C, A> {

    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified table resource by table ID. This method does not return the data in the table, it only returns the table resource, which describes the structure of this table.
    ///
    /// # Arguments
    ///
    /// * `projectId` - Project ID of the requested table
    /// * `datasetId` - Dataset ID of the requested table
    /// * `tableId` - Table ID of the requested table
    pub fn get(&self, project_id: &str, dataset_id: &str, table_id: &str) -> TableGetCall<'a, C, A> {
        TableGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _table_id: table_id.to_string(),
            _selected_fields: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Updates information in an existing table. The update method replaces the entire table resource, whereas the patch method only replaces fields that are provided in the submitted table resource. This method supports patch semantics.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Project ID of the table to update
    /// * `datasetId` - Dataset ID of the table to update
    /// * `tableId` - Table ID of the table to update
    pub fn patch(&self, request: Table, project_id: &str, dataset_id: &str, table_id: &str) -> TablePatchCall<'a, C, A> {
        TablePatchCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _table_id: table_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Updates information in an existing table. The update method replaces the entire table resource, whereas the patch method only replaces fields that are provided in the submitted table resource.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Project ID of the table to update
    /// * `datasetId` - Dataset ID of the table to update
    /// * `tableId` - Table ID of the table to update
    pub fn update(&self, request: Table, project_id: &str, dataset_id: &str, table_id: &str) -> TableUpdateCall<'a, C, A> {
        TableUpdateCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _table_id: table_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Lists all tables in the specified dataset. Requires the READER dataset role.
    ///
    /// # Arguments
    ///
    /// * `projectId` - Project ID of the tables to list
    /// * `datasetId` - Dataset ID of the tables to list
    pub fn list(&self, project_id: &str, dataset_id: &str) -> TableListCall<'a, C, A> {
        TableListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new, empty table in the dataset.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Project ID of the new table
    /// * `datasetId` - Dataset ID of the new table
    pub fn insert(&self, request: Table, project_id: &str, dataset_id: &str) -> TableInsertCall<'a, C, A> {
        TableInsertCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the table specified by tableId from the dataset. If the table contains data, all the data will be deleted.
    ///
    /// # Arguments
    ///
    /// * `projectId` - Project ID of the table to delete
    /// * `datasetId` - Dataset ID of the table to delete
    /// * `tableId` - Table ID of the table to delete
    pub fn delete(&self, project_id: &str, dataset_id: &str, table_id: &str) -> TableDeleteCall<'a, C, A> {
        TableDeleteCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _table_id: table_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *dataset* resources.
/// It is not used directly, but through the `Bigquery` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_bigquery2 as bigquery2;
///
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use bigquery2::Bigquery;
///
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::client::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::)),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Bigquery::new(hyper::client::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::)), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.datasets();
/// # }
/// ```
pub struct DatasetMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
}

impl<'a, C, A> MethodsBuilder for DatasetMethods<'a, C, A> {}

impl<'a, C, A> DatasetMethods<'a, C, A> {

    /// Create a builder to help you perform the following task:
    ///
    /// Lists all datasets in the specified project to which you have been granted the READER dataset role.
    ///
    /// # Arguments
    ///
    /// * `projectId` - Project ID of the datasets to be listed
    pub fn list(&self, project_id: &str) -> DatasetListCall<'a, C, A> {
        DatasetListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _all: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Returns the dataset specified by datasetID.
    ///
    /// # Arguments
    ///
    /// * `projectId` - Project ID of the requested dataset
    /// * `datasetId` - Dataset ID of the requested dataset
    pub fn get(&self, project_id: &str, dataset_id: &str) -> DatasetGetCall<'a, C, A> {
        DatasetGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Updates information in an existing dataset. The update method replaces the entire dataset resource, whereas the patch method only replaces fields that are provided in the submitted dataset resource. This method supports patch semantics.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Project ID of the dataset being updated
    /// * `datasetId` - Dataset ID of the dataset being updated
    pub fn patch(&self, request: Dataset, project_id: &str, dataset_id: &str) -> DatasetPatchCall<'a, C, A> {
        DatasetPatchCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the dataset specified by the datasetId value. Before you can delete a dataset, you must delete all its tables, either manually or by specifying deleteContents. Immediately after deletion, you can create another dataset with the same name.
    ///
    /// # Arguments
    ///
    /// * `projectId` - Project ID of the dataset being deleted
    /// * `datasetId` - Dataset ID of dataset being deleted
    pub fn delete(&self, project_id: &str, dataset_id: &str) -> DatasetDeleteCall<'a, C, A> {
        DatasetDeleteCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _delete_contents: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Updates information in an existing dataset. The update method replaces the entire dataset resource, whereas the patch method only replaces fields that are provided in the submitted dataset resource.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Project ID of the dataset being updated
    /// * `datasetId` - Dataset ID of the dataset being updated
    pub fn update(&self, request: Dataset, project_id: &str, dataset_id: &str) -> DatasetUpdateCall<'a, C, A> {
        DatasetUpdateCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new empty dataset.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Project ID of the new dataset
    pub fn insert(&self, request: Dataset, project_id: &str) -> DatasetInsertCall<'a, C, A> {
        DatasetInsertCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *job* resources.
/// It is not used directly, but through the `Bigquery` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_bigquery2 as bigquery2;
///
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use bigquery2::Bigquery;
///
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::client::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::)),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Bigquery::new(hyper::client::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::)), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `cancel(...)`, `get(...)`, `get_query_results(...)`, `insert(...)`, `list(...)` and `query(...)`
/// // to build up your call.
/// let rb = hub.jobs();
/// # }
/// ```
pub struct JobMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
}

impl<'a, C, A> MethodsBuilder for JobMethods<'a, C, A> {}

impl<'a, C, A> JobMethods<'a, C, A> {

    /// Create a builder to help you perform the following task:
    ///
    /// Returns information about a specific job. Job information is available for a six month period after creation. Requires that you're the person who ran the job, or have the Is Owner project role.
    ///
    /// # Arguments
    ///
    /// * `projectId` - [Required] Project ID of the requested job
    /// * `jobId` - [Required] Job ID of the requested job
    pub fn get(&self, project_id: &str, job_id: &str) -> JobGetCall<'a, C, A> {
        JobGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _job_id: job_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Requests that a job be cancelled. This call will return immediately, and the client will need to poll for the job status to see if the cancel completed successfully. Cancelled jobs may still incur costs.
    ///
    /// # Arguments
    ///
    /// * `projectId` - [Required] Project ID of the job to cancel
    /// * `jobId` - [Required] Job ID of the job to cancel
    pub fn cancel(&self, project_id: &str, job_id: &str) -> JobCancelCall<'a, C, A> {
        JobCancelCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _job_id: job_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Runs a BigQuery SQL query synchronously and returns query results if the query completes within a specified timeout.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Project ID of the project billed for the query
    pub fn query(&self, request: QueryRequest, project_id: &str) -> JobQueryCall<'a, C, A> {
        JobQueryCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Starts a new asynchronous job. Requires the Can View project role.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Project ID of the project that will be billed for the job
    pub fn insert(&self, request: Job, project_id: &str) -> JobInsertCall<'a, C, A> {
        JobInsertCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Lists all jobs that you started in the specified project. Job information is available for a six month period after creation. The job list is sorted in reverse chronological order, by job creation time. Requires the Can View project role, or the Is Owner project role if you set the allUsers property.
    ///
    /// # Arguments
    ///
    /// * `projectId` - Project ID of the jobs to list
    pub fn list(&self, project_id: &str) -> JobListCall<'a, C, A> {
        JobListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _state_filter: Default::default(),
            _projection: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _all_users: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the results of a query job.
    ///
    /// # Arguments
    ///
    /// * `projectId` - [Required] Project ID of the query job
    /// * `jobId` - [Required] Job ID of the query job
    pub fn get_query_results(&self, project_id: &str, job_id: &str) -> JobGetQueryResultCall<'a, C, A> {
        JobGetQueryResultCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _job_id: job_id.to_string(),
            _timeout_ms: Default::default(),
            _start_index: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *tabledata* resources.
/// It is not used directly, but through the `Bigquery` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_bigquery2 as bigquery2;
///
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use bigquery2::Bigquery;
///
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::client::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::)),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Bigquery::new(hyper::client::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::)), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `insert_all(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.tabledata();
/// # }
/// ```
pub struct TabledataMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
}

impl<'a, C, A> MethodsBuilder for TabledataMethods<'a, C, A> {}

impl<'a, C, A> TabledataMethods<'a, C, A> {

    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves table data from a specified set of rows. Requires the READER dataset role.
    ///
    /// # Arguments
    ///
    /// * `projectId` - Project ID of the table to read
    /// * `datasetId` - Dataset ID of the table to read
    /// * `tableId` - Table ID of the table to read
    pub fn list(&self, project_id: &str, dataset_id: &str, table_id: &str) -> TabledataListCall<'a, C, A> {
        TabledataListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _table_id: table_id.to_string(),
            _start_index: Default::default(),
            _selected_fields: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Streams data into BigQuery one record at a time without needing to run a load job. Requires the WRITER dataset role.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Project ID of the destination table.
    /// * `datasetId` - Dataset ID of the destination table.
    /// * `tableId` - Table ID of the destination table.
    pub fn insert_all(&self, request: TableDataInsertAllRequest, project_id: &str, dataset_id: &str, table_id: &str) -> TabledataInsertAllCall<'a, C, A> {
        TabledataInsertAllCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _table_id: table_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the `Bigquery` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_bigquery2 as bigquery2;
///
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use bigquery2::Bigquery;
///
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::client::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::)),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Bigquery::new(hyper::client::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::)), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_service_account(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
}

impl<'a, C, A> MethodsBuilder for ProjectMethods<'a, C, A> {}

impl<'a, C, A> ProjectMethods<'a, C, A> {

    /// Create a builder to help you perform the following task:
    ///
    /// Returns the email address of the service account for your project used for interactions with Google Cloud KMS.
    ///
    /// # Arguments
    ///
    /// * `projectId` - Project ID for which the service account is requested.
    pub fn get_service_account(&self, project_id: &str) -> ProjectGetServiceAccountCall<'a, C, A> {
        ProjectGetServiceAccountCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Lists all projects to which you have been granted any project role.
    pub fn list(&self) -> ProjectListCall<'a, C, A> {
        ProjectListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Gets the specified table resource by table ID. This method does not return the data in the table, it only returns the table resource, which describes the structure of this table.
///
/// A builder for the *get* method supported by a *table* resource.
/// It is not used directly, but through a `TableMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_bigquery2 as bigquery2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use bigquery2::Bigquery;
///
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::client::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::)),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Bigquery::new(hyper::client::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::)), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.tables().get("projectId", "datasetId", "tableId")
///              .selected_fields("labore")
///              .doit();
/// # }
/// ```
pub struct TableGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
    _project_id: String,
    _dataset_id: String,
    _table_id: String,
    _selected_fields: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for TableGetCall<'a, C, A> {}

impl<'a, C, A> TableGetCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, Table)> {
        use std::io::{Read, Seek};
        use hyper::Body;
        use hyper::client::Client;
        use hyper::header::{HeaderValue, CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "bigquery.tables.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        params.push(("datasetId", self._dataset_id.to_string()));
        params.push(("tableId", self._table_id.to_string()));
        if let Some(value) = self._selected_fields {
            params.push(("selectedFields", value.to_string()));
        }
        for &field in ["alt", "projectId", "datasetId", "tableId", "selectedFields"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "projects/{projectId}/datasets/{datasetId}/tables/{tableId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{datasetId}", "datasetId"), ("{tableId}", "tableId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["tableId", "datasetId", "projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



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
            let auth_header = HeaderValue::from_str(&token.access_token).unwrap();
            let mut req_result = {
                let mut hub_mut = self.hub.client.borrow_mut();
                let mut client = hub_mut.deref_mut().borrow_mut();
                let mut req = hyper::Request::get(&url)
                    .header(USER_AGENT, HeaderValue::from_str(&self.hub._user_agent).unwrap())
                    .header(AUTHORIZATION, auth_header.clone())
                    .body(Body::empty())
                    .unwrap();

                dlg.pre_request();

                // Get Future
                let fut = client.request(req);

                // Get response
                match fut.wait() {
                    Ok(i) => Ok(i),
                    Err(e) => Err(e),
                }
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
                    if !res.status().is_success() {
                        let json_err = cmn::read_to_string(&res).unwrap();
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
                        let json_response = cmn::read_to_string(&res).unwrap();
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


    /// Project ID of the requested table
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> TableGetCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Dataset ID of the requested table
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> TableGetCall<'a, C, A> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Table ID of the requested table
    ///
    /// Sets the *table id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn table_id(mut self, new_value: &str) -> TableGetCall<'a, C, A> {
        self._table_id = new_value.to_string();
        self
    }
    /// List of fields to return (comma-separated). If unspecified, all fields are returned
    ///
    /// Sets the *selected fields* query property to the given value.
    pub fn selected_fields(mut self, new_value: &str) -> TableGetCall<'a, C, A> {
        self._selected_fields = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TableGetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> TableGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> TableGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates information in an existing table. The update method replaces the entire table resource, whereas the patch method only replaces fields that are provided in the submitted table resource. This method supports patch semantics.
///
/// A builder for the *patch* method supported by a *table* resource.
/// It is not used directly, but through a `TableMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::Table;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use bigquery2::Bigquery;
///
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::client::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::)),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Bigquery::new(hyper::client::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::)), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Table::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.tables().patch(req, "projectId", "datasetId", "tableId")
///              .doit();
/// # }
/// ```
pub struct TablePatchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
    _request: Table,
    _project_id: String,
    _dataset_id: String,
    _table_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for TablePatchCall<'a, C, A> {}

impl<'a, C, A> TablePatchCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, Table)> {
        use std::io::{Read, Seek};
        use hyper::Body;
        use hyper::client::Client;
        use hyper::header::{HeaderValue, CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "bigquery.tables.patch",
                               http_method: hyper::Method::PATCH });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        params.push(("datasetId", self._dataset_id.to_string()));
        params.push(("tableId", self._table_id.to_string()));
        for &field in ["alt", "projectId", "datasetId", "tableId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "projects/{projectId}/datasets/{datasetId}/tables/{tableId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{datasetId}", "datasetId"), ("{tableId}", "tableId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["tableId", "datasetId", "projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::APPLICATION_JSON;
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
            let auth_header = HeaderValue::from_str(&token.access_token).unwrap();
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut hub_mut = self.hub.client.borrow_mut();
                let mut client = hub_mut.deref_mut().borrow_mut();

                // Get contents of Cursor
                let inside_cursor = request_value_reader.clone().into_inner();
                //Build a stream
                let stream = futures::stream::iter_ok::<_, ::std::io::Error>(vec![inside_cursor]);
                // Wrap stream in a box inside a body
                let body = Body::wrap_stream(stream);

                // build a request
                let mut req = hyper::Request::patch(&url)
                    .header(USER_AGENT, HeaderValue::from_str(&self.hub._user_agent).unwrap())
                    .header(AUTHORIZATION, auth_header.clone())
                    .header(CONTENT_TYPE, HeaderValue::from_str(&json_mime_type.to_string()).unwrap())
                    .header(CONTENT_LENGTH, HeaderValue::from_str(&request_size.to_string()).unwrap())
                    .body(body)
                    .unwrap();

                dlg.pre_request();
                // Get Future
                let fut = client.request(req);

                // Get response
                match fut.wait() {
                    Ok(i) => Ok(i),
                    Err(e) => Err(e),
                }
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
                    if !res.status().is_success() {
                        let json_err = cmn::read_to_string(&res).unwrap();
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
                        let json_response = cmn::read_to_string(&res).unwrap();
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
    pub fn request(mut self, new_value: Table) -> TablePatchCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Project ID of the table to update
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> TablePatchCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Dataset ID of the table to update
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> TablePatchCall<'a, C, A> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Table ID of the table to update
    ///
    /// Sets the *table id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn table_id(mut self, new_value: &str) -> TablePatchCall<'a, C, A> {
        self._table_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TablePatchCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> TablePatchCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> TablePatchCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates information in an existing table. The update method replaces the entire table resource, whereas the patch method only replaces fields that are provided in the submitted table resource.
///
/// A builder for the *update* method supported by a *table* resource.
/// It is not used directly, but through a `TableMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::Table;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use bigquery2::Bigquery;
///
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::client::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Bigquery::new(hyper::client::Client::new()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Table::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.tables().update(req, "projectId", "datasetId", "tableId")
///              .doit();
/// # }
/// ```
pub struct TableUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
    _request: Table,
    _project_id: String,
    _dataset_id: String,
    _table_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for TableUpdateCall<'a, C, A> {}

impl<'a, C, A> TableUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, Table)> {
        use std::io::{Read, Seek};
        use hyper::Body;
        use hyper::client::Client;
        use hyper::header::{HeaderValue, CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "bigquery.tables.update",
                               http_method: hyper::Method::PUT });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        params.push(("datasetId", self._dataset_id.to_string()));
        params.push(("tableId", self._table_id.to_string()));
        for &field in ["alt", "projectId", "datasetId", "tableId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "projects/{projectId}/datasets/{datasetId}/tables/{tableId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{datasetId}", "datasetId"), ("{tableId}", "tableId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["tableId", "datasetId", "projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::APPLICATION_JSON;
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
            let auth_header = HeaderValue::from_str(&token.access_token).unwrap();
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut hub_mut = self.hub.client.borrow_mut();
                let mut client = hub_mut.deref_mut().borrow_mut();

                // Get contents of Cursor
                let inside_cursor = request_value_reader.clone().into_inner();
                //Build a stream
                let stream = futures::stream::iter_ok::<_, ::std::io::Error>(vec![inside_cursor]);
                // Wrap stream in a box inside a body
                let body = Body::wrap_stream(stream);

                // Build a request
                let mut req = hyper::Request::put(&url)
                    .header(USER_AGENT, HeaderValue::from_str(&self.hub._user_agent).unwrap())
                    .header(AUTHORIZATION, auth_header.clone())
                    .header(CONTENT_TYPE, HeaderValue::from_str(&json_mime_type.to_string()).unwrap())
                    .header(CONTENT_LENGTH, HeaderValue::from_str(&request_size.to_string()).unwrap())
                    .body(body)
                    .unwrap();

                dlg.pre_request();
                // Get Future
                let fut = client.request(req);

                // Get response
                match fut.wait() {
                    Ok(i) => Ok(i),
                    Err(e) => Err(e),
                }
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
                    if !res.status().is_success() {
                        let json_err = cmn::read_to_string(&res).unwrap();
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
                        let json_response = cmn::read_to_string(&res).unwrap();
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
    pub fn request(mut self, new_value: Table) -> TableUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Project ID of the table to update
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> TableUpdateCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Dataset ID of the table to update
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> TableUpdateCall<'a, C, A> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Table ID of the table to update
    ///
    /// Sets the *table id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn table_id(mut self, new_value: &str) -> TableUpdateCall<'a, C, A> {
        self._table_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TableUpdateCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> TableUpdateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> TableUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists all tables in the specified dataset. Requires the READER dataset role.
///
/// A builder for the *list* method supported by a *table* resource.
/// It is not used directly, but through a `TableMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_bigquery2 as bigquery2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use bigquery2::Bigquery;
///
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::client::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Bigquery::new(hyper::client::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.tables().list("projectId", "datasetId")
///              .page_token("justo")
///              .max_results(80)
///              .doit();
/// # }
/// ```
pub struct TableListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
    _project_id: String,
    _dataset_id: String,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for TableListCall<'a, C, A> {}

impl<'a, C, A> TableListCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, TableList)> {
        use std::io::{Read, Seek};
        use hyper::Body;
        use hyper::client::Client;
        use hyper::header::{HeaderValue, CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "bigquery.tables.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        params.push(("datasetId", self._dataset_id.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        for &field in ["alt", "projectId", "datasetId", "pageToken", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "projects/{projectId}/datasets/{datasetId}/tables";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{datasetId}", "datasetId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["datasetId", "projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



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
            let auth_header = HeaderValue::from_str(&token.access_token).unwrap();
            let mut req_result = {
                let mut hub_mut = self.hub.client.borrow_mut();
                let mut client = hub_mut.deref_mut().borrow_mut();
                let mut req = hyper::Request::get(&url)
                    .header(USER_AGENT, HeaderValue::from_str(&self.hub._user_agent).unwrap())
                    .header(AUTHORIZATION, auth_header.clone())
                    .body(Body::empty())
                    .unwrap();

                dlg.pre_request();
                // Get Future
                let fut = client.request(req);

                // Get response
                match fut.wait() {
                    Ok(i) => Ok(i),
                    Err(e) => Err(e),
                }
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
                    if !res.status().is_success() {
                        let json_err = cmn::read_to_string(&res).unwrap();
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
                        let json_response = cmn::read_to_string(&res).unwrap();
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


    /// Project ID of the tables to list
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> TableListCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Dataset ID of the tables to list
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> TableListCall<'a, C, A> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Page token, returned by a previous call, to request the next page of results
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> TableListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> TableListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TableListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> TableListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> TableListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a new, empty table in the dataset.
///
/// A builder for the *insert* method supported by a *table* resource.
/// It is not used directly, but through a `TableMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::Table;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use bigquery2::Bigquery;
///
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::client::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Bigquery::new(hyper::client::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Table::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.tables().insert(req, "projectId", "datasetId")
///              .doit();
/// # }
/// ```
pub struct TableInsertCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
    _request: Table,
    _project_id: String,
    _dataset_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for TableInsertCall<'a, C, A> {}

impl<'a, C, A> TableInsertCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, Table)> {
        use std::io::{Read, Seek};
        use hyper::Body;
        use hyper::client::Client;
        use hyper::header::{HeaderValue, CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "bigquery.tables.insert",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        params.push(("datasetId", self._dataset_id.to_string()));
        for &field in ["alt", "projectId", "datasetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "projects/{projectId}/datasets/{datasetId}/tables";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{datasetId}", "datasetId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["datasetId", "projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::APPLICATION_JSON;
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
            let auth_header = HeaderValue::from_str(&token.access_token).unwrap();
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut hub_mut = self.hub.client.borrow_mut();
                let mut client = hub_mut.deref_mut().borrow_mut();

                // Get contents of Cursor
                let inside_cursor = request_value_reader.clone().into_inner();
                //Build a stream
                let stream = futures::stream::iter_ok::<_, ::std::io::Error>(vec![inside_cursor]);
                // Wrap stream in a box inside a body
                let body = Body::wrap_stream(stream);

                // Build a request
                let mut req = hyper::Request::post(&url)
                    .header(USER_AGENT, HeaderValue::from_str(&self.hub._user_agent).unwrap())
                    .header(AUTHORIZATION, auth_header.clone())
                    .header(CONTENT_TYPE, HeaderValue::from_str(&json_mime_type.to_string()).unwrap())
                    .header(CONTENT_LENGTH, HeaderValue::from_str(&request_size.to_string()).unwrap())
                    .body(body)
                    .unwrap();

                dlg.pre_request();
                // Get Future
                let fut = client.request(req);

                // Get response
                match fut.wait() {
                    Ok(i) => Ok(i),
                    Err(e) => Err(e),
                }
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
                    if !res.status().is_success() {
                        let json_err = cmn::read_to_string(&res).unwrap();
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
                        let json_response = cmn::read_to_string(&res).unwrap();
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
    pub fn request(mut self, new_value: Table) -> TableInsertCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Project ID of the new table
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> TableInsertCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Dataset ID of the new table
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> TableInsertCall<'a, C, A> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TableInsertCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> TableInsertCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> TableInsertCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes the table specified by tableId from the dataset. If the table contains data, all the data will be deleted.
///
/// A builder for the *delete* method supported by a *table* resource.
/// It is not used directly, but through a `TableMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_bigquery2 as bigquery2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use bigquery2::Bigquery;
///
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::client::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Bigquery::new(hyper::client::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.tables().delete("projectId", "datasetId", "tableId")
///              .doit();
/// # }
/// ```
pub struct TableDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
    _project_id: String,
    _dataset_id: String,
    _table_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for TableDeleteCall<'a, C, A> {}

impl<'a, C, A> TableDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::Response<hyper::Body>> {
        use std::io::{Read, Seek};
        use hyper::Body;
        use hyper::client::Client;
        use hyper::header::{HeaderValue, CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "bigquery.tables.delete",
                               http_method: hyper::Method::DELETE });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        params.push(("datasetId", self._dataset_id.to_string()));
        params.push(("tableId", self._table_id.to_string()));
        for &field in ["projectId", "datasetId", "tableId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "projects/{projectId}/datasets/{datasetId}/tables/{tableId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{datasetId}", "datasetId"), ("{tableId}", "tableId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["tableId", "datasetId", "projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



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
            let auth_header = HeaderValue::from_str(&token.access_token).unwrap();
            let mut req_result = {
                let mut hub_mut = self.hub.client.borrow_mut();
                let mut client = hub_mut.deref_mut().borrow_mut();
                let mut req = hyper::Request::delete(&url)
                    .header(USER_AGENT, HeaderValue::from_str(&self.hub._user_agent).unwrap())
                    .header(AUTHORIZATION, auth_header.clone())
                    .body(Body::empty())
                    .unwrap();

                dlg.pre_request();
                // Get Future
                let fut = client.request(req);

                // Get response
                match fut.wait() {
                    Ok(i) => Ok(i),
                    Err(e) => Err(e),
                }
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
                    if !res.status().is_success() {
                        let json_err = cmn::read_to_string(&res).unwrap();
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Project ID of the table to delete
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> TableDeleteCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Dataset ID of the table to delete
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> TableDeleteCall<'a, C, A> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Table ID of the table to delete
    ///
    /// Sets the *table id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn table_id(mut self, new_value: &str) -> TableDeleteCall<'a, C, A> {
        self._table_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TableDeleteCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> TableDeleteCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> TableDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists all datasets in the specified project to which you have been granted the READER dataset role.
///
/// A builder for the *list* method supported by a *dataset* resource.
/// It is not used directly, but through a `DatasetMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_bigquery2 as bigquery2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use bigquery2::Bigquery;
///
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::client::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Bigquery::new(hyper::client::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.datasets().list("projectId")
///              .page_token("duo")
///              .max_results(69)
///              .filter("sea")
///              .all(false)
///              .doit();
/// # }
/// ```
pub struct DatasetListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
    _project_id: String,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _filter: Option<String>,
    _all: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DatasetListCall<'a, C, A> {}

impl<'a, C, A> DatasetListCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, DatasetList)> {
        use std::io::{Read, Seek};
        use hyper::Body;
        use hyper::client::Client;
        use hyper::header::{HeaderValue, CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "bigquery.datasets.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._filter {
            params.push(("filter", value.to_string()));
        }
        if let Some(value) = self._all {
            params.push(("all", value.to_string()));
        }
        for &field in ["alt", "projectId", "pageToken", "maxResults", "filter", "all"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "projects/{projectId}/datasets";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId")].iter() {
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
            for param_name in ["projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



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
            let auth_header = HeaderValue::from_str(&token.access_token).unwrap();
            let mut req_result = {
                let mut hub_mut = self.hub.client.borrow_mut();
                let mut client = hub_mut.deref_mut().borrow_mut();
                let mut req = hyper::Request::get(&url)
                    .header(USER_AGENT, HeaderValue::from_str(&self.hub._user_agent).unwrap())
                    .header(AUTHORIZATION, auth_header.clone())
                    .body(Body::empty())
                    .unwrap();

                dlg.pre_request();
                // Get Future
                let fut = client.request(req);

                // Get response
                match fut.wait() {
                    Ok(i) => Ok(i),
                    Err(e) => Err(e),
                }
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
                    if !res.status().is_success() {
                        let json_err = cmn::read_to_string(&res).unwrap();
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
                        let json_response = cmn::read_to_string(&res).unwrap();
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


    /// Project ID of the datasets to be listed
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> DatasetListCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Page token, returned by a previous call, to request the next page of results
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> DatasetListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of results to return
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> DatasetListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// An expression for filtering the results of the request by label. The syntax is "labels.<name>[:<value>]". Multiple filters can be ANDed together by connecting with a space. Example: "labels.department:receiving labels.active". See Filtering datasets using labels for details.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> DatasetListCall<'a, C, A> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// Whether to list all datasets, including hidden ones
    ///
    /// Sets the *all* query property to the given value.
    pub fn all(mut self, new_value: bool) -> DatasetListCall<'a, C, A> {
        self._all = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DatasetListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DatasetListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DatasetListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns the dataset specified by datasetID.
///
/// A builder for the *get* method supported by a *dataset* resource.
/// It is not used directly, but through a `DatasetMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_bigquery2 as bigquery2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use bigquery2::Bigquery;
///
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::client::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Bigquery::new(hyper::client::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.datasets().get("projectId", "datasetId")
///              .doit();
/// # }
/// ```
pub struct DatasetGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
    _project_id: String,
    _dataset_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DatasetGetCall<'a, C, A> {}

impl<'a, C, A> DatasetGetCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, Dataset)> {
        use std::io::{Read, Seek};
        use hyper::Body;
        use hyper::client::Client;
        use hyper::header::{HeaderValue, CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "bigquery.datasets.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        params.push(("datasetId", self._dataset_id.to_string()));
        for &field in ["alt", "projectId", "datasetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "projects/{projectId}/datasets/{datasetId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{datasetId}", "datasetId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["datasetId", "projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



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
            let auth_header = HeaderValue::from_str(&token.access_token).unwrap();
            let mut req_result = {
                let mut hub_mut = self.hub.client.borrow_mut();
                let mut client = hub_mut.deref_mut().borrow_mut();
                let mut req = hyper::Request::get(&url)
                    .header(USER_AGENT, HeaderValue::from_str(&self.hub._user_agent).unwrap())
                    .header(AUTHORIZATION, auth_header.clone())
                    .body(Body::empty())
                    .unwrap();

                dlg.pre_request();
                // Get Future
                let fut = client.request(req);

                // Get response
                match fut.wait() {
                    Ok(i) => Ok(i),
                    Err(e) => Err(e),
                }
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
                    if !res.status().is_success() {
                        let json_err = cmn::read_to_string(&res).unwrap();
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
                        let json_response = cmn::read_to_string(&res).unwrap();
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


    /// Project ID of the requested dataset
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> DatasetGetCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Dataset ID of the requested dataset
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> DatasetGetCall<'a, C, A> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DatasetGetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DatasetGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DatasetGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates information in an existing dataset. The update method replaces the entire dataset resource, whereas the patch method only replaces fields that are provided in the submitted dataset resource. This method supports patch semantics.
///
/// A builder for the *patch* method supported by a *dataset* resource.
/// It is not used directly, but through a `DatasetMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::Dataset;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use bigquery2::Bigquery;
///
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::client::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Bigquery::new(hyper::client::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Dataset::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.datasets().patch(req, "projectId", "datasetId")
///              .doit();
/// # }
/// ```
pub struct DatasetPatchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
    _request: Dataset,
    _project_id: String,
    _dataset_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DatasetPatchCall<'a, C, A> {}

impl<'a, C, A> DatasetPatchCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, Dataset)> {
        use std::io::{Read, Seek};
        use hyper::Body;
        use hyper::client::Client;
        use hyper::header::{HeaderValue, CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "bigquery.datasets.patch",
                               http_method: hyper::Method::PATCH });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        params.push(("datasetId", self._dataset_id.to_string()));
        for &field in ["alt", "projectId", "datasetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "projects/{projectId}/datasets/{datasetId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{datasetId}", "datasetId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["datasetId", "projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::APPLICATION_JSON;
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
            let auth_header = HeaderValue::from_str(&token.access_token).unwrap();
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut hub_mut = self.hub.client.borrow_mut();
                let mut client = hub_mut.deref_mut().borrow_mut();

                // Get contents of Cursor
                let inside_cursor = request_value_reader.clone().into_inner();
                //Build a stream
                let stream = futures::stream::iter_ok::<_, ::std::io::Error>(vec![inside_cursor]);
                // Wrap stream in a box inside a body
                let body = Body::wrap_stream(stream);

                // Build a request
                let mut req = hyper::Request::patch(&url)
                    .header(USER_AGENT, HeaderValue::from_str(&self.hub._user_agent).unwrap())
                    .header(AUTHORIZATION, auth_header.clone())
                    .header(CONTENT_TYPE, HeaderValue::from_str(&json_mime_type.to_string()).unwrap())
                    .header(CONTENT_LENGTH, HeaderValue::from_str(&request_size.to_string()).unwrap())
                    .body(body)
                    .unwrap();

                dlg.pre_request();
                // Get Future
                let fut = client.request(req);

                // Get response
                match fut.wait() {
                    Ok(i) => Ok(i),
                    Err(e) => Err(e),
                }
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
                    if !res.status().is_success() {
                        let json_err = cmn::read_to_string(&res).unwrap();
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
                        let json_response = cmn::read_to_string(&res).unwrap();
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
    pub fn request(mut self, new_value: Dataset) -> DatasetPatchCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Project ID of the dataset being updated
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> DatasetPatchCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Dataset ID of the dataset being updated
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> DatasetPatchCall<'a, C, A> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DatasetPatchCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DatasetPatchCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DatasetPatchCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes the dataset specified by the datasetId value. Before you can delete a dataset, you must delete all its tables, either manually or by specifying deleteContents. Immediately after deletion, you can create another dataset with the same name.
///
/// A builder for the *delete* method supported by a *dataset* resource.
/// It is not used directly, but through a `DatasetMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_bigquery2 as bigquery2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use bigquery2::Bigquery;
///
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::client::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Bigquery::new(hyper::client::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.datasets().delete("projectId", "datasetId")
///              .delete_contents(false)
///              .doit();
/// # }
/// ```
pub struct DatasetDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
    _project_id: String,
    _dataset_id: String,
    _delete_contents: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DatasetDeleteCall<'a, C, A> {}

impl<'a, C, A> DatasetDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::Response<hyper::Body>> {
        use std::io::{Read, Seek};
        use hyper::Body;
        use hyper::client::Client;
        use hyper::header::{HeaderValue, CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "bigquery.datasets.delete",
                               http_method: hyper::Method::DELETE });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        params.push(("datasetId", self._dataset_id.to_string()));
        if let Some(value) = self._delete_contents {
            params.push(("deleteContents", value.to_string()));
        }
        for &field in ["projectId", "datasetId", "deleteContents"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "projects/{projectId}/datasets/{datasetId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{datasetId}", "datasetId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["datasetId", "projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



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
            let auth_header = HeaderValue::from_str(&token.access_token).unwrap();
            let mut req_result = {
                let mut hub_mut = self.hub.client.borrow_mut();
                let mut client = hub_mut.deref_mut().borrow_mut();

                let mut req = hyper::Request::delete(&url)
                    .header(USER_AGENT, HeaderValue::from_str(&self.hub._user_agent).unwrap())
                    .header(AUTHORIZATION, auth_header.clone())
                    .body(Body::empty())
                    .unwrap();

                dlg.pre_request();
                // Get Future
                let fut = client.request(req);

                // Get response
                match fut.wait() {
                    Ok(i) => Ok(i),
                    Err(e) => Err(e),
                }
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
                    if !res.status().is_success() {
                        let json_err = cmn::read_to_string(&res).unwrap();
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Project ID of the dataset being deleted
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> DatasetDeleteCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Dataset ID of dataset being deleted
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> DatasetDeleteCall<'a, C, A> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// If True, delete all the tables in the dataset. If False and the dataset contains tables, the request will fail. Default is False
    ///
    /// Sets the *delete contents* query property to the given value.
    pub fn delete_contents(mut self, new_value: bool) -> DatasetDeleteCall<'a, C, A> {
        self._delete_contents = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DatasetDeleteCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DatasetDeleteCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DatasetDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates information in an existing dataset. The update method replaces the entire dataset resource, whereas the patch method only replaces fields that are provided in the submitted dataset resource.
///
/// A builder for the *update* method supported by a *dataset* resource.
/// It is not used directly, but through a `DatasetMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::Dataset;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use bigquery2::Bigquery;
///
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::client::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Bigquery::new(hyper::client::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Dataset::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.datasets().update(req, "projectId", "datasetId")
///              .doit();
/// # }
/// ```
pub struct DatasetUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
    _request: Dataset,
    _project_id: String,
    _dataset_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DatasetUpdateCall<'a, C, A> {}

impl<'a, C, A> DatasetUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, Dataset)> {
        use std::io::{Read, Seek};
        use hyper::Body;
        use hyper::client::Client;
        use hyper::header::{HeaderValue, CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "bigquery.datasets.update",
                               http_method: hyper::Method::PUT });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        params.push(("datasetId", self._dataset_id.to_string()));
        for &field in ["alt", "projectId", "datasetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "projects/{projectId}/datasets/{datasetId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{datasetId}", "datasetId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["datasetId", "projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::APPLICATION_JSON;
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
            let auth_header = HeaderValue::from_str(&token.access_token).unwrap();
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut hub_mut = self.hub.client.borrow_mut();
                let mut client = hub_mut.deref_mut().borrow_mut();

                // Get contents of Cursor
                let inside_cursor = request_value_reader.clone().into_inner();
                //Build a stream
                let stream = futures::stream::iter_ok::<_, ::std::io::Error>(vec![inside_cursor]);
                // Wrap stream in a box inside a body
                let body = Body::wrap_stream(stream);

                // Build a request
                let mut req = hyper::Request::put(&url)
                    .header(USER_AGENT, HeaderValue::from_str(&self.hub._user_agent).unwrap())
                    .header(AUTHORIZATION, auth_header.clone())
                    .header(CONTENT_TYPE, HeaderValue::from_str(&json_mime_type.to_string()).unwrap())
                    .header(CONTENT_LENGTH, HeaderValue::from_str(&request_size.to_string()).unwrap())
                    .body(body)
                    .unwrap();

                dlg.pre_request();
                // Get Future
                let fut = client.request(req);

                // Get response
                match fut.wait() {
                    Ok(i) => Ok(i),
                    Err(e) => Err(e),
                }
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
                    if !res.status().is_success() {
                        let json_err = cmn::read_to_string(&res).unwrap();
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
                        let json_response = cmn::read_to_string(&res).unwrap();
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
    pub fn request(mut self, new_value: Dataset) -> DatasetUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Project ID of the dataset being updated
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> DatasetUpdateCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Dataset ID of the dataset being updated
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> DatasetUpdateCall<'a, C, A> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DatasetUpdateCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DatasetUpdateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DatasetUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a new empty dataset.
///
/// A builder for the *insert* method supported by a *dataset* resource.
/// It is not used directly, but through a `DatasetMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::Dataset;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use bigquery2::Bigquery;
///
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::client::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Bigquery::new(hyper::client::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Dataset::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.datasets().insert(req, "projectId")
///              .doit();
/// # }
/// ```
pub struct DatasetInsertCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
    _request: Dataset,
    _project_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DatasetInsertCall<'a, C, A> {}

impl<'a, C, A> DatasetInsertCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, Dataset)> {
        use std::io::{Read, Seek};
        use hyper::Body;
        use hyper::client::Client;
        use hyper::header::{HeaderValue, CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "bigquery.datasets.insert",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        for &field in ["alt", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "projects/{projectId}/datasets";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId")].iter() {
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
            for param_name in ["projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::APPLICATION_JSON;
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
            let auth_header = HeaderValue::from_str(&token.access_token).unwrap();
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut hub_mut = self.hub.client.borrow_mut();
                let mut client = hub_mut.deref_mut().borrow_mut();

                // Get contents of Cursor
                let inside_cursor = request_value_reader.clone().into_inner();
                //Build a stream
                let stream = futures::stream::iter_ok::<_, ::std::io::Error>(vec![inside_cursor]);
                // Wrap stream in a box inside a body
                let body = Body::wrap_stream(stream);

                // Build a request
                let mut req = hyper::Request::post(&url)
                    .header(USER_AGENT, HeaderValue::from_str(&self.hub._user_agent).unwrap())
                    .header(AUTHORIZATION, auth_header.clone())
                    .header(CONTENT_TYPE, HeaderValue::from_str(&json_mime_type.to_string()).unwrap())
                    .header(CONTENT_LENGTH, HeaderValue::from_str(&request_size.to_string()).unwrap())
                    .body(body)
                    .unwrap();

                dlg.pre_request();
                // Get Future
                let fut = client.request(req);

                // Get response
                match fut.wait() {
                    Ok(i) => Ok(i),
                    Err(e) => Err(e),
                }
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
                    if !res.status().is_success() {
                        let json_err = cmn::read_to_string(&res).unwrap();
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
                        let json_response = cmn::read_to_string(&res).unwrap();
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
    pub fn request(mut self, new_value: Dataset) -> DatasetInsertCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Project ID of the new dataset
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> DatasetInsertCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DatasetInsertCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DatasetInsertCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DatasetInsertCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns information about a specific job. Job information is available for a six month period after creation. Requires that you're the person who ran the job, or have the Is Owner project role.
///
/// A builder for the *get* method supported by a *job* resource.
/// It is not used directly, but through a `JobMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_bigquery2 as bigquery2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use bigquery2::Bigquery;
///
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::client::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Bigquery::new(hyper::client::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().get("projectId", "jobId")
///              .doit();
/// # }
/// ```
pub struct JobGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
    _project_id: String,
    _job_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for JobGetCall<'a, C, A> {}

impl<'a, C, A> JobGetCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, Job)> {
        use std::io::{Read, Seek};
        use hyper::Body;
        use hyper::client::Client;
        use hyper::header::{HeaderValue, CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "bigquery.jobs.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        params.push(("jobId", self._job_id.to_string()));
        for &field in ["alt", "projectId", "jobId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "projects/{projectId}/jobs/{jobId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{jobId}", "jobId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["jobId", "projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



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
            let auth_header = HeaderValue::from_str(&token.access_token).unwrap();
            let mut req_result = {
                let mut hub_mut = self.hub.client.borrow_mut();
                let mut client = hub_mut.deref_mut().borrow_mut();
                let mut req = hyper::Request::get(&url)
                    .header(USER_AGENT, HeaderValue::from_str(&self.hub._user_agent).unwrap())
                    .header(AUTHORIZATION, auth_header.clone())
                    .body(Body::empty())
                    .unwrap();

                dlg.pre_request();
                // Get Future
                let fut = client.request(req);

                // Get response
                match fut.wait() {
                    Ok(i) => Ok(i),
                    Err(e) => Err(e),
                }
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
                    if !res.status().is_success() {
                        let json_err = cmn::read_to_string(&res).unwrap();
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
                        let json_response = cmn::read_to_string(&res).unwrap();
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


    /// [Required] Project ID of the requested job
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> JobGetCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// [Required] Job ID of the requested job
    ///
    /// Sets the *job id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn job_id(mut self, new_value: &str) -> JobGetCall<'a, C, A> {
        self._job_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> JobGetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> JobGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> JobGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Requests that a job be cancelled. This call will return immediately, and the client will need to poll for the job status to see if the cancel completed successfully. Cancelled jobs may still incur costs.
///
/// A builder for the *cancel* method supported by a *job* resource.
/// It is not used directly, but through a `JobMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_bigquery2 as bigquery2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use bigquery2::Bigquery;
///
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::client::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Bigquery::new(hyper::client::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().cancel("projectId", "jobId")
///              .doit();
/// # }
/// ```
pub struct JobCancelCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
    _project_id: String,
    _job_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for JobCancelCall<'a, C, A> {}

impl<'a, C, A> JobCancelCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, JobCancelResponse)> {
        use std::io::{Read, Seek};
        use hyper::Body;
        use hyper::client::Client;
        use hyper::header::{HeaderValue, CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "bigquery.jobs.cancel",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        params.push(("jobId", self._job_id.to_string()));
        for &field in ["alt", "projectId", "jobId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "projects/{projectId}/jobs/{jobId}/cancel";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{jobId}", "jobId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["jobId", "projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



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
            let auth_header = HeaderValue::from_str(&token.access_token).unwrap();
            let mut req_result = {
                let mut hub_mut = self.hub.client.borrow_mut();
                let mut client = hub_mut.deref_mut().borrow_mut();
                let mut req = hyper::Request::post(&url)
                    .header(USER_AGENT, HeaderValue::from_str(&self.hub._user_agent).unwrap())
                    .header(AUTHORIZATION, auth_header.clone())
                    .body(Body::empty())
                    .unwrap();

                dlg.pre_request();
                // Get Future
                let fut = client.request(req);

                // Get response
                match fut.wait() {
                    Ok(i) => Ok(i),
                    Err(e) => Err(e),
                }
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
                    if !res.status().is_success() {
                        let json_err = cmn::read_to_string(&res).unwrap();
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
                        let json_response = cmn::read_to_string(&res).unwrap();
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


    /// [Required] Project ID of the job to cancel
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> JobCancelCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// [Required] Job ID of the job to cancel
    ///
    /// Sets the *job id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn job_id(mut self, new_value: &str) -> JobCancelCall<'a, C, A> {
        self._job_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> JobCancelCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> JobCancelCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> JobCancelCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Runs a BigQuery SQL query synchronously and returns query results if the query completes within a specified timeout.
///
/// A builder for the *query* method supported by a *job* resource.
/// It is not used directly, but through a `JobMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::QueryRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use bigquery2::Bigquery;
///
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::client::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Bigquery::new(hyper::client::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = QueryRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().query(req, "projectId")
///              .doit();
/// # }
/// ```
pub struct JobQueryCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
    _request: QueryRequest,
    _project_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for JobQueryCall<'a, C, A> {}

impl<'a, C, A> JobQueryCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, QueryResponse)> {
        use std::io::{Read, Seek};
        use hyper::Body;
        use hyper::client::Client;
        use hyper::header::{HeaderValue, CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "bigquery.jobs.query",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        for &field in ["alt", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "projects/{projectId}/queries";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId")].iter() {
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
            for param_name in ["projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::APPLICATION_JSON;
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
            let auth_header = HeaderValue::from_str(&token.access_token).unwrap();
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut hub_mut = self.hub.client.borrow_mut();
                let mut client = hub_mut.deref_mut().borrow_mut();

                // Get contents of Cursor
                let inside_cursor = request_value_reader.clone().into_inner();
                //Build a stream
                let stream = futures::stream::iter_ok::<_, ::std::io::Error>(vec![inside_cursor]);
                // Wrap stream in a box inside a body
                let body = Body::wrap_stream(stream);

                // Build a request
                let mut req = hyper::Request::post(&url)
                    .header(USER_AGENT, HeaderValue::from_str(&self.hub._user_agent).unwrap())
                    .header(AUTHORIZATION, auth_header.clone())
                    .header(CONTENT_TYPE, HeaderValue::from_str(&json_mime_type.to_string()).unwrap())
                    .header(CONTENT_LENGTH, HeaderValue::from_str(&request_size.to_string()).unwrap())
                    .body(body)
                    .unwrap();

                dlg.pre_request();
                // Get Future
                let fut = client.request(req);

                // Get response
                match fut.wait() {
                    Ok(i) => Ok(i),
                    Err(e) => Err(e),
                }
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
                    if !res.status().is_success() {
                        let json_err = cmn::read_to_string(&res).unwrap();
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
                        let json_response = cmn::read_to_string(&res).unwrap();
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
    pub fn request(mut self, new_value: QueryRequest) -> JobQueryCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Project ID of the project billed for the query
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> JobQueryCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> JobQueryCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> JobQueryCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> JobQueryCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Starts a new asynchronous job. Requires the Can View project role.
///
/// A builder for the *insert* method supported by a *job* resource.
/// It is not used directly, but through a `JobMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::Job;
/// use std::fs;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use bigquery2::Bigquery;
///
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::client::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Bigquery::new(hyper::client::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Job::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `upload(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().insert(req, "projectId")
///              .upload(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap());
/// # }
/// ```
pub struct JobInsertCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
    _request: Job,
    _project_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for JobInsertCall<'a, C, A> {}

impl<'a, C, A> JobInsertCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    fn doit<RS>(mut self, mut reader: RS, reader_mime_type: mime::Mime, protocol: &'static str) -> Result<(hyper::Response<hyper::Body>, Job)>
		where RS: ReadSeek {
        use std::io::{Read, Seek};
        use hyper::Body;
        use hyper::client::Client;
        use hyper::header::{HeaderValue, X_CONTENT_TYPE_OPTIONS, CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "bigquery.jobs.insert",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        for &field in ["alt", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let (mut url, upload_type) =
            if protocol == "simple" {
                (self.hub._root_url.clone() + "upload/bigquery/v2/projects/{projectId}/jobs", "multipart")
            } else if protocol == "resumable" {
                (self.hub._root_url.clone() + "resumable/upload/bigquery/v2/projects/{projectId}/jobs", "resumable")
            } else {
                unreachable!()
            };
        params.push(("uploadType", upload_type.to_string()));
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId")].iter() {
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
            for param_name in ["projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::APPLICATION_JSON;
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

        let mut should_ask_dlg_for_url = false;
        let mut upload_url_from_server;
        let mut upload_url: Option<String> = None;

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
            let auth_header = HeaderValue::from_str(&token.access_token).unwrap();
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                if should_ask_dlg_for_url && (upload_url = dlg.upload_url()) == () && upload_url.is_some() {
                    should_ask_dlg_for_url = false;
                    upload_url_from_server = false;
                    let url = upload_url.as_ref().unwrap();
                    let mut res = hyper::Response::new(Body::empty());
                    *res.status_mut() = hyper::StatusCode::OK;
                    res.headers_mut().insert(LOCATION, HeaderValue::from_str(&url).unwrap());
                    Ok(res)
                } else {
                    let mut mp_reader: MultiPartReader = Default::default();
                    let (mut body_reader, content_type) = match protocol {
                        "simple" => {
                            mp_reader.reserve_exact(2);
                            let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                            reader.seek(io::SeekFrom::Start(0)).unwrap();

                            mp_reader.add_part(&mut request_value_reader, request_size, json_mime_type.clone())
                                     .add_part(&mut reader, size, reader_mime_type.clone());
                            let mime_type = mp_reader.mime_type();
                            (&mut mp_reader as &mut io::Read, HeaderValue::from_str(&mime_type.to_string()).unwrap())
                        },
                        _ => (&mut request_value_reader as &mut io::Read, HeaderValue::from_str(&json_mime_type.to_string()).unwrap()),
                    };
                    let mut hub_mut = self.hub.client.borrow_mut();
                    let mut client = hub_mut.deref_mut().borrow_mut();

                    // Pull some bytes into buffer, return number of read bytes
                    // TODO: Change so the range is not set randomly
                    let body = format!("Total bytes read: {:?}", body_reader.read(&mut [0; 1000]));
                    //Build a stream
                    let stream = futures::stream::iter_ok::<_, ::std::io::Error>(vec![body]);
                    // Wrap stream in a box inside a body
                    let body = Body::wrap_stream(stream);

                    // Build a request
                    let mut req = hyper::Request::post(&url)
                        .header(USER_AGENT, HeaderValue::from_str(&self.hub._user_agent).unwrap())
                        .header(AUTHORIZATION, auth_header.clone())
                        .header(CONTENT_TYPE, content_type)
                        .body(body)
                        .unwrap();
                    upload_url_from_server = true;
                    if protocol == "resumable" {
                        req.headers_mut().insert(X_CONTENT_TYPE_OPTIONS, HeaderValue::from_str(&reader_mime_type.to_string()).unwrap());
                    }

                    dlg.pre_request();
                    // Get Future
                    let fut = client.request(req);

                    // Get response
                    match fut.wait() {
                        Ok(i) => Ok(i),
                        Err(e) => Err(e),
                    }
                }
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
                    if !res.status().is_success() {
                        let json_err = cmn::read_to_string(&res).unwrap();
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
                    if protocol == "resumable" {
                        let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();

                        let mut hub_mut = self.hub.client.borrow_mut();
                        let mut client = hub_mut.deref_mut().borrow_mut();
                        
                        let upload_result = {
                            let url_str = res.headers().get(LOCATION).expect("Location header is part of protocol")
                                .to_str().expect("Could not convert to &str");
                            if upload_url_from_server {
                                dlg.store_upload_url(Some(url_str));
                            }

                            cmn::ResumableUploadHelper {
                                client: &mut client.borrow_mut(),
                                delegate: dlg,
                                start_at: if upload_url_from_server { Some(0) } else { None },
                                auth: &mut *self.hub.auth.borrow_mut(),
                                user_agent: &self.hub._user_agent,
                                auth_header: auth_header.clone(),
                                url: url_str,
                                reader: &mut reader,
                                media_type: reader_mime_type.clone(),
                                content_length: size
                            }.upload()
                        };
                        match upload_result {
                            None => {
                                dlg.finished(false);
                                return Err(Error::Cancelled)
                            }
                            Some(Err(err)) => {
                                dlg.finished(false);
                                return Err(Error::HttpError(err))
                            }
                            Some(Ok(upload_result)) => {
                                res = upload_result;
                                if !res.status().is_success() {
                                    dlg.store_upload_url(None);
                                    dlg.finished(false);
                                    return Err(Error::Failure(res))
                                }
                            }
                        }
                    }
                    let result_value = {
                        let json_response = cmn::read_to_string(&res).unwrap();
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

    /// Upload media all at once.
    /// If the upload fails for whichever reason, all progress is lost.
    ///
    /// * *max size*: 0kb
    /// * *multipart*: yes
    /// * *valid mime types*: '*/*'
    pub fn upload<RS>(self, stream: RS, mime_type: mime::Mime) -> Result<(hyper::Response<hyper::Body>, Job)>
                where RS: ReadSeek {
        self.doit(stream, mime_type, "simple")
    }
    /// Upload media in a resumable fashion.
    /// Even if the upload fails or is interrupted, it can be resumed for a
    /// certain amount of time as the server maintains state temporarily.
    ///
    /// The delegate will be asked for an `upload_url()`, and if not provided, will be asked to store an upload URL
    /// that was provided by the server, using `store_upload_url(...)`. The upload will be done in chunks, the delegate
    /// may specify the `chunk_size()` and may cancel the operation before each chunk is uploaded, using
    /// `cancel_chunk_upload(...)`.
    ///
    /// * *max size*: 0kb
    /// * *multipart*: yes
    /// * *valid mime types*: '*/*'
    pub fn upload_resumable<RS>(self, resumeable_stream: RS, mime_type: mime::Mime) -> Result<(hyper::Response<hyper::Body>, Job)>
                where RS: ReadSeek {
        self.doit(resumeable_stream, mime_type, "resumable")
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Job) -> JobInsertCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Project ID of the project that will be billed for the job
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> JobInsertCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> JobInsertCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> JobInsertCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> JobInsertCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists all jobs that you started in the specified project. Job information is available for a six month period after creation. The job list is sorted in reverse chronological order, by job creation time. Requires the Can View project role, or the Is Owner project role if you set the allUsers property.
///
/// A builder for the *list* method supported by a *job* resource.
/// It is not used directly, but through a `JobMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_bigquery2 as bigquery2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use bigquery2::Bigquery;
///
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::client::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Bigquery::new(hyper::client::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().list("projectId")
///              .add_state_filter("duo")
///              .projection("et")
///              .page_token("eirmod")
///              .max_results(43)
///              .all_users(true)
///              .doit();
/// # }
/// ```
pub struct JobListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
    _project_id: String,
    _state_filter: Vec<String>,
    _projection: Option<String>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _all_users: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for JobListCall<'a, C, A> {}

impl<'a, C, A> JobListCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, JobList)> {
        use std::io::{Read, Seek};
        use hyper::Body;
        use hyper::client::Client;
        use hyper::header::{HeaderValue, CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "bigquery.jobs.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(8 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        if self._state_filter.len() > 0 {
            for f in self._state_filter.iter() {
                params.push(("stateFilter", f.to_string()));
            }
        }
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._all_users {
            params.push(("allUsers", value.to_string()));
        }
        for &field in ["alt", "projectId", "stateFilter", "projection", "pageToken", "maxResults", "allUsers"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "projects/{projectId}/jobs";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId")].iter() {
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
            for param_name in ["projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



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
            let auth_header = HeaderValue::from_str(&token.access_token).unwrap();
            let mut req_result = {
                let mut hub_mut = self.hub.client.borrow_mut();
                let mut client = hub_mut.deref_mut().borrow_mut();
                let mut req = hyper::Request::get(&url)
                    .header(USER_AGENT, HeaderValue::from_str(&self.hub._user_agent).unwrap())
                    .header(AUTHORIZATION, auth_header.clone())
                    .body(Body::empty())
                    .unwrap();

                dlg.pre_request();
                // Get Future
                let fut = client.request(req);

                // Get response
                match fut.wait() {
                    Ok(i) => Ok(i),
                    Err(e) => Err(e),
                }
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
                    if !res.status().is_success() {
                        let json_err = cmn::read_to_string(&res).unwrap();
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
                        let json_response = cmn::read_to_string(&res).unwrap();
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


    /// Project ID of the jobs to list
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> JobListCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Filter for job state
    ///
    /// Append the given value to the *state filter* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_state_filter(mut self, new_value: &str) -> JobListCall<'a, C, A> {
        self._state_filter.push(new_value.to_string());
        self
    }
    /// Restrict information returned to a set of selected fields
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> JobListCall<'a, C, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Page token, returned by a previous call, to request the next page of results
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> JobListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> JobListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Whether to display jobs owned by all users in the project. Default false
    ///
    /// Sets the *all users* query property to the given value.
    pub fn all_users(mut self, new_value: bool) -> JobListCall<'a, C, A> {
        self._all_users = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> JobListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> JobListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> JobListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves the results of a query job.
///
/// A builder for the *getQueryResults* method supported by a *job* resource.
/// It is not used directly, but through a `JobMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_bigquery2 as bigquery2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use bigquery2::Bigquery;
///
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::client::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Bigquery::new(hyper::client::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().get_query_results("projectId", "jobId")
///              .timeout_ms(56)
///              .start_index("ut")
///              .page_token("ea")
///              .max_results(21)
///              .doit();
/// # }
/// ```
pub struct JobGetQueryResultCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
    _project_id: String,
    _job_id: String,
    _timeout_ms: Option<u32>,
    _start_index: Option<String>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for JobGetQueryResultCall<'a, C, A> {}

impl<'a, C, A> JobGetQueryResultCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, GetQueryResultsResponse)> {
        use std::io::{Read, Seek};
        use hyper::Body;
        use hyper::rt::Future;
        use hyper::client::Client;
        use hyper::header::{HeaderValue, CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "bigquery.jobs.getQueryResults",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(8 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        params.push(("jobId", self._job_id.to_string()));
        if let Some(value) = self._timeout_ms {
            params.push(("timeoutMs", value.to_string()));
        }
        if let Some(value) = self._start_index {
            params.push(("startIndex", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        for &field in ["alt", "projectId", "jobId", "timeoutMs", "startIndex", "pageToken", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "projects/{projectId}/queries/{jobId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{jobId}", "jobId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["jobId", "projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



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
            let auth_header = HeaderValue::from_str(&token.access_token).unwrap();
            let mut req_result = {
                let mut hub_mut = self.hub.client.borrow_mut();
                let mut client = hub_mut.deref_mut().borrow_mut();
                let mut req = hyper::Request::get(&url)
                    .header(USER_AGENT, HeaderValue::from_str(&self.hub._user_agent).unwrap())
                    .header(AUTHORIZATION, auth_header.clone())
                    .body(Body::empty())
                    .unwrap();

                dlg.pre_request();
                // Get Future
                let fut = client.request(req);

                // Get response
                match fut.wait() {
                    Ok(i) => Ok(i),
                    Err(e) => Err(e),
                }

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
                    if !res.status().is_success() {
                        let json_err = cmn::read_to_string(&res).unwrap();
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
                        let json_response = cmn::read_to_string(&res).unwrap();
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


    /// [Required] Project ID of the query job
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> JobGetQueryResultCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// [Required] Job ID of the query job
    ///
    /// Sets the *job id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn job_id(mut self, new_value: &str) -> JobGetQueryResultCall<'a, C, A> {
        self._job_id = new_value.to_string();
        self
    }
    /// How long to wait for the query to complete, in milliseconds, before returning. Default is 10 seconds. If the timeout passes before the job completes, the 'jobComplete' field in the response will be false
    ///
    /// Sets the *timeout ms* query property to the given value.
    pub fn timeout_ms(mut self, new_value: u32) -> JobGetQueryResultCall<'a, C, A> {
        self._timeout_ms = Some(new_value);
        self
    }
    /// Zero-based index of the starting row
    ///
    /// Sets the *start index* query property to the given value.
    pub fn start_index(mut self, new_value: &str) -> JobGetQueryResultCall<'a, C, A> {
        self._start_index = Some(new_value.to_string());
        self
    }
    /// Page token, returned by a previous call, to request the next page of results
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> JobGetQueryResultCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to read
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> JobGetQueryResultCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> JobGetQueryResultCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> JobGetQueryResultCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> JobGetQueryResultCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves table data from a specified set of rows. Requires the READER dataset role.
///
/// A builder for the *list* method supported by a *tabledata* resource.
/// It is not used directly, but through a `TabledataMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_bigquery2 as bigquery2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use bigquery2::Bigquery;
///
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::client::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Bigquery::new(hyper::client::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.tabledata().list("projectId", "datasetId", "tableId")
///              .start_index("et")
///              .selected_fields("consetetur")
///              .page_token("amet.")
///              .max_results(74)
///              .doit();
/// # }
/// ```
pub struct TabledataListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
    _project_id: String,
    _dataset_id: String,
    _table_id: String,
    _start_index: Option<String>,
    _selected_fields: Option<String>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for TabledataListCall<'a, C, A> {}

impl<'a, C, A> TabledataListCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, TableDataList)> {
        use std::io::{Read, Seek};
        use hyper::Body;
        use hyper::client::Client;
        use hyper::header::{HeaderValue, CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "bigquery.tabledata.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        params.push(("datasetId", self._dataset_id.to_string()));
        params.push(("tableId", self._table_id.to_string()));
        if let Some(value) = self._start_index {
            params.push(("startIndex", value.to_string()));
        }
        if let Some(value) = self._selected_fields {
            params.push(("selectedFields", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        for &field in ["alt", "projectId", "datasetId", "tableId", "startIndex", "selectedFields", "pageToken", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "projects/{projectId}/datasets/{datasetId}/tables/{tableId}/data";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{datasetId}", "datasetId"), ("{tableId}", "tableId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["tableId", "datasetId", "projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



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
            let auth_header = HeaderValue::from_str(&token.access_token).unwrap();
            let mut req_result = {
                let mut hub_mut = self.hub.client.borrow_mut();
                let mut client = hub_mut.deref_mut().borrow_mut();
                let mut req = hyper::Request::get(&url)
                    .header(USER_AGENT, HeaderValue::from_str(&self.hub._user_agent).unwrap())
                    .header(AUTHORIZATION, auth_header.clone())
                    .body(Body::empty())
                    .unwrap();

                dlg.pre_request();
                // Get Future
                let fut = client.request(req);

                // Get response
                match fut.wait() {
                    Ok(i) => Ok(i),
                    Err(e) => Err(e),
                }
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
                    if !res.status().is_success() {
                        let json_err = cmn::read_to_string(&res).unwrap();
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
                        let json_response = cmn::read_to_string(&res).unwrap();
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


    /// Project ID of the table to read
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> TabledataListCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Dataset ID of the table to read
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> TabledataListCall<'a, C, A> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Table ID of the table to read
    ///
    /// Sets the *table id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn table_id(mut self, new_value: &str) -> TabledataListCall<'a, C, A> {
        self._table_id = new_value.to_string();
        self
    }
    /// Zero-based index of the starting row to read
    ///
    /// Sets the *start index* query property to the given value.
    pub fn start_index(mut self, new_value: &str) -> TabledataListCall<'a, C, A> {
        self._start_index = Some(new_value.to_string());
        self
    }
    /// List of fields to return (comma-separated). If unspecified, all fields are returned
    ///
    /// Sets the *selected fields* query property to the given value.
    pub fn selected_fields(mut self, new_value: &str) -> TabledataListCall<'a, C, A> {
        self._selected_fields = Some(new_value.to_string());
        self
    }
    /// Page token, returned by a previous call, identifying the result set
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> TabledataListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> TabledataListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TabledataListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> TabledataListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> TabledataListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Streams data into BigQuery one record at a time without needing to run a load job. Requires the WRITER dataset role.
///
/// A builder for the *insertAll* method supported by a *tabledata* resource.
/// It is not used directly, but through a `TabledataMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::TableDataInsertAllRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use bigquery2::Bigquery;
///
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::client::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Bigquery::new(hyper::client::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = TableDataInsertAllRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.tabledata().insert_all(req, "projectId", "datasetId", "tableId")
///              .doit();
/// # }
/// ```
pub struct TabledataInsertAllCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
    _request: TableDataInsertAllRequest,
    _project_id: String,
    _dataset_id: String,
    _table_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for TabledataInsertAllCall<'a, C, A> {}

impl<'a, C, A> TabledataInsertAllCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, TableDataInsertAllResponse)> {
        use std::io::{Read, Seek};
        use hyper::Body;
        use hyper::client::Client;
        use hyper::header::{HeaderValue, CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "bigquery.tabledata.insertAll",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        params.push(("datasetId", self._dataset_id.to_string()));
        params.push(("tableId", self._table_id.to_string()));
        for &field in ["alt", "projectId", "datasetId", "tableId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "projects/{projectId}/datasets/{datasetId}/tables/{tableId}/insertAll";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{datasetId}", "datasetId"), ("{tableId}", "tableId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["tableId", "datasetId", "projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::APPLICATION_JSON;
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
            let auth_header = HeaderValue::from_str(&token.access_token).unwrap();
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut hub_mut = self.hub.client.borrow_mut();
                let mut client = hub_mut.deref_mut().borrow_mut();

                // Get contents of Cursor
                let inside_cursor = request_value_reader.clone().into_inner();
                //Build a stream
                let stream = futures::stream::iter_ok::<_, ::std::io::Error>(vec![inside_cursor]);
                // Wrap stream in a box inside a body
                let body = Body::wrap_stream(stream);

                // Build a request
                let mut req = hyper::Request::post(&url)
                    .header(USER_AGENT, HeaderValue::from_str(&self.hub._user_agent).unwrap())
                    .header(AUTHORIZATION, auth_header.clone())
                    .header(CONTENT_TYPE, HeaderValue::from_str(&json_mime_type.to_string()).unwrap())
                    .header(CONTENT_LENGTH, HeaderValue::from_str(&request_size.to_string()).unwrap())
                    .body(body)
                    .unwrap();

                dlg.pre_request();
                // Get Future
                let fut = client.request(req);

                // Get response
                match fut.wait() {
                    Ok(i) => Ok(i),
                    Err(e) => Err(e),
                }
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
                    if !res.status().is_success() {
                        let json_err = cmn::read_to_string(&res).unwrap();
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
                        let json_response = cmn::read_to_string(&res).unwrap();
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
    pub fn request(mut self, new_value: TableDataInsertAllRequest) -> TabledataInsertAllCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Project ID of the destination table.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> TabledataInsertAllCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Dataset ID of the destination table.
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> TabledataInsertAllCall<'a, C, A> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Table ID of the destination table.
    ///
    /// Sets the *table id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn table_id(mut self, new_value: &str) -> TabledataInsertAllCall<'a, C, A> {
        self._table_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TabledataInsertAllCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> TabledataInsertAllCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> TabledataInsertAllCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns the email address of the service account for your project used for interactions with Google Cloud KMS.
///
/// A builder for the *getServiceAccount* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_bigquery2 as bigquery2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use bigquery2::Bigquery;
///
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::client::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Bigquery::new(hyper::client::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().get_service_account("projectId")
///              .doit();
/// # }
/// ```
pub struct ProjectGetServiceAccountCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
    _project_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectGetServiceAccountCall<'a, C, A> {}

impl<'a, C, A> ProjectGetServiceAccountCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, GetServiceAccountResponse)> {
        use std::io::{Read, Seek};
        use hyper::Body;
        use hyper::client::Client;
        use hyper::header::{HeaderValue, CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "bigquery.projects.getServiceAccount",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        for &field in ["alt", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "projects/{projectId}/serviceAccount";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId")].iter() {
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
            for param_name in ["projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



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
            let auth_header = HeaderValue::from_str(&token.access_token).unwrap();
            let mut req_result = {
                let mut hub_mut = self.hub.client.borrow_mut();
                let mut client = hub_mut.deref_mut().borrow_mut();
                let mut req = hyper::Request::get(&url)
                    .header(USER_AGENT, HeaderValue::from_str(&self.hub._user_agent).unwrap())
                    .header(AUTHORIZATION, auth_header.clone())
                    .body(Body::empty())
                    .unwrap();

                dlg.pre_request();
                // Get Future
                let fut = client.request(req);

                // Get response
                match fut.wait() {
                    Ok(i) => Ok(i),
                    Err(e) => Err(e),
                }
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
                    if !res.status().is_success() {
                        let json_err = cmn::read_to_string(&res).unwrap();
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
                        let json_response = cmn::read_to_string(&res).unwrap();
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


    /// Project ID for which the service account is requested.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ProjectGetServiceAccountCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectGetServiceAccountCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectGetServiceAccountCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectGetServiceAccountCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists all projects to which you have been granted any project role.
///
/// A builder for the *list* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_bigquery2 as bigquery2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use bigquery2::Bigquery;
///
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::client::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Bigquery::new(hyper::client::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().list()
///              .page_token("vero")
///              .max_results(73)
///              .doit();
/// # }
/// ```
pub struct ProjectListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Bigquery<C, A>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectListCall<'a, C, A> {}

impl<'a, C, A> ProjectListCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, ProjectList)> {
        use std::io::{Read, Seek};
        use hyper::Body;
        use hyper::header::{HeaderValue, CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "bigquery.projects.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        for &field in ["alt", "pageToken", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "projects";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }


        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



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
            let auth_header = HeaderValue::from_str(&token.access_token).unwrap();
            let mut req_result = {
                let mut hub_mut = self.hub.client.borrow_mut();
                let mut client = hub_mut.deref_mut().borrow_mut();
                let mut req = hyper::Request::get(&url)
                    .header(USER_AGENT, HeaderValue::from_str(&self.hub._user_agent).unwrap())
                    .header(AUTHORIZATION, auth_header.clone())
                    .body(Body::empty())
                    .unwrap();

                dlg.pre_request();
                // Get Future
                let fut = client.request(req);

                // Get response
                match fut.wait() {
                    Ok(i) => Ok(i),
                    Err(e) => Err(e),
                }
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
                    if !res.status().is_success() {
                        let json_err = cmn::read_to_string(&res).unwrap();
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
                        let json_response = cmn::read_to_string(&res).unwrap();
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


    /// Page token, returned by a previous call, to request the next page of results
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ProjectListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> ProjectListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


