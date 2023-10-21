use super::*;
/// A builder providing access to all methods supported on *bidder* resources.
/// It is not used directly, but through the [`RealTimeBidding`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_realtimebidding1 as realtimebidding1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use realtimebidding1::{RealTimeBidding, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = RealTimeBidding::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `creatives_list(...)`, `creatives_watch(...)`, `endpoints_get(...)`, `endpoints_list(...)`, `endpoints_patch(...)`, `get(...)`, `list(...)`, `pretargeting_configs_activate(...)`, `pretargeting_configs_add_targeted_apps(...)`, `pretargeting_configs_add_targeted_publishers(...)`, `pretargeting_configs_add_targeted_sites(...)`, `pretargeting_configs_create(...)`, `pretargeting_configs_delete(...)`, `pretargeting_configs_get(...)`, `pretargeting_configs_list(...)`, `pretargeting_configs_patch(...)`, `pretargeting_configs_remove_targeted_apps(...)`, `pretargeting_configs_remove_targeted_publishers(...)`, `pretargeting_configs_remove_targeted_sites(...)`, `pretargeting_configs_suspend(...)`, `publisher_connections_batch_approve(...)`, `publisher_connections_batch_reject(...)`, `publisher_connections_get(...)` and `publisher_connections_list(...)`
/// // to build up your call.
/// let rb = hub.bidders();
/// # }
/// ```
pub struct BidderMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a RealTimeBidding<S>,
}

impl<'a, S> client::MethodsBuilder for BidderMethods<'a, S> {}

impl<'a, S> BidderMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists creatives as they are at the time of the initial request. This call may take multiple hours to complete. For large, paginated requests, this method returns a snapshot of creatives at the time of request for the first page. `lastStatusUpdate` and `creativeServingDecision` may be outdated for creatives on sequential pages. We recommend [Google Cloud Pub/Sub](https://developers.google.com//cloud.google.com/pubsub/docs/overview) to view the latest status.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the parent buyer that owns the creatives. The pattern for this resource is either `buyers/{buyerAccountId}` or `bidders/{bidderAccountId}`. For `buyers/{buyerAccountId}`, the `buyerAccountId` can be one of the following: 1. The ID of the buyer that is accessing their own creatives. 2. The ID of the child seat buyer under a bidder account. So for listing creatives pertaining to the child seat buyer (`456`) under bidder account (`123`), you would use the pattern: `buyers/456`. 3. The ID of the bidder itself. So for listing creatives pertaining to bidder (`123`), you would use `buyers/123`. If you want to access all creatives pertaining to both the bidder and all of its child seat accounts, you would use `bidders/{bidderAccountId}`, for example, for all creatives pertaining to bidder (`123`), use `bidders/123`.
    pub fn creatives_list(&self, parent: &str) -> BidderCreativeListCall<'a, S> {
        BidderCreativeListCall {
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
    /// Watches all creatives pertaining to a bidder. It is sufficient to invoke this endpoint once per bidder. A Pub/Sub topic will be created and notifications will be pushed to the topic when any of the bidder's creatives change status. All of the bidder's service accounts will have access to read from the topic. Subsequent invocations of this method will return the existing Pub/Sub configuration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. To watch all creatives pertaining to the bidder and all its child seat accounts, the bidder must follow the pattern `bidders/{bidderAccountId}`.
    pub fn creatives_watch(&self, request: WatchCreativesRequest, parent: &str) -> BidderCreativeWatchCall<'a, S> {
        BidderCreativeWatchCall {
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
    /// Gets a bidder endpoint by its name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the bidder endpoint to get. Format: `bidders/{bidderAccountId}/endpoints/{endpointId}`
    pub fn endpoints_get(&self, name: &str) -> BidderEndpointGetCall<'a, S> {
        BidderEndpointGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the bidder's endpoints.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the bidder whose endpoints will be listed. Format: `bidders/{bidderAccountId}`
    pub fn endpoints_list(&self, parent: &str) -> BidderEndpointListCall<'a, S> {
        BidderEndpointListCall {
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
    /// Updates a bidder's endpoint.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Name of the endpoint resource that must follow the pattern `bidders/{bidderAccountId}/endpoints/{endpointId}`, where {bidderAccountId} is the account ID of the bidder who operates this endpoint, and {endpointId} is a unique ID assigned by the server.
    pub fn endpoints_patch(&self, request: Endpoint, name: &str) -> BidderEndpointPatchCall<'a, S> {
        BidderEndpointPatchCall {
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
    /// Activates a pretargeting configuration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the pretargeting configuration. Format: bidders/{bidderAccountId}/pretargetingConfig/{configId}
    pub fn pretargeting_configs_activate(&self, request: ActivatePretargetingConfigRequest, name: &str) -> BidderPretargetingConfigActivateCall<'a, S> {
        BidderPretargetingConfigActivateCall {
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
    /// Adds targeted apps to the pretargeting configuration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `pretargetingConfig` - Required. The name of the pretargeting configuration. Format: bidders/{bidderAccountId}/pretargetingConfig/{configId}
    pub fn pretargeting_configs_add_targeted_apps(&self, request: AddTargetedAppsRequest, pretargeting_config: &str) -> BidderPretargetingConfigAddTargetedAppCall<'a, S> {
        BidderPretargetingConfigAddTargetedAppCall {
            hub: self.hub,
            _request: request,
            _pretargeting_config: pretargeting_config.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds targeted publishers to the pretargeting config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `pretargetingConfig` - Required. The name of the pretargeting configuration. Format: bidders/{bidderAccountId}/pretargetingConfig/{configId}
    pub fn pretargeting_configs_add_targeted_publishers(&self, request: AddTargetedPublishersRequest, pretargeting_config: &str) -> BidderPretargetingConfigAddTargetedPublisherCall<'a, S> {
        BidderPretargetingConfigAddTargetedPublisherCall {
            hub: self.hub,
            _request: request,
            _pretargeting_config: pretargeting_config.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds targeted sites to the pretargeting configuration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `pretargetingConfig` - Required. The name of the pretargeting configuration. Format: bidders/{bidderAccountId}/pretargetingConfig/{configId}
    pub fn pretargeting_configs_add_targeted_sites(&self, request: AddTargetedSitesRequest, pretargeting_config: &str) -> BidderPretargetingConfigAddTargetedSiteCall<'a, S> {
        BidderPretargetingConfigAddTargetedSiteCall {
            hub: self.hub,
            _request: request,
            _pretargeting_config: pretargeting_config.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a pretargeting configuration. A pretargeting configuration's state (PretargetingConfig.state) is active upon creation, and it will start to affect traffic shortly after. A bidder may create a maximum of 10 pretargeting configurations. Attempts to exceed this maximum results in a 400 bad request error.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the bidder to create the pretargeting configuration for. Format: bidders/{bidderAccountId}
    pub fn pretargeting_configs_create(&self, request: PretargetingConfig, parent: &str) -> BidderPretargetingConfigCreateCall<'a, S> {
        BidderPretargetingConfigCreateCall {
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
    /// Deletes a pretargeting configuration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the pretargeting configuration to delete. Format: bidders/{bidderAccountId}/pretargetingConfig/{configId}
    pub fn pretargeting_configs_delete(&self, name: &str) -> BidderPretargetingConfigDeleteCall<'a, S> {
        BidderPretargetingConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a pretargeting configuration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the pretargeting configuration to get. Format: bidders/{bidderAccountId}/pretargetingConfig/{configId}
    pub fn pretargeting_configs_get(&self, name: &str) -> BidderPretargetingConfigGetCall<'a, S> {
        BidderPretargetingConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all pretargeting configurations for a single bidder.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the bidder whose pretargeting configurations will be listed. Format: bidders/{bidderAccountId}
    pub fn pretargeting_configs_list(&self, parent: &str) -> BidderPretargetingConfigListCall<'a, S> {
        BidderPretargetingConfigListCall {
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
    /// Updates a pretargeting configuration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Name of the pretargeting configuration that must follow the pattern `bidders/{bidder_account_id}/pretargetingConfigs/{config_id}`
    pub fn pretargeting_configs_patch(&self, request: PretargetingConfig, name: &str) -> BidderPretargetingConfigPatchCall<'a, S> {
        BidderPretargetingConfigPatchCall {
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
    /// Removes targeted apps from the pretargeting configuration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `pretargetingConfig` - Required. The name of the pretargeting configuration. Format: bidders/{bidderAccountId}/pretargetingConfig/{configId}
    pub fn pretargeting_configs_remove_targeted_apps(&self, request: RemoveTargetedAppsRequest, pretargeting_config: &str) -> BidderPretargetingConfigRemoveTargetedAppCall<'a, S> {
        BidderPretargetingConfigRemoveTargetedAppCall {
            hub: self.hub,
            _request: request,
            _pretargeting_config: pretargeting_config.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes targeted publishers from the pretargeting config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `pretargetingConfig` - Required. The name of the pretargeting configuration. Format: bidders/{bidderAccountId}/pretargetingConfig/{configId}
    pub fn pretargeting_configs_remove_targeted_publishers(&self, request: RemoveTargetedPublishersRequest, pretargeting_config: &str) -> BidderPretargetingConfigRemoveTargetedPublisherCall<'a, S> {
        BidderPretargetingConfigRemoveTargetedPublisherCall {
            hub: self.hub,
            _request: request,
            _pretargeting_config: pretargeting_config.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes targeted sites from the pretargeting configuration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `pretargetingConfig` - Required. The name of the pretargeting configuration. Format: bidders/{bidderAccountId}/pretargetingConfig/{configId}
    pub fn pretargeting_configs_remove_targeted_sites(&self, request: RemoveTargetedSitesRequest, pretargeting_config: &str) -> BidderPretargetingConfigRemoveTargetedSiteCall<'a, S> {
        BidderPretargetingConfigRemoveTargetedSiteCall {
            hub: self.hub,
            _request: request,
            _pretargeting_config: pretargeting_config.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Suspends a pretargeting configuration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the pretargeting configuration. Format: bidders/{bidderAccountId}/pretargetingConfig/{configId}
    pub fn pretargeting_configs_suspend(&self, request: SuspendPretargetingConfigRequest, name: &str) -> BidderPretargetingConfigSuspendCall<'a, S> {
        BidderPretargetingConfigSuspendCall {
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
    /// Batch approves multiple publisher connections.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The bidder for whom publisher connections will be approved. Format: `bidders/{bidder}` where `{bidder}` is the account ID of the bidder.
    pub fn publisher_connections_batch_approve(&self, request: BatchApprovePublisherConnectionsRequest, parent: &str) -> BidderPublisherConnectionBatchApproveCall<'a, S> {
        BidderPublisherConnectionBatchApproveCall {
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
    /// Batch rejects multiple publisher connections.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The bidder for whom publisher connections will be rejected. Format: `bidders/{bidder}` where `{bidder}` is the account ID of the bidder.
    pub fn publisher_connections_batch_reject(&self, request: BatchRejectPublisherConnectionsRequest, parent: &str) -> BidderPublisherConnectionBatchRejectCall<'a, S> {
        BidderPublisherConnectionBatchRejectCall {
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
    /// Gets a publisher connection.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the publisher whose connection information is to be retrieved. In the pattern `bidders/{bidder}/publisherConnections/{publisher}` where `{bidder}` is the account ID of the bidder, and `{publisher}` is the ads.txt/app-ads.txt publisher ID. See publisherConnection.name.
    pub fn publisher_connections_get(&self, name: &str) -> BidderPublisherConnectionGetCall<'a, S> {
        BidderPublisherConnectionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists publisher connections for a given bidder.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the bidder for which publishers have initiated connections. The pattern for this resource is `bidders/{bidder}` where `{bidder}` represents the account ID of the bidder.
    pub fn publisher_connections_list(&self, parent: &str) -> BidderPublisherConnectionListCall<'a, S> {
        BidderPublisherConnectionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
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
    /// Gets a bidder account by its name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the bidder to get. Format: `bidders/{bidderAccountId}`
    pub fn get(&self, name: &str) -> BidderGetCall<'a, S> {
        BidderGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the bidder accounts that belong to the caller.
    pub fn list(&self) -> BidderListCall<'a, S> {
        BidderListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *buyer* resources.
/// It is not used directly, but through the [`RealTimeBidding`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_realtimebidding1 as realtimebidding1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use realtimebidding1::{RealTimeBidding, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = RealTimeBidding::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `creatives_create(...)`, `creatives_get(...)`, `creatives_list(...)`, `creatives_patch(...)`, `get(...)`, `get_remarketing_tag(...)`, `list(...)`, `user_lists_close(...)`, `user_lists_create(...)`, `user_lists_get(...)`, `user_lists_get_remarketing_tag(...)`, `user_lists_list(...)`, `user_lists_open(...)` and `user_lists_update(...)`
/// // to build up your call.
/// let rb = hub.buyers();
/// # }
/// ```
pub struct BuyerMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a RealTimeBidding<S>,
}

impl<'a, S> client::MethodsBuilder for BuyerMethods<'a, S> {}

impl<'a, S> BuyerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a creative.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent buyer that the new creative belongs to that must follow the pattern `buyers/{buyerAccountId}`, where `{buyerAccountId}` represents the account ID of the buyer who owns a creative. For a bidder accessing creatives on behalf of a child seat buyer, `{buyerAccountId}` should represent the account ID of the child seat buyer.
    pub fn creatives_create(&self, request: Creative, parent: &str) -> BuyerCreativeCreateCall<'a, S> {
        BuyerCreativeCreateCall {
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
    /// Gets a creative.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the creative to retrieve. See creative.name.
    pub fn creatives_get(&self, name: &str) -> BuyerCreativeGetCall<'a, S> {
        BuyerCreativeGetCall {
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
    /// Lists creatives as they are at the time of the initial request. This call may take multiple hours to complete. For large, paginated requests, this method returns a snapshot of creatives at the time of request for the first page. `lastStatusUpdate` and `creativeServingDecision` may be outdated for creatives on sequential pages. We recommend [Google Cloud Pub/Sub](https://developers.google.com//cloud.google.com/pubsub/docs/overview) to view the latest status.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the parent buyer that owns the creatives. The pattern for this resource is either `buyers/{buyerAccountId}` or `bidders/{bidderAccountId}`. For `buyers/{buyerAccountId}`, the `buyerAccountId` can be one of the following: 1. The ID of the buyer that is accessing their own creatives. 2. The ID of the child seat buyer under a bidder account. So for listing creatives pertaining to the child seat buyer (`456`) under bidder account (`123`), you would use the pattern: `buyers/456`. 3. The ID of the bidder itself. So for listing creatives pertaining to bidder (`123`), you would use `buyers/123`. If you want to access all creatives pertaining to both the bidder and all of its child seat accounts, you would use `bidders/{bidderAccountId}`, for example, for all creatives pertaining to bidder (`123`), use `bidders/123`.
    pub fn creatives_list(&self, parent: &str) -> BuyerCreativeListCall<'a, S> {
        BuyerCreativeListCall {
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
    /// Updates a creative.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Name of the creative. Follows the pattern `buyers/{buyer}/creatives/{creative}`, where `{buyer}` represents the account ID of the buyer who owns the creative, and `{creative}` is the buyer-specific creative ID that references this creative in the bid response.
    pub fn creatives_patch(&self, request: Creative, name: &str) -> BuyerCreativePatchCall<'a, S> {
        BuyerCreativePatchCall {
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
    /// Change the status of a user list to CLOSED. This prevents new users from being added to the user list.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the user list to close. See UserList.name
    pub fn user_lists_close(&self, request: CloseUserListRequest, name: &str) -> BuyerUserListCloseCall<'a, S> {
        BuyerUserListCloseCall {
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
    /// Create a new user list.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent buyer of the user list to be retrieved that must follow the pattern `buyers/{buyerAccountId}`, where `{buyerAccountId}` represents the account ID of the buyer who owns user lists. For a bidder accessing user lists on behalf of a child seat buyer , `{buyerAccountId}` should represent the account ID of the child seat buyer.
    pub fn user_lists_create(&self, request: UserList, parent: &str) -> BuyerUserListCreateCall<'a, S> {
        BuyerUserListCreateCall {
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
    /// Gets a user list by its name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the user list to be retrieved. See UserList.name.
    pub fn user_lists_get(&self, name: &str) -> BuyerUserListGetCall<'a, S> {
        BuyerUserListGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets remarketing tag for a buyer. A remarketing tag is a piece of JavaScript code that can be placed on a web page. When a user visits a page containing a remarketing tag, Google adds the user to a user list.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. To fetch remarketing tag for an account, name must follow the pattern `buyers/{accountId}` where `{accountId}` represents ID of a buyer that owns the remarketing tag. For a bidder accessing remarketing tag on behalf of a child seat buyer, `{accountId}` should represent the ID of the child seat buyer. To fetch remarketing tag for a specific user list, name must follow the pattern `buyers/{accountId}/userLists/{userListId}`. See UserList.name.
    pub fn user_lists_get_remarketing_tag(&self, name: &str) -> BuyerUserListGetRemarketingTagCall<'a, S> {
        BuyerUserListGetRemarketingTagCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the user lists visible to the current user.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the parent buyer for the user lists to be returned that must follow the pattern `buyers/{buyerAccountId}`, where `{buyerAccountId}` represents the account ID of the buyer who owns user lists. For a bidder accessing user lists on behalf of a child seat buyer , `{buyerAccountId}` should represent the account ID of the child seat buyer.
    pub fn user_lists_list(&self, parent: &str) -> BuyerUserListListCall<'a, S> {
        BuyerUserListListCall {
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
    /// Change the status of a user list to OPEN. This allows new users to be added to the user list.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the user list to open. See UserList.name
    pub fn user_lists_open(&self, request: OpenUserListRequest, name: &str) -> BuyerUserListOpenCall<'a, S> {
        BuyerUserListOpenCall {
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
    /// Update the given user list. Only user lists with URLRestrictions can be updated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Name of the user list that must follow the pattern `buyers/{buyer}/userLists/{user_list}`, where `{buyer}` represents the account ID of the buyer who owns the user list. For a bidder accessing user lists on behalf of a child seat buyer, `{buyer}` represents the account ID of the child seat buyer. `{user_list}` is an int64 identifier assigned by Google to uniquely identify a user list.
    pub fn user_lists_update(&self, request: UserList, name: &str) -> BuyerUserListUpdateCall<'a, S> {
        BuyerUserListUpdateCall {
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
    /// Gets a buyer account by its name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the buyer to get. Format: `buyers/{buyerId}`
    pub fn get(&self, name: &str) -> BuyerGetCall<'a, S> {
        BuyerGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets remarketing tag for a buyer. A remarketing tag is a piece of JavaScript code that can be placed on a web page. When a user visits a page containing a remarketing tag, Google adds the user to a user list.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. To fetch remarketing tag for an account, name must follow the pattern `buyers/{accountId}` where `{accountId}` represents ID of a buyer that owns the remarketing tag. For a bidder accessing remarketing tag on behalf of a child seat buyer, `{accountId}` should represent the ID of the child seat buyer. To fetch remarketing tag for a specific user list, name must follow the pattern `buyers/{accountId}/userLists/{userListId}`. See UserList.name.
    pub fn get_remarketing_tag(&self, name: &str) -> BuyerGetRemarketingTagCall<'a, S> {
        BuyerGetRemarketingTagCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all buyer account information the calling buyer user or service account is permissioned to manage.
    pub fn list(&self) -> BuyerListCall<'a, S> {
        BuyerListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



