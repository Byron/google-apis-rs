use super::*;
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
/// let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.custom_field_def();
/// # }
/// ```
pub struct CustomFieldDefMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Coordinate<S>,
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
/// let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.jobs();
/// # }
/// ```
pub struct JobMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Coordinate<S>,
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
/// let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.location();
/// # }
/// ```
pub struct LocationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Coordinate<S>,
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
/// let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.schedule();
/// # }
/// ```
pub struct ScheduleMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Coordinate<S>,
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
/// let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.team();
/// # }
/// ```
pub struct TeamMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Coordinate<S>,
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
/// let mut hub = Coordinate::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.worker();
/// # }
/// ```
pub struct WorkerMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Coordinate<S>,
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



