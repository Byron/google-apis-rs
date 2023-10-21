use super::*;
/// A builder providing access to all methods supported on *folder* resources.
/// It is not used directly, but through the [`Monitoring`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_monitoring3 as monitoring3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use monitoring3::{Monitoring, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Monitoring::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `time_series_list(...)`
/// // to build up your call.
/// let rb = hub.folders();
/// # }
/// ```
pub struct FolderMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Monitoring<S>,
}

impl<'a, S> client::MethodsBuilder for FolderMethods<'a, S> {}

impl<'a, S> FolderMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists time series that match a filter.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The project (https://cloud.google.com/monitoring/api/v3#project_name), organization or folder on which to execute the request. The format is: projects/[PROJECT_ID_OR_NUMBER] organizations/[ORGANIZATION_ID] folders/[FOLDER_ID] 
    pub fn time_series_list(&self, name: &str) -> FolderTimeSeryListCall<'a, S> {
        FolderTimeSeryListCall {
            hub: self.hub,
            _name: name.to_string(),
            _view: Default::default(),
            _secondary_aggregation_per_series_aligner: Default::default(),
            _secondary_aggregation_group_by_fields: Default::default(),
            _secondary_aggregation_cross_series_reducer: Default::default(),
            _secondary_aggregation_alignment_period: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _interval_start_time: Default::default(),
            _interval_end_time: Default::default(),
            _filter: Default::default(),
            _aggregation_per_series_aligner: Default::default(),
            _aggregation_group_by_fields: Default::default(),
            _aggregation_cross_series_reducer: Default::default(),
            _aggregation_alignment_period: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *organization* resources.
/// It is not used directly, but through the [`Monitoring`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_monitoring3 as monitoring3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use monitoring3::{Monitoring, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Monitoring::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `time_series_list(...)`
/// // to build up your call.
/// let rb = hub.organizations();
/// # }
/// ```
pub struct OrganizationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Monitoring<S>,
}

impl<'a, S> client::MethodsBuilder for OrganizationMethods<'a, S> {}

impl<'a, S> OrganizationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists time series that match a filter.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The project (https://cloud.google.com/monitoring/api/v3#project_name), organization or folder on which to execute the request. The format is: projects/[PROJECT_ID_OR_NUMBER] organizations/[ORGANIZATION_ID] folders/[FOLDER_ID] 
    pub fn time_series_list(&self, name: &str) -> OrganizationTimeSeryListCall<'a, S> {
        OrganizationTimeSeryListCall {
            hub: self.hub,
            _name: name.to_string(),
            _view: Default::default(),
            _secondary_aggregation_per_series_aligner: Default::default(),
            _secondary_aggregation_group_by_fields: Default::default(),
            _secondary_aggregation_cross_series_reducer: Default::default(),
            _secondary_aggregation_alignment_period: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _interval_start_time: Default::default(),
            _interval_end_time: Default::default(),
            _filter: Default::default(),
            _aggregation_per_series_aligner: Default::default(),
            _aggregation_group_by_fields: Default::default(),
            _aggregation_cross_series_reducer: Default::default(),
            _aggregation_alignment_period: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Monitoring`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_monitoring3 as monitoring3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use monitoring3::{Monitoring, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Monitoring::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `alert_policies_create(...)`, `alert_policies_delete(...)`, `alert_policies_get(...)`, `alert_policies_list(...)`, `alert_policies_patch(...)`, `collectd_time_series_create(...)`, `groups_create(...)`, `groups_delete(...)`, `groups_get(...)`, `groups_list(...)`, `groups_members_list(...)`, `groups_update(...)`, `metric_descriptors_create(...)`, `metric_descriptors_delete(...)`, `metric_descriptors_get(...)`, `metric_descriptors_list(...)`, `monitored_resource_descriptors_get(...)`, `monitored_resource_descriptors_list(...)`, `notification_channel_descriptors_get(...)`, `notification_channel_descriptors_list(...)`, `notification_channels_create(...)`, `notification_channels_delete(...)`, `notification_channels_get(...)`, `notification_channels_get_verification_code(...)`, `notification_channels_list(...)`, `notification_channels_patch(...)`, `notification_channels_send_verification_code(...)`, `notification_channels_verify(...)`, `snoozes_create(...)`, `snoozes_get(...)`, `snoozes_list(...)`, `snoozes_patch(...)`, `time_series_create(...)`, `time_series_create_service(...)`, `time_series_list(...)`, `time_series_query(...)`, `uptime_check_configs_create(...)`, `uptime_check_configs_delete(...)`, `uptime_check_configs_get(...)`, `uptime_check_configs_list(...)` and `uptime_check_configs_patch(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Monitoring<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new alerting policy.Design your application to single-thread API calls that modify the state of alerting policies in a single project. This includes calls to CreateAlertPolicy, DeleteAlertPolicy and UpdateAlertPolicy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) in which to create the alerting policy. The format is: projects/[PROJECT_ID_OR_NUMBER] Note that this field names the parent container in which the alerting policy will be written, not the name of the created policy. |name| must be a host project of a Metrics Scope, otherwise INVALID_ARGUMENT error will return. The alerting policy that is returned will have a name that contains a normalized representation of this name as a prefix but adds a suffix of the form /alertPolicies/[ALERT_POLICY_ID], identifying the policy in the container.
    pub fn alert_policies_create(&self, request: AlertPolicy, name: &str) -> ProjectAlertPolicyCreateCall<'a, S> {
        ProjectAlertPolicyCreateCall {
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
    /// Deletes an alerting policy.Design your application to single-thread API calls that modify the state of alerting policies in a single project. This includes calls to CreateAlertPolicy, DeleteAlertPolicy and UpdateAlertPolicy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The alerting policy to delete. The format is: projects/[PROJECT_ID_OR_NUMBER]/alertPolicies/[ALERT_POLICY_ID] For more information, see AlertPolicy.
    pub fn alert_policies_delete(&self, name: &str) -> ProjectAlertPolicyDeleteCall<'a, S> {
        ProjectAlertPolicyDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a single alerting policy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The alerting policy to retrieve. The format is: projects/[PROJECT_ID_OR_NUMBER]/alertPolicies/[ALERT_POLICY_ID] 
    pub fn alert_policies_get(&self, name: &str) -> ProjectAlertPolicyGetCall<'a, S> {
        ProjectAlertPolicyGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the existing alerting policies for the workspace.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) whose alert policies are to be listed. The format is: projects/[PROJECT_ID_OR_NUMBER] Note that this field names the parent container in which the alerting policies to be listed are stored. To retrieve a single alerting policy by name, use the GetAlertPolicy operation, instead.
    pub fn alert_policies_list(&self, name: &str) -> ProjectAlertPolicyListCall<'a, S> {
        ProjectAlertPolicyListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an alerting policy. You can either replace the entire policy with a new one or replace only certain fields in the current alerting policy by specifying the fields to be updated via updateMask. Returns the updated alerting policy.Design your application to single-thread API calls that modify the state of alerting policies in a single project. This includes calls to CreateAlertPolicy, DeleteAlertPolicy and UpdateAlertPolicy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required if the policy exists. The resource name for this policy. The format is: projects/[PROJECT_ID_OR_NUMBER]/alertPolicies/[ALERT_POLICY_ID] [ALERT_POLICY_ID] is assigned by Cloud Monitoring when the policy is created. When calling the alertPolicies.create method, do not include the name field in the alerting policy passed as part of the request.
    pub fn alert_policies_patch(&self, request: AlertPolicy, name: &str) -> ProjectAlertPolicyPatchCall<'a, S> {
        ProjectAlertPolicyPatchCall {
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
    /// Cloud Monitoring Agent only: Creates a new time series.This method is only for use by the Cloud Monitoring Agent. Use projects.timeSeries.create instead.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The project (https://cloud.google.com/monitoring/api/v3#project_name) in which to create the time series. The format is: projects/[PROJECT_ID_OR_NUMBER] 
    pub fn collectd_time_series_create(&self, request: CreateCollectdTimeSeriesRequest, name: &str) -> ProjectCollectdTimeSeryCreateCall<'a, S> {
        ProjectCollectdTimeSeryCreateCall {
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
    /// Lists the monitored resources that are members of a group.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The group whose members are listed. The format is: projects/[PROJECT_ID_OR_NUMBER]/groups/[GROUP_ID] 
    pub fn groups_members_list(&self, name: &str) -> ProjectGroupMemberListCall<'a, S> {
        ProjectGroupMemberListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _interval_start_time: Default::default(),
            _interval_end_time: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) in which to create the group. The format is: projects/[PROJECT_ID_OR_NUMBER] 
    pub fn groups_create(&self, request: Group, name: &str) -> ProjectGroupCreateCall<'a, S> {
        ProjectGroupCreateCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing group.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The group to delete. The format is: projects/[PROJECT_ID_OR_NUMBER]/groups/[GROUP_ID] 
    pub fn groups_delete(&self, name: &str) -> ProjectGroupDeleteCall<'a, S> {
        ProjectGroupDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _recursive: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a single group.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The group to retrieve. The format is: projects/[PROJECT_ID_OR_NUMBER]/groups/[GROUP_ID] 
    pub fn groups_get(&self, name: &str) -> ProjectGroupGetCall<'a, S> {
        ProjectGroupGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the existing groups.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) whose groups are to be listed. The format is: projects/[PROJECT_ID_OR_NUMBER] 
    pub fn groups_list(&self, name: &str) -> ProjectGroupListCall<'a, S> {
        ProjectGroupListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _descendants_of_group: Default::default(),
            _children_of_group: Default::default(),
            _ancestors_of_group: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing group. You can change any group attributes except name.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The name of this group. The format is: projects/[PROJECT_ID_OR_NUMBER]/groups/[GROUP_ID] When creating a group, this field is ignored and a new name is created consisting of the project specified in the call to CreateGroup and a unique [GROUP_ID] that is generated automatically.
    pub fn groups_update(&self, request: Group, name: &str) -> ProjectGroupUpdateCall<'a, S> {
        ProjectGroupUpdateCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new metric descriptor. The creation is executed asynchronously. User-created metric descriptors define custom metrics (https://cloud.google.com/monitoring/custom-metrics). The metric descriptor is updated if it already exists, except that metric labels are never removed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) on which to execute the request. The format is: 4 projects/PROJECT_ID_OR_NUMBER
    pub fn metric_descriptors_create(&self, request: MetricDescriptor, name: &str) -> ProjectMetricDescriptorCreateCall<'a, S> {
        ProjectMetricDescriptorCreateCall {
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
    /// Deletes a metric descriptor. Only user-created custom metrics (https://cloud.google.com/monitoring/custom-metrics) can be deleted.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The metric descriptor on which to execute the request. The format is: projects/[PROJECT_ID_OR_NUMBER]/metricDescriptors/[METRIC_ID] An example of [METRIC_ID] is: "custom.googleapis.com/my_test_metric".
    pub fn metric_descriptors_delete(&self, name: &str) -> ProjectMetricDescriptorDeleteCall<'a, S> {
        ProjectMetricDescriptorDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a single metric descriptor.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The metric descriptor on which to execute the request. The format is: projects/[PROJECT_ID_OR_NUMBER]/metricDescriptors/[METRIC_ID] An example value of [METRIC_ID] is "compute.googleapis.com/instance/disk/read_bytes_count".
    pub fn metric_descriptors_get(&self, name: &str) -> ProjectMetricDescriptorGetCall<'a, S> {
        ProjectMetricDescriptorGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists metric descriptors that match a filter.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) on which to execute the request. The format is: projects/[PROJECT_ID_OR_NUMBER] 
    pub fn metric_descriptors_list(&self, name: &str) -> ProjectMetricDescriptorListCall<'a, S> {
        ProjectMetricDescriptorListCall {
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
    /// Gets a single monitored resource descriptor.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The monitored resource descriptor to get. The format is: projects/[PROJECT_ID_OR_NUMBER]/monitoredResourceDescriptors/[RESOURCE_TYPE] The [RESOURCE_TYPE] is a predefined type, such as cloudsql_database.
    pub fn monitored_resource_descriptors_get(&self, name: &str) -> ProjectMonitoredResourceDescriptorGetCall<'a, S> {
        ProjectMonitoredResourceDescriptorGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists monitored resource descriptors that match a filter.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) on which to execute the request. The format is: projects/[PROJECT_ID_OR_NUMBER] 
    pub fn monitored_resource_descriptors_list(&self, name: &str) -> ProjectMonitoredResourceDescriptorListCall<'a, S> {
        ProjectMonitoredResourceDescriptorListCall {
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
    /// Gets a single channel descriptor. The descriptor indicates which fields are expected / permitted for a notification channel of the given type.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The channel type for which to execute the request. The format is: projects/[PROJECT_ID_OR_NUMBER]/notificationChannelDescriptors/[CHANNEL_TYPE] 
    pub fn notification_channel_descriptors_get(&self, name: &str) -> ProjectNotificationChannelDescriptorGetCall<'a, S> {
        ProjectNotificationChannelDescriptorGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the descriptors for supported channel types. The use of descriptors makes it possible for new channel types to be dynamically added.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The REST resource name of the parent from which to retrieve the notification channel descriptors. The expected syntax is: projects/[PROJECT_ID_OR_NUMBER] Note that this names (https://cloud.google.com/monitoring/api/v3#project_name) the parent container in which to look for the descriptors; to retrieve a single descriptor by name, use the GetNotificationChannelDescriptor operation, instead.
    pub fn notification_channel_descriptors_list(&self, name: &str) -> ProjectNotificationChannelDescriptorListCall<'a, S> {
        ProjectNotificationChannelDescriptorListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new notification channel, representing a single notification endpoint such as an email address, SMS number, or PagerDuty service.Design your application to single-thread API calls that modify the state of notification channels in a single project. This includes calls to CreateNotificationChannel, DeleteNotificationChannel and UpdateNotificationChannel.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) on which to execute the request. The format is: projects/[PROJECT_ID_OR_NUMBER] This names the container into which the channel will be written, this does not name the newly created channel. The resulting channel's name will have a normalized version of this field as a prefix, but will add /notificationChannels/[CHANNEL_ID] to identify the channel.
    pub fn notification_channels_create(&self, request: NotificationChannel, name: &str) -> ProjectNotificationChannelCreateCall<'a, S> {
        ProjectNotificationChannelCreateCall {
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
    /// Deletes a notification channel.Design your application to single-thread API calls that modify the state of notification channels in a single project. This includes calls to CreateNotificationChannel, DeleteNotificationChannel and UpdateNotificationChannel.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The channel for which to execute the request. The format is: projects/[PROJECT_ID_OR_NUMBER]/notificationChannels/[CHANNEL_ID] 
    pub fn notification_channels_delete(&self, name: &str) -> ProjectNotificationChannelDeleteCall<'a, S> {
        ProjectNotificationChannelDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _force: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a single notification channel. The channel includes the relevant configuration details with which the channel was created. However, the response may truncate or omit passwords, API keys, or other private key matter and thus the response may not be 100% identical to the information that was supplied in the call to the create method.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The channel for which to execute the request. The format is: projects/[PROJECT_ID_OR_NUMBER]/notificationChannels/[CHANNEL_ID] 
    pub fn notification_channels_get(&self, name: &str) -> ProjectNotificationChannelGetCall<'a, S> {
        ProjectNotificationChannelGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Requests a verification code for an already verified channel that can then be used in a call to VerifyNotificationChannel() on a different channel with an equivalent identity in the same or in a different project. This makes it possible to copy a channel between projects without requiring manual reverification of the channel. If the channel is not in the verified state, this method will fail (in other words, this may only be used if the SendNotificationChannelVerificationCode and VerifyNotificationChannel paths have already been used to put the given channel into the verified state).There is no guarantee that the verification codes returned by this method will be of a similar structure or form as the ones that are delivered to the channel via SendNotificationChannelVerificationCode; while VerifyNotificationChannel() will recognize both the codes delivered via SendNotificationChannelVerificationCode() and returned from GetNotificationChannelVerificationCode(), it is typically the case that the verification codes delivered via SendNotificationChannelVerificationCode() will be shorter and also have a shorter expiration (e.g. codes such as "G-123456") whereas GetVerificationCode() will typically return a much longer, websafe base 64 encoded string that has a longer expiration time.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The notification channel for which a verification code is to be generated and retrieved. This must name a channel that is already verified; if the specified channel is not verified, the request will fail.
    pub fn notification_channels_get_verification_code(&self, request: GetNotificationChannelVerificationCodeRequest, name: &str) -> ProjectNotificationChannelGetVerificationCodeCall<'a, S> {
        ProjectNotificationChannelGetVerificationCodeCall {
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
    /// Lists the notification channels that have been created for the project.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) on which to execute the request. The format is: projects/[PROJECT_ID_OR_NUMBER] This names the container in which to look for the notification channels; it does not name a specific channel. To query a specific channel by REST resource name, use the GetNotificationChannel operation.
    pub fn notification_channels_list(&self, name: &str) -> ProjectNotificationChannelListCall<'a, S> {
        ProjectNotificationChannelListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a notification channel. Fields not specified in the field mask remain unchanged.Design your application to single-thread API calls that modify the state of notification channels in a single project. This includes calls to CreateNotificationChannel, DeleteNotificationChannel and UpdateNotificationChannel.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The full REST resource name for this channel. The format is: projects/[PROJECT_ID_OR_NUMBER]/notificationChannels/[CHANNEL_ID] The [CHANNEL_ID] is automatically assigned by the server on creation.
    pub fn notification_channels_patch(&self, request: NotificationChannel, name: &str) -> ProjectNotificationChannelPatchCall<'a, S> {
        ProjectNotificationChannelPatchCall {
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
    /// Causes a verification code to be delivered to the channel. The code can then be supplied in VerifyNotificationChannel to verify the channel.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The notification channel to which to send a verification code.
    pub fn notification_channels_send_verification_code(&self, request: SendNotificationChannelVerificationCodeRequest, name: &str) -> ProjectNotificationChannelSendVerificationCodeCall<'a, S> {
        ProjectNotificationChannelSendVerificationCodeCall {
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
    /// Verifies a NotificationChannel by proving receipt of the code delivered to the channel as a result of calling SendNotificationChannelVerificationCode.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The notification channel to verify.
    pub fn notification_channels_verify(&self, request: VerifyNotificationChannelRequest, name: &str) -> ProjectNotificationChannelVerifyCall<'a, S> {
        ProjectNotificationChannelVerifyCall {
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
    /// Creates a Snooze that will prevent alerts, which match the provided criteria, from being opened. The Snooze applies for a specific time interval.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) in which a Snooze should be created. The format is: projects/[PROJECT_ID_OR_NUMBER] 
    pub fn snoozes_create(&self, request: Snooze, parent: &str) -> ProjectSnoozeCreateCall<'a, S> {
        ProjectSnoozeCreateCall {
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
    /// Retrieves a Snooze by name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The ID of the Snooze to retrieve. The format is: projects/[PROJECT_ID_OR_NUMBER]/snoozes/[SNOOZE_ID] 
    pub fn snoozes_get(&self, name: &str) -> ProjectSnoozeGetCall<'a, S> {
        ProjectSnoozeGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the Snoozes associated with a project. Can optionally pass in filter, which specifies predicates to match Snoozes.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) whose Snoozes should be listed. The format is: projects/[PROJECT_ID_OR_NUMBER] 
    pub fn snoozes_list(&self, parent: &str) -> ProjectSnoozeListCall<'a, S> {
        ProjectSnoozeListCall {
            hub: self.hub,
            _parent: parent.to_string(),
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
    /// Updates a Snooze, identified by its name, with the parameters in the given Snooze object.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the Snooze. The format is: projects/[PROJECT_ID_OR_NUMBER]/snoozes/[SNOOZE_ID] The ID of the Snooze will be generated by the system.
    pub fn snoozes_patch(&self, request: Snooze, name: &str) -> ProjectSnoozePatchCall<'a, S> {
        ProjectSnoozePatchCall {
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
    /// Creates or adds data to one or more time series. The response is empty if all time series in the request were written. If any time series could not be written, a corresponding failure message is included in the error response.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) on which to execute the request. The format is: projects/[PROJECT_ID_OR_NUMBER] 
    pub fn time_series_create(&self, request: CreateTimeSeriesRequest, name: &str) -> ProjectTimeSeryCreateCall<'a, S> {
        ProjectTimeSeryCreateCall {
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
    /// Creates or adds data to one or more service time series. A service time series is a time series for a metric from a Google Cloud service. The response is empty if all time series in the request were written. If any time series could not be written, a corresponding failure message is included in the error response. This endpoint rejects writes to user-defined metrics. This method is only for use by Google Cloud services. Use projects.timeSeries.create instead.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) on which to execute the request. The format is: projects/[PROJECT_ID_OR_NUMBER] 
    pub fn time_series_create_service(&self, request: CreateTimeSeriesRequest, name: &str) -> ProjectTimeSeryCreateServiceCall<'a, S> {
        ProjectTimeSeryCreateServiceCall {
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
    /// Lists time series that match a filter.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The project (https://cloud.google.com/monitoring/api/v3#project_name), organization or folder on which to execute the request. The format is: projects/[PROJECT_ID_OR_NUMBER] organizations/[ORGANIZATION_ID] folders/[FOLDER_ID] 
    pub fn time_series_list(&self, name: &str) -> ProjectTimeSeryListCall<'a, S> {
        ProjectTimeSeryListCall {
            hub: self.hub,
            _name: name.to_string(),
            _view: Default::default(),
            _secondary_aggregation_per_series_aligner: Default::default(),
            _secondary_aggregation_group_by_fields: Default::default(),
            _secondary_aggregation_cross_series_reducer: Default::default(),
            _secondary_aggregation_alignment_period: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _interval_start_time: Default::default(),
            _interval_end_time: Default::default(),
            _filter: Default::default(),
            _aggregation_per_series_aligner: Default::default(),
            _aggregation_group_by_fields: Default::default(),
            _aggregation_cross_series_reducer: Default::default(),
            _aggregation_alignment_period: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Queries time series using Monitoring Query Language.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) on which to execute the request. The format is: projects/[PROJECT_ID_OR_NUMBER] 
    pub fn time_series_query(&self, request: QueryTimeSeriesRequest, name: &str) -> ProjectTimeSeryQueryCall<'a, S> {
        ProjectTimeSeryQueryCall {
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
    /// Creates a new Uptime check configuration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) in which to create the Uptime check. The format is: projects/[PROJECT_ID_OR_NUMBER] 
    pub fn uptime_check_configs_create(&self, request: UptimeCheckConfig, parent: &str) -> ProjectUptimeCheckConfigCreateCall<'a, S> {
        ProjectUptimeCheckConfigCreateCall {
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
    /// Deletes an Uptime check configuration. Note that this method will fail if the Uptime check configuration is referenced by an alert policy or other dependent configs that would be rendered invalid by the deletion.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The Uptime check configuration to delete. The format is: projects/[PROJECT_ID_OR_NUMBER]/uptimeCheckConfigs/[UPTIME_CHECK_ID] 
    pub fn uptime_check_configs_delete(&self, name: &str) -> ProjectUptimeCheckConfigDeleteCall<'a, S> {
        ProjectUptimeCheckConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a single Uptime check configuration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The Uptime check configuration to retrieve. The format is: projects/[PROJECT_ID_OR_NUMBER]/uptimeCheckConfigs/[UPTIME_CHECK_ID] 
    pub fn uptime_check_configs_get(&self, name: &str) -> ProjectUptimeCheckConfigGetCall<'a, S> {
        ProjectUptimeCheckConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the existing valid Uptime check configurations for the project (leaving out any invalid configurations).
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) whose Uptime check configurations are listed. The format is: projects/[PROJECT_ID_OR_NUMBER] 
    pub fn uptime_check_configs_list(&self, parent: &str) -> ProjectUptimeCheckConfigListCall<'a, S> {
        ProjectUptimeCheckConfigListCall {
            hub: self.hub,
            _parent: parent.to_string(),
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
    /// Updates an Uptime check configuration. You can either replace the entire configuration with a new one or replace only certain fields in the current configuration by specifying the fields to be updated via updateMask. Returns the updated configuration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - A unique resource name for this Uptime check configuration. The format is: projects/[PROJECT_ID_OR_NUMBER]/uptimeCheckConfigs/[UPTIME_CHECK_ID] [PROJECT_ID_OR_NUMBER] is the Workspace host project associated with the Uptime check.This field should be omitted when creating the Uptime check configuration; on create, the resource name is assigned by the server and included in the response.
    pub fn uptime_check_configs_patch(&self, request: UptimeCheckConfig, name: &str) -> ProjectUptimeCheckConfigPatchCall<'a, S> {
        ProjectUptimeCheckConfigPatchCall {
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



/// A builder providing access to all methods supported on *service* resources.
/// It is not used directly, but through the [`Monitoring`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_monitoring3 as monitoring3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use monitoring3::{Monitoring, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Monitoring::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `list(...)`, `patch(...)`, `service_level_objectives_create(...)`, `service_level_objectives_delete(...)`, `service_level_objectives_get(...)`, `service_level_objectives_list(...)` and `service_level_objectives_patch(...)`
/// // to build up your call.
/// let rb = hub.services();
/// # }
/// ```
pub struct ServiceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Monitoring<S>,
}

impl<'a, S> client::MethodsBuilder for ServiceMethods<'a, S> {}

impl<'a, S> ServiceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a ServiceLevelObjective for the given Service.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the parent Service. The format is: projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID] 
    pub fn service_level_objectives_create(&self, request: ServiceLevelObjective, parent: &str) -> ServiceServiceLevelObjectiveCreateCall<'a, S> {
        ServiceServiceLevelObjectiveCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _service_level_objective_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete the given ServiceLevelObjective.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the ServiceLevelObjective to delete. The format is: projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID]/serviceLevelObjectives/[SLO_NAME] 
    pub fn service_level_objectives_delete(&self, name: &str) -> ServiceServiceLevelObjectiveDeleteCall<'a, S> {
        ServiceServiceLevelObjectiveDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a ServiceLevelObjective by name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the ServiceLevelObjective to get. The format is: projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID]/serviceLevelObjectives/[SLO_NAME] 
    pub fn service_level_objectives_get(&self, name: &str) -> ServiceServiceLevelObjectiveGetCall<'a, S> {
        ServiceServiceLevelObjectiveGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the ServiceLevelObjectives for the given Service.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name of the parent containing the listed SLOs, either a project or a Monitoring Metrics Scope. The formats are: projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID] workspaces/[HOST_PROJECT_ID_OR_NUMBER]/services/- 
    pub fn service_level_objectives_list(&self, parent: &str) -> ServiceServiceLevelObjectiveListCall<'a, S> {
        ServiceServiceLevelObjectiveListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _view: Default::default(),
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
    /// Update the given ServiceLevelObjective.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Resource name for this ServiceLevelObjective. The format is: projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID]/serviceLevelObjectives/[SLO_NAME] 
    pub fn service_level_objectives_patch(&self, request: ServiceLevelObjective, name: &str) -> ServiceServiceLevelObjectivePatchCall<'a, S> {
        ServiceServiceLevelObjectivePatchCall {
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
    /// Create a Service.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name (https://cloud.google.com/monitoring/api/v3#project_name) of the parent Metrics Scope. The format is: projects/[PROJECT_ID_OR_NUMBER] 
    pub fn create(&self, request: Service, parent: &str) -> ServiceCreateCall<'a, S> {
        ServiceCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _service_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Soft delete this Service.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the Service to delete. The format is: projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID] 
    pub fn delete(&self, name: &str) -> ServiceDeleteCall<'a, S> {
        ServiceDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get the named Service.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the Service. The format is: projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID] 
    pub fn get(&self, name: &str) -> ServiceGetCall<'a, S> {
        ServiceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List Services for this Metrics Scope.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name of the parent containing the listed services, either a project (https://cloud.google.com/monitoring/api/v3#project_name) or a Monitoring Metrics Scope. The formats are: projects/[PROJECT_ID_OR_NUMBER] workspaces/[HOST_PROJECT_ID_OR_NUMBER] 
    pub fn list(&self, parent: &str) -> ServiceListCall<'a, S> {
        ServiceListCall {
            hub: self.hub,
            _parent: parent.to_string(),
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
    /// Update this Service.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Resource name for this Service. The format is: projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID] 
    pub fn patch(&self, request: Service, name: &str) -> ServicePatchCall<'a, S> {
        ServicePatchCall {
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



/// A builder providing access to all methods supported on *uptimeCheckIp* resources.
/// It is not used directly, but through the [`Monitoring`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_monitoring3 as monitoring3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use monitoring3::{Monitoring, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Monitoring::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.uptime_check_ips();
/// # }
/// ```
pub struct UptimeCheckIpMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Monitoring<S>,
}

impl<'a, S> client::MethodsBuilder for UptimeCheckIpMethods<'a, S> {}

impl<'a, S> UptimeCheckIpMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of IP addresses that checkers run from
    pub fn list(&self) -> UptimeCheckIpListCall<'a, S> {
        UptimeCheckIpListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



