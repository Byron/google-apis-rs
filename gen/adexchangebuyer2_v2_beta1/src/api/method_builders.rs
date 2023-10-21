use super::*;
/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the [`AdExchangeBuyerII`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexchangebuyer2_v2_beta1 as adexchangebuyer2_v2_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adexchangebuyer2_v2_beta1::{AdExchangeBuyerII, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdExchangeBuyerII::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `clients_create(...)`, `clients_get(...)`, `clients_invitations_create(...)`, `clients_invitations_get(...)`, `clients_invitations_list(...)`, `clients_list(...)`, `clients_update(...)`, `clients_users_get(...)`, `clients_users_list(...)`, `clients_users_update(...)`, `creatives_create(...)`, `creatives_deal_associations_add(...)`, `creatives_deal_associations_list(...)`, `creatives_deal_associations_remove(...)`, `creatives_get(...)`, `creatives_list(...)`, `creatives_stop_watching(...)`, `creatives_update(...)`, `creatives_watch(...)`, `finalized_proposals_list(...)`, `finalized_proposals_pause(...)`, `finalized_proposals_resume(...)`, `products_get(...)`, `products_list(...)`, `proposals_accept(...)`, `proposals_add_note(...)`, `proposals_cancel_negotiation(...)`, `proposals_complete_setup(...)`, `proposals_create(...)`, `proposals_get(...)`, `proposals_list(...)`, `proposals_pause(...)`, `proposals_resume(...)`, `proposals_update(...)`, `publisher_profiles_get(...)` and `publisher_profiles_list(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdExchangeBuyerII<S>,
}

impl<'a, S> client::MethodsBuilder for AccountMethods<'a, S> {}

impl<'a, S> AccountMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates and sends out an email invitation to access an Ad Exchange client buyer account.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Numerical account ID of the client's sponsor buyer. (required)
    /// * `clientAccountId` - Numerical account ID of the client buyer that the user should be associated with. (required)
    pub fn clients_invitations_create(&self, request: ClientUserInvitation, account_id: i64, client_account_id: i64) -> AccountClientInvitationCreateCall<'a, S> {
        AccountClientInvitationCreateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id,
            _client_account_id: client_account_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an existing client user invitation.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Numerical account ID of the client's sponsor buyer. (required)
    /// * `clientAccountId` - Numerical account ID of the client buyer that the user invitation to be retrieved is associated with. (required)
    /// * `invitationId` - Numerical identifier of the user invitation to retrieve. (required)
    pub fn clients_invitations_get(&self, account_id: i64, client_account_id: i64, invitation_id: i64) -> AccountClientInvitationGetCall<'a, S> {
        AccountClientInvitationGetCall {
            hub: self.hub,
            _account_id: account_id,
            _client_account_id: client_account_id,
            _invitation_id: invitation_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the client users invitations for a client with a given account ID.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Numerical account ID of the client's sponsor buyer. (required)
    /// * `clientAccountId` - Numerical account ID of the client buyer to list invitations for. (required) You must either specify a string representation of a numerical account identifier or the `-` character to list all the invitations for all the clients of a given sponsor buyer.
    pub fn clients_invitations_list(&self, account_id: i64, client_account_id: &str) -> AccountClientInvitationListCall<'a, S> {
        AccountClientInvitationListCall {
            hub: self.hub,
            _account_id: account_id,
            _client_account_id: client_account_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
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
    /// * `accountId` - Numerical account ID of the client's sponsor buyer. (required)
    /// * `clientAccountId` - Numerical account ID of the client buyer that the user to be retrieved is associated with. (required)
    /// * `userId` - Numerical identifier of the user to retrieve. (required)
    pub fn clients_users_get(&self, account_id: i64, client_account_id: i64, user_id: i64) -> AccountClientUserGetCall<'a, S> {
        AccountClientUserGetCall {
            hub: self.hub,
            _account_id: account_id,
            _client_account_id: client_account_id,
            _user_id: user_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the known client users for a specified sponsor buyer account ID.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Numerical account ID of the sponsor buyer of the client to list users for. (required)
    /// * `clientAccountId` - The account ID of the client buyer to list users for. (required) You must specify either a string representation of a numerical account identifier or the `-` character to list all the client users for all the clients of a given sponsor buyer.
    pub fn clients_users_list(&self, account_id: i64, client_account_id: &str) -> AccountClientUserListCall<'a, S> {
        AccountClientUserListCall {
            hub: self.hub,
            _account_id: account_id,
            _client_account_id: client_account_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing client user. Only the user status can be changed on update.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Numerical account ID of the client's sponsor buyer. (required)
    /// * `clientAccountId` - Numerical account ID of the client buyer that the user to be retrieved is associated with. (required)
    /// * `userId` - Numerical identifier of the user to retrieve. (required)
    pub fn clients_users_update(&self, request: ClientUser, account_id: i64, client_account_id: i64, user_id: i64) -> AccountClientUserUpdateCall<'a, S> {
        AccountClientUserUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id,
            _client_account_id: client_account_id,
            _user_id: user_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new client buyer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Unique numerical account ID for the buyer of which the client buyer is a customer; the sponsor buyer to create a client for. (required)
    pub fn clients_create(&self, request: Client, account_id: i64) -> AccountClientCreateCall<'a, S> {
        AccountClientCreateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a client buyer with a given client account ID.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Numerical account ID of the client's sponsor buyer. (required)
    /// * `clientAccountId` - Numerical account ID of the client buyer to retrieve. (required)
    pub fn clients_get(&self, account_id: i64, client_account_id: i64) -> AccountClientGetCall<'a, S> {
        AccountClientGetCall {
            hub: self.hub,
            _account_id: account_id,
            _client_account_id: client_account_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the clients for the current sponsor buyer.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Unique numerical account ID of the sponsor buyer to list the clients for.
    pub fn clients_list(&self, account_id: i64) -> AccountClientListCall<'a, S> {
        AccountClientListCall {
            hub: self.hub,
            _account_id: account_id,
            _partner_client_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing client buyer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Unique numerical account ID for the buyer of which the client buyer is a customer; the sponsor buyer to update a client for. (required)
    /// * `clientAccountId` - Unique numerical account ID of the client to update. (required)
    pub fn clients_update(&self, request: Client, account_id: i64, client_account_id: i64) -> AccountClientUpdateCall<'a, S> {
        AccountClientUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id,
            _client_account_id: client_account_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Associate an existing deal with a creative.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - The account the creative belongs to.
    /// * `creativeId` - The ID of the creative associated with the deal.
    pub fn creatives_deal_associations_add(&self, request: AddDealAssociationRequest, account_id: &str, creative_id: &str) -> AccountCreativeDealAssociationAddCall<'a, S> {
        AccountCreativeDealAssociationAddCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _creative_id: creative_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all creative-deal associations.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - The account to list the associations from. Specify "-" to list all creatives the current user has access to.
    /// * `creativeId` - The creative ID to list the associations from. Specify "-" to list all creatives under the above account.
    pub fn creatives_deal_associations_list(&self, account_id: &str, creative_id: &str) -> AccountCreativeDealAssociationListCall<'a, S> {
        AccountCreativeDealAssociationListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _creative_id: creative_id.to_string(),
            _query: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Remove the association between a deal and a creative.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - The account the creative belongs to.
    /// * `creativeId` - The ID of the creative associated with the deal.
    pub fn creatives_deal_associations_remove(&self, request: RemoveDealAssociationRequest, account_id: &str, creative_id: &str) -> AccountCreativeDealAssociationRemoveCall<'a, S> {
        AccountCreativeDealAssociationRemoveCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _creative_id: creative_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a creative.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - The account that this creative belongs to. Can be used to filter the response of the creatives.list method.
    pub fn creatives_create(&self, request: Creative, account_id: &str) -> AccountCreativeCreateCall<'a, S> {
        AccountCreativeCreateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _duplicate_id_mode: Default::default(),
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
    /// * `accountId` - The account the creative belongs to.
    /// * `creativeId` - The ID of the creative to retrieve.
    pub fn creatives_get(&self, account_id: &str, creative_id: &str) -> AccountCreativeGetCall<'a, S> {
        AccountCreativeGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _creative_id: creative_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists creatives.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - The account to list the creatives from. Specify "-" to list all creatives the current user has access to.
    pub fn creatives_list(&self, account_id: &str) -> AccountCreativeListCall<'a, S> {
        AccountCreativeListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _query: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Stops watching a creative. Will stop push notifications being sent to the topics when the creative changes status.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - The account of the creative to stop notifications for.
    /// * `creativeId` - The creative ID of the creative to stop notifications for. Specify "-" to specify stopping account level notifications.
    pub fn creatives_stop_watching(&self, request: StopWatchingCreativeRequest, account_id: &str, creative_id: &str) -> AccountCreativeStopWatchingCall<'a, S> {
        AccountCreativeStopWatchingCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _creative_id: creative_id.to_string(),
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
    /// * `accountId` - The account that this creative belongs to. Can be used to filter the response of the creatives.list method.
    /// * `creativeId` - The buyer-defined creative ID of this creative. Can be used to filter the response of the creatives.list method.
    pub fn creatives_update(&self, request: Creative, account_id: &str, creative_id: &str) -> AccountCreativeUpdateCall<'a, S> {
        AccountCreativeUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _creative_id: creative_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Watches a creative. Will result in push notifications being sent to the topic when the creative changes status.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - The account of the creative to watch.
    /// * `creativeId` - The creative ID to watch for status changes. Specify "-" to watch all creatives under the above account. If both creative-level and account-level notifications are sent, only a single notification will be sent to the creative-level notification topic.
    pub fn creatives_watch(&self, request: WatchCreativeRequest, account_id: &str, creative_id: &str) -> AccountCreativeWatchCall<'a, S> {
        AccountCreativeWatchCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _creative_id: creative_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List finalized proposals, regardless if a proposal is being renegotiated. A filter expression (PQL query) may be specified to filter the results. The notes will not be returned.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID of the buyer.
    pub fn finalized_proposals_list(&self, account_id: &str) -> AccountFinalizedProposalListCall<'a, S> {
        AccountFinalizedProposalListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter_syntax: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update given deals to pause serving. This method will set the `DealServingMetadata.DealPauseStatus.has_buyer_paused` bit to true for all listed deals in the request. Currently, this method only applies to PG and PD deals. For PA deals, call accounts.proposals.pause endpoint. It is a no-op to pause already-paused deals. It is an error to call PauseProposalDeals for deals which are not part of the proposal of proposal_id or which are not finalized or renegotiating.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID of the buyer.
    /// * `proposalId` - The proposal_id of the proposal containing the deals.
    pub fn finalized_proposals_pause(&self, request: PauseProposalDealsRequest, account_id: &str, proposal_id: &str) -> AccountFinalizedProposalPauseCall<'a, S> {
        AccountFinalizedProposalPauseCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _proposal_id: proposal_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update given deals to resume serving. This method will set the `DealServingMetadata.DealPauseStatus.has_buyer_paused` bit to false for all listed deals in the request. Currently, this method only applies to PG and PD deals. For PA deals, call accounts.proposals.resume endpoint. It is a no-op to resume running deals or deals paused by the other party. It is an error to call ResumeProposalDeals for deals which are not part of the proposal of proposal_id or which are not finalized or renegotiating.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID of the buyer.
    /// * `proposalId` - The proposal_id of the proposal containing the deals.
    pub fn finalized_proposals_resume(&self, request: ResumeProposalDealsRequest, account_id: &str, proposal_id: &str) -> AccountFinalizedProposalResumeCall<'a, S> {
        AccountFinalizedProposalResumeCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _proposal_id: proposal_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the requested product by ID.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID of the buyer.
    /// * `productId` - The ID for the product to get the head revision for.
    pub fn products_get(&self, account_id: &str, product_id: &str) -> AccountProductGetCall<'a, S> {
        AccountProductGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _product_id: product_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all products visible to the buyer (optionally filtered by the specified PQL query).
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID of the buyer.
    pub fn products_list(&self, account_id: &str) -> AccountProductListCall<'a, S> {
        AccountProductListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
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
    /// Mark the proposal as accepted at the given revision number. If the number does not match the server's revision number an `ABORTED` error message will be returned. This call updates the proposal_state from `PROPOSED` to `BUYER_ACCEPTED`, or from `SELLER_ACCEPTED` to `FINALIZED`. Upon calling this endpoint, the buyer implicitly agrees to the terms and conditions optionally set within the proposal by the publisher.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID of the buyer.
    /// * `proposalId` - The ID of the proposal to accept.
    pub fn proposals_accept(&self, request: AcceptProposalRequest, account_id: &str, proposal_id: &str) -> AccountProposalAcceptCall<'a, S> {
        AccountProposalAcceptCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _proposal_id: proposal_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a new note and attach it to the proposal. The note is assigned a unique ID by the server. The proposal revision number will not increase when associated with a new note.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID of the buyer.
    /// * `proposalId` - The ID of the proposal to attach the note to.
    pub fn proposals_add_note(&self, request: AddNoteRequest, account_id: &str, proposal_id: &str) -> AccountProposalAddNoteCall<'a, S> {
        AccountProposalAddNoteCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _proposal_id: proposal_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancel an ongoing negotiation on a proposal. This does not cancel or end serving for the deals if the proposal has been finalized, but only cancels a negotiation unilaterally.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID of the buyer.
    /// * `proposalId` - The ID of the proposal to cancel negotiation for.
    pub fn proposals_cancel_negotiation(&self, request: CancelNegotiationRequest, account_id: &str, proposal_id: &str) -> AccountProposalCancelNegotiationCall<'a, S> {
        AccountProposalCancelNegotiationCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _proposal_id: proposal_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// You can opt-in to manually update proposals to indicate that setup is complete. By default, proposal setup is automatically completed after their deals are finalized. Contact your Technical Account Manager to opt in. Buyers can call this method when the proposal has been finalized, and all the required creatives have been uploaded using the Creatives API. This call updates the `is_setup_completed` field on the deals in the proposal, and notifies the seller. The server then advances the revision number of the most recent proposal. To mark an individual deal as ready to serve, call `buyers.finalizedDeals.setReadyToServe` in the Marketplace API.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID of the buyer.
    /// * `proposalId` - The ID of the proposal to mark as setup completed.
    pub fn proposals_complete_setup(&self, request: CompleteSetupRequest, account_id: &str, proposal_id: &str) -> AccountProposalCompleteSetupCall<'a, S> {
        AccountProposalCompleteSetupCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _proposal_id: proposal_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create the given proposal. Each created proposal and any deals it contains are assigned a unique ID by the server.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID of the buyer.
    pub fn proposals_create(&self, request: Proposal, account_id: &str) -> AccountProposalCreateCall<'a, S> {
        AccountProposalCreateCall {
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
    /// Gets a proposal given its ID. The proposal is returned at its head revision.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID of the buyer.
    /// * `proposalId` - The unique ID of the proposal
    pub fn proposals_get(&self, account_id: &str, proposal_id: &str) -> AccountProposalGetCall<'a, S> {
        AccountProposalGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _proposal_id: proposal_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List proposals. A filter expression (PQL query) may be specified to filter the results. To retrieve all finalized proposals, regardless if a proposal is being renegotiated, see the FinalizedProposals resource. Note that Bidder/ChildSeat relationships differ from the usual behavior. A Bidder account can only see its child seats' proposals by specifying the ChildSeat's accountId in the request path.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID of the buyer.
    pub fn proposals_list(&self, account_id: &str) -> AccountProposalListCall<'a, S> {
        AccountProposalListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter_syntax: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update the given proposal to pause serving. This method will set the `DealServingMetadata.DealPauseStatus.has_buyer_paused` bit to true for all deals in the proposal. It is a no-op to pause an already-paused proposal. It is an error to call PauseProposal for a proposal that is not finalized or renegotiating.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID of the buyer.
    /// * `proposalId` - The ID of the proposal to pause.
    pub fn proposals_pause(&self, request: PauseProposalRequest, account_id: &str, proposal_id: &str) -> AccountProposalPauseCall<'a, S> {
        AccountProposalPauseCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _proposal_id: proposal_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update the given proposal to resume serving. This method will set the `DealServingMetadata.DealPauseStatus.has_buyer_paused` bit to false for all deals in the proposal. Note that if the `has_seller_paused` bit is also set, serving will not resume until the seller also resumes. It is a no-op to resume an already-running proposal. It is an error to call ResumeProposal for a proposal that is not finalized or renegotiating.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID of the buyer.
    /// * `proposalId` - The ID of the proposal to resume.
    pub fn proposals_resume(&self, request: ResumeProposalRequest, account_id: &str, proposal_id: &str) -> AccountProposalResumeCall<'a, S> {
        AccountProposalResumeCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _proposal_id: proposal_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update the given proposal at the client known revision number. If the server revision has advanced since the passed-in `proposal.proposal_revision`, an `ABORTED` error message will be returned. Only the buyer-modifiable fields of the proposal will be updated. Note that the deals in the proposal will be updated to match the passed-in copy. If a passed-in deal does not have a `deal_id`, the server will assign a new unique ID and create the deal. If passed-in deal has a `deal_id`, it will be updated to match the passed-in copy. Any existing deals not present in the passed-in proposal will be deleted. It is an error to pass in a deal with a `deal_id` not present at head.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account ID of the buyer.
    /// * `proposalId` - The unique ID of the proposal.
    pub fn proposals_update(&self, request: Proposal, account_id: &str, proposal_id: &str) -> AccountProposalUpdateCall<'a, S> {
        AccountProposalUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _proposal_id: proposal_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the requested publisher profile by id.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID of the buyer.
    /// * `publisherProfileId` - The id for the publisher profile to get.
    pub fn publisher_profiles_get(&self, account_id: &str, publisher_profile_id: &str) -> AccountPublisherProfileGetCall<'a, S> {
        AccountPublisherProfileGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _publisher_profile_id: publisher_profile_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all publisher profiles visible to the buyer
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account ID of the buyer.
    pub fn publisher_profiles_list(&self, account_id: &str) -> AccountPublisherProfileListCall<'a, S> {
        AccountPublisherProfileListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *bidder* resources.
/// It is not used directly, but through the [`AdExchangeBuyerII`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexchangebuyer2_v2_beta1 as adexchangebuyer2_v2_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adexchangebuyer2_v2_beta1::{AdExchangeBuyerII, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdExchangeBuyerII::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `accounts_filter_sets_bid_metrics_list(...)`, `accounts_filter_sets_bid_response_errors_list(...)`, `accounts_filter_sets_bid_responses_without_bids_list(...)`, `accounts_filter_sets_create(...)`, `accounts_filter_sets_delete(...)`, `accounts_filter_sets_filtered_bid_requests_list(...)`, `accounts_filter_sets_filtered_bids_creatives_list(...)`, `accounts_filter_sets_filtered_bids_details_list(...)`, `accounts_filter_sets_filtered_bids_list(...)`, `accounts_filter_sets_get(...)`, `accounts_filter_sets_impression_metrics_list(...)`, `accounts_filter_sets_list(...)`, `accounts_filter_sets_losing_bids_list(...)`, `accounts_filter_sets_non_billable_winning_bids_list(...)`, `filter_sets_bid_metrics_list(...)`, `filter_sets_bid_response_errors_list(...)`, `filter_sets_bid_responses_without_bids_list(...)`, `filter_sets_create(...)`, `filter_sets_delete(...)`, `filter_sets_filtered_bid_requests_list(...)`, `filter_sets_filtered_bids_creatives_list(...)`, `filter_sets_filtered_bids_details_list(...)`, `filter_sets_filtered_bids_list(...)`, `filter_sets_get(...)`, `filter_sets_impression_metrics_list(...)`, `filter_sets_list(...)`, `filter_sets_losing_bids_list(...)` and `filter_sets_non_billable_winning_bids_list(...)`
/// // to build up your call.
/// let rb = hub.bidders();
/// # }
/// ```
pub struct BidderMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdExchangeBuyerII<S>,
}

impl<'a, S> client::MethodsBuilder for BidderMethods<'a, S> {}

impl<'a, S> BidderMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all metrics that are measured in terms of number of bids.
    /// 
    /// # Arguments
    ///
    /// * `filterSetName` - Name of the filter set that should be applied to the requested metrics. For example: - For a bidder-level filter set for bidder 123: `bidders/123/filterSets/abc` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123/filterSets/abc` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456/filterSets/abc`
    pub fn accounts_filter_sets_bid_metrics_list(&self, filter_set_name: &str) -> BidderAccountFilterSetBidMetricListCall<'a, S> {
        BidderAccountFilterSetBidMetricListCall {
            hub: self.hub,
            _filter_set_name: filter_set_name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all errors that occurred in bid responses, with the number of bid responses affected for each reason.
    /// 
    /// # Arguments
    ///
    /// * `filterSetName` - Name of the filter set that should be applied to the requested metrics. For example: - For a bidder-level filter set for bidder 123: `bidders/123/filterSets/abc` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123/filterSets/abc` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456/filterSets/abc`
    pub fn accounts_filter_sets_bid_response_errors_list(&self, filter_set_name: &str) -> BidderAccountFilterSetBidResponseErrorListCall<'a, S> {
        BidderAccountFilterSetBidResponseErrorListCall {
            hub: self.hub,
            _filter_set_name: filter_set_name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all reasons for which bid responses were considered to have no applicable bids, with the number of bid responses affected for each reason.
    /// 
    /// # Arguments
    ///
    /// * `filterSetName` - Name of the filter set that should be applied to the requested metrics. For example: - For a bidder-level filter set for bidder 123: `bidders/123/filterSets/abc` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123/filterSets/abc` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456/filterSets/abc`
    pub fn accounts_filter_sets_bid_responses_without_bids_list(&self, filter_set_name: &str) -> BidderAccountFilterSetBidResponsesWithoutBidListCall<'a, S> {
        BidderAccountFilterSetBidResponsesWithoutBidListCall {
            hub: self.hub,
            _filter_set_name: filter_set_name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all reasons that caused a bid request not to be sent for an impression, with the number of bid requests not sent for each reason.
    /// 
    /// # Arguments
    ///
    /// * `filterSetName` - Name of the filter set that should be applied to the requested metrics. For example: - For a bidder-level filter set for bidder 123: `bidders/123/filterSets/abc` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123/filterSets/abc` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456/filterSets/abc`
    pub fn accounts_filter_sets_filtered_bid_requests_list(&self, filter_set_name: &str) -> BidderAccountFilterSetFilteredBidRequestListCall<'a, S> {
        BidderAccountFilterSetFilteredBidRequestListCall {
            hub: self.hub,
            _filter_set_name: filter_set_name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all creatives associated with a specific reason for which bids were filtered, with the number of bids filtered for each creative.
    /// 
    /// # Arguments
    ///
    /// * `filterSetName` - Name of the filter set that should be applied to the requested metrics. For example: - For a bidder-level filter set for bidder 123: `bidders/123/filterSets/abc` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123/filterSets/abc` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456/filterSets/abc`
    /// * `creativeStatusId` - The ID of the creative status for which to retrieve a breakdown by creative. See [creative-status-codes](https://developers.google.com/authorized-buyers/rtb/downloads/creative-status-codes).
    pub fn accounts_filter_sets_filtered_bids_creatives_list(&self, filter_set_name: &str, creative_status_id: i32) -> BidderAccountFilterSetFilteredBidCreativeListCall<'a, S> {
        BidderAccountFilterSetFilteredBidCreativeListCall {
            hub: self.hub,
            _filter_set_name: filter_set_name.to_string(),
            _creative_status_id: creative_status_id,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all details associated with a specific reason for which bids were filtered, with the number of bids filtered for each detail.
    /// 
    /// # Arguments
    ///
    /// * `filterSetName` - Name of the filter set that should be applied to the requested metrics. For example: - For a bidder-level filter set for bidder 123: `bidders/123/filterSets/abc` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123/filterSets/abc` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456/filterSets/abc`
    /// * `creativeStatusId` - The ID of the creative status for which to retrieve a breakdown by detail. See [creative-status-codes](https://developers.google.com/authorized-buyers/rtb/downloads/creative-status-codes). Details are only available for statuses 10, 14, 15, 17, 18, 19, 86, and 87.
    pub fn accounts_filter_sets_filtered_bids_details_list(&self, filter_set_name: &str, creative_status_id: i32) -> BidderAccountFilterSetFilteredBidDetailListCall<'a, S> {
        BidderAccountFilterSetFilteredBidDetailListCall {
            hub: self.hub,
            _filter_set_name: filter_set_name.to_string(),
            _creative_status_id: creative_status_id,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all reasons for which bids were filtered, with the number of bids filtered for each reason.
    /// 
    /// # Arguments
    ///
    /// * `filterSetName` - Name of the filter set that should be applied to the requested metrics. For example: - For a bidder-level filter set for bidder 123: `bidders/123/filterSets/abc` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123/filterSets/abc` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456/filterSets/abc`
    pub fn accounts_filter_sets_filtered_bids_list(&self, filter_set_name: &str) -> BidderAccountFilterSetFilteredBidListCall<'a, S> {
        BidderAccountFilterSetFilteredBidListCall {
            hub: self.hub,
            _filter_set_name: filter_set_name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all metrics that are measured in terms of number of impressions.
    /// 
    /// # Arguments
    ///
    /// * `filterSetName` - Name of the filter set that should be applied to the requested metrics. For example: - For a bidder-level filter set for bidder 123: `bidders/123/filterSets/abc` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123/filterSets/abc` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456/filterSets/abc`
    pub fn accounts_filter_sets_impression_metrics_list(&self, filter_set_name: &str) -> BidderAccountFilterSetImpressionMetricListCall<'a, S> {
        BidderAccountFilterSetImpressionMetricListCall {
            hub: self.hub,
            _filter_set_name: filter_set_name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all reasons for which bids lost in the auction, with the number of bids that lost for each reason.
    /// 
    /// # Arguments
    ///
    /// * `filterSetName` - Name of the filter set that should be applied to the requested metrics. For example: - For a bidder-level filter set for bidder 123: `bidders/123/filterSets/abc` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123/filterSets/abc` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456/filterSets/abc`
    pub fn accounts_filter_sets_losing_bids_list(&self, filter_set_name: &str) -> BidderAccountFilterSetLosingBidListCall<'a, S> {
        BidderAccountFilterSetLosingBidListCall {
            hub: self.hub,
            _filter_set_name: filter_set_name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all reasons for which winning bids were not billable, with the number of bids not billed for each reason.
    /// 
    /// # Arguments
    ///
    /// * `filterSetName` - Name of the filter set that should be applied to the requested metrics. For example: - For a bidder-level filter set for bidder 123: `bidders/123/filterSets/abc` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123/filterSets/abc` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456/filterSets/abc`
    pub fn accounts_filter_sets_non_billable_winning_bids_list(&self, filter_set_name: &str) -> BidderAccountFilterSetNonBillableWinningBidListCall<'a, S> {
        BidderAccountFilterSetNonBillableWinningBidListCall {
            hub: self.hub,
            _filter_set_name: filter_set_name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates the specified filter set for the account with the given account ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `ownerName` - Name of the owner (bidder or account) of the filter set to be created. For example: - For a bidder-level filter set for bidder 123: `bidders/123` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456`
    pub fn accounts_filter_sets_create(&self, request: FilterSet, owner_name: &str) -> BidderAccountFilterSetCreateCall<'a, S> {
        BidderAccountFilterSetCreateCall {
            hub: self.hub,
            _request: request,
            _owner_name: owner_name.to_string(),
            _is_transient: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the requested filter set from the account with the given account ID.
    /// 
    /// # Arguments
    ///
    /// * `name` - Full name of the resource to delete. For example: - For a bidder-level filter set for bidder 123: `bidders/123/filterSets/abc` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123/filterSets/abc` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456/filterSets/abc`
    pub fn accounts_filter_sets_delete(&self, name: &str) -> BidderAccountFilterSetDeleteCall<'a, S> {
        BidderAccountFilterSetDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the requested filter set for the account with the given account ID.
    /// 
    /// # Arguments
    ///
    /// * `name` - Full name of the resource being requested. For example: - For a bidder-level filter set for bidder 123: `bidders/123/filterSets/abc` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123/filterSets/abc` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456/filterSets/abc`
    pub fn accounts_filter_sets_get(&self, name: &str) -> BidderAccountFilterSetGetCall<'a, S> {
        BidderAccountFilterSetGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all filter sets for the account with the given account ID.
    /// 
    /// # Arguments
    ///
    /// * `ownerName` - Name of the owner (bidder or account) of the filter sets to be listed. For example: - For a bidder-level filter set for bidder 123: `bidders/123` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456`
    pub fn accounts_filter_sets_list(&self, owner_name: &str) -> BidderAccountFilterSetListCall<'a, S> {
        BidderAccountFilterSetListCall {
            hub: self.hub,
            _owner_name: owner_name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all metrics that are measured in terms of number of bids.
    /// 
    /// # Arguments
    ///
    /// * `filterSetName` - Name of the filter set that should be applied to the requested metrics. For example: - For a bidder-level filter set for bidder 123: `bidders/123/filterSets/abc` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123/filterSets/abc` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456/filterSets/abc`
    pub fn filter_sets_bid_metrics_list(&self, filter_set_name: &str) -> BidderFilterSetBidMetricListCall<'a, S> {
        BidderFilterSetBidMetricListCall {
            hub: self.hub,
            _filter_set_name: filter_set_name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all errors that occurred in bid responses, with the number of bid responses affected for each reason.
    /// 
    /// # Arguments
    ///
    /// * `filterSetName` - Name of the filter set that should be applied to the requested metrics. For example: - For a bidder-level filter set for bidder 123: `bidders/123/filterSets/abc` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123/filterSets/abc` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456/filterSets/abc`
    pub fn filter_sets_bid_response_errors_list(&self, filter_set_name: &str) -> BidderFilterSetBidResponseErrorListCall<'a, S> {
        BidderFilterSetBidResponseErrorListCall {
            hub: self.hub,
            _filter_set_name: filter_set_name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all reasons for which bid responses were considered to have no applicable bids, with the number of bid responses affected for each reason.
    /// 
    /// # Arguments
    ///
    /// * `filterSetName` - Name of the filter set that should be applied to the requested metrics. For example: - For a bidder-level filter set for bidder 123: `bidders/123/filterSets/abc` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123/filterSets/abc` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456/filterSets/abc`
    pub fn filter_sets_bid_responses_without_bids_list(&self, filter_set_name: &str) -> BidderFilterSetBidResponsesWithoutBidListCall<'a, S> {
        BidderFilterSetBidResponsesWithoutBidListCall {
            hub: self.hub,
            _filter_set_name: filter_set_name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all reasons that caused a bid request not to be sent for an impression, with the number of bid requests not sent for each reason.
    /// 
    /// # Arguments
    ///
    /// * `filterSetName` - Name of the filter set that should be applied to the requested metrics. For example: - For a bidder-level filter set for bidder 123: `bidders/123/filterSets/abc` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123/filterSets/abc` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456/filterSets/abc`
    pub fn filter_sets_filtered_bid_requests_list(&self, filter_set_name: &str) -> BidderFilterSetFilteredBidRequestListCall<'a, S> {
        BidderFilterSetFilteredBidRequestListCall {
            hub: self.hub,
            _filter_set_name: filter_set_name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all creatives associated with a specific reason for which bids were filtered, with the number of bids filtered for each creative.
    /// 
    /// # Arguments
    ///
    /// * `filterSetName` - Name of the filter set that should be applied to the requested metrics. For example: - For a bidder-level filter set for bidder 123: `bidders/123/filterSets/abc` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123/filterSets/abc` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456/filterSets/abc`
    /// * `creativeStatusId` - The ID of the creative status for which to retrieve a breakdown by creative. See [creative-status-codes](https://developers.google.com/authorized-buyers/rtb/downloads/creative-status-codes).
    pub fn filter_sets_filtered_bids_creatives_list(&self, filter_set_name: &str, creative_status_id: i32) -> BidderFilterSetFilteredBidCreativeListCall<'a, S> {
        BidderFilterSetFilteredBidCreativeListCall {
            hub: self.hub,
            _filter_set_name: filter_set_name.to_string(),
            _creative_status_id: creative_status_id,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all details associated with a specific reason for which bids were filtered, with the number of bids filtered for each detail.
    /// 
    /// # Arguments
    ///
    /// * `filterSetName` - Name of the filter set that should be applied to the requested metrics. For example: - For a bidder-level filter set for bidder 123: `bidders/123/filterSets/abc` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123/filterSets/abc` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456/filterSets/abc`
    /// * `creativeStatusId` - The ID of the creative status for which to retrieve a breakdown by detail. See [creative-status-codes](https://developers.google.com/authorized-buyers/rtb/downloads/creative-status-codes). Details are only available for statuses 10, 14, 15, 17, 18, 19, 86, and 87.
    pub fn filter_sets_filtered_bids_details_list(&self, filter_set_name: &str, creative_status_id: i32) -> BidderFilterSetFilteredBidDetailListCall<'a, S> {
        BidderFilterSetFilteredBidDetailListCall {
            hub: self.hub,
            _filter_set_name: filter_set_name.to_string(),
            _creative_status_id: creative_status_id,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all reasons for which bids were filtered, with the number of bids filtered for each reason.
    /// 
    /// # Arguments
    ///
    /// * `filterSetName` - Name of the filter set that should be applied to the requested metrics. For example: - For a bidder-level filter set for bidder 123: `bidders/123/filterSets/abc` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123/filterSets/abc` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456/filterSets/abc`
    pub fn filter_sets_filtered_bids_list(&self, filter_set_name: &str) -> BidderFilterSetFilteredBidListCall<'a, S> {
        BidderFilterSetFilteredBidListCall {
            hub: self.hub,
            _filter_set_name: filter_set_name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all metrics that are measured in terms of number of impressions.
    /// 
    /// # Arguments
    ///
    /// * `filterSetName` - Name of the filter set that should be applied to the requested metrics. For example: - For a bidder-level filter set for bidder 123: `bidders/123/filterSets/abc` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123/filterSets/abc` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456/filterSets/abc`
    pub fn filter_sets_impression_metrics_list(&self, filter_set_name: &str) -> BidderFilterSetImpressionMetricListCall<'a, S> {
        BidderFilterSetImpressionMetricListCall {
            hub: self.hub,
            _filter_set_name: filter_set_name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all reasons for which bids lost in the auction, with the number of bids that lost for each reason.
    /// 
    /// # Arguments
    ///
    /// * `filterSetName` - Name of the filter set that should be applied to the requested metrics. For example: - For a bidder-level filter set for bidder 123: `bidders/123/filterSets/abc` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123/filterSets/abc` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456/filterSets/abc`
    pub fn filter_sets_losing_bids_list(&self, filter_set_name: &str) -> BidderFilterSetLosingBidListCall<'a, S> {
        BidderFilterSetLosingBidListCall {
            hub: self.hub,
            _filter_set_name: filter_set_name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all reasons for which winning bids were not billable, with the number of bids not billed for each reason.
    /// 
    /// # Arguments
    ///
    /// * `filterSetName` - Name of the filter set that should be applied to the requested metrics. For example: - For a bidder-level filter set for bidder 123: `bidders/123/filterSets/abc` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123/filterSets/abc` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456/filterSets/abc`
    pub fn filter_sets_non_billable_winning_bids_list(&self, filter_set_name: &str) -> BidderFilterSetNonBillableWinningBidListCall<'a, S> {
        BidderFilterSetNonBillableWinningBidListCall {
            hub: self.hub,
            _filter_set_name: filter_set_name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates the specified filter set for the account with the given account ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `ownerName` - Name of the owner (bidder or account) of the filter set to be created. For example: - For a bidder-level filter set for bidder 123: `bidders/123` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456`
    pub fn filter_sets_create(&self, request: FilterSet, owner_name: &str) -> BidderFilterSetCreateCall<'a, S> {
        BidderFilterSetCreateCall {
            hub: self.hub,
            _request: request,
            _owner_name: owner_name.to_string(),
            _is_transient: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the requested filter set from the account with the given account ID.
    /// 
    /// # Arguments
    ///
    /// * `name` - Full name of the resource to delete. For example: - For a bidder-level filter set for bidder 123: `bidders/123/filterSets/abc` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123/filterSets/abc` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456/filterSets/abc`
    pub fn filter_sets_delete(&self, name: &str) -> BidderFilterSetDeleteCall<'a, S> {
        BidderFilterSetDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the requested filter set for the account with the given account ID.
    /// 
    /// # Arguments
    ///
    /// * `name` - Full name of the resource being requested. For example: - For a bidder-level filter set for bidder 123: `bidders/123/filterSets/abc` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123/filterSets/abc` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456/filterSets/abc`
    pub fn filter_sets_get(&self, name: &str) -> BidderFilterSetGetCall<'a, S> {
        BidderFilterSetGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all filter sets for the account with the given account ID.
    /// 
    /// # Arguments
    ///
    /// * `ownerName` - Name of the owner (bidder or account) of the filter sets to be listed. For example: - For a bidder-level filter set for bidder 123: `bidders/123` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456`
    pub fn filter_sets_list(&self, owner_name: &str) -> BidderFilterSetListCall<'a, S> {
        BidderFilterSetListCall {
            hub: self.hub,
            _owner_name: owner_name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



