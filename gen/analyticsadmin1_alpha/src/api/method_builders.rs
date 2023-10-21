use super::*;
/// A builder providing access to all methods supported on *accountSummary* resources.
/// It is not used directly, but through the [`GoogleAnalyticsAdmin`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_analyticsadmin1_alpha as analyticsadmin1_alpha;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use analyticsadmin1_alpha::{GoogleAnalyticsAdmin, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = GoogleAnalyticsAdmin::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.account_summaries();
/// # }
/// ```
pub struct AccountSummaryMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a GoogleAnalyticsAdmin<S>,
}

impl<'a, S> client::MethodsBuilder for AccountSummaryMethods<'a, S> {}

impl<'a, S> AccountSummaryMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns summaries of all accounts accessible by the caller.
    pub fn list(&self) -> AccountSummaryListCall<'a, S> {
        AccountSummaryListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the [`GoogleAnalyticsAdmin`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_analyticsadmin1_alpha as analyticsadmin1_alpha;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use analyticsadmin1_alpha::{GoogleAnalyticsAdmin, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = GoogleAnalyticsAdmin::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `get_data_sharing_settings(...)`, `list(...)`, `patch(...)`, `provision_account_ticket(...)`, `search_change_history_events(...)`, `user_links_audit(...)`, `user_links_batch_create(...)`, `user_links_batch_delete(...)`, `user_links_batch_get(...)`, `user_links_batch_update(...)`, `user_links_create(...)`, `user_links_delete(...)`, `user_links_get(...)`, `user_links_list(...)` and `user_links_patch(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a GoogleAnalyticsAdmin<S>,
}

impl<'a, S> client::MethodsBuilder for AccountMethods<'a, S> {}

impl<'a, S> AccountMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all user links on an account or property, including implicit ones that come from effective permissions granted by groups or organization admin roles. If a returned user link does not have direct permissions, they cannot be removed from the account or property directly with the DeleteUserLink command. They have to be removed from the group/etc that gives them permissions, which is currently only usable/discoverable in the GA or GMP UIs.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Example format: accounts/1234
    pub fn user_links_audit(&self, request: GoogleAnalyticsAdminV1alphaAuditUserLinksRequest, parent: &str) -> AccountUserLinkAuditCall<'a, S> {
        AccountUserLinkAuditCall {
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
    /// Creates information about multiple users' links to an account or property. This method is transactional. If any UserLink cannot be created, none of the UserLinks will be created.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The account or property that all user links in the request are for. This field is required. The parent field in the CreateUserLinkRequest messages must either be empty or match this field. Example format: accounts/1234
    pub fn user_links_batch_create(&self, request: GoogleAnalyticsAdminV1alphaBatchCreateUserLinksRequest, parent: &str) -> AccountUserLinkBatchCreateCall<'a, S> {
        AccountUserLinkBatchCreateCall {
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
    /// Deletes information about multiple users' links to an account or property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The account or property that all user links in the request are for. The parent of all values for user link names to delete must match this field. Example format: accounts/1234
    pub fn user_links_batch_delete(&self, request: GoogleAnalyticsAdminV1alphaBatchDeleteUserLinksRequest, parent: &str) -> AccountUserLinkBatchDeleteCall<'a, S> {
        AccountUserLinkBatchDeleteCall {
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
    /// Gets information about multiple users' links to an account or property.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The account or property that all user links in the request are for. The parent of all provided values for the 'names' field must match this field. Example format: accounts/1234
    pub fn user_links_batch_get(&self, parent: &str) -> AccountUserLinkBatchGetCall<'a, S> {
        AccountUserLinkBatchGetCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _names: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates information about multiple users' links to an account or property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The account or property that all user links in the request are for. The parent field in the UpdateUserLinkRequest messages must either be empty or match this field. Example format: accounts/1234
    pub fn user_links_batch_update(&self, request: GoogleAnalyticsAdminV1alphaBatchUpdateUserLinksRequest, parent: &str) -> AccountUserLinkBatchUpdateCall<'a, S> {
        AccountUserLinkBatchUpdateCall {
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
    /// Creates a user link on an account or property. If the user with the specified email already has permissions on the account or property, then the user's existing permissions will be unioned with the permissions specified in the new UserLink.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Example format: accounts/1234
    pub fn user_links_create(&self, request: GoogleAnalyticsAdminV1alphaUserLink, parent: &str) -> AccountUserLinkCreateCall<'a, S> {
        AccountUserLinkCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _notify_new_user: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a user link on an account or property.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Example format: accounts/1234/userLinks/5678
    pub fn user_links_delete(&self, name: &str) -> AccountUserLinkDeleteCall<'a, S> {
        AccountUserLinkDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about a user's link to an account or property.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Example format: accounts/1234/userLinks/5678
    pub fn user_links_get(&self, name: &str) -> AccountUserLinkGetCall<'a, S> {
        AccountUserLinkGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all user links on an account or property.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Example format: accounts/1234
    pub fn user_links_list(&self, parent: &str) -> AccountUserLinkListCall<'a, S> {
        AccountUserLinkListCall {
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
    /// Updates a user link on an account or property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Example format: properties/1234/userLinks/5678
    pub fn user_links_patch(&self, request: GoogleAnalyticsAdminV1alphaUserLink, name: &str) -> AccountUserLinkPatchCall<'a, S> {
        AccountUserLinkPatchCall {
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
    /// Marks target Account as soft-deleted (ie: "trashed") and returns it. This API does not have a method to restore soft-deleted accounts. However, they can be restored using the Trash Can UI. If the accounts are not restored before the expiration time, the account and all child resources (eg: Properties, GoogleAdsLinks, Streams, UserLinks) will be permanently purged. https://support.google.com/analytics/answer/6154772 Returns an error if the target is not found.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the Account to soft-delete. Format: accounts/{account} Example: "accounts/100"
    pub fn delete(&self, name: &str) -> AccountDeleteCall<'a, S> {
        AccountDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lookup for a single Account.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the account to lookup. Format: accounts/{account} Example: "accounts/100"
    pub fn get(&self, name: &str) -> AccountGetCall<'a, S> {
        AccountGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get data sharing settings on an account. Data sharing settings are singletons.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the settings to lookup. Format: accounts/{account}/dataSharingSettings Example: "accounts/1000/dataSharingSettings"
    pub fn get_data_sharing_settings(&self, name: &str) -> AccountGetDataSharingSettingCall<'a, S> {
        AccountGetDataSharingSettingCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns all accounts accessible by the caller. Note that these accounts might not currently have GA4 properties. Soft-deleted (ie: "trashed") accounts are excluded by default. Returns an empty list if no relevant accounts are found.
    pub fn list(&self) -> AccountListCall<'a, S> {
        AccountListCall {
            hub: self.hub,
            _show_deleted: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an account.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name of this account. Format: accounts/{account} Example: "accounts/100"
    pub fn patch(&self, request: GoogleAnalyticsAdminV1alphaAccount, name: &str) -> AccountPatchCall<'a, S> {
        AccountPatchCall {
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
    /// Requests a ticket for creating an account.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn provision_account_ticket(&self, request: GoogleAnalyticsAdminV1alphaProvisionAccountTicketRequest) -> AccountProvisionAccountTicketCall<'a, S> {
        AccountProvisionAccountTicketCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Searches through all changes to an account or its children given the specified set of filters.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `account` - Required. The account resource for which to return change history resources.
    pub fn search_change_history_events(&self, request: GoogleAnalyticsAdminV1alphaSearchChangeHistoryEventsRequest, account: &str) -> AccountSearchChangeHistoryEventCall<'a, S> {
        AccountSearchChangeHistoryEventCall {
            hub: self.hub,
            _request: request,
            _account: account.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *property* resources.
/// It is not used directly, but through the [`GoogleAnalyticsAdmin`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_analyticsadmin1_alpha as analyticsadmin1_alpha;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use analyticsadmin1_alpha::{GoogleAnalyticsAdmin, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = GoogleAnalyticsAdmin::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `acknowledge_user_data_collection(...)`, `conversion_events_create(...)`, `conversion_events_delete(...)`, `conversion_events_get(...)`, `conversion_events_list(...)`, `create(...)`, `custom_dimensions_archive(...)`, `custom_dimensions_create(...)`, `custom_dimensions_get(...)`, `custom_dimensions_list(...)`, `custom_dimensions_patch(...)`, `custom_metrics_archive(...)`, `custom_metrics_create(...)`, `custom_metrics_get(...)`, `custom_metrics_list(...)`, `custom_metrics_patch(...)`, `data_streams_create(...)`, `data_streams_delete(...)`, `data_streams_get(...)`, `data_streams_get_global_site_tag(...)`, `data_streams_list(...)`, `data_streams_measurement_protocol_secrets_create(...)`, `data_streams_measurement_protocol_secrets_delete(...)`, `data_streams_measurement_protocol_secrets_get(...)`, `data_streams_measurement_protocol_secrets_list(...)`, `data_streams_measurement_protocol_secrets_patch(...)`, `data_streams_patch(...)`, `delete(...)`, `display_video360_advertiser_link_proposals_approve(...)`, `display_video360_advertiser_link_proposals_cancel(...)`, `display_video360_advertiser_link_proposals_create(...)`, `display_video360_advertiser_link_proposals_delete(...)`, `display_video360_advertiser_link_proposals_get(...)`, `display_video360_advertiser_link_proposals_list(...)`, `display_video360_advertiser_links_create(...)`, `display_video360_advertiser_links_delete(...)`, `display_video360_advertiser_links_get(...)`, `display_video360_advertiser_links_list(...)`, `display_video360_advertiser_links_patch(...)`, `firebase_links_create(...)`, `firebase_links_delete(...)`, `firebase_links_list(...)`, `get(...)`, `get_data_retention_settings(...)`, `get_google_signals_settings(...)`, `google_ads_links_create(...)`, `google_ads_links_delete(...)`, `google_ads_links_list(...)`, `google_ads_links_patch(...)`, `list(...)`, `patch(...)`, `update_data_retention_settings(...)`, `update_google_signals_settings(...)`, `user_links_audit(...)`, `user_links_batch_create(...)`, `user_links_batch_delete(...)`, `user_links_batch_get(...)`, `user_links_batch_update(...)`, `user_links_create(...)`, `user_links_delete(...)`, `user_links_get(...)`, `user_links_list(...)` and `user_links_patch(...)`
/// // to build up your call.
/// let rb = hub.properties();
/// # }
/// ```
pub struct PropertyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a GoogleAnalyticsAdmin<S>,
}

impl<'a, S> client::MethodsBuilder for PropertyMethods<'a, S> {}

impl<'a, S> PropertyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a conversion event with the specified attributes.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the parent property where this conversion event will be created. Format: properties/123
    pub fn conversion_events_create(&self, request: GoogleAnalyticsAdminV1alphaConversionEvent, parent: &str) -> PropertyConversionEventCreateCall<'a, S> {
        PropertyConversionEventCreateCall {
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
    /// Deletes a conversion event in a property.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the conversion event to delete. Format: properties/{property}/conversionEvents/{conversion_event} Example: "properties/123/conversionEvents/456"
    pub fn conversion_events_delete(&self, name: &str) -> PropertyConversionEventDeleteCall<'a, S> {
        PropertyConversionEventDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieve a single conversion event.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the conversion event to retrieve. Format: properties/{property}/conversionEvents/{conversion_event} Example: "properties/123/conversionEvents/456"
    pub fn conversion_events_get(&self, name: &str) -> PropertyConversionEventGetCall<'a, S> {
        PropertyConversionEventGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of conversion events in the specified parent property. Returns an empty list if no conversion events are found.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the parent property. Example: 'properties/123'
    pub fn conversion_events_list(&self, parent: &str) -> PropertyConversionEventListCall<'a, S> {
        PropertyConversionEventListCall {
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
    /// Archives a CustomDimension on a property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the CustomDimension to archive. Example format: properties/1234/customDimensions/5678
    pub fn custom_dimensions_archive(&self, request: GoogleAnalyticsAdminV1alphaArchiveCustomDimensionRequest, name: &str) -> PropertyCustomDimensionArchiveCall<'a, S> {
        PropertyCustomDimensionArchiveCall {
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
    /// Creates a CustomDimension.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Example format: properties/1234
    pub fn custom_dimensions_create(&self, request: GoogleAnalyticsAdminV1alphaCustomDimension, parent: &str) -> PropertyCustomDimensionCreateCall<'a, S> {
        PropertyCustomDimensionCreateCall {
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
    /// Lookup for a single CustomDimension.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the CustomDimension to get. Example format: properties/1234/customDimensions/5678
    pub fn custom_dimensions_get(&self, name: &str) -> PropertyCustomDimensionGetCall<'a, S> {
        PropertyCustomDimensionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists CustomDimensions on a property.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Example format: properties/1234
    pub fn custom_dimensions_list(&self, parent: &str) -> PropertyCustomDimensionListCall<'a, S> {
        PropertyCustomDimensionListCall {
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
    /// Updates a CustomDimension on a property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name for this CustomDimension resource. Format: properties/{property}/customDimensions/{customDimension}
    pub fn custom_dimensions_patch(&self, request: GoogleAnalyticsAdminV1alphaCustomDimension, name: &str) -> PropertyCustomDimensionPatchCall<'a, S> {
        PropertyCustomDimensionPatchCall {
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
    /// Archives a CustomMetric on a property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the CustomMetric to archive. Example format: properties/1234/customMetrics/5678
    pub fn custom_metrics_archive(&self, request: GoogleAnalyticsAdminV1alphaArchiveCustomMetricRequest, name: &str) -> PropertyCustomMetricArchiveCall<'a, S> {
        PropertyCustomMetricArchiveCall {
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
    /// Creates a CustomMetric.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Example format: properties/1234
    pub fn custom_metrics_create(&self, request: GoogleAnalyticsAdminV1alphaCustomMetric, parent: &str) -> PropertyCustomMetricCreateCall<'a, S> {
        PropertyCustomMetricCreateCall {
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
    /// Lookup for a single CustomMetric.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the CustomMetric to get. Example format: properties/1234/customMetrics/5678
    pub fn custom_metrics_get(&self, name: &str) -> PropertyCustomMetricGetCall<'a, S> {
        PropertyCustomMetricGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists CustomMetrics on a property.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Example format: properties/1234
    pub fn custom_metrics_list(&self, parent: &str) -> PropertyCustomMetricListCall<'a, S> {
        PropertyCustomMetricListCall {
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
    /// Updates a CustomMetric on a property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name for this CustomMetric resource. Format: properties/{property}/customMetrics/{customMetric}
    pub fn custom_metrics_patch(&self, request: GoogleAnalyticsAdminV1alphaCustomMetric, name: &str) -> PropertyCustomMetricPatchCall<'a, S> {
        PropertyCustomMetricPatchCall {
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
    /// Creates a measurement protocol secret.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource where this secret will be created. Format: properties/{property}/dataStreams/{dataStream}
    pub fn data_streams_measurement_protocol_secrets_create(&self, request: GoogleAnalyticsAdminV1alphaMeasurementProtocolSecret, parent: &str) -> PropertyDataStreamMeasurementProtocolSecretCreateCall<'a, S> {
        PropertyDataStreamMeasurementProtocolSecretCreateCall {
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
    /// Deletes target MeasurementProtocolSecret.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the MeasurementProtocolSecret to delete. Format: properties/{property}/dataStreams/{dataStream}/measurementProtocolSecrets/{measurementProtocolSecret}
    pub fn data_streams_measurement_protocol_secrets_delete(&self, name: &str) -> PropertyDataStreamMeasurementProtocolSecretDeleteCall<'a, S> {
        PropertyDataStreamMeasurementProtocolSecretDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lookup for a single "GA4" MeasurementProtocolSecret.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the measurement protocol secret to lookup. Format: properties/{property}/dataStreams/{dataStream}/measurementProtocolSecrets/{measurementProtocolSecret}
    pub fn data_streams_measurement_protocol_secrets_get(&self, name: &str) -> PropertyDataStreamMeasurementProtocolSecretGetCall<'a, S> {
        PropertyDataStreamMeasurementProtocolSecretGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns child MeasurementProtocolSecrets under the specified parent Property.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the parent stream. Format: properties/{property}/dataStreams/{dataStream}/measurementProtocolSecrets
    pub fn data_streams_measurement_protocol_secrets_list(&self, parent: &str) -> PropertyDataStreamMeasurementProtocolSecretListCall<'a, S> {
        PropertyDataStreamMeasurementProtocolSecretListCall {
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
    /// Updates a measurement protocol secret.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name of this secret. This secret may be a child of any type of stream. Format: properties/{property}/webDataStreams/{webDataStream}/measurementProtocolSecrets/{measurementProtocolSecret}
    pub fn data_streams_measurement_protocol_secrets_patch(&self, request: GoogleAnalyticsAdminV1alphaMeasurementProtocolSecret, name: &str) -> PropertyDataStreamMeasurementProtocolSecretPatchCall<'a, S> {
        PropertyDataStreamMeasurementProtocolSecretPatchCall {
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
    /// Creates a DataStream.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Example format: properties/1234
    pub fn data_streams_create(&self, request: GoogleAnalyticsAdminV1alphaDataStream, parent: &str) -> PropertyDataStreamCreateCall<'a, S> {
        PropertyDataStreamCreateCall {
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
    /// Deletes a DataStream on a property.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the DataStream to delete. Example format: properties/1234/dataStreams/5678
    pub fn data_streams_delete(&self, name: &str) -> PropertyDataStreamDeleteCall<'a, S> {
        PropertyDataStreamDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lookup for a single DataStream.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the DataStream to get. Example format: properties/1234/dataStreams/5678
    pub fn data_streams_get(&self, name: &str) -> PropertyDataStreamGetCall<'a, S> {
        PropertyDataStreamGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the Site Tag for the specified web stream. Site Tags are immutable singletons.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the site tag to lookup. Note that site tags are singletons and do not have unique IDs. Format: properties/{property_id}/dataStreams/{stream_id}/globalSiteTag Example: "properties/123/dataStreams/456/globalSiteTag"
    pub fn data_streams_get_global_site_tag(&self, name: &str) -> PropertyDataStreamGetGlobalSiteTagCall<'a, S> {
        PropertyDataStreamGetGlobalSiteTagCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists DataStreams on a property.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Example format: properties/1234
    pub fn data_streams_list(&self, parent: &str) -> PropertyDataStreamListCall<'a, S> {
        PropertyDataStreamListCall {
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
    /// Updates a DataStream on a property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name of this Data Stream. Format: properties/{property_id}/dataStreams/{stream_id} Example: "properties/1000/dataStreams/2000"
    pub fn data_streams_patch(&self, request: GoogleAnalyticsAdminV1alphaDataStream, name: &str) -> PropertyDataStreamPatchCall<'a, S> {
        PropertyDataStreamPatchCall {
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
    /// Approves a DisplayVideo360AdvertiserLinkProposal. The DisplayVideo360AdvertiserLinkProposal will be deleted and a new DisplayVideo360AdvertiserLink will be created.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the DisplayVideo360AdvertiserLinkProposal to approve. Example format: properties/1234/displayVideo360AdvertiserLinkProposals/5678
    pub fn display_video360_advertiser_link_proposals_approve(&self, request: GoogleAnalyticsAdminV1alphaApproveDisplayVideo360AdvertiserLinkProposalRequest, name: &str) -> PropertyDisplayVideo360AdvertiserLinkProposalApproveCall<'a, S> {
        PropertyDisplayVideo360AdvertiserLinkProposalApproveCall {
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
    /// Cancels a DisplayVideo360AdvertiserLinkProposal. Cancelling can mean either: - Declining a proposal initiated from Display & Video 360 - Withdrawing a proposal initiated from Google Analytics After being cancelled, a proposal will eventually be deleted automatically.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the DisplayVideo360AdvertiserLinkProposal to cancel. Example format: properties/1234/displayVideo360AdvertiserLinkProposals/5678
    pub fn display_video360_advertiser_link_proposals_cancel(&self, request: GoogleAnalyticsAdminV1alphaCancelDisplayVideo360AdvertiserLinkProposalRequest, name: &str) -> PropertyDisplayVideo360AdvertiserLinkProposalCancelCall<'a, S> {
        PropertyDisplayVideo360AdvertiserLinkProposalCancelCall {
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
    /// Creates a DisplayVideo360AdvertiserLinkProposal.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Example format: properties/1234
    pub fn display_video360_advertiser_link_proposals_create(&self, request: GoogleAnalyticsAdminV1alphaDisplayVideo360AdvertiserLinkProposal, parent: &str) -> PropertyDisplayVideo360AdvertiserLinkProposalCreateCall<'a, S> {
        PropertyDisplayVideo360AdvertiserLinkProposalCreateCall {
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
    /// Deletes a DisplayVideo360AdvertiserLinkProposal on a property. This can only be used on cancelled proposals.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the DisplayVideo360AdvertiserLinkProposal to delete. Example format: properties/1234/displayVideo360AdvertiserLinkProposals/5678
    pub fn display_video360_advertiser_link_proposals_delete(&self, name: &str) -> PropertyDisplayVideo360AdvertiserLinkProposalDeleteCall<'a, S> {
        PropertyDisplayVideo360AdvertiserLinkProposalDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lookup for a single DisplayVideo360AdvertiserLinkProposal.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the DisplayVideo360AdvertiserLinkProposal to get. Example format: properties/1234/displayVideo360AdvertiserLinkProposals/5678
    pub fn display_video360_advertiser_link_proposals_get(&self, name: &str) -> PropertyDisplayVideo360AdvertiserLinkProposalGetCall<'a, S> {
        PropertyDisplayVideo360AdvertiserLinkProposalGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists DisplayVideo360AdvertiserLinkProposals on a property.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Example format: properties/1234
    pub fn display_video360_advertiser_link_proposals_list(&self, parent: &str) -> PropertyDisplayVideo360AdvertiserLinkProposalListCall<'a, S> {
        PropertyDisplayVideo360AdvertiserLinkProposalListCall {
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
    /// Creates a DisplayVideo360AdvertiserLink. This can only be utilized by users who have proper authorization both on the Google Analytics property and on the Display & Video 360 advertiser. Users who do not have access to the Display & Video 360 advertiser should instead seek to create a DisplayVideo360LinkProposal.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Example format: properties/1234
    pub fn display_video360_advertiser_links_create(&self, request: GoogleAnalyticsAdminV1alphaDisplayVideo360AdvertiserLink, parent: &str) -> PropertyDisplayVideo360AdvertiserLinkCreateCall<'a, S> {
        PropertyDisplayVideo360AdvertiserLinkCreateCall {
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
    /// Deletes a DisplayVideo360AdvertiserLink on a property.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the DisplayVideo360AdvertiserLink to delete. Example format: properties/1234/displayVideo360AdvertiserLinks/5678
    pub fn display_video360_advertiser_links_delete(&self, name: &str) -> PropertyDisplayVideo360AdvertiserLinkDeleteCall<'a, S> {
        PropertyDisplayVideo360AdvertiserLinkDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Look up a single DisplayVideo360AdvertiserLink
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the DisplayVideo360AdvertiserLink to get. Example format: properties/1234/displayVideo360AdvertiserLink/5678
    pub fn display_video360_advertiser_links_get(&self, name: &str) -> PropertyDisplayVideo360AdvertiserLinkGetCall<'a, S> {
        PropertyDisplayVideo360AdvertiserLinkGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all DisplayVideo360AdvertiserLinks on a property.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Example format: properties/1234
    pub fn display_video360_advertiser_links_list(&self, parent: &str) -> PropertyDisplayVideo360AdvertiserLinkListCall<'a, S> {
        PropertyDisplayVideo360AdvertiserLinkListCall {
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
    /// Updates a DisplayVideo360AdvertiserLink on a property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name for this DisplayVideo360AdvertiserLink resource. Format: properties/{propertyId}/displayVideo360AdvertiserLinks/{linkId} Note: linkId is not the Display & Video 360 Advertiser ID
    pub fn display_video360_advertiser_links_patch(&self, request: GoogleAnalyticsAdminV1alphaDisplayVideo360AdvertiserLink, name: &str) -> PropertyDisplayVideo360AdvertiserLinkPatchCall<'a, S> {
        PropertyDisplayVideo360AdvertiserLinkPatchCall {
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
    /// Creates a FirebaseLink. Properties can have at most one FirebaseLink.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Format: properties/{property_id} Example: properties/1234
    pub fn firebase_links_create(&self, request: GoogleAnalyticsAdminV1alphaFirebaseLink, parent: &str) -> PropertyFirebaseLinkCreateCall<'a, S> {
        PropertyFirebaseLinkCreateCall {
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
    /// Deletes a FirebaseLink on a property
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Format: properties/{property_id}/firebaseLinks/{firebase_link_id} Example: properties/1234/firebaseLinks/5678
    pub fn firebase_links_delete(&self, name: &str) -> PropertyFirebaseLinkDeleteCall<'a, S> {
        PropertyFirebaseLinkDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists FirebaseLinks on a property. Properties can have at most one FirebaseLink.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Format: properties/{property_id} Example: properties/1234
    pub fn firebase_links_list(&self, parent: &str) -> PropertyFirebaseLinkListCall<'a, S> {
        PropertyFirebaseLinkListCall {
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
    /// Creates a GoogleAdsLink.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Example format: properties/1234
    pub fn google_ads_links_create(&self, request: GoogleAnalyticsAdminV1alphaGoogleAdsLink, parent: &str) -> PropertyGoogleAdsLinkCreateCall<'a, S> {
        PropertyGoogleAdsLinkCreateCall {
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
    /// Deletes a GoogleAdsLink on a property
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Example format: properties/1234/googleAdsLinks/5678
    pub fn google_ads_links_delete(&self, name: &str) -> PropertyGoogleAdsLinkDeleteCall<'a, S> {
        PropertyGoogleAdsLinkDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists GoogleAdsLinks on a property.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Example format: properties/1234
    pub fn google_ads_links_list(&self, parent: &str) -> PropertyGoogleAdsLinkListCall<'a, S> {
        PropertyGoogleAdsLinkListCall {
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
    /// Updates a GoogleAdsLink on a property
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Format: properties/{propertyId}/googleAdsLinks/{googleAdsLinkId} Note: googleAdsLinkId is not the Google Ads customer ID.
    pub fn google_ads_links_patch(&self, request: GoogleAnalyticsAdminV1alphaGoogleAdsLink, name: &str) -> PropertyGoogleAdsLinkPatchCall<'a, S> {
        PropertyGoogleAdsLinkPatchCall {
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
    /// Lists all user links on an account or property, including implicit ones that come from effective permissions granted by groups or organization admin roles. If a returned user link does not have direct permissions, they cannot be removed from the account or property directly with the DeleteUserLink command. They have to be removed from the group/etc that gives them permissions, which is currently only usable/discoverable in the GA or GMP UIs.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Example format: accounts/1234
    pub fn user_links_audit(&self, request: GoogleAnalyticsAdminV1alphaAuditUserLinksRequest, parent: &str) -> PropertyUserLinkAuditCall<'a, S> {
        PropertyUserLinkAuditCall {
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
    /// Creates information about multiple users' links to an account or property. This method is transactional. If any UserLink cannot be created, none of the UserLinks will be created.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The account or property that all user links in the request are for. This field is required. The parent field in the CreateUserLinkRequest messages must either be empty or match this field. Example format: accounts/1234
    pub fn user_links_batch_create(&self, request: GoogleAnalyticsAdminV1alphaBatchCreateUserLinksRequest, parent: &str) -> PropertyUserLinkBatchCreateCall<'a, S> {
        PropertyUserLinkBatchCreateCall {
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
    /// Deletes information about multiple users' links to an account or property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The account or property that all user links in the request are for. The parent of all values for user link names to delete must match this field. Example format: accounts/1234
    pub fn user_links_batch_delete(&self, request: GoogleAnalyticsAdminV1alphaBatchDeleteUserLinksRequest, parent: &str) -> PropertyUserLinkBatchDeleteCall<'a, S> {
        PropertyUserLinkBatchDeleteCall {
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
    /// Gets information about multiple users' links to an account or property.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The account or property that all user links in the request are for. The parent of all provided values for the 'names' field must match this field. Example format: accounts/1234
    pub fn user_links_batch_get(&self, parent: &str) -> PropertyUserLinkBatchGetCall<'a, S> {
        PropertyUserLinkBatchGetCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _names: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates information about multiple users' links to an account or property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The account or property that all user links in the request are for. The parent field in the UpdateUserLinkRequest messages must either be empty or match this field. Example format: accounts/1234
    pub fn user_links_batch_update(&self, request: GoogleAnalyticsAdminV1alphaBatchUpdateUserLinksRequest, parent: &str) -> PropertyUserLinkBatchUpdateCall<'a, S> {
        PropertyUserLinkBatchUpdateCall {
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
    /// Creates a user link on an account or property. If the user with the specified email already has permissions on the account or property, then the user's existing permissions will be unioned with the permissions specified in the new UserLink.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Example format: accounts/1234
    pub fn user_links_create(&self, request: GoogleAnalyticsAdminV1alphaUserLink, parent: &str) -> PropertyUserLinkCreateCall<'a, S> {
        PropertyUserLinkCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _notify_new_user: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a user link on an account or property.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Example format: accounts/1234/userLinks/5678
    pub fn user_links_delete(&self, name: &str) -> PropertyUserLinkDeleteCall<'a, S> {
        PropertyUserLinkDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about a user's link to an account or property.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Example format: accounts/1234/userLinks/5678
    pub fn user_links_get(&self, name: &str) -> PropertyUserLinkGetCall<'a, S> {
        PropertyUserLinkGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all user links on an account or property.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Example format: accounts/1234
    pub fn user_links_list(&self, parent: &str) -> PropertyUserLinkListCall<'a, S> {
        PropertyUserLinkListCall {
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
    /// Updates a user link on an account or property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Example format: properties/1234/userLinks/5678
    pub fn user_links_patch(&self, request: GoogleAnalyticsAdminV1alphaUserLink, name: &str) -> PropertyUserLinkPatchCall<'a, S> {
        PropertyUserLinkPatchCall {
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
    /// Acknowledges the terms of user data collection for the specified property. This acknowledgement must be completed (either in the Google Analytics UI or via this API) before MeasurementProtocolSecret resources may be created.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `property` - Required. The property for which to acknowledge user data collection.
    pub fn acknowledge_user_data_collection(&self, request: GoogleAnalyticsAdminV1alphaAcknowledgeUserDataCollectionRequest, property: &str) -> PropertyAcknowledgeUserDataCollectionCall<'a, S> {
        PropertyAcknowledgeUserDataCollectionCall {
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
    /// Creates an "GA4" property with the specified location and attributes.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: GoogleAnalyticsAdminV1alphaProperty) -> PropertyCreateCall<'a, S> {
        PropertyCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Marks target Property as soft-deleted (ie: "trashed") and returns it. This API does not have a method to restore soft-deleted properties. However, they can be restored using the Trash Can UI. If the properties are not restored before the expiration time, the Property and all child resources (eg: GoogleAdsLinks, Streams, UserLinks) will be permanently purged. https://support.google.com/analytics/answer/6154772 Returns an error if the target is not found, or is not an GA4 Property.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the Property to soft-delete. Format: properties/{property_id} Example: "properties/1000"
    pub fn delete(&self, name: &str) -> PropertyDeleteCall<'a, S> {
        PropertyDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lookup for a single "GA4" Property.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the property to lookup. Format: properties/{property_id} Example: "properties/1000"
    pub fn get(&self, name: &str) -> PropertyGetCall<'a, S> {
        PropertyGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the singleton data retention settings for this property.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the settings to lookup. Format: properties/{property}/dataRetentionSettings Example: "properties/1000/dataRetentionSettings"
    pub fn get_data_retention_settings(&self, name: &str) -> PropertyGetDataRetentionSettingCall<'a, S> {
        PropertyGetDataRetentionSettingCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lookup for Google Signals settings for a property.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the google signals settings to retrieve. Format: properties/{property}/googleSignalsSettings
    pub fn get_google_signals_settings(&self, name: &str) -> PropertyGetGoogleSignalsSettingCall<'a, S> {
        PropertyGetGoogleSignalsSettingCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns child Properties under the specified parent Account. Only "GA4" properties will be returned. Properties will be excluded if the caller does not have access. Soft-deleted (ie: "trashed") properties are excluded by default. Returns an empty list if no relevant properties are found.
    pub fn list(&self) -> PropertyListCall<'a, S> {
        PropertyListCall {
            hub: self.hub,
            _show_deleted: Default::default(),
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
    /// Updates a property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name of this property. Format: properties/{property_id} Example: "properties/1000"
    pub fn patch(&self, request: GoogleAnalyticsAdminV1alphaProperty, name: &str) -> PropertyPatchCall<'a, S> {
        PropertyPatchCall {
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
    /// Updates the singleton data retention settings for this property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name for this DataRetentionSetting resource. Format: properties/{property}/dataRetentionSettings
    pub fn update_data_retention_settings(&self, request: GoogleAnalyticsAdminV1alphaDataRetentionSettings, name: &str) -> PropertyUpdateDataRetentionSettingCall<'a, S> {
        PropertyUpdateDataRetentionSettingCall {
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
    /// Updates Google Signals settings for a property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name of this setting. Format: properties/{property_id}/googleSignalsSettings Example: "properties/1000/googleSignalsSettings"
    pub fn update_google_signals_settings(&self, request: GoogleAnalyticsAdminV1alphaGoogleSignalsSettings, name: &str) -> PropertyUpdateGoogleSignalsSettingCall<'a, S> {
        PropertyUpdateGoogleSignalsSettingCall {
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



