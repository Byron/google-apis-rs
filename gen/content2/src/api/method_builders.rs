use super::*;
/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the [`ShoppingContent`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_content2 as content2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use content2::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `authinfo(...)`, `claimwebsite(...)`, `custombatch(...)`, `delete(...)`, `get(...)`, `insert(...)`, `link(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ShoppingContent<S>,
}

impl<'a, S> client::MethodsBuilder for AccountMethods<'a, S> {}

impl<'a, S> AccountMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns information about the authenticated user.
    pub fn authinfo(&self) -> AccountAuthinfoCall<'a, S> {
        AccountAuthinfoCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Claims the website of a Merchant Center sub-account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the managing account. If this parameter is not the same as accountId, then this account must be a multi-client account and `accountId` must be the ID of a sub-account of this account.
    /// * `accountId` - The ID of the account whose website is claimed.
    pub fn claimwebsite(&self, merchant_id: u64, account_id: u64) -> AccountClaimwebsiteCall<'a, S> {
        AccountClaimwebsiteCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _account_id: account_id,
            _overwrite: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves, inserts, updates, and deletes multiple Merchant Center (sub-)accounts in a single request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn custombatch(&self, request: AccountsCustomBatchRequest) -> AccountCustombatchCall<'a, S> {
        AccountCustombatchCall {
            hub: self.hub,
            _request: request,
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a Merchant Center sub-account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the managing account. This must be a multi-client account, and accountId must be the ID of a sub-account of this account.
    /// * `accountId` - The ID of the account.
    pub fn delete(&self, merchant_id: u64, account_id: u64) -> AccountDeleteCall<'a, S> {
        AccountDeleteCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _account_id: account_id,
            _force: Default::default(),
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the managing account. If this parameter is not the same as accountId, then this account must be a multi-client account and `accountId` must be the ID of a sub-account of this account.
    /// * `accountId` - The ID of the account.
    pub fn get(&self, merchant_id: u64, account_id: u64) -> AccountGetCall<'a, S> {
        AccountGetCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _account_id: account_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a Merchant Center sub-account.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the managing account. This must be a multi-client account.
    pub fn insert(&self, request: Account, merchant_id: u64) -> AccountInsertCall<'a, S> {
        AccountInsertCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Performs an action on a link between two Merchant Center accounts, namely accountId and linkedAccountId.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the managing account. If this parameter is not the same as accountId, then this account must be a multi-client account and `accountId` must be the ID of a sub-account of this account.
    /// * `accountId` - The ID of the account that should be linked.
    pub fn link(&self, request: AccountsLinkRequest, merchant_id: u64, account_id: u64) -> AccountLinkCall<'a, S> {
        AccountLinkCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _account_id: account_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the sub-accounts in your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the managing account. This must be a multi-client account.
    pub fn list(&self, merchant_id: u64) -> AccountListCall<'a, S> {
        AccountListCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a Merchant Center account. Any fields that are not provided are deleted from the resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the managing account. If this parameter is not the same as accountId, then this account must be a multi-client account and `accountId` must be the ID of a sub-account of this account.
    /// * `accountId` - The ID of the account.
    pub fn update(&self, request: Account, merchant_id: u64, account_id: u64) -> AccountUpdateCall<'a, S> {
        AccountUpdateCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _account_id: account_id,
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *accountstatus* resources.
/// It is not used directly, but through the [`ShoppingContent`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_content2 as content2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use content2::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `custombatch(...)`, `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.accountstatuses();
/// # }
/// ```
pub struct AccountstatusMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ShoppingContent<S>,
}

impl<'a, S> client::MethodsBuilder for AccountstatusMethods<'a, S> {}

impl<'a, S> AccountstatusMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves multiple Merchant Center account statuses in a single request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn custombatch(&self, request: AccountstatusesCustomBatchRequest) -> AccountstatusCustombatchCall<'a, S> {
        AccountstatusCustombatchCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the status of a Merchant Center account. No itemLevelIssues are returned for multi-client accounts.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the managing account. If this parameter is not the same as accountId, then this account must be a multi-client account and `accountId` must be the ID of a sub-account of this account.
    /// * `accountId` - The ID of the account.
    pub fn get(&self, merchant_id: u64, account_id: u64) -> AccountstatusGetCall<'a, S> {
        AccountstatusGetCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _account_id: account_id,
            _destinations: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the statuses of the sub-accounts in your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the managing account. This must be a multi-client account.
    pub fn list(&self, merchant_id: u64) -> AccountstatusListCall<'a, S> {
        AccountstatusListCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _destinations: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *accounttax* resources.
/// It is not used directly, but through the [`ShoppingContent`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_content2 as content2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use content2::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `custombatch(...)`, `get(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.accounttax();
/// # }
/// ```
pub struct AccounttaxMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ShoppingContent<S>,
}

impl<'a, S> client::MethodsBuilder for AccounttaxMethods<'a, S> {}

impl<'a, S> AccounttaxMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves and updates tax settings of multiple accounts in a single request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn custombatch(&self, request: AccounttaxCustomBatchRequest) -> AccounttaxCustombatchCall<'a, S> {
        AccounttaxCustombatchCall {
            hub: self.hub,
            _request: request,
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the tax settings of the account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the managing account. If this parameter is not the same as accountId, then this account must be a multi-client account and `accountId` must be the ID of a sub-account of this account.
    /// * `accountId` - The ID of the account for which to get/update account tax settings.
    pub fn get(&self, merchant_id: u64, account_id: u64) -> AccounttaxGetCall<'a, S> {
        AccounttaxGetCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _account_id: account_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the tax settings of the sub-accounts in your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the managing account. This must be a multi-client account.
    pub fn list(&self, merchant_id: u64) -> AccounttaxListCall<'a, S> {
        AccounttaxListCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the tax settings of the account. Any fields that are not provided are deleted from the resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the managing account. If this parameter is not the same as accountId, then this account must be a multi-client account and `accountId` must be the ID of a sub-account of this account.
    /// * `accountId` - The ID of the account for which to get/update account tax settings.
    pub fn update(&self, request: AccountTax, merchant_id: u64, account_id: u64) -> AccounttaxUpdateCall<'a, S> {
        AccounttaxUpdateCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _account_id: account_id,
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *datafeed* resources.
/// It is not used directly, but through the [`ShoppingContent`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_content2 as content2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use content2::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `custombatch(...)`, `delete(...)`, `fetchnow(...)`, `get(...)`, `insert(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.datafeeds();
/// # }
/// ```
pub struct DatafeedMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ShoppingContent<S>,
}

impl<'a, S> client::MethodsBuilder for DatafeedMethods<'a, S> {}

impl<'a, S> DatafeedMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes, fetches, gets, inserts and updates multiple datafeeds in a single request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn custombatch(&self, request: DatafeedsCustomBatchRequest) -> DatafeedCustombatchCall<'a, S> {
        DatafeedCustombatchCall {
            hub: self.hub,
            _request: request,
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a datafeed configuration from your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the datafeed. This account cannot be a multi-client account.
    /// * `datafeedId` - The ID of the datafeed.
    pub fn delete(&self, merchant_id: u64, datafeed_id: u64) -> DatafeedDeleteCall<'a, S> {
        DatafeedDeleteCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _datafeed_id: datafeed_id,
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Invokes a fetch for the datafeed in your Merchant Center account. If you need to call this method more than once per day, we recommend you use the Products service to update your product data.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the datafeed. This account cannot be a multi-client account.
    /// * `datafeedId` - The ID of the datafeed to be fetched.
    pub fn fetchnow(&self, merchant_id: u64, datafeed_id: u64) -> DatafeedFetchnowCall<'a, S> {
        DatafeedFetchnowCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _datafeed_id: datafeed_id,
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a datafeed configuration from your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the datafeed. This account cannot be a multi-client account.
    /// * `datafeedId` - The ID of the datafeed.
    pub fn get(&self, merchant_id: u64, datafeed_id: u64) -> DatafeedGetCall<'a, S> {
        DatafeedGetCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _datafeed_id: datafeed_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Registers a datafeed configuration with your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the datafeed. This account cannot be a multi-client account.
    pub fn insert(&self, request: Datafeed, merchant_id: u64) -> DatafeedInsertCall<'a, S> {
        DatafeedInsertCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the configurations for datafeeds in your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the datafeeds. This account cannot be a multi-client account.
    pub fn list(&self, merchant_id: u64) -> DatafeedListCall<'a, S> {
        DatafeedListCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a datafeed configuration of your Merchant Center account. Any fields that are not provided are deleted from the resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the datafeed. This account cannot be a multi-client account.
    /// * `datafeedId` - The ID of the datafeed.
    pub fn update(&self, request: Datafeed, merchant_id: u64, datafeed_id: u64) -> DatafeedUpdateCall<'a, S> {
        DatafeedUpdateCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _datafeed_id: datafeed_id,
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *datafeedstatus* resources.
/// It is not used directly, but through the [`ShoppingContent`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_content2 as content2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use content2::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `custombatch(...)`, `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.datafeedstatuses();
/// # }
/// ```
pub struct DatafeedstatusMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ShoppingContent<S>,
}

impl<'a, S> client::MethodsBuilder for DatafeedstatusMethods<'a, S> {}

impl<'a, S> DatafeedstatusMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets multiple Merchant Center datafeed statuses in a single request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn custombatch(&self, request: DatafeedstatusesCustomBatchRequest) -> DatafeedstatusCustombatchCall<'a, S> {
        DatafeedstatusCustombatchCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the status of a datafeed from your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the datafeed. This account cannot be a multi-client account.
    /// * `datafeedId` - The ID of the datafeed.
    pub fn get(&self, merchant_id: u64, datafeed_id: u64) -> DatafeedstatusGetCall<'a, S> {
        DatafeedstatusGetCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _datafeed_id: datafeed_id,
            _language: Default::default(),
            _country: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the statuses of the datafeeds in your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the datafeeds. This account cannot be a multi-client account.
    pub fn list(&self, merchant_id: u64) -> DatafeedstatusListCall<'a, S> {
        DatafeedstatusListCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *inventory* resources.
/// It is not used directly, but through the [`ShoppingContent`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_content2 as content2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use content2::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `custombatch(...)` and `set(...)`
/// // to build up your call.
/// let rb = hub.inventory();
/// # }
/// ```
pub struct InventoryMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ShoppingContent<S>,
}

impl<'a, S> client::MethodsBuilder for InventoryMethods<'a, S> {}

impl<'a, S> InventoryMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates price and availability for multiple products or stores in a single request. This operation does not update the expiration date of the products.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn custombatch(&self, request: InventoryCustomBatchRequest) -> InventoryCustombatchCall<'a, S> {
        InventoryCustombatchCall {
            hub: self.hub,
            _request: request,
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates price and availability of a product in your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that contains the product. This account cannot be a multi-client account.
    /// * `storeCode` - The code of the store for which to update price and availability. Use `online` to update price and availability of an online product.
    /// * `productId` - The REST ID of the product for which to update price and availability.
    pub fn set(&self, request: InventorySetRequest, merchant_id: u64, store_code: &str, product_id: &str) -> InventorySetCall<'a, S> {
        InventorySetCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _store_code: store_code.to_string(),
            _product_id: product_id.to_string(),
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *liasetting* resources.
/// It is not used directly, but through the [`ShoppingContent`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_content2 as content2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use content2::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `custombatch(...)`, `get(...)`, `getaccessiblegmbaccounts(...)`, `list(...)`, `listposdataproviders(...)`, `requestgmbaccess(...)`, `requestinventoryverification(...)`, `setinventoryverificationcontact(...)`, `setposdataprovider(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.liasettings();
/// # }
/// ```
pub struct LiasettingMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ShoppingContent<S>,
}

impl<'a, S> client::MethodsBuilder for LiasettingMethods<'a, S> {}

impl<'a, S> LiasettingMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves and/or updates the LIA settings of multiple accounts in a single request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn custombatch(&self, request: LiasettingsCustomBatchRequest) -> LiasettingCustombatchCall<'a, S> {
        LiasettingCustombatchCall {
            hub: self.hub,
            _request: request,
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the LIA settings of the account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the managing account. If this parameter is not the same as accountId, then this account must be a multi-client account and `accountId` must be the ID of a sub-account of this account.
    /// * `accountId` - The ID of the account for which to get or update LIA settings.
    pub fn get(&self, merchant_id: u64, account_id: u64) -> LiasettingGetCall<'a, S> {
        LiasettingGetCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _account_id: account_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of accessible Google My Business accounts.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the managing account. If this parameter is not the same as accountId, then this account must be a multi-client account and `accountId` must be the ID of a sub-account of this account.
    /// * `accountId` - The ID of the account for which to retrieve accessible Google My Business accounts.
    pub fn getaccessiblegmbaccounts(&self, merchant_id: u64, account_id: u64) -> LiasettingGetaccessiblegmbaccountCall<'a, S> {
        LiasettingGetaccessiblegmbaccountCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _account_id: account_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the LIA settings of the sub-accounts in your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the managing account. This must be a multi-client account.
    pub fn list(&self, merchant_id: u64) -> LiasettingListCall<'a, S> {
        LiasettingListCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of POS data providers that have active settings for the all eiligible countries.
    pub fn listposdataproviders(&self) -> LiasettingListposdataproviderCall<'a, S> {
        LiasettingListposdataproviderCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Requests access to a specified Google My Business account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the managing account. If this parameter is not the same as accountId, then this account must be a multi-client account and `accountId` must be the ID of a sub-account of this account.
    /// * `accountId` - The ID of the account for which GMB access is requested.
    /// * `gmbEmail` - The email of the Google My Business account.
    pub fn requestgmbaccess(&self, merchant_id: u64, account_id: u64, gmb_email: &str) -> LiasettingRequestgmbaccesCall<'a, S> {
        LiasettingRequestgmbaccesCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _account_id: account_id,
            _gmb_email: gmb_email.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Requests inventory validation for the specified country.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the managing account. If this parameter is not the same as accountId, then this account must be a multi-client account and `accountId` must be the ID of a sub-account of this account.
    /// * `accountId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `country` - The country for which inventory validation is requested.
    pub fn requestinventoryverification(&self, merchant_id: u64, account_id: u64, country: &str) -> LiasettingRequestinventoryverificationCall<'a, S> {
        LiasettingRequestinventoryverificationCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _account_id: account_id,
            _country: country.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the inventory verification contract for the specified country.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the managing account. If this parameter is not the same as accountId, then this account must be a multi-client account and `accountId` must be the ID of a sub-account of this account.
    /// * `accountId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `country` - The country for which inventory verification is requested.
    /// * `language` - The language for which inventory verification is requested.
    /// * `contactName` - The name of the inventory verification contact.
    /// * `contactEmail` - The email of the inventory verification contact.
    pub fn setinventoryverificationcontact(&self, merchant_id: u64, account_id: u64, country: &str, language: &str, contact_name: &str, contact_email: &str) -> LiasettingSetinventoryverificationcontactCall<'a, S> {
        LiasettingSetinventoryverificationcontactCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _account_id: account_id,
            _country: country.to_string(),
            _language: language.to_string(),
            _contact_name: contact_name.to_string(),
            _contact_email: contact_email.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the POS data provider for the specified country.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the managing account. If this parameter is not the same as accountId, then this account must be a multi-client account and `accountId` must be the ID of a sub-account of this account.
    /// * `accountId` - The ID of the account for which to retrieve accessible Google My Business accounts.
    /// * `country` - The country for which the POS data provider is selected.
    pub fn setposdataprovider(&self, merchant_id: u64, account_id: u64, country: &str) -> LiasettingSetposdataproviderCall<'a, S> {
        LiasettingSetposdataproviderCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _account_id: account_id,
            _country: country.to_string(),
            _pos_external_account_id: Default::default(),
            _pos_data_provider_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the LIA settings of the account. Any fields that are not provided are deleted from the resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the managing account. If this parameter is not the same as accountId, then this account must be a multi-client account and `accountId` must be the ID of a sub-account of this account.
    /// * `accountId` - The ID of the account for which to get or update LIA settings.
    pub fn update(&self, request: LiaSettings, merchant_id: u64, account_id: u64) -> LiasettingUpdateCall<'a, S> {
        LiasettingUpdateCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _account_id: account_id,
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *orderinvoice* resources.
/// It is not used directly, but through the [`ShoppingContent`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_content2 as content2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use content2::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `createchargeinvoice(...)` and `createrefundinvoice(...)`
/// // to build up your call.
/// let rb = hub.orderinvoices();
/// # }
/// ```
pub struct OrderinvoiceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ShoppingContent<S>,
}

impl<'a, S> client::MethodsBuilder for OrderinvoiceMethods<'a, S> {}

impl<'a, S> OrderinvoiceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a charge invoice for a shipment group, and triggers a charge capture for orderinvoice enabled orders.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn createchargeinvoice(&self, request: OrderinvoicesCreateChargeInvoiceRequest, merchant_id: u64, order_id: &str) -> OrderinvoiceCreatechargeinvoiceCall<'a, S> {
        OrderinvoiceCreatechargeinvoiceCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a refund invoice for one or more shipment groups, and triggers a refund for orderinvoice enabled orders. This can only be used for line items that have previously been charged using `createChargeInvoice`. All amounts (except for the summary) are incremental with respect to the previous invoice.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn createrefundinvoice(&self, request: OrderinvoicesCreateRefundInvoiceRequest, merchant_id: u64, order_id: &str) -> OrderinvoiceCreaterefundinvoiceCall<'a, S> {
        OrderinvoiceCreaterefundinvoiceCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *orderreport* resources.
/// It is not used directly, but through the [`ShoppingContent`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_content2 as content2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use content2::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `listdisbursements(...)` and `listtransactions(...)`
/// // to build up your call.
/// let rb = hub.orderreports();
/// # }
/// ```
pub struct OrderreportMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ShoppingContent<S>,
}

impl<'a, S> client::MethodsBuilder for OrderreportMethods<'a, S> {}

impl<'a, S> OrderreportMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a report for disbursements from your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    pub fn listdisbursements(&self, merchant_id: u64) -> OrderreportListdisbursementCall<'a, S> {
        OrderreportListdisbursementCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _disbursement_start_date: Default::default(),
            _disbursement_end_date: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of transactions for a disbursement from your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `disbursementId` - The Google-provided ID of the disbursement (found in Wallet).
    pub fn listtransactions(&self, merchant_id: u64, disbursement_id: &str) -> OrderreportListtransactionCall<'a, S> {
        OrderreportListtransactionCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _disbursement_id: disbursement_id.to_string(),
            _transaction_start_date: Default::default(),
            _transaction_end_date: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *orderreturn* resources.
/// It is not used directly, but through the [`ShoppingContent`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_content2 as content2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use content2::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.orderreturns();
/// # }
/// ```
pub struct OrderreturnMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ShoppingContent<S>,
}

impl<'a, S> client::MethodsBuilder for OrderreturnMethods<'a, S> {}

impl<'a, S> OrderreturnMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an order return from your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `returnId` - Merchant order return ID generated by Google.
    pub fn get(&self, merchant_id: u64, return_id: &str) -> OrderreturnGetCall<'a, S> {
        OrderreturnGetCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _return_id: return_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists order returns in your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    pub fn list(&self, merchant_id: u64) -> OrderreturnListCall<'a, S> {
        OrderreturnListCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _created_start_date: Default::default(),
            _created_end_date: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *order* resources.
/// It is not used directly, but through the [`ShoppingContent`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_content2 as content2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use content2::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `acknowledge(...)`, `advancetestorder(...)`, `cancel(...)`, `cancellineitem(...)`, `canceltestorderbycustomer(...)`, `createtestorder(...)`, `createtestreturn(...)`, `custombatch(...)`, `get(...)`, `getbymerchantorderid(...)`, `gettestordertemplate(...)`, `instorerefundlineitem(...)`, `list(...)`, `refund(...)`, `rejectreturnlineitem(...)`, `returnlineitem(...)`, `returnrefundlineitem(...)`, `setlineitemmetadata(...)`, `shiplineitems(...)`, `updatelineitemshippingdetails(...)`, `updatemerchantorderid(...)` and `updateshipment(...)`
/// // to build up your call.
/// let rb = hub.orders();
/// # }
/// ```
pub struct OrderMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ShoppingContent<S>,
}

impl<'a, S> client::MethodsBuilder for OrderMethods<'a, S> {}

impl<'a, S> OrderMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Marks an order as acknowledged.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn acknowledge(&self, request: OrdersAcknowledgeRequest, merchant_id: u64, order_id: &str) -> OrderAcknowledgeCall<'a, S> {
        OrderAcknowledgeCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sandbox only. Moves a test order from state "`inProgress`" to state "`pendingShipment`".
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the test order to modify.
    pub fn advancetestorder(&self, merchant_id: u64, order_id: &str) -> OrderAdvancetestorderCall<'a, S> {
        OrderAdvancetestorderCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancels all line items in an order, making a full refund.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order to cancel.
    pub fn cancel(&self, request: OrdersCancelRequest, merchant_id: u64, order_id: &str) -> OrderCancelCall<'a, S> {
        OrderCancelCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancels a line item, making a full refund.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn cancellineitem(&self, request: OrdersCancelLineItemRequest, merchant_id: u64, order_id: &str) -> OrderCancellineitemCall<'a, S> {
        OrderCancellineitemCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sandbox only. Cancels a test order for customer-initiated cancellation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the test order to cancel.
    pub fn canceltestorderbycustomer(&self, request: OrdersCancelTestOrderByCustomerRequest, merchant_id: u64, order_id: &str) -> OrderCanceltestorderbycustomerCall<'a, S> {
        OrderCanceltestorderbycustomerCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sandbox only. Creates a test order.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that should manage the order. This cannot be a multi-client account.
    pub fn createtestorder(&self, request: OrdersCreateTestOrderRequest, merchant_id: u64) -> OrderCreatetestorderCall<'a, S> {
        OrderCreatetestorderCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sandbox only. Creates a test return.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn createtestreturn(&self, request: OrdersCreateTestReturnRequest, merchant_id: u64, order_id: &str) -> OrderCreatetestreturnCall<'a, S> {
        OrderCreatetestreturnCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves or modifies multiple orders in a single request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn custombatch(&self, request: OrdersCustomBatchRequest) -> OrderCustombatchCall<'a, S> {
        OrderCustombatchCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an order from your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn get(&self, merchant_id: u64, order_id: &str) -> OrderGetCall<'a, S> {
        OrderGetCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an order using merchant order ID.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `merchantOrderId` - The merchant order ID to be looked for.
    pub fn getbymerchantorderid(&self, merchant_id: u64, merchant_order_id: &str) -> OrderGetbymerchantorderidCall<'a, S> {
        OrderGetbymerchantorderidCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _merchant_order_id: merchant_order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sandbox only. Retrieves an order template that can be used to quickly create a new order in sandbox.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that should manage the order. This cannot be a multi-client account.
    /// * `templateName` - The name of the template to retrieve.
    pub fn gettestordertemplate(&self, merchant_id: u64, template_name: &OrderTemplateNameEnum) -> OrderGettestordertemplateCall<'a, S> {
        OrderGettestordertemplateCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _template_name: template_name.clone(),
            _country: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deprecated. Notifies that item return and refund was handled directly by merchant outside of Google payments processing (e.g. cash refund done in store). Note: We recommend calling the returnrefundlineitem method to refund in-store returns. We will issue the refund directly to the customer. This helps to prevent possible differences arising between merchant and Google transaction records. We also recommend having the point of sale system communicate with Google to ensure that customers do not receive a double refund by first refunding via Google then via an in-store return.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn instorerefundlineitem(&self, request: OrdersInStoreRefundLineItemRequest, merchant_id: u64, order_id: &str) -> OrderInstorerefundlineitemCall<'a, S> {
        OrderInstorerefundlineitemCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the orders in your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    pub fn list(&self, merchant_id: u64) -> OrderListCall<'a, S> {
        OrderListCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _statuses: Default::default(),
            _placed_date_start: Default::default(),
            _placed_date_end: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _acknowledged: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deprecated, please use returnRefundLineItem instead.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order to refund.
    pub fn refund(&self, request: OrdersRefundRequest, merchant_id: u64, order_id: &str) -> OrderRefundCall<'a, S> {
        OrderRefundCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Rejects return on an line item.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn rejectreturnlineitem(&self, request: OrdersRejectReturnLineItemRequest, merchant_id: u64, order_id: &str) -> OrderRejectreturnlineitemCall<'a, S> {
        OrderRejectreturnlineitemCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a line item.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn returnlineitem(&self, request: OrdersReturnLineItemRequest, merchant_id: u64, order_id: &str) -> OrderReturnlineitemCall<'a, S> {
        OrderReturnlineitemCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns and refunds a line item. Note that this method can only be called on fully shipped orders. Please also note that the Orderreturns API is the preferred way to handle returns after you receive a return from a customer. You can use Orderreturns.list or Orderreturns.get to search for the return, and then use Orderreturns.processreturn to issue the refund. If the return cannot be found, then we recommend using this API to issue a refund.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn returnrefundlineitem(&self, request: OrdersReturnRefundLineItemRequest, merchant_id: u64, order_id: &str) -> OrderReturnrefundlineitemCall<'a, S> {
        OrderReturnrefundlineitemCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets (or overrides if it already exists) merchant provided annotations in the form of key-value pairs. A common use case would be to supply us with additional structured information about a line item that cannot be provided via other methods. Submitted key-value pairs can be retrieved as part of the orders resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn setlineitemmetadata(&self, request: OrdersSetLineItemMetadataRequest, merchant_id: u64, order_id: &str) -> OrderSetlineitemmetadataCall<'a, S> {
        OrderSetlineitemmetadataCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Marks line item(s) as shipped.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn shiplineitems(&self, request: OrdersShipLineItemsRequest, merchant_id: u64, order_id: &str) -> OrderShiplineitemCall<'a, S> {
        OrderShiplineitemCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates ship by and delivery by dates for a line item.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn updatelineitemshippingdetails(&self, request: OrdersUpdateLineItemShippingDetailsRequest, merchant_id: u64, order_id: &str) -> OrderUpdatelineitemshippingdetailCall<'a, S> {
        OrderUpdatelineitemshippingdetailCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the merchant order ID for a given order.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn updatemerchantorderid(&self, request: OrdersUpdateMerchantOrderIdRequest, merchant_id: u64, order_id: &str) -> OrderUpdatemerchantorderidCall<'a, S> {
        OrderUpdatemerchantorderidCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a shipment's status, carrier, and/or tracking ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn updateshipment(&self, request: OrdersUpdateShipmentRequest, merchant_id: u64, order_id: &str) -> OrderUpdateshipmentCall<'a, S> {
        OrderUpdateshipmentCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *po* resources.
/// It is not used directly, but through the [`ShoppingContent`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_content2 as content2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use content2::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `custombatch(...)`, `delete(...)`, `get(...)`, `insert(...)`, `inventory(...)`, `list(...)` and `sale(...)`
/// // to build up your call.
/// let rb = hub.pos();
/// # }
/// ```
pub struct PoMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ShoppingContent<S>,
}

impl<'a, S> client::MethodsBuilder for PoMethods<'a, S> {}

impl<'a, S> PoMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Batches multiple POS-related calls in a single request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn custombatch(&self, request: PosCustomBatchRequest) -> PoCustombatchCall<'a, S> {
        PoCustombatchCall {
            hub: self.hub,
            _request: request,
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a store for the given merchant.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the POS or inventory data provider.
    /// * `targetMerchantId` - The ID of the target merchant.
    /// * `storeCode` - A store code that is unique per merchant.
    pub fn delete(&self, merchant_id: u64, target_merchant_id: u64, store_code: &str) -> PoDeleteCall<'a, S> {
        PoDeleteCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _target_merchant_id: target_merchant_id,
            _store_code: store_code.to_string(),
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves information about the given store.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the POS or inventory data provider.
    /// * `targetMerchantId` - The ID of the target merchant.
    /// * `storeCode` - A store code that is unique per merchant.
    pub fn get(&self, merchant_id: u64, target_merchant_id: u64, store_code: &str) -> PoGetCall<'a, S> {
        PoGetCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _target_merchant_id: target_merchant_id,
            _store_code: store_code.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a store for the given merchant.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the POS or inventory data provider.
    /// * `targetMerchantId` - The ID of the target merchant.
    pub fn insert(&self, request: PosStore, merchant_id: u64, target_merchant_id: u64) -> PoInsertCall<'a, S> {
        PoInsertCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _target_merchant_id: target_merchant_id,
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Submit inventory for the given merchant.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the POS or inventory data provider.
    /// * `targetMerchantId` - The ID of the target merchant.
    pub fn inventory(&self, request: PosInventoryRequest, merchant_id: u64, target_merchant_id: u64) -> PoInventoryCall<'a, S> {
        PoInventoryCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _target_merchant_id: target_merchant_id,
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the stores of the target merchant.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the POS or inventory data provider.
    /// * `targetMerchantId` - The ID of the target merchant.
    pub fn list(&self, merchant_id: u64, target_merchant_id: u64) -> PoListCall<'a, S> {
        PoListCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _target_merchant_id: target_merchant_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Submit a sale event for the given merchant.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the POS or inventory data provider.
    /// * `targetMerchantId` - The ID of the target merchant.
    pub fn sale(&self, request: PosSaleRequest, merchant_id: u64, target_merchant_id: u64) -> PoSaleCall<'a, S> {
        PoSaleCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _target_merchant_id: target_merchant_id,
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *product* resources.
/// It is not used directly, but through the [`ShoppingContent`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_content2 as content2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use content2::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `custombatch(...)`, `delete(...)`, `get(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.products();
/// # }
/// ```
pub struct ProductMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ShoppingContent<S>,
}

impl<'a, S> client::MethodsBuilder for ProductMethods<'a, S> {}

impl<'a, S> ProductMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves, inserts, and deletes multiple products in a single request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn custombatch(&self, request: ProductsCustomBatchRequest) -> ProductCustombatchCall<'a, S> {
        ProductCustombatchCall {
            hub: self.hub,
            _request: request,
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a product from your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that contains the product. This account cannot be a multi-client account.
    /// * `productId` - The REST ID of the product.
    pub fn delete(&self, merchant_id: u64, product_id: &str) -> ProductDeleteCall<'a, S> {
        ProductDeleteCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _product_id: product_id.to_string(),
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a product from your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that contains the product. This account cannot be a multi-client account.
    /// * `productId` - The REST ID of the product.
    pub fn get(&self, merchant_id: u64, product_id: &str) -> ProductGetCall<'a, S> {
        ProductGetCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _product_id: product_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads a product to your Merchant Center account. If an item with the same channel, contentLanguage, offerId, and targetCountry already exists, this method updates that entry.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that contains the product. This account cannot be a multi-client account.
    pub fn insert(&self, request: Product, merchant_id: u64) -> ProductInsertCall<'a, S> {
        ProductInsertCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the products in your Merchant Center account. The response might contain fewer items than specified by maxResults. Rely on nextPageToken to determine if there are more items to be requested.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that contains the products. This account cannot be a multi-client account.
    pub fn list(&self, merchant_id: u64) -> ProductListCall<'a, S> {
        ProductListCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _include_invalid_inserted_items: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *productstatus* resources.
/// It is not used directly, but through the [`ShoppingContent`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_content2 as content2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use content2::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `custombatch(...)`, `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.productstatuses();
/// # }
/// ```
pub struct ProductstatusMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ShoppingContent<S>,
}

impl<'a, S> client::MethodsBuilder for ProductstatusMethods<'a, S> {}

impl<'a, S> ProductstatusMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the statuses of multiple products in a single request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn custombatch(&self, request: ProductstatusesCustomBatchRequest) -> ProductstatusCustombatchCall<'a, S> {
        ProductstatusCustombatchCall {
            hub: self.hub,
            _request: request,
            _include_attributes: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the status of a product from your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that contains the product. This account cannot be a multi-client account.
    /// * `productId` - The REST ID of the product.
    pub fn get(&self, merchant_id: u64, product_id: &str) -> ProductstatusGetCall<'a, S> {
        ProductstatusGetCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _product_id: product_id.to_string(),
            _include_attributes: Default::default(),
            _destinations: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the statuses of the products in your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that contains the products. This account cannot be a multi-client account.
    pub fn list(&self, merchant_id: u64) -> ProductstatusListCall<'a, S> {
        ProductstatusListCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _include_invalid_inserted_items: Default::default(),
            _include_attributes: Default::default(),
            _destinations: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *shippingsetting* resources.
/// It is not used directly, but through the [`ShoppingContent`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_content2 as content2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use content2::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `custombatch(...)`, `get(...)`, `getsupportedcarriers(...)`, `getsupportedholidays(...)`, `getsupportedpickupservices(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.shippingsettings();
/// # }
/// ```
pub struct ShippingsettingMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ShoppingContent<S>,
}

impl<'a, S> client::MethodsBuilder for ShippingsettingMethods<'a, S> {}

impl<'a, S> ShippingsettingMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves and updates the shipping settings of multiple accounts in a single request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn custombatch(&self, request: ShippingsettingsCustomBatchRequest) -> ShippingsettingCustombatchCall<'a, S> {
        ShippingsettingCustombatchCall {
            hub: self.hub,
            _request: request,
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the shipping settings of the account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the managing account. If this parameter is not the same as accountId, then this account must be a multi-client account and `accountId` must be the ID of a sub-account of this account.
    /// * `accountId` - The ID of the account for which to get/update shipping settings.
    pub fn get(&self, merchant_id: u64, account_id: u64) -> ShippingsettingGetCall<'a, S> {
        ShippingsettingGetCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _account_id: account_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves supported carriers and carrier services for an account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account for which to retrieve the supported carriers.
    pub fn getsupportedcarriers(&self, merchant_id: u64) -> ShippingsettingGetsupportedcarrierCall<'a, S> {
        ShippingsettingGetsupportedcarrierCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves supported holidays for an account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account for which to retrieve the supported holidays.
    pub fn getsupportedholidays(&self, merchant_id: u64) -> ShippingsettingGetsupportedholidayCall<'a, S> {
        ShippingsettingGetsupportedholidayCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves supported pickup services for an account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account for which to retrieve the supported pickup services.
    pub fn getsupportedpickupservices(&self, merchant_id: u64) -> ShippingsettingGetsupportedpickupserviceCall<'a, S> {
        ShippingsettingGetsupportedpickupserviceCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the shipping settings of the sub-accounts in your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the managing account. This must be a multi-client account.
    pub fn list(&self, merchant_id: u64) -> ShippingsettingListCall<'a, S> {
        ShippingsettingListCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the shipping settings of the account. Any fields that are not provided are deleted from the resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the managing account. If this parameter is not the same as accountId, then this account must be a multi-client account and `accountId` must be the ID of a sub-account of this account.
    /// * `accountId` - The ID of the account for which to get/update shipping settings.
    pub fn update(&self, request: ShippingSettings, merchant_id: u64, account_id: u64) -> ShippingsettingUpdateCall<'a, S> {
        ShippingsettingUpdateCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _account_id: account_id,
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



