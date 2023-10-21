use super::*;
/// A builder providing access to all methods supported on *billingAccount* resources.
/// It is not used directly, but through the [`CloudBillingBudget`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_billingbudgets1_beta1 as billingbudgets1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use billingbudgets1_beta1::{CloudBillingBudget, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudBillingBudget::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `budgets_create(...)`, `budgets_delete(...)`, `budgets_get(...)`, `budgets_list(...)` and `budgets_patch(...)`
/// // to build up your call.
/// let rb = hub.billing_accounts();
/// # }
/// ```
pub struct BillingAccountMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudBillingBudget<S>,
}

impl<'a, S> client::MethodsBuilder for BillingAccountMethods<'a, S> {}

impl<'a, S> BillingAccountMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new budget. See [Quotas and limits](https://cloud.google.com/billing/quotas) for more information on the limits of the number of budgets you can create.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the billing account to create the budget in. Values are of the form `billingAccounts/{billingAccountId}`.
    pub fn budgets_create(&self, request: GoogleCloudBillingBudgetsV1beta1CreateBudgetRequest, parent: &str) -> BillingAccountBudgetCreateCall<'a, S> {
        BillingAccountBudgetCreateCall {
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
    /// Deletes a budget. Returns successfully if already deleted.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the budget to delete. Values are of the form `billingAccounts/{billingAccountId}/budgets/{budgetId}`.
    pub fn budgets_delete(&self, name: &str) -> BillingAccountBudgetDeleteCall<'a, S> {
        BillingAccountBudgetDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a budget. WARNING: There are some fields exposed on the Google Cloud Console that aren't available on this API. When reading from the API, you will not see these fields in the return value, though they may have been set in the Cloud Console.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of budget to get. Values are of the form `billingAccounts/{billingAccountId}/budgets/{budgetId}`.
    pub fn budgets_get(&self, name: &str) -> BillingAccountBudgetGetCall<'a, S> {
        BillingAccountBudgetGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of budgets for a billing account. WARNING: There are some fields exposed on the Google Cloud Console that aren't available on this API. When reading from the API, you will not see these fields in the return value, though they may have been set in the Cloud Console.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of billing account to list budgets under. Values are of the form `billingAccounts/{billingAccountId}`.
    pub fn budgets_list(&self, parent: &str) -> BillingAccountBudgetListCall<'a, S> {
        BillingAccountBudgetListCall {
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
    /// Updates a budget and returns the updated budget. WARNING: There are some fields exposed on the Google Cloud Console that aren't available on this API. Budget fields that are not exposed in this API will not be changed by this method.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name of the budget. The resource name implies the scope of a budget. Values are of the form `billingAccounts/{billingAccountId}/budgets/{budgetId}`.
    pub fn budgets_patch(&self, request: GoogleCloudBillingBudgetsV1beta1UpdateBudgetRequest, name: &str) -> BillingAccountBudgetPatchCall<'a, S> {
        BillingAccountBudgetPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



