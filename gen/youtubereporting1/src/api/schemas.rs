use super::*;
/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete jobs](JobDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// gdata
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GdataBlobstore2Info {
    /// gdata
    #[serde(rename="blobGeneration")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub blob_generation: Option<i64>,
    /// gdata
    #[serde(rename="blobId")]
    
    pub blob_id: Option<String>,
    /// gdata
    #[serde(rename="downloadReadHandle")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub download_read_handle: Option<Vec<u8>>,
    /// gdata
    #[serde(rename="readToken")]
    
    pub read_token: Option<String>,
    /// gdata
    #[serde(rename="uploadMetadataContainer")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub upload_metadata_container: Option<Vec<u8>>,
}

impl client::Part for GdataBlobstore2Info {}


/// gdata
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GdataCompositeMedia {
    /// gdata
    #[serde(rename="blobRef")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub blob_ref: Option<Vec<u8>>,
    /// gdata
    #[serde(rename="blobstore2Info")]
    
    pub blobstore2_info: Option<GdataBlobstore2Info>,
    /// gdata
    #[serde(rename="cosmoBinaryReference")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub cosmo_binary_reference: Option<Vec<u8>>,
    /// gdata
    #[serde(rename="crc32cHash")]
    
    pub crc32c_hash: Option<u32>,
    /// gdata
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub inline: Option<Vec<u8>>,
    /// gdata
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub length: Option<i64>,
    /// gdata
    #[serde(rename="md5Hash")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub md5_hash: Option<Vec<u8>>,
    /// gdata
    #[serde(rename="objectId")]
    
    pub object_id: Option<GdataObjectId>,
    /// gdata
    
    pub path: Option<String>,
    /// gdata
    #[serde(rename="referenceType")]
    
    pub reference_type: Option<GdataCompositeMediaReferenceTypeEnum>,
    /// gdata
    #[serde(rename="sha1Hash")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub sha1_hash: Option<Vec<u8>>,
}

impl client::Part for GdataCompositeMedia {}


/// gdata
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GdataContentTypeInfo {
    /// gdata
    #[serde(rename="bestGuess")]
    
    pub best_guess: Option<String>,
    /// gdata
    #[serde(rename="fromBytes")]
    
    pub from_bytes: Option<String>,
    /// gdata
    #[serde(rename="fromFileName")]
    
    pub from_file_name: Option<String>,
    /// gdata
    #[serde(rename="fromHeader")]
    
    pub from_header: Option<String>,
    /// gdata
    #[serde(rename="fromUrlPath")]
    
    pub from_url_path: Option<String>,
}

impl client::Part for GdataContentTypeInfo {}


/// gdata
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GdataDiffChecksumsResponse {
    /// gdata
    #[serde(rename="checksumsLocation")]
    
    pub checksums_location: Option<GdataCompositeMedia>,
    /// gdata
    #[serde(rename="chunkSizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub chunk_size_bytes: Option<i64>,
    /// gdata
    #[serde(rename="objectLocation")]
    
    pub object_location: Option<GdataCompositeMedia>,
    /// gdata
    #[serde(rename="objectSizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub object_size_bytes: Option<i64>,
    /// gdata
    #[serde(rename="objectVersion")]
    
    pub object_version: Option<String>,
}

impl client::Part for GdataDiffChecksumsResponse {}


/// gdata
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GdataDiffDownloadResponse {
    /// gdata
    #[serde(rename="objectLocation")]
    
    pub object_location: Option<GdataCompositeMedia>,
}

impl client::Part for GdataDiffDownloadResponse {}


/// gdata
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GdataDiffUploadRequest {
    /// gdata
    #[serde(rename="checksumsInfo")]
    
    pub checksums_info: Option<GdataCompositeMedia>,
    /// gdata
    #[serde(rename="objectInfo")]
    
    pub object_info: Option<GdataCompositeMedia>,
    /// gdata
    #[serde(rename="objectVersion")]
    
    pub object_version: Option<String>,
}

impl client::Part for GdataDiffUploadRequest {}


/// gdata
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GdataDiffUploadResponse {
    /// gdata
    #[serde(rename="objectVersion")]
    
    pub object_version: Option<String>,
    /// gdata
    #[serde(rename="originalObject")]
    
    pub original_object: Option<GdataCompositeMedia>,
}

impl client::Part for GdataDiffUploadResponse {}


/// gdata
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GdataDiffVersionResponse {
    /// gdata
    #[serde(rename="objectSizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub object_size_bytes: Option<i64>,
    /// gdata
    #[serde(rename="objectVersion")]
    
    pub object_version: Option<String>,
}

impl client::Part for GdataDiffVersionResponse {}


/// gdata
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GdataDownloadParameters {
    /// gdata
    #[serde(rename="allowGzipCompression")]
    
    pub allow_gzip_compression: Option<bool>,
    /// gdata
    #[serde(rename="ignoreRange")]
    
    pub ignore_range: Option<bool>,
}

impl client::Part for GdataDownloadParameters {}


/// gdata
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [download media](MediaDownloadCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GdataMedia {
    /// gdata
    
    pub algorithm: Option<String>,
    /// gdata
    #[serde(rename="bigstoreObjectRef")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub bigstore_object_ref: Option<Vec<u8>>,
    /// gdata
    #[serde(rename="blobRef")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub blob_ref: Option<Vec<u8>>,
    /// gdata
    #[serde(rename="blobstore2Info")]
    
    pub blobstore2_info: Option<GdataBlobstore2Info>,
    /// gdata
    #[serde(rename="compositeMedia")]
    
    pub composite_media: Option<Vec<GdataCompositeMedia>>,
    /// gdata
    #[serde(rename="contentType")]
    
    pub content_type: Option<String>,
    /// gdata
    #[serde(rename="contentTypeInfo")]
    
    pub content_type_info: Option<GdataContentTypeInfo>,
    /// gdata
    #[serde(rename="cosmoBinaryReference")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub cosmo_binary_reference: Option<Vec<u8>>,
    /// gdata
    #[serde(rename="crc32cHash")]
    
    pub crc32c_hash: Option<u32>,
    /// gdata
    #[serde(rename="diffChecksumsResponse")]
    
    pub diff_checksums_response: Option<GdataDiffChecksumsResponse>,
    /// gdata
    #[serde(rename="diffDownloadResponse")]
    
    pub diff_download_response: Option<GdataDiffDownloadResponse>,
    /// gdata
    #[serde(rename="diffUploadRequest")]
    
    pub diff_upload_request: Option<GdataDiffUploadRequest>,
    /// gdata
    #[serde(rename="diffUploadResponse")]
    
    pub diff_upload_response: Option<GdataDiffUploadResponse>,
    /// gdata
    #[serde(rename="diffVersionResponse")]
    
    pub diff_version_response: Option<GdataDiffVersionResponse>,
    /// gdata
    #[serde(rename="downloadParameters")]
    
    pub download_parameters: Option<GdataDownloadParameters>,
    /// gdata
    
    pub filename: Option<String>,
    /// gdata
    
    pub hash: Option<String>,
    /// gdata
    #[serde(rename="hashVerified")]
    
    pub hash_verified: Option<bool>,
    /// gdata
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub inline: Option<Vec<u8>>,
    /// gdata
    #[serde(rename="isPotentialRetry")]
    
    pub is_potential_retry: Option<bool>,
    /// gdata
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub length: Option<i64>,
    /// gdata
    #[serde(rename="md5Hash")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub md5_hash: Option<Vec<u8>>,
    /// gdata
    #[serde(rename="mediaId")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub media_id: Option<Vec<u8>>,
    /// gdata
    #[serde(rename="objectId")]
    
    pub object_id: Option<GdataObjectId>,
    /// gdata
    
    pub path: Option<String>,
    /// gdata
    #[serde(rename="referenceType")]
    
    pub reference_type: Option<GdataMediaReferenceTypeEnum>,
    /// gdata
    #[serde(rename="sha1Hash")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub sha1_hash: Option<Vec<u8>>,
    /// gdata
    #[serde(rename="sha256Hash")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub sha256_hash: Option<Vec<u8>>,
    /// gdata
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub timestamp: Option<u64>,
    /// gdata
    
    pub token: Option<String>,
}

impl client::ResponseResult for GdataMedia {}


/// gdata
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GdataObjectId {
    /// gdata
    #[serde(rename="bucketName")]
    
    pub bucket_name: Option<String>,
    /// gdata
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub generation: Option<i64>,
    /// gdata
    #[serde(rename="objectName")]
    
    pub object_name: Option<String>,
}

impl client::Part for GdataObjectId {}


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
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    /// The creation date/time of the job.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The date/time when this job will expire/expired. After a job expired, no new reports are generated.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The server-generated ID of the job (max. 40 characters).
    
    pub id: Option<String>,
    /// The name of the job (max. 100 characters).
    
    pub name: Option<String>,
    /// The type of reports this job creates. Corresponds to the ID of a ReportType.
    #[serde(rename="reportTypeId")]
    
    pub report_type_id: Option<String>,
    /// True if this a system-managed job that cannot be modified by the user; otherwise false.
    #[serde(rename="systemManaged")]
    
    pub system_managed: Option<bool>,
}

impl client::RequestValue for Job {}
impl client::Resource for Job {}
impl client::ResponseResult for Job {}


/// Response message for ReportingService.ListJobs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list jobs](JobListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListJobsResponse {
    /// The list of jobs.
    
    pub jobs: Option<Vec<Job>>,
    /// A token to retrieve next page of results. Pass this value in the ListJobsRequest.page_token field in the subsequent call to `ListJobs` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListJobsResponse {}


/// Response message for ReportingService.ListReportTypes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list report types](ReportTypeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListReportTypesResponse {
    /// A token to retrieve next page of results. Pass this value in the ListReportTypesRequest.page_token field in the subsequent call to `ListReportTypes` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of report types.
    #[serde(rename="reportTypes")]
    
    pub report_types: Option<Vec<ReportType>>,
}

impl client::ResponseResult for ListReportTypesResponse {}


/// Response message for ReportingService.ListReports.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports list jobs](JobReportListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListReportsResponse {
    /// A token to retrieve next page of results. Pass this value in the ListReportsRequest.page_token field in the subsequent call to `ListReports` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of report types.
    
    pub reports: Option<Vec<Report>>,
}

impl client::ResponseResult for ListReportsResponse {}


/// A report’s metadata including the URL from which the report itself can be downloaded.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports get jobs](JobReportGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Report {
    /// The date/time when this report was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The URL from which the report can be downloaded (max. 1000 characters).
    #[serde(rename="downloadUrl")]
    
    pub download_url: Option<String>,
    /// The end of the time period that the report instance covers. The value is exclusive.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The server-generated ID of the report.
    
    pub id: Option<String>,
    /// The date/time when the job this report belongs to will expire/expired.
    #[serde(rename="jobExpireTime")]
    
    pub job_expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The ID of the job that created this report.
    #[serde(rename="jobId")]
    
    pub job_id: Option<String>,
    /// The start of the time period that the report instance covers. The value is inclusive.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for Report {}


/// A report type.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list report types](ReportTypeListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportType {
    /// The date/time when this report type was/will be deprecated.
    #[serde(rename="deprecateTime")]
    
    pub deprecate_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The ID of the report type (max. 100 characters).
    
    pub id: Option<String>,
    /// The name of the report type (max. 100 characters).
    
    pub name: Option<String>,
    /// True if this a system-managed report type; otherwise false. Reporting jobs for system-managed report types are created automatically and can thus not be used in the `CreateJob` method.
    #[serde(rename="systemManaged")]
    
    pub system_managed: Option<bool>,
}

impl client::Resource for ReportType {}


