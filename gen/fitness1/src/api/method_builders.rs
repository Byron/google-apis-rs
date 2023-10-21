use super::*;
/// A builder providing access to all methods supported on *user* resources.
/// It is not used directly, but through the [`Fitness`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_fitness1 as fitness1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use fitness1::{Fitness, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Fitness::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `data_sources_create(...)`, `data_sources_data_point_changes_list(...)`, `data_sources_datasets_delete(...)`, `data_sources_datasets_get(...)`, `data_sources_datasets_patch(...)`, `data_sources_delete(...)`, `data_sources_get(...)`, `data_sources_list(...)`, `data_sources_update(...)`, `dataset_aggregate(...)`, `sessions_delete(...)`, `sessions_list(...)` and `sessions_update(...)`
/// // to build up your call.
/// let rb = hub.users();
/// # }
/// ```
pub struct UserMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Fitness<S>,
}

impl<'a, S> client::MethodsBuilder for UserMethods<'a, S> {}

impl<'a, S> UserMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Queries for user's data point changes for a particular data source.
    /// 
    /// # Arguments
    ///
    /// * `userId` - List data points for the person identified. Use me to indicate the authenticated user. Only me is supported at this time.
    /// * `dataSourceId` - The data stream ID of the data source that created the dataset.
    pub fn data_sources_data_point_changes_list(&self, user_id: &str, data_source_id: &str) -> UserDataSourceDataPointChangeListCall<'a, S> {
        UserDataSourceDataPointChangeListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _data_source_id: data_source_id.to_string(),
            _page_token: Default::default(),
            _limit: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Performs an inclusive delete of all data points whose start and end times have any overlap with the time range specified by the dataset ID. For most data types, the entire data point will be deleted. For data types where the time span represents a consistent value (such as com.google.activity.segment), and a data point straddles either end point of the dataset, only the overlapping portion of the data point will be deleted.
    /// 
    /// # Arguments
    ///
    /// * `userId` - Delete a dataset for the person identified. Use me to indicate the authenticated user. Only me is supported at this time.
    /// * `dataSourceId` - The data stream ID of the data source that created the dataset.
    /// * `datasetId` - Dataset identifier that is a composite of the minimum data point start time and maximum data point end time represented as nanoseconds from the epoch. The ID is formatted like: "startTime-endTime" where startTime and endTime are 64 bit integers.
    pub fn data_sources_datasets_delete(&self, user_id: &str, data_source_id: &str, dataset_id: &str) -> UserDataSourceDatasetDeleteCall<'a, S> {
        UserDataSourceDatasetDeleteCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _data_source_id: data_source_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a dataset containing all data points whose start and end times overlap with the specified range of the dataset minimum start time and maximum end time. Specifically, any data point whose start time is less than or equal to the dataset end time and whose end time is greater than or equal to the dataset start time.
    /// 
    /// # Arguments
    ///
    /// * `userId` - Retrieve a dataset for the person identified. Use me to indicate the authenticated user. Only me is supported at this time.
    /// * `dataSourceId` - The data stream ID of the data source that created the dataset.
    /// * `datasetId` - Dataset identifier that is a composite of the minimum data point start time and maximum data point end time represented as nanoseconds from the epoch. The ID is formatted like: "startTime-endTime" where startTime and endTime are 64 bit integers.
    pub fn data_sources_datasets_get(&self, user_id: &str, data_source_id: &str, dataset_id: &str) -> UserDataSourceDatasetGetCall<'a, S> {
        UserDataSourceDatasetGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _data_source_id: data_source_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _page_token: Default::default(),
            _limit: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds data points to a dataset. The dataset need not be previously created. All points within the given dataset will be returned with subsquent calls to retrieve this dataset. Data points can belong to more than one dataset. This method does not use patch semantics: the data points provided are merely inserted, with no existing data replaced.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - Patch a dataset for the person identified. Use me to indicate the authenticated user. Only me is supported at this time.
    /// * `dataSourceId` - The data stream ID of the data source that created the dataset.
    /// * `datasetId` - This field is not used, and can be safely omitted.
    pub fn data_sources_datasets_patch(&self, request: Dataset, user_id: &str, data_source_id: &str, dataset_id: &str) -> UserDataSourceDatasetPatchCall<'a, S> {
        UserDataSourceDatasetPatchCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _data_source_id: data_source_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new data source that is unique across all data sources belonging to this user. A data source is a unique source of sensor data. Data sources can expose raw data coming from hardware sensors on local or companion devices. They can also expose derived data, created by transforming or merging other data sources. Multiple data sources can exist for the same data type. Every data point in every dataset inserted into or read from the Fitness API has an associated data source. Each data source produces a unique stream of dataset updates, with a unique data source identifier. Not all changes to data source affect the data stream ID, so that data collected by updated versions of the same application/device can still be considered to belong to the same data source. Data sources are identified using a string generated by the server, based on the contents of the source being created. The dataStreamId field should not be set when invoking this method. It will be automatically generated by the server with the correct format. If a dataStreamId is set, it must match the format that the server would generate. This format is a combination of some fields from the data source, and has a specific order. If it doesn't match, the request will fail with an error. Specifying a DataType which is not a known type (beginning with "com.google.") will create a DataSource with a *custom data type*. Custom data types are only readable by the application that created them. Custom data types are *deprecated*; use standard data types instead. In addition to the data source fields included in the data source ID, the developer project number that is authenticated when creating the data source is included. This developer project number is obfuscated when read by any other developer reading public data types.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - Create the data source for the person identified. Use me to indicate the authenticated user. Only me is supported at this time.
    pub fn data_sources_create(&self, request: DataSource, user_id: &str) -> UserDataSourceCreateCall<'a, S> {
        UserDataSourceCreateCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified data source. The request will fail if the data source contains any data points.
    /// 
    /// # Arguments
    ///
    /// * `userId` - Retrieve a data source for the person identified. Use me to indicate the authenticated user. Only me is supported at this time.
    /// * `dataSourceId` - The data stream ID of the data source to delete.
    pub fn data_sources_delete(&self, user_id: &str, data_source_id: &str) -> UserDataSourceDeleteCall<'a, S> {
        UserDataSourceDeleteCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _data_source_id: data_source_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified data source.
    /// 
    /// # Arguments
    ///
    /// * `userId` - Retrieve a data source for the person identified. Use me to indicate the authenticated user. Only me is supported at this time.
    /// * `dataSourceId` - The data stream ID of the data source to retrieve.
    pub fn data_sources_get(&self, user_id: &str, data_source_id: &str) -> UserDataSourceGetCall<'a, S> {
        UserDataSourceGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _data_source_id: data_source_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all data sources that are visible to the developer, using the OAuth scopes provided. The list is not exhaustive; the user may have private data sources that are only visible to other developers, or calls using other scopes.
    /// 
    /// # Arguments
    ///
    /// * `userId` - List data sources for the person identified. Use me to indicate the authenticated user. Only me is supported at this time.
    pub fn data_sources_list(&self, user_id: &str) -> UserDataSourceListCall<'a, S> {
        UserDataSourceListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _data_type_name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified data source. The dataStreamId, dataType, type, dataStreamName, and device properties with the exception of version, cannot be modified. Data sources are identified by their dataStreamId.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - Update the data source for the person identified. Use me to indicate the authenticated user. Only me is supported at this time.
    /// * `dataSourceId` - The data stream ID of the data source to update.
    pub fn data_sources_update(&self, request: DataSource, user_id: &str, data_source_id: &str) -> UserDataSourceUpdateCall<'a, S> {
        UserDataSourceUpdateCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _data_source_id: data_source_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Aggregates data of a certain type or stream into buckets divided by a given type of boundary. Multiple data sets of multiple types and from multiple sources can be aggregated into exactly one bucket type per request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - Aggregate data for the person identified. Use me to indicate the authenticated user. Only me is supported at this time.
    pub fn dataset_aggregate(&self, request: AggregateRequest, user_id: &str) -> UserDatasetAggregateCall<'a, S> {
        UserDatasetAggregateCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a session specified by the given session ID.
    /// 
    /// # Arguments
    ///
    /// * `userId` - Delete a session for the person identified. Use me to indicate the authenticated user. Only me is supported at this time.
    /// * `sessionId` - The ID of the session to be deleted.
    pub fn sessions_delete(&self, user_id: &str, session_id: &str) -> UserSessionDeleteCall<'a, S> {
        UserSessionDeleteCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _session_id: session_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists sessions previously created.
    /// 
    /// # Arguments
    ///
    /// * `userId` - List sessions for the person identified. Use me to indicate the authenticated user. Only me is supported at this time.
    pub fn sessions_list(&self, user_id: &str) -> UserSessionListCall<'a, S> {
        UserSessionListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _start_time: Default::default(),
            _page_token: Default::default(),
            _include_deleted: Default::default(),
            _end_time: Default::default(),
            _activity_type: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates or insert a given session.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - Create sessions for the person identified. Use me to indicate the authenticated user. Only me is supported at this time.
    /// * `sessionId` - The ID of the session to be created.
    pub fn sessions_update(&self, request: Session, user_id: &str, session_id: &str) -> UserSessionUpdateCall<'a, S> {
        UserSessionUpdateCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _session_id: session_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



