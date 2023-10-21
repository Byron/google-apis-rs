use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`BigQueryDataTransfer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_bigquerydatatransfer1 as bigquerydatatransfer1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use bigquerydatatransfer1::{BigQueryDataTransfer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = BigQueryDataTransfer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `data_sources_check_valid_creds(...)`, `data_sources_get(...)`, `data_sources_list(...)`, `enroll_data_sources(...)`, `locations_data_sources_check_valid_creds(...)`, `locations_data_sources_get(...)`, `locations_data_sources_list(...)`, `locations_enroll_data_sources(...)`, `locations_get(...)`, `locations_list(...)`, `locations_transfer_configs_create(...)`, `locations_transfer_configs_delete(...)`, `locations_transfer_configs_get(...)`, `locations_transfer_configs_list(...)`, `locations_transfer_configs_patch(...)`, `locations_transfer_configs_runs_delete(...)`, `locations_transfer_configs_runs_get(...)`, `locations_transfer_configs_runs_list(...)`, `locations_transfer_configs_runs_transfer_logs_list(...)`, `locations_transfer_configs_schedule_runs(...)`, `locations_transfer_configs_start_manual_runs(...)`, `transfer_configs_create(...)`, `transfer_configs_delete(...)`, `transfer_configs_get(...)`, `transfer_configs_list(...)`, `transfer_configs_patch(...)`, `transfer_configs_runs_delete(...)`, `transfer_configs_runs_get(...)`, `transfer_configs_runs_list(...)`, `transfer_configs_runs_transfer_logs_list(...)`, `transfer_configs_schedule_runs(...)` and `transfer_configs_start_manual_runs(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a BigQueryDataTransfer<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns true if valid credentials exist for the given data source and requesting user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The data source in the form: `projects/{project_id}/dataSources/{data_source_id}` or `projects/{project_id}/locations/{location_id}/dataSources/{data_source_id}`.
    pub fn data_sources_check_valid_creds(&self, request: CheckValidCredsRequest, name: &str) -> ProjectDataSourceCheckValidCredCall<'a, S> {
        ProjectDataSourceCheckValidCredCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a supported data source and returns its settings.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The field will contain name of the resource requested, for example: `projects/{project_id}/dataSources/{data_source_id}` or `projects/{project_id}/locations/{location_id}/dataSources/{data_source_id}`
    pub fn data_sources_get(&self, name: &str) -> ProjectDataSourceGetCall<'a, S> {
        ProjectDataSourceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists supported data sources and returns their settings.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The BigQuery project id for which data sources should be returned. Must be in the form: `projects/{project_id}` or `projects/{project_id}/locations/{location_id}`
    pub fn data_sources_list(&self, parent: &str) -> ProjectDataSourceListCall<'a, S> {
        ProjectDataSourceListCall {
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
    /// Returns true if valid credentials exist for the given data source and requesting user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The data source in the form: `projects/{project_id}/dataSources/{data_source_id}` or `projects/{project_id}/locations/{location_id}/dataSources/{data_source_id}`.
    pub fn locations_data_sources_check_valid_creds(&self, request: CheckValidCredsRequest, name: &str) -> ProjectLocationDataSourceCheckValidCredCall<'a, S> {
        ProjectLocationDataSourceCheckValidCredCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a supported data source and returns its settings.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The field will contain name of the resource requested, for example: `projects/{project_id}/dataSources/{data_source_id}` or `projects/{project_id}/locations/{location_id}/dataSources/{data_source_id}`
    pub fn locations_data_sources_get(&self, name: &str) -> ProjectLocationDataSourceGetCall<'a, S> {
        ProjectLocationDataSourceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists supported data sources and returns their settings.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The BigQuery project id for which data sources should be returned. Must be in the form: `projects/{project_id}` or `projects/{project_id}/locations/{location_id}`
    pub fn locations_data_sources_list(&self, parent: &str) -> ProjectLocationDataSourceListCall<'a, S> {
        ProjectLocationDataSourceListCall {
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
    /// Returns log messages for the transfer run.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Transfer run name in the form: `projects/{project_id}/transferConfigs/{config_id}/runs/{run_id}` or `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}/runs/{run_id}`
    pub fn locations_transfer_configs_runs_transfer_logs_list(&self, parent: &str) -> ProjectLocationTransferConfigRunTransferLogListCall<'a, S> {
        ProjectLocationTransferConfigRunTransferLogListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _message_types: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified transfer run.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The field will contain name of the resource requested, for example: `projects/{project_id}/transferConfigs/{config_id}/runs/{run_id}` or `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}/runs/{run_id}`
    pub fn locations_transfer_configs_runs_delete(&self, name: &str) -> ProjectLocationTransferConfigRunDeleteCall<'a, S> {
        ProjectLocationTransferConfigRunDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns information about the particular transfer run.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The field will contain name of the resource requested, for example: `projects/{project_id}/transferConfigs/{config_id}/runs/{run_id}` or `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}/runs/{run_id}`
    pub fn locations_transfer_configs_runs_get(&self, name: &str) -> ProjectLocationTransferConfigRunGetCall<'a, S> {
        ProjectLocationTransferConfigRunGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns information about running and completed transfer runs.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of transfer configuration for which transfer runs should be retrieved. Format of transfer configuration resource name is: `projects/{project_id}/transferConfigs/{config_id}` or `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}`.
    pub fn locations_transfer_configs_runs_list(&self, parent: &str) -> ProjectLocationTransferConfigRunListCall<'a, S> {
        ProjectLocationTransferConfigRunListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _states: Default::default(),
            _run_attempt: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new data transfer configuration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The BigQuery project id where the transfer configuration should be created. Must be in the format projects/{project_id}/locations/{location_id} or projects/{project_id}. If specified location and location of the destination bigquery dataset do not match - the request will fail.
    pub fn locations_transfer_configs_create(&self, request: TransferConfig, parent: &str) -> ProjectLocationTransferConfigCreateCall<'a, S> {
        ProjectLocationTransferConfigCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _version_info: Default::default(),
            _service_account_name: Default::default(),
            _authorization_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a data transfer configuration, including any associated transfer runs and logs.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The field will contain name of the resource requested, for example: `projects/{project_id}/transferConfigs/{config_id}` or `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}`
    pub fn locations_transfer_configs_delete(&self, name: &str) -> ProjectLocationTransferConfigDeleteCall<'a, S> {
        ProjectLocationTransferConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns information about a data transfer config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The field will contain name of the resource requested, for example: `projects/{project_id}/transferConfigs/{config_id}` or `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}`
    pub fn locations_transfer_configs_get(&self, name: &str) -> ProjectLocationTransferConfigGetCall<'a, S> {
        ProjectLocationTransferConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns information about all transfer configs owned by a project in the specified location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The BigQuery project id for which transfer configs should be returned: `projects/{project_id}` or `projects/{project_id}/locations/{location_id}`
    pub fn locations_transfer_configs_list(&self, parent: &str) -> ProjectLocationTransferConfigListCall<'a, S> {
        ProjectLocationTransferConfigListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _data_source_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a data transfer configuration. All fields must be set, even if they are not updated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the transfer config. Transfer config names have the form `projects/{project_id}/locations/{region}/transferConfigs/{config_id}`. Where `config_id` is usually a uuid, even though it is not guaranteed or required. The name is ignored when creating a transfer config.
    pub fn locations_transfer_configs_patch(&self, request: TransferConfig, name: &str) -> ProjectLocationTransferConfigPatchCall<'a, S> {
        ProjectLocationTransferConfigPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _version_info: Default::default(),
            _update_mask: Default::default(),
            _service_account_name: Default::default(),
            _authorization_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates transfer runs for a time range [start_time, end_time]. For each date - or whatever granularity the data source supports - in the range, one transfer run is created. Note that runs are created per UTC time in the time range. DEPRECATED: use StartManualTransferRuns instead.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Transfer configuration name in the form: `projects/{project_id}/transferConfigs/{config_id}` or `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}`.
    pub fn locations_transfer_configs_schedule_runs(&self, request: ScheduleTransferRunsRequest, parent: &str) -> ProjectLocationTransferConfigScheduleRunCall<'a, S> {
        ProjectLocationTransferConfigScheduleRunCall {
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
    /// Start manual transfer runs to be executed now with schedule_time equal to current time. The transfer runs can be created for a time range where the run_time is between start_time (inclusive) and end_time (exclusive), or for a specific run_time.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Transfer configuration name in the form: `projects/{project_id}/transferConfigs/{config_id}` or `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}`.
    pub fn locations_transfer_configs_start_manual_runs(&self, request: StartManualTransferRunsRequest, parent: &str) -> ProjectLocationTransferConfigStartManualRunCall<'a, S> {
        ProjectLocationTransferConfigStartManualRunCall {
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
    /// Enroll data sources in a user project. This allows users to create transfer configurations for these data sources. They will also appear in the ListDataSources RPC and as such, will appear in the [BigQuery UI](https://console.cloud.google.com/bigquery), and the documents can be found in the public guide for [BigQuery Web UI](https://cloud.google.com/bigquery/bigquery-web-ui) and [Data Transfer Service](https://cloud.google.com/bigquery/docs/working-with-transfers).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the project resource in the form: `projects/{project_id}`
    pub fn locations_enroll_data_sources(&self, request: EnrollDataSourcesRequest, name: &str) -> ProjectLocationEnrollDataSourceCall<'a, S> {
        ProjectLocationEnrollDataSourceCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about a location.
    /// 
    /// # Arguments
    ///
    /// * `name` - Resource name for the location.
    pub fn locations_get(&self, name: &str) -> ProjectLocationGetCall<'a, S> {
        ProjectLocationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists information about the supported locations for this service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource that owns the locations collection, if applicable.
    pub fn locations_list(&self, name: &str) -> ProjectLocationListCall<'a, S> {
        ProjectLocationListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns log messages for the transfer run.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Transfer run name in the form: `projects/{project_id}/transferConfigs/{config_id}/runs/{run_id}` or `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}/runs/{run_id}`
    pub fn transfer_configs_runs_transfer_logs_list(&self, parent: &str) -> ProjectTransferConfigRunTransferLogListCall<'a, S> {
        ProjectTransferConfigRunTransferLogListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _message_types: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified transfer run.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The field will contain name of the resource requested, for example: `projects/{project_id}/transferConfigs/{config_id}/runs/{run_id}` or `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}/runs/{run_id}`
    pub fn transfer_configs_runs_delete(&self, name: &str) -> ProjectTransferConfigRunDeleteCall<'a, S> {
        ProjectTransferConfigRunDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns information about the particular transfer run.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The field will contain name of the resource requested, for example: `projects/{project_id}/transferConfigs/{config_id}/runs/{run_id}` or `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}/runs/{run_id}`
    pub fn transfer_configs_runs_get(&self, name: &str) -> ProjectTransferConfigRunGetCall<'a, S> {
        ProjectTransferConfigRunGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns information about running and completed transfer runs.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of transfer configuration for which transfer runs should be retrieved. Format of transfer configuration resource name is: `projects/{project_id}/transferConfigs/{config_id}` or `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}`.
    pub fn transfer_configs_runs_list(&self, parent: &str) -> ProjectTransferConfigRunListCall<'a, S> {
        ProjectTransferConfigRunListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _states: Default::default(),
            _run_attempt: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new data transfer configuration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The BigQuery project id where the transfer configuration should be created. Must be in the format projects/{project_id}/locations/{location_id} or projects/{project_id}. If specified location and location of the destination bigquery dataset do not match - the request will fail.
    pub fn transfer_configs_create(&self, request: TransferConfig, parent: &str) -> ProjectTransferConfigCreateCall<'a, S> {
        ProjectTransferConfigCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _version_info: Default::default(),
            _service_account_name: Default::default(),
            _authorization_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a data transfer configuration, including any associated transfer runs and logs.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The field will contain name of the resource requested, for example: `projects/{project_id}/transferConfigs/{config_id}` or `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}`
    pub fn transfer_configs_delete(&self, name: &str) -> ProjectTransferConfigDeleteCall<'a, S> {
        ProjectTransferConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns information about a data transfer config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The field will contain name of the resource requested, for example: `projects/{project_id}/transferConfigs/{config_id}` or `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}`
    pub fn transfer_configs_get(&self, name: &str) -> ProjectTransferConfigGetCall<'a, S> {
        ProjectTransferConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns information about all transfer configs owned by a project in the specified location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The BigQuery project id for which transfer configs should be returned: `projects/{project_id}` or `projects/{project_id}/locations/{location_id}`
    pub fn transfer_configs_list(&self, parent: &str) -> ProjectTransferConfigListCall<'a, S> {
        ProjectTransferConfigListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _data_source_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a data transfer configuration. All fields must be set, even if they are not updated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the transfer config. Transfer config names have the form `projects/{project_id}/locations/{region}/transferConfigs/{config_id}`. Where `config_id` is usually a uuid, even though it is not guaranteed or required. The name is ignored when creating a transfer config.
    pub fn transfer_configs_patch(&self, request: TransferConfig, name: &str) -> ProjectTransferConfigPatchCall<'a, S> {
        ProjectTransferConfigPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _version_info: Default::default(),
            _update_mask: Default::default(),
            _service_account_name: Default::default(),
            _authorization_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates transfer runs for a time range [start_time, end_time]. For each date - or whatever granularity the data source supports - in the range, one transfer run is created. Note that runs are created per UTC time in the time range. DEPRECATED: use StartManualTransferRuns instead.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Transfer configuration name in the form: `projects/{project_id}/transferConfigs/{config_id}` or `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}`.
    pub fn transfer_configs_schedule_runs(&self, request: ScheduleTransferRunsRequest, parent: &str) -> ProjectTransferConfigScheduleRunCall<'a, S> {
        ProjectTransferConfigScheduleRunCall {
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
    /// Start manual transfer runs to be executed now with schedule_time equal to current time. The transfer runs can be created for a time range where the run_time is between start_time (inclusive) and end_time (exclusive), or for a specific run_time.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Transfer configuration name in the form: `projects/{project_id}/transferConfigs/{config_id}` or `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}`.
    pub fn transfer_configs_start_manual_runs(&self, request: StartManualTransferRunsRequest, parent: &str) -> ProjectTransferConfigStartManualRunCall<'a, S> {
        ProjectTransferConfigStartManualRunCall {
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
    /// Enroll data sources in a user project. This allows users to create transfer configurations for these data sources. They will also appear in the ListDataSources RPC and as such, will appear in the [BigQuery UI](https://console.cloud.google.com/bigquery), and the documents can be found in the public guide for [BigQuery Web UI](https://cloud.google.com/bigquery/bigquery-web-ui) and [Data Transfer Service](https://cloud.google.com/bigquery/docs/working-with-transfers).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the project resource in the form: `projects/{project_id}`
    pub fn enroll_data_sources(&self, request: EnrollDataSourcesRequest, name: &str) -> ProjectEnrollDataSourceCall<'a, S> {
        ProjectEnrollDataSourceCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



