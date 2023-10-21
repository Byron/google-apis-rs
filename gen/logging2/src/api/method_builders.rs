use super::*;
/// A builder providing access to all methods supported on *billingAccount* resources.
/// It is not used directly, but through the [`Logging`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_logging2 as logging2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use logging2::{Logging, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Logging::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `exclusions_create(...)`, `exclusions_delete(...)`, `exclusions_get(...)`, `exclusions_list(...)`, `exclusions_patch(...)`, `get_cmek_settings(...)`, `get_settings(...)`, `locations_buckets_create(...)`, `locations_buckets_delete(...)`, `locations_buckets_get(...)`, `locations_buckets_list(...)`, `locations_buckets_patch(...)`, `locations_buckets_undelete(...)`, `locations_buckets_views_create(...)`, `locations_buckets_views_delete(...)`, `locations_buckets_views_get(...)`, `locations_buckets_views_list(...)`, `locations_buckets_views_logs_list(...)`, `locations_buckets_views_patch(...)`, `locations_get(...)`, `locations_list(...)`, `locations_operations_cancel(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `logs_delete(...)`, `logs_list(...)`, `sinks_create(...)`, `sinks_delete(...)`, `sinks_get(...)`, `sinks_list(...)`, `sinks_patch(...)` and `sinks_update(...)`
/// // to build up your call.
/// let rb = hub.billing_accounts();
/// # }
/// ```
pub struct BillingAccountMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Logging<S>,
}

impl<'a, S> client::MethodsBuilder for BillingAccountMethods<'a, S> {}

impl<'a, S> BillingAccountMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new exclusion in the _Default sink in a specified parent resource. Only log entries belonging to that resource can be excluded. You can have up to 10 exclusions in a resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource in which to create the exclusion: "projects/[PROJECT_ID]" "organizations/[ORGANIZATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]" "folders/[FOLDER_ID]" For examples:"projects/my-logging-project" "organizations/123456789"
    pub fn exclusions_create(&self, request: LogExclusion, parent: &str) -> BillingAccountExclusionCreateCall<'a, S> {
        BillingAccountExclusionCreateCall {
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
    /// Deletes an exclusion in the _Default sink.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of an existing exclusion to delete: "projects/[PROJECT_ID]/exclusions/[EXCLUSION_ID]" "organizations/[ORGANIZATION_ID]/exclusions/[EXCLUSION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/exclusions/[EXCLUSION_ID]" "folders/[FOLDER_ID]/exclusions/[EXCLUSION_ID]" For example:"projects/my-project/exclusions/my-exclusion"
    pub fn exclusions_delete(&self, name: &str) -> BillingAccountExclusionDeleteCall<'a, S> {
        BillingAccountExclusionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the description of an exclusion in the _Default sink.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of an existing exclusion: "projects/[PROJECT_ID]/exclusions/[EXCLUSION_ID]" "organizations/[ORGANIZATION_ID]/exclusions/[EXCLUSION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/exclusions/[EXCLUSION_ID]" "folders/[FOLDER_ID]/exclusions/[EXCLUSION_ID]" For example:"projects/my-project/exclusions/my-exclusion"
    pub fn exclusions_get(&self, name: &str) -> BillingAccountExclusionGetCall<'a, S> {
        BillingAccountExclusionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the exclusions on the _Default sink in a parent resource.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource whose exclusions are to be listed. "projects/[PROJECT_ID]" "organizations/[ORGANIZATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]" "folders/[FOLDER_ID]" 
    pub fn exclusions_list(&self, parent: &str) -> BillingAccountExclusionListCall<'a, S> {
        BillingAccountExclusionListCall {
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
    /// Changes one or more properties of an existing exclusion in the _Default sink.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the exclusion to update: "projects/[PROJECT_ID]/exclusions/[EXCLUSION_ID]" "organizations/[ORGANIZATION_ID]/exclusions/[EXCLUSION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/exclusions/[EXCLUSION_ID]" "folders/[FOLDER_ID]/exclusions/[EXCLUSION_ID]" For example:"projects/my-project/exclusions/my-exclusion"
    pub fn exclusions_patch(&self, request: LogExclusion, name: &str) -> BillingAccountExclusionPatchCall<'a, S> {
        BillingAccountExclusionPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the logs in projects, organizations, folders, or billing accounts. Only logs that have entries are listed.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name that owns the logs: projects/[PROJECT_ID] organizations/[ORGANIZATION_ID] billingAccounts/[BILLING_ACCOUNT_ID] folders/[FOLDER_ID]
    pub fn locations_buckets_views_logs_list(&self, parent: &str) -> BillingAccountLocationBucketViewLogListCall<'a, S> {
        BillingAccountLocationBucketViewLogListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _resource_names: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a view over log entries in a log bucket. A bucket may contain a maximum of 30 views.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The bucket in which to create the view `"projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]"` For example:"projects/my-project/locations/global/buckets/my-bucket"
    pub fn locations_buckets_views_create(&self, request: LogView, parent: &str) -> BillingAccountLocationBucketViewCreateCall<'a, S> {
        BillingAccountLocationBucketViewCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _view_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a view on a log bucket. If an UNAVAILABLE error is returned, this indicates that system is not in a state where it can delete the view. If this occurs, please try again in a few minutes.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The full resource name of the view to delete: "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket/views/my-view"
    pub fn locations_buckets_views_delete(&self, name: &str) -> BillingAccountLocationBucketViewDeleteCall<'a, S> {
        BillingAccountLocationBucketViewDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a view on a log bucket..
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the policy: "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket/views/my-view"
    pub fn locations_buckets_views_get(&self, name: &str) -> BillingAccountLocationBucketViewGetCall<'a, S> {
        BillingAccountLocationBucketViewGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists views on a log bucket.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The bucket whose views are to be listed: "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" 
    pub fn locations_buckets_views_list(&self, parent: &str) -> BillingAccountLocationBucketViewListCall<'a, S> {
        BillingAccountLocationBucketViewListCall {
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
    /// Updates a view on a log bucket. This method replaces the following fields in the existing view with values from the new view: filter. If an UNAVAILABLE error is returned, this indicates that system is not in a state where it can update the view. If this occurs, please try again in a few minutes.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The full resource name of the view to update "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket/views/my-view"
    pub fn locations_buckets_views_patch(&self, request: LogView, name: &str) -> BillingAccountLocationBucketViewPatchCall<'a, S> {
        BillingAccountLocationBucketViewPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a log bucket that can be used to store log entries. After a bucket has been created, the bucket's location cannot be changed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource in which to create the log bucket: "projects/[PROJECT_ID]/locations/[LOCATION_ID]" For example:"projects/my-project/locations/global"
    pub fn locations_buckets_create(&self, request: LogBucket, parent: &str) -> BillingAccountLocationBucketCreateCall<'a, S> {
        BillingAccountLocationBucketCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _bucket_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a log bucket.Changes the bucket's lifecycle_state to the DELETE_REQUESTED state. After 7 days, the bucket will be purged and all log entries in the bucket will be permanently deleted.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The full resource name of the bucket to delete. "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket"
    pub fn locations_buckets_delete(&self, name: &str) -> BillingAccountLocationBucketDeleteCall<'a, S> {
        BillingAccountLocationBucketDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a log bucket.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the bucket: "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket"
    pub fn locations_buckets_get(&self, name: &str) -> BillingAccountLocationBucketGetCall<'a, S> {
        BillingAccountLocationBucketGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists log buckets.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource whose buckets are to be listed: "projects/[PROJECT_ID]/locations/[LOCATION_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]" Note: The locations portion of the resource must be specified, but supplying the character - in place of LOCATION_ID will return all buckets.
    pub fn locations_buckets_list(&self, parent: &str) -> BillingAccountLocationBucketListCall<'a, S> {
        BillingAccountLocationBucketListCall {
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
    /// Updates a log bucket. This method replaces the following fields in the existing bucket with values from the new bucket: retention_periodIf the retention period is decreased and the bucket is locked, FAILED_PRECONDITION will be returned.If the bucket has a lifecycle_state of DELETE_REQUESTED, then FAILED_PRECONDITION will be returned.After a bucket has been created, the bucket's location cannot be changed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The full resource name of the bucket to update. "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket"
    pub fn locations_buckets_patch(&self, request: LogBucket, name: &str) -> BillingAccountLocationBucketPatchCall<'a, S> {
        BillingAccountLocationBucketPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Undeletes a log bucket. A bucket that has been deleted can be undeleted within the grace period of 7 days.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The full resource name of the bucket to undelete. "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket"
    pub fn locations_buckets_undelete(&self, request: UndeleteBucketRequest, name: &str) -> BillingAccountLocationBucketUndeleteCall<'a, S> {
        BillingAccountLocationBucketUndeleteCall {
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
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn locations_operations_cancel(&self, request: CancelOperationRequest, name: &str) -> BillingAccountLocationOperationCancelCall<'a, S> {
        BillingAccountLocationOperationCancelCall {
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
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn locations_operations_get(&self, name: &str) -> BillingAccountLocationOperationGetCall<'a, S> {
        BillingAccountLocationOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns UNIMPLEMENTED.NOTE: the name binding allows API services to override the binding to use different resource name schemes, such as users/*/operations. To override the binding, API services can add a binding such as "/v1/{name=users/*}/operations" to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn locations_operations_list(&self, name: &str) -> BillingAccountLocationOperationListCall<'a, S> {
        BillingAccountLocationOperationListCall {
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
    /// Gets information about a location.
    /// 
    /// # Arguments
    ///
    /// * `name` - Resource name for the location.
    pub fn locations_get(&self, name: &str) -> BillingAccountLocationGetCall<'a, S> {
        BillingAccountLocationGetCall {
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
    pub fn locations_list(&self, name: &str) -> BillingAccountLocationListCall<'a, S> {
        BillingAccountLocationListCall {
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
    /// Deletes all the log entries in a log for the _Default Log Bucket. The log reappears if it receives new entries. Log entries written shortly before the delete operation might not be deleted. Entries received after the delete operation with a timestamp before the operation will be deleted.
    /// 
    /// # Arguments
    ///
    /// * `logName` - Required. The resource name of the log to delete: projects/[PROJECT_ID]/logs/[LOG_ID] organizations/[ORGANIZATION_ID]/logs/[LOG_ID] billingAccounts/[BILLING_ACCOUNT_ID]/logs/[LOG_ID] folders/[FOLDER_ID]/logs/[LOG_ID][LOG_ID] must be URL-encoded. For example, "projects/my-project-id/logs/syslog", "organizations/123/logs/cloudaudit.googleapis.com%2Factivity".For more information about log names, see LogEntry.
    pub fn logs_delete(&self, log_name: &str) -> BillingAccountLogDeleteCall<'a, S> {
        BillingAccountLogDeleteCall {
            hub: self.hub,
            _log_name: log_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the logs in projects, organizations, folders, or billing accounts. Only logs that have entries are listed.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name that owns the logs: projects/[PROJECT_ID] organizations/[ORGANIZATION_ID] billingAccounts/[BILLING_ACCOUNT_ID] folders/[FOLDER_ID]
    pub fn logs_list(&self, parent: &str) -> BillingAccountLogListCall<'a, S> {
        BillingAccountLogListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _resource_names: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
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
    /// * `parent` - Required. The resource in which to create the sink: "projects/[PROJECT_ID]" "organizations/[ORGANIZATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]" "folders/[FOLDER_ID]" For examples:"projects/my-project" "organizations/123456789"
    pub fn sinks_create(&self, request: LogSink, parent: &str) -> BillingAccountSinkCreateCall<'a, S> {
        BillingAccountSinkCreateCall {
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
    /// * `sinkName` - Required. The full resource name of the sink to delete, including the parent resource and the sink identifier: "projects/[PROJECT_ID]/sinks/[SINK_ID]" "organizations/[ORGANIZATION_ID]/sinks/[SINK_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_ID]" "folders/[FOLDER_ID]/sinks/[SINK_ID]" For example:"projects/my-project/sinks/my-sink"
    pub fn sinks_delete(&self, sink_name: &str) -> BillingAccountSinkDeleteCall<'a, S> {
        BillingAccountSinkDeleteCall {
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
    /// * `sinkName` - Required. The resource name of the sink: "projects/[PROJECT_ID]/sinks/[SINK_ID]" "organizations/[ORGANIZATION_ID]/sinks/[SINK_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_ID]" "folders/[FOLDER_ID]/sinks/[SINK_ID]" For example:"projects/my-project/sinks/my-sink"
    pub fn sinks_get(&self, sink_name: &str) -> BillingAccountSinkGetCall<'a, S> {
        BillingAccountSinkGetCall {
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
    /// * `parent` - Required. The parent resource whose sinks are to be listed: "projects/[PROJECT_ID]" "organizations/[ORGANIZATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]" "folders/[FOLDER_ID]" 
    pub fn sinks_list(&self, parent: &str) -> BillingAccountSinkListCall<'a, S> {
        BillingAccountSinkListCall {
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
    /// * `sinkName` - Required. The full resource name of the sink to update, including the parent resource and the sink identifier: "projects/[PROJECT_ID]/sinks/[SINK_ID]" "organizations/[ORGANIZATION_ID]/sinks/[SINK_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_ID]" "folders/[FOLDER_ID]/sinks/[SINK_ID]" For example:"projects/my-project/sinks/my-sink"
    pub fn sinks_patch(&self, request: LogSink, sink_name: &str) -> BillingAccountSinkPatchCall<'a, S> {
        BillingAccountSinkPatchCall {
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
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a sink. This method replaces the following fields in the existing sink with values from the new sink: destination, and filter.The updated sink might also have a new writer_identity; see the unique_writer_identity field.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `sinkName` - Required. The full resource name of the sink to update, including the parent resource and the sink identifier: "projects/[PROJECT_ID]/sinks/[SINK_ID]" "organizations/[ORGANIZATION_ID]/sinks/[SINK_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_ID]" "folders/[FOLDER_ID]/sinks/[SINK_ID]" For example:"projects/my-project/sinks/my-sink"
    pub fn sinks_update(&self, request: LogSink, sink_name: &str) -> BillingAccountSinkUpdateCall<'a, S> {
        BillingAccountSinkUpdateCall {
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
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the Logging CMEK settings for the given resource.Note: CMEK for the Log Router can be configured for Google Cloud projects, folders, organizations and billing accounts. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource for which to retrieve CMEK settings. "projects/[PROJECT_ID]/cmekSettings" "organizations/[ORGANIZATION_ID]/cmekSettings" "billingAccounts/[BILLING_ACCOUNT_ID]/cmekSettings" "folders/[FOLDER_ID]/cmekSettings" For example:"organizations/12345/cmekSettings"Note: CMEK for the Log Router can be configured for Google Cloud projects, folders, organizations and billing accounts. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.
    pub fn get_cmek_settings(&self, name: &str) -> BillingAccountGetCmekSettingCall<'a, S> {
        BillingAccountGetCmekSettingCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the Log Router settings for the given resource.Note: Settings for the Log Router can be get for Google Cloud projects, folders, organizations and billing accounts. Currently it can only be configured for organizations. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource for which to retrieve settings. "projects/[PROJECT_ID]/settings" "organizations/[ORGANIZATION_ID]/settings" "billingAccounts/[BILLING_ACCOUNT_ID]/settings" "folders/[FOLDER_ID]/settings" For example:"organizations/12345/settings"Note: Settings for the Log Router can be get for Google Cloud projects, folders, organizations and billing accounts. Currently it can only be configured for organizations. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.
    pub fn get_settings(&self, name: &str) -> BillingAccountGetSettingCall<'a, S> {
        BillingAccountGetSettingCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



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
/// extern crate google_logging2 as logging2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use logging2::{Logging, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Logging::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `copy(...)`, `list(...)`, `tail(...)` and `write(...)`
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
    /// Copies a set of log entries from a log bucket to a Cloud Storage bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn copy(&self, request: CopyLogEntriesRequest) -> EntryCopyCall<'a, S> {
        EntryCopyCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists log entries. Use this method to retrieve log entries that originated from a project/folder/organization/billing account. For ways to export log entries, see Exporting Logs (https://cloud.google.com/logging/docs/export).
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
    /// Streaming read of log entries as they are ingested. Until the stream is terminated, it will continue reading logs.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn tail(&self, request: TailLogEntriesRequest) -> EntryTailCall<'a, S> {
        EntryTailCall {
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



/// A builder providing access to all methods supported on *exclusion* resources.
/// It is not used directly, but through the [`Logging`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_logging2 as logging2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use logging2::{Logging, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Logging::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `list(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.exclusions();
/// # }
/// ```
pub struct ExclusionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Logging<S>,
}

impl<'a, S> client::MethodsBuilder for ExclusionMethods<'a, S> {}

impl<'a, S> ExclusionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new exclusion in the _Default sink in a specified parent resource. Only log entries belonging to that resource can be excluded. You can have up to 10 exclusions in a resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource in which to create the exclusion: "projects/[PROJECT_ID]" "organizations/[ORGANIZATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]" "folders/[FOLDER_ID]" For examples:"projects/my-logging-project" "organizations/123456789"
    pub fn create(&self, request: LogExclusion, parent: &str) -> ExclusionCreateCall<'a, S> {
        ExclusionCreateCall {
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
    /// Deletes an exclusion in the _Default sink.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of an existing exclusion to delete: "projects/[PROJECT_ID]/exclusions/[EXCLUSION_ID]" "organizations/[ORGANIZATION_ID]/exclusions/[EXCLUSION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/exclusions/[EXCLUSION_ID]" "folders/[FOLDER_ID]/exclusions/[EXCLUSION_ID]" For example:"projects/my-project/exclusions/my-exclusion"
    pub fn delete(&self, name: &str) -> ExclusionDeleteCall<'a, S> {
        ExclusionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the description of an exclusion in the _Default sink.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of an existing exclusion: "projects/[PROJECT_ID]/exclusions/[EXCLUSION_ID]" "organizations/[ORGANIZATION_ID]/exclusions/[EXCLUSION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/exclusions/[EXCLUSION_ID]" "folders/[FOLDER_ID]/exclusions/[EXCLUSION_ID]" For example:"projects/my-project/exclusions/my-exclusion"
    pub fn get(&self, name: &str) -> ExclusionGetCall<'a, S> {
        ExclusionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the exclusions on the _Default sink in a parent resource.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource whose exclusions are to be listed. "projects/[PROJECT_ID]" "organizations/[ORGANIZATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]" "folders/[FOLDER_ID]" 
    pub fn list(&self, parent: &str) -> ExclusionListCall<'a, S> {
        ExclusionListCall {
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
    /// Changes one or more properties of an existing exclusion in the _Default sink.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the exclusion to update: "projects/[PROJECT_ID]/exclusions/[EXCLUSION_ID]" "organizations/[ORGANIZATION_ID]/exclusions/[EXCLUSION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/exclusions/[EXCLUSION_ID]" "folders/[FOLDER_ID]/exclusions/[EXCLUSION_ID]" For example:"projects/my-project/exclusions/my-exclusion"
    pub fn patch(&self, request: LogExclusion, name: &str) -> ExclusionPatchCall<'a, S> {
        ExclusionPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *folder* resources.
/// It is not used directly, but through the [`Logging`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_logging2 as logging2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use logging2::{Logging, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Logging::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `exclusions_create(...)`, `exclusions_delete(...)`, `exclusions_get(...)`, `exclusions_list(...)`, `exclusions_patch(...)`, `get_cmek_settings(...)`, `get_settings(...)`, `locations_buckets_create(...)`, `locations_buckets_delete(...)`, `locations_buckets_get(...)`, `locations_buckets_list(...)`, `locations_buckets_patch(...)`, `locations_buckets_undelete(...)`, `locations_buckets_views_create(...)`, `locations_buckets_views_delete(...)`, `locations_buckets_views_get(...)`, `locations_buckets_views_list(...)`, `locations_buckets_views_logs_list(...)`, `locations_buckets_views_patch(...)`, `locations_get(...)`, `locations_list(...)`, `locations_operations_cancel(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `logs_delete(...)`, `logs_list(...)`, `sinks_create(...)`, `sinks_delete(...)`, `sinks_get(...)`, `sinks_list(...)`, `sinks_patch(...)`, `sinks_update(...)` and `update_settings(...)`
/// // to build up your call.
/// let rb = hub.folders();
/// # }
/// ```
pub struct FolderMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Logging<S>,
}

impl<'a, S> client::MethodsBuilder for FolderMethods<'a, S> {}

impl<'a, S> FolderMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new exclusion in the _Default sink in a specified parent resource. Only log entries belonging to that resource can be excluded. You can have up to 10 exclusions in a resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource in which to create the exclusion: "projects/[PROJECT_ID]" "organizations/[ORGANIZATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]" "folders/[FOLDER_ID]" For examples:"projects/my-logging-project" "organizations/123456789"
    pub fn exclusions_create(&self, request: LogExclusion, parent: &str) -> FolderExclusionCreateCall<'a, S> {
        FolderExclusionCreateCall {
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
    /// Deletes an exclusion in the _Default sink.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of an existing exclusion to delete: "projects/[PROJECT_ID]/exclusions/[EXCLUSION_ID]" "organizations/[ORGANIZATION_ID]/exclusions/[EXCLUSION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/exclusions/[EXCLUSION_ID]" "folders/[FOLDER_ID]/exclusions/[EXCLUSION_ID]" For example:"projects/my-project/exclusions/my-exclusion"
    pub fn exclusions_delete(&self, name: &str) -> FolderExclusionDeleteCall<'a, S> {
        FolderExclusionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the description of an exclusion in the _Default sink.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of an existing exclusion: "projects/[PROJECT_ID]/exclusions/[EXCLUSION_ID]" "organizations/[ORGANIZATION_ID]/exclusions/[EXCLUSION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/exclusions/[EXCLUSION_ID]" "folders/[FOLDER_ID]/exclusions/[EXCLUSION_ID]" For example:"projects/my-project/exclusions/my-exclusion"
    pub fn exclusions_get(&self, name: &str) -> FolderExclusionGetCall<'a, S> {
        FolderExclusionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the exclusions on the _Default sink in a parent resource.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource whose exclusions are to be listed. "projects/[PROJECT_ID]" "organizations/[ORGANIZATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]" "folders/[FOLDER_ID]" 
    pub fn exclusions_list(&self, parent: &str) -> FolderExclusionListCall<'a, S> {
        FolderExclusionListCall {
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
    /// Changes one or more properties of an existing exclusion in the _Default sink.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the exclusion to update: "projects/[PROJECT_ID]/exclusions/[EXCLUSION_ID]" "organizations/[ORGANIZATION_ID]/exclusions/[EXCLUSION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/exclusions/[EXCLUSION_ID]" "folders/[FOLDER_ID]/exclusions/[EXCLUSION_ID]" For example:"projects/my-project/exclusions/my-exclusion"
    pub fn exclusions_patch(&self, request: LogExclusion, name: &str) -> FolderExclusionPatchCall<'a, S> {
        FolderExclusionPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the logs in projects, organizations, folders, or billing accounts. Only logs that have entries are listed.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name that owns the logs: projects/[PROJECT_ID] organizations/[ORGANIZATION_ID] billingAccounts/[BILLING_ACCOUNT_ID] folders/[FOLDER_ID]
    pub fn locations_buckets_views_logs_list(&self, parent: &str) -> FolderLocationBucketViewLogListCall<'a, S> {
        FolderLocationBucketViewLogListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _resource_names: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a view over log entries in a log bucket. A bucket may contain a maximum of 30 views.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The bucket in which to create the view `"projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]"` For example:"projects/my-project/locations/global/buckets/my-bucket"
    pub fn locations_buckets_views_create(&self, request: LogView, parent: &str) -> FolderLocationBucketViewCreateCall<'a, S> {
        FolderLocationBucketViewCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _view_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a view on a log bucket. If an UNAVAILABLE error is returned, this indicates that system is not in a state where it can delete the view. If this occurs, please try again in a few minutes.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The full resource name of the view to delete: "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket/views/my-view"
    pub fn locations_buckets_views_delete(&self, name: &str) -> FolderLocationBucketViewDeleteCall<'a, S> {
        FolderLocationBucketViewDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a view on a log bucket..
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the policy: "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket/views/my-view"
    pub fn locations_buckets_views_get(&self, name: &str) -> FolderLocationBucketViewGetCall<'a, S> {
        FolderLocationBucketViewGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists views on a log bucket.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The bucket whose views are to be listed: "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" 
    pub fn locations_buckets_views_list(&self, parent: &str) -> FolderLocationBucketViewListCall<'a, S> {
        FolderLocationBucketViewListCall {
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
    /// Updates a view on a log bucket. This method replaces the following fields in the existing view with values from the new view: filter. If an UNAVAILABLE error is returned, this indicates that system is not in a state where it can update the view. If this occurs, please try again in a few minutes.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The full resource name of the view to update "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket/views/my-view"
    pub fn locations_buckets_views_patch(&self, request: LogView, name: &str) -> FolderLocationBucketViewPatchCall<'a, S> {
        FolderLocationBucketViewPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a log bucket that can be used to store log entries. After a bucket has been created, the bucket's location cannot be changed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource in which to create the log bucket: "projects/[PROJECT_ID]/locations/[LOCATION_ID]" For example:"projects/my-project/locations/global"
    pub fn locations_buckets_create(&self, request: LogBucket, parent: &str) -> FolderLocationBucketCreateCall<'a, S> {
        FolderLocationBucketCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _bucket_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a log bucket.Changes the bucket's lifecycle_state to the DELETE_REQUESTED state. After 7 days, the bucket will be purged and all log entries in the bucket will be permanently deleted.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The full resource name of the bucket to delete. "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket"
    pub fn locations_buckets_delete(&self, name: &str) -> FolderLocationBucketDeleteCall<'a, S> {
        FolderLocationBucketDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a log bucket.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the bucket: "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket"
    pub fn locations_buckets_get(&self, name: &str) -> FolderLocationBucketGetCall<'a, S> {
        FolderLocationBucketGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists log buckets.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource whose buckets are to be listed: "projects/[PROJECT_ID]/locations/[LOCATION_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]" Note: The locations portion of the resource must be specified, but supplying the character - in place of LOCATION_ID will return all buckets.
    pub fn locations_buckets_list(&self, parent: &str) -> FolderLocationBucketListCall<'a, S> {
        FolderLocationBucketListCall {
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
    /// Updates a log bucket. This method replaces the following fields in the existing bucket with values from the new bucket: retention_periodIf the retention period is decreased and the bucket is locked, FAILED_PRECONDITION will be returned.If the bucket has a lifecycle_state of DELETE_REQUESTED, then FAILED_PRECONDITION will be returned.After a bucket has been created, the bucket's location cannot be changed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The full resource name of the bucket to update. "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket"
    pub fn locations_buckets_patch(&self, request: LogBucket, name: &str) -> FolderLocationBucketPatchCall<'a, S> {
        FolderLocationBucketPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Undeletes a log bucket. A bucket that has been deleted can be undeleted within the grace period of 7 days.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The full resource name of the bucket to undelete. "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket"
    pub fn locations_buckets_undelete(&self, request: UndeleteBucketRequest, name: &str) -> FolderLocationBucketUndeleteCall<'a, S> {
        FolderLocationBucketUndeleteCall {
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
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn locations_operations_cancel(&self, request: CancelOperationRequest, name: &str) -> FolderLocationOperationCancelCall<'a, S> {
        FolderLocationOperationCancelCall {
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
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn locations_operations_get(&self, name: &str) -> FolderLocationOperationGetCall<'a, S> {
        FolderLocationOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns UNIMPLEMENTED.NOTE: the name binding allows API services to override the binding to use different resource name schemes, such as users/*/operations. To override the binding, API services can add a binding such as "/v1/{name=users/*}/operations" to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn locations_operations_list(&self, name: &str) -> FolderLocationOperationListCall<'a, S> {
        FolderLocationOperationListCall {
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
    /// Gets information about a location.
    /// 
    /// # Arguments
    ///
    /// * `name` - Resource name for the location.
    pub fn locations_get(&self, name: &str) -> FolderLocationGetCall<'a, S> {
        FolderLocationGetCall {
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
    pub fn locations_list(&self, name: &str) -> FolderLocationListCall<'a, S> {
        FolderLocationListCall {
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
    /// Deletes all the log entries in a log for the _Default Log Bucket. The log reappears if it receives new entries. Log entries written shortly before the delete operation might not be deleted. Entries received after the delete operation with a timestamp before the operation will be deleted.
    /// 
    /// # Arguments
    ///
    /// * `logName` - Required. The resource name of the log to delete: projects/[PROJECT_ID]/logs/[LOG_ID] organizations/[ORGANIZATION_ID]/logs/[LOG_ID] billingAccounts/[BILLING_ACCOUNT_ID]/logs/[LOG_ID] folders/[FOLDER_ID]/logs/[LOG_ID][LOG_ID] must be URL-encoded. For example, "projects/my-project-id/logs/syslog", "organizations/123/logs/cloudaudit.googleapis.com%2Factivity".For more information about log names, see LogEntry.
    pub fn logs_delete(&self, log_name: &str) -> FolderLogDeleteCall<'a, S> {
        FolderLogDeleteCall {
            hub: self.hub,
            _log_name: log_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the logs in projects, organizations, folders, or billing accounts. Only logs that have entries are listed.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name that owns the logs: projects/[PROJECT_ID] organizations/[ORGANIZATION_ID] billingAccounts/[BILLING_ACCOUNT_ID] folders/[FOLDER_ID]
    pub fn logs_list(&self, parent: &str) -> FolderLogListCall<'a, S> {
        FolderLogListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _resource_names: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
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
    /// * `parent` - Required. The resource in which to create the sink: "projects/[PROJECT_ID]" "organizations/[ORGANIZATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]" "folders/[FOLDER_ID]" For examples:"projects/my-project" "organizations/123456789"
    pub fn sinks_create(&self, request: LogSink, parent: &str) -> FolderSinkCreateCall<'a, S> {
        FolderSinkCreateCall {
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
    /// * `sinkName` - Required. The full resource name of the sink to delete, including the parent resource and the sink identifier: "projects/[PROJECT_ID]/sinks/[SINK_ID]" "organizations/[ORGANIZATION_ID]/sinks/[SINK_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_ID]" "folders/[FOLDER_ID]/sinks/[SINK_ID]" For example:"projects/my-project/sinks/my-sink"
    pub fn sinks_delete(&self, sink_name: &str) -> FolderSinkDeleteCall<'a, S> {
        FolderSinkDeleteCall {
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
    /// * `sinkName` - Required. The resource name of the sink: "projects/[PROJECT_ID]/sinks/[SINK_ID]" "organizations/[ORGANIZATION_ID]/sinks/[SINK_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_ID]" "folders/[FOLDER_ID]/sinks/[SINK_ID]" For example:"projects/my-project/sinks/my-sink"
    pub fn sinks_get(&self, sink_name: &str) -> FolderSinkGetCall<'a, S> {
        FolderSinkGetCall {
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
    /// * `parent` - Required. The parent resource whose sinks are to be listed: "projects/[PROJECT_ID]" "organizations/[ORGANIZATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]" "folders/[FOLDER_ID]" 
    pub fn sinks_list(&self, parent: &str) -> FolderSinkListCall<'a, S> {
        FolderSinkListCall {
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
    /// * `sinkName` - Required. The full resource name of the sink to update, including the parent resource and the sink identifier: "projects/[PROJECT_ID]/sinks/[SINK_ID]" "organizations/[ORGANIZATION_ID]/sinks/[SINK_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_ID]" "folders/[FOLDER_ID]/sinks/[SINK_ID]" For example:"projects/my-project/sinks/my-sink"
    pub fn sinks_patch(&self, request: LogSink, sink_name: &str) -> FolderSinkPatchCall<'a, S> {
        FolderSinkPatchCall {
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
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a sink. This method replaces the following fields in the existing sink with values from the new sink: destination, and filter.The updated sink might also have a new writer_identity; see the unique_writer_identity field.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `sinkName` - Required. The full resource name of the sink to update, including the parent resource and the sink identifier: "projects/[PROJECT_ID]/sinks/[SINK_ID]" "organizations/[ORGANIZATION_ID]/sinks/[SINK_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_ID]" "folders/[FOLDER_ID]/sinks/[SINK_ID]" For example:"projects/my-project/sinks/my-sink"
    pub fn sinks_update(&self, request: LogSink, sink_name: &str) -> FolderSinkUpdateCall<'a, S> {
        FolderSinkUpdateCall {
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
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the Logging CMEK settings for the given resource.Note: CMEK for the Log Router can be configured for Google Cloud projects, folders, organizations and billing accounts. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource for which to retrieve CMEK settings. "projects/[PROJECT_ID]/cmekSettings" "organizations/[ORGANIZATION_ID]/cmekSettings" "billingAccounts/[BILLING_ACCOUNT_ID]/cmekSettings" "folders/[FOLDER_ID]/cmekSettings" For example:"organizations/12345/cmekSettings"Note: CMEK for the Log Router can be configured for Google Cloud projects, folders, organizations and billing accounts. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.
    pub fn get_cmek_settings(&self, name: &str) -> FolderGetCmekSettingCall<'a, S> {
        FolderGetCmekSettingCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the Log Router settings for the given resource.Note: Settings for the Log Router can be get for Google Cloud projects, folders, organizations and billing accounts. Currently it can only be configured for organizations. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource for which to retrieve settings. "projects/[PROJECT_ID]/settings" "organizations/[ORGANIZATION_ID]/settings" "billingAccounts/[BILLING_ACCOUNT_ID]/settings" "folders/[FOLDER_ID]/settings" For example:"organizations/12345/settings"Note: Settings for the Log Router can be get for Google Cloud projects, folders, organizations and billing accounts. Currently it can only be configured for organizations. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.
    pub fn get_settings(&self, name: &str) -> FolderGetSettingCall<'a, S> {
        FolderGetSettingCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the Log Router settings for the given resource.Note: Settings for the Log Router can currently only be configured for Google Cloud organizations. Once configured, it applies to all projects and folders in the Google Cloud organization.UpdateSettings will fail if 1) kms_key_name is invalid, or 2) the associated service account does not have the required roles/cloudkms.cryptoKeyEncrypterDecrypter role assigned for the key, or 3) access to the key is disabled. 4) location_id is not supported by Logging. 5) location_id violate OrgPolicy.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name for the settings to update. "organizations/[ORGANIZATION_ID]/settings" For example:"organizations/12345/settings"Note: Settings for the Log Router can currently only be configured for Google Cloud organizations. Once configured, it applies to all projects and folders in the Google Cloud organization.
    pub fn update_settings(&self, request: Settings, name: &str) -> FolderUpdateSettingCall<'a, S> {
        FolderUpdateSettingCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *location* resources.
/// It is not used directly, but through the [`Logging`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_logging2 as logging2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use logging2::{Logging, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Logging::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `buckets_create(...)`, `buckets_delete(...)`, `buckets_get(...)`, `buckets_list(...)`, `buckets_patch(...)`, `buckets_undelete(...)`, `buckets_views_create(...)`, `buckets_views_delete(...)`, `buckets_views_get(...)`, `buckets_views_list(...)`, `buckets_views_patch(...)`, `get(...)`, `list(...)`, `operations_cancel(...)`, `operations_get(...)` and `operations_list(...)`
/// // to build up your call.
/// let rb = hub.locations();
/// # }
/// ```
pub struct LocationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Logging<S>,
}

impl<'a, S> client::MethodsBuilder for LocationMethods<'a, S> {}

impl<'a, S> LocationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a view over log entries in a log bucket. A bucket may contain a maximum of 30 views.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The bucket in which to create the view `"projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]"` For example:"projects/my-project/locations/global/buckets/my-bucket"
    pub fn buckets_views_create(&self, request: LogView, parent: &str) -> LocationBucketViewCreateCall<'a, S> {
        LocationBucketViewCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _view_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a view on a log bucket. If an UNAVAILABLE error is returned, this indicates that system is not in a state where it can delete the view. If this occurs, please try again in a few minutes.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The full resource name of the view to delete: "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket/views/my-view"
    pub fn buckets_views_delete(&self, name: &str) -> LocationBucketViewDeleteCall<'a, S> {
        LocationBucketViewDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a view on a log bucket..
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the policy: "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket/views/my-view"
    pub fn buckets_views_get(&self, name: &str) -> LocationBucketViewGetCall<'a, S> {
        LocationBucketViewGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists views on a log bucket.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The bucket whose views are to be listed: "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" 
    pub fn buckets_views_list(&self, parent: &str) -> LocationBucketViewListCall<'a, S> {
        LocationBucketViewListCall {
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
    /// Updates a view on a log bucket. This method replaces the following fields in the existing view with values from the new view: filter. If an UNAVAILABLE error is returned, this indicates that system is not in a state where it can update the view. If this occurs, please try again in a few minutes.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The full resource name of the view to update "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket/views/my-view"
    pub fn buckets_views_patch(&self, request: LogView, name: &str) -> LocationBucketViewPatchCall<'a, S> {
        LocationBucketViewPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a log bucket that can be used to store log entries. After a bucket has been created, the bucket's location cannot be changed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource in which to create the log bucket: "projects/[PROJECT_ID]/locations/[LOCATION_ID]" For example:"projects/my-project/locations/global"
    pub fn buckets_create(&self, request: LogBucket, parent: &str) -> LocationBucketCreateCall<'a, S> {
        LocationBucketCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _bucket_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a log bucket.Changes the bucket's lifecycle_state to the DELETE_REQUESTED state. After 7 days, the bucket will be purged and all log entries in the bucket will be permanently deleted.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The full resource name of the bucket to delete. "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket"
    pub fn buckets_delete(&self, name: &str) -> LocationBucketDeleteCall<'a, S> {
        LocationBucketDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a log bucket.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the bucket: "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket"
    pub fn buckets_get(&self, name: &str) -> LocationBucketGetCall<'a, S> {
        LocationBucketGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists log buckets.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource whose buckets are to be listed: "projects/[PROJECT_ID]/locations/[LOCATION_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]" Note: The locations portion of the resource must be specified, but supplying the character - in place of LOCATION_ID will return all buckets.
    pub fn buckets_list(&self, parent: &str) -> LocationBucketListCall<'a, S> {
        LocationBucketListCall {
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
    /// Updates a log bucket. This method replaces the following fields in the existing bucket with values from the new bucket: retention_periodIf the retention period is decreased and the bucket is locked, FAILED_PRECONDITION will be returned.If the bucket has a lifecycle_state of DELETE_REQUESTED, then FAILED_PRECONDITION will be returned.After a bucket has been created, the bucket's location cannot be changed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The full resource name of the bucket to update. "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket"
    pub fn buckets_patch(&self, request: LogBucket, name: &str) -> LocationBucketPatchCall<'a, S> {
        LocationBucketPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Undeletes a log bucket. A bucket that has been deleted can be undeleted within the grace period of 7 days.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The full resource name of the bucket to undelete. "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket"
    pub fn buckets_undelete(&self, request: UndeleteBucketRequest, name: &str) -> LocationBucketUndeleteCall<'a, S> {
        LocationBucketUndeleteCall {
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
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn operations_cancel(&self, request: CancelOperationRequest, name: &str) -> LocationOperationCancelCall<'a, S> {
        LocationOperationCancelCall {
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
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn operations_get(&self, name: &str) -> LocationOperationGetCall<'a, S> {
        LocationOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns UNIMPLEMENTED.NOTE: the name binding allows API services to override the binding to use different resource name schemes, such as users/*/operations. To override the binding, API services can add a binding such as "/v1/{name=users/*}/operations" to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn operations_list(&self, name: &str) -> LocationOperationListCall<'a, S> {
        LocationOperationListCall {
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
    /// Gets information about a location.
    /// 
    /// # Arguments
    ///
    /// * `name` - Resource name for the location.
    pub fn get(&self, name: &str) -> LocationGetCall<'a, S> {
        LocationGetCall {
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
    pub fn list(&self, name: &str) -> LocationListCall<'a, S> {
        LocationListCall {
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
}



/// A builder providing access to all methods supported on *log* resources.
/// It is not used directly, but through the [`Logging`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_logging2 as logging2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use logging2::{Logging, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Logging::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.logs();
/// # }
/// ```
pub struct LogMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Logging<S>,
}

impl<'a, S> client::MethodsBuilder for LogMethods<'a, S> {}

impl<'a, S> LogMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes all the log entries in a log for the _Default Log Bucket. The log reappears if it receives new entries. Log entries written shortly before the delete operation might not be deleted. Entries received after the delete operation with a timestamp before the operation will be deleted.
    /// 
    /// # Arguments
    ///
    /// * `logName` - Required. The resource name of the log to delete: projects/[PROJECT_ID]/logs/[LOG_ID] organizations/[ORGANIZATION_ID]/logs/[LOG_ID] billingAccounts/[BILLING_ACCOUNT_ID]/logs/[LOG_ID] folders/[FOLDER_ID]/logs/[LOG_ID][LOG_ID] must be URL-encoded. For example, "projects/my-project-id/logs/syslog", "organizations/123/logs/cloudaudit.googleapis.com%2Factivity".For more information about log names, see LogEntry.
    pub fn delete(&self, log_name: &str) -> LogDeleteCall<'a, S> {
        LogDeleteCall {
            hub: self.hub,
            _log_name: log_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the logs in projects, organizations, folders, or billing accounts. Only logs that have entries are listed.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name that owns the logs: projects/[PROJECT_ID] organizations/[ORGANIZATION_ID] billingAccounts/[BILLING_ACCOUNT_ID] folders/[FOLDER_ID]
    pub fn list(&self, parent: &str) -> LogListCall<'a, S> {
        LogListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _resource_names: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
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
/// extern crate google_logging2 as logging2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use logging2::{Logging, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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



/// A builder providing access to all methods supported on *organization* resources.
/// It is not used directly, but through the [`Logging`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_logging2 as logging2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use logging2::{Logging, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Logging::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `exclusions_create(...)`, `exclusions_delete(...)`, `exclusions_get(...)`, `exclusions_list(...)`, `exclusions_patch(...)`, `get_cmek_settings(...)`, `get_settings(...)`, `locations_buckets_create(...)`, `locations_buckets_delete(...)`, `locations_buckets_get(...)`, `locations_buckets_list(...)`, `locations_buckets_patch(...)`, `locations_buckets_undelete(...)`, `locations_buckets_views_create(...)`, `locations_buckets_views_delete(...)`, `locations_buckets_views_get(...)`, `locations_buckets_views_list(...)`, `locations_buckets_views_logs_list(...)`, `locations_buckets_views_patch(...)`, `locations_get(...)`, `locations_list(...)`, `locations_operations_cancel(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `logs_delete(...)`, `logs_list(...)`, `sinks_create(...)`, `sinks_delete(...)`, `sinks_get(...)`, `sinks_list(...)`, `sinks_patch(...)`, `sinks_update(...)`, `update_cmek_settings(...)` and `update_settings(...)`
/// // to build up your call.
/// let rb = hub.organizations();
/// # }
/// ```
pub struct OrganizationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Logging<S>,
}

impl<'a, S> client::MethodsBuilder for OrganizationMethods<'a, S> {}

impl<'a, S> OrganizationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new exclusion in the _Default sink in a specified parent resource. Only log entries belonging to that resource can be excluded. You can have up to 10 exclusions in a resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource in which to create the exclusion: "projects/[PROJECT_ID]" "organizations/[ORGANIZATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]" "folders/[FOLDER_ID]" For examples:"projects/my-logging-project" "organizations/123456789"
    pub fn exclusions_create(&self, request: LogExclusion, parent: &str) -> OrganizationExclusionCreateCall<'a, S> {
        OrganizationExclusionCreateCall {
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
    /// Deletes an exclusion in the _Default sink.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of an existing exclusion to delete: "projects/[PROJECT_ID]/exclusions/[EXCLUSION_ID]" "organizations/[ORGANIZATION_ID]/exclusions/[EXCLUSION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/exclusions/[EXCLUSION_ID]" "folders/[FOLDER_ID]/exclusions/[EXCLUSION_ID]" For example:"projects/my-project/exclusions/my-exclusion"
    pub fn exclusions_delete(&self, name: &str) -> OrganizationExclusionDeleteCall<'a, S> {
        OrganizationExclusionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the description of an exclusion in the _Default sink.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of an existing exclusion: "projects/[PROJECT_ID]/exclusions/[EXCLUSION_ID]" "organizations/[ORGANIZATION_ID]/exclusions/[EXCLUSION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/exclusions/[EXCLUSION_ID]" "folders/[FOLDER_ID]/exclusions/[EXCLUSION_ID]" For example:"projects/my-project/exclusions/my-exclusion"
    pub fn exclusions_get(&self, name: &str) -> OrganizationExclusionGetCall<'a, S> {
        OrganizationExclusionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the exclusions on the _Default sink in a parent resource.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource whose exclusions are to be listed. "projects/[PROJECT_ID]" "organizations/[ORGANIZATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]" "folders/[FOLDER_ID]" 
    pub fn exclusions_list(&self, parent: &str) -> OrganizationExclusionListCall<'a, S> {
        OrganizationExclusionListCall {
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
    /// Changes one or more properties of an existing exclusion in the _Default sink.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the exclusion to update: "projects/[PROJECT_ID]/exclusions/[EXCLUSION_ID]" "organizations/[ORGANIZATION_ID]/exclusions/[EXCLUSION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/exclusions/[EXCLUSION_ID]" "folders/[FOLDER_ID]/exclusions/[EXCLUSION_ID]" For example:"projects/my-project/exclusions/my-exclusion"
    pub fn exclusions_patch(&self, request: LogExclusion, name: &str) -> OrganizationExclusionPatchCall<'a, S> {
        OrganizationExclusionPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the logs in projects, organizations, folders, or billing accounts. Only logs that have entries are listed.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name that owns the logs: projects/[PROJECT_ID] organizations/[ORGANIZATION_ID] billingAccounts/[BILLING_ACCOUNT_ID] folders/[FOLDER_ID]
    pub fn locations_buckets_views_logs_list(&self, parent: &str) -> OrganizationLocationBucketViewLogListCall<'a, S> {
        OrganizationLocationBucketViewLogListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _resource_names: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a view over log entries in a log bucket. A bucket may contain a maximum of 30 views.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The bucket in which to create the view `"projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]"` For example:"projects/my-project/locations/global/buckets/my-bucket"
    pub fn locations_buckets_views_create(&self, request: LogView, parent: &str) -> OrganizationLocationBucketViewCreateCall<'a, S> {
        OrganizationLocationBucketViewCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _view_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a view on a log bucket. If an UNAVAILABLE error is returned, this indicates that system is not in a state where it can delete the view. If this occurs, please try again in a few minutes.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The full resource name of the view to delete: "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket/views/my-view"
    pub fn locations_buckets_views_delete(&self, name: &str) -> OrganizationLocationBucketViewDeleteCall<'a, S> {
        OrganizationLocationBucketViewDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a view on a log bucket..
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the policy: "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket/views/my-view"
    pub fn locations_buckets_views_get(&self, name: &str) -> OrganizationLocationBucketViewGetCall<'a, S> {
        OrganizationLocationBucketViewGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists views on a log bucket.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The bucket whose views are to be listed: "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" 
    pub fn locations_buckets_views_list(&self, parent: &str) -> OrganizationLocationBucketViewListCall<'a, S> {
        OrganizationLocationBucketViewListCall {
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
    /// Updates a view on a log bucket. This method replaces the following fields in the existing view with values from the new view: filter. If an UNAVAILABLE error is returned, this indicates that system is not in a state where it can update the view. If this occurs, please try again in a few minutes.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The full resource name of the view to update "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket/views/my-view"
    pub fn locations_buckets_views_patch(&self, request: LogView, name: &str) -> OrganizationLocationBucketViewPatchCall<'a, S> {
        OrganizationLocationBucketViewPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a log bucket that can be used to store log entries. After a bucket has been created, the bucket's location cannot be changed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource in which to create the log bucket: "projects/[PROJECT_ID]/locations/[LOCATION_ID]" For example:"projects/my-project/locations/global"
    pub fn locations_buckets_create(&self, request: LogBucket, parent: &str) -> OrganizationLocationBucketCreateCall<'a, S> {
        OrganizationLocationBucketCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _bucket_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a log bucket.Changes the bucket's lifecycle_state to the DELETE_REQUESTED state. After 7 days, the bucket will be purged and all log entries in the bucket will be permanently deleted.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The full resource name of the bucket to delete. "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket"
    pub fn locations_buckets_delete(&self, name: &str) -> OrganizationLocationBucketDeleteCall<'a, S> {
        OrganizationLocationBucketDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a log bucket.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the bucket: "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket"
    pub fn locations_buckets_get(&self, name: &str) -> OrganizationLocationBucketGetCall<'a, S> {
        OrganizationLocationBucketGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists log buckets.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource whose buckets are to be listed: "projects/[PROJECT_ID]/locations/[LOCATION_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]" Note: The locations portion of the resource must be specified, but supplying the character - in place of LOCATION_ID will return all buckets.
    pub fn locations_buckets_list(&self, parent: &str) -> OrganizationLocationBucketListCall<'a, S> {
        OrganizationLocationBucketListCall {
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
    /// Updates a log bucket. This method replaces the following fields in the existing bucket with values from the new bucket: retention_periodIf the retention period is decreased and the bucket is locked, FAILED_PRECONDITION will be returned.If the bucket has a lifecycle_state of DELETE_REQUESTED, then FAILED_PRECONDITION will be returned.After a bucket has been created, the bucket's location cannot be changed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The full resource name of the bucket to update. "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket"
    pub fn locations_buckets_patch(&self, request: LogBucket, name: &str) -> OrganizationLocationBucketPatchCall<'a, S> {
        OrganizationLocationBucketPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Undeletes a log bucket. A bucket that has been deleted can be undeleted within the grace period of 7 days.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The full resource name of the bucket to undelete. "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket"
    pub fn locations_buckets_undelete(&self, request: UndeleteBucketRequest, name: &str) -> OrganizationLocationBucketUndeleteCall<'a, S> {
        OrganizationLocationBucketUndeleteCall {
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
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn locations_operations_cancel(&self, request: CancelOperationRequest, name: &str) -> OrganizationLocationOperationCancelCall<'a, S> {
        OrganizationLocationOperationCancelCall {
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
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn locations_operations_get(&self, name: &str) -> OrganizationLocationOperationGetCall<'a, S> {
        OrganizationLocationOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns UNIMPLEMENTED.NOTE: the name binding allows API services to override the binding to use different resource name schemes, such as users/*/operations. To override the binding, API services can add a binding such as "/v1/{name=users/*}/operations" to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn locations_operations_list(&self, name: &str) -> OrganizationLocationOperationListCall<'a, S> {
        OrganizationLocationOperationListCall {
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
    /// Gets information about a location.
    /// 
    /// # Arguments
    ///
    /// * `name` - Resource name for the location.
    pub fn locations_get(&self, name: &str) -> OrganizationLocationGetCall<'a, S> {
        OrganizationLocationGetCall {
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
    pub fn locations_list(&self, name: &str) -> OrganizationLocationListCall<'a, S> {
        OrganizationLocationListCall {
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
    /// Deletes all the log entries in a log for the _Default Log Bucket. The log reappears if it receives new entries. Log entries written shortly before the delete operation might not be deleted. Entries received after the delete operation with a timestamp before the operation will be deleted.
    /// 
    /// # Arguments
    ///
    /// * `logName` - Required. The resource name of the log to delete: projects/[PROJECT_ID]/logs/[LOG_ID] organizations/[ORGANIZATION_ID]/logs/[LOG_ID] billingAccounts/[BILLING_ACCOUNT_ID]/logs/[LOG_ID] folders/[FOLDER_ID]/logs/[LOG_ID][LOG_ID] must be URL-encoded. For example, "projects/my-project-id/logs/syslog", "organizations/123/logs/cloudaudit.googleapis.com%2Factivity".For more information about log names, see LogEntry.
    pub fn logs_delete(&self, log_name: &str) -> OrganizationLogDeleteCall<'a, S> {
        OrganizationLogDeleteCall {
            hub: self.hub,
            _log_name: log_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the logs in projects, organizations, folders, or billing accounts. Only logs that have entries are listed.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name that owns the logs: projects/[PROJECT_ID] organizations/[ORGANIZATION_ID] billingAccounts/[BILLING_ACCOUNT_ID] folders/[FOLDER_ID]
    pub fn logs_list(&self, parent: &str) -> OrganizationLogListCall<'a, S> {
        OrganizationLogListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _resource_names: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
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
    /// * `parent` - Required. The resource in which to create the sink: "projects/[PROJECT_ID]" "organizations/[ORGANIZATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]" "folders/[FOLDER_ID]" For examples:"projects/my-project" "organizations/123456789"
    pub fn sinks_create(&self, request: LogSink, parent: &str) -> OrganizationSinkCreateCall<'a, S> {
        OrganizationSinkCreateCall {
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
    /// * `sinkName` - Required. The full resource name of the sink to delete, including the parent resource and the sink identifier: "projects/[PROJECT_ID]/sinks/[SINK_ID]" "organizations/[ORGANIZATION_ID]/sinks/[SINK_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_ID]" "folders/[FOLDER_ID]/sinks/[SINK_ID]" For example:"projects/my-project/sinks/my-sink"
    pub fn sinks_delete(&self, sink_name: &str) -> OrganizationSinkDeleteCall<'a, S> {
        OrganizationSinkDeleteCall {
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
    /// * `sinkName` - Required. The resource name of the sink: "projects/[PROJECT_ID]/sinks/[SINK_ID]" "organizations/[ORGANIZATION_ID]/sinks/[SINK_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_ID]" "folders/[FOLDER_ID]/sinks/[SINK_ID]" For example:"projects/my-project/sinks/my-sink"
    pub fn sinks_get(&self, sink_name: &str) -> OrganizationSinkGetCall<'a, S> {
        OrganizationSinkGetCall {
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
    /// * `parent` - Required. The parent resource whose sinks are to be listed: "projects/[PROJECT_ID]" "organizations/[ORGANIZATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]" "folders/[FOLDER_ID]" 
    pub fn sinks_list(&self, parent: &str) -> OrganizationSinkListCall<'a, S> {
        OrganizationSinkListCall {
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
    /// * `sinkName` - Required. The full resource name of the sink to update, including the parent resource and the sink identifier: "projects/[PROJECT_ID]/sinks/[SINK_ID]" "organizations/[ORGANIZATION_ID]/sinks/[SINK_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_ID]" "folders/[FOLDER_ID]/sinks/[SINK_ID]" For example:"projects/my-project/sinks/my-sink"
    pub fn sinks_patch(&self, request: LogSink, sink_name: &str) -> OrganizationSinkPatchCall<'a, S> {
        OrganizationSinkPatchCall {
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
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a sink. This method replaces the following fields in the existing sink with values from the new sink: destination, and filter.The updated sink might also have a new writer_identity; see the unique_writer_identity field.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `sinkName` - Required. The full resource name of the sink to update, including the parent resource and the sink identifier: "projects/[PROJECT_ID]/sinks/[SINK_ID]" "organizations/[ORGANIZATION_ID]/sinks/[SINK_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_ID]" "folders/[FOLDER_ID]/sinks/[SINK_ID]" For example:"projects/my-project/sinks/my-sink"
    pub fn sinks_update(&self, request: LogSink, sink_name: &str) -> OrganizationSinkUpdateCall<'a, S> {
        OrganizationSinkUpdateCall {
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
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the Logging CMEK settings for the given resource.Note: CMEK for the Log Router can be configured for Google Cloud projects, folders, organizations and billing accounts. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource for which to retrieve CMEK settings. "projects/[PROJECT_ID]/cmekSettings" "organizations/[ORGANIZATION_ID]/cmekSettings" "billingAccounts/[BILLING_ACCOUNT_ID]/cmekSettings" "folders/[FOLDER_ID]/cmekSettings" For example:"organizations/12345/cmekSettings"Note: CMEK for the Log Router can be configured for Google Cloud projects, folders, organizations and billing accounts. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.
    pub fn get_cmek_settings(&self, name: &str) -> OrganizationGetCmekSettingCall<'a, S> {
        OrganizationGetCmekSettingCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the Log Router settings for the given resource.Note: Settings for the Log Router can be get for Google Cloud projects, folders, organizations and billing accounts. Currently it can only be configured for organizations. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource for which to retrieve settings. "projects/[PROJECT_ID]/settings" "organizations/[ORGANIZATION_ID]/settings" "billingAccounts/[BILLING_ACCOUNT_ID]/settings" "folders/[FOLDER_ID]/settings" For example:"organizations/12345/settings"Note: Settings for the Log Router can be get for Google Cloud projects, folders, organizations and billing accounts. Currently it can only be configured for organizations. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.
    pub fn get_settings(&self, name: &str) -> OrganizationGetSettingCall<'a, S> {
        OrganizationGetSettingCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the Log Router CMEK settings for the given resource.Note: CMEK for the Log Router can currently only be configured for Google Cloud organizations. Once configured, it applies to all projects and folders in the Google Cloud organization.UpdateCmekSettings will fail if 1) kms_key_name is invalid, or 2) the associated service account does not have the required roles/cloudkms.cryptoKeyEncrypterDecrypter role assigned for the key, or 3) access to the key is disabled.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name for the CMEK settings to update. "projects/[PROJECT_ID]/cmekSettings" "organizations/[ORGANIZATION_ID]/cmekSettings" "billingAccounts/[BILLING_ACCOUNT_ID]/cmekSettings" "folders/[FOLDER_ID]/cmekSettings" For example:"organizations/12345/cmekSettings"Note: CMEK for the Log Router can currently only be configured for Google Cloud organizations. Once configured, it applies to all projects and folders in the Google Cloud organization.
    pub fn update_cmek_settings(&self, request: CmekSettings, name: &str) -> OrganizationUpdateCmekSettingCall<'a, S> {
        OrganizationUpdateCmekSettingCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the Log Router settings for the given resource.Note: Settings for the Log Router can currently only be configured for Google Cloud organizations. Once configured, it applies to all projects and folders in the Google Cloud organization.UpdateSettings will fail if 1) kms_key_name is invalid, or 2) the associated service account does not have the required roles/cloudkms.cryptoKeyEncrypterDecrypter role assigned for the key, or 3) access to the key is disabled. 4) location_id is not supported by Logging. 5) location_id violate OrgPolicy.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name for the settings to update. "organizations/[ORGANIZATION_ID]/settings" For example:"organizations/12345/settings"Note: Settings for the Log Router can currently only be configured for Google Cloud organizations. Once configured, it applies to all projects and folders in the Google Cloud organization.
    pub fn update_settings(&self, request: Settings, name: &str) -> OrganizationUpdateSettingCall<'a, S> {
        OrganizationUpdateSettingCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
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
/// extern crate google_logging2 as logging2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use logging2::{Logging, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Logging::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `exclusions_create(...)`, `exclusions_delete(...)`, `exclusions_get(...)`, `exclusions_list(...)`, `exclusions_patch(...)`, `get_cmek_settings(...)`, `get_settings(...)`, `locations_buckets_create(...)`, `locations_buckets_delete(...)`, `locations_buckets_get(...)`, `locations_buckets_list(...)`, `locations_buckets_patch(...)`, `locations_buckets_undelete(...)`, `locations_buckets_views_create(...)`, `locations_buckets_views_delete(...)`, `locations_buckets_views_get(...)`, `locations_buckets_views_list(...)`, `locations_buckets_views_logs_list(...)`, `locations_buckets_views_patch(...)`, `locations_get(...)`, `locations_list(...)`, `locations_operations_cancel(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `logs_delete(...)`, `logs_list(...)`, `metrics_create(...)`, `metrics_delete(...)`, `metrics_get(...)`, `metrics_list(...)`, `metrics_update(...)`, `sinks_create(...)`, `sinks_delete(...)`, `sinks_get(...)`, `sinks_list(...)`, `sinks_patch(...)` and `sinks_update(...)`
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
    /// Creates a new exclusion in the _Default sink in a specified parent resource. Only log entries belonging to that resource can be excluded. You can have up to 10 exclusions in a resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource in which to create the exclusion: "projects/[PROJECT_ID]" "organizations/[ORGANIZATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]" "folders/[FOLDER_ID]" For examples:"projects/my-logging-project" "organizations/123456789"
    pub fn exclusions_create(&self, request: LogExclusion, parent: &str) -> ProjectExclusionCreateCall<'a, S> {
        ProjectExclusionCreateCall {
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
    /// Deletes an exclusion in the _Default sink.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of an existing exclusion to delete: "projects/[PROJECT_ID]/exclusions/[EXCLUSION_ID]" "organizations/[ORGANIZATION_ID]/exclusions/[EXCLUSION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/exclusions/[EXCLUSION_ID]" "folders/[FOLDER_ID]/exclusions/[EXCLUSION_ID]" For example:"projects/my-project/exclusions/my-exclusion"
    pub fn exclusions_delete(&self, name: &str) -> ProjectExclusionDeleteCall<'a, S> {
        ProjectExclusionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the description of an exclusion in the _Default sink.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of an existing exclusion: "projects/[PROJECT_ID]/exclusions/[EXCLUSION_ID]" "organizations/[ORGANIZATION_ID]/exclusions/[EXCLUSION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/exclusions/[EXCLUSION_ID]" "folders/[FOLDER_ID]/exclusions/[EXCLUSION_ID]" For example:"projects/my-project/exclusions/my-exclusion"
    pub fn exclusions_get(&self, name: &str) -> ProjectExclusionGetCall<'a, S> {
        ProjectExclusionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the exclusions on the _Default sink in a parent resource.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource whose exclusions are to be listed. "projects/[PROJECT_ID]" "organizations/[ORGANIZATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]" "folders/[FOLDER_ID]" 
    pub fn exclusions_list(&self, parent: &str) -> ProjectExclusionListCall<'a, S> {
        ProjectExclusionListCall {
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
    /// Changes one or more properties of an existing exclusion in the _Default sink.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the exclusion to update: "projects/[PROJECT_ID]/exclusions/[EXCLUSION_ID]" "organizations/[ORGANIZATION_ID]/exclusions/[EXCLUSION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/exclusions/[EXCLUSION_ID]" "folders/[FOLDER_ID]/exclusions/[EXCLUSION_ID]" For example:"projects/my-project/exclusions/my-exclusion"
    pub fn exclusions_patch(&self, request: LogExclusion, name: &str) -> ProjectExclusionPatchCall<'a, S> {
        ProjectExclusionPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the logs in projects, organizations, folders, or billing accounts. Only logs that have entries are listed.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name that owns the logs: projects/[PROJECT_ID] organizations/[ORGANIZATION_ID] billingAccounts/[BILLING_ACCOUNT_ID] folders/[FOLDER_ID]
    pub fn locations_buckets_views_logs_list(&self, parent: &str) -> ProjectLocationBucketViewLogListCall<'a, S> {
        ProjectLocationBucketViewLogListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _resource_names: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a view over log entries in a log bucket. A bucket may contain a maximum of 30 views.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The bucket in which to create the view `"projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]"` For example:"projects/my-project/locations/global/buckets/my-bucket"
    pub fn locations_buckets_views_create(&self, request: LogView, parent: &str) -> ProjectLocationBucketViewCreateCall<'a, S> {
        ProjectLocationBucketViewCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _view_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a view on a log bucket. If an UNAVAILABLE error is returned, this indicates that system is not in a state where it can delete the view. If this occurs, please try again in a few minutes.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The full resource name of the view to delete: "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket/views/my-view"
    pub fn locations_buckets_views_delete(&self, name: &str) -> ProjectLocationBucketViewDeleteCall<'a, S> {
        ProjectLocationBucketViewDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a view on a log bucket..
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the policy: "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket/views/my-view"
    pub fn locations_buckets_views_get(&self, name: &str) -> ProjectLocationBucketViewGetCall<'a, S> {
        ProjectLocationBucketViewGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists views on a log bucket.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The bucket whose views are to be listed: "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" 
    pub fn locations_buckets_views_list(&self, parent: &str) -> ProjectLocationBucketViewListCall<'a, S> {
        ProjectLocationBucketViewListCall {
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
    /// Updates a view on a log bucket. This method replaces the following fields in the existing view with values from the new view: filter. If an UNAVAILABLE error is returned, this indicates that system is not in a state where it can update the view. If this occurs, please try again in a few minutes.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The full resource name of the view to update "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket/views/my-view"
    pub fn locations_buckets_views_patch(&self, request: LogView, name: &str) -> ProjectLocationBucketViewPatchCall<'a, S> {
        ProjectLocationBucketViewPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a log bucket that can be used to store log entries. After a bucket has been created, the bucket's location cannot be changed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource in which to create the log bucket: "projects/[PROJECT_ID]/locations/[LOCATION_ID]" For example:"projects/my-project/locations/global"
    pub fn locations_buckets_create(&self, request: LogBucket, parent: &str) -> ProjectLocationBucketCreateCall<'a, S> {
        ProjectLocationBucketCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _bucket_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a log bucket.Changes the bucket's lifecycle_state to the DELETE_REQUESTED state. After 7 days, the bucket will be purged and all log entries in the bucket will be permanently deleted.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The full resource name of the bucket to delete. "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket"
    pub fn locations_buckets_delete(&self, name: &str) -> ProjectLocationBucketDeleteCall<'a, S> {
        ProjectLocationBucketDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a log bucket.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the bucket: "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket"
    pub fn locations_buckets_get(&self, name: &str) -> ProjectLocationBucketGetCall<'a, S> {
        ProjectLocationBucketGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists log buckets.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource whose buckets are to be listed: "projects/[PROJECT_ID]/locations/[LOCATION_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]" Note: The locations portion of the resource must be specified, but supplying the character - in place of LOCATION_ID will return all buckets.
    pub fn locations_buckets_list(&self, parent: &str) -> ProjectLocationBucketListCall<'a, S> {
        ProjectLocationBucketListCall {
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
    /// Updates a log bucket. This method replaces the following fields in the existing bucket with values from the new bucket: retention_periodIf the retention period is decreased and the bucket is locked, FAILED_PRECONDITION will be returned.If the bucket has a lifecycle_state of DELETE_REQUESTED, then FAILED_PRECONDITION will be returned.After a bucket has been created, the bucket's location cannot be changed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The full resource name of the bucket to update. "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket"
    pub fn locations_buckets_patch(&self, request: LogBucket, name: &str) -> ProjectLocationBucketPatchCall<'a, S> {
        ProjectLocationBucketPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Undeletes a log bucket. A bucket that has been deleted can be undeleted within the grace period of 7 days.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The full resource name of the bucket to undelete. "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" For example:"projects/my-project/locations/global/buckets/my-bucket"
    pub fn locations_buckets_undelete(&self, request: UndeleteBucketRequest, name: &str) -> ProjectLocationBucketUndeleteCall<'a, S> {
        ProjectLocationBucketUndeleteCall {
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
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn locations_operations_cancel(&self, request: CancelOperationRequest, name: &str) -> ProjectLocationOperationCancelCall<'a, S> {
        ProjectLocationOperationCancelCall {
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
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn locations_operations_get(&self, name: &str) -> ProjectLocationOperationGetCall<'a, S> {
        ProjectLocationOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns UNIMPLEMENTED.NOTE: the name binding allows API services to override the binding to use different resource name schemes, such as users/*/operations. To override the binding, API services can add a binding such as "/v1/{name=users/*}/operations" to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn locations_operations_list(&self, name: &str) -> ProjectLocationOperationListCall<'a, S> {
        ProjectLocationOperationListCall {
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
    /// Deletes all the log entries in a log for the _Default Log Bucket. The log reappears if it receives new entries. Log entries written shortly before the delete operation might not be deleted. Entries received after the delete operation with a timestamp before the operation will be deleted.
    /// 
    /// # Arguments
    ///
    /// * `logName` - Required. The resource name of the log to delete: projects/[PROJECT_ID]/logs/[LOG_ID] organizations/[ORGANIZATION_ID]/logs/[LOG_ID] billingAccounts/[BILLING_ACCOUNT_ID]/logs/[LOG_ID] folders/[FOLDER_ID]/logs/[LOG_ID][LOG_ID] must be URL-encoded. For example, "projects/my-project-id/logs/syslog", "organizations/123/logs/cloudaudit.googleapis.com%2Factivity".For more information about log names, see LogEntry.
    pub fn logs_delete(&self, log_name: &str) -> ProjectLogDeleteCall<'a, S> {
        ProjectLogDeleteCall {
            hub: self.hub,
            _log_name: log_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the logs in projects, organizations, folders, or billing accounts. Only logs that have entries are listed.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name that owns the logs: projects/[PROJECT_ID] organizations/[ORGANIZATION_ID] billingAccounts/[BILLING_ACCOUNT_ID] folders/[FOLDER_ID]
    pub fn logs_list(&self, parent: &str) -> ProjectLogListCall<'a, S> {
        ProjectLogListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _resource_names: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a logs-based metric.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the project in which to create the metric: "projects/[PROJECT_ID]" The new metric must be provided in the request.
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
    /// * `metricName` - Required. The resource name of the metric to delete: "projects/[PROJECT_ID]/metrics/[METRIC_ID]" 
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
    /// * `metricName` - Required. The resource name of the desired metric: "projects/[PROJECT_ID]/metrics/[METRIC_ID]" 
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
    /// * `parent` - Required. The name of the project containing the metrics: "projects/[PROJECT_ID]" 
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
    /// * `metricName` - Required. The resource name of the metric to update: "projects/[PROJECT_ID]/metrics/[METRIC_ID]" The updated metric must be provided in the request and it's name field must be the same as [METRIC_ID] If the metric does not exist in [PROJECT_ID], then a new metric is created.
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
    /// * `parent` - Required. The resource in which to create the sink: "projects/[PROJECT_ID]" "organizations/[ORGANIZATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]" "folders/[FOLDER_ID]" For examples:"projects/my-project" "organizations/123456789"
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
    /// * `sinkName` - Required. The full resource name of the sink to delete, including the parent resource and the sink identifier: "projects/[PROJECT_ID]/sinks/[SINK_ID]" "organizations/[ORGANIZATION_ID]/sinks/[SINK_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_ID]" "folders/[FOLDER_ID]/sinks/[SINK_ID]" For example:"projects/my-project/sinks/my-sink"
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
    /// * `sinkName` - Required. The resource name of the sink: "projects/[PROJECT_ID]/sinks/[SINK_ID]" "organizations/[ORGANIZATION_ID]/sinks/[SINK_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_ID]" "folders/[FOLDER_ID]/sinks/[SINK_ID]" For example:"projects/my-project/sinks/my-sink"
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
    /// * `parent` - Required. The parent resource whose sinks are to be listed: "projects/[PROJECT_ID]" "organizations/[ORGANIZATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]" "folders/[FOLDER_ID]" 
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
    /// * `sinkName` - Required. The full resource name of the sink to update, including the parent resource and the sink identifier: "projects/[PROJECT_ID]/sinks/[SINK_ID]" "organizations/[ORGANIZATION_ID]/sinks/[SINK_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_ID]" "folders/[FOLDER_ID]/sinks/[SINK_ID]" For example:"projects/my-project/sinks/my-sink"
    pub fn sinks_patch(&self, request: LogSink, sink_name: &str) -> ProjectSinkPatchCall<'a, S> {
        ProjectSinkPatchCall {
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
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a sink. This method replaces the following fields in the existing sink with values from the new sink: destination, and filter.The updated sink might also have a new writer_identity; see the unique_writer_identity field.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `sinkName` - Required. The full resource name of the sink to update, including the parent resource and the sink identifier: "projects/[PROJECT_ID]/sinks/[SINK_ID]" "organizations/[ORGANIZATION_ID]/sinks/[SINK_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_ID]" "folders/[FOLDER_ID]/sinks/[SINK_ID]" For example:"projects/my-project/sinks/my-sink"
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
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the Logging CMEK settings for the given resource.Note: CMEK for the Log Router can be configured for Google Cloud projects, folders, organizations and billing accounts. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource for which to retrieve CMEK settings. "projects/[PROJECT_ID]/cmekSettings" "organizations/[ORGANIZATION_ID]/cmekSettings" "billingAccounts/[BILLING_ACCOUNT_ID]/cmekSettings" "folders/[FOLDER_ID]/cmekSettings" For example:"organizations/12345/cmekSettings"Note: CMEK for the Log Router can be configured for Google Cloud projects, folders, organizations and billing accounts. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.
    pub fn get_cmek_settings(&self, name: &str) -> ProjectGetCmekSettingCall<'a, S> {
        ProjectGetCmekSettingCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the Log Router settings for the given resource.Note: Settings for the Log Router can be get for Google Cloud projects, folders, organizations and billing accounts. Currently it can only be configured for organizations. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource for which to retrieve settings. "projects/[PROJECT_ID]/settings" "organizations/[ORGANIZATION_ID]/settings" "billingAccounts/[BILLING_ACCOUNT_ID]/settings" "folders/[FOLDER_ID]/settings" For example:"organizations/12345/settings"Note: Settings for the Log Router can be get for Google Cloud projects, folders, organizations and billing accounts. Currently it can only be configured for organizations. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.
    pub fn get_settings(&self, name: &str) -> ProjectGetSettingCall<'a, S> {
        ProjectGetSettingCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *sink* resources.
/// It is not used directly, but through the [`Logging`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_logging2 as logging2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use logging2::{Logging, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Logging::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.sinks();
/// # }
/// ```
pub struct SinkMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Logging<S>,
}

impl<'a, S> client::MethodsBuilder for SinkMethods<'a, S> {}

impl<'a, S> SinkMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a sink that exports specified log entries to a destination. The export of newly-ingested log entries begins immediately, unless the sink's writer_identity is not permitted to write to the destination. A sink can export log entries only from the resource owning the sink.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource in which to create the sink: "projects/[PROJECT_ID]" "organizations/[ORGANIZATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]" "folders/[FOLDER_ID]" For examples:"projects/my-project" "organizations/123456789"
    pub fn create(&self, request: LogSink, parent: &str) -> SinkCreateCall<'a, S> {
        SinkCreateCall {
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
    /// * `sinkName` - Required. The full resource name of the sink to delete, including the parent resource and the sink identifier: "projects/[PROJECT_ID]/sinks/[SINK_ID]" "organizations/[ORGANIZATION_ID]/sinks/[SINK_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_ID]" "folders/[FOLDER_ID]/sinks/[SINK_ID]" For example:"projects/my-project/sinks/my-sink"
    pub fn delete(&self, sink_name: &str) -> SinkDeleteCall<'a, S> {
        SinkDeleteCall {
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
    /// * `sinkName` - Required. The resource name of the sink: "projects/[PROJECT_ID]/sinks/[SINK_ID]" "organizations/[ORGANIZATION_ID]/sinks/[SINK_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_ID]" "folders/[FOLDER_ID]/sinks/[SINK_ID]" For example:"projects/my-project/sinks/my-sink"
    pub fn get(&self, sink_name: &str) -> SinkGetCall<'a, S> {
        SinkGetCall {
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
    /// * `parent` - Required. The parent resource whose sinks are to be listed: "projects/[PROJECT_ID]" "organizations/[ORGANIZATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]" "folders/[FOLDER_ID]" 
    pub fn list(&self, parent: &str) -> SinkListCall<'a, S> {
        SinkListCall {
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
    /// * `sinkName` - Required. The full resource name of the sink to update, including the parent resource and the sink identifier: "projects/[PROJECT_ID]/sinks/[SINK_ID]" "organizations/[ORGANIZATION_ID]/sinks/[SINK_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_ID]" "folders/[FOLDER_ID]/sinks/[SINK_ID]" For example:"projects/my-project/sinks/my-sink"
    pub fn update(&self, request: LogSink, sink_name: &str) -> SinkUpdateCall<'a, S> {
        SinkUpdateCall {
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



/// A builder providing access to all free methods, which are not associated with a particular resource.
/// It is not used directly, but through the [`Logging`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_logging2 as logging2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use logging2::{Logging, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Logging::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_cmek_settings(...)`, `get_settings(...)`, `update_cmek_settings(...)` and `update_settings(...)`
/// // to build up your call.
/// let rb = hub.methods();
/// # }
/// ```
pub struct MethodMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Logging<S>,
}

impl<'a, S> client::MethodsBuilder for MethodMethods<'a, S> {}

impl<'a, S> MethodMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the Logging CMEK settings for the given resource.Note: CMEK for the Log Router can be configured for Google Cloud projects, folders, organizations and billing accounts. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource for which to retrieve CMEK settings. "projects/[PROJECT_ID]/cmekSettings" "organizations/[ORGANIZATION_ID]/cmekSettings" "billingAccounts/[BILLING_ACCOUNT_ID]/cmekSettings" "folders/[FOLDER_ID]/cmekSettings" For example:"organizations/12345/cmekSettings"Note: CMEK for the Log Router can be configured for Google Cloud projects, folders, organizations and billing accounts. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.
    pub fn get_cmek_settings(&self, name: &str) -> MethodGetCmekSettingCall<'a, S> {
        MethodGetCmekSettingCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the Log Router settings for the given resource.Note: Settings for the Log Router can be get for Google Cloud projects, folders, organizations and billing accounts. Currently it can only be configured for organizations. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource for which to retrieve settings. "projects/[PROJECT_ID]/settings" "organizations/[ORGANIZATION_ID]/settings" "billingAccounts/[BILLING_ACCOUNT_ID]/settings" "folders/[FOLDER_ID]/settings" For example:"organizations/12345/settings"Note: Settings for the Log Router can be get for Google Cloud projects, folders, organizations and billing accounts. Currently it can only be configured for organizations. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.
    pub fn get_settings(&self, name: &str) -> MethodGetSettingCall<'a, S> {
        MethodGetSettingCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the Log Router CMEK settings for the given resource.Note: CMEK for the Log Router can currently only be configured for Google Cloud organizations. Once configured, it applies to all projects and folders in the Google Cloud organization.UpdateCmekSettings will fail if 1) kms_key_name is invalid, or 2) the associated service account does not have the required roles/cloudkms.cryptoKeyEncrypterDecrypter role assigned for the key, or 3) access to the key is disabled.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name for the CMEK settings to update. "projects/[PROJECT_ID]/cmekSettings" "organizations/[ORGANIZATION_ID]/cmekSettings" "billingAccounts/[BILLING_ACCOUNT_ID]/cmekSettings" "folders/[FOLDER_ID]/cmekSettings" For example:"organizations/12345/cmekSettings"Note: CMEK for the Log Router can currently only be configured for Google Cloud organizations. Once configured, it applies to all projects and folders in the Google Cloud organization.
    pub fn update_cmek_settings(&self, request: CmekSettings, name: &str) -> MethodUpdateCmekSettingCall<'a, S> {
        MethodUpdateCmekSettingCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the Log Router settings for the given resource.Note: Settings for the Log Router can currently only be configured for Google Cloud organizations. Once configured, it applies to all projects and folders in the Google Cloud organization.UpdateSettings will fail if 1) kms_key_name is invalid, or 2) the associated service account does not have the required roles/cloudkms.cryptoKeyEncrypterDecrypter role assigned for the key, or 3) access to the key is disabled. 4) location_id is not supported by Logging. 5) location_id violate OrgPolicy.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name for the settings to update. "organizations/[ORGANIZATION_ID]/settings" For example:"organizations/12345/settings"Note: Settings for the Log Router can currently only be configured for Google Cloud organizations. Once configured, it applies to all projects and folders in the Google Cloud organization.
    pub fn update_settings(&self, request: Settings, name: &str) -> MethodUpdateSettingCall<'a, S> {
        MethodUpdateSettingCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



