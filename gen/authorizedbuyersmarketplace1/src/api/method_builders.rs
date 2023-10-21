use super::*;
/// A builder providing access to all methods supported on *bidder* resources.
/// It is not used directly, but through the [`AuthorizedBuyersMarketplace`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_authorizedbuyersmarketplace1 as authorizedbuyersmarketplace1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use authorizedbuyersmarketplace1::{AuthorizedBuyersMarketplace, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AuthorizedBuyersMarketplace::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `finalized_deals_list(...)`
/// // to build up your call.
/// let rb = hub.bidders();
/// # }
/// ```
pub struct BidderMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AuthorizedBuyersMarketplace<S>,
}

impl<'a, S> client::MethodsBuilder for BidderMethods<'a, S> {}

impl<'a, S> BidderMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists finalized deals. Use the URL path "/v1/buyers/{accountId}/finalizedDeals" to list finalized deals for the current buyer and its clients. Bidders can use the URL path "/v1/bidders/{accountId}/finalizedDeals" to list finalized deals for the bidder, its buyers and all their clients.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The buyer to list the finalized deals for, in the format: `buyers/{accountId}`. When used to list finalized deals for a bidder, its buyers and clients, in the format `bidders/{accountId}`.
    pub fn finalized_deals_list(&self, parent: &str) -> BidderFinalizedDealListCall<'a, S> {
        BidderFinalizedDealListCall {
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
}



/// A builder providing access to all methods supported on *buyer* resources.
/// It is not used directly, but through the [`AuthorizedBuyersMarketplace`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_authorizedbuyersmarketplace1 as authorizedbuyersmarketplace1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use authorizedbuyersmarketplace1::{AuthorizedBuyersMarketplace, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AuthorizedBuyersMarketplace::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `auction_packages_get(...)`, `auction_packages_list(...)`, `auction_packages_subscribe(...)`, `auction_packages_subscribe_clients(...)`, `auction_packages_unsubscribe(...)`, `auction_packages_unsubscribe_clients(...)`, `clients_activate(...)`, `clients_create(...)`, `clients_deactivate(...)`, `clients_get(...)`, `clients_list(...)`, `clients_patch(...)`, `clients_users_activate(...)`, `clients_users_create(...)`, `clients_users_deactivate(...)`, `clients_users_delete(...)`, `clients_users_get(...)`, `clients_users_list(...)`, `finalized_deals_add_creative(...)`, `finalized_deals_get(...)`, `finalized_deals_list(...)`, `finalized_deals_pause(...)`, `finalized_deals_resume(...)`, `finalized_deals_set_ready_to_serve(...)`, `proposals_accept(...)`, `proposals_add_note(...)`, `proposals_cancel_negotiation(...)`, `proposals_deals_batch_update(...)`, `proposals_deals_get(...)`, `proposals_deals_list(...)`, `proposals_deals_patch(...)`, `proposals_get(...)`, `proposals_list(...)`, `proposals_patch(...)`, `proposals_send_rfp(...)`, `publisher_profiles_get(...)` and `publisher_profiles_list(...)`
/// // to build up your call.
/// let rb = hub.buyers();
/// # }
/// ```
pub struct BuyerMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AuthorizedBuyersMarketplace<S>,
}

impl<'a, S> client::MethodsBuilder for BuyerMethods<'a, S> {}

impl<'a, S> BuyerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an auction package given its name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of auction package to get. Format: `buyers/{accountId}/auctionPackages/{auctionPackageId}`
    pub fn auction_packages_get(&self, name: &str) -> BuyerAuctionPackageGetCall<'a, S> {
        BuyerAuctionPackageGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the auction packages subscribed by a buyer and its clients.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the parent buyer that can access the auction package. Format: `buyers/{accountId}`
    pub fn auction_packages_list(&self, parent: &str) -> BuyerAuctionPackageListCall<'a, S> {
        BuyerAuctionPackageListCall {
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
    /// Subscribe to the auction package for the specified buyer. Once subscribed, the bidder will receive a call out for inventory matching the auction package targeting criteria with the auction package deal ID and the specified buyer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the auction package. Format: `buyers/{accountId}/auctionPackages/{auctionPackageId}`
    pub fn auction_packages_subscribe(&self, request: SubscribeAuctionPackageRequest, name: &str) -> BuyerAuctionPackageSubscribeCall<'a, S> {
        BuyerAuctionPackageSubscribeCall {
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
    /// Subscribe the specified clients of the buyer to the auction package. If a client in the list does not belong to the buyer, an error response will be returned, and all of the following clients in the list will not be subscribed. Subscribing an already subscribed client will have no effect.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `auctionPackage` - Required. Name of the auction package. Format: `buyers/{accountId}/auctionPackages/{auctionPackageId}`
    pub fn auction_packages_subscribe_clients(&self, request: SubscribeClientsRequest, auction_package: &str) -> BuyerAuctionPackageSubscribeClientCall<'a, S> {
        BuyerAuctionPackageSubscribeClientCall {
            hub: self.hub,
            _request: request,
            _auction_package: auction_package.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Unsubscribe from the auction package for the specified buyer. Once unsubscribed, the bidder will no longer receive a call out for the auction package deal ID and the specified buyer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the auction package. Format: `buyers/{accountId}/auctionPackages/{auctionPackageId}`
    pub fn auction_packages_unsubscribe(&self, request: UnsubscribeAuctionPackageRequest, name: &str) -> BuyerAuctionPackageUnsubscribeCall<'a, S> {
        BuyerAuctionPackageUnsubscribeCall {
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
    /// Unsubscribe from the auction package for the specified clients of the buyer. Unsubscribing a client that is not subscribed will have no effect.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `auctionPackage` - Required. Name of the auction package. Format: `buyers/{accountId}/auctionPackages/{auctionPackageId}`
    pub fn auction_packages_unsubscribe_clients(&self, request: UnsubscribeClientsRequest, auction_package: &str) -> BuyerAuctionPackageUnsubscribeClientCall<'a, S> {
        BuyerAuctionPackageUnsubscribeClientCall {
            hub: self.hub,
            _request: request,
            _auction_package: auction_package.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Activates an existing client user. The state of the client user will be updated from "INACTIVE" to "ACTIVE". This method has no effect if the client user is already in "ACTIVE" state. An error will be returned if the client user to activate is still in "INVITED" state.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `buyers/{buyerAccountId}/clients/{clientAccountId}/clientUsers/{userId}`
    pub fn clients_users_activate(&self, request: ActivateClientUserRequest, name: &str) -> BuyerClientUserActivateCall<'a, S> {
        BuyerClientUserActivateCall {
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
    /// Creates a new client user in "INVITED" state. An email invitation will be sent to the new user, once accepted the user will become active.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the client. Format: `buyers/{accountId}/clients/{clientAccountId}`
    pub fn clients_users_create(&self, request: ClientUser, parent: &str) -> BuyerClientUserCreateCall<'a, S> {
        BuyerClientUserCreateCall {
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
    /// Deactivates an existing client user. The state of the client user will be updated from "ACTIVE" to "INACTIVE". This method has no effect if the client user is already in "INACTIVE" state. An error will be returned if the client user to deactivate is still in "INVITED" state.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `buyers/{buyerAccountId}/clients/{clientAccountId}/clientUsers/{userId}`
    pub fn clients_users_deactivate(&self, request: DeactivateClientUserRequest, name: &str) -> BuyerClientUserDeactivateCall<'a, S> {
        BuyerClientUserDeactivateCall {
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
    /// Deletes an existing client user. The client user will lose access to the Authorized Buyers UI. Note that if a client user is deleted, the user's access to the UI can't be restored unless a new client user is created and activated.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Format: `buyers/{buyerAccountId}/clients/{clientAccountId}/clientUsers/{userId}`
    pub fn clients_users_delete(&self, name: &str) -> BuyerClientUserDeleteCall<'a, S> {
        BuyerClientUserDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an existing client user.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Format: `buyers/{buyerAccountId}/clients/{clientAccountId}/clientUsers/{userId}`
    pub fn clients_users_get(&self, name: &str) -> BuyerClientUserGetCall<'a, S> {
        BuyerClientUserGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all client users for a specified client.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the client. Format: `buyers/{buyerAccountId}/clients/{clientAccountId}`
    pub fn clients_users_list(&self, parent: &str) -> BuyerClientUserListCall<'a, S> {
        BuyerClientUserListCall {
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
    /// Activates an existing client. The state of the client will be updated to "ACTIVE". This method has no effect if the client is already in "ACTIVE" state.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `buyers/{buyerAccountId}/clients/{clientAccountId}`
    pub fn clients_activate(&self, request: ActivateClientRequest, name: &str) -> BuyerClientActivateCall<'a, S> {
        BuyerClientActivateCall {
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
    /// Creates a new client.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the buyer. Format: `buyers/{accountId}`
    pub fn clients_create(&self, request: Client, parent: &str) -> BuyerClientCreateCall<'a, S> {
        BuyerClientCreateCall {
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
    /// Deactivates an existing client. The state of the client will be updated to "INACTIVE". This method has no effect if the client is already in "INACTIVE" state.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `buyers/{buyerAccountId}/clients/{clientAccountId}`
    pub fn clients_deactivate(&self, request: DeactivateClientRequest, name: &str) -> BuyerClientDeactivateCall<'a, S> {
        BuyerClientDeactivateCall {
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
    /// Gets a client with a given resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Format: `buyers/{accountId}/clients/{clientAccountId}`
    pub fn clients_get(&self, name: &str) -> BuyerClientGetCall<'a, S> {
        BuyerClientGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the clients for the current buyer.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the buyer. Format: `buyers/{accountId}`
    pub fn clients_list(&self, parent: &str) -> BuyerClientListCall<'a, S> {
        BuyerClientListCall {
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
    /// Updates an existing client.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name of the client. Format: `buyers/{accountId}/clients/{clientAccountId}`
    pub fn clients_patch(&self, request: Client, name: &str) -> BuyerClientPatchCall<'a, S> {
        BuyerClientPatchCall {
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
    /// Add creative to be used in the bidding process for a finalized deal. For programmatic guaranteed deals, it's recommended that you associate at least one approved creative with the deal before calling SetReadyToServe, to help reduce the number of bid responses filtered because they don't contain approved creatives. Creatives successfully added to a deal can be found in the Realtime-bidding Creatives API creative.deal_ids. This method only applies to programmatic guaranteed deals. Maximum number of 1000 creatives can be added to a finalized deal.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `deal` - Required. Name of the finalized deal in the format of: `buyers/{accountId}/finalizedDeals/{dealId}`
    pub fn finalized_deals_add_creative(&self, request: AddCreativeRequest, deal: &str) -> BuyerFinalizedDealAddCreativeCall<'a, S> {
        BuyerFinalizedDealAddCreativeCall {
            hub: self.hub,
            _request: request,
            _deal: deal.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a finalized deal given its name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Format: `buyers/{accountId}/finalizedDeals/{dealId}`
    pub fn finalized_deals_get(&self, name: &str) -> BuyerFinalizedDealGetCall<'a, S> {
        BuyerFinalizedDealGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists finalized deals. Use the URL path "/v1/buyers/{accountId}/finalizedDeals" to list finalized deals for the current buyer and its clients. Bidders can use the URL path "/v1/bidders/{accountId}/finalizedDeals" to list finalized deals for the bidder, its buyers and all their clients.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The buyer to list the finalized deals for, in the format: `buyers/{accountId}`. When used to list finalized deals for a bidder, its buyers and clients, in the format `bidders/{accountId}`.
    pub fn finalized_deals_list(&self, parent: &str) -> BuyerFinalizedDealListCall<'a, S> {
        BuyerFinalizedDealListCall {
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
    /// Pauses serving of the given finalized deal. This call only pauses the serving status, and does not affect other fields of the finalized deal. Calling this method for an already paused deal has no effect. This method only applies to programmatic guaranteed deals.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `buyers/{accountId}/finalizedDeals/{dealId}`
    pub fn finalized_deals_pause(&self, request: PauseFinalizedDealRequest, name: &str) -> BuyerFinalizedDealPauseCall<'a, S> {
        BuyerFinalizedDealPauseCall {
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
    /// Resumes serving of the given finalized deal. Calling this method for an running deal has no effect. If a deal is initially paused by the seller, calling this method will not resume serving of the deal until the seller also resumes the deal. This method only applies to programmatic guaranteed deals.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `buyers/{accountId}/finalizedDeals/{dealId}`
    pub fn finalized_deals_resume(&self, request: ResumeFinalizedDealRequest, name: &str) -> BuyerFinalizedDealResumeCall<'a, S> {
        BuyerFinalizedDealResumeCall {
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
    /// Sets the given finalized deal as ready to serve. By default, deals are set as ready to serve as soon as they're finalized. If you want to opt out of the default behavior, and manually indicate that deals are ready to serve, ask your Technical Account Manager to add you to the allowlist. If you choose to use this method, finalized deals belonging to the bidder and its child seats don't start serving until after you call `setReadyToServe`, and after the deals become active. For example, you can use this method to delay receiving bid requests until your creative is ready. This method only applies to programmatic guaranteed deals.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `deal` - Required. Format: `buyers/{accountId}/finalizedDeals/{dealId}`
    pub fn finalized_deals_set_ready_to_serve(&self, request: SetReadyToServeRequest, deal: &str) -> BuyerFinalizedDealSetReadyToServeCall<'a, S> {
        BuyerFinalizedDealSetReadyToServeCall {
            hub: self.hub,
            _request: request,
            _deal: deal.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Batch updates multiple deals in the same proposal.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the proposal containing the deals to batch update. Format: buyers/{accountId}/proposals/{proposalId}
    pub fn proposals_deals_batch_update(&self, request: BatchUpdateDealsRequest, parent: &str) -> BuyerProposalDealBatchUpdateCall<'a, S> {
        BuyerProposalDealBatchUpdateCall {
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
    /// Gets a deal given its name. The deal is returned at its head revision.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Format: buyers/{accountId}/proposals/{proposalId}/deals/{dealId}
    pub fn proposals_deals_get(&self, name: &str) -> BuyerProposalDealGetCall<'a, S> {
        BuyerProposalDealGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all deals in a proposal. To retrieve only the finalized revision deals regardless if a deal is being renegotiated, see the FinalizedDeals resource.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the proposal containing the deals to retrieve. Format: buyers/{accountId}/proposals/{proposalId}
    pub fn proposals_deals_list(&self, parent: &str) -> BuyerProposalDealListCall<'a, S> {
        BuyerProposalDealListCall {
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
    /// Updates the given deal at the buyer known revision number. If the server revision has advanced since the passed-in proposal.proposal_revision an ABORTED error message will be returned. The revision number is incremented by the server whenever the proposal or its constituent deals are updated. Note: The revision number is kept at a proposal level. The buyer of the API is expected to keep track of the revision number after the last update operation and send it in as part of the next update request. This way, if there are further changes on the server (for example, seller making new updates), then the server can detect conflicts and reject the proposed changes.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The unique identifier of the deal. Auto-generated by the server when a deal is created. Format: buyers/{accountId}/proposals/{proposalId}/deals/{dealId}
    pub fn proposals_deals_patch(&self, request: Deal, name: &str) -> BuyerProposalDealPatchCall<'a, S> {
        BuyerProposalDealPatchCall {
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
    /// Accepts the proposal at the given revision number. If the revision number in the request is behind the latest from the server, an error message will be returned. This call updates the Proposal.state from `BUYER_ACCEPTANCE_REQUESTED` to `FINALIZED`; it has no side effect if the Proposal.state is already `FINALIZED` and throws exception if the Proposal.state is not either `BUYER_ACCEPTANCE_REQUESTED` or `FINALIZED`. Accepting a proposal means the buyer understands and accepts the Proposal.terms_and_conditions proposed by the seller.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Name of the proposal. Format: `buyers/{accountId}/proposals/{proposalId}`
    pub fn proposals_accept(&self, request: AcceptProposalRequest, name: &str) -> BuyerProposalAcceptCall<'a, S> {
        BuyerProposalAcceptCall {
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
    /// Creates a note for this proposal and sends to the seller.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `proposal` - Name of the proposal. Format: `buyers/{accountId}/proposals/{proposalId}`
    pub fn proposals_add_note(&self, request: AddNoteRequest, proposal: &str) -> BuyerProposalAddNoteCall<'a, S> {
        BuyerProposalAddNoteCall {
            hub: self.hub,
            _request: request,
            _proposal: proposal.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancels an ongoing negotiation on a proposal. This does not cancel or end serving for the deals if the proposal has been finalized. If the proposal has not been finalized before, calling this method will set the Proposal.state to `TERMINATED` and increment the Proposal.proposal_revision. If the proposal has been finalized before and is under renegotiation now, calling this method will reset the Proposal.state to `FINALIZED` and increment the Proposal.proposal_revision. This method does not support private auction proposals whose Proposal.deal_type is 'PRIVATE_AUCTION'.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `proposal` - Name of the proposal. Format: `buyers/{accountId}/proposals/{proposalId}`
    pub fn proposals_cancel_negotiation(&self, request: CancelNegotiationRequest, proposal: &str) -> BuyerProposalCancelNegotiationCall<'a, S> {
        BuyerProposalCancelNegotiationCall {
            hub: self.hub,
            _request: request,
            _proposal: proposal.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a proposal using its name. The proposal is returned at most recent revision. revision.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the proposal. Format: `buyers/{accountId}/proposals/{proposalId}`
    pub fn proposals_get(&self, name: &str) -> BuyerProposalGetCall<'a, S> {
        BuyerProposalGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists proposals. A filter expression (list filter syntax) may be specified to filter the results. This will not list finalized versions of proposals that are being renegotiated; to retrieve these use the finalizedProposals resource.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent that owns the collection of proposals Format: `buyers/{accountId}`
    pub fn proposals_list(&self, parent: &str) -> BuyerProposalListCall<'a, S> {
        BuyerProposalListCall {
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
    /// Updates the proposal at the given revision number. If the revision number in the request is behind the latest from the server, an error message will be returned. See FieldMask for how to use FieldMask. Only fields specified in the UpdateProposalRequest.update_mask will be updated; Fields noted as 'Immutable' or 'Output only' yet specified in the UpdateProposalRequest.update_mask will be ignored and left unchanged. Updating a private auction proposal is not allowed and will result in an error.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The name of the proposal serving as a unique identifier. Format: buyers/{accountId}/proposals/{proposalId}
    pub fn proposals_patch(&self, request: Proposal, name: &str) -> BuyerProposalPatchCall<'a, S> {
        BuyerProposalPatchCall {
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
    /// Sends a request for proposal (RFP) to a publisher to initiate the negotiation regarding certain inventory. In the RFP, buyers can specify the deal type, deal terms, start and end dates, targeting, and a message to the publisher. Once the RFP is sent, a proposal in `SELLER_REVIEW_REQUESTED` state will be created and returned in the response. The publisher may review your request and respond with detailed deals in the proposal.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `buyer` - Required. The current buyer who is sending the RFP in the format: `buyers/{accountId}`.
    pub fn proposals_send_rfp(&self, request: SendRfpRequest, buyer: &str) -> BuyerProposalSendRfpCall<'a, S> {
        BuyerProposalSendRfpCall {
            hub: self.hub,
            _request: request,
            _buyer: buyer.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the requested publisher profile by name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the publisher profile. Format: `buyers/{buyerId}/publisherProfiles/{publisherProfileId}`
    pub fn publisher_profiles_get(&self, name: &str) -> BuyerPublisherProfileGetCall<'a, S> {
        BuyerPublisherProfileGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists publisher profiles. The returned publisher profiles aren't in any defined order. The order of the results might change. A new publisher profile can appear in any place in the list of returned results.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent that owns the collection of publisher profiles Format: `buyers/{buyerId}`
    pub fn publisher_profiles_list(&self, parent: &str) -> BuyerPublisherProfileListCall<'a, S> {
        BuyerPublisherProfileListCall {
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
}



