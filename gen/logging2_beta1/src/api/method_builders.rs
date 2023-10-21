use super::*;
/// A builder providing access to all methods supported on *entry* resources.
/// It is not used directly, but through the [`Logging`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_logging2_beta1 as logging2_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use logging2_beta1::{Logging, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Logging::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)` and `write(...)`
/// // to build up your call.
/// let rb = hub.entries();
/// # }
/// ```
pub struct EntryMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Logging<S>,
}

impl<'a, S> client::MethodsBuilder for EntryMethods<'a, S> {}

impl<'a, S> EntryMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists log entries. Use this method to retrieve log entries that originated from a project/folder/organization/billing account. For ways to export log entries, see Exporting Logs.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn list(&self, request: ListLogEntriesRequest) -> EntryListCall<'a, S> {
        EntryListCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Writes log entries to Logging. This API method is the only way to send log entries to Logging. This method is used, directly or indirectly, by the Logging agent (fluentd) and all logging libraries configured to use Logging. A single request may contain log entries for a maximum of 1000 different resources (projects, organizations, billing accounts or folders)
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn write(&self, request: WriteLogEntriesRequest) -> EntryWriteCall<'a, S> {
        EntryWriteCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *monitoredResourceDescriptor* resources.
/// It is not used directly, but through the [`Logging`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_logging2_beta1 as logging2_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use logging2_beta1::{Logging, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Logging::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.monitored_resource_descriptors();
/// # }
/// ```
pub struct MonitoredResourceDescriptorMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Logging<S>,
}

impl<'a, S> client::MethodsBuilder for MonitoredResourceDescriptorMethods<'a, S> {}

impl<'a, S> MonitoredResourceDescriptorMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the descriptors for monitored resource types used by Logging.
    pub fn list(&self) -> MonitoredResourceDescriptorListCall<'a, S> {
        MonitoredResourceDescriptorListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Logging`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_logging2_beta1 as logging2_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use logging2_beta1::{Logging, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Logging::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `metrics_create(...)`, `metrics_delete(...)`, `metrics_get(...)`, `metrics_list(...)`, `metrics_update(...)`, `sinks_create(...)`, `sinks_delete(...)`, `sinks_get(...)`, `sinks_list(...)` and `sinks_update(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Logging<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a logs-based metric.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The resource name of the project in which to create the metric:
    ///              "projects/[PROJECT_ID]"
    ///              The new metric must be provided in the request.
    pub fn metrics_create(&self, request: LogMetric, parent: &str) -> ProjectMetricCreateCall<'a, S> {
        ProjectMetricCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a logs-based metric.
    /// 
    /// # Arguments
    ///
    /// * `metricName` - The resource name of the metric to delete:
    ///                  "projects/[PROJECT_ID]/metrics/[METRIC_ID]"
    ///                  
    pub fn metrics_delete(&self, metric_name: &str) -> ProjectMetricDeleteCall<'a, S> {
        ProjectMetricDeleteCall {
            hub: self.hub,
            _metric_name: metric_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a logs-based metric.
    /// 
    /// # Arguments
    ///
    /// * `metricName` - The resource name of the desired metric:
    ///                  "projects/[PROJECT_ID]/metrics/[METRIC_ID]"
    ///                  
    pub fn metrics_get(&self, metric_name: &str) -> ProjectMetricGetCall<'a, S> {
        ProjectMetricGetCall {
            hub: self.hub,
            _metric_name: metric_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists logs-based metrics.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the project containing the metrics:
    ///              "projects/[PROJECT_ID]"
    ///              
    pub fn metrics_list(&self, parent: &str) -> ProjectMetricListCall<'a, S> {
        ProjectMetricListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates or updates a logs-based metric.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `metricName` - The resource name of the metric to update:
    ///                  "projects/[PROJECT_ID]/metrics/[METRIC_ID]"
    ///                  The updated metric must be provided in the request and it's name field must be the same as [METRIC_ID] If the metric does not exist in [PROJECT_ID], then a new metric is created.
    pub fn metrics_update(&self, request: LogMetric, metric_name: &str) -> ProjectMetricUpdateCall<'a, S> {
        ProjectMetricUpdateCall {
            hub: self.hub,
            _request: request,
            _metric_name: metric_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a sink that exports specified log entries to a destination. The export of newly-ingested log entries begins immediately, unless the sink's writer_identity is not permitted to write to the destination. A sink can export log entries only from the resource owning the sink.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource in which to create the sink:
    ///              "projects/[PROJECT_ID]"
    ///              "organizations/[ORGANIZATION_ID]"
    ///              "billingAccounts/[BILLING_ACCOUNT_ID]"
    ///              "folders/[FOLDER_ID]"
    ///              Examples: "projects/my-logging-project", "organizations/123456789".
    pub fn sinks_create(&self, request: LogSink, parent: &str) -> ProjectSinkCreateCall<'a, S> {
        ProjectSinkCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _unique_writer_identity: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a sink. If the sink has a unique writer_identity, then that service account is also deleted.
    /// 
    /// # Arguments
    ///
    /// * `sinkName` - Required. The full resource name of the sink to delete, including the parent resource and the sink identifier:
    ///                "projects/[PROJECT_ID]/sinks/[SINK_ID]"
    ///                "organizations/[ORGANIZATION_ID]/sinks/[SINK_ID]"
    ///                "billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_ID]"
    ///                "folders/[FOLDER_ID]/sinks/[SINK_ID]"
    ///                Example: "projects/my-project-id/sinks/my-sink-id".
    pub fn sinks_delete(&self, sink_name: &str) -> ProjectSinkDeleteCall<'a, S> {
        ProjectSinkDeleteCall {
            hub: self.hub,
            _sink_name: sink_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a sink.
    /// 
    /// # Arguments
    ///
    /// * `sinkName` - Required. The resource name of the sink:
    ///                "projects/[PROJECT_ID]/sinks/[SINK_ID]"
    ///                "organizations/[ORGANIZATION_ID]/sinks/[SINK_ID]"
    ///                "billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_ID]"
    ///                "folders/[FOLDER_ID]/sinks/[SINK_ID]"
    ///                Example: "projects/my-project-id/sinks/my-sink-id".
    pub fn sinks_get(&self, sink_name: &str) -> ProjectSinkGetCall<'a, S> {
        ProjectSinkGetCall {
            hub: self.hub,
            _sink_name: sink_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists sinks.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource whose sinks are to be listed:
    ///              "projects/[PROJECT_ID]"
    ///              "organizations/[ORGANIZATION_ID]"
    ///              "billingAccounts/[BILLING_ACCOUNT_ID]"
    ///              "folders/[FOLDER_ID]"
    ///              
    pub fn sinks_list(&self, parent: &str) -> ProjectSinkListCall<'a, S> {
        ProjectSinkListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a sink. This method replaces the following fields in the existing sink with values from the new sink: destination, and filter.The updated sink might also have a new writer_identity; see the unique_writer_identity field.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `sinkName` - Required. The full resource name of the sink to update, including the parent resource and the sink identifier:
    ///                "projects/[PROJECT_ID]/sinks/[SINK_ID]"
    ///                "organizations/[ORGANIZATION_ID]/sinks/[SINK_ID]"
    ///                "billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_ID]"
    ///                "folders/[FOLDER_ID]/sinks/[SINK_ID]"
    ///                Example: "projects/my-project-id/sinks/my-sink-id".
    pub fn sinks_update(&self, request: LogSink, sink_name: &str) -> ProjectSinkUpdateCall<'a, S> {
        ProjectSinkUpdateCall {
            hub: self.hub,
            _request: request,
            _sink_name: sink_name.to_string(),
            _update_mask: Default::default(),
            _unique_writer_identity: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



