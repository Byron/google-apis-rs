use super::*;
/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the [`AdExchangeBuyer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexchangebuyer1d4 as adexchangebuyer1d4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adexchangebuyer1d4::{AdExchangeBuyer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdExchangeBuyer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdExchangeBuyer<S>,
}

impl<'a, S> client::MethodsBuilder for AccountMethods<'a, S> {}

impl<'a, S> AccountMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one account by ID.
    /// 
    /// # Arguments
    ///
    /// * `id` - The account id
    pub fn get(&self, id: i32) -> AccountGetCall<'a, S> {
        AccountGetCall {
            hub: self.hub,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the authenticated user's list of accounts.
    pub fn list(&self) -> AccountListCall<'a, S> {
        AccountListCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing account. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `id` - The account id
    pub fn patch(&self, request: Account, id: i32) -> AccountPatchCall<'a, S> {
        AccountPatchCall {
            hub: self.hub,
            _request: request,
            _id: id,
            _confirm_unsafe_account_change: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing account.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `id` - The account id
    pub fn update(&self, request: Account, id: i32) -> AccountUpdateCall<'a, S> {
        AccountUpdateCall {
            hub: self.hub,
            _request: request,
            _id: id,
            _confirm_unsafe_account_change: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *billingInfo* resources.
/// It is not used directly, but through the [`AdExchangeBuyer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexchangebuyer1d4 as adexchangebuyer1d4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adexchangebuyer1d4::{AdExchangeBuyer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdExchangeBuyer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.billing_info();
/// # }
/// ```
pub struct BillingInfoMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdExchangeBuyer<S>,
}

impl<'a, S> client::MethodsBuilder for BillingInfoMethods<'a, S> {}

impl<'a, S> BillingInfoMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the billing information for one account specified by account ID.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - The account id.
    pub fn get(&self, account_id: i32) -> BillingInfoGetCall<'a, S> {
        BillingInfoGetCall {
            hub: self.hub,
            _account_id: account_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of billing information for all accounts of the authenticated user.
    pub fn list(&self) -> BillingInfoListCall<'a, S> {
        BillingInfoListCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *budget* resources.
/// It is not used directly, but through the [`AdExchangeBuyer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexchangebuyer1d4 as adexchangebuyer1d4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adexchangebuyer1d4::{AdExchangeBuyer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdExchangeBuyer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.budget();
/// # }
/// ```
pub struct BudgetMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdExchangeBuyer<S>,
}

impl<'a, S> client::MethodsBuilder for BudgetMethods<'a, S> {}

impl<'a, S> BudgetMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the budget information for the adgroup specified by the accountId and billingId.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - The account id to get the budget information for.
    /// * `billingId` - The billing id to get the budget information for.
    pub fn get(&self, account_id: i64, billing_id: i64) -> BudgetGetCall<'a, S> {
        BudgetGetCall {
            hub: self.hub,
            _account_id: account_id,
            _billing_id: billing_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the budget amount for the budget of the adgroup specified by the accountId and billingId, with the budget amount in the request. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - The account id associated with the budget being updated.
    /// * `billingId` - The billing id associated with the budget being updated.
    pub fn patch(&self, request: Budget, account_id: i64, billing_id: i64) -> BudgetPatchCall<'a, S> {
        BudgetPatchCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id,
            _billing_id: billing_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the budget amount for the budget of the adgroup specified by the accountId and billingId, with the budget amount in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - The account id associated with the budget being updated.
    /// * `billingId` - The billing id associated with the budget being updated.
    pub fn update(&self, request: Budget, account_id: i64, billing_id: i64) -> BudgetUpdateCall<'a, S> {
        BudgetUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id,
            _billing_id: billing_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *creative* resources.
/// It is not used directly, but through the [`AdExchangeBuyer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexchangebuyer1d4 as adexchangebuyer1d4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adexchangebuyer1d4::{AdExchangeBuyer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdExchangeBuyer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `add_deal(...)`, `get(...)`, `insert(...)`, `list(...)`, `list_deals(...)` and `remove_deal(...)`
/// // to build up your call.
/// let rb = hub.creatives();
/// # }
/// ```
pub struct CreativeMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdExchangeBuyer<S>,
}

impl<'a, S> client::MethodsBuilder for CreativeMethods<'a, S> {}

impl<'a, S> CreativeMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Add a deal id association for the creative.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - The id for the account that will serve this creative.
    /// * `buyerCreativeId` - The buyer-specific id for this creative.
    /// * `dealId` - The id of the deal id to associate with this creative.
    pub fn add_deal(&self, account_id: i32, buyer_creative_id: &str, deal_id: i64) -> CreativeAddDealCall<'a, S> {
        CreativeAddDealCall {
            hub: self.hub,
            _account_id: account_id,
            _buyer_creative_id: buyer_creative_id.to_string(),
            _deal_id: deal_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the status for a single creative. A creative will be available 30-40 minutes after submission.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - The id for the account that will serve this creative.
    /// * `buyerCreativeId` - The buyer-specific id for this creative.
    pub fn get(&self, account_id: i32, buyer_creative_id: &str) -> CreativeGetCall<'a, S> {
        CreativeGetCall {
            hub: self.hub,
            _account_id: account_id,
            _buyer_creative_id: buyer_creative_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Submit a new creative.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: Creative) -> CreativeInsertCall<'a, S> {
        CreativeInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of the authenticated user's active creatives. A creative will be available 30-40 minutes after submission.
    pub fn list(&self) -> CreativeListCall<'a, S> {
        CreativeListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _open_auction_status_filter: Default::default(),
            _max_results: Default::default(),
            _deals_status_filter: Default::default(),
            _buyer_creative_id: Default::default(),
            _account_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the external deal ids associated with the creative.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - The id for the account that will serve this creative.
    /// * `buyerCreativeId` - The buyer-specific id for this creative.
    pub fn list_deals(&self, account_id: i32, buyer_creative_id: &str) -> CreativeListDealCall<'a, S> {
        CreativeListDealCall {
            hub: self.hub,
            _account_id: account_id,
            _buyer_creative_id: buyer_creative_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Remove a deal id associated with the creative.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - The id for the account that will serve this creative.
    /// * `buyerCreativeId` - The buyer-specific id for this creative.
    /// * `dealId` - The id of the deal id to disassociate with this creative.
    pub fn remove_deal(&self, account_id: i32, buyer_creative_id: &str, deal_id: i64) -> CreativeRemoveDealCall<'a, S> {
        CreativeRemoveDealCall {
            hub: self.hub,
            _account_id: account_id,
            _buyer_creative_id: buyer_creative_id.to_string(),
            _deal_id: deal_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *marketplacedeal* resources.
/// It is not used directly, but through the [`AdExchangeBuyer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexchangebuyer1d4 as adexchangebuyer1d4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adexchangebuyer1d4::{AdExchangeBuyer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdExchangeBuyer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `insert(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.marketplacedeals();
/// # }
/// ```
pub struct MarketplacedealMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdExchangeBuyer<S>,
}

impl<'a, S> client::MethodsBuilder for MarketplacedealMethods<'a, S> {}

impl<'a, S> MarketplacedealMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete the specified deals from the proposal
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `proposalId` - The proposalId to delete deals from.
    pub fn delete(&self, request: DeleteOrderDealsRequest, proposal_id: &str) -> MarketplacedealDeleteCall<'a, S> {
        MarketplacedealDeleteCall {
            hub: self.hub,
            _request: request,
            _proposal_id: proposal_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Add new deals for the specified proposal
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `proposalId` - proposalId for which deals need to be added.
    pub fn insert(&self, request: AddOrderDealsRequest, proposal_id: &str) -> MarketplacedealInsertCall<'a, S> {
        MarketplacedealInsertCall {
            hub: self.hub,
            _request: request,
            _proposal_id: proposal_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all the deals for a given proposal
    /// 
    /// # Arguments
    ///
    /// * `proposalId` - The proposalId to get deals for. To search across all proposals specify order_id = '-' as part of the URL.
    pub fn list(&self, proposal_id: &str) -> MarketplacedealListCall<'a, S> {
        MarketplacedealListCall {
            hub: self.hub,
            _proposal_id: proposal_id.to_string(),
            _pql_query: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Replaces all the deals in the proposal with the passed in deals
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `proposalId` - The proposalId to edit deals on.
    pub fn update(&self, request: EditAllOrderDealsRequest, proposal_id: &str) -> MarketplacedealUpdateCall<'a, S> {
        MarketplacedealUpdateCall {
            hub: self.hub,
            _request: request,
            _proposal_id: proposal_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *marketplacenote* resources.
/// It is not used directly, but through the [`AdExchangeBuyer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexchangebuyer1d4 as adexchangebuyer1d4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adexchangebuyer1d4::{AdExchangeBuyer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdExchangeBuyer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.marketplacenotes();
/// # }
/// ```
pub struct MarketplacenoteMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdExchangeBuyer<S>,
}

impl<'a, S> client::MethodsBuilder for MarketplacenoteMethods<'a, S> {}

impl<'a, S> MarketplacenoteMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Add notes to the proposal
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `proposalId` - The proposalId to add notes for.
    pub fn insert(&self, request: AddOrderNotesRequest, proposal_id: &str) -> MarketplacenoteInsertCall<'a, S> {
        MarketplacenoteInsertCall {
            hub: self.hub,
            _request: request,
            _proposal_id: proposal_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get all the notes associated with a proposal
    /// 
    /// # Arguments
    ///
    /// * `proposalId` - The proposalId to get notes for. To search across all proposals specify order_id = '-' as part of the URL.
    pub fn list(&self, proposal_id: &str) -> MarketplacenoteListCall<'a, S> {
        MarketplacenoteListCall {
            hub: self.hub,
            _proposal_id: proposal_id.to_string(),
            _pql_query: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *marketplaceprivateauction* resources.
/// It is not used directly, but through the [`AdExchangeBuyer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexchangebuyer1d4 as adexchangebuyer1d4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adexchangebuyer1d4::{AdExchangeBuyer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdExchangeBuyer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `updateproposal(...)`
/// // to build up your call.
/// let rb = hub.marketplaceprivateauction();
/// # }
/// ```
pub struct MarketplaceprivateauctionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdExchangeBuyer<S>,
}

impl<'a, S> client::MethodsBuilder for MarketplaceprivateauctionMethods<'a, S> {}

impl<'a, S> MarketplaceprivateauctionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update a given private auction proposal
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `privateAuctionId` - The private auction id to be updated.
    pub fn updateproposal(&self, request: UpdatePrivateAuctionProposalRequest, private_auction_id: &str) -> MarketplaceprivateauctionUpdateproposalCall<'a, S> {
        MarketplaceprivateauctionUpdateproposalCall {
            hub: self.hub,
            _request: request,
            _private_auction_id: private_auction_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *performanceReport* resources.
/// It is not used directly, but through the [`AdExchangeBuyer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexchangebuyer1d4 as adexchangebuyer1d4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adexchangebuyer1d4::{AdExchangeBuyer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdExchangeBuyer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.performance_report();
/// # }
/// ```
pub struct PerformanceReportMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdExchangeBuyer<S>,
}

impl<'a, S> client::MethodsBuilder for PerformanceReportMethods<'a, S> {}

impl<'a, S> PerformanceReportMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the authenticated user's list of performance metrics.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - The account id to get the reports.
    /// * `endDateTime` - The end time of the report in ISO 8601 timestamp format using UTC.
    /// * `startDateTime` - The start time of the report in ISO 8601 timestamp format using UTC.
    pub fn list(&self, account_id: i64, end_date_time: &str, start_date_time: &str) -> PerformanceReportListCall<'a, S> {
        PerformanceReportListCall {
            hub: self.hub,
            _account_id: account_id,
            _end_date_time: end_date_time.to_string(),
            _start_date_time: start_date_time.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *pretargetingConfig* resources.
/// It is not used directly, but through the [`AdExchangeBuyer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexchangebuyer1d4 as adexchangebuyer1d4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adexchangebuyer1d4::{AdExchangeBuyer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdExchangeBuyer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.pretargeting_config();
/// # }
/// ```
pub struct PretargetingConfigMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdExchangeBuyer<S>,
}

impl<'a, S> client::MethodsBuilder for PretargetingConfigMethods<'a, S> {}

impl<'a, S> PretargetingConfigMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing pretargeting config.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - The account id to delete the pretargeting config for.
    /// * `configId` - The specific id of the configuration to delete.
    pub fn delete(&self, account_id: i64, config_id: i64) -> PretargetingConfigDeleteCall<'a, S> {
        PretargetingConfigDeleteCall {
            hub: self.hub,
            _account_id: account_id,
            _config_id: config_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a specific pretargeting configuration
    /// 
    /// # Arguments
    ///
    /// * `accountId` - The account id to get the pretargeting config for.
    /// * `configId` - The specific id of the configuration to retrieve.
    pub fn get(&self, account_id: i64, config_id: i64) -> PretargetingConfigGetCall<'a, S> {
        PretargetingConfigGetCall {
            hub: self.hub,
            _account_id: account_id,
            _config_id: config_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new pretargeting configuration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - The account id to insert the pretargeting config for.
    pub fn insert(&self, request: PretargetingConfig, account_id: i64) -> PretargetingConfigInsertCall<'a, S> {
        PretargetingConfigInsertCall {
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
    /// Retrieves a list of the authenticated user's pretargeting configurations.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - The account id to get the pretargeting configs for.
    pub fn list(&self, account_id: i64) -> PretargetingConfigListCall<'a, S> {
        PretargetingConfigListCall {
            hub: self.hub,
            _account_id: account_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing pretargeting config. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - The account id to update the pretargeting config for.
    /// * `configId` - The specific id of the configuration to update.
    pub fn patch(&self, request: PretargetingConfig, account_id: i64, config_id: i64) -> PretargetingConfigPatchCall<'a, S> {
        PretargetingConfigPatchCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id,
            _config_id: config_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing pretargeting config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - The account id to update the pretargeting config for.
    /// * `configId` - The specific id of the configuration to update.
    pub fn update(&self, request: PretargetingConfig, account_id: i64, config_id: i64) -> PretargetingConfigUpdateCall<'a, S> {
        PretargetingConfigUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id,
            _config_id: config_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *product* resources.
/// It is not used directly, but through the [`AdExchangeBuyer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexchangebuyer1d4 as adexchangebuyer1d4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adexchangebuyer1d4::{AdExchangeBuyer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdExchangeBuyer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `search(...)`
/// // to build up your call.
/// let rb = hub.products();
/// # }
/// ```
pub struct ProductMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdExchangeBuyer<S>,
}

impl<'a, S> client::MethodsBuilder for ProductMethods<'a, S> {}

impl<'a, S> ProductMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the requested product by id.
    /// 
    /// # Arguments
    ///
    /// * `productId` - The id for the product to get the head revision for.
    pub fn get(&self, product_id: &str) -> ProductGetCall<'a, S> {
        ProductGetCall {
            hub: self.hub,
            _product_id: product_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the requested product.
    pub fn search(&self) -> ProductSearchCall<'a, S> {
        ProductSearchCall {
            hub: self.hub,
            _pql_query: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *proposal* resources.
/// It is not used directly, but through the [`AdExchangeBuyer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexchangebuyer1d4 as adexchangebuyer1d4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adexchangebuyer1d4::{AdExchangeBuyer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdExchangeBuyer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)`, `patch(...)`, `search(...)`, `setupcomplete(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.proposals();
/// # }
/// ```
pub struct ProposalMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdExchangeBuyer<S>,
}

impl<'a, S> client::MethodsBuilder for ProposalMethods<'a, S> {}

impl<'a, S> ProposalMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a proposal given its id
    /// 
    /// # Arguments
    ///
    /// * `proposalId` - Id of the proposal to retrieve.
    pub fn get(&self, proposal_id: &str) -> ProposalGetCall<'a, S> {
        ProposalGetCall {
            hub: self.hub,
            _proposal_id: proposal_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create the given list of proposals
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: CreateOrdersRequest) -> ProposalInsertCall<'a, S> {
        ProposalInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update the given proposal. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `proposalId` - The proposal id to update.
    /// * `revisionNumber` - The last known revision number to update. If the head revision in the marketplace database has since changed, an error will be thrown. The caller should then fetch the latest proposal at head revision and retry the update at that revision.
    /// * `updateAction` - The proposed action to take on the proposal. This field is required and it must be set when updating a proposal.
    pub fn patch(&self, request: Proposal, proposal_id: &str, revision_number: i64, update_action: &ProposalUpdateActionEnum) -> ProposalPatchCall<'a, S> {
        ProposalPatchCall {
            hub: self.hub,
            _request: request,
            _proposal_id: proposal_id.to_string(),
            _revision_number: revision_number,
            _update_action: update_action.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Search for proposals using pql query
    pub fn search(&self) -> ProposalSearchCall<'a, S> {
        ProposalSearchCall {
            hub: self.hub,
            _pql_query: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update the given proposal to indicate that setup has been completed.
    /// 
    /// # Arguments
    ///
    /// * `proposalId` - The proposal id for which the setup is complete
    pub fn setupcomplete(&self, proposal_id: &str) -> ProposalSetupcompleteCall<'a, S> {
        ProposalSetupcompleteCall {
            hub: self.hub,
            _proposal_id: proposal_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update the given proposal
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `proposalId` - The proposal id to update.
    /// * `revisionNumber` - The last known revision number to update. If the head revision in the marketplace database has since changed, an error will be thrown. The caller should then fetch the latest proposal at head revision and retry the update at that revision.
    /// * `updateAction` - The proposed action to take on the proposal. This field is required and it must be set when updating a proposal.
    pub fn update(&self, request: Proposal, proposal_id: &str, revision_number: i64, update_action: &ProposalUpdateActionEnum) -> ProposalUpdateCall<'a, S> {
        ProposalUpdateCall {
            hub: self.hub,
            _request: request,
            _proposal_id: proposal_id.to_string(),
            _revision_number: revision_number,
            _update_action: update_action.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *pubprofile* resources.
/// It is not used directly, but through the [`AdExchangeBuyer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexchangebuyer1d4 as adexchangebuyer1d4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adexchangebuyer1d4::{AdExchangeBuyer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdExchangeBuyer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.pubprofiles();
/// # }
/// ```
pub struct PubprofileMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdExchangeBuyer<S>,
}

impl<'a, S> client::MethodsBuilder for PubprofileMethods<'a, S> {}

impl<'a, S> PubprofileMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the requested publisher profile(s) by publisher accountId.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - The accountId of the publisher to get profiles for.
    pub fn list(&self, account_id: i32) -> PubprofileListCall<'a, S> {
        PubprofileListCall {
            hub: self.hub,
            _account_id: account_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



