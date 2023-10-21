use super::*;
/// A builder providing access to all methods supported on *service* resources.
/// It is not used directly, but through the [`ServiceControl`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_servicecontrol1 as servicecontrol1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use servicecontrol1::{ServiceControl, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ServiceControl::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `allocate_quota(...)`, `check(...)` and `report(...)`
/// // to build up your call.
/// let rb = hub.services();
/// # }
/// ```
pub struct ServiceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ServiceControl<S>,
}

impl<'a, S> client::MethodsBuilder for ServiceMethods<'a, S> {}

impl<'a, S> ServiceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Attempts to allocate quota for the specified consumer. It should be called before the operation is executed. This method requires the `servicemanagement.services.quota` permission on the specified service. For more information, see [Cloud IAM](https://cloud.google.com/iam). **NOTE:** The client **must** fail-open on server errors `INTERNAL`, `UNKNOWN`, `DEADLINE_EXCEEDED`, and `UNAVAILABLE`. To ensure system reliability, the server may inject these errors to prohibit any hard dependency on the quota functionality.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `serviceName` - Name of the service as specified in the service configuration. For example, `"pubsub.googleapis.com"`. See google.api.Service for the definition of a service name.
    pub fn allocate_quota(&self, request: AllocateQuotaRequest, service_name: &str) -> ServiceAllocateQuotaCall<'a, S> {
        ServiceAllocateQuotaCall {
            hub: self.hub,
            _request: request,
            _service_name: service_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Checks whether an operation on a service should be allowed to proceed based on the configuration of the service and related policies. It must be called before the operation is executed. If feasible, the client should cache the check results and reuse them for 60 seconds. In case of any server errors, the client should rely on the cached results for much longer time to avoid outage. WARNING: There is general 60s delay for the configuration and policy propagation, therefore callers MUST NOT depend on the `Check` method having the latest policy information. NOTE: the CheckRequest has the size limit (wire-format byte size) of 1MB. This method requires the `servicemanagement.services.check` permission on the specified service. For more information, see [Cloud IAM](https://cloud.google.com/iam).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `serviceName` - The service name as specified in its service configuration. For example, `"pubsub.googleapis.com"`. See [google.api.Service](https://cloud.google.com/service-management/reference/rpc/google.api#google.api.Service) for the definition of a service name.
    pub fn check(&self, request: CheckRequest, service_name: &str) -> ServiceCheckCall<'a, S> {
        ServiceCheckCall {
            hub: self.hub,
            _request: request,
            _service_name: service_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reports operation results to Google Service Control, such as logs and metrics. It should be called after an operation is completed. If feasible, the client should aggregate reporting data for up to 5 seconds to reduce API traffic. Limiting aggregation to 5 seconds is to reduce data loss during client crashes. Clients should carefully choose the aggregation time window to avoid data loss risk more than 0.01% for business and compliance reasons. NOTE: the ReportRequest has the size limit (wire-format byte size) of 1MB. This method requires the `servicemanagement.services.report` permission on the specified service. For more information, see [Google Cloud IAM](https://cloud.google.com/iam).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `serviceName` - The service name as specified in its service configuration. For example, `"pubsub.googleapis.com"`. See [google.api.Service](https://cloud.google.com/service-management/reference/rpc/google.api#google.api.Service) for the definition of a service name.
    pub fn report(&self, request: ReportRequest, service_name: &str) -> ServiceReportCall<'a, S> {
        ServiceReportCall {
            hub: self.hub,
            _request: request,
            _service_name: service_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



