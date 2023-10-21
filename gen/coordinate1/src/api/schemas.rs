use super::*;
/// Custom field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomField {
    /// Custom field id.
    #[serde(rename="customFieldId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub custom_field_id: Option<i64>,
    /// Identifies this object as a custom field.
    
    pub kind: Option<String>,
    /// Custom field value.
    
    pub value: Option<String>,
}

impl client::Part for CustomField {}


/// Custom field definition.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomFieldDef {
    /// Whether the field is enabled.
    
    pub enabled: Option<bool>,
    /// List of enum items for this custom field. Populated only if the field type is enum. Enum fields appear as 'lists' in the Coordinate web and mobile UI.
    
    pub enumitems: Option<Vec<EnumItemDef>>,
    /// Custom field id.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Identifies this object as a custom field definition.
    
    pub kind: Option<String>,
    /// Custom field name.
    
    pub name: Option<String>,
    /// Whether the field is required for checkout.
    #[serde(rename="requiredForCheckout")]
    
    pub required_for_checkout: Option<bool>,
    /// Custom field type.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for CustomFieldDef {}


/// Collection of custom field definitions for a team.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list custom field def](CustomFieldDefListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomFieldDefListResponse {
    /// Collection of custom field definitions in a team.
    
    pub items: Option<Vec<CustomFieldDef>>,
    /// Identifies this object as a collection of custom field definitions in a team.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for CustomFieldDefListResponse {}


/// Collection of custom fields.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomFields {
    /// Collection of custom fields.
    #[serde(rename="customField")]
    
    pub custom_field: Option<Vec<CustomField>>,
    /// Identifies this object as a collection of custom fields.
    
    pub kind: Option<String>,
}

impl client::Part for CustomFields {}


/// Enum Item definition.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnumItemDef {
    /// Whether the enum item is active. Jobs may contain inactive enum values; however, setting an enum to an inactive value when creating or updating a job will result in a 500 error.
    
    pub active: Option<bool>,
    /// Identifies this object as an enum item definition.
    
    pub kind: Option<String>,
    /// Custom field value.
    
    pub value: Option<String>,
}

impl client::Part for EnumItemDef {}


/// A job.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get jobs](JobGetCall) (response)
/// * [insert jobs](JobInsertCall) (request|response)
/// * [list jobs](JobListCall) (none)
/// * [patch jobs](JobPatchCall) (request|response)
/// * [update jobs](JobUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    /// Job id.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<u64>,
    /// List of job changes since it was created. The first change corresponds to the state of the job when it was created.
    #[serde(rename="jobChange")]
    
    pub job_change: Option<Vec<JobChange>>,
    /// Identifies this object as a job.
    
    pub kind: Option<String>,
    /// Current job state.
    
    pub state: Option<JobState>,
}

impl client::RequestValue for Job {}
impl client::Resource for Job {}
impl client::ResponseResult for Job {}


/// Change to a job. For example assigning the job to a different worker.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobChange {
    /// Identifies this object as a job change.
    
    pub kind: Option<String>,
    /// Change applied to the job. Only the fields that were changed are set.
    
    pub state: Option<JobState>,
    /// Time at which this change was applied.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub timestamp: Option<u64>,
}

impl client::Part for JobChange {}


/// Response from a List Jobs request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list jobs](JobListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobListResponse {
    /// Jobs in the collection.
    
    pub items: Option<Vec<Job>>,
    /// Identifies this object as a list of jobs.
    
    pub kind: Option<String>,
    /// A token to provide to get the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for JobListResponse {}


/// Current state of a job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobState {
    /// Email address of the assignee, or the string "DELETED_USER" if the account is no longer available.
    
    pub assignee: Option<String>,
    /// Custom fields.
    #[serde(rename="customFields")]
    
    pub custom_fields: Option<CustomFields>,
    /// Customer name.
    #[serde(rename="customerName")]
    
    pub customer_name: Option<String>,
    /// Customer phone number.
    #[serde(rename="customerPhoneNumber")]
    
    pub customer_phone_number: Option<String>,
    /// Identifies this object as a job state.
    
    pub kind: Option<String>,
    /// Job location.
    
    pub location: Option<Location>,
    /// Note added to the job.
    
    pub note: Option<Vec<String>>,
    /// Job progress.
    
    pub progress: Option<String>,
    /// Job title.
    
    pub title: Option<String>,
}

impl client::Part for JobState {}


/// Location of a job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// Address.
    #[serde(rename="addressLine")]
    
    pub address_line: Option<Vec<String>>,
    /// Identifies this object as a location.
    
    pub kind: Option<String>,
    /// Latitude.
    
    pub lat: Option<f64>,
    /// Longitude.
    
    pub lng: Option<f64>,
}

impl client::Part for Location {}


/// Response from a List Locations request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list location](LocationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocationListResponse {
    /// Locations in the collection.
    
    pub items: Option<Vec<LocationRecord>>,
    /// Identifies this object as a list of locations.
    
    pub kind: Option<String>,
    /// A token to provide to get the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Pagination information for token pagination.
    #[serde(rename="tokenPagination")]
    
    pub token_pagination: Option<TokenPagination>,
}

impl client::ResponseResult for LocationListResponse {}


/// Recorded location of a worker.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocationRecord {
    /// The collection time in milliseconds since the epoch.
    #[serde(rename="collectionTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub collection_time: Option<i64>,
    /// The location accuracy in meters. This is the radius of a 95% confidence interval around the location measurement.
    #[serde(rename="confidenceRadius")]
    
    pub confidence_radius: Option<f64>,
    /// Identifies this object as a location.
    
    pub kind: Option<String>,
    /// Latitude.
    
    pub latitude: Option<f64>,
    /// Longitude.
    
    pub longitude: Option<f64>,
}

impl client::Part for LocationRecord {}


/// Job schedule.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get schedule](ScheduleGetCall) (response)
/// * [patch schedule](SchedulePatchCall) (request|response)
/// * [update schedule](ScheduleUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Schedule {
    /// Whether the job is scheduled for the whole day. Time of day in start/end times is ignored if this is true.
    #[serde(rename="allDay")]
    
    pub all_day: Option<bool>,
    /// Job duration in milliseconds.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub duration: Option<u64>,
    /// Scheduled end time in milliseconds since epoch.
    #[serde(rename="endTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub end_time: Option<u64>,
    /// Identifies this object as a job schedule.
    
    pub kind: Option<String>,
    /// Scheduled start time in milliseconds since epoch.
    #[serde(rename="startTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start_time: Option<u64>,
}

impl client::RequestValue for Schedule {}
impl client::ResponseResult for Schedule {}


/// A Coordinate team.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Team {
    /// Team id, as found in a coordinate team url e.g. https://coordinate.google.com/f/xyz where "xyz" is the team id.
    
    pub id: Option<String>,
    /// Identifies this object as a team.
    
    pub kind: Option<String>,
    /// Team name
    
    pub name: Option<String>,
}

impl client::Part for Team {}


/// Response from a List Teams request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list team](TeamListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TeamListResponse {
    /// Teams in the collection.
    
    pub items: Option<Vec<Team>>,
    /// Identifies this object as a list of teams.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for TeamListResponse {}


/// Pagination information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TokenPagination {
    /// Identifies this object as pagination information.
    
    pub kind: Option<String>,
    /// A token to provide to get the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A token to provide to get the previous page of results.
    #[serde(rename="previousPageToken")]
    
    pub previous_page_token: Option<String>,
}

impl client::Part for TokenPagination {}


/// A worker in a Coordinate team.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Worker {
    /// Worker email address. If a worker has been deleted from your team, the email address will appear as DELETED_USER.
    
    pub id: Option<String>,
    /// Identifies this object as a worker.
    
    pub kind: Option<String>,
}

impl client::Part for Worker {}


/// Response from a List Workers request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list worker](WorkerListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WorkerListResponse {
    /// Workers in the collection.
    
    pub items: Option<Vec<Worker>>,
    /// Identifies this object as a list of workers.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for WorkerListResponse {}


