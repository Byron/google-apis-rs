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

use crate::client;

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// View and manage your Google Maps Coordinate jobs
    Full,

    /// View your Google Coordinate jobs
    Readonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Full => "https://www.googleapis.com/auth/coordinate",
            Scope::Readonly => "https://www.googleapis.com/auth/coordinate.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Readonly
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Coordinate related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_coordinate1 as coordinate1;
/// use coordinate1::api::Job;
/// use coordinate1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use coordinate1::Coordinate;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Job::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().patch(req, "teamId", "jobId")
///              .title("est")
///              .progress("ipsum")
///              .note("ipsum")
///              .lng(0.8016125465746553)
///              .lat(0.8000651323113592)
///              .customer_phone_number("ea")
///              .customer_name("dolor")
///              .add_custom_field("Lorem")
///              .assignee("eos")
///              .address("labore")
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
pub struct Coordinate<C> {
    client: RefCell<C>,
    auth: RefCell<oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C> client::Hub for Coordinate<C> {}

impl<'a, C> Coordinate<C>
    where  C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {

    pub fn new(client: C, authenticator: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>) -> Coordinate<C> {
        Coordinate {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/2.0.0".to_string(),
            _base_url: "https://www.googleapis.com/coordinate/v1/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn custom_field_def(&'a self) -> CustomFieldDefMethods<'a, C> {
        CustomFieldDefMethods { hub: &self }
    }
    pub fn jobs(&'a self) -> JobMethods<'a, C> {
        JobMethods { hub: &self }
    }
    pub fn location(&'a self) -> LocationMethods<'a, C> {
        LocationMethods { hub: &self }
    }
    pub fn schedule(&'a self) -> ScheduleMethods<'a, C> {
        ScheduleMethods { hub: &self }
    }
    pub fn team(&'a self) -> TeamMethods<'a, C> {
        TeamMethods { hub: &self }
    }
    pub fn worker(&'a self) -> WorkerMethods<'a, C> {
        WorkerMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/2.0.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/coordinate/v1/`.
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
/// Custom field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomField {
    /// Custom field id.
    #[serde(rename="customFieldId")]
    pub custom_field_id: Option<String>,
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomFieldDef {
    /// Whether the field is enabled.
    pub enabled: Option<bool>,
    /// List of enum items for this custom field. Populated only if the field type is enum. Enum fields appear as 'lists' in the Coordinate web and mobile UI.
    pub enumitems: Option<Vec<EnumItemDef>>,
    /// Custom field id.
    pub id: Option<String>,
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
/// 
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
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    /// Job id.
    pub id: Option<String>,
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobChange {
    /// Identifies this object as a job change.
    pub kind: Option<String>,
    /// Change applied to the job. Only the fields that were changed are set.
    pub state: Option<JobState>,
    /// Time at which this change was applied.
    pub timestamp: Option<String>,
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
/// 
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
/// 
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocationRecord {
    /// The collection time in milliseconds since the epoch.
    #[serde(rename="collectionTime")]
    pub collection_time: Option<String>,
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
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Schedule {
    /// Whether the job is scheduled for the whole day. Time of day in start/end times is ignored if this is true.
    #[serde(rename="allDay")]
    pub all_day: Option<bool>,
    /// Job duration in milliseconds.
    pub duration: Option<String>,
    /// Scheduled end time in milliseconds since epoch.
    #[serde(rename="endTime")]
    pub end_time: Option<String>,
    /// Identifies this object as a job schedule.
    pub kind: Option<String>,
    /// Scheduled start time in milliseconds since epoch.
    #[serde(rename="startTime")]
    pub start_time: Option<String>,
}

impl client::RequestValue for Schedule {}
impl client::ResponseResult for Schedule {}


/// A Coordinate team.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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
/// 
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
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WorkerListResponse {
    /// Workers in the collection.
    pub items: Option<Vec<Worker>>,
    /// Identifies this object as a list of workers.
    pub kind: Option<String>,
}

impl client::ResponseResult for WorkerListResponse {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *customFieldDef* resources.
/// It is not used directly, but through the `Coordinate` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_coordinate1 as coordinate1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use coordinate1::Coordinate;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.custom_field_def();
/// # }
/// ```
pub struct CustomFieldDefMethods<'a, C>
    where C: 'a {

    hub: &'a Coordinate<C>,
}

impl<'a, C> client::MethodsBuilder for CustomFieldDefMethods<'a, C> {}

impl<'a, C> CustomFieldDefMethods<'a, C> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of custom field definitions for a team.
    /// 
    /// # Arguments
    ///
    /// * `teamId` - Team ID
    pub fn list(&self, team_id: &str) -> CustomFieldDefListCall<'a, C> {
        CustomFieldDefListCall {
            hub: self.hub,
            _team_id: team_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *job* resources.
/// It is not used directly, but through the `Coordinate` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_coordinate1 as coordinate1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use coordinate1::Coordinate;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.jobs();
/// # }
/// ```
pub struct JobMethods<'a, C>
    where C: 'a {

    hub: &'a Coordinate<C>,
}

impl<'a, C> client::MethodsBuilder for JobMethods<'a, C> {}

impl<'a, C> JobMethods<'a, C> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a job, including all the changes made to the job.
    /// 
    /// # Arguments
    ///
    /// * `teamId` - Team ID
    /// * `jobId` - Job number
    pub fn get(&self, team_id: &str, job_id: &str) -> JobGetCall<'a, C> {
        JobGetCall {
            hub: self.hub,
            _team_id: team_id.to_string(),
            _job_id: job_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new job. Only the state field of the job should be set.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `teamId` - Team ID
    /// * `address` - Job address as newline (Unix) separated string
    /// * `lat` - The latitude coordinate of this job's location.
    /// * `lng` - The longitude coordinate of this job's location.
    /// * `title` - Job title
    pub fn insert(&self, request: Job, team_id: &str, address: &str, lat: f64, lng: f64, title: &str) -> JobInsertCall<'a, C> {
        JobInsertCall {
            hub: self.hub,
            _request: request,
            _team_id: team_id.to_string(),
            _address: address.to_string(),
            _lat: lat,
            _lng: lng,
            _title: title.to_string(),
            _note: Default::default(),
            _customer_phone_number: Default::default(),
            _customer_name: Default::default(),
            _custom_field: Default::default(),
            _assignee: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves jobs created or modified since the given timestamp.
    /// 
    /// # Arguments
    ///
    /// * `teamId` - Team ID
    pub fn list(&self, team_id: &str) -> JobListCall<'a, C> {
        JobListCall {
            hub: self.hub,
            _team_id: team_id.to_string(),
            _page_token: Default::default(),
            _omit_job_changes: Default::default(),
            _min_modified_timestamp_ms: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a job. Fields that are set in the job state will be updated. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `teamId` - Team ID
    /// * `jobId` - Job number
    pub fn patch(&self, request: Job, team_id: &str, job_id: &str) -> JobPatchCall<'a, C> {
        JobPatchCall {
            hub: self.hub,
            _request: request,
            _team_id: team_id.to_string(),
            _job_id: job_id.to_string(),
            _title: Default::default(),
            _progress: Default::default(),
            _note: Default::default(),
            _lng: Default::default(),
            _lat: Default::default(),
            _customer_phone_number: Default::default(),
            _customer_name: Default::default(),
            _custom_field: Default::default(),
            _assignee: Default::default(),
            _address: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a job. Fields that are set in the job state will be updated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `teamId` - Team ID
    /// * `jobId` - Job number
    pub fn update(&self, request: Job, team_id: &str, job_id: &str) -> JobUpdateCall<'a, C> {
        JobUpdateCall {
            hub: self.hub,
            _request: request,
            _team_id: team_id.to_string(),
            _job_id: job_id.to_string(),
            _title: Default::default(),
            _progress: Default::default(),
            _note: Default::default(),
            _lng: Default::default(),
            _lat: Default::default(),
            _customer_phone_number: Default::default(),
            _customer_name: Default::default(),
            _custom_field: Default::default(),
            _assignee: Default::default(),
            _address: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *location* resources.
/// It is not used directly, but through the `Coordinate` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_coordinate1 as coordinate1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use coordinate1::Coordinate;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.location();
/// # }
/// ```
pub struct LocationMethods<'a, C>
    where C: 'a {

    hub: &'a Coordinate<C>,
}

impl<'a, C> client::MethodsBuilder for LocationMethods<'a, C> {}

impl<'a, C> LocationMethods<'a, C> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of locations for a worker.
    /// 
    /// # Arguments
    ///
    /// * `teamId` - Team ID
    /// * `workerEmail` - Worker email address.
    /// * `startTimestampMs` - Start timestamp in milliseconds since the epoch.
    pub fn list(&self, team_id: &str, worker_email: &str, start_timestamp_ms: &str) -> LocationListCall<'a, C> {
        LocationListCall {
            hub: self.hub,
            _team_id: team_id.to_string(),
            _worker_email: worker_email.to_string(),
            _start_timestamp_ms: start_timestamp_ms.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *schedule* resources.
/// It is not used directly, but through the `Coordinate` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_coordinate1 as coordinate1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use coordinate1::Coordinate;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.schedule();
/// # }
/// ```
pub struct ScheduleMethods<'a, C>
    where C: 'a {

    hub: &'a Coordinate<C>,
}

impl<'a, C> client::MethodsBuilder for ScheduleMethods<'a, C> {}

impl<'a, C> ScheduleMethods<'a, C> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the schedule for a job.
    /// 
    /// # Arguments
    ///
    /// * `teamId` - Team ID
    /// * `jobId` - Job number
    pub fn get(&self, team_id: &str, job_id: &str) -> ScheduleGetCall<'a, C> {
        ScheduleGetCall {
            hub: self.hub,
            _team_id: team_id.to_string(),
            _job_id: job_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Replaces the schedule of a job with the provided schedule. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `teamId` - Team ID
    /// * `jobId` - Job number
    pub fn patch(&self, request: Schedule, team_id: &str, job_id: &str) -> SchedulePatchCall<'a, C> {
        SchedulePatchCall {
            hub: self.hub,
            _request: request,
            _team_id: team_id.to_string(),
            _job_id: job_id.to_string(),
            _start_time: Default::default(),
            _end_time: Default::default(),
            _duration: Default::default(),
            _all_day: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Replaces the schedule of a job with the provided schedule.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `teamId` - Team ID
    /// * `jobId` - Job number
    pub fn update(&self, request: Schedule, team_id: &str, job_id: &str) -> ScheduleUpdateCall<'a, C> {
        ScheduleUpdateCall {
            hub: self.hub,
            _request: request,
            _team_id: team_id.to_string(),
            _job_id: job_id.to_string(),
            _start_time: Default::default(),
            _end_time: Default::default(),
            _duration: Default::default(),
            _all_day: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *team* resources.
/// It is not used directly, but through the `Coordinate` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_coordinate1 as coordinate1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use coordinate1::Coordinate;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.team();
/// # }
/// ```
pub struct TeamMethods<'a, C>
    where C: 'a {

    hub: &'a Coordinate<C>,
}

impl<'a, C> client::MethodsBuilder for TeamMethods<'a, C> {}

impl<'a, C> TeamMethods<'a, C> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of teams for a user.
    pub fn list(&self) -> TeamListCall<'a, C> {
        TeamListCall {
            hub: self.hub,
            _worker: Default::default(),
            _dispatcher: Default::default(),
            _admin: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *worker* resources.
/// It is not used directly, but through the `Coordinate` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_coordinate1 as coordinate1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use coordinate1::Coordinate;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.worker();
/// # }
/// ```
pub struct WorkerMethods<'a, C>
    where C: 'a {

    hub: &'a Coordinate<C>,
}

impl<'a, C> client::MethodsBuilder for WorkerMethods<'a, C> {}

impl<'a, C> WorkerMethods<'a, C> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of workers in a team.
    /// 
    /// # Arguments
    ///
    /// * `teamId` - Team ID
    pub fn list(&self, team_id: &str) -> WorkerListCall<'a, C> {
        WorkerListCall {
            hub: self.hub,
            _team_id: team_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Retrieves a list of custom field definitions for a team.
///
/// A builder for the *list* method supported by a *customFieldDef* resource.
/// It is not used directly, but through a `CustomFieldDefMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_coordinate1 as coordinate1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use coordinate1::Coordinate;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.custom_field_def().list("teamId")
///              .doit().await;
/// # }
/// ```
pub struct CustomFieldDefListCall<'a, C>
    where C: 'a {

    hub: &'a Coordinate<C>,
    _team_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for CustomFieldDefListCall<'a, C> {}

impl<'a, C> CustomFieldDefListCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, CustomFieldDefListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "coordinate.customFieldDef.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("teamId", self._team_id.to_string()));
        for &field in ["alt", "teamId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "teams/{teamId}/custom_fields";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{teamId}", "teamId")].iter() {
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
            for param_name in ["teamId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    let (res_parts, res_body) = res.into_parts();
                    let res_body_string: String = String::from_utf8(
                        hyper::body::to_bytes(res_body)
                            .await
                            .unwrap()
                            .into_iter()
                            .collect(),
                    )
                    .unwrap();
                    let reconstructed_result =
                        hyper::Response::from_parts(res_parts, res_body_string.clone().into());

                    if !reconstructed_result.status().is_success() {
                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&reconstructed_result,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(reconstructed_result)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (reconstructed_result, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Team ID
    ///
    /// Sets the *team id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn team_id(mut self, new_value: &str) -> CustomFieldDefListCall<'a, C> {
        self._team_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CustomFieldDefListCall<'a, C> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> CustomFieldDefListCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Readonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> CustomFieldDefListCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves a job, including all the changes made to the job.
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
/// # extern crate google_coordinate1 as coordinate1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use coordinate1::Coordinate;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().get("teamId", "jobId")
///              .doit().await;
/// # }
/// ```
pub struct JobGetCall<'a, C>
    where C: 'a {

    hub: &'a Coordinate<C>,
    _team_id: String,
    _job_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for JobGetCall<'a, C> {}

impl<'a, C> JobGetCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Job)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "coordinate.jobs.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("teamId", self._team_id.to_string()));
        params.push(("jobId", self._job_id.to_string()));
        for &field in ["alt", "teamId", "jobId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "teams/{teamId}/jobs/{jobId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{teamId}", "teamId"), ("{jobId}", "jobId")].iter() {
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
            for param_name in ["jobId", "teamId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    let (res_parts, res_body) = res.into_parts();
                    let res_body_string: String = String::from_utf8(
                        hyper::body::to_bytes(res_body)
                            .await
                            .unwrap()
                            .into_iter()
                            .collect(),
                    )
                    .unwrap();
                    let reconstructed_result =
                        hyper::Response::from_parts(res_parts, res_body_string.clone().into());

                    if !reconstructed_result.status().is_success() {
                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&reconstructed_result,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(reconstructed_result)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (reconstructed_result, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Team ID
    ///
    /// Sets the *team id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn team_id(mut self, new_value: &str) -> JobGetCall<'a, C> {
        self._team_id = new_value.to_string();
        self
    }
    /// Job number
    ///
    /// Sets the *job id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn job_id(mut self, new_value: &str) -> JobGetCall<'a, C> {
        self._job_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> JobGetCall<'a, C> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> JobGetCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Readonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> JobGetCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Inserts a new job. Only the state field of the job should be set.
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
/// # extern crate google_coordinate1 as coordinate1;
/// use coordinate1::api::Job;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use coordinate1::Coordinate;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Job::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().insert(req, "teamId", "address", 0.6905413711203235, 0.9150999978526841, "title")
///              .note("et")
///              .customer_phone_number("et")
///              .customer_name("vero")
///              .add_custom_field("erat")
///              .assignee("sed")
///              .doit().await;
/// # }
/// ```
pub struct JobInsertCall<'a, C>
    where C: 'a {

    hub: &'a Coordinate<C>,
    _request: Job,
    _team_id: String,
    _address: String,
    _lat: f64,
    _lng: f64,
    _title: String,
    _note: Option<String>,
    _customer_phone_number: Option<String>,
    _customer_name: Option<String>,
    _custom_field: Vec<String>,
    _assignee: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for JobInsertCall<'a, C> {}

impl<'a, C> JobInsertCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Job)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "coordinate.jobs.insert",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(13 + self._additional_params.len());
        params.push(("teamId", self._team_id.to_string()));
        params.push(("address", self._address.to_string()));
        params.push(("lat", self._lat.to_string()));
        params.push(("lng", self._lng.to_string()));
        params.push(("title", self._title.to_string()));
        if let Some(value) = self._note {
            params.push(("note", value.to_string()));
        }
        if let Some(value) = self._customer_phone_number {
            params.push(("customerPhoneNumber", value.to_string()));
        }
        if let Some(value) = self._customer_name {
            params.push(("customerName", value.to_string()));
        }
        if self._custom_field.len() > 0 {
            for f in self._custom_field.iter() {
                params.push(("customField", f.to_string()));
            }
        }
        if let Some(value) = self._assignee {
            params.push(("assignee", value.to_string()));
        }
        for &field in ["alt", "teamId", "address", "lat", "lng", "title", "note", "customerPhoneNumber", "customerName", "customField", "assignee"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "teams/{teamId}/jobs";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{teamId}", "teamId")].iter() {
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
            for param_name in ["teamId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    let (res_parts, res_body) = res.into_parts();
                    let res_body_string: String = String::from_utf8(
                        hyper::body::to_bytes(res_body)
                            .await
                            .unwrap()
                            .into_iter()
                            .collect(),
                    )
                    .unwrap();
                    let reconstructed_result =
                        hyper::Response::from_parts(res_parts, res_body_string.clone().into());

                    if !reconstructed_result.status().is_success() {
                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&reconstructed_result,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(reconstructed_result)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (reconstructed_result, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
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
    pub fn request(mut self, new_value: Job) -> JobInsertCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Team ID
    ///
    /// Sets the *team id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn team_id(mut self, new_value: &str) -> JobInsertCall<'a, C> {
        self._team_id = new_value.to_string();
        self
    }
    /// Job address as newline (Unix) separated string
    ///
    /// Sets the *address* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn address(mut self, new_value: &str) -> JobInsertCall<'a, C> {
        self._address = new_value.to_string();
        self
    }
    /// The latitude coordinate of this job's location.
    ///
    /// Sets the *lat* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn lat(mut self, new_value: f64) -> JobInsertCall<'a, C> {
        self._lat = new_value;
        self
    }
    /// The longitude coordinate of this job's location.
    ///
    /// Sets the *lng* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn lng(mut self, new_value: f64) -> JobInsertCall<'a, C> {
        self._lng = new_value;
        self
    }
    /// Job title
    ///
    /// Sets the *title* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn title(mut self, new_value: &str) -> JobInsertCall<'a, C> {
        self._title = new_value.to_string();
        self
    }
    /// Job note as newline (Unix) separated string
    ///
    /// Sets the *note* query property to the given value.
    pub fn note(mut self, new_value: &str) -> JobInsertCall<'a, C> {
        self._note = Some(new_value.to_string());
        self
    }
    /// Customer phone number
    ///
    /// Sets the *customer phone number* query property to the given value.
    pub fn customer_phone_number(mut self, new_value: &str) -> JobInsertCall<'a, C> {
        self._customer_phone_number = Some(new_value.to_string());
        self
    }
    /// Customer name
    ///
    /// Sets the *customer name* query property to the given value.
    pub fn customer_name(mut self, new_value: &str) -> JobInsertCall<'a, C> {
        self._customer_name = Some(new_value.to_string());
        self
    }
    /// Sets the value of custom fields. To set a custom field, pass the field id (from /team/teamId/custom_fields), a URL escaped '=' character, and the desired value as a parameter. For example, customField=12%3DAlice. Repeat the parameter for each custom field. Note that '=' cannot appear in the parameter value. Specifying an invalid, or inactive enum field will result in an error 500.
    ///
    /// Append the given value to the *custom field* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_custom_field(mut self, new_value: &str) -> JobInsertCall<'a, C> {
        self._custom_field.push(new_value.to_string());
        self
    }
    /// Assignee email address, or empty string to unassign.
    ///
    /// Sets the *assignee* query property to the given value.
    pub fn assignee(mut self, new_value: &str) -> JobInsertCall<'a, C> {
        self._assignee = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> JobInsertCall<'a, C> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> JobInsertCall<'a, C>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> JobInsertCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves jobs created or modified since the given timestamp.
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
/// # extern crate google_coordinate1 as coordinate1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use coordinate1::Coordinate;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().list("teamId")
///              .page_token("dolore")
///              .omit_job_changes(false)
///              .min_modified_timestamp_ms("diam")
///              .max_results(52)
///              .doit().await;
/// # }
/// ```
pub struct JobListCall<'a, C>
    where C: 'a {

    hub: &'a Coordinate<C>,
    _team_id: String,
    _page_token: Option<String>,
    _omit_job_changes: Option<bool>,
    _min_modified_timestamp_ms: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for JobListCall<'a, C> {}

impl<'a, C> JobListCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, JobListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "coordinate.jobs.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("teamId", self._team_id.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._omit_job_changes {
            params.push(("omitJobChanges", value.to_string()));
        }
        if let Some(value) = self._min_modified_timestamp_ms {
            params.push(("minModifiedTimestampMs", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        for &field in ["alt", "teamId", "pageToken", "omitJobChanges", "minModifiedTimestampMs", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "teams/{teamId}/jobs";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{teamId}", "teamId")].iter() {
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
            for param_name in ["teamId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    let (res_parts, res_body) = res.into_parts();
                    let res_body_string: String = String::from_utf8(
                        hyper::body::to_bytes(res_body)
                            .await
                            .unwrap()
                            .into_iter()
                            .collect(),
                    )
                    .unwrap();
                    let reconstructed_result =
                        hyper::Response::from_parts(res_parts, res_body_string.clone().into());

                    if !reconstructed_result.status().is_success() {
                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&reconstructed_result,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(reconstructed_result)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (reconstructed_result, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Team ID
    ///
    /// Sets the *team id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn team_id(mut self, new_value: &str) -> JobListCall<'a, C> {
        self._team_id = new_value.to_string();
        self
    }
    /// Continuation token
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> JobListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Whether to omit detail job history information.
    ///
    /// Sets the *omit job changes* query property to the given value.
    pub fn omit_job_changes(mut self, new_value: bool) -> JobListCall<'a, C> {
        self._omit_job_changes = Some(new_value);
        self
    }
    /// Minimum time a job was modified in milliseconds since epoch.
    ///
    /// Sets the *min modified timestamp ms* query property to the given value.
    pub fn min_modified_timestamp_ms(mut self, new_value: &str) -> JobListCall<'a, C> {
        self._min_modified_timestamp_ms = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return in one page.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> JobListCall<'a, C> {
        self._max_results = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> JobListCall<'a, C> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> JobListCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Readonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> JobListCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates a job. Fields that are set in the job state will be updated. This method supports patch semantics.
///
/// A builder for the *patch* method supported by a *job* resource.
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
/// # extern crate google_coordinate1 as coordinate1;
/// use coordinate1::api::Job;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use coordinate1::Coordinate;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Job::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().patch(req, "teamId", "jobId")
///              .title("sadipscing")
///              .progress("Stet")
///              .note("dolor")
///              .lng(0.6349978834153693)
///              .lat(0.1970220513983837)
///              .customer_phone_number("Stet")
///              .customer_name("vero")
///              .add_custom_field("elitr")
///              .assignee("Lorem")
///              .address("diam")
///              .doit().await;
/// # }
/// ```
pub struct JobPatchCall<'a, C>
    where C: 'a {

    hub: &'a Coordinate<C>,
    _request: Job,
    _team_id: String,
    _job_id: String,
    _title: Option<String>,
    _progress: Option<String>,
    _note: Option<String>,
    _lng: Option<f64>,
    _lat: Option<f64>,
    _customer_phone_number: Option<String>,
    _customer_name: Option<String>,
    _custom_field: Vec<String>,
    _assignee: Option<String>,
    _address: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for JobPatchCall<'a, C> {}

impl<'a, C> JobPatchCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Job)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "coordinate.jobs.patch",
                               http_method: hyper::Method::PATCH });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(15 + self._additional_params.len());
        params.push(("teamId", self._team_id.to_string()));
        params.push(("jobId", self._job_id.to_string()));
        if let Some(value) = self._title {
            params.push(("title", value.to_string()));
        }
        if let Some(value) = self._progress {
            params.push(("progress", value.to_string()));
        }
        if let Some(value) = self._note {
            params.push(("note", value.to_string()));
        }
        if let Some(value) = self._lng {
            params.push(("lng", value.to_string()));
        }
        if let Some(value) = self._lat {
            params.push(("lat", value.to_string()));
        }
        if let Some(value) = self._customer_phone_number {
            params.push(("customerPhoneNumber", value.to_string()));
        }
        if let Some(value) = self._customer_name {
            params.push(("customerName", value.to_string()));
        }
        if self._custom_field.len() > 0 {
            for f in self._custom_field.iter() {
                params.push(("customField", f.to_string()));
            }
        }
        if let Some(value) = self._assignee {
            params.push(("assignee", value.to_string()));
        }
        if let Some(value) = self._address {
            params.push(("address", value.to_string()));
        }
        for &field in ["alt", "teamId", "jobId", "title", "progress", "note", "lng", "lat", "customerPhoneNumber", "customerName", "customField", "assignee", "address"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "teams/{teamId}/jobs/{jobId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{teamId}", "teamId"), ("{jobId}", "jobId")].iter() {
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
            for param_name in ["jobId", "teamId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PATCH).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    let (res_parts, res_body) = res.into_parts();
                    let res_body_string: String = String::from_utf8(
                        hyper::body::to_bytes(res_body)
                            .await
                            .unwrap()
                            .into_iter()
                            .collect(),
                    )
                    .unwrap();
                    let reconstructed_result =
                        hyper::Response::from_parts(res_parts, res_body_string.clone().into());

                    if !reconstructed_result.status().is_success() {
                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&reconstructed_result,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(reconstructed_result)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (reconstructed_result, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
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
    pub fn request(mut self, new_value: Job) -> JobPatchCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Team ID
    ///
    /// Sets the *team id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn team_id(mut self, new_value: &str) -> JobPatchCall<'a, C> {
        self._team_id = new_value.to_string();
        self
    }
    /// Job number
    ///
    /// Sets the *job id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn job_id(mut self, new_value: &str) -> JobPatchCall<'a, C> {
        self._job_id = new_value.to_string();
        self
    }
    /// Job title
    ///
    /// Sets the *title* query property to the given value.
    pub fn title(mut self, new_value: &str) -> JobPatchCall<'a, C> {
        self._title = Some(new_value.to_string());
        self
    }
    /// Job progress
    ///
    /// Sets the *progress* query property to the given value.
    pub fn progress(mut self, new_value: &str) -> JobPatchCall<'a, C> {
        self._progress = Some(new_value.to_string());
        self
    }
    /// Job note as newline (Unix) separated string
    ///
    /// Sets the *note* query property to the given value.
    pub fn note(mut self, new_value: &str) -> JobPatchCall<'a, C> {
        self._note = Some(new_value.to_string());
        self
    }
    /// The longitude coordinate of this job's location.
    ///
    /// Sets the *lng* query property to the given value.
    pub fn lng(mut self, new_value: f64) -> JobPatchCall<'a, C> {
        self._lng = Some(new_value);
        self
    }
    /// The latitude coordinate of this job's location.
    ///
    /// Sets the *lat* query property to the given value.
    pub fn lat(mut self, new_value: f64) -> JobPatchCall<'a, C> {
        self._lat = Some(new_value);
        self
    }
    /// Customer phone number
    ///
    /// Sets the *customer phone number* query property to the given value.
    pub fn customer_phone_number(mut self, new_value: &str) -> JobPatchCall<'a, C> {
        self._customer_phone_number = Some(new_value.to_string());
        self
    }
    /// Customer name
    ///
    /// Sets the *customer name* query property to the given value.
    pub fn customer_name(mut self, new_value: &str) -> JobPatchCall<'a, C> {
        self._customer_name = Some(new_value.to_string());
        self
    }
    /// Sets the value of custom fields. To set a custom field, pass the field id (from /team/teamId/custom_fields), a URL escaped '=' character, and the desired value as a parameter. For example, customField=12%3DAlice. Repeat the parameter for each custom field. Note that '=' cannot appear in the parameter value. Specifying an invalid, or inactive enum field will result in an error 500.
    ///
    /// Append the given value to the *custom field* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_custom_field(mut self, new_value: &str) -> JobPatchCall<'a, C> {
        self._custom_field.push(new_value.to_string());
        self
    }
    /// Assignee email address, or empty string to unassign.
    ///
    /// Sets the *assignee* query property to the given value.
    pub fn assignee(mut self, new_value: &str) -> JobPatchCall<'a, C> {
        self._assignee = Some(new_value.to_string());
        self
    }
    /// Job address as newline (Unix) separated string
    ///
    /// Sets the *address* query property to the given value.
    pub fn address(mut self, new_value: &str) -> JobPatchCall<'a, C> {
        self._address = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> JobPatchCall<'a, C> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> JobPatchCall<'a, C>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> JobPatchCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates a job. Fields that are set in the job state will be updated.
///
/// A builder for the *update* method supported by a *job* resource.
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
/// # extern crate google_coordinate1 as coordinate1;
/// use coordinate1::api::Job;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use coordinate1::Coordinate;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Job::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().update(req, "teamId", "jobId")
///              .title("accusam")
///              .progress("takimata")
///              .note("consetetur")
///              .lng(0.5721089079262882)
///              .lat(0.22880530766851637)
///              .customer_phone_number("consetetur")
///              .customer_name("amet.")
///              .add_custom_field("sed")
///              .assignee("takimata")
///              .address("dolores")
///              .doit().await;
/// # }
/// ```
pub struct JobUpdateCall<'a, C>
    where C: 'a {

    hub: &'a Coordinate<C>,
    _request: Job,
    _team_id: String,
    _job_id: String,
    _title: Option<String>,
    _progress: Option<String>,
    _note: Option<String>,
    _lng: Option<f64>,
    _lat: Option<f64>,
    _customer_phone_number: Option<String>,
    _customer_name: Option<String>,
    _custom_field: Vec<String>,
    _assignee: Option<String>,
    _address: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for JobUpdateCall<'a, C> {}

impl<'a, C> JobUpdateCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Job)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "coordinate.jobs.update",
                               http_method: hyper::Method::PUT });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(15 + self._additional_params.len());
        params.push(("teamId", self._team_id.to_string()));
        params.push(("jobId", self._job_id.to_string()));
        if let Some(value) = self._title {
            params.push(("title", value.to_string()));
        }
        if let Some(value) = self._progress {
            params.push(("progress", value.to_string()));
        }
        if let Some(value) = self._note {
            params.push(("note", value.to_string()));
        }
        if let Some(value) = self._lng {
            params.push(("lng", value.to_string()));
        }
        if let Some(value) = self._lat {
            params.push(("lat", value.to_string()));
        }
        if let Some(value) = self._customer_phone_number {
            params.push(("customerPhoneNumber", value.to_string()));
        }
        if let Some(value) = self._customer_name {
            params.push(("customerName", value.to_string()));
        }
        if self._custom_field.len() > 0 {
            for f in self._custom_field.iter() {
                params.push(("customField", f.to_string()));
            }
        }
        if let Some(value) = self._assignee {
            params.push(("assignee", value.to_string()));
        }
        if let Some(value) = self._address {
            params.push(("address", value.to_string()));
        }
        for &field in ["alt", "teamId", "jobId", "title", "progress", "note", "lng", "lat", "customerPhoneNumber", "customerName", "customField", "assignee", "address"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "teams/{teamId}/jobs/{jobId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{teamId}", "teamId"), ("{jobId}", "jobId")].iter() {
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
            for param_name in ["jobId", "teamId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PUT).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    let (res_parts, res_body) = res.into_parts();
                    let res_body_string: String = String::from_utf8(
                        hyper::body::to_bytes(res_body)
                            .await
                            .unwrap()
                            .into_iter()
                            .collect(),
                    )
                    .unwrap();
                    let reconstructed_result =
                        hyper::Response::from_parts(res_parts, res_body_string.clone().into());

                    if !reconstructed_result.status().is_success() {
                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&reconstructed_result,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(reconstructed_result)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (reconstructed_result, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
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
    pub fn request(mut self, new_value: Job) -> JobUpdateCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Team ID
    ///
    /// Sets the *team id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn team_id(mut self, new_value: &str) -> JobUpdateCall<'a, C> {
        self._team_id = new_value.to_string();
        self
    }
    /// Job number
    ///
    /// Sets the *job id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn job_id(mut self, new_value: &str) -> JobUpdateCall<'a, C> {
        self._job_id = new_value.to_string();
        self
    }
    /// Job title
    ///
    /// Sets the *title* query property to the given value.
    pub fn title(mut self, new_value: &str) -> JobUpdateCall<'a, C> {
        self._title = Some(new_value.to_string());
        self
    }
    /// Job progress
    ///
    /// Sets the *progress* query property to the given value.
    pub fn progress(mut self, new_value: &str) -> JobUpdateCall<'a, C> {
        self._progress = Some(new_value.to_string());
        self
    }
    /// Job note as newline (Unix) separated string
    ///
    /// Sets the *note* query property to the given value.
    pub fn note(mut self, new_value: &str) -> JobUpdateCall<'a, C> {
        self._note = Some(new_value.to_string());
        self
    }
    /// The longitude coordinate of this job's location.
    ///
    /// Sets the *lng* query property to the given value.
    pub fn lng(mut self, new_value: f64) -> JobUpdateCall<'a, C> {
        self._lng = Some(new_value);
        self
    }
    /// The latitude coordinate of this job's location.
    ///
    /// Sets the *lat* query property to the given value.
    pub fn lat(mut self, new_value: f64) -> JobUpdateCall<'a, C> {
        self._lat = Some(new_value);
        self
    }
    /// Customer phone number
    ///
    /// Sets the *customer phone number* query property to the given value.
    pub fn customer_phone_number(mut self, new_value: &str) -> JobUpdateCall<'a, C> {
        self._customer_phone_number = Some(new_value.to_string());
        self
    }
    /// Customer name
    ///
    /// Sets the *customer name* query property to the given value.
    pub fn customer_name(mut self, new_value: &str) -> JobUpdateCall<'a, C> {
        self._customer_name = Some(new_value.to_string());
        self
    }
    /// Sets the value of custom fields. To set a custom field, pass the field id (from /team/teamId/custom_fields), a URL escaped '=' character, and the desired value as a parameter. For example, customField=12%3DAlice. Repeat the parameter for each custom field. Note that '=' cannot appear in the parameter value. Specifying an invalid, or inactive enum field will result in an error 500.
    ///
    /// Append the given value to the *custom field* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_custom_field(mut self, new_value: &str) -> JobUpdateCall<'a, C> {
        self._custom_field.push(new_value.to_string());
        self
    }
    /// Assignee email address, or empty string to unassign.
    ///
    /// Sets the *assignee* query property to the given value.
    pub fn assignee(mut self, new_value: &str) -> JobUpdateCall<'a, C> {
        self._assignee = Some(new_value.to_string());
        self
    }
    /// Job address as newline (Unix) separated string
    ///
    /// Sets the *address* query property to the given value.
    pub fn address(mut self, new_value: &str) -> JobUpdateCall<'a, C> {
        self._address = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> JobUpdateCall<'a, C> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> JobUpdateCall<'a, C>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> JobUpdateCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves a list of locations for a worker.
///
/// A builder for the *list* method supported by a *location* resource.
/// It is not used directly, but through a `LocationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_coordinate1 as coordinate1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use coordinate1::Coordinate;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.location().list("teamId", "workerEmail", "startTimestampMs")
///              .page_token("voluptua.")
///              .max_results(67)
///              .doit().await;
/// # }
/// ```
pub struct LocationListCall<'a, C>
    where C: 'a {

    hub: &'a Coordinate<C>,
    _team_id: String,
    _worker_email: String,
    _start_timestamp_ms: String,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for LocationListCall<'a, C> {}

impl<'a, C> LocationListCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, LocationListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "coordinate.location.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("teamId", self._team_id.to_string()));
        params.push(("workerEmail", self._worker_email.to_string()));
        params.push(("startTimestampMs", self._start_timestamp_ms.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        for &field in ["alt", "teamId", "workerEmail", "startTimestampMs", "pageToken", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "teams/{teamId}/workers/{workerEmail}/locations";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{teamId}", "teamId"), ("{workerEmail}", "workerEmail")].iter() {
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
            for param_name in ["workerEmail", "teamId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    let (res_parts, res_body) = res.into_parts();
                    let res_body_string: String = String::from_utf8(
                        hyper::body::to_bytes(res_body)
                            .await
                            .unwrap()
                            .into_iter()
                            .collect(),
                    )
                    .unwrap();
                    let reconstructed_result =
                        hyper::Response::from_parts(res_parts, res_body_string.clone().into());

                    if !reconstructed_result.status().is_success() {
                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&reconstructed_result,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(reconstructed_result)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (reconstructed_result, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Team ID
    ///
    /// Sets the *team id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn team_id(mut self, new_value: &str) -> LocationListCall<'a, C> {
        self._team_id = new_value.to_string();
        self
    }
    /// Worker email address.
    ///
    /// Sets the *worker email* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn worker_email(mut self, new_value: &str) -> LocationListCall<'a, C> {
        self._worker_email = new_value.to_string();
        self
    }
    /// Start timestamp in milliseconds since the epoch.
    ///
    /// Sets the *start timestamp ms* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn start_timestamp_ms(mut self, new_value: &str) -> LocationListCall<'a, C> {
        self._start_timestamp_ms = new_value.to_string();
        self
    }
    /// Continuation token
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> LocationListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return in one page.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> LocationListCall<'a, C> {
        self._max_results = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> LocationListCall<'a, C> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> LocationListCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Readonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> LocationListCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves the schedule for a job.
///
/// A builder for the *get* method supported by a *schedule* resource.
/// It is not used directly, but through a `ScheduleMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_coordinate1 as coordinate1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use coordinate1::Coordinate;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.schedule().get("teamId", "jobId")
///              .doit().await;
/// # }
/// ```
pub struct ScheduleGetCall<'a, C>
    where C: 'a {

    hub: &'a Coordinate<C>,
    _team_id: String,
    _job_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for ScheduleGetCall<'a, C> {}

impl<'a, C> ScheduleGetCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Schedule)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "coordinate.schedule.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("teamId", self._team_id.to_string()));
        params.push(("jobId", self._job_id.to_string()));
        for &field in ["alt", "teamId", "jobId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "teams/{teamId}/jobs/{jobId}/schedule";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{teamId}", "teamId"), ("{jobId}", "jobId")].iter() {
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
            for param_name in ["jobId", "teamId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    let (res_parts, res_body) = res.into_parts();
                    let res_body_string: String = String::from_utf8(
                        hyper::body::to_bytes(res_body)
                            .await
                            .unwrap()
                            .into_iter()
                            .collect(),
                    )
                    .unwrap();
                    let reconstructed_result =
                        hyper::Response::from_parts(res_parts, res_body_string.clone().into());

                    if !reconstructed_result.status().is_success() {
                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&reconstructed_result,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(reconstructed_result)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (reconstructed_result, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Team ID
    ///
    /// Sets the *team id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn team_id(mut self, new_value: &str) -> ScheduleGetCall<'a, C> {
        self._team_id = new_value.to_string();
        self
    }
    /// Job number
    ///
    /// Sets the *job id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn job_id(mut self, new_value: &str) -> ScheduleGetCall<'a, C> {
        self._job_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ScheduleGetCall<'a, C> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> ScheduleGetCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Readonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ScheduleGetCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Replaces the schedule of a job with the provided schedule. This method supports patch semantics.
///
/// A builder for the *patch* method supported by a *schedule* resource.
/// It is not used directly, but through a `ScheduleMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_coordinate1 as coordinate1;
/// use coordinate1::api::Schedule;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use coordinate1::Coordinate;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Schedule::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.schedule().patch(req, "teamId", "jobId")
///              .start_time("ea")
///              .end_time("sadipscing")
///              .duration("Lorem")
///              .all_day(true)
///              .doit().await;
/// # }
/// ```
pub struct SchedulePatchCall<'a, C>
    where C: 'a {

    hub: &'a Coordinate<C>,
    _request: Schedule,
    _team_id: String,
    _job_id: String,
    _start_time: Option<String>,
    _end_time: Option<String>,
    _duration: Option<String>,
    _all_day: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for SchedulePatchCall<'a, C> {}

impl<'a, C> SchedulePatchCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Schedule)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "coordinate.schedule.patch",
                               http_method: hyper::Method::PATCH });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
        params.push(("teamId", self._team_id.to_string()));
        params.push(("jobId", self._job_id.to_string()));
        if let Some(value) = self._start_time {
            params.push(("startTime", value.to_string()));
        }
        if let Some(value) = self._end_time {
            params.push(("endTime", value.to_string()));
        }
        if let Some(value) = self._duration {
            params.push(("duration", value.to_string()));
        }
        if let Some(value) = self._all_day {
            params.push(("allDay", value.to_string()));
        }
        for &field in ["alt", "teamId", "jobId", "startTime", "endTime", "duration", "allDay"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "teams/{teamId}/jobs/{jobId}/schedule";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{teamId}", "teamId"), ("{jobId}", "jobId")].iter() {
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
            for param_name in ["jobId", "teamId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PATCH).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    let (res_parts, res_body) = res.into_parts();
                    let res_body_string: String = String::from_utf8(
                        hyper::body::to_bytes(res_body)
                            .await
                            .unwrap()
                            .into_iter()
                            .collect(),
                    )
                    .unwrap();
                    let reconstructed_result =
                        hyper::Response::from_parts(res_parts, res_body_string.clone().into());

                    if !reconstructed_result.status().is_success() {
                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&reconstructed_result,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(reconstructed_result)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (reconstructed_result, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
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
    pub fn request(mut self, new_value: Schedule) -> SchedulePatchCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Team ID
    ///
    /// Sets the *team id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn team_id(mut self, new_value: &str) -> SchedulePatchCall<'a, C> {
        self._team_id = new_value.to_string();
        self
    }
    /// Job number
    ///
    /// Sets the *job id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn job_id(mut self, new_value: &str) -> SchedulePatchCall<'a, C> {
        self._job_id = new_value.to_string();
        self
    }
    /// Scheduled start time in milliseconds since epoch.
    ///
    /// Sets the *start time* query property to the given value.
    pub fn start_time(mut self, new_value: &str) -> SchedulePatchCall<'a, C> {
        self._start_time = Some(new_value.to_string());
        self
    }
    /// Scheduled end time in milliseconds since epoch.
    ///
    /// Sets the *end time* query property to the given value.
    pub fn end_time(mut self, new_value: &str) -> SchedulePatchCall<'a, C> {
        self._end_time = Some(new_value.to_string());
        self
    }
    /// Job duration in milliseconds.
    ///
    /// Sets the *duration* query property to the given value.
    pub fn duration(mut self, new_value: &str) -> SchedulePatchCall<'a, C> {
        self._duration = Some(new_value.to_string());
        self
    }
    /// Whether the job is scheduled for the whole day. Time of day in start/end times is ignored if this is true.
    ///
    /// Sets the *all day* query property to the given value.
    pub fn all_day(mut self, new_value: bool) -> SchedulePatchCall<'a, C> {
        self._all_day = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> SchedulePatchCall<'a, C> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> SchedulePatchCall<'a, C>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> SchedulePatchCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Replaces the schedule of a job with the provided schedule.
///
/// A builder for the *update* method supported by a *schedule* resource.
/// It is not used directly, but through a `ScheduleMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_coordinate1 as coordinate1;
/// use coordinate1::api::Schedule;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use coordinate1::Coordinate;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Schedule::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.schedule().update(req, "teamId", "jobId")
///              .start_time("At")
///              .end_time("sed")
///              .duration("sit")
///              .all_day(true)
///              .doit().await;
/// # }
/// ```
pub struct ScheduleUpdateCall<'a, C>
    where C: 'a {

    hub: &'a Coordinate<C>,
    _request: Schedule,
    _team_id: String,
    _job_id: String,
    _start_time: Option<String>,
    _end_time: Option<String>,
    _duration: Option<String>,
    _all_day: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for ScheduleUpdateCall<'a, C> {}

impl<'a, C> ScheduleUpdateCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Schedule)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "coordinate.schedule.update",
                               http_method: hyper::Method::PUT });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
        params.push(("teamId", self._team_id.to_string()));
        params.push(("jobId", self._job_id.to_string()));
        if let Some(value) = self._start_time {
            params.push(("startTime", value.to_string()));
        }
        if let Some(value) = self._end_time {
            params.push(("endTime", value.to_string()));
        }
        if let Some(value) = self._duration {
            params.push(("duration", value.to_string()));
        }
        if let Some(value) = self._all_day {
            params.push(("allDay", value.to_string()));
        }
        for &field in ["alt", "teamId", "jobId", "startTime", "endTime", "duration", "allDay"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "teams/{teamId}/jobs/{jobId}/schedule";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{teamId}", "teamId"), ("{jobId}", "jobId")].iter() {
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
            for param_name in ["jobId", "teamId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PUT).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    let (res_parts, res_body) = res.into_parts();
                    let res_body_string: String = String::from_utf8(
                        hyper::body::to_bytes(res_body)
                            .await
                            .unwrap()
                            .into_iter()
                            .collect(),
                    )
                    .unwrap();
                    let reconstructed_result =
                        hyper::Response::from_parts(res_parts, res_body_string.clone().into());

                    if !reconstructed_result.status().is_success() {
                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&reconstructed_result,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(reconstructed_result)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (reconstructed_result, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
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
    pub fn request(mut self, new_value: Schedule) -> ScheduleUpdateCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Team ID
    ///
    /// Sets the *team id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn team_id(mut self, new_value: &str) -> ScheduleUpdateCall<'a, C> {
        self._team_id = new_value.to_string();
        self
    }
    /// Job number
    ///
    /// Sets the *job id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn job_id(mut self, new_value: &str) -> ScheduleUpdateCall<'a, C> {
        self._job_id = new_value.to_string();
        self
    }
    /// Scheduled start time in milliseconds since epoch.
    ///
    /// Sets the *start time* query property to the given value.
    pub fn start_time(mut self, new_value: &str) -> ScheduleUpdateCall<'a, C> {
        self._start_time = Some(new_value.to_string());
        self
    }
    /// Scheduled end time in milliseconds since epoch.
    ///
    /// Sets the *end time* query property to the given value.
    pub fn end_time(mut self, new_value: &str) -> ScheduleUpdateCall<'a, C> {
        self._end_time = Some(new_value.to_string());
        self
    }
    /// Job duration in milliseconds.
    ///
    /// Sets the *duration* query property to the given value.
    pub fn duration(mut self, new_value: &str) -> ScheduleUpdateCall<'a, C> {
        self._duration = Some(new_value.to_string());
        self
    }
    /// Whether the job is scheduled for the whole day. Time of day in start/end times is ignored if this is true.
    ///
    /// Sets the *all day* query property to the given value.
    pub fn all_day(mut self, new_value: bool) -> ScheduleUpdateCall<'a, C> {
        self._all_day = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ScheduleUpdateCall<'a, C> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> ScheduleUpdateCall<'a, C>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ScheduleUpdateCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves a list of teams for a user.
///
/// A builder for the *list* method supported by a *team* resource.
/// It is not used directly, but through a `TeamMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_coordinate1 as coordinate1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use coordinate1::Coordinate;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.team().list()
///              .worker(true)
///              .dispatcher(true)
///              .admin(true)
///              .doit().await;
/// # }
/// ```
pub struct TeamListCall<'a, C>
    where C: 'a {

    hub: &'a Coordinate<C>,
    _worker: Option<bool>,
    _dispatcher: Option<bool>,
    _admin: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for TeamListCall<'a, C> {}

impl<'a, C> TeamListCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, TeamListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "coordinate.team.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        if let Some(value) = self._worker {
            params.push(("worker", value.to_string()));
        }
        if let Some(value) = self._dispatcher {
            params.push(("dispatcher", value.to_string()));
        }
        if let Some(value) = self._admin {
            params.push(("admin", value.to_string()));
        }
        for &field in ["alt", "worker", "dispatcher", "admin"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "teams";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_ref().to_string(), ());
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    let (res_parts, res_body) = res.into_parts();
                    let res_body_string: String = String::from_utf8(
                        hyper::body::to_bytes(res_body)
                            .await
                            .unwrap()
                            .into_iter()
                            .collect(),
                    )
                    .unwrap();
                    let reconstructed_result =
                        hyper::Response::from_parts(res_parts, res_body_string.clone().into());

                    if !reconstructed_result.status().is_success() {
                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&reconstructed_result,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(reconstructed_result)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (reconstructed_result, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Whether to include teams for which the user has the Worker role.
    ///
    /// Sets the *worker* query property to the given value.
    pub fn worker(mut self, new_value: bool) -> TeamListCall<'a, C> {
        self._worker = Some(new_value);
        self
    }
    /// Whether to include teams for which the user has the Dispatcher role.
    ///
    /// Sets the *dispatcher* query property to the given value.
    pub fn dispatcher(mut self, new_value: bool) -> TeamListCall<'a, C> {
        self._dispatcher = Some(new_value);
        self
    }
    /// Whether to include teams for which the user has the Admin role.
    ///
    /// Sets the *admin* query property to the given value.
    pub fn admin(mut self, new_value: bool) -> TeamListCall<'a, C> {
        self._admin = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> TeamListCall<'a, C> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> TeamListCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Readonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> TeamListCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves a list of workers in a team.
///
/// A builder for the *list* method supported by a *worker* resource.
/// It is not used directly, but through a `WorkerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_coordinate1 as coordinate1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use coordinate1::Coordinate;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.worker().list("teamId")
///              .doit().await;
/// # }
/// ```
pub struct WorkerListCall<'a, C>
    where C: 'a {

    hub: &'a Coordinate<C>,
    _team_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for WorkerListCall<'a, C> {}

impl<'a, C> WorkerListCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, WorkerListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "coordinate.worker.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("teamId", self._team_id.to_string()));
        for &field in ["alt", "teamId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "teams/{teamId}/workers";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{teamId}", "teamId")].iter() {
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
            for param_name in ["teamId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    let (res_parts, res_body) = res.into_parts();
                    let res_body_string: String = String::from_utf8(
                        hyper::body::to_bytes(res_body)
                            .await
                            .unwrap()
                            .into_iter()
                            .collect(),
                    )
                    .unwrap();
                    let reconstructed_result =
                        hyper::Response::from_parts(res_parts, res_body_string.clone().into());

                    if !reconstructed_result.status().is_success() {
                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&reconstructed_result,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(reconstructed_result)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (reconstructed_result, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Team ID
    ///
    /// Sets the *team id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn team_id(mut self, new_value: &str) -> WorkerListCall<'a, C> {
        self._team_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> WorkerListCall<'a, C> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> WorkerListCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Readonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> WorkerListCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


