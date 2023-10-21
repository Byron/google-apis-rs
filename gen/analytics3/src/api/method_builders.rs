use super::*;
/// A builder providing access to all methods supported on *data* resources.
/// It is not used directly, but through the [`Analytics`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_analytics3 as analytics3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use analytics3::{Analytics, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Analytics::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `ga_get(...)`, `mcf_get(...)` and `realtime_get(...)`
/// // to build up your call.
/// let rb = hub.data();
/// # }
/// ```
pub struct DataMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Analytics<S>,
}

impl<'a, S> client::MethodsBuilder for DataMethods<'a, S> {}

impl<'a, S> DataMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns Analytics data for a view (profile).
    /// 
    /// # Arguments
    ///
    /// * `ids` - Unique table ID for retrieving Analytics data. Table ID is of the form ga:XXXX, where XXXX is the Analytics view (profile) ID.
    /// * `start-date` - Start date for fetching Analytics data. Requests can specify a start date formatted as YYYY-MM-DD, or as a relative date (e.g., today, yesterday, or 7daysAgo). The default value is 7daysAgo.
    /// * `end-date` - End date for fetching Analytics data. Request can should specify an end date formatted as YYYY-MM-DD, or as a relative date (e.g., today, yesterday, or 7daysAgo). The default value is yesterday.
    /// * `metrics` - A comma-separated list of Analytics metrics. E.g., 'ga:sessions,ga:pageviews'. At least one metric must be specified.
    pub fn ga_get(&self, ids: &str, start_date: &str, end_date: &str, metrics: &str) -> DataGaGetCall<'a, S> {
        DataGaGetCall {
            hub: self.hub,
            _ids: ids.to_string(),
            _start_date: start_date.to_string(),
            _end_date: end_date.to_string(),
            _metrics: metrics.to_string(),
            _start_index: Default::default(),
            _sort: Default::default(),
            _segment: Default::default(),
            _sampling_level: Default::default(),
            _output: Default::default(),
            _max_results: Default::default(),
            _include_empty_rows: Default::default(),
            _filters: Default::default(),
            _dimensions: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns Analytics Multi-Channel Funnels data for a view (profile).
    /// 
    /// # Arguments
    ///
    /// * `ids` - Unique table ID for retrieving Analytics data. Table ID is of the form ga:XXXX, where XXXX is the Analytics view (profile) ID.
    /// * `start-date` - Start date for fetching Analytics data. Requests can specify a start date formatted as YYYY-MM-DD, or as a relative date (e.g., today, yesterday, or 7daysAgo). The default value is 7daysAgo.
    /// * `end-date` - End date for fetching Analytics data. Requests can specify a start date formatted as YYYY-MM-DD, or as a relative date (e.g., today, yesterday, or 7daysAgo). The default value is 7daysAgo.
    /// * `metrics` - A comma-separated list of Multi-Channel Funnels metrics. E.g., 'mcf:totalConversions,mcf:totalConversionValue'. At least one metric must be specified.
    pub fn mcf_get(&self, ids: &str, start_date: &str, end_date: &str, metrics: &str) -> DataMcfGetCall<'a, S> {
        DataMcfGetCall {
            hub: self.hub,
            _ids: ids.to_string(),
            _start_date: start_date.to_string(),
            _end_date: end_date.to_string(),
            _metrics: metrics.to_string(),
            _start_index: Default::default(),
            _sort: Default::default(),
            _sampling_level: Default::default(),
            _max_results: Default::default(),
            _filters: Default::default(),
            _dimensions: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns real time data for a view (profile).
    /// 
    /// # Arguments
    ///
    /// * `ids` - Unique table ID for retrieving real time data. Table ID is of the form ga:XXXX, where XXXX is the Analytics view (profile) ID.
    /// * `metrics` - A comma-separated list of real time metrics. E.g., 'rt:activeUsers'. At least one metric must be specified.
    pub fn realtime_get(&self, ids: &str, metrics: &str) -> DataRealtimeGetCall<'a, S> {
        DataRealtimeGetCall {
            hub: self.hub,
            _ids: ids.to_string(),
            _metrics: metrics.to_string(),
            _sort: Default::default(),
            _max_results: Default::default(),
            _filters: Default::default(),
            _dimensions: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *management* resources.
/// It is not used directly, but through the [`Analytics`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_analytics3 as analytics3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use analytics3::{Analytics, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Analytics::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `account_summaries_list(...)`, `account_user_links_delete(...)`, `account_user_links_insert(...)`, `account_user_links_list(...)`, `account_user_links_update(...)`, `accounts_list(...)`, `client_id_hash_client_id(...)`, `custom_data_sources_list(...)`, `custom_dimensions_get(...)`, `custom_dimensions_insert(...)`, `custom_dimensions_list(...)`, `custom_dimensions_patch(...)`, `custom_dimensions_update(...)`, `custom_metrics_get(...)`, `custom_metrics_insert(...)`, `custom_metrics_list(...)`, `custom_metrics_patch(...)`, `custom_metrics_update(...)`, `experiments_delete(...)`, `experiments_get(...)`, `experiments_insert(...)`, `experiments_list(...)`, `experiments_patch(...)`, `experiments_update(...)`, `filters_delete(...)`, `filters_get(...)`, `filters_insert(...)`, `filters_list(...)`, `filters_patch(...)`, `filters_update(...)`, `goals_get(...)`, `goals_insert(...)`, `goals_list(...)`, `goals_patch(...)`, `goals_update(...)`, `profile_filter_links_delete(...)`, `profile_filter_links_get(...)`, `profile_filter_links_insert(...)`, `profile_filter_links_list(...)`, `profile_filter_links_patch(...)`, `profile_filter_links_update(...)`, `profile_user_links_delete(...)`, `profile_user_links_insert(...)`, `profile_user_links_list(...)`, `profile_user_links_update(...)`, `profiles_delete(...)`, `profiles_get(...)`, `profiles_insert(...)`, `profiles_list(...)`, `profiles_patch(...)`, `profiles_update(...)`, `remarketing_audience_delete(...)`, `remarketing_audience_get(...)`, `remarketing_audience_insert(...)`, `remarketing_audience_list(...)`, `remarketing_audience_patch(...)`, `remarketing_audience_update(...)`, `segments_list(...)`, `unsampled_reports_delete(...)`, `unsampled_reports_get(...)`, `unsampled_reports_insert(...)`, `unsampled_reports_list(...)`, `uploads_delete_upload_data(...)`, `uploads_get(...)`, `uploads_list(...)`, `uploads_upload_data(...)`, `web_property_ad_words_links_delete(...)`, `web_property_ad_words_links_get(...)`, `web_property_ad_words_links_insert(...)`, `web_property_ad_words_links_list(...)`, `web_property_ad_words_links_patch(...)`, `web_property_ad_words_links_update(...)`, `webproperties_get(...)`, `webproperties_insert(...)`, `webproperties_list(...)`, `webproperties_patch(...)`, `webproperties_update(...)`, `webproperty_user_links_delete(...)`, `webproperty_user_links_insert(...)`, `webproperty_user_links_list(...)` and `webproperty_user_links_update(...)`
/// // to build up your call.
/// let rb = hub.management();
/// # }
/// ```
pub struct ManagementMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Analytics<S>,
}

impl<'a, S> client::MethodsBuilder for ManagementMethods<'a, S> {}

impl<'a, S> ManagementMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists account summaries (lightweight tree comprised of accounts/properties/profiles) to which the user has access.
    pub fn account_summaries_list(&self) -> ManagementAccountSummaryListCall<'a, S> {
        ManagementAccountSummaryListCall {
            hub: self.hub,
            _start_index: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes a user from the given account.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID to delete the user link for.
    /// * `linkId` - Link ID to delete the user link for.
    pub fn account_user_links_delete(&self, account_id: &str, link_id: &str) -> ManagementAccountUserLinkDeleteCall<'a, S> {
        ManagementAccountUserLinkDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _link_id: link_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a new user to the given account.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID to create the user link for.
    pub fn account_user_links_insert(&self, request: EntityUserLink, account_id: &str) -> ManagementAccountUserLinkInsertCall<'a, S> {
        ManagementAccountUserLinkInsertCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists account-user links for a given account.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID to retrieve the user links for.
    pub fn account_user_links_list(&self, account_id: &str) -> ManagementAccountUserLinkListCall<'a, S> {
        ManagementAccountUserLinkListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _start_index: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates permissions for an existing user on the given account.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID to update the account-user link for.
    /// * `linkId` - Link ID to update the account-user link for.
    pub fn account_user_links_update(&self, request: EntityUserLink, account_id: &str, link_id: &str) -> ManagementAccountUserLinkUpdateCall<'a, S> {
        ManagementAccountUserLinkUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _link_id: link_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all accounts to which the user has access.
    pub fn accounts_list(&self) -> ManagementAccountListCall<'a, S> {
        ManagementAccountListCall {
            hub: self.hub,
            _start_index: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Hashes the given Client ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn client_id_hash_client_id(&self, request: HashClientIdRequest) -> ManagementClientIdHashClientIdCall<'a, S> {
        ManagementClientIdHashClientIdCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List custom data sources to which the user has access.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account Id for the custom data sources to retrieve.
    /// * `webPropertyId` - Web property Id for the custom data sources to retrieve.
    pub fn custom_data_sources_list(&self, account_id: &str, web_property_id: &str) -> ManagementCustomDataSourceListCall<'a, S> {
        ManagementCustomDataSourceListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _start_index: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a custom dimension to which the user has access.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID for the custom dimension to retrieve.
    /// * `webPropertyId` - Web property ID for the custom dimension to retrieve.
    /// * `customDimensionId` - The ID of the custom dimension to retrieve.
    pub fn custom_dimensions_get(&self, account_id: &str, web_property_id: &str, custom_dimension_id: &str) -> ManagementCustomDimensionGetCall<'a, S> {
        ManagementCustomDimensionGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _custom_dimension_id: custom_dimension_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a new custom dimension.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID for the custom dimension to create.
    /// * `webPropertyId` - Web property ID for the custom dimension to create.
    pub fn custom_dimensions_insert(&self, request: CustomDimension, account_id: &str, web_property_id: &str) -> ManagementCustomDimensionInsertCall<'a, S> {
        ManagementCustomDimensionInsertCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists custom dimensions to which the user has access.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID for the custom dimensions to retrieve.
    /// * `webPropertyId` - Web property ID for the custom dimensions to retrieve.
    pub fn custom_dimensions_list(&self, account_id: &str, web_property_id: &str) -> ManagementCustomDimensionListCall<'a, S> {
        ManagementCustomDimensionListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _start_index: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing custom dimension. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID for the custom dimension to update.
    /// * `webPropertyId` - Web property ID for the custom dimension to update.
    /// * `customDimensionId` - Custom dimension ID for the custom dimension to update.
    pub fn custom_dimensions_patch(&self, request: CustomDimension, account_id: &str, web_property_id: &str, custom_dimension_id: &str) -> ManagementCustomDimensionPatchCall<'a, S> {
        ManagementCustomDimensionPatchCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _custom_dimension_id: custom_dimension_id.to_string(),
            _ignore_custom_data_source_links: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing custom dimension.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID for the custom dimension to update.
    /// * `webPropertyId` - Web property ID for the custom dimension to update.
    /// * `customDimensionId` - Custom dimension ID for the custom dimension to update.
    pub fn custom_dimensions_update(&self, request: CustomDimension, account_id: &str, web_property_id: &str, custom_dimension_id: &str) -> ManagementCustomDimensionUpdateCall<'a, S> {
        ManagementCustomDimensionUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _custom_dimension_id: custom_dimension_id.to_string(),
            _ignore_custom_data_source_links: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a custom metric to which the user has access.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID for the custom metric to retrieve.
    /// * `webPropertyId` - Web property ID for the custom metric to retrieve.
    /// * `customMetricId` - The ID of the custom metric to retrieve.
    pub fn custom_metrics_get(&self, account_id: &str, web_property_id: &str, custom_metric_id: &str) -> ManagementCustomMetricGetCall<'a, S> {
        ManagementCustomMetricGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _custom_metric_id: custom_metric_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a new custom metric.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID for the custom metric to create.
    /// * `webPropertyId` - Web property ID for the custom dimension to create.
    pub fn custom_metrics_insert(&self, request: CustomMetric, account_id: &str, web_property_id: &str) -> ManagementCustomMetricInsertCall<'a, S> {
        ManagementCustomMetricInsertCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists custom metrics to which the user has access.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID for the custom metrics to retrieve.
    /// * `webPropertyId` - Web property ID for the custom metrics to retrieve.
    pub fn custom_metrics_list(&self, account_id: &str, web_property_id: &str) -> ManagementCustomMetricListCall<'a, S> {
        ManagementCustomMetricListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _start_index: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing custom metric. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID for the custom metric to update.
    /// * `webPropertyId` - Web property ID for the custom metric to update.
    /// * `customMetricId` - Custom metric ID for the custom metric to update.
    pub fn custom_metrics_patch(&self, request: CustomMetric, account_id: &str, web_property_id: &str, custom_metric_id: &str) -> ManagementCustomMetricPatchCall<'a, S> {
        ManagementCustomMetricPatchCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _custom_metric_id: custom_metric_id.to_string(),
            _ignore_custom_data_source_links: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing custom metric.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID for the custom metric to update.
    /// * `webPropertyId` - Web property ID for the custom metric to update.
    /// * `customMetricId` - Custom metric ID for the custom metric to update.
    pub fn custom_metrics_update(&self, request: CustomMetric, account_id: &str, web_property_id: &str, custom_metric_id: &str) -> ManagementCustomMetricUpdateCall<'a, S> {
        ManagementCustomMetricUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _custom_metric_id: custom_metric_id.to_string(),
            _ignore_custom_data_source_links: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete an experiment.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID to which the experiment belongs
    /// * `webPropertyId` - Web property ID to which the experiment belongs
    /// * `profileId` - View (Profile) ID to which the experiment belongs
    /// * `experimentId` - ID of the experiment to delete
    pub fn experiments_delete(&self, account_id: &str, web_property_id: &str, profile_id: &str, experiment_id: &str) -> ManagementExperimentDeleteCall<'a, S> {
        ManagementExperimentDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _experiment_id: experiment_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns an experiment to which the user has access.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID to retrieve the experiment for.
    /// * `webPropertyId` - Web property ID to retrieve the experiment for.
    /// * `profileId` - View (Profile) ID to retrieve the experiment for.
    /// * `experimentId` - Experiment ID to retrieve the experiment for.
    pub fn experiments_get(&self, account_id: &str, web_property_id: &str, profile_id: &str, experiment_id: &str) -> ManagementExperimentGetCall<'a, S> {
        ManagementExperimentGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _experiment_id: experiment_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a new experiment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID to create the experiment for.
    /// * `webPropertyId` - Web property ID to create the experiment for.
    /// * `profileId` - View (Profile) ID to create the experiment for.
    pub fn experiments_insert(&self, request: Experiment, account_id: &str, web_property_id: &str, profile_id: &str) -> ManagementExperimentInsertCall<'a, S> {
        ManagementExperimentInsertCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists experiments to which the user has access.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID to retrieve experiments for.
    /// * `webPropertyId` - Web property ID to retrieve experiments for.
    /// * `profileId` - View (Profile) ID to retrieve experiments for.
    pub fn experiments_list(&self, account_id: &str, web_property_id: &str, profile_id: &str) -> ManagementExperimentListCall<'a, S> {
        ManagementExperimentListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _start_index: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update an existing experiment. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID of the experiment to update.
    /// * `webPropertyId` - Web property ID of the experiment to update.
    /// * `profileId` - View (Profile) ID of the experiment to update.
    /// * `experimentId` - Experiment ID of the experiment to update.
    pub fn experiments_patch(&self, request: Experiment, account_id: &str, web_property_id: &str, profile_id: &str, experiment_id: &str) -> ManagementExperimentPatchCall<'a, S> {
        ManagementExperimentPatchCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _experiment_id: experiment_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update an existing experiment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID of the experiment to update.
    /// * `webPropertyId` - Web property ID of the experiment to update.
    /// * `profileId` - View (Profile) ID of the experiment to update.
    /// * `experimentId` - Experiment ID of the experiment to update.
    pub fn experiments_update(&self, request: Experiment, account_id: &str, web_property_id: &str, profile_id: &str, experiment_id: &str) -> ManagementExperimentUpdateCall<'a, S> {
        ManagementExperimentUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _experiment_id: experiment_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete a filter.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID to delete the filter for.
    /// * `filterId` - ID of the filter to be deleted.
    pub fn filters_delete(&self, account_id: &str, filter_id: &str) -> ManagementFilterDeleteCall<'a, S> {
        ManagementFilterDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _filter_id: filter_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns filters to which the user has access.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID to retrieve filters for.
    /// * `filterId` - Filter ID to retrieve filters for.
    pub fn filters_get(&self, account_id: &str, filter_id: &str) -> ManagementFilterGetCall<'a, S> {
        ManagementFilterGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _filter_id: filter_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a new filter.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID to create filter for.
    pub fn filters_insert(&self, request: Filter, account_id: &str) -> ManagementFilterInsertCall<'a, S> {
        ManagementFilterInsertCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all filters for an account
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID to retrieve filters for.
    pub fn filters_list(&self, account_id: &str) -> ManagementFilterListCall<'a, S> {
        ManagementFilterListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _start_index: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing filter. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID to which the filter belongs.
    /// * `filterId` - ID of the filter to be updated.
    pub fn filters_patch(&self, request: Filter, account_id: &str, filter_id: &str) -> ManagementFilterPatchCall<'a, S> {
        ManagementFilterPatchCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _filter_id: filter_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing filter.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID to which the filter belongs.
    /// * `filterId` - ID of the filter to be updated.
    pub fn filters_update(&self, request: Filter, account_id: &str, filter_id: &str) -> ManagementFilterUpdateCall<'a, S> {
        ManagementFilterUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _filter_id: filter_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a goal to which the user has access.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID to retrieve the goal for.
    /// * `webPropertyId` - Web property ID to retrieve the goal for.
    /// * `profileId` - View (Profile) ID to retrieve the goal for.
    /// * `goalId` - Goal ID to retrieve the goal for.
    pub fn goals_get(&self, account_id: &str, web_property_id: &str, profile_id: &str, goal_id: &str) -> ManagementGoalGetCall<'a, S> {
        ManagementGoalGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _goal_id: goal_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a new goal.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID to create the goal for.
    /// * `webPropertyId` - Web property ID to create the goal for.
    /// * `profileId` - View (Profile) ID to create the goal for.
    pub fn goals_insert(&self, request: Goal, account_id: &str, web_property_id: &str, profile_id: &str) -> ManagementGoalInsertCall<'a, S> {
        ManagementGoalInsertCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists goals to which the user has access.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID to retrieve goals for. Can either be a specific account ID or '~all', which refers to all the accounts that user has access to.
    /// * `webPropertyId` - Web property ID to retrieve goals for. Can either be a specific web property ID or '~all', which refers to all the web properties that user has access to.
    /// * `profileId` - View (Profile) ID to retrieve goals for. Can either be a specific view (profile) ID or '~all', which refers to all the views (profiles) that user has access to.
    pub fn goals_list(&self, account_id: &str, web_property_id: &str, profile_id: &str) -> ManagementGoalListCall<'a, S> {
        ManagementGoalListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _start_index: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing goal. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID to update the goal.
    /// * `webPropertyId` - Web property ID to update the goal.
    /// * `profileId` - View (Profile) ID to update the goal.
    /// * `goalId` - Index of the goal to be updated.
    pub fn goals_patch(&self, request: Goal, account_id: &str, web_property_id: &str, profile_id: &str, goal_id: &str) -> ManagementGoalPatchCall<'a, S> {
        ManagementGoalPatchCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _goal_id: goal_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing goal.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID to update the goal.
    /// * `webPropertyId` - Web property ID to update the goal.
    /// * `profileId` - View (Profile) ID to update the goal.
    /// * `goalId` - Index of the goal to be updated.
    pub fn goals_update(&self, request: Goal, account_id: &str, web_property_id: &str, profile_id: &str, goal_id: &str) -> ManagementGoalUpdateCall<'a, S> {
        ManagementGoalUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _goal_id: goal_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete a profile filter link.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID to which the profile filter link belongs.
    /// * `webPropertyId` - Web property Id to which the profile filter link belongs.
    /// * `profileId` - Profile ID to which the filter link belongs.
    /// * `linkId` - ID of the profile filter link to delete.
    pub fn profile_filter_links_delete(&self, account_id: &str, web_property_id: &str, profile_id: &str, link_id: &str) -> ManagementProfileFilterLinkDeleteCall<'a, S> {
        ManagementProfileFilterLinkDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _link_id: link_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a single profile filter link.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID to retrieve profile filter link for.
    /// * `webPropertyId` - Web property Id to retrieve profile filter link for.
    /// * `profileId` - Profile ID to retrieve filter link for.
    /// * `linkId` - ID of the profile filter link.
    pub fn profile_filter_links_get(&self, account_id: &str, web_property_id: &str, profile_id: &str, link_id: &str) -> ManagementProfileFilterLinkGetCall<'a, S> {
        ManagementProfileFilterLinkGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _link_id: link_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a new profile filter link.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID to create profile filter link for.
    /// * `webPropertyId` - Web property Id to create profile filter link for.
    /// * `profileId` - Profile ID to create filter link for.
    pub fn profile_filter_links_insert(&self, request: ProfileFilterLink, account_id: &str, web_property_id: &str, profile_id: &str) -> ManagementProfileFilterLinkInsertCall<'a, S> {
        ManagementProfileFilterLinkInsertCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all profile filter links for a profile.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID to retrieve profile filter links for.
    /// * `webPropertyId` - Web property Id for profile filter links for. Can either be a specific web property ID or '~all', which refers to all the web properties that user has access to.
    /// * `profileId` - Profile ID to retrieve filter links for. Can either be a specific profile ID or '~all', which refers to all the profiles that user has access to.
    pub fn profile_filter_links_list(&self, account_id: &str, web_property_id: &str, profile_id: &str) -> ManagementProfileFilterLinkListCall<'a, S> {
        ManagementProfileFilterLinkListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _start_index: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update an existing profile filter link. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID to which profile filter link belongs.
    /// * `webPropertyId` - Web property Id to which profile filter link belongs
    /// * `profileId` - Profile ID to which filter link belongs
    /// * `linkId` - ID of the profile filter link to be updated.
    pub fn profile_filter_links_patch(&self, request: ProfileFilterLink, account_id: &str, web_property_id: &str, profile_id: &str, link_id: &str) -> ManagementProfileFilterLinkPatchCall<'a, S> {
        ManagementProfileFilterLinkPatchCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _link_id: link_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update an existing profile filter link.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID to which profile filter link belongs.
    /// * `webPropertyId` - Web property Id to which profile filter link belongs
    /// * `profileId` - Profile ID to which filter link belongs
    /// * `linkId` - ID of the profile filter link to be updated.
    pub fn profile_filter_links_update(&self, request: ProfileFilterLink, account_id: &str, web_property_id: &str, profile_id: &str, link_id: &str) -> ManagementProfileFilterLinkUpdateCall<'a, S> {
        ManagementProfileFilterLinkUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _link_id: link_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes a user from the given view (profile).
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID to delete the user link for.
    /// * `webPropertyId` - Web Property ID to delete the user link for.
    /// * `profileId` - View (Profile) ID to delete the user link for.
    /// * `linkId` - Link ID to delete the user link for.
    pub fn profile_user_links_delete(&self, account_id: &str, web_property_id: &str, profile_id: &str, link_id: &str) -> ManagementProfileUserLinkDeleteCall<'a, S> {
        ManagementProfileUserLinkDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _link_id: link_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a new user to the given view (profile).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID to create the user link for.
    /// * `webPropertyId` - Web Property ID to create the user link for.
    /// * `profileId` - View (Profile) ID to create the user link for.
    pub fn profile_user_links_insert(&self, request: EntityUserLink, account_id: &str, web_property_id: &str, profile_id: &str) -> ManagementProfileUserLinkInsertCall<'a, S> {
        ManagementProfileUserLinkInsertCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists profile-user links for a given view (profile).
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID which the given view (profile) belongs to.
    /// * `webPropertyId` - Web Property ID which the given view (profile) belongs to. Can either be a specific web property ID or '~all', which refers to all the web properties that user has access to.
    /// * `profileId` - View (Profile) ID to retrieve the profile-user links for. Can either be a specific profile ID or '~all', which refers to all the profiles that user has access to.
    pub fn profile_user_links_list(&self, account_id: &str, web_property_id: &str, profile_id: &str) -> ManagementProfileUserLinkListCall<'a, S> {
        ManagementProfileUserLinkListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _start_index: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates permissions for an existing user on the given view (profile).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID to update the user link for.
    /// * `webPropertyId` - Web Property ID to update the user link for.
    /// * `profileId` - View (Profile ID) to update the user link for.
    /// * `linkId` - Link ID to update the user link for.
    pub fn profile_user_links_update(&self, request: EntityUserLink, account_id: &str, web_property_id: &str, profile_id: &str, link_id: &str) -> ManagementProfileUserLinkUpdateCall<'a, S> {
        ManagementProfileUserLinkUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _link_id: link_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a view (profile).
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID to delete the view (profile) for.
    /// * `webPropertyId` - Web property ID to delete the view (profile) for.
    /// * `profileId` - ID of the view (profile) to be deleted.
    pub fn profiles_delete(&self, account_id: &str, web_property_id: &str, profile_id: &str) -> ManagementProfileDeleteCall<'a, S> {
        ManagementProfileDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a view (profile) to which the user has access.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID to retrieve the view (profile) for.
    /// * `webPropertyId` - Web property ID to retrieve the view (profile) for.
    /// * `profileId` - View (Profile) ID to retrieve the view (profile) for.
    pub fn profiles_get(&self, account_id: &str, web_property_id: &str, profile_id: &str) -> ManagementProfileGetCall<'a, S> {
        ManagementProfileGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a new view (profile).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID to create the view (profile) for.
    /// * `webPropertyId` - Web property ID to create the view (profile) for.
    pub fn profiles_insert(&self, request: Profile, account_id: &str, web_property_id: &str) -> ManagementProfileInsertCall<'a, S> {
        ManagementProfileInsertCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists views (profiles) to which the user has access.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID for the view (profiles) to retrieve. Can either be a specific account ID or '~all', which refers to all the accounts to which the user has access.
    /// * `webPropertyId` - Web property ID for the views (profiles) to retrieve. Can either be a specific web property ID or '~all', which refers to all the web properties to which the user has access.
    pub fn profiles_list(&self, account_id: &str, web_property_id: &str) -> ManagementProfileListCall<'a, S> {
        ManagementProfileListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _start_index: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing view (profile). This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID to which the view (profile) belongs
    /// * `webPropertyId` - Web property ID to which the view (profile) belongs
    /// * `profileId` - ID of the view (profile) to be updated.
    pub fn profiles_patch(&self, request: Profile, account_id: &str, web_property_id: &str, profile_id: &str) -> ManagementProfilePatchCall<'a, S> {
        ManagementProfilePatchCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing view (profile).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID to which the view (profile) belongs
    /// * `webPropertyId` - Web property ID to which the view (profile) belongs
    /// * `profileId` - ID of the view (profile) to be updated.
    pub fn profiles_update(&self, request: Profile, account_id: &str, web_property_id: &str, profile_id: &str) -> ManagementProfileUpdateCall<'a, S> {
        ManagementProfileUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete a remarketing audience.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID to which the remarketing audience belongs.
    /// * `webPropertyId` - Web property ID to which the remarketing audience belongs.
    /// * `remarketingAudienceId` - The ID of the remarketing audience to delete.
    pub fn remarketing_audience_delete(&self, account_id: &str, web_property_id: &str, remarketing_audience_id: &str) -> ManagementRemarketingAudienceDeleteCall<'a, S> {
        ManagementRemarketingAudienceDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _remarketing_audience_id: remarketing_audience_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a remarketing audience to which the user has access.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - The account ID of the remarketing audience to retrieve.
    /// * `webPropertyId` - The web property ID of the remarketing audience to retrieve.
    /// * `remarketingAudienceId` - The ID of the remarketing audience to retrieve.
    pub fn remarketing_audience_get(&self, account_id: &str, web_property_id: &str, remarketing_audience_id: &str) -> ManagementRemarketingAudienceGetCall<'a, S> {
        ManagementRemarketingAudienceGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _remarketing_audience_id: remarketing_audience_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new remarketing audience.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - The account ID for which to create the remarketing audience.
    /// * `webPropertyId` - Web property ID for which to create the remarketing audience.
    pub fn remarketing_audience_insert(&self, request: RemarketingAudience, account_id: &str, web_property_id: &str) -> ManagementRemarketingAudienceInsertCall<'a, S> {
        ManagementRemarketingAudienceInsertCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists remarketing audiences to which the user has access.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - The account ID of the remarketing audiences to retrieve.
    /// * `webPropertyId` - The web property ID of the remarketing audiences to retrieve.
    pub fn remarketing_audience_list(&self, account_id: &str, web_property_id: &str) -> ManagementRemarketingAudienceListCall<'a, S> {
        ManagementRemarketingAudienceListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _type_: Default::default(),
            _start_index: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing remarketing audience. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - The account ID of the remarketing audience to update.
    /// * `webPropertyId` - The web property ID of the remarketing audience to update.
    /// * `remarketingAudienceId` - The ID of the remarketing audience to update.
    pub fn remarketing_audience_patch(&self, request: RemarketingAudience, account_id: &str, web_property_id: &str, remarketing_audience_id: &str) -> ManagementRemarketingAudiencePatchCall<'a, S> {
        ManagementRemarketingAudiencePatchCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _remarketing_audience_id: remarketing_audience_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing remarketing audience.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - The account ID of the remarketing audience to update.
    /// * `webPropertyId` - The web property ID of the remarketing audience to update.
    /// * `remarketingAudienceId` - The ID of the remarketing audience to update.
    pub fn remarketing_audience_update(&self, request: RemarketingAudience, account_id: &str, web_property_id: &str, remarketing_audience_id: &str) -> ManagementRemarketingAudienceUpdateCall<'a, S> {
        ManagementRemarketingAudienceUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _remarketing_audience_id: remarketing_audience_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists segments to which the user has access.
    pub fn segments_list(&self) -> ManagementSegmentListCall<'a, S> {
        ManagementSegmentListCall {
            hub: self.hub,
            _start_index: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an unsampled report.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID to delete the unsampled report for.
    /// * `webPropertyId` - Web property ID to delete the unsampled reports for.
    /// * `profileId` - View (Profile) ID to delete the unsampled report for.
    /// * `unsampledReportId` - ID of the unsampled report to be deleted.
    pub fn unsampled_reports_delete(&self, account_id: &str, web_property_id: &str, profile_id: &str, unsampled_report_id: &str) -> ManagementUnsampledReportDeleteCall<'a, S> {
        ManagementUnsampledReportDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _unsampled_report_id: unsampled_report_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a single unsampled report.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID to retrieve unsampled report for.
    /// * `webPropertyId` - Web property ID to retrieve unsampled reports for.
    /// * `profileId` - View (Profile) ID to retrieve unsampled report for.
    /// * `unsampledReportId` - ID of the unsampled report to retrieve.
    pub fn unsampled_reports_get(&self, account_id: &str, web_property_id: &str, profile_id: &str, unsampled_report_id: &str) -> ManagementUnsampledReportGetCall<'a, S> {
        ManagementUnsampledReportGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _unsampled_report_id: unsampled_report_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a new unsampled report.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID to create the unsampled report for.
    /// * `webPropertyId` - Web property ID to create the unsampled report for.
    /// * `profileId` - View (Profile) ID to create the unsampled report for.
    pub fn unsampled_reports_insert(&self, request: UnsampledReport, account_id: &str, web_property_id: &str, profile_id: &str) -> ManagementUnsampledReportInsertCall<'a, S> {
        ManagementUnsampledReportInsertCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists unsampled reports to which the user has access.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID to retrieve unsampled reports for. Must be a specific account ID, ~all is not supported.
    /// * `webPropertyId` - Web property ID to retrieve unsampled reports for. Must be a specific web property ID, ~all is not supported.
    /// * `profileId` - View (Profile) ID to retrieve unsampled reports for. Must be a specific view (profile) ID, ~all is not supported.
    pub fn unsampled_reports_list(&self, account_id: &str, web_property_id: &str, profile_id: &str) -> ManagementUnsampledReportListCall<'a, S> {
        ManagementUnsampledReportListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _profile_id: profile_id.to_string(),
            _start_index: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete data associated with a previous upload.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account Id for the uploads to be deleted.
    /// * `webPropertyId` - Web property Id for the uploads to be deleted.
    /// * `customDataSourceId` - Custom data source Id for the uploads to be deleted.
    pub fn uploads_delete_upload_data(&self, request: AnalyticsDataimportDeleteUploadDataRequest, account_id: &str, web_property_id: &str, custom_data_source_id: &str) -> ManagementUploadDeleteUploadDataCall<'a, S> {
        ManagementUploadDeleteUploadDataCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _custom_data_source_id: custom_data_source_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List uploads to which the user has access.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account Id for the upload to retrieve.
    /// * `webPropertyId` - Web property Id for the upload to retrieve.
    /// * `customDataSourceId` - Custom data source Id for upload to retrieve.
    /// * `uploadId` - Upload Id to retrieve.
    pub fn uploads_get(&self, account_id: &str, web_property_id: &str, custom_data_source_id: &str, upload_id: &str) -> ManagementUploadGetCall<'a, S> {
        ManagementUploadGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _custom_data_source_id: custom_data_source_id.to_string(),
            _upload_id: upload_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List uploads to which the user has access.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account Id for the uploads to retrieve.
    /// * `webPropertyId` - Web property Id for the uploads to retrieve.
    /// * `customDataSourceId` - Custom data source Id for uploads to retrieve.
    pub fn uploads_list(&self, account_id: &str, web_property_id: &str, custom_data_source_id: &str) -> ManagementUploadListCall<'a, S> {
        ManagementUploadListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _custom_data_source_id: custom_data_source_id.to_string(),
            _start_index: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Upload data for a custom data source.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account Id associated with the upload.
    /// * `webPropertyId` - Web property UA-string associated with the upload.
    /// * `customDataSourceId` - Custom data source Id to which the data being uploaded belongs.
    pub fn uploads_upload_data(&self, account_id: &str, web_property_id: &str, custom_data_source_id: &str) -> ManagementUploadUploadDataCall<'a, S> {
        ManagementUploadUploadDataCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _custom_data_source_id: custom_data_source_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a web property-Google Ads link.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - ID of the account which the given web property belongs to.
    /// * `webPropertyId` - Web property ID to delete the Google Ads link for.
    /// * `webPropertyAdWordsLinkId` - Web property Google Ads link ID.
    pub fn web_property_ad_words_links_delete(&self, account_id: &str, web_property_id: &str, web_property_ad_words_link_id: &str) -> ManagementWebPropertyAdWordsLinkDeleteCall<'a, S> {
        ManagementWebPropertyAdWordsLinkDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _web_property_ad_words_link_id: web_property_ad_words_link_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a web property-Google Ads link to which the user has access.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - ID of the account which the given web property belongs to.
    /// * `webPropertyId` - Web property ID to retrieve the Google Ads link for.
    /// * `webPropertyAdWordsLinkId` - Web property-Google Ads link ID.
    pub fn web_property_ad_words_links_get(&self, account_id: &str, web_property_id: &str, web_property_ad_words_link_id: &str) -> ManagementWebPropertyAdWordsLinkGetCall<'a, S> {
        ManagementWebPropertyAdWordsLinkGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _web_property_ad_words_link_id: web_property_ad_words_link_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a webProperty-Google Ads link.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - ID of the Google Analytics account to create the link for.
    /// * `webPropertyId` - Web property ID to create the link for.
    pub fn web_property_ad_words_links_insert(&self, request: EntityAdWordsLink, account_id: &str, web_property_id: &str) -> ManagementWebPropertyAdWordsLinkInsertCall<'a, S> {
        ManagementWebPropertyAdWordsLinkInsertCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists webProperty-Google Ads links for a given web property.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - ID of the account which the given web property belongs to.
    /// * `webPropertyId` - Web property ID to retrieve the Google Ads links for.
    pub fn web_property_ad_words_links_list(&self, account_id: &str, web_property_id: &str) -> ManagementWebPropertyAdWordsLinkListCall<'a, S> {
        ManagementWebPropertyAdWordsLinkListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _start_index: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing webProperty-Google Ads link. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - ID of the account which the given web property belongs to.
    /// * `webPropertyId` - Web property ID to retrieve the Google Ads link for.
    /// * `webPropertyAdWordsLinkId` - Web property-Google Ads link ID.
    pub fn web_property_ad_words_links_patch(&self, request: EntityAdWordsLink, account_id: &str, web_property_id: &str, web_property_ad_words_link_id: &str) -> ManagementWebPropertyAdWordsLinkPatchCall<'a, S> {
        ManagementWebPropertyAdWordsLinkPatchCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _web_property_ad_words_link_id: web_property_ad_words_link_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing webProperty-Google Ads link.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - ID of the account which the given web property belongs to.
    /// * `webPropertyId` - Web property ID to retrieve the Google Ads link for.
    /// * `webPropertyAdWordsLinkId` - Web property-Google Ads link ID.
    pub fn web_property_ad_words_links_update(&self, request: EntityAdWordsLink, account_id: &str, web_property_id: &str, web_property_ad_words_link_id: &str) -> ManagementWebPropertyAdWordsLinkUpdateCall<'a, S> {
        ManagementWebPropertyAdWordsLinkUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _web_property_ad_words_link_id: web_property_ad_words_link_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a web property to which the user has access.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID to retrieve the web property for.
    /// * `webPropertyId` - ID to retrieve the web property for.
    pub fn webproperties_get(&self, account_id: &str, web_property_id: &str) -> ManagementWebpropertyGetCall<'a, S> {
        ManagementWebpropertyGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a new property if the account has fewer than 20 properties. Web properties are visible in the Google Analytics interface only if they have at least one profile.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID to create the web property for.
    pub fn webproperties_insert(&self, request: Webproperty, account_id: &str) -> ManagementWebpropertyInsertCall<'a, S> {
        ManagementWebpropertyInsertCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists web properties to which the user has access.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID to retrieve web properties for. Can either be a specific account ID or '~all', which refers to all the accounts that user has access to.
    pub fn webproperties_list(&self, account_id: &str) -> ManagementWebpropertyListCall<'a, S> {
        ManagementWebpropertyListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _start_index: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing web property. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID to which the web property belongs
    /// * `webPropertyId` - Web property ID
    pub fn webproperties_patch(&self, request: Webproperty, account_id: &str, web_property_id: &str) -> ManagementWebpropertyPatchCall<'a, S> {
        ManagementWebpropertyPatchCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing web property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID to which the web property belongs
    /// * `webPropertyId` - Web property ID
    pub fn webproperties_update(&self, request: Webproperty, account_id: &str, web_property_id: &str) -> ManagementWebpropertyUpdateCall<'a, S> {
        ManagementWebpropertyUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes a user from the given web property.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID to delete the user link for.
    /// * `webPropertyId` - Web Property ID to delete the user link for.
    /// * `linkId` - Link ID to delete the user link for.
    pub fn webproperty_user_links_delete(&self, account_id: &str, web_property_id: &str, link_id: &str) -> ManagementWebpropertyUserLinkDeleteCall<'a, S> {
        ManagementWebpropertyUserLinkDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _link_id: link_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a new user to the given web property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID to create the user link for.
    /// * `webPropertyId` - Web Property ID to create the user link for.
    pub fn webproperty_user_links_insert(&self, request: EntityUserLink, account_id: &str, web_property_id: &str) -> ManagementWebpropertyUserLinkInsertCall<'a, S> {
        ManagementWebpropertyUserLinkInsertCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists webProperty-user links for a given web property.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID which the given web property belongs to.
    /// * `webPropertyId` - Web Property ID for the webProperty-user links to retrieve. Can either be a specific web property ID or '~all', which refers to all the web properties that user has access to.
    pub fn webproperty_user_links_list(&self, account_id: &str, web_property_id: &str) -> ManagementWebpropertyUserLinkListCall<'a, S> {
        ManagementWebpropertyUserLinkListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _start_index: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates permissions for an existing user on the given web property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID to update the account-user link for.
    /// * `webPropertyId` - Web property ID to update the account-user link for.
    /// * `linkId` - Link ID to update the account-user link for.
    pub fn webproperty_user_links_update(&self, request: EntityUserLink, account_id: &str, web_property_id: &str, link_id: &str) -> ManagementWebpropertyUserLinkUpdateCall<'a, S> {
        ManagementWebpropertyUserLinkUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _web_property_id: web_property_id.to_string(),
            _link_id: link_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *metadata* resources.
/// It is not used directly, but through the [`Analytics`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_analytics3 as analytics3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use analytics3::{Analytics, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Analytics::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `columns_list(...)`
/// // to build up your call.
/// let rb = hub.metadata();
/// # }
/// ```
pub struct MetadataMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Analytics<S>,
}

impl<'a, S> client::MethodsBuilder for MetadataMethods<'a, S> {}

impl<'a, S> MetadataMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all columns for a report type
    /// 
    /// # Arguments
    ///
    /// * `reportType` - Report type. Allowed Values: 'ga'. Where 'ga' corresponds to the Core Reporting API
    pub fn columns_list(&self, report_type: &str) -> MetadataColumnListCall<'a, S> {
        MetadataColumnListCall {
            hub: self.hub,
            _report_type: report_type.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *provisioning* resources.
/// It is not used directly, but through the [`Analytics`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_analytics3 as analytics3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use analytics3::{Analytics, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Analytics::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create_account_ticket(...)` and `create_account_tree(...)`
/// // to build up your call.
/// let rb = hub.provisioning();
/// # }
/// ```
pub struct ProvisioningMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Analytics<S>,
}

impl<'a, S> client::MethodsBuilder for ProvisioningMethods<'a, S> {}

impl<'a, S> ProvisioningMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an account ticket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create_account_ticket(&self, request: AccountTicket) -> ProvisioningCreateAccountTicketCall<'a, S> {
        ProvisioningCreateAccountTicketCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Provision account.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create_account_tree(&self, request: AccountTreeRequest) -> ProvisioningCreateAccountTreeCall<'a, S> {
        ProvisioningCreateAccountTreeCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *userDeletion* resources.
/// It is not used directly, but through the [`Analytics`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_analytics3 as analytics3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use analytics3::{Analytics, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Analytics::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `user_deletion_request_upsert(...)`
/// // to build up your call.
/// let rb = hub.user_deletion();
/// # }
/// ```
pub struct UserDeletionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Analytics<S>,
}

impl<'a, S> client::MethodsBuilder for UserDeletionMethods<'a, S> {}

impl<'a, S> UserDeletionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Insert or update a user deletion requests.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn user_deletion_request_upsert(&self, request: UserDeletionRequest) -> UserDeletionUserDeletionRequestUpsertCall<'a, S> {
        UserDeletionUserDeletionRequestUpsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



