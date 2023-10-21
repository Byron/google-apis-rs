use super::*;
/// A builder providing access to all methods supported on *metricDescriptor* resources.
/// It is not used directly, but through the [`CloudMonitoring`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudmonitoring2_beta2 as cloudmonitoring2_beta2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudmonitoring2_beta2::{CloudMonitoring, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudMonitoring::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.metric_descriptors();
/// # }
/// ```
pub struct MetricDescriptorMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudMonitoring<S>,
}

impl<'a, S> client::MethodsBuilder for MetricDescriptorMethods<'a, S> {}

impl<'a, S> MetricDescriptorMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a new metric.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project id. The value can be the numeric project ID or string-based project name.
    pub fn create(&self, request: MetricDescriptor, project: &str) -> MetricDescriptorCreateCall<'a, S> {
        MetricDescriptorCreateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete an existing metric.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project ID to which the metric belongs.
    /// * `metric` - Name of the metric.
    pub fn delete(&self, project: &str, metric: &str) -> MetricDescriptorDeleteCall<'a, S> {
        MetricDescriptorDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _metric: metric.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List metric descriptors that match the query. If the query is not set, then all of the metric descriptors will be returned. Large responses will be paginated, use the nextPageToken returned in the response to request subsequent pages of results by setting the pageToken query parameter to the value of the nextPageToken.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project id. The value can be the numeric project ID or string-based project name.
    pub fn list(&self, request: ListMetricDescriptorsRequest, project: &str) -> MetricDescriptorListCall<'a, S> {
        MetricDescriptorListCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _query: Default::default(),
            _page_token: Default::default(),
            _count: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *timesery* resources.
/// It is not used directly, but through the [`CloudMonitoring`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudmonitoring2_beta2 as cloudmonitoring2_beta2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudmonitoring2_beta2::{CloudMonitoring, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudMonitoring::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)` and `write(...)`
/// // to build up your call.
/// let rb = hub.timeseries();
/// # }
/// ```
pub struct TimeseryMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudMonitoring<S>,
}

impl<'a, S> client::MethodsBuilder for TimeseryMethods<'a, S> {}

impl<'a, S> TimeseryMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the data points of the time series that match the metric and labels values and that have data points in the interval. Large responses are paginated; use the nextPageToken returned in the response to request subsequent pages of results by setting the pageToken query parameter to the value of the nextPageToken.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project ID to which this time series belongs. The value can be the numeric project ID or string-based project name.
    /// * `metric` - Metric names are protocol-free URLs as listed in the Supported Metrics page. For example, compute.googleapis.com/instance/disk/read_ops_count.
    /// * `youngest` - End of the time interval (inclusive), which is expressed as an RFC 3339 timestamp.
    pub fn list(&self, request: ListTimeseriesRequest, project: &str, metric: &str, youngest: &str) -> TimeseryListCall<'a, S> {
        TimeseryListCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _metric: metric.to_string(),
            _youngest: youngest.to_string(),
            _window: Default::default(),
            _timespan: Default::default(),
            _page_token: Default::default(),
            _oldest: Default::default(),
            _labels: Default::default(),
            _count: Default::default(),
            _aggregator: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Put data points to one or more time series for one or more metrics. If a time series does not exist, a new time series will be created. It is not allowed to write a time series point that is older than the existing youngest point of that time series. Points that are older than the existing youngest point of that time series will be discarded silently. Therefore, users should make sure that points of a time series are written sequentially in the order of their end time.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project ID. The value can be the numeric project ID or string-based project name.
    pub fn write(&self, request: WriteTimeseriesRequest, project: &str) -> TimeseryWriteCall<'a, S> {
        TimeseryWriteCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *timeseriesDescriptor* resources.
/// It is not used directly, but through the [`CloudMonitoring`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudmonitoring2_beta2 as cloudmonitoring2_beta2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudmonitoring2_beta2::{CloudMonitoring, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudMonitoring::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.timeseries_descriptors();
/// # }
/// ```
pub struct TimeseriesDescriptorMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudMonitoring<S>,
}

impl<'a, S> client::MethodsBuilder for TimeseriesDescriptorMethods<'a, S> {}

impl<'a, S> TimeseriesDescriptorMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the descriptors of the time series that match the metric and labels values and that have data points in the interval. Large responses are paginated; use the nextPageToken returned in the response to request subsequent pages of results by setting the pageToken query parameter to the value of the nextPageToken.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project ID to which this time series belongs. The value can be the numeric project ID or string-based project name.
    /// * `metric` - Metric names are protocol-free URLs as listed in the Supported Metrics page. For example, compute.googleapis.com/instance/disk/read_ops_count.
    /// * `youngest` - End of the time interval (inclusive), which is expressed as an RFC 3339 timestamp.
    pub fn list(&self, request: ListTimeseriesDescriptorsRequest, project: &str, metric: &str, youngest: &str) -> TimeseriesDescriptorListCall<'a, S> {
        TimeseriesDescriptorListCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _metric: metric.to_string(),
            _youngest: youngest.to_string(),
            _window: Default::default(),
            _timespan: Default::default(),
            _page_token: Default::default(),
            _oldest: Default::default(),
            _labels: Default::default(),
            _count: Default::default(),
            _aggregator: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



