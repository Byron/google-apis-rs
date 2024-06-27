use std::collections::HashMap;
use std::cell::RefCell;
use std::default::Default;
use std::collections::BTreeSet;
use std::error::Error as StdError;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;

use hyper::client::connect;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::time::sleep;
use tower_service;
use serde::{Serialize, Deserialize};

use crate::{client, client::GetToken, client::serde_with};

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Clone, Copy)]
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
/// extern crate google_coordinate1 as coordinate1;
/// use coordinate1::api::Job;
/// use coordinate1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use coordinate1::{Coordinate, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Job::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().patch(req, "teamId", 85)
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
#[derive(Clone)]
pub struct Coordinate<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, S> client::Hub for Coordinate<S> {}

impl<'a, S> Coordinate<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> Coordinate<S> {
        Coordinate {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.5".to_string(),
            _base_url: "https://www.googleapis.com/coordinate/v1/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn custom_field_def(&'a self) -> CustomFieldDefMethods<'a, S> {
        CustomFieldDefMethods { hub: &self }
    }
    pub fn jobs(&'a self) -> JobMethods<'a, S> {
        JobMethods { hub: &self }
    }
    pub fn location(&'a self) -> LocationMethods<'a, S> {
        LocationMethods { hub: &self }
    }
    pub fn schedule(&'a self) -> ScheduleMethods<'a, S> {
        ScheduleMethods { hub: &self }
    }
    pub fn team(&'a self) -> TeamMethods<'a, S> {
        TeamMethods { hub: &self }
    }
    pub fn worker(&'a self) -> WorkerMethods<'a, S> {
        WorkerMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.5`.
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
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
/// It is not used directly, but through the [`Coordinate`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_coordinate1 as coordinate1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use coordinate1::{Coordinate, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.custom_field_def();
/// # }
/// ```
pub struct CustomFieldDefMethods<'a, S>
    where S: 'a {

    hub: &'a Coordinate<S>,
}

impl<'a, S> client::MethodsBuilder for CustomFieldDefMethods<'a, S> {}

impl<'a, S> CustomFieldDefMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of custom field definitions for a team.
    /// 
    /// # Arguments
    ///
    /// * `teamId` - Team ID
    pub fn list(&self, team_id: &str) -> CustomFieldDefListCall<'a, S> {
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
/// It is not used directly, but through the [`Coordinate`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_coordinate1 as coordinate1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use coordinate1::{Coordinate, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.jobs();
/// # }
/// ```
pub struct JobMethods<'a, S>
    where S: 'a {

    hub: &'a Coordinate<S>,
}

impl<'a, S> client::MethodsBuilder for JobMethods<'a, S> {}

impl<'a, S> JobMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a job, including all the changes made to the job.
    /// 
    /// # Arguments
    ///
    /// * `teamId` - Team ID
    /// * `jobId` - Job number
    pub fn get(&self, team_id: &str, job_id: u64) -> JobGetCall<'a, S> {
        JobGetCall {
            hub: self.hub,
            _team_id: team_id.to_string(),
            _job_id: job_id,
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
    pub fn insert(&self, request: Job, team_id: &str, address: &str, lat: f64, lng: f64, title: &str) -> JobInsertCall<'a, S> {
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
    pub fn list(&self, team_id: &str) -> JobListCall<'a, S> {
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
    pub fn patch(&self, request: Job, team_id: &str, job_id: u64) -> JobPatchCall<'a, S> {
        JobPatchCall {
            hub: self.hub,
            _request: request,
            _team_id: team_id.to_string(),
            _job_id: job_id,
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
    pub fn update(&self, request: Job, team_id: &str, job_id: u64) -> JobUpdateCall<'a, S> {
        JobUpdateCall {
            hub: self.hub,
            _request: request,
            _team_id: team_id.to_string(),
            _job_id: job_id,
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
/// It is not used directly, but through the [`Coordinate`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_coordinate1 as coordinate1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use coordinate1::{Coordinate, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.location();
/// # }
/// ```
pub struct LocationMethods<'a, S>
    where S: 'a {

    hub: &'a Coordinate<S>,
}

impl<'a, S> client::MethodsBuilder for LocationMethods<'a, S> {}

impl<'a, S> LocationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of locations for a worker.
    /// 
    /// # Arguments
    ///
    /// * `teamId` - Team ID
    /// * `workerEmail` - Worker email address.
    /// * `startTimestampMs` - Start timestamp in milliseconds since the epoch.
    pub fn list(&self, team_id: &str, worker_email: &str, start_timestamp_ms: u64) -> LocationListCall<'a, S> {
        LocationListCall {
            hub: self.hub,
            _team_id: team_id.to_string(),
            _worker_email: worker_email.to_string(),
            _start_timestamp_ms: start_timestamp_ms,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *schedule* resources.
/// It is not used directly, but through the [`Coordinate`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_coordinate1 as coordinate1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use coordinate1::{Coordinate, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.schedule();
/// # }
/// ```
pub struct ScheduleMethods<'a, S>
    where S: 'a {

    hub: &'a Coordinate<S>,
}

impl<'a, S> client::MethodsBuilder for ScheduleMethods<'a, S> {}

impl<'a, S> ScheduleMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the schedule for a job.
    /// 
    /// # Arguments
    ///
    /// * `teamId` - Team ID
    /// * `jobId` - Job number
    pub fn get(&self, team_id: &str, job_id: u64) -> ScheduleGetCall<'a, S> {
        ScheduleGetCall {
            hub: self.hub,
            _team_id: team_id.to_string(),
            _job_id: job_id,
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
    pub fn patch(&self, request: Schedule, team_id: &str, job_id: u64) -> SchedulePatchCall<'a, S> {
        SchedulePatchCall {
            hub: self.hub,
            _request: request,
            _team_id: team_id.to_string(),
            _job_id: job_id,
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
    pub fn update(&self, request: Schedule, team_id: &str, job_id: u64) -> ScheduleUpdateCall<'a, S> {
        ScheduleUpdateCall {
            hub: self.hub,
            _request: request,
            _team_id: team_id.to_string(),
            _job_id: job_id,
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
/// It is not used directly, but through the [`Coordinate`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_coordinate1 as coordinate1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use coordinate1::{Coordinate, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.team();
/// # }
/// ```
pub struct TeamMethods<'a, S>
    where S: 'a {

    hub: &'a Coordinate<S>,
}

impl<'a, S> client::MethodsBuilder for TeamMethods<'a, S> {}

impl<'a, S> TeamMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of teams for a user.
    pub fn list(&self) -> TeamListCall<'a, S> {
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
/// It is not used directly, but through the [`Coordinate`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_coordinate1 as coordinate1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use coordinate1::{Coordinate, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.worker();
/// # }
/// ```
pub struct WorkerMethods<'a, S>
    where S: 'a {

    hub: &'a Coordinate<S>,
}

impl<'a, S> client::MethodsBuilder for WorkerMethods<'a, S> {}

impl<'a, S> WorkerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of workers in a team.
    /// 
    /// # Arguments
    ///
    /// * `teamId` - Team ID
    pub fn list(&self, team_id: &str) -> WorkerListCall<'a, S> {
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
/// It is not used directly, but through a [`CustomFieldDefMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_coordinate1 as coordinate1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use coordinate1::{Coordinate, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.custom_field_def().list("teamId")
///              .doit().await;
/// # }
/// ```
pub struct CustomFieldDefListCall<'a, S>
    where S: 'a {

    hub: &'a Coordinate<S>,
    _team_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for CustomFieldDefListCall<'a, S> {}

impl<'a, S> CustomFieldDefListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, CustomFieldDefListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "coordinate.customFieldDef.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "teamId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("teamId", self._team_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "teams/{teamId}/custom_fields";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Readonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{teamId}", "teamId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["teamId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
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
    pub fn team_id(mut self, new_value: &str) -> CustomFieldDefListCall<'a, S> {
        self._team_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CustomFieldDefListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> CustomFieldDefListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Readonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> CustomFieldDefListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> CustomFieldDefListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> CustomFieldDefListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Retrieves a job, including all the changes made to the job.
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
/// # extern crate google_coordinate1 as coordinate1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use coordinate1::{Coordinate, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().get("teamId", 21)
///              .doit().await;
/// # }
/// ```
pub struct JobGetCall<'a, S>
    where S: 'a {

    hub: &'a Coordinate<S>,
    _team_id: String,
    _job_id: u64,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for JobGetCall<'a, S> {}

impl<'a, S> JobGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Job)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "coordinate.jobs.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "teamId", "jobId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("teamId", self._team_id);
        params.push("jobId", self._job_id.to_string());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "teams/{teamId}/jobs/{jobId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Readonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{teamId}", "teamId"), ("{jobId}", "jobId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["jobId", "teamId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
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
    pub fn team_id(mut self, new_value: &str) -> JobGetCall<'a, S> {
        self._team_id = new_value.to_string();
        self
    }
    /// Job number
    ///
    /// Sets the *job id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn job_id(mut self, new_value: u64) -> JobGetCall<'a, S> {
        self._job_id = new_value;
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> JobGetCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> JobGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Readonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> JobGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> JobGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> JobGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Inserts a new job. Only the state field of the job should be set.
///
/// A builder for the *insert* method supported by a *job* resource.
/// It is not used directly, but through a [`JobMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_coordinate1 as coordinate1;
/// use coordinate1::api::Job;
/// # async fn dox() {
/// # use std::default::Default;
/// # use coordinate1::{Coordinate, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
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
pub struct JobInsertCall<'a, S>
    where S: 'a {

    hub: &'a Coordinate<S>,
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
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for JobInsertCall<'a, S> {}

impl<'a, S> JobInsertCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Job)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "coordinate.jobs.insert",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "teamId", "address", "lat", "lng", "title", "note", "customerPhoneNumber", "customerName", "customField", "assignee"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(13 + self._additional_params.len());
        params.push("teamId", self._team_id);
        params.push("address", self._address);
        params.push("lat", self._lat.to_string());
        params.push("lng", self._lng.to_string());
        params.push("title", self._title);
        if let Some(value) = self._note.as_ref() {
            params.push("note", value);
        }
        if let Some(value) = self._customer_phone_number.as_ref() {
            params.push("customerPhoneNumber", value);
        }
        if let Some(value) = self._customer_name.as_ref() {
            params.push("customerName", value);
        }
        if self._custom_field.len() > 0 {
            for f in self._custom_field.iter() {
                params.push("customField", f);
            }
        }
        if let Some(value) = self._assignee.as_ref() {
            params.push("assignee", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "teams/{teamId}/jobs";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{teamId}", "teamId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["teamId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
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
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
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
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
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
    pub fn request(mut self, new_value: Job) -> JobInsertCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Team ID
    ///
    /// Sets the *team id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn team_id(mut self, new_value: &str) -> JobInsertCall<'a, S> {
        self._team_id = new_value.to_string();
        self
    }
    /// Job address as newline (Unix) separated string
    ///
    /// Sets the *address* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn address(mut self, new_value: &str) -> JobInsertCall<'a, S> {
        self._address = new_value.to_string();
        self
    }
    /// The latitude coordinate of this job's location.
    ///
    /// Sets the *lat* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn lat(mut self, new_value: f64) -> JobInsertCall<'a, S> {
        self._lat = new_value;
        self
    }
    /// The longitude coordinate of this job's location.
    ///
    /// Sets the *lng* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn lng(mut self, new_value: f64) -> JobInsertCall<'a, S> {
        self._lng = new_value;
        self
    }
    /// Job title
    ///
    /// Sets the *title* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn title(mut self, new_value: &str) -> JobInsertCall<'a, S> {
        self._title = new_value.to_string();
        self
    }
    /// Job note as newline (Unix) separated string
    ///
    /// Sets the *note* query property to the given value.
    pub fn note(mut self, new_value: &str) -> JobInsertCall<'a, S> {
        self._note = Some(new_value.to_string());
        self
    }
    /// Customer phone number
    ///
    /// Sets the *customer phone number* query property to the given value.
    pub fn customer_phone_number(mut self, new_value: &str) -> JobInsertCall<'a, S> {
        self._customer_phone_number = Some(new_value.to_string());
        self
    }
    /// Customer name
    ///
    /// Sets the *customer name* query property to the given value.
    pub fn customer_name(mut self, new_value: &str) -> JobInsertCall<'a, S> {
        self._customer_name = Some(new_value.to_string());
        self
    }
    /// Sets the value of custom fields. To set a custom field, pass the field id (from /team/teamId/custom_fields), a URL escaped '=' character, and the desired value as a parameter. For example, customField=12%3DAlice. Repeat the parameter for each custom field. Note that '=' cannot appear in the parameter value. Specifying an invalid, or inactive enum field will result in an error 500.
    ///
    /// Append the given value to the *custom field* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_custom_field(mut self, new_value: &str) -> JobInsertCall<'a, S> {
        self._custom_field.push(new_value.to_string());
        self
    }
    /// Assignee email address, or empty string to unassign.
    ///
    /// Sets the *assignee* query property to the given value.
    pub fn assignee(mut self, new_value: &str) -> JobInsertCall<'a, S> {
        self._assignee = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> JobInsertCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> JobInsertCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> JobInsertCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> JobInsertCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> JobInsertCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Retrieves jobs created or modified since the given timestamp.
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
/// # extern crate google_coordinate1 as coordinate1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use coordinate1::{Coordinate, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().list("teamId")
///              .page_token("dolore")
///              .omit_job_changes(false)
///              .min_modified_timestamp_ms(9)
///              .max_results(52)
///              .doit().await;
/// # }
/// ```
pub struct JobListCall<'a, S>
    where S: 'a {

    hub: &'a Coordinate<S>,
    _team_id: String,
    _page_token: Option<String>,
    _omit_job_changes: Option<bool>,
    _min_modified_timestamp_ms: Option<u64>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for JobListCall<'a, S> {}

impl<'a, S> JobListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, JobListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "coordinate.jobs.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "teamId", "pageToken", "omitJobChanges", "minModifiedTimestampMs", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("teamId", self._team_id);
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._omit_job_changes.as_ref() {
            params.push("omitJobChanges", value.to_string());
        }
        if let Some(value) = self._min_modified_timestamp_ms.as_ref() {
            params.push("minModifiedTimestampMs", value.to_string());
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "teams/{teamId}/jobs";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Readonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{teamId}", "teamId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["teamId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
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
    pub fn team_id(mut self, new_value: &str) -> JobListCall<'a, S> {
        self._team_id = new_value.to_string();
        self
    }
    /// Continuation token
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> JobListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Whether to omit detail job history information.
    ///
    /// Sets the *omit job changes* query property to the given value.
    pub fn omit_job_changes(mut self, new_value: bool) -> JobListCall<'a, S> {
        self._omit_job_changes = Some(new_value);
        self
    }
    /// Minimum time a job was modified in milliseconds since epoch.
    ///
    /// Sets the *min modified timestamp ms* query property to the given value.
    pub fn min_modified_timestamp_ms(mut self, new_value: u64) -> JobListCall<'a, S> {
        self._min_modified_timestamp_ms = Some(new_value);
        self
    }
    /// Maximum number of results to return in one page.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> JobListCall<'a, S> {
        self._max_results = Some(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> JobListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> JobListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Readonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> JobListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> JobListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> JobListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Updates a job. Fields that are set in the job state will be updated. This method supports patch semantics.
///
/// A builder for the *patch* method supported by a *job* resource.
/// It is not used directly, but through a [`JobMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_coordinate1 as coordinate1;
/// use coordinate1::api::Job;
/// # async fn dox() {
/// # use std::default::Default;
/// # use coordinate1::{Coordinate, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Job::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().patch(req, "teamId", 79)
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
pub struct JobPatchCall<'a, S>
    where S: 'a {

    hub: &'a Coordinate<S>,
    _request: Job,
    _team_id: String,
    _job_id: u64,
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
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for JobPatchCall<'a, S> {}

impl<'a, S> JobPatchCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Job)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "coordinate.jobs.patch",
                               http_method: hyper::Method::PATCH });

        for &field in ["alt", "teamId", "jobId", "title", "progress", "note", "lng", "lat", "customerPhoneNumber", "customerName", "customField", "assignee", "address"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(15 + self._additional_params.len());
        params.push("teamId", self._team_id);
        params.push("jobId", self._job_id.to_string());
        if let Some(value) = self._title.as_ref() {
            params.push("title", value);
        }
        if let Some(value) = self._progress.as_ref() {
            params.push("progress", value);
        }
        if let Some(value) = self._note.as_ref() {
            params.push("note", value);
        }
        if let Some(value) = self._lng.as_ref() {
            params.push("lng", value.to_string());
        }
        if let Some(value) = self._lat.as_ref() {
            params.push("lat", value.to_string());
        }
        if let Some(value) = self._customer_phone_number.as_ref() {
            params.push("customerPhoneNumber", value);
        }
        if let Some(value) = self._customer_name.as_ref() {
            params.push("customerName", value);
        }
        if self._custom_field.len() > 0 {
            for f in self._custom_field.iter() {
                params.push("customField", f);
            }
        }
        if let Some(value) = self._assignee.as_ref() {
            params.push("assignee", value);
        }
        if let Some(value) = self._address.as_ref() {
            params.push("address", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "teams/{teamId}/jobs/{jobId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{teamId}", "teamId"), ("{jobId}", "jobId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["jobId", "teamId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
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
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::PATCH)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
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
    pub fn request(mut self, new_value: Job) -> JobPatchCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Team ID
    ///
    /// Sets the *team id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn team_id(mut self, new_value: &str) -> JobPatchCall<'a, S> {
        self._team_id = new_value.to_string();
        self
    }
    /// Job number
    ///
    /// Sets the *job id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn job_id(mut self, new_value: u64) -> JobPatchCall<'a, S> {
        self._job_id = new_value;
        self
    }
    /// Job title
    ///
    /// Sets the *title* query property to the given value.
    pub fn title(mut self, new_value: &str) -> JobPatchCall<'a, S> {
        self._title = Some(new_value.to_string());
        self
    }
    /// Job progress
    ///
    /// Sets the *progress* query property to the given value.
    pub fn progress(mut self, new_value: &str) -> JobPatchCall<'a, S> {
        self._progress = Some(new_value.to_string());
        self
    }
    /// Job note as newline (Unix) separated string
    ///
    /// Sets the *note* query property to the given value.
    pub fn note(mut self, new_value: &str) -> JobPatchCall<'a, S> {
        self._note = Some(new_value.to_string());
        self
    }
    /// The longitude coordinate of this job's location.
    ///
    /// Sets the *lng* query property to the given value.
    pub fn lng(mut self, new_value: f64) -> JobPatchCall<'a, S> {
        self._lng = Some(new_value);
        self
    }
    /// The latitude coordinate of this job's location.
    ///
    /// Sets the *lat* query property to the given value.
    pub fn lat(mut self, new_value: f64) -> JobPatchCall<'a, S> {
        self._lat = Some(new_value);
        self
    }
    /// Customer phone number
    ///
    /// Sets the *customer phone number* query property to the given value.
    pub fn customer_phone_number(mut self, new_value: &str) -> JobPatchCall<'a, S> {
        self._customer_phone_number = Some(new_value.to_string());
        self
    }
    /// Customer name
    ///
    /// Sets the *customer name* query property to the given value.
    pub fn customer_name(mut self, new_value: &str) -> JobPatchCall<'a, S> {
        self._customer_name = Some(new_value.to_string());
        self
    }
    /// Sets the value of custom fields. To set a custom field, pass the field id (from /team/teamId/custom_fields), a URL escaped '=' character, and the desired value as a parameter. For example, customField=12%3DAlice. Repeat the parameter for each custom field. Note that '=' cannot appear in the parameter value. Specifying an invalid, or inactive enum field will result in an error 500.
    ///
    /// Append the given value to the *custom field* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_custom_field(mut self, new_value: &str) -> JobPatchCall<'a, S> {
        self._custom_field.push(new_value.to_string());
        self
    }
    /// Assignee email address, or empty string to unassign.
    ///
    /// Sets the *assignee* query property to the given value.
    pub fn assignee(mut self, new_value: &str) -> JobPatchCall<'a, S> {
        self._assignee = Some(new_value.to_string());
        self
    }
    /// Job address as newline (Unix) separated string
    ///
    /// Sets the *address* query property to the given value.
    pub fn address(mut self, new_value: &str) -> JobPatchCall<'a, S> {
        self._address = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> JobPatchCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> JobPatchCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> JobPatchCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> JobPatchCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> JobPatchCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Updates a job. Fields that are set in the job state will be updated.
///
/// A builder for the *update* method supported by a *job* resource.
/// It is not used directly, but through a [`JobMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_coordinate1 as coordinate1;
/// use coordinate1::api::Job;
/// # async fn dox() {
/// # use std::default::Default;
/// # use coordinate1::{Coordinate, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Job::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().update(req, "teamId", 1)
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
pub struct JobUpdateCall<'a, S>
    where S: 'a {

    hub: &'a Coordinate<S>,
    _request: Job,
    _team_id: String,
    _job_id: u64,
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
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for JobUpdateCall<'a, S> {}

impl<'a, S> JobUpdateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Job)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "coordinate.jobs.update",
                               http_method: hyper::Method::PUT });

        for &field in ["alt", "teamId", "jobId", "title", "progress", "note", "lng", "lat", "customerPhoneNumber", "customerName", "customField", "assignee", "address"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(15 + self._additional_params.len());
        params.push("teamId", self._team_id);
        params.push("jobId", self._job_id.to_string());
        if let Some(value) = self._title.as_ref() {
            params.push("title", value);
        }
        if let Some(value) = self._progress.as_ref() {
            params.push("progress", value);
        }
        if let Some(value) = self._note.as_ref() {
            params.push("note", value);
        }
        if let Some(value) = self._lng.as_ref() {
            params.push("lng", value.to_string());
        }
        if let Some(value) = self._lat.as_ref() {
            params.push("lat", value.to_string());
        }
        if let Some(value) = self._customer_phone_number.as_ref() {
            params.push("customerPhoneNumber", value);
        }
        if let Some(value) = self._customer_name.as_ref() {
            params.push("customerName", value);
        }
        if self._custom_field.len() > 0 {
            for f in self._custom_field.iter() {
                params.push("customField", f);
            }
        }
        if let Some(value) = self._assignee.as_ref() {
            params.push("assignee", value);
        }
        if let Some(value) = self._address.as_ref() {
            params.push("address", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "teams/{teamId}/jobs/{jobId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{teamId}", "teamId"), ("{jobId}", "jobId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["jobId", "teamId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
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
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::PUT)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
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
    pub fn request(mut self, new_value: Job) -> JobUpdateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Team ID
    ///
    /// Sets the *team id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn team_id(mut self, new_value: &str) -> JobUpdateCall<'a, S> {
        self._team_id = new_value.to_string();
        self
    }
    /// Job number
    ///
    /// Sets the *job id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn job_id(mut self, new_value: u64) -> JobUpdateCall<'a, S> {
        self._job_id = new_value;
        self
    }
    /// Job title
    ///
    /// Sets the *title* query property to the given value.
    pub fn title(mut self, new_value: &str) -> JobUpdateCall<'a, S> {
        self._title = Some(new_value.to_string());
        self
    }
    /// Job progress
    ///
    /// Sets the *progress* query property to the given value.
    pub fn progress(mut self, new_value: &str) -> JobUpdateCall<'a, S> {
        self._progress = Some(new_value.to_string());
        self
    }
    /// Job note as newline (Unix) separated string
    ///
    /// Sets the *note* query property to the given value.
    pub fn note(mut self, new_value: &str) -> JobUpdateCall<'a, S> {
        self._note = Some(new_value.to_string());
        self
    }
    /// The longitude coordinate of this job's location.
    ///
    /// Sets the *lng* query property to the given value.
    pub fn lng(mut self, new_value: f64) -> JobUpdateCall<'a, S> {
        self._lng = Some(new_value);
        self
    }
    /// The latitude coordinate of this job's location.
    ///
    /// Sets the *lat* query property to the given value.
    pub fn lat(mut self, new_value: f64) -> JobUpdateCall<'a, S> {
        self._lat = Some(new_value);
        self
    }
    /// Customer phone number
    ///
    /// Sets the *customer phone number* query property to the given value.
    pub fn customer_phone_number(mut self, new_value: &str) -> JobUpdateCall<'a, S> {
        self._customer_phone_number = Some(new_value.to_string());
        self
    }
    /// Customer name
    ///
    /// Sets the *customer name* query property to the given value.
    pub fn customer_name(mut self, new_value: &str) -> JobUpdateCall<'a, S> {
        self._customer_name = Some(new_value.to_string());
        self
    }
    /// Sets the value of custom fields. To set a custom field, pass the field id (from /team/teamId/custom_fields), a URL escaped '=' character, and the desired value as a parameter. For example, customField=12%3DAlice. Repeat the parameter for each custom field. Note that '=' cannot appear in the parameter value. Specifying an invalid, or inactive enum field will result in an error 500.
    ///
    /// Append the given value to the *custom field* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_custom_field(mut self, new_value: &str) -> JobUpdateCall<'a, S> {
        self._custom_field.push(new_value.to_string());
        self
    }
    /// Assignee email address, or empty string to unassign.
    ///
    /// Sets the *assignee* query property to the given value.
    pub fn assignee(mut self, new_value: &str) -> JobUpdateCall<'a, S> {
        self._assignee = Some(new_value.to_string());
        self
    }
    /// Job address as newline (Unix) separated string
    ///
    /// Sets the *address* query property to the given value.
    pub fn address(mut self, new_value: &str) -> JobUpdateCall<'a, S> {
        self._address = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> JobUpdateCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> JobUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> JobUpdateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> JobUpdateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> JobUpdateCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Retrieves a list of locations for a worker.
///
/// A builder for the *list* method supported by a *location* resource.
/// It is not used directly, but through a [`LocationMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_coordinate1 as coordinate1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use coordinate1::{Coordinate, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.location().list("teamId", "workerEmail", 78)
///              .page_token("voluptua.")
///              .max_results(67)
///              .doit().await;
/// # }
/// ```
pub struct LocationListCall<'a, S>
    where S: 'a {

    hub: &'a Coordinate<S>,
    _team_id: String,
    _worker_email: String,
    _start_timestamp_ms: u64,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for LocationListCall<'a, S> {}

impl<'a, S> LocationListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, LocationListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "coordinate.location.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "teamId", "workerEmail", "startTimestampMs", "pageToken", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("teamId", self._team_id);
        params.push("workerEmail", self._worker_email);
        params.push("startTimestampMs", self._start_timestamp_ms.to_string());
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "teams/{teamId}/workers/{workerEmail}/locations";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Readonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{teamId}", "teamId"), ("{workerEmail}", "workerEmail")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["workerEmail", "teamId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
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
    pub fn team_id(mut self, new_value: &str) -> LocationListCall<'a, S> {
        self._team_id = new_value.to_string();
        self
    }
    /// Worker email address.
    ///
    /// Sets the *worker email* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn worker_email(mut self, new_value: &str) -> LocationListCall<'a, S> {
        self._worker_email = new_value.to_string();
        self
    }
    /// Start timestamp in milliseconds since the epoch.
    ///
    /// Sets the *start timestamp ms* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn start_timestamp_ms(mut self, new_value: u64) -> LocationListCall<'a, S> {
        self._start_timestamp_ms = new_value;
        self
    }
    /// Continuation token
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> LocationListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return in one page.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> LocationListCall<'a, S> {
        self._max_results = Some(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> LocationListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> LocationListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Readonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> LocationListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> LocationListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> LocationListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Retrieves the schedule for a job.
///
/// A builder for the *get* method supported by a *schedule* resource.
/// It is not used directly, but through a [`ScheduleMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_coordinate1 as coordinate1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use coordinate1::{Coordinate, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.schedule().get("teamId", 67)
///              .doit().await;
/// # }
/// ```
pub struct ScheduleGetCall<'a, S>
    where S: 'a {

    hub: &'a Coordinate<S>,
    _team_id: String,
    _job_id: u64,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for ScheduleGetCall<'a, S> {}

impl<'a, S> ScheduleGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Schedule)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "coordinate.schedule.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "teamId", "jobId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("teamId", self._team_id);
        params.push("jobId", self._job_id.to_string());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "teams/{teamId}/jobs/{jobId}/schedule";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Readonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{teamId}", "teamId"), ("{jobId}", "jobId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["jobId", "teamId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
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
    pub fn team_id(mut self, new_value: &str) -> ScheduleGetCall<'a, S> {
        self._team_id = new_value.to_string();
        self
    }
    /// Job number
    ///
    /// Sets the *job id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn job_id(mut self, new_value: u64) -> ScheduleGetCall<'a, S> {
        self._job_id = new_value;
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ScheduleGetCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ScheduleGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Readonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ScheduleGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ScheduleGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ScheduleGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Replaces the schedule of a job with the provided schedule. This method supports patch semantics.
///
/// A builder for the *patch* method supported by a *schedule* resource.
/// It is not used directly, but through a [`ScheduleMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_coordinate1 as coordinate1;
/// use coordinate1::api::Schedule;
/// # async fn dox() {
/// # use std::default::Default;
/// # use coordinate1::{Coordinate, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Schedule::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.schedule().patch(req, "teamId", 99)
///              .start_time(84)
///              .end_time(6)
///              .duration(95)
///              .all_day(true)
///              .doit().await;
/// # }
/// ```
pub struct SchedulePatchCall<'a, S>
    where S: 'a {

    hub: &'a Coordinate<S>,
    _request: Schedule,
    _team_id: String,
    _job_id: u64,
    _start_time: Option<u64>,
    _end_time: Option<u64>,
    _duration: Option<u64>,
    _all_day: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for SchedulePatchCall<'a, S> {}

impl<'a, S> SchedulePatchCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Schedule)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "coordinate.schedule.patch",
                               http_method: hyper::Method::PATCH });

        for &field in ["alt", "teamId", "jobId", "startTime", "endTime", "duration", "allDay"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(9 + self._additional_params.len());
        params.push("teamId", self._team_id);
        params.push("jobId", self._job_id.to_string());
        if let Some(value) = self._start_time.as_ref() {
            params.push("startTime", value.to_string());
        }
        if let Some(value) = self._end_time.as_ref() {
            params.push("endTime", value.to_string());
        }
        if let Some(value) = self._duration.as_ref() {
            params.push("duration", value.to_string());
        }
        if let Some(value) = self._all_day.as_ref() {
            params.push("allDay", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "teams/{teamId}/jobs/{jobId}/schedule";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{teamId}", "teamId"), ("{jobId}", "jobId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["jobId", "teamId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
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
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::PATCH)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
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
    pub fn request(mut self, new_value: Schedule) -> SchedulePatchCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Team ID
    ///
    /// Sets the *team id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn team_id(mut self, new_value: &str) -> SchedulePatchCall<'a, S> {
        self._team_id = new_value.to_string();
        self
    }
    /// Job number
    ///
    /// Sets the *job id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn job_id(mut self, new_value: u64) -> SchedulePatchCall<'a, S> {
        self._job_id = new_value;
        self
    }
    /// Scheduled start time in milliseconds since epoch.
    ///
    /// Sets the *start time* query property to the given value.
    pub fn start_time(mut self, new_value: u64) -> SchedulePatchCall<'a, S> {
        self._start_time = Some(new_value);
        self
    }
    /// Scheduled end time in milliseconds since epoch.
    ///
    /// Sets the *end time* query property to the given value.
    pub fn end_time(mut self, new_value: u64) -> SchedulePatchCall<'a, S> {
        self._end_time = Some(new_value);
        self
    }
    /// Job duration in milliseconds.
    ///
    /// Sets the *duration* query property to the given value.
    pub fn duration(mut self, new_value: u64) -> SchedulePatchCall<'a, S> {
        self._duration = Some(new_value);
        self
    }
    /// Whether the job is scheduled for the whole day. Time of day in start/end times is ignored if this is true.
    ///
    /// Sets the *all day* query property to the given value.
    pub fn all_day(mut self, new_value: bool) -> SchedulePatchCall<'a, S> {
        self._all_day = Some(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> SchedulePatchCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> SchedulePatchCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> SchedulePatchCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> SchedulePatchCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> SchedulePatchCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Replaces the schedule of a job with the provided schedule.
///
/// A builder for the *update* method supported by a *schedule* resource.
/// It is not used directly, but through a [`ScheduleMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_coordinate1 as coordinate1;
/// use coordinate1::api::Schedule;
/// # async fn dox() {
/// # use std::default::Default;
/// # use coordinate1::{Coordinate, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Schedule::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.schedule().update(req, "teamId", 94)
///              .start_time(74)
///              .end_time(58)
///              .duration(3)
///              .all_day(true)
///              .doit().await;
/// # }
/// ```
pub struct ScheduleUpdateCall<'a, S>
    where S: 'a {

    hub: &'a Coordinate<S>,
    _request: Schedule,
    _team_id: String,
    _job_id: u64,
    _start_time: Option<u64>,
    _end_time: Option<u64>,
    _duration: Option<u64>,
    _all_day: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for ScheduleUpdateCall<'a, S> {}

impl<'a, S> ScheduleUpdateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Schedule)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "coordinate.schedule.update",
                               http_method: hyper::Method::PUT });

        for &field in ["alt", "teamId", "jobId", "startTime", "endTime", "duration", "allDay"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(9 + self._additional_params.len());
        params.push("teamId", self._team_id);
        params.push("jobId", self._job_id.to_string());
        if let Some(value) = self._start_time.as_ref() {
            params.push("startTime", value.to_string());
        }
        if let Some(value) = self._end_time.as_ref() {
            params.push("endTime", value.to_string());
        }
        if let Some(value) = self._duration.as_ref() {
            params.push("duration", value.to_string());
        }
        if let Some(value) = self._all_day.as_ref() {
            params.push("allDay", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "teams/{teamId}/jobs/{jobId}/schedule";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{teamId}", "teamId"), ("{jobId}", "jobId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["jobId", "teamId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
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
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::PUT)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
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
    pub fn request(mut self, new_value: Schedule) -> ScheduleUpdateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Team ID
    ///
    /// Sets the *team id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn team_id(mut self, new_value: &str) -> ScheduleUpdateCall<'a, S> {
        self._team_id = new_value.to_string();
        self
    }
    /// Job number
    ///
    /// Sets the *job id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn job_id(mut self, new_value: u64) -> ScheduleUpdateCall<'a, S> {
        self._job_id = new_value;
        self
    }
    /// Scheduled start time in milliseconds since epoch.
    ///
    /// Sets the *start time* query property to the given value.
    pub fn start_time(mut self, new_value: u64) -> ScheduleUpdateCall<'a, S> {
        self._start_time = Some(new_value);
        self
    }
    /// Scheduled end time in milliseconds since epoch.
    ///
    /// Sets the *end time* query property to the given value.
    pub fn end_time(mut self, new_value: u64) -> ScheduleUpdateCall<'a, S> {
        self._end_time = Some(new_value);
        self
    }
    /// Job duration in milliseconds.
    ///
    /// Sets the *duration* query property to the given value.
    pub fn duration(mut self, new_value: u64) -> ScheduleUpdateCall<'a, S> {
        self._duration = Some(new_value);
        self
    }
    /// Whether the job is scheduled for the whole day. Time of day in start/end times is ignored if this is true.
    ///
    /// Sets the *all day* query property to the given value.
    pub fn all_day(mut self, new_value: bool) -> ScheduleUpdateCall<'a, S> {
        self._all_day = Some(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ScheduleUpdateCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ScheduleUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ScheduleUpdateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ScheduleUpdateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ScheduleUpdateCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Retrieves a list of teams for a user.
///
/// A builder for the *list* method supported by a *team* resource.
/// It is not used directly, but through a [`TeamMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_coordinate1 as coordinate1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use coordinate1::{Coordinate, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
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
pub struct TeamListCall<'a, S>
    where S: 'a {

    hub: &'a Coordinate<S>,
    _worker: Option<bool>,
    _dispatcher: Option<bool>,
    _admin: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for TeamListCall<'a, S> {}

impl<'a, S> TeamListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, TeamListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "coordinate.team.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "worker", "dispatcher", "admin"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        if let Some(value) = self._worker.as_ref() {
            params.push("worker", value.to_string());
        }
        if let Some(value) = self._dispatcher.as_ref() {
            params.push("dispatcher", value.to_string());
        }
        if let Some(value) = self._admin.as_ref() {
            params.push("admin", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "teams";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Readonly.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
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
    pub fn worker(mut self, new_value: bool) -> TeamListCall<'a, S> {
        self._worker = Some(new_value);
        self
    }
    /// Whether to include teams for which the user has the Dispatcher role.
    ///
    /// Sets the *dispatcher* query property to the given value.
    pub fn dispatcher(mut self, new_value: bool) -> TeamListCall<'a, S> {
        self._dispatcher = Some(new_value);
        self
    }
    /// Whether to include teams for which the user has the Admin role.
    ///
    /// Sets the *admin* query property to the given value.
    pub fn admin(mut self, new_value: bool) -> TeamListCall<'a, S> {
        self._admin = Some(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> TeamListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> TeamListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Readonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> TeamListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> TeamListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> TeamListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Retrieves a list of workers in a team.
///
/// A builder for the *list* method supported by a *worker* resource.
/// It is not used directly, but through a [`WorkerMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_coordinate1 as coordinate1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use coordinate1::{Coordinate, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.worker().list("teamId")
///              .doit().await;
/// # }
/// ```
pub struct WorkerListCall<'a, S>
    where S: 'a {

    hub: &'a Coordinate<S>,
    _team_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for WorkerListCall<'a, S> {}

impl<'a, S> WorkerListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, WorkerListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "coordinate.worker.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "teamId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("teamId", self._team_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "teams/{teamId}/workers";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Readonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{teamId}", "teamId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["teamId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
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
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
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
    pub fn team_id(mut self, new_value: &str) -> WorkerListCall<'a, S> {
        self._team_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> WorkerListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> WorkerListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Readonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> WorkerListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> WorkerListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> WorkerListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


