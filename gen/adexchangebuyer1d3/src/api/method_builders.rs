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
/// extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adexchangebuyer1d3::{AdExchangeBuyer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adexchangebuyer1d3::{AdExchangeBuyer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adexchangebuyer1d3::{AdExchangeBuyer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adexchangebuyer1d3::{AdExchangeBuyer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdExchangeBuyer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)` and `list(...)`
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
            _status_filter: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _buyer_creative_id: Default::default(),
            _account_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *directDeal* resources.
/// It is not used directly, but through the [`AdExchangeBuyer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adexchangebuyer1d3::{AdExchangeBuyer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// let rb = hub.direct_deals();
/// # }
/// ```
pub struct DirectDealMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdExchangeBuyer<S>,
}

impl<'a, S> client::MethodsBuilder for DirectDealMethods<'a, S> {}

impl<'a, S> DirectDealMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one direct deal by ID.
    /// 
    /// # Arguments
    ///
    /// * `id` - The direct deal id
    pub fn get(&self, id: i64) -> DirectDealGetCall<'a, S> {
        DirectDealGetCall {
            hub: self.hub,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the authenticated user's list of direct deals.
    pub fn list(&self) -> DirectDealListCall<'a, S> {
        DirectDealListCall {
            hub: self.hub,
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
/// extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adexchangebuyer1d3::{AdExchangeBuyer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adexchangebuyer1d3::{AdExchangeBuyer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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



