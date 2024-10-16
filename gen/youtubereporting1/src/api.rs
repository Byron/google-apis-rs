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
    /// View monetary and non-monetary YouTube Analytics reports for your YouTube content
    YtAnalyticMonetaryReadonly,

    /// View YouTube Analytics reports for your YouTube content
    YtAnalyticReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::YtAnalyticMonetaryReadonly => {
                "https://www.googleapis.com/auth/yt-analytics-monetary.readonly"
            }
            Scope::YtAnalyticReadonly => "https://www.googleapis.com/auth/yt-analytics.readonly",
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for Scope {
    fn default() -> Scope {
        Scope::YtAnalyticMonetaryReadonly
    }
}

// ########
// HUB ###
// ######

/// Central instance to access all YouTubeReporting related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtubereporting1 as youtubereporting1;
/// use youtubereporting1::{Result, Error};
/// # async fn dox() {
/// use youtubereporting1::{YouTubeReporting, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// let mut hub = YouTubeReporting::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().reports_list("jobId")
///              .start_time_before(chrono::Utc::now())
///              .start_time_at_or_after(chrono::Utc::now())
///              .page_token("takimata")
///              .page_size(-52)
///              .on_behalf_of_content_owner("duo")
///              .created_after(chrono::Utc::now())
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
pub struct YouTubeReporting<C> {
    pub client: common::Client<C>,
    pub auth: Box<dyn common::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<C> common::Hub for YouTubeReporting<C> {}

impl<'a, C> YouTubeReporting<C> {
    pub fn new<A: 'static + common::GetToken>(
        client: common::Client<C>,
        auth: A,
    ) -> YouTubeReporting<C> {
        YouTubeReporting {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/6.0.0".to_string(),
            _base_url: "https://youtubereporting.googleapis.com/".to_string(),
            _root_url: "https://youtubereporting.googleapis.com/".to_string(),
        }
    }

    pub fn jobs(&'a self) -> JobMethods<'a, C> {
        JobMethods { hub: self }
    }
    pub fn media(&'a self) -> MediaMethods<'a, C> {
        MediaMethods { hub: self }
    }
    pub fn report_types(&'a self) -> ReportTypeMethods<'a, C> {
        ReportTypeMethods { hub: self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/6.0.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        std::mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://youtubereporting.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        std::mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://youtubereporting.googleapis.com/`.
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
/// * [delete jobs](JobDeleteCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Empty {
    _never_set: Option<bool>,
}

impl common::ResponseResult for Empty {}

/// gdata
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GdataBlobstore2Info {
    /// gdata
    #[serde(rename = "blobGeneration")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub blob_generation: Option<i64>,
    /// gdata
    #[serde(rename = "blobId")]
    pub blob_id: Option<String>,
    /// gdata
    #[serde(rename = "downloadReadHandle")]
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub download_read_handle: Option<Vec<u8>>,
    /// gdata
    #[serde(rename = "readToken")]
    pub read_token: Option<String>,
    /// gdata
    #[serde(rename = "uploadMetadataContainer")]
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub upload_metadata_container: Option<Vec<u8>>,
}

impl common::Part for GdataBlobstore2Info {}

/// gdata
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GdataCompositeMedia {
    /// gdata
    #[serde(rename = "blobRef")]
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub blob_ref: Option<Vec<u8>>,
    /// gdata
    #[serde(rename = "blobstore2Info")]
    pub blobstore2_info: Option<GdataBlobstore2Info>,
    /// gdata
    #[serde(rename = "cosmoBinaryReference")]
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub cosmo_binary_reference: Option<Vec<u8>>,
    /// gdata
    #[serde(rename = "crc32cHash")]
    pub crc32c_hash: Option<u32>,
    /// gdata
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub inline: Option<Vec<u8>>,
    /// gdata
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub length: Option<i64>,
    /// gdata
    #[serde(rename = "md5Hash")]
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub md5_hash: Option<Vec<u8>>,
    /// gdata
    #[serde(rename = "objectId")]
    pub object_id: Option<GdataObjectId>,
    /// gdata
    pub path: Option<String>,
    /// gdata
    #[serde(rename = "referenceType")]
    pub reference_type: Option<String>,
    /// gdata
    #[serde(rename = "sha1Hash")]
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub sha1_hash: Option<Vec<u8>>,
}

impl common::Part for GdataCompositeMedia {}

/// gdata
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GdataContentTypeInfo {
    /// gdata
    #[serde(rename = "bestGuess")]
    pub best_guess: Option<String>,
    /// gdata
    #[serde(rename = "fromBytes")]
    pub from_bytes: Option<String>,
    /// gdata
    #[serde(rename = "fromFileName")]
    pub from_file_name: Option<String>,
    /// gdata
    #[serde(rename = "fromHeader")]
    pub from_header: Option<String>,
    /// gdata
    #[serde(rename = "fromUrlPath")]
    pub from_url_path: Option<String>,
}

impl common::Part for GdataContentTypeInfo {}

/// gdata
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GdataDiffChecksumsResponse {
    /// gdata
    #[serde(rename = "checksumsLocation")]
    pub checksums_location: Option<GdataCompositeMedia>,
    /// gdata
    #[serde(rename = "chunkSizeBytes")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub chunk_size_bytes: Option<i64>,
    /// gdata
    #[serde(rename = "objectLocation")]
    pub object_location: Option<GdataCompositeMedia>,
    /// gdata
    #[serde(rename = "objectSizeBytes")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub object_size_bytes: Option<i64>,
    /// gdata
    #[serde(rename = "objectVersion")]
    pub object_version: Option<String>,
}

impl common::Part for GdataDiffChecksumsResponse {}

/// gdata
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GdataDiffDownloadResponse {
    /// gdata
    #[serde(rename = "objectLocation")]
    pub object_location: Option<GdataCompositeMedia>,
}

impl common::Part for GdataDiffDownloadResponse {}

/// gdata
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GdataDiffUploadRequest {
    /// gdata
    #[serde(rename = "checksumsInfo")]
    pub checksums_info: Option<GdataCompositeMedia>,
    /// gdata
    #[serde(rename = "objectInfo")]
    pub object_info: Option<GdataCompositeMedia>,
    /// gdata
    #[serde(rename = "objectVersion")]
    pub object_version: Option<String>,
}

impl common::Part for GdataDiffUploadRequest {}

/// gdata
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GdataDiffUploadResponse {
    /// gdata
    #[serde(rename = "objectVersion")]
    pub object_version: Option<String>,
    /// gdata
    #[serde(rename = "originalObject")]
    pub original_object: Option<GdataCompositeMedia>,
}

impl common::Part for GdataDiffUploadResponse {}

/// gdata
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GdataDiffVersionResponse {
    /// gdata
    #[serde(rename = "objectSizeBytes")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub object_size_bytes: Option<i64>,
    /// gdata
    #[serde(rename = "objectVersion")]
    pub object_version: Option<String>,
}

impl common::Part for GdataDiffVersionResponse {}

/// gdata
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GdataDownloadParameters {
    /// gdata
    #[serde(rename = "allowGzipCompression")]
    pub allow_gzip_compression: Option<bool>,
    /// gdata
    #[serde(rename = "ignoreRange")]
    pub ignore_range: Option<bool>,
}

impl common::Part for GdataDownloadParameters {}

/// gdata
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [download media](MediaDownloadCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GdataMedia {
    /// gdata
    pub algorithm: Option<String>,
    /// gdata
    #[serde(rename = "bigstoreObjectRef")]
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub bigstore_object_ref: Option<Vec<u8>>,
    /// gdata
    #[serde(rename = "blobRef")]
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub blob_ref: Option<Vec<u8>>,
    /// gdata
    #[serde(rename = "blobstore2Info")]
    pub blobstore2_info: Option<GdataBlobstore2Info>,
    /// gdata
    #[serde(rename = "compositeMedia")]
    pub composite_media: Option<Vec<GdataCompositeMedia>>,
    /// gdata
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    /// gdata
    #[serde(rename = "contentTypeInfo")]
    pub content_type_info: Option<GdataContentTypeInfo>,
    /// gdata
    #[serde(rename = "cosmoBinaryReference")]
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub cosmo_binary_reference: Option<Vec<u8>>,
    /// gdata
    #[serde(rename = "crc32cHash")]
    pub crc32c_hash: Option<u32>,
    /// gdata
    #[serde(rename = "diffChecksumsResponse")]
    pub diff_checksums_response: Option<GdataDiffChecksumsResponse>,
    /// gdata
    #[serde(rename = "diffDownloadResponse")]
    pub diff_download_response: Option<GdataDiffDownloadResponse>,
    /// gdata
    #[serde(rename = "diffUploadRequest")]
    pub diff_upload_request: Option<GdataDiffUploadRequest>,
    /// gdata
    #[serde(rename = "diffUploadResponse")]
    pub diff_upload_response: Option<GdataDiffUploadResponse>,
    /// gdata
    #[serde(rename = "diffVersionResponse")]
    pub diff_version_response: Option<GdataDiffVersionResponse>,
    /// gdata
    #[serde(rename = "downloadParameters")]
    pub download_parameters: Option<GdataDownloadParameters>,
    /// gdata
    pub filename: Option<String>,
    /// gdata
    pub hash: Option<String>,
    /// gdata
    #[serde(rename = "hashVerified")]
    pub hash_verified: Option<bool>,
    /// gdata
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub inline: Option<Vec<u8>>,
    /// gdata
    #[serde(rename = "isPotentialRetry")]
    pub is_potential_retry: Option<bool>,
    /// gdata
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub length: Option<i64>,
    /// gdata
    #[serde(rename = "md5Hash")]
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub md5_hash: Option<Vec<u8>>,
    /// gdata
    #[serde(rename = "mediaId")]
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub media_id: Option<Vec<u8>>,
    /// gdata
    #[serde(rename = "objectId")]
    pub object_id: Option<GdataObjectId>,
    /// gdata
    pub path: Option<String>,
    /// gdata
    #[serde(rename = "referenceType")]
    pub reference_type: Option<String>,
    /// gdata
    #[serde(rename = "sha1Hash")]
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub sha1_hash: Option<Vec<u8>>,
    /// gdata
    #[serde(rename = "sha256Hash")]
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub sha256_hash: Option<Vec<u8>>,
    /// gdata
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub timestamp: Option<u64>,
    /// gdata
    pub token: Option<String>,
}

impl common::ResponseResult for GdataMedia {}

/// gdata
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GdataObjectId {
    /// gdata
    #[serde(rename = "bucketName")]
    pub bucket_name: Option<String>,
    /// gdata
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub generation: Option<i64>,
    /// gdata
    #[serde(rename = "objectName")]
    pub object_name: Option<String>,
}

impl common::Part for GdataObjectId {}

/// A job creating reports of a specific type.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [reports get jobs](JobReportGetCall) (none)
/// * [reports list jobs](JobReportListCall) (none)
/// * [create jobs](JobCreateCall) (request|response)
/// * [delete jobs](JobDeleteCall) (none)
/// * [get jobs](JobGetCall) (response)
/// * [list jobs](JobListCall) (none)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Job {
    /// The creation date/time of the job.
    #[serde(rename = "createTime")]
    pub create_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// The date/time when this job will expire/expired. After a job expired, no new reports are generated.
    #[serde(rename = "expireTime")]
    pub expire_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// The server-generated ID of the job (max. 40 characters).
    pub id: Option<String>,
    /// The name of the job (max. 100 characters).
    pub name: Option<String>,
    /// The type of reports this job creates. Corresponds to the ID of a ReportType.
    #[serde(rename = "reportTypeId")]
    pub report_type_id: Option<String>,
    /// True if this a system-managed job that cannot be modified by the user; otherwise false.
    #[serde(rename = "systemManaged")]
    pub system_managed: Option<bool>,
}

impl common::RequestValue for Job {}
impl common::Resource for Job {}
impl common::ResponseResult for Job {}

/// Response message for ReportingService.ListJobs.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list jobs](JobListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListJobsResponse {
    /// The list of jobs.
    pub jobs: Option<Vec<Job>>,
    /// A token to retrieve next page of results. Pass this value in the ListJobsRequest.page_token field in the subsequent call to `ListJobs` method to retrieve the next page of results.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}

impl common::ResponseResult for ListJobsResponse {}

/// Response message for ReportingService.ListReportTypes.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list report types](ReportTypeListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListReportTypesResponse {
    /// A token to retrieve next page of results. Pass this value in the ListReportTypesRequest.page_token field in the subsequent call to `ListReportTypes` method to retrieve the next page of results.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    /// The list of report types.
    #[serde(rename = "reportTypes")]
    pub report_types: Option<Vec<ReportType>>,
}

impl common::ResponseResult for ListReportTypesResponse {}

/// Response message for ReportingService.ListReports.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [reports list jobs](JobReportListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListReportsResponse {
    /// A token to retrieve next page of results. Pass this value in the ListReportsRequest.page_token field in the subsequent call to `ListReports` method to retrieve the next page of results.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    /// The list of report types.
    pub reports: Option<Vec<Report>>,
}

impl common::ResponseResult for ListReportsResponse {}

/// A report’s metadata including the URL from which the report itself can be downloaded.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [reports get jobs](JobReportGetCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Report {
    /// The date/time when this report was created.
    #[serde(rename = "createTime")]
    pub create_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// The URL from which the report can be downloaded (max. 1000 characters).
    #[serde(rename = "downloadUrl")]
    pub download_url: Option<String>,
    /// The end of the time period that the report instance covers. The value is exclusive.
    #[serde(rename = "endTime")]
    pub end_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// The server-generated ID of the report.
    pub id: Option<String>,
    /// The date/time when the job this report belongs to will expire/expired.
    #[serde(rename = "jobExpireTime")]
    pub job_expire_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// The ID of the job that created this report.
    #[serde(rename = "jobId")]
    pub job_id: Option<String>,
    /// The start of the time period that the report instance covers. The value is inclusive.
    #[serde(rename = "startTime")]
    pub start_time: Option<chrono::DateTime<chrono::offset::Utc>>,
}

impl common::ResponseResult for Report {}

/// A report type.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list report types](ReportTypeListCall) (none)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ReportType {
    /// The date/time when this report type was/will be deprecated.
    #[serde(rename = "deprecateTime")]
    pub deprecate_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// The ID of the report type (max. 100 characters).
    pub id: Option<String>,
    /// The name of the report type (max. 100 characters).
    pub name: Option<String>,
    /// True if this a system-managed report type; otherwise false. Reporting jobs for system-managed report types are created automatically and can thus not be used in the `CreateJob` method.
    #[serde(rename = "systemManaged")]
    pub system_managed: Option<bool>,
}

impl common::Resource for ReportType {}

// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *job* resources.
/// It is not used directly, but through the [`YouTubeReporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtubereporting1 as youtubereporting1;
///
/// # async fn dox() {
/// use youtubereporting1::{YouTubeReporting, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// let mut hub = YouTubeReporting::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `list(...)`, `reports_get(...)` and `reports_list(...)`
/// // to build up your call.
/// let rb = hub.jobs();
/// # }
/// ```
pub struct JobMethods<'a, C>
where
    C: 'a,
{
    hub: &'a YouTubeReporting<C>,
}

impl<'a, C> common::MethodsBuilder for JobMethods<'a, C> {}

impl<'a, C> JobMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the metadata of a specific report.
    ///
    /// # Arguments
    ///
    /// * `jobId` - The ID of the job.
    /// * `reportId` - The ID of the report to retrieve.
    pub fn reports_get(&self, job_id: &str, report_id: &str) -> JobReportGetCall<'a, C> {
        JobReportGetCall {
            hub: self.hub,
            _job_id: job_id.to_string(),
            _report_id: report_id.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Lists reports created by a specific job. Returns NOT_FOUND if the job does not exist.
    ///
    /// # Arguments
    ///
    /// * `jobId` - The ID of the job.
    pub fn reports_list(&self, job_id: &str) -> JobReportListCall<'a, C> {
        JobReportListCall {
            hub: self.hub,
            _job_id: job_id.to_string(),
            _start_time_before: Default::default(),
            _start_time_at_or_after: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _created_after: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Creates a job and returns it.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: Job) -> JobCreateCall<'a, C> {
        JobCreateCall {
            hub: self.hub,
            _request: request,
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a job.
    ///
    /// # Arguments
    ///
    /// * `jobId` - The ID of the job to delete.
    pub fn delete(&self, job_id: &str) -> JobDeleteCall<'a, C> {
        JobDeleteCall {
            hub: self.hub,
            _job_id: job_id.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Gets a job.
    ///
    /// # Arguments
    ///
    /// * `jobId` - The ID of the job to retrieve.
    pub fn get(&self, job_id: &str) -> JobGetCall<'a, C> {
        JobGetCall {
            hub: self.hub,
            _job_id: job_id.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Lists jobs.
    pub fn list(&self) -> JobListCall<'a, C> {
        JobListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _include_system_managed: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *media* resources.
/// It is not used directly, but through the [`YouTubeReporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtubereporting1 as youtubereporting1;
///
/// # async fn dox() {
/// use youtubereporting1::{YouTubeReporting, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// let mut hub = YouTubeReporting::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `download(...)`
/// // to build up your call.
/// let rb = hub.media();
/// # }
/// ```
pub struct MediaMethods<'a, C>
where
    C: 'a,
{
    hub: &'a YouTubeReporting<C>,
}

impl<'a, C> common::MethodsBuilder for MediaMethods<'a, C> {}

impl<'a, C> MediaMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Method for media download. Download is supported on the URI `/v1/media/{+name}?alt=media`.
    ///
    /// # Arguments
    ///
    /// * `resourceName` - Name of the media that is being downloaded.
    pub fn download(&self, resource_name: &str) -> MediaDownloadCall<'a, C> {
        MediaDownloadCall {
            hub: self.hub,
            _resource_name: resource_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *reportType* resources.
/// It is not used directly, but through the [`YouTubeReporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtubereporting1 as youtubereporting1;
///
/// # async fn dox() {
/// use youtubereporting1::{YouTubeReporting, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// let mut hub = YouTubeReporting::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.report_types();
/// # }
/// ```
pub struct ReportTypeMethods<'a, C>
where
    C: 'a,
{
    hub: &'a YouTubeReporting<C>,
}

impl<'a, C> common::MethodsBuilder for ReportTypeMethods<'a, C> {}

impl<'a, C> ReportTypeMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Lists report types.
    pub fn list(&self) -> ReportTypeListCall<'a, C> {
        ReportTypeListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _include_system_managed: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

// ###################
// CallBuilders   ###
// #################

/// Gets the metadata of a specific report.
///
/// A builder for the *reports.get* method supported by a *job* resource.
/// It is not used directly, but through a [`JobMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_youtubereporting1 as youtubereporting1;
/// # async fn dox() {
/// # use youtubereporting1::{YouTubeReporting, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = YouTubeReporting::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().reports_get("jobId", "reportId")
///              .on_behalf_of_content_owner("Lorem")
///              .doit().await;
/// # }
/// ```
pub struct JobReportGetCall<'a, C>
where
    C: 'a,
{
    hub: &'a YouTubeReporting<C>,
    _job_id: String,
    _report_id: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for JobReportGetCall<'a, C> {}

impl<'a, C> JobReportGetCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Report)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "youtubereporting.jobs.reports.get",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "jobId", "reportId", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("jobId", self._job_id);
        params.push("reportId", self._report_id);
        if let Some(value) = self._on_behalf_of_content_owner.as_ref() {
            params.push("onBehalfOfContentOwner", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/jobs/{jobId}/reports/{reportId}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::YtAnalyticMonetaryReadonly.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{jobId}", "jobId"), ("{reportId}", "reportId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["reportId", "jobId"];
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

    /// The ID of the job.
    ///
    /// Sets the *job id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn job_id(mut self, new_value: &str) -> JobReportGetCall<'a, C> {
        self._job_id = new_value.to_string();
        self
    }
    /// The ID of the report to retrieve.
    ///
    /// Sets the *report id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn report_id(mut self, new_value: &str) -> JobReportGetCall<'a, C> {
        self._report_id = new_value.to_string();
        self
    }
    /// The content owner's external ID on which behalf the user is acting on. If not set, the user is acting for himself (his own channel).
    ///
    /// Sets the *on behalf of content owner* query property to the given value.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> JobReportGetCall<'a, C> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> JobReportGetCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> JobReportGetCall<'a, C>
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
    /// [`Scope::YtAnalyticMonetaryReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> JobReportGetCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> JobReportGetCall<'a, C>
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
    pub fn clear_scopes(mut self) -> JobReportGetCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Lists reports created by a specific job. Returns NOT_FOUND if the job does not exist.
///
/// A builder for the *reports.list* method supported by a *job* resource.
/// It is not used directly, but through a [`JobMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_youtubereporting1 as youtubereporting1;
/// # async fn dox() {
/// # use youtubereporting1::{YouTubeReporting, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = YouTubeReporting::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().reports_list("jobId")
///              .start_time_before(chrono::Utc::now())
///              .start_time_at_or_after(chrono::Utc::now())
///              .page_token("eos")
///              .page_size(-4)
///              .on_behalf_of_content_owner("ea")
///              .created_after(chrono::Utc::now())
///              .doit().await;
/// # }
/// ```
pub struct JobReportListCall<'a, C>
where
    C: 'a,
{
    hub: &'a YouTubeReporting<C>,
    _job_id: String,
    _start_time_before: Option<chrono::DateTime<chrono::offset::Utc>>,
    _start_time_at_or_after: Option<chrono::DateTime<chrono::offset::Utc>>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _on_behalf_of_content_owner: Option<String>,
    _created_after: Option<chrono::DateTime<chrono::offset::Utc>>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for JobReportListCall<'a, C> {}

impl<'a, C> JobReportListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, ListReportsResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "youtubereporting.jobs.reports.list",
            http_method: hyper::Method::GET,
        });

        for &field in [
            "alt",
            "jobId",
            "startTimeBefore",
            "startTimeAtOrAfter",
            "pageToken",
            "pageSize",
            "onBehalfOfContentOwner",
            "createdAfter",
        ]
        .iter()
        {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(9 + self._additional_params.len());
        params.push("jobId", self._job_id);
        if let Some(value) = self._start_time_before.as_ref() {
            params.push("startTimeBefore", common::serde::datetime_to_string(&value));
        }
        if let Some(value) = self._start_time_at_or_after.as_ref() {
            params.push(
                "startTimeAtOrAfter",
                common::serde::datetime_to_string(&value),
            );
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._on_behalf_of_content_owner.as_ref() {
            params.push("onBehalfOfContentOwner", value);
        }
        if let Some(value) = self._created_after.as_ref() {
            params.push("createdAfter", common::serde::datetime_to_string(&value));
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/jobs/{jobId}/reports";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::YtAnalyticMonetaryReadonly.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{jobId}", "jobId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["jobId"];
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

    /// The ID of the job.
    ///
    /// Sets the *job id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn job_id(mut self, new_value: &str) -> JobReportListCall<'a, C> {
        self._job_id = new_value.to_string();
        self
    }
    /// If set, only reports whose start time is smaller than the specified date/time are returned.
    ///
    /// Sets the *start time before* query property to the given value.
    pub fn start_time_before(
        mut self,
        new_value: chrono::DateTime<chrono::offset::Utc>,
    ) -> JobReportListCall<'a, C> {
        self._start_time_before = Some(new_value);
        self
    }
    /// If set, only reports whose start time is greater than or equal the specified date/time are returned.
    ///
    /// Sets the *start time at or after* query property to the given value.
    pub fn start_time_at_or_after(
        mut self,
        new_value: chrono::DateTime<chrono::offset::Utc>,
    ) -> JobReportListCall<'a, C> {
        self._start_time_at_or_after = Some(new_value);
        self
    }
    /// A token identifying a page of results the server should return. Typically, this is the value of ListReportsResponse.next_page_token returned in response to the previous call to the `ListReports` method.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> JobReportListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Requested page size. Server may return fewer report types than requested. If unspecified, server will pick an appropriate default.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> JobReportListCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// The content owner's external ID on which behalf the user is acting on. If not set, the user is acting for himself (his own channel).
    ///
    /// Sets the *on behalf of content owner* query property to the given value.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> JobReportListCall<'a, C> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// If set, only reports created after the specified date/time are returned.
    ///
    /// Sets the *created after* query property to the given value.
    pub fn created_after(
        mut self,
        new_value: chrono::DateTime<chrono::offset::Utc>,
    ) -> JobReportListCall<'a, C> {
        self._created_after = Some(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> JobReportListCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> JobReportListCall<'a, C>
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
    /// [`Scope::YtAnalyticMonetaryReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> JobReportListCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> JobReportListCall<'a, C>
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
    pub fn clear_scopes(mut self) -> JobReportListCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Creates a job and returns it.
///
/// A builder for the *create* method supported by a *job* resource.
/// It is not used directly, but through a [`JobMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_youtubereporting1 as youtubereporting1;
/// use youtubereporting1::api::Job;
/// # async fn dox() {
/// # use youtubereporting1::{YouTubeReporting, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = YouTubeReporting::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Job::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().create(req)
///              .on_behalf_of_content_owner("ipsum")
///              .doit().await;
/// # }
/// ```
pub struct JobCreateCall<'a, C>
where
    C: 'a,
{
    hub: &'a YouTubeReporting<C>,
    _request: Job,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for JobCreateCall<'a, C> {}

impl<'a, C> JobCreateCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Job)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "youtubereporting.jobs.create",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        if let Some(value) = self._on_behalf_of_content_owner.as_ref() {
            params.push("onBehalfOfContentOwner", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/jobs";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::YtAnalyticMonetaryReadonly.as_ref().to_string());
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
    pub fn request(mut self, new_value: Job) -> JobCreateCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The content owner's external ID on which behalf the user is acting on. If not set, the user is acting for himself (his own channel).
    ///
    /// Sets the *on behalf of content owner* query property to the given value.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> JobCreateCall<'a, C> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> JobCreateCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> JobCreateCall<'a, C>
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
    /// [`Scope::YtAnalyticMonetaryReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> JobCreateCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> JobCreateCall<'a, C>
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
    pub fn clear_scopes(mut self) -> JobCreateCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Deletes a job.
///
/// A builder for the *delete* method supported by a *job* resource.
/// It is not used directly, but through a [`JobMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_youtubereporting1 as youtubereporting1;
/// # async fn dox() {
/// # use youtubereporting1::{YouTubeReporting, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = YouTubeReporting::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().delete("jobId")
///              .on_behalf_of_content_owner("amet")
///              .doit().await;
/// # }
/// ```
pub struct JobDeleteCall<'a, C>
where
    C: 'a,
{
    hub: &'a YouTubeReporting<C>,
    _job_id: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for JobDeleteCall<'a, C> {}

impl<'a, C> JobDeleteCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Empty)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "youtubereporting.jobs.delete",
            http_method: hyper::Method::DELETE,
        });

        for &field in ["alt", "jobId", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("jobId", self._job_id);
        if let Some(value) = self._on_behalf_of_content_owner.as_ref() {
            params.push("onBehalfOfContentOwner", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/jobs/{jobId}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::YtAnalyticMonetaryReadonly.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{jobId}", "jobId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["jobId"];
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
                    .method(hyper::Method::DELETE)
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

    /// The ID of the job to delete.
    ///
    /// Sets the *job id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn job_id(mut self, new_value: &str) -> JobDeleteCall<'a, C> {
        self._job_id = new_value.to_string();
        self
    }
    /// The content owner's external ID on which behalf the user is acting on. If not set, the user is acting for himself (his own channel).
    ///
    /// Sets the *on behalf of content owner* query property to the given value.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> JobDeleteCall<'a, C> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> JobDeleteCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> JobDeleteCall<'a, C>
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
    /// [`Scope::YtAnalyticMonetaryReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> JobDeleteCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> JobDeleteCall<'a, C>
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
    pub fn clear_scopes(mut self) -> JobDeleteCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Gets a job.
///
/// A builder for the *get* method supported by a *job* resource.
/// It is not used directly, but through a [`JobMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_youtubereporting1 as youtubereporting1;
/// # async fn dox() {
/// # use youtubereporting1::{YouTubeReporting, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = YouTubeReporting::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().get("jobId")
///              .on_behalf_of_content_owner("ipsum")
///              .doit().await;
/// # }
/// ```
pub struct JobGetCall<'a, C>
where
    C: 'a,
{
    hub: &'a YouTubeReporting<C>,
    _job_id: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for JobGetCall<'a, C> {}

impl<'a, C> JobGetCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Job)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "youtubereporting.jobs.get",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "jobId", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("jobId", self._job_id);
        if let Some(value) = self._on_behalf_of_content_owner.as_ref() {
            params.push("onBehalfOfContentOwner", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/jobs/{jobId}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::YtAnalyticMonetaryReadonly.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{jobId}", "jobId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["jobId"];
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

    /// The ID of the job to retrieve.
    ///
    /// Sets the *job id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn job_id(mut self, new_value: &str) -> JobGetCall<'a, C> {
        self._job_id = new_value.to_string();
        self
    }
    /// The content owner's external ID on which behalf the user is acting on. If not set, the user is acting for himself (his own channel).
    ///
    /// Sets the *on behalf of content owner* query property to the given value.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> JobGetCall<'a, C> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> JobGetCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> JobGetCall<'a, C>
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
    /// [`Scope::YtAnalyticMonetaryReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> JobGetCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> JobGetCall<'a, C>
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
    pub fn clear_scopes(mut self) -> JobGetCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Lists jobs.
///
/// A builder for the *list* method supported by a *job* resource.
/// It is not used directly, but through a [`JobMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_youtubereporting1 as youtubereporting1;
/// # async fn dox() {
/// # use youtubereporting1::{YouTubeReporting, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = YouTubeReporting::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().list()
///              .page_token("sed")
///              .page_size(-37)
///              .on_behalf_of_content_owner("gubergren")
///              .include_system_managed(true)
///              .doit().await;
/// # }
/// ```
pub struct JobListCall<'a, C>
where
    C: 'a,
{
    hub: &'a YouTubeReporting<C>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _on_behalf_of_content_owner: Option<String>,
    _include_system_managed: Option<bool>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for JobListCall<'a, C> {}

impl<'a, C> JobListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, ListJobsResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "youtubereporting.jobs.list",
            http_method: hyper::Method::GET,
        });

        for &field in [
            "alt",
            "pageToken",
            "pageSize",
            "onBehalfOfContentOwner",
            "includeSystemManaged",
        ]
        .iter()
        {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._on_behalf_of_content_owner.as_ref() {
            params.push("onBehalfOfContentOwner", value);
        }
        if let Some(value) = self._include_system_managed.as_ref() {
            params.push("includeSystemManaged", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/jobs";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::YtAnalyticMonetaryReadonly.as_ref().to_string());
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

    /// A token identifying a page of results the server should return. Typically, this is the value of ListReportTypesResponse.next_page_token returned in response to the previous call to the `ListJobs` method.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> JobListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Requested page size. Server may return fewer jobs than requested. If unspecified, server will pick an appropriate default.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> JobListCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// The content owner's external ID on which behalf the user is acting on. If not set, the user is acting for himself (his own channel).
    ///
    /// Sets the *on behalf of content owner* query property to the given value.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> JobListCall<'a, C> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// If set to true, also system-managed jobs will be returned; otherwise only user-created jobs will be returned. System-managed jobs can neither be modified nor deleted.
    ///
    /// Sets the *include system managed* query property to the given value.
    pub fn include_system_managed(mut self, new_value: bool) -> JobListCall<'a, C> {
        self._include_system_managed = Some(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> JobListCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> JobListCall<'a, C>
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
    /// [`Scope::YtAnalyticMonetaryReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> JobListCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> JobListCall<'a, C>
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
    pub fn clear_scopes(mut self) -> JobListCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Method for media download. Download is supported on the URI `/v1/media/{+name}?alt=media`.
///
/// This method supports **media download**. To enable it, adjust the builder like this:
/// `.param("alt", "media")`.
/// Please note that due to missing multi-part support on the server side, you will only receive the media,
/// but not the `GdataMedia` structure that you would usually get. The latter will be a default value.
///
/// A builder for the *download* method supported by a *media* resource.
/// It is not used directly, but through a [`MediaMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_youtubereporting1 as youtubereporting1;
/// # async fn dox() {
/// # use youtubereporting1::{YouTubeReporting, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = YouTubeReporting::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.media().download("resourceName")
///              .doit().await;
/// # }
/// ```
pub struct MediaDownloadCall<'a, C>
where
    C: 'a,
{
    hub: &'a YouTubeReporting<C>,
    _resource_name: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for MediaDownloadCall<'a, C> {}

impl<'a, C> MediaDownloadCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, GdataMedia)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "youtubereporting.media.download",
            http_method: hyper::Method::GET,
        });

        for &field in ["resourceName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(2 + self._additional_params.len());
        params.push("resourceName", self._resource_name);

        params.extend(self._additional_params.iter());

        let (alt_field_missing, enable_resource_parsing) = {
            if let Some(value) = params.get("alt") {
                (false, value == "json")
            } else {
                (true, true)
            }
        };
        if alt_field_missing {
            params.push("alt", "json");
        }
        let mut url = self.hub._base_url.clone() + "v1/media/{+resourceName}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::YtAnalyticMonetaryReadonly.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+resourceName}", "resourceName")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["resourceName"];
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
                    let response = if enable_resource_parsing {
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
                    } else {
                        (
                            common::Response::from_parts(parts, body),
                            Default::default(),
                        )
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// Name of the media that is being downloaded.
    ///
    /// Sets the *resource name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn resource_name(mut self, new_value: &str) -> MediaDownloadCall<'a, C> {
        self._resource_name = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> MediaDownloadCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> MediaDownloadCall<'a, C>
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
    /// [`Scope::YtAnalyticMonetaryReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MediaDownloadCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MediaDownloadCall<'a, C>
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
    pub fn clear_scopes(mut self) -> MediaDownloadCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Lists report types.
///
/// A builder for the *list* method supported by a *reportType* resource.
/// It is not used directly, but through a [`ReportTypeMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_youtubereporting1 as youtubereporting1;
/// # async fn dox() {
/// # use youtubereporting1::{YouTubeReporting, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = YouTubeReporting::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.report_types().list()
///              .page_token("ipsum")
///              .page_size(-7)
///              .on_behalf_of_content_owner("gubergren")
///              .include_system_managed(false)
///              .doit().await;
/// # }
/// ```
pub struct ReportTypeListCall<'a, C>
where
    C: 'a,
{
    hub: &'a YouTubeReporting<C>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _on_behalf_of_content_owner: Option<String>,
    _include_system_managed: Option<bool>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ReportTypeListCall<'a, C> {}

impl<'a, C> ReportTypeListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, ListReportTypesResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "youtubereporting.reportTypes.list",
            http_method: hyper::Method::GET,
        });

        for &field in [
            "alt",
            "pageToken",
            "pageSize",
            "onBehalfOfContentOwner",
            "includeSystemManaged",
        ]
        .iter()
        {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._on_behalf_of_content_owner.as_ref() {
            params.push("onBehalfOfContentOwner", value);
        }
        if let Some(value) = self._include_system_managed.as_ref() {
            params.push("includeSystemManaged", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/reportTypes";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::YtAnalyticMonetaryReadonly.as_ref().to_string());
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

    /// A token identifying a page of results the server should return. Typically, this is the value of ListReportTypesResponse.next_page_token returned in response to the previous call to the `ListReportTypes` method.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ReportTypeListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Requested page size. Server may return fewer report types than requested. If unspecified, server will pick an appropriate default.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ReportTypeListCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// The content owner's external ID on which behalf the user is acting on. If not set, the user is acting for himself (his own channel).
    ///
    /// Sets the *on behalf of content owner* query property to the given value.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> ReportTypeListCall<'a, C> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// If set to true, also system-managed report types will be returned; otherwise only the report types that can be used to create new reporting jobs will be returned.
    ///
    /// Sets the *include system managed* query property to the given value.
    pub fn include_system_managed(mut self, new_value: bool) -> ReportTypeListCall<'a, C> {
        self._include_system_managed = Some(new_value);
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
    ) -> ReportTypeListCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ReportTypeListCall<'a, C>
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
    /// [`Scope::YtAnalyticMonetaryReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ReportTypeListCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ReportTypeListCall<'a, C>
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
    pub fn clear_scopes(mut self) -> ReportTypeListCall<'a, C> {
        self._scopes.clear();
        self
    }
}
