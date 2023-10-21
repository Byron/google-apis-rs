use super::*;
/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the [`AdExchangeSeller`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexchangeseller2 as adexchangeseller2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adexchangeseller2::{AdExchangeSeller, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdExchangeSeller::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `adclients_list(...)`, `alerts_list(...)`, `customchannels_get(...)`, `customchannels_list(...)`, `get(...)`, `list(...)`, `metadata_dimensions_list(...)`, `metadata_metrics_list(...)`, `preferreddeals_get(...)`, `preferreddeals_list(...)`, `reports_generate(...)`, `reports_saved_generate(...)`, `reports_saved_list(...)` and `urlchannels_list(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdExchangeSeller<S>,
}

impl<'a, S> client::MethodsBuilder for AccountMethods<'a, S> {}

impl<'a, S> AccountMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all ad clients in this Ad Exchange account.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account to which the ad client belongs.
    pub fn adclients_list(&self, account_id: &str) -> AccountAdclientListCall<'a, S> {
        AccountAdclientListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the alerts for this Ad Exchange account.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account owning the alerts.
    pub fn alerts_list(&self, account_id: &str) -> AccountAlertListCall<'a, S> {
        AccountAlertListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get the specified custom channel from the specified ad client.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account to which the ad client belongs.
    /// * `adClientId` - Ad client which contains the custom channel.
    /// * `customChannelId` - Custom channel to retrieve.
    pub fn customchannels_get(&self, account_id: &str, ad_client_id: &str, custom_channel_id: &str) -> AccountCustomchannelGetCall<'a, S> {
        AccountCustomchannelGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _ad_client_id: ad_client_id.to_string(),
            _custom_channel_id: custom_channel_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all custom channels in the specified ad client for this Ad Exchange account.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account to which the ad client belongs.
    /// * `adClientId` - Ad client for which to list custom channels.
    pub fn customchannels_list(&self, account_id: &str, ad_client_id: &str) -> AccountCustomchannelListCall<'a, S> {
        AccountCustomchannelListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _ad_client_id: ad_client_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the metadata for the dimensions available to this AdExchange account.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account with visibility to the dimensions.
    pub fn metadata_dimensions_list(&self, account_id: &str) -> AccountMetadataDimensionListCall<'a, S> {
        AccountMetadataDimensionListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the metadata for the metrics available to this AdExchange account.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account with visibility to the metrics.
    pub fn metadata_metrics_list(&self, account_id: &str) -> AccountMetadataMetricListCall<'a, S> {
        AccountMetadataMetricListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get information about the selected Ad Exchange Preferred Deal.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account owning the deal.
    /// * `dealId` - Preferred deal to get information about.
    pub fn preferreddeals_get(&self, account_id: &str, deal_id: &str) -> AccountPreferreddealGetCall<'a, S> {
        AccountPreferreddealGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _deal_id: deal_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the preferred deals for this Ad Exchange account.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account owning the deals.
    pub fn preferreddeals_list(&self, account_id: &str) -> AccountPreferreddealListCall<'a, S> {
        AccountPreferreddealListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generate an Ad Exchange report based on the saved report ID sent in the query parameters.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account owning the saved report.
    /// * `savedReportId` - The saved report to retrieve.
    pub fn reports_saved_generate(&self, account_id: &str, saved_report_id: &str) -> AccountReportSavedGenerateCall<'a, S> {
        AccountReportSavedGenerateCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _saved_report_id: saved_report_id.to_string(),
            _start_index: Default::default(),
            _max_results: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all saved reports in this Ad Exchange account.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account owning the saved reports.
    pub fn reports_saved_list(&self, account_id: &str) -> AccountReportSavedListCall<'a, S> {
        AccountReportSavedListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generate an Ad Exchange report based on the report request sent in the query parameters. Returns the result as JSON; to retrieve output in CSV format specify "alt=csv" as a query parameter.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account which owns the generated report.
    /// * `startDate` - Start of the date range to report on in "YYYY-MM-DD" format, inclusive.
    /// * `endDate` - End of the date range to report on in "YYYY-MM-DD" format, inclusive.
    pub fn reports_generate(&self, account_id: &str, start_date: &str, end_date: &str) -> AccountReportGenerateCall<'a, S> {
        AccountReportGenerateCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _start_date: start_date.to_string(),
            _end_date: end_date.to_string(),
            _start_index: Default::default(),
            _sort: Default::default(),
            _metric: Default::default(),
            _max_results: Default::default(),
            _locale: Default::default(),
            _filter: Default::default(),
            _dimension: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all URL channels in the specified ad client for this Ad Exchange account.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account to which the ad client belongs.
    /// * `adClientId` - Ad client for which to list URL channels.
    pub fn urlchannels_list(&self, account_id: &str, ad_client_id: &str) -> AccountUrlchannelListCall<'a, S> {
        AccountUrlchannelListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _ad_client_id: ad_client_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get information about the selected Ad Exchange account.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account to get information about. Tip: 'myaccount' is a valid ID.
    pub fn get(&self, account_id: &str) -> AccountGetCall<'a, S> {
        AccountGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all accounts available to this Ad Exchange account.
    pub fn list(&self) -> AccountListCall<'a, S> {
        AccountListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



