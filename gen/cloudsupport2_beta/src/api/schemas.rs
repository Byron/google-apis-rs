use super::*;
/// An object containing information about the effective user and authenticated principal responsible for an action.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Actor {
    /// The name to display for the actor. If not provided, it is inferred from credentials supplied during case creation. When an email is provided, a display name must also be provided. This will be obfuscated if the user is a Google Support agent.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The email address of the actor. If not provided, it is inferred from credentials supplied during case creation. If the authenticated principal does not have an email address, one must be provided. When a name is provided, an email must also be provided. This will be obfuscated if the user is a Google Support agent.
    
    pub email: Option<String>,
    /// Output only. Whether the actor is a Google support actor.
    #[serde(rename="googleSupport")]
    
    pub google_support: Option<bool>,
}

impl client::Part for Actor {}


/// Represents a file attached to a support case.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create attachments](AttachmentCreateCall) (response)
/// * [upload media](MediaUploadCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Attachment {
    /// Output only. The time at which the attachment was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The user who uploaded the attachment. Note, the name and email will be obfuscated if the attachment was uploaded by Google support.
    
    pub creator: Option<Actor>,
    /// The filename of the attachment (e.g. `"graph.jpg"`).
    
    pub filename: Option<String>,
    /// Output only. The MIME type of the attachment (e.g. text/plain).
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// Output only. The resource name of the attachment.
    
    pub name: Option<String>,
    /// Output only. The size of the attachment in bytes.
    #[serde(rename="sizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub size_bytes: Option<i64>,
}

impl client::Resource for Attachment {}
impl client::ResponseResult for Attachment {}


/// # gdata.* are outside protos with mising documentation
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Blobstore2Info {
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="blobGeneration")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub blob_generation: Option<i64>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="blobId")]
    
    pub blob_id: Option<String>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="downloadReadHandle")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub download_read_handle: Option<Vec<u8>>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="readToken")]
    
    pub read_token: Option<String>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="uploadMetadataContainer")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub upload_metadata_container: Option<Vec<u8>>,
}

impl client::Part for Blobstore2Info {}


/// A support case.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [attachments list cases](CaseAttachmentListCall) (none)
/// * [comments create cases](CaseCommentCreateCall) (none)
/// * [comments list cases](CaseCommentListCall) (none)
/// * [close cases](CaseCloseCall) (response)
/// * [create cases](CaseCreateCall) (request|response)
/// * [escalate cases](CaseEscalateCall) (response)
/// * [get cases](CaseGetCall) (response)
/// * [list cases](CaseListCall) (none)
/// * [patch cases](CasePatchCall) (request|response)
/// * [search cases](CaseSearchCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Case {
    /// The issue classification applicable to this case.
    
    pub classification: Option<CaseClassification>,
    /// Output only. The time this case was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The user who created the case. Note: The name and email will be obfuscated if the case was created by Google Support.
    
    pub creator: Option<Actor>,
    /// A broad description of the issue.
    
    pub description: Option<String>,
    /// The short summary of the issue reported in this case.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Whether the case is currently escalated.
    
    pub escalated: Option<bool>,
    /// The language the user has requested to receive support in. This should be a BCP 47 language code (e.g., `"en"`, `"zh-CN"`, `"zh-TW"`, `"ja"`, `"ko"`). If no language or an unsupported language is specified, this field defaults to English (en). Language selection during case creation may affect your available support options. For a list of supported languages and their support working hours, see: https://cloud.google.com/support/docs/language-working-hours
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// The resource name for the case.
    
    pub name: Option<String>,
    /// The priority of this case. If this is set, do not set severity.
    
    pub priority: Option<CasePriorityEnum>,
    /// The severity of this case. Deprecated. Use priority instead.
    
    pub severity: Option<CaseSeverityEnum>,
    /// Output only. The current status of the support case.
    
    pub state: Option<CaseStateEnum>,
    /// The email addresses to receive updates on this case.
    #[serde(rename="subscriberEmailAddresses")]
    
    pub subscriber_email_addresses: Option<Vec<String>>,
    /// Whether this case was created for internal API testing and should not be acted on by the support team.
    #[serde(rename="testCase")]
    
    pub test_case: Option<bool>,
    /// The timezone of the user who created the support case. It should be in a format IANA recognizes: https://www.iana.org/time-zones. There is no additional validation done by the API.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
    /// Output only. The time this case was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Case {}
impl client::Resource for Case {}
impl client::ResponseResult for Case {}


/// A classification object with a product type and value.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search case classifications](CaseClassificationSearchCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CaseClassification {
    /// The display name of the classification.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The unique ID for a classification. Must be specified for case creation. To retrieve valid classification IDs for case creation, use `caseClassifications.search`.
    
    pub id: Option<String>,
}

impl client::Resource for CaseClassification {}


/// The request message for the CloseCase endpoint.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [close cases](CaseCloseCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloseCaseRequest { _never_set: Option<bool> }

impl client::RequestValue for CloseCaseRequest {}


/// A comment associated with a support case.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [comments create cases](CaseCommentCreateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Comment {
    /// The full comment body. Maximum of 120000 characters. This can contain rich text syntax.
    
    pub body: Option<String>,
    /// Output only. The time when this comment was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The user or Google Support agent created this comment.
    
    pub creator: Option<Actor>,
    /// Output only. The resource name for the comment.
    
    pub name: Option<String>,
    /// Output only. An automatically generated plain text version of body with all rich text syntax stripped.
    #[serde(rename="plainTextBody")]
    
    pub plain_text_body: Option<String>,
}

impl client::RequestValue for Comment {}
impl client::ResponseResult for Comment {}


/// # gdata.* are outside protos with mising documentation
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompositeMedia {
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="blobRef")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub blob_ref: Option<Vec<u8>>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="blobstore2Info")]
    
    pub blobstore2_info: Option<Blobstore2Info>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="cosmoBinaryReference")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub cosmo_binary_reference: Option<Vec<u8>>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="crc32cHash")]
    
    pub crc32c_hash: Option<u32>,
    /// # gdata.* are outside protos with mising documentation
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub inline: Option<Vec<u8>>,
    /// # gdata.* are outside protos with mising documentation
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub length: Option<i64>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="md5Hash")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub md5_hash: Option<Vec<u8>>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="objectId")]
    
    pub object_id: Option<ObjectId>,
    /// # gdata.* are outside protos with mising documentation
    
    pub path: Option<String>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="referenceType")]
    
    pub reference_type: Option<CompositeMediaReferenceTypeEnum>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="sha1Hash")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub sha1_hash: Option<Vec<u8>>,
}

impl client::Part for CompositeMedia {}


/// # gdata.* are outside protos with mising documentation
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContentTypeInfo {
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="bestGuess")]
    
    pub best_guess: Option<String>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="fromBytes")]
    
    pub from_bytes: Option<String>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="fromFileName")]
    
    pub from_file_name: Option<String>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="fromHeader")]
    
    pub from_header: Option<String>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="fromUrlPath")]
    
    pub from_url_path: Option<String>,
}

impl client::Part for ContentTypeInfo {}


/// The request message for the CreateAttachment endpoint.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create attachments](AttachmentCreateCall) (request)
/// * [upload media](MediaUploadCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateAttachmentRequest {
    /// Required. The attachment to be created.
    
    pub attachment: Option<Attachment>,
}

impl client::RequestValue for CreateAttachmentRequest {}


/// # gdata.* are outside protos with mising documentation
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiffChecksumsResponse {
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="checksumsLocation")]
    
    pub checksums_location: Option<CompositeMedia>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="chunkSizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub chunk_size_bytes: Option<i64>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="objectLocation")]
    
    pub object_location: Option<CompositeMedia>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="objectSizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub object_size_bytes: Option<i64>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="objectVersion")]
    
    pub object_version: Option<String>,
}

impl client::Part for DiffChecksumsResponse {}


/// # gdata.* are outside protos with mising documentation
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiffDownloadResponse {
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="objectLocation")]
    
    pub object_location: Option<CompositeMedia>,
}

impl client::Part for DiffDownloadResponse {}


/// # gdata.* are outside protos with mising documentation
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiffUploadRequest {
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="checksumsInfo")]
    
    pub checksums_info: Option<CompositeMedia>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="objectInfo")]
    
    pub object_info: Option<CompositeMedia>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="objectVersion")]
    
    pub object_version: Option<String>,
}

impl client::Part for DiffUploadRequest {}


/// # gdata.* are outside protos with mising documentation
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiffUploadResponse {
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="objectVersion")]
    
    pub object_version: Option<String>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="originalObject")]
    
    pub original_object: Option<CompositeMedia>,
}

impl client::Part for DiffUploadResponse {}


/// # gdata.* are outside protos with mising documentation
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiffVersionResponse {
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="objectSizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub object_size_bytes: Option<i64>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="objectVersion")]
    
    pub object_version: Option<String>,
}

impl client::Part for DiffVersionResponse {}


/// # gdata.* are outside protos with mising documentation
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DownloadParameters {
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="allowGzipCompression")]
    
    pub allow_gzip_compression: Option<bool>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="ignoreRange")]
    
    pub ignore_range: Option<bool>,
}

impl client::Part for DownloadParameters {}


/// The request message for the EscalateCase endpoint.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [escalate cases](CaseEscalateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EscalateCaseRequest {
    /// The escalation object to be sent with the escalation request.
    
    pub escalation: Option<Escalation>,
}

impl client::RequestValue for EscalateCaseRequest {}


/// An escalation of a support case.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Escalation {
    /// Required. A free text description to accompany the `reason` field above. Provides additional context on why the case is being escalated.
    
    pub justification: Option<String>,
    /// Required. The reason why the Case is being escalated.
    
    pub reason: Option<EscalationReasonEnum>,
}

impl client::Part for Escalation {}


/// The response message for the ListAttachments endpoint.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [attachments list cases](CaseAttachmentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAttachmentsResponse {
    /// The list of attachments associated with the given case.
    
    pub attachments: Option<Vec<Attachment>>,
    /// A token to retrieve the next page of results. This should be set in the `page_token` field of subsequent `cases.attachments.list` requests. If unspecified, there are no more results to retrieve.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAttachmentsResponse {}


/// The response message for the ListCases endpoint.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list cases](CaseListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCasesResponse {
    /// The list of cases associated with the cloud resource, after any filters have been applied.
    
    pub cases: Option<Vec<Case>>,
    /// A token to retrieve the next page of results. This should be set in the `page_token` field of subsequent `ListCasesRequest` message that is issued. If unspecified, there are no more results to retrieve.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListCasesResponse {}


/// The response message for the ListComments endpoint.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [comments list cases](CaseCommentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCommentsResponse {
    /// The list of Comments associated with the given Case.
    
    pub comments: Option<Vec<Comment>>,
    /// A token to retrieve the next page of results. This should be set in the `page_token` field of subsequent `ListCommentsRequest` message that is issued. If unspecified, there are no more results to retrieve.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListCommentsResponse {}


/// # gdata.\* are outside protos with mising documentation
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [download media](MediaDownloadCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Media {
    /// # gdata.* are outside protos with mising documentation
    
    pub algorithm: Option<String>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="bigstoreObjectRef")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub bigstore_object_ref: Option<Vec<u8>>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="blobRef")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub blob_ref: Option<Vec<u8>>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="blobstore2Info")]
    
    pub blobstore2_info: Option<Blobstore2Info>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="compositeMedia")]
    
    pub composite_media: Option<Vec<CompositeMedia>>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="contentType")]
    
    pub content_type: Option<String>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="contentTypeInfo")]
    
    pub content_type_info: Option<ContentTypeInfo>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="cosmoBinaryReference")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub cosmo_binary_reference: Option<Vec<u8>>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="crc32cHash")]
    
    pub crc32c_hash: Option<u32>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="diffChecksumsResponse")]
    
    pub diff_checksums_response: Option<DiffChecksumsResponse>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="diffDownloadResponse")]
    
    pub diff_download_response: Option<DiffDownloadResponse>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="diffUploadRequest")]
    
    pub diff_upload_request: Option<DiffUploadRequest>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="diffUploadResponse")]
    
    pub diff_upload_response: Option<DiffUploadResponse>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="diffVersionResponse")]
    
    pub diff_version_response: Option<DiffVersionResponse>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="downloadParameters")]
    
    pub download_parameters: Option<DownloadParameters>,
    /// # gdata.* are outside protos with mising documentation
    
    pub filename: Option<String>,
    /// # gdata.* are outside protos with mising documentation
    
    pub hash: Option<String>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="hashVerified")]
    
    pub hash_verified: Option<bool>,
    /// # gdata.* are outside protos with mising documentation
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub inline: Option<Vec<u8>>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="isPotentialRetry")]
    
    pub is_potential_retry: Option<bool>,
    /// # gdata.* are outside protos with mising documentation
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub length: Option<i64>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="md5Hash")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub md5_hash: Option<Vec<u8>>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="mediaId")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub media_id: Option<Vec<u8>>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="objectId")]
    
    pub object_id: Option<ObjectId>,
    /// # gdata.* are outside protos with mising documentation
    
    pub path: Option<String>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="referenceType")]
    
    pub reference_type: Option<MediaReferenceTypeEnum>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="sha1Hash")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub sha1_hash: Option<Vec<u8>>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="sha256Hash")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub sha256_hash: Option<Vec<u8>>,
    /// # gdata.* are outside protos with mising documentation
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub timestamp: Option<u64>,
    /// # gdata.* are outside protos with mising documentation
    
    pub token: Option<String>,
}

impl client::ResponseResult for Media {}


/// # gdata.* are outside protos with mising documentation
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ObjectId {
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="bucketName")]
    
    pub bucket_name: Option<String>,
    /// # gdata.* are outside protos with mising documentation
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub generation: Option<i64>,
    /// # gdata.* are outside protos with mising documentation
    #[serde(rename="objectName")]
    
    pub object_name: Option<String>,
}

impl client::Part for ObjectId {}


/// The response message for SearchCaseClassifications endpoint.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search case classifications](CaseClassificationSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchCaseClassificationsResponse {
    /// The classifications retrieved.
    #[serde(rename="caseClassifications")]
    
    pub case_classifications: Option<Vec<CaseClassification>>,
    /// A token to retrieve the next page of results. This should be set in the `page_token` field of subsequent `SearchCaseClassificationsRequest` message that is issued. If unspecified, there are no more results to retrieve.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for SearchCaseClassificationsResponse {}


/// The response message for the SearchCases endpoint.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search cases](CaseSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchCasesResponse {
    /// The list of Case associated with the cloud resource, after any filters have been applied.
    
    pub cases: Option<Vec<Case>>,
    /// A token to retrieve the next page of results. This should be set in the `page_token` field of subsequent `SearchCaseRequest` message that is issued. If unspecified, there are no more results to retrieve.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for SearchCasesResponse {}


