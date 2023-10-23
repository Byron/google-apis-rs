use super::*;
/// The accounts to search
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    /// A set of accounts to search.
    
    pub emails: Option<Vec<String>>,
}

impl client::Part for AccountInfo {}


/// The status of each account creation, and the **HeldAccount**, if successful.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddHeldAccountResult {
    /// Returned when the account was successfully created.
    
    pub account: Option<HeldAccount>,
    /// Reports the request status. If it failed, returns an error message.
    
    pub status: Option<Status>,
}

impl client::Part for AddHeldAccountResult {}


/// Add a list of accounts to a hold.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [holds add held accounts matters](MatterHoldAddHeldAccountCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddHeldAccountsRequest {
    /// A comma-separated list of the account IDs of the accounts to add to the hold. Specify either **emails** or **account_ids**, but not both.
    #[serde(rename="accountIds")]
    
    pub account_ids: Option<Vec<String>>,
    /// A comma-separated list of the emails of the accounts to add to the hold. Specify either **emails** or **account_ids**, but not both.
    
    pub emails: Option<Vec<String>>,
}

impl client::RequestValue for AddHeldAccountsRequest {}


/// Response for batch create held accounts.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [holds add held accounts matters](MatterHoldAddHeldAccountCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddHeldAccountsResponse {
    /// The list of responses, in the same order as the batch request.
    
    pub responses: Option<Vec<AddHeldAccountResult>>,
}

impl client::ResponseResult for AddHeldAccountsResponse {}


/// Add an account with the permission specified. The role cannot be owner. If an account already has a role in the matter, the existing role is overwritten.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [add permissions matters](MatterAddPermissionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddMatterPermissionsRequest {
    /// Only relevant if **sendEmails** is **true**. To CC the requestor in the email message, set to **true**. To not CC requestor, set to **false**.
    #[serde(rename="ccMe")]
    
    pub cc_me: Option<bool>,
    /// The account and its role to add.
    #[serde(rename="matterPermission")]
    
    pub matter_permission: Option<MatterPermission>,
    /// To send a notification email to the added account, set to **true**. To not send a notification email, set to **false**.
    #[serde(rename="sendEmails")]
    
    pub send_emails: Option<bool>,
}

impl client::RequestValue for AddMatterPermissionsRequest {}


/// The request message for Operations.CancelOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [cancel operations](OperationCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelOperationRequest {}


/// Close a matter by ID.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [close matters](MatterCloseCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloseMatterRequest { _never_set: Option<bool> }

impl client::RequestValue for CloseMatterRequest {}


/// Response to a CloseMatterRequest.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [close matters](MatterCloseCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloseMatterResponse {
    /// The updated matter, with state **CLOSED**.
    
    pub matter: Option<Matter>,
}

impl client::ResponseResult for CloseMatterResponse {}


/// The export file in Cloud Storage
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudStorageFile {
    /// The name of the Cloud Storage bucket for the export file. You can use this value in the [Cloud Storage JSON or XML APIs](https://cloud.google.com/storage/docs/apis), but not to list the bucket contents. Instead, you can [get individual export files](https://cloud.google.com/storage/docs/json_api/v1/objects/get) by object name.
    #[serde(rename="bucketName")]
    
    pub bucket_name: Option<String>,
    /// The md5 hash of the file.
    #[serde(rename="md5Hash")]
    
    pub md5_hash: Option<String>,
    /// The name of the Cloud Storage object for the export file. You can use this value in the [Cloud Storage JSON or XML APIs](https://cloud.google.com/storage/docs/apis).
    #[serde(rename="objectName")]
    
    pub object_name: Option<String>,
    /// The export file size.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub size: Option<i64>,
}

impl client::Part for CloudStorageFile {}


/// Export sink for Cloud Storage files.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudStorageSink {
    /// Output only. The exported files in Cloud Storage.
    
    pub files: Option<Vec<CloudStorageFile>>,
}

impl client::Part for CloudStorageSink {}


/// Service-specific options for holds.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CorpusQuery {
    /// Service-specific options for Drive holds. If set, **CorpusType** must be **DRIVE**.
    #[serde(rename="driveQuery")]
    
    pub drive_query: Option<HeldDriveQuery>,
    /// Service-specific options for Groups holds. If set, **CorpusType** must be **GROUPS**.
    #[serde(rename="groupsQuery")]
    
    pub groups_query: Option<HeldGroupsQuery>,
    /// Service-specific options for Chat holds. If set, **CorpusType** must be **HANGOUTS_CHAT**.
    #[serde(rename="hangoutsChatQuery")]
    
    pub hangouts_chat_query: Option<HeldHangoutsChatQuery>,
    /// Service-specific options for Gmail holds. If set, **CorpusType** must be **MAIL**.
    #[serde(rename="mailQuery")]
    
    pub mail_query: Option<HeldMailQuery>,
    /// Service-specific options for Voice holds. If set, **CorpusType** must be **VOICE**.
    #[serde(rename="voiceQuery")]
    
    pub voice_query: Option<HeldVoiceQuery>,
}

impl client::Part for CorpusQuery {}


/// Count artifacts request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [count matters](MatterCountCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CountArtifactsRequest {
    /// The search query.
    
    pub query: Option<Query>,
    /// Sets the granularity of the count results.
    
    pub view: Option<CountArtifactsRequestViewEnum>,
}

impl client::RequestValue for CountArtifactsRequest {}


/// Options for Drive exports.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveExportOptions {
    /// To include access level information for users with [indirect access](https://support.google.com/vault/answer/6099459#metadata) to files, set to **true**.
    #[serde(rename="includeAccessInfo")]
    
    pub include_access_info: Option<bool>,
}

impl client::Part for DriveExportOptions {}


/// Additional options for Drive search
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveOptions {
    /// Set whether the results include only content encrypted with [Google Workspace Client-side encryption](https://support.google.com/a?p=cse_ov) content, only unencrypted content, or both. Defaults to both. Currently supported for Drive.
    #[serde(rename="clientSideEncryptedOption")]
    
    pub client_side_encrypted_option: Option<DriveOptionClientSideEncryptedOptionEnum>,
    /// Set to **true** to include shared drives.
    #[serde(rename="includeSharedDrives")]
    
    pub include_shared_drives: Option<bool>,
    /// Set to true to include Team Drive.
    #[serde(rename="includeTeamDrives")]
    
    pub include_team_drives: Option<bool>,
    /// Search the current version of the Drive file, but export the contents of the last version saved before 12:00 AM UTC on the specified date. Enter the date in UTC.
    #[serde(rename="versionDate")]
    
    pub version_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for DriveOptions {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [exports delete matters](MatterExportDeleteCall) (response)
/// * [holds accounts delete matters](MatterHoldAccountDeleteCall) (response)
/// * [holds delete matters](MatterHoldDeleteCall) (response)
/// * [saved queries delete matters](MatterSavedQueryDeleteCall) (response)
/// * [remove permissions matters](MatterRemovePermissionCall) (response)
/// * [cancel operations](OperationCancelCall) (response)
/// * [delete operations](OperationDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// An export. To work with Vault resources, the account must have the [required Vault privileges](https://support.google.com/vault/answer/2799699) and access to the matter. To access a matter, the account must have created the matter, have the matter shared with them, or have the **View All Matters** privilege.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [exports create matters](MatterExportCreateCall) (request|response)
/// * [exports get matters](MatterExportGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Export {
    /// Output only. The sink for export files in Cloud Storage.
    #[serde(rename="cloudStorageSink")]
    
    pub cloud_storage_sink: Option<CloudStorageSink>,
    /// Output only. The time when the export was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Additional export options.
    #[serde(rename="exportOptions")]
    
    pub export_options: Option<ExportOptions>,
    /// Output only. The generated export ID.
    
    pub id: Option<String>,
    /// Output only. The matter ID.
    #[serde(rename="matterId")]
    
    pub matter_id: Option<String>,
    /// The export name. Don't use special characters (~!$'(),;@:/?) in the name, they can prevent you from downloading exports.
    
    pub name: Option<String>,
    /// The query parameters used to create the export.
    
    pub query: Option<Query>,
    /// Output only. The requester of the export.
    
    pub requester: Option<UserInfo>,
    /// Output only. Details about the export progress and size.
    
    pub stats: Option<ExportStats>,
    /// Output only. The status of the export.
    
    pub status: Option<ExportStatusEnum>,
}

impl client::RequestValue for Export {}
impl client::ResponseResult for Export {}


/// Additional options for exports
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExportOptions {
    /// Options for Drive exports.
    #[serde(rename="driveOptions")]
    
    pub drive_options: Option<DriveExportOptions>,
    /// Options for Groups exports.
    #[serde(rename="groupsOptions")]
    
    pub groups_options: Option<GroupsExportOptions>,
    /// Options for Chat exports.
    #[serde(rename="hangoutsChatOptions")]
    
    pub hangouts_chat_options: Option<HangoutsChatExportOptions>,
    /// Options for Gmail exports.
    #[serde(rename="mailOptions")]
    
    pub mail_options: Option<MailExportOptions>,
    /// The requested data region for the export.
    
    pub region: Option<ExportOptionRegionEnum>,
    /// Options for Voice exports.
    #[serde(rename="voiceOptions")]
    
    pub voice_options: Option<VoiceExportOptions>,
}

impl client::Part for ExportOptions {}


/// Progress information for an export.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExportStats {
    /// The number of messages or files already processed for export.
    #[serde(rename="exportedArtifactCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub exported_artifact_count: Option<i64>,
    /// The size of export in bytes.
    #[serde(rename="sizeInBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub size_in_bytes: Option<i64>,
    /// The number of messages or files to be exported.
    #[serde(rename="totalArtifactCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_artifact_count: Option<i64>,
}

impl client::Part for ExportStats {}


/// Options for Groups exports.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GroupsExportOptions {
    /// The file format for exported messages.
    #[serde(rename="exportFormat")]
    
    pub export_format: Option<GroupsExportOptionExportFormatEnum>,
}

impl client::Part for GroupsExportOptions {}


/// Options for Chat exports.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HangoutsChatExportOptions {
    /// The file format for exported messages.
    #[serde(rename="exportFormat")]
    
    pub export_format: Option<HangoutsChatExportOptionExportFormatEnum>,
}

impl client::Part for HangoutsChatExportOptions {}


/// The Chat spaces to search
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HangoutsChatInfo {
    /// A list of Chat spaces IDs, as provided by the [Chat API](https://developers.google.com/hangouts/chat).
    #[serde(rename="roomId")]
    
    pub room_id: Option<Vec<String>>,
}

impl client::Part for HangoutsChatInfo {}


/// Additional options for Google Chat search
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HangoutsChatOptions {
    /// For searches by account or organizational unit, set to **true** to include rooms.
    #[serde(rename="includeRooms")]
    
    pub include_rooms: Option<bool>,
}

impl client::Part for HangoutsChatOptions {}


/// An account covered by a hold. This structure is immutable. It can be an individual account or a Google Group, depending on the service. To work with Vault resources, the account must have the \[required Vault privileges\] (https://support.google.com/vault/answer/2799699) and access to the matter. To access a matter, the account must have created the matter, have the matter shared with them, or have the **View All Matters** privilege.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [holds accounts create matters](MatterHoldAccountCreateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HeldAccount {
    /// The account ID, as provided by the [Admin SDK](https://developers.google.com/admin-sdk/).
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// The primary email address of the account. If used as an input, this takes precedence over **accountId**.
    
    pub email: Option<String>,
    /// Output only. The first name of the account holder.
    #[serde(rename="firstName")]
    
    pub first_name: Option<String>,
    /// Output only. When the account was put on hold.
    #[serde(rename="holdTime")]
    
    pub hold_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The last name of the account holder.
    #[serde(rename="lastName")]
    
    pub last_name: Option<String>,
}

impl client::RequestValue for HeldAccount {}
impl client::ResponseResult for HeldAccount {}


/// Options for Drive holds.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HeldDriveQuery {
    /// To include files in shared drives in the hold, set to **true**.
    #[serde(rename="includeSharedDriveFiles")]
    
    pub include_shared_drive_files: Option<bool>,
    /// To include files in Team Drives in the hold, set to **true**.
    #[serde(rename="includeTeamDriveFiles")]
    
    pub include_team_drive_files: Option<bool>,
}

impl client::Part for HeldDriveQuery {}


/// Query options for group holds.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HeldGroupsQuery {
    /// The end time for the query. Specify in GMT. The value is rounded to 12 AM on the specified date.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The start time for the query. Specify in GMT. The value is rounded to 12 AM on the specified date.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The [search operators](https://support.google.com/vault/answer/2474474) used to refine the messages covered by the hold.
    
    pub terms: Option<String>,
}

impl client::Part for HeldGroupsQuery {}


/// Options for Chat holds.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HeldHangoutsChatQuery {
    /// To include messages in Chat spaces the user was a member of, set to **true**.
    #[serde(rename="includeRooms")]
    
    pub include_rooms: Option<bool>,
}

impl client::Part for HeldHangoutsChatQuery {}


/// Query options for Gmail holds.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HeldMailQuery {
    /// The end time for the query. Specify in GMT. The value is rounded to 12 AM on the specified date.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The start time for the query. Specify in GMT. The value is rounded to 12 AM on the specified date.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The [search operators](https://support.google.com/vault/answer/2474474) used to refine the messages covered by the hold.
    
    pub terms: Option<String>,
}

impl client::Part for HeldMailQuery {}


/// The organizational unit covered by a hold. This structure is immutable.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HeldOrgUnit {
    /// When the organizational unit was put on hold. This property is immutable.
    #[serde(rename="holdTime")]
    
    pub hold_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The organizational unit's immutable ID as provided by the [Admin SDK](https://developers.google.com/admin-sdk/).
    #[serde(rename="orgUnitId")]
    
    pub org_unit_id: Option<String>,
}

impl client::Part for HeldOrgUnit {}


/// Options for Voice holds.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HeldVoiceQuery {
    /// A list of data types covered by the hold. Should be non-empty. Order does not matter and duplicates are ignored.
    #[serde(rename="coveredData")]
    
    pub covered_data: Option<Vec<HeldVoiceQueryCoveredDataEnum>>,
}

impl client::Part for HeldVoiceQuery {}


/// A hold. A hold prevents the specified Google Workspace service from purging data for specific accounts or all members of an organizational unit. To work with Vault resources, the account must have the \[required Vault privileges\] (https://support.google.com/vault/answer/2799699) and access to the matter. To access a matter, the account must have created the matter, have the matter shared with them, or have the **View All Matters** privilege.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [holds create matters](MatterHoldCreateCall) (request|response)
/// * [holds get matters](MatterHoldGetCall) (response)
/// * [holds update matters](MatterHoldUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Hold {
    /// If set, the hold applies to the specified accounts and **orgUnit** must be empty.
    
    pub accounts: Option<Vec<HeldAccount>>,
    /// The service to be searched.
    
    pub corpus: Option<HoldCorpusEnum>,
    /// The unique immutable ID of the hold. Assigned during creation.
    #[serde(rename="holdId")]
    
    pub hold_id: Option<String>,
    /// The name of the hold.
    
    pub name: Option<String>,
    /// If set, the hold applies to all members of the organizational unit and **accounts** must be empty. This property is mutable. For Groups holds, set **accounts**.
    #[serde(rename="orgUnit")]
    
    pub org_unit: Option<HeldOrgUnit>,
    /// Service-specific options. If set, **CorpusQuery** must match **CorpusType**.
    
    pub query: Option<CorpusQuery>,
    /// The last time this hold was modified.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Hold {}
impl client::ResponseResult for Hold {}


/// The exports for a matter.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [exports list matters](MatterExportListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListExportsResponse {
    /// The list of exports.
    
    pub exports: Option<Vec<Export>>,
    /// Page token to retrieve the next page of results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListExportsResponse {}


/// Returns a list of the accounts covered by a hold.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [holds accounts list matters](MatterHoldAccountListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListHeldAccountsResponse {
    /// The held accounts on a hold.
    
    pub accounts: Option<Vec<HeldAccount>>,
}

impl client::ResponseResult for ListHeldAccountsResponse {}


/// The holds for a matter.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [holds list matters](MatterHoldListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListHoldsResponse {
    /// The list of holds.
    
    pub holds: Option<Vec<Hold>>,
    /// Page token to retrieve the next page of results in the list. If this is empty, then there are no more holds to list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListHoldsResponse {}


/// Provides the list of matters.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list matters](MatterListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMattersResponse {
    /// List of matters.
    
    pub matters: Option<Vec<Matter>>,
    /// Page token to retrieve the next page of results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListMattersResponse {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list operations](OperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOperationsResponse {
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of operations that matches the specified filter in the request.
    
    pub operations: Option<Vec<Operation>>,
}

impl client::ResponseResult for ListOperationsResponse {}


/// Definition of the response for method ListSaveQuery.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [saved queries list matters](MatterSavedQueryListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSavedQueriesResponse {
    /// Page token to retrieve the next page of results in the list. If this is empty, then there are no more saved queries to list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of saved queries.
    #[serde(rename="savedQueries")]
    
    pub saved_queries: Option<Vec<SavedQuery>>,
}

impl client::ResponseResult for ListSavedQueriesResponse {}


/// Options for Gmail exports.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MailExportOptions {
    /// The file format for exported messages.
    #[serde(rename="exportFormat")]
    
    pub export_format: Option<MailExportOptionExportFormatEnum>,
    /// To export confidential mode content, set to **true**.
    #[serde(rename="showConfidentialModeContent")]
    
    pub show_confidential_mode_content: Option<bool>,
    /// To use the new export system, set to **true**.
    #[serde(rename="useNewExport")]
    
    pub use_new_export: Option<bool>,
}

impl client::Part for MailExportOptions {}


/// Additional options for Gmail search
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MailOptions {
    /// Set to **true** to exclude drafts.
    #[serde(rename="excludeDrafts")]
    
    pub exclude_drafts: Option<bool>,
}

impl client::Part for MailOptions {}


/// Represents a matter. To work with Vault resources, the account must have the \[required Vault privileges\] (https://support.google.com/vault/answer/2799699) and access to the matter. To access a matter, the account must have created the matter, have the matter shared with them, or have the **View All Matters** privilege.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [exports create matters](MatterExportCreateCall) (none)
/// * [exports delete matters](MatterExportDeleteCall) (none)
/// * [exports get matters](MatterExportGetCall) (none)
/// * [exports list matters](MatterExportListCall) (none)
/// * [holds accounts create matters](MatterHoldAccountCreateCall) (none)
/// * [holds accounts delete matters](MatterHoldAccountDeleteCall) (none)
/// * [holds accounts list matters](MatterHoldAccountListCall) (none)
/// * [holds add held accounts matters](MatterHoldAddHeldAccountCall) (none)
/// * [holds create matters](MatterHoldCreateCall) (none)
/// * [holds delete matters](MatterHoldDeleteCall) (none)
/// * [holds get matters](MatterHoldGetCall) (none)
/// * [holds list matters](MatterHoldListCall) (none)
/// * [holds remove held accounts matters](MatterHoldRemoveHeldAccountCall) (none)
/// * [holds update matters](MatterHoldUpdateCall) (none)
/// * [saved queries create matters](MatterSavedQueryCreateCall) (none)
/// * [saved queries delete matters](MatterSavedQueryDeleteCall) (none)
/// * [saved queries get matters](MatterSavedQueryGetCall) (none)
/// * [saved queries list matters](MatterSavedQueryListCall) (none)
/// * [add permissions matters](MatterAddPermissionCall) (none)
/// * [close matters](MatterCloseCall) (none)
/// * [count matters](MatterCountCall) (none)
/// * [create matters](MatterCreateCall) (request|response)
/// * [delete matters](MatterDeleteCall) (response)
/// * [get matters](MatterGetCall) (response)
/// * [list matters](MatterListCall) (none)
/// * [remove permissions matters](MatterRemovePermissionCall) (none)
/// * [reopen matters](MatterReopenCall) (none)
/// * [undelete matters](MatterUndeleteCall) (response)
/// * [update matters](MatterUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Matter {
    /// An optional description for the matter.
    
    pub description: Option<String>,
    /// The matter ID, which is generated by the server. Leave blank when creating a matter.
    #[serde(rename="matterId")]
    
    pub matter_id: Option<String>,
    /// Lists the users and their permission for the matter. Currently there is no programmer defined limit on the number of permissions a matter can have.
    #[serde(rename="matterPermissions")]
    
    pub matter_permissions: Option<Vec<MatterPermission>>,
    /// The name of the matter.
    
    pub name: Option<String>,
    /// The state of the matter.
    
    pub state: Option<MatterStateEnum>,
}

impl client::RequestValue for Matter {}
impl client::Resource for Matter {}
impl client::ResponseResult for Matter {}


/// Users can be matter owners or collaborators. Each matter has only one owner. All others users who can access the matter are collaborators. When an account is purged, its corresponding MatterPermission resources cease to exist.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [add permissions matters](MatterAddPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MatterPermission {
    /// The account ID, as provided by the [Admin SDK](https://developers.google.com/admin-sdk/).
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// The user's role for the matter.
    
    pub role: Option<MatterPermissionRoleEnum>,
}

impl client::ResponseResult for MatterPermission {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [count matters](MatterCountCall) (response)
/// * [cancel operations](OperationCancelCall) (none)
/// * [delete operations](OperationDeleteCall) (none)
/// * [get operations](OperationGetCall) (response)
/// * [list operations](OperationListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<Status>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::Resource for Operation {}
impl client::ResponseResult for Operation {}


/// The organizational unit to search
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrgUnitInfo {
    /// The name of the organizational unit to search, as provided by the [Admin SDK Directory API](https://developers.google.com/admin-sdk/directory/).
    #[serde(rename="orgUnitId")]
    
    pub org_unit_id: Option<String>,
}

impl client::Part for OrgUnitInfo {}


/// The query definition used for search and export.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Query {
    /// Required when **SearchMethod** is **ACCOUNT**.
    #[serde(rename="accountInfo")]
    
    pub account_info: Option<AccountInfo>,
    /// The Google Workspace service to search.
    
    pub corpus: Option<QueryCorpusEnum>,
    /// The data source to search.
    #[serde(rename="dataScope")]
    
    pub data_scope: Option<QueryDataScopeEnum>,
    /// Set Drive search-specific options.
    #[serde(rename="driveOptions")]
    
    pub drive_options: Option<DriveOptions>,
    /// The end time for the search query. Specify in GMT. The value is rounded to 12 AM on the specified date.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required when **SearchMethod** is **ROOM**. (read-only)
    #[serde(rename="hangoutsChatInfo")]
    
    pub hangouts_chat_info: Option<HangoutsChatInfo>,
    /// Set Chat search-specific options. (read-only)
    #[serde(rename="hangoutsChatOptions")]
    
    pub hangouts_chat_options: Option<HangoutsChatOptions>,
    /// Set Gmail search-specific options.
    #[serde(rename="mailOptions")]
    
    pub mail_options: Option<MailOptions>,
    /// The entity to search. This field replaces **searchMethod** to support shared drives. When **searchMethod** is **TEAM_DRIVE**, the response of this field is **SHARED_DRIVE**.
    
    pub method: Option<QueryMethodEnum>,
    /// Required when **SearchMethod** is **ORG_UNIT**.
    #[serde(rename="orgUnitInfo")]
    
    pub org_unit_info: Option<OrgUnitInfo>,
    /// The search method to use.
    #[serde(rename="searchMethod")]
    
    pub search_method: Option<QuerySearchMethodEnum>,
    /// Required when **SearchMethod** is **SHARED_DRIVE**.
    #[serde(rename="sharedDriveInfo")]
    
    pub shared_drive_info: Option<SharedDriveInfo>,
    /// Required when **SearchMethod** is **SITES_URL**.
    #[serde(rename="sitesUrlInfo")]
    
    pub sites_url_info: Option<SitesUrlInfo>,
    /// The start time for the search query. Specify in GMT. The value is rounded to 12 AM on the specified date.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required when **SearchMethod** is **TEAM_DRIVE**.
    #[serde(rename="teamDriveInfo")]
    
    pub team_drive_info: Option<TeamDriveInfo>,
    /// Service-specific [search operators](https://support.google.com/vault/answer/2474474) to filter search results.
    
    pub terms: Option<String>,
    /// The time zone name. It should be an IANA TZ name, such as "America/Los_Angeles". For a list of time zone names, see [Time Zone](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones). For more information about how Vault uses time zones, see [the Vault help center](https://support.google.com/vault/answer/6092995#time).
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
    /// Set Voice search-specific options.
    #[serde(rename="voiceOptions")]
    
    pub voice_options: Option<VoiceOptions>,
}

impl client::Part for Query {}


/// Remove a list of accounts from a hold.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [holds remove held accounts matters](MatterHoldRemoveHeldAccountCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoveHeldAccountsRequest {
    /// The account IDs of the accounts to remove from the hold.
    #[serde(rename="accountIds")]
    
    pub account_ids: Option<Vec<String>>,
}

impl client::RequestValue for RemoveHeldAccountsRequest {}


/// Response for batch delete held accounts.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [holds remove held accounts matters](MatterHoldRemoveHeldAccountCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoveHeldAccountsResponse {
    /// A list of statuses for the deleted accounts. Results have the same order as the request.
    
    pub statuses: Option<Vec<Status>>,
}

impl client::ResponseResult for RemoveHeldAccountsResponse {}


/// Remove an account as a matter collaborator.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [remove permissions matters](MatterRemovePermissionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoveMatterPermissionsRequest {
    /// The account ID.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
}

impl client::RequestValue for RemoveMatterPermissionsRequest {}


/// Reopen a matter by ID.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reopen matters](MatterReopenCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReopenMatterRequest { _never_set: Option<bool> }

impl client::RequestValue for ReopenMatterRequest {}


/// Response to a ReopenMatterRequest.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reopen matters](MatterReopenCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReopenMatterResponse {
    /// The updated matter, with state **OPEN**.
    
    pub matter: Option<Matter>,
}

impl client::ResponseResult for ReopenMatterResponse {}


/// The definition of a saved query. To work with Vault resources, the account must have the [required Vault privileges](https://support.google.com/vault/answer/2799699) and access to the matter. To access a matter, the account must have created the matter, have the matter shared with them, or have the **View All Matters** privilege.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [saved queries create matters](MatterSavedQueryCreateCall) (request|response)
/// * [saved queries get matters](MatterSavedQueryGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SavedQuery {
    /// Output only. The server-generated timestamp when the saved query was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The name of the saved query.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The matter ID of the matter the saved query is saved in. The server does not use this field during create and always uses matter ID in the URL.
    #[serde(rename="matterId")]
    
    pub matter_id: Option<String>,
    /// The search parameters of the saved query.
    
    pub query: Option<Query>,
    /// A unique identifier for the saved query.
    #[serde(rename="savedQueryId")]
    
    pub saved_query_id: Option<String>,
}

impl client::RequestValue for SavedQuery {}
impl client::ResponseResult for SavedQuery {}


/// The shared drives to search
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SharedDriveInfo {
    /// A list of shared drive IDs, as provided by the [Drive API](https://developers.google.com/drive).
    #[serde(rename="sharedDriveIds")]
    
    pub shared_drive_ids: Option<Vec<String>>,
}

impl client::Part for SharedDriveInfo {}


/// The published site URLs of new Google Sites to search
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SitesUrlInfo {
    /// A list of published site URLs.
    
    pub urls: Option<Vec<String>>,
}

impl client::Part for SitesUrlInfo {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for Status {}


/// Team Drives to search
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TeamDriveInfo {
    /// List of Team Drive IDs, as provided by the [Drive API](https://developers.google.com/drive).
    #[serde(rename="teamDriveIds")]
    
    pub team_drive_ids: Option<Vec<String>>,
}

impl client::Part for TeamDriveInfo {}


/// Undelete a matter by ID.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [undelete matters](MatterUndeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UndeleteMatterRequest { _never_set: Option<bool> }

impl client::RequestValue for UndeleteMatterRequest {}


/// User's information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserInfo {
    /// The displayed name of the user.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The email address of the user.
    
    pub email: Option<String>,
}

impl client::Part for UserInfo {}


/// The options for Voice exports.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VoiceExportOptions {
    /// The file format for exported text messages.
    #[serde(rename="exportFormat")]
    
    pub export_format: Option<VoiceExportOptionExportFormatEnum>,
}

impl client::Part for VoiceExportOptions {}


/// Additional options for Voice search
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VoiceOptions {
    /// Datatypes to search
    #[serde(rename="coveredData")]
    
    pub covered_data: Option<Vec<VoiceOptionCoveredDataEnum>>,
}

impl client::Part for VoiceOptions {}


