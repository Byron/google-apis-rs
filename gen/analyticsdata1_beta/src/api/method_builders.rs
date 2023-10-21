use super::*;
/// A builder providing access to all methods supported on *property* resources.
/// It is not used directly, but through the [`AnalyticsData`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_analyticsdata1_beta as analyticsdata1_beta;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use analyticsdata1_beta::{AnalyticsData, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AnalyticsData::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `batch_run_pivot_reports(...)`, `batch_run_reports(...)`, `check_compatibility(...)`, `get_metadata(...)`, `run_pivot_report(...)`, `run_realtime_report(...)` and `run_report(...)`
/// // to build up your call.
/// let rb = hub.properties();
/// # }
/// ```
pub struct PropertyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AnalyticsData<S>,
}

impl<'a, S> client::MethodsBuilder for PropertyMethods<'a, S> {}

impl<'a, S> PropertyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns multiple pivot reports in a batch. All reports must be for the same GA4 Property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `property` - A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). This property must be specified for the batch. The property within RunPivotReportRequest may either be unspecified or consistent with this property. Example: properties/1234
    pub fn batch_run_pivot_reports(&self, request: BatchRunPivotReportsRequest, property: &str) -> PropertyBatchRunPivotReportCall<'a, S> {
        PropertyBatchRunPivotReportCall {
            hub: self.hub,
            _request: request,
            _property: property.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns multiple reports in a batch. All reports must be for the same GA4 Property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `property` - A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). This property must be specified for the batch. The property within RunReportRequest may either be unspecified or consistent with this property. Example: properties/1234
    pub fn batch_run_reports(&self, request: BatchRunReportsRequest, property: &str) -> PropertyBatchRunReportCall<'a, S> {
        PropertyBatchRunReportCall {
            hub: self.hub,
            _request: request,
            _property: property.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// This compatibility method lists dimensions and metrics that can be added to a report request and maintain compatibility. This method fails if the request's dimensions and metrics are incompatible. In Google Analytics, reports fail if they request incompatible dimensions and/or metrics; in that case, you will need to remove dimensions and/or metrics from the incompatible report until the report is compatible. The Realtime and Core reports have different compatibility rules. This method checks compatibility for Core reports.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `property` - A Google Analytics GA4 property identifier whose events are tracked. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). `property` should be the same value as in your `runReport` request. Example: properties/1234
    pub fn check_compatibility(&self, request: CheckCompatibilityRequest, property: &str) -> PropertyCheckCompatibilityCall<'a, S> {
        PropertyCheckCompatibilityCall {
            hub: self.hub,
            _request: request,
            _property: property.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns metadata for dimensions and metrics available in reporting methods. Used to explore the dimensions and metrics. In this method, a Google Analytics GA4 Property Identifier is specified in the request, and the metadata response includes Custom dimensions and metrics as well as Universal metadata. For example if a custom metric with parameter name `levels_unlocked` is registered to a property, the Metadata response will contain `customEvent:levels_unlocked`. Universal metadata are dimensions and metrics applicable to any property such as `country` and `totalUsers`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the metadata to retrieve. This name field is specified in the URL path and not URL parameters. Property is a numeric Google Analytics GA4 Property identifier. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). Example: properties/1234/metadata Set the Property ID to 0 for dimensions and metrics common to all properties. In this special mode, this method will not return custom dimensions and metrics.
    pub fn get_metadata(&self, name: &str) -> PropertyGetMetadataCall<'a, S> {
        PropertyGetMetadataCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a customized pivot report of your Google Analytics event data. Pivot reports are more advanced and expressive formats than regular reports. In a pivot report, dimensions are only visible if they are included in a pivot. Multiple pivots can be specified to further dissect your data.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `property` - A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). Within a batch request, this property should either be unspecified or consistent with the batch-level property. Example: properties/1234
    pub fn run_pivot_report(&self, request: RunPivotReportRequest, property: &str) -> PropertyRunPivotReportCall<'a, S> {
        PropertyRunPivotReportCall {
            hub: self.hub,
            _request: request,
            _property: property.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a customized report of realtime event data for your property. Events appear in realtime reports seconds after they have been sent to the Google Analytics. Realtime reports show events and usage data for the periods of time ranging from the present moment to 30 minutes ago (up to 60 minutes for Google Analytics 360 properties). For a guide to constructing realtime requests & understanding responses, see [Creating a Realtime Report](https://developers.google.com/analytics/devguides/reporting/data/v1/realtime-basics).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `property` - A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). Example: properties/1234
    pub fn run_realtime_report(&self, request: RunRealtimeReportRequest, property: &str) -> PropertyRunRealtimeReportCall<'a, S> {
        PropertyRunRealtimeReportCall {
            hub: self.hub,
            _request: request,
            _property: property.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a customized report of your Google Analytics event data. Reports contain statistics derived from data collected by the Google Analytics tracking code. The data returned from the API is as a table with columns for the requested dimensions and metrics. Metrics are individual measurements of user activity on your property, such as active users or event count. Dimensions break down metrics across some common criteria, such as country or event name. For a guide to constructing requests & understanding responses, see [Creating a Report](https://developers.google.com/analytics/devguides/reporting/data/v1/basics).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `property` - A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). Within a batch request, this property should either be unspecified or consistent with the batch-level property. Example: properties/1234
    pub fn run_report(&self, request: RunReportRequest, property: &str) -> PropertyRunReportCall<'a, S> {
        PropertyRunReportCall {
            hub: self.hub,
            _request: request,
            _property: property.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



