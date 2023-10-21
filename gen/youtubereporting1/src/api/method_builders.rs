use super::*;
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
/// use std::default::Default;
/// use youtubereporting1::{YouTubeReporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTubeReporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `list(...)`, `reports_get(...)` and `reports_list(...)`
/// // to build up your call.
/// let rb = hub.jobs();
/// # }
/// ```
pub struct JobMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTubeReporting<S>,
}

impl<'a, S> client::MethodsBuilder for JobMethods<'a, S> {}

impl<'a, S> JobMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the metadata of a specific report.
    /// 
    /// # Arguments
    ///
    /// * `jobId` - The ID of the job.
    /// * `reportId` - The ID of the report to retrieve.
    pub fn reports_get(&self, job_id: &str, report_id: &str) -> JobReportGetCall<'a, S> {
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
    pub fn reports_list(&self, job_id: &str) -> JobReportListCall<'a, S> {
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
    pub fn create(&self, request: Job) -> JobCreateCall<'a, S> {
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
    pub fn delete(&self, job_id: &str) -> JobDeleteCall<'a, S> {
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
    pub fn get(&self, job_id: &str) -> JobGetCall<'a, S> {
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
    pub fn list(&self) -> JobListCall<'a, S> {
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
/// use std::default::Default;
/// use youtubereporting1::{YouTubeReporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTubeReporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `download(...)`
/// // to build up your call.
/// let rb = hub.media();
/// # }
/// ```
pub struct MediaMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTubeReporting<S>,
}

impl<'a, S> client::MethodsBuilder for MediaMethods<'a, S> {}

impl<'a, S> MediaMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Method for media download. Download is supported on the URI `/v1/media/{+name}?alt=media`.
    /// 
    /// # Arguments
    ///
    /// * `resourceName` - Name of the media that is being downloaded.
    pub fn download(&self, resource_name: &str) -> MediaDownloadCall<'a, S> {
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
/// use std::default::Default;
/// use youtubereporting1::{YouTubeReporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTubeReporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.report_types();
/// # }
/// ```
pub struct ReportTypeMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTubeReporting<S>,
}

impl<'a, S> client::MethodsBuilder for ReportTypeMethods<'a, S> {}

impl<'a, S> ReportTypeMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists report types.
    pub fn list(&self) -> ReportTypeListCall<'a, S> {
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



