use super::*;
/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the [`AdSense`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adsense1d4 as adsense1d4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adsense1d4::{AdSense, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdSense::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `adclients_get_ad_code(...)`, `adclients_list(...)`, `adunits_customchannels_list(...)`, `adunits_get(...)`, `adunits_get_ad_code(...)`, `adunits_list(...)`, `alerts_delete(...)`, `alerts_list(...)`, `customchannels_adunits_list(...)`, `customchannels_get(...)`, `customchannels_list(...)`, `get(...)`, `list(...)`, `payments_list(...)`, `reports_generate(...)`, `reports_saved_generate(...)`, `reports_saved_list(...)`, `savedadstyles_get(...)`, `savedadstyles_list(...)` and `urlchannels_list(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdSense<S>,
}

impl<'a, S> client::MethodsBuilder for AccountMethods<'a, S> {}

impl<'a, S> AccountMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get Auto ad code for a given ad client.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account which contains the ad client.
    /// * `adClientId` - Ad client to get the code for.
    pub fn adclients_get_ad_code(&self, account_id: &str, ad_client_id: &str) -> AccountAdclientGetAdCodeCall<'a, S> {
        AccountAdclientGetAdCodeCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _ad_client_id: ad_client_id.to_string(),
            _tag_partner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all ad clients in the specified account.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account for which to list ad clients.
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
    /// List all custom channels which the specified ad unit belongs to.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account to which the ad client belongs.
    /// * `adClientId` - Ad client which contains the ad unit.
    /// * `adUnitId` - Ad unit for which to list custom channels.
    pub fn adunits_customchannels_list(&self, account_id: &str, ad_client_id: &str, ad_unit_id: &str) -> AccountAdunitCustomchannelListCall<'a, S> {
        AccountAdunitCustomchannelListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _ad_client_id: ad_client_id.to_string(),
            _ad_unit_id: ad_unit_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified ad unit in the specified ad client for the specified account.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account to which the ad client belongs.
    /// * `adClientId` - Ad client for which to get the ad unit.
    /// * `adUnitId` - Ad unit to retrieve.
    pub fn adunits_get(&self, account_id: &str, ad_client_id: &str, ad_unit_id: &str) -> AccountAdunitGetCall<'a, S> {
        AccountAdunitGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _ad_client_id: ad_client_id.to_string(),
            _ad_unit_id: ad_unit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get ad code for the specified ad unit.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account which contains the ad client.
    /// * `adClientId` - Ad client with contains the ad unit.
    /// * `adUnitId` - Ad unit to get the code for.
    pub fn adunits_get_ad_code(&self, account_id: &str, ad_client_id: &str, ad_unit_id: &str) -> AccountAdunitGetAdCodeCall<'a, S> {
        AccountAdunitGetAdCodeCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _ad_client_id: ad_client_id.to_string(),
            _ad_unit_id: ad_unit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all ad units in the specified ad client for the specified account.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account to which the ad client belongs.
    /// * `adClientId` - Ad client for which to list ad units.
    pub fn adunits_list(&self, account_id: &str, ad_client_id: &str) -> AccountAdunitListCall<'a, S> {
        AccountAdunitListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _ad_client_id: ad_client_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _include_inactive: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Dismiss (delete) the specified alert from the specified publisher AdSense account.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account which contains the ad unit.
    /// * `alertId` - Alert to delete.
    pub fn alerts_delete(&self, account_id: &str, alert_id: &str) -> AccountAlertDeleteCall<'a, S> {
        AccountAlertDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _alert_id: alert_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the alerts for the specified AdSense account.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account for which to retrieve the alerts.
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
    /// List all ad units in the specified custom channel.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account to which the ad client belongs.
    /// * `adClientId` - Ad client which contains the custom channel.
    /// * `customChannelId` - Custom channel for which to list ad units.
    pub fn customchannels_adunits_list(&self, account_id: &str, ad_client_id: &str, custom_channel_id: &str) -> AccountCustomchannelAdunitListCall<'a, S> {
        AccountCustomchannelAdunitListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _ad_client_id: ad_client_id.to_string(),
            _custom_channel_id: custom_channel_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _include_inactive: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get the specified custom channel from the specified ad client for the specified account.
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
    /// List all custom channels in the specified ad client for the specified account.
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
    /// List the payments for the specified AdSense account.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account for which to retrieve the payments.
    pub fn payments_list(&self, account_id: &str) -> AccountPaymentListCall<'a, S> {
        AccountPaymentListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generate an AdSense report based on the saved report ID sent in the query parameters.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account to which the saved reports belong.
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
    /// List all saved reports in the specified AdSense account.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account to which the saved reports belong.
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
    /// Generate an AdSense report based on the report request sent in the query parameters. Returns the result as JSON; to retrieve output in CSV format specify "alt=csv" as a query parameter.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account upon which to report.
    /// * `startDate` - Start of the date range to report on in "YYYY-MM-DD" format, inclusive.
    /// * `endDate` - End of the date range to report on in "YYYY-MM-DD" format, inclusive.
    pub fn reports_generate(&self, account_id: &str, start_date: &str, end_date: &str) -> AccountReportGenerateCall<'a, S> {
        AccountReportGenerateCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _start_date: start_date.to_string(),
            _end_date: end_date.to_string(),
            _use_timezone_reporting: Default::default(),
            _start_index: Default::default(),
            _sort: Default::default(),
            _metric: Default::default(),
            _max_results: Default::default(),
            _locale: Default::default(),
            _filter: Default::default(),
            _dimension: Default::default(),
            _currency: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List a specific saved ad style for the specified account.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account for which to get the saved ad style.
    /// * `savedAdStyleId` - Saved ad style to retrieve.
    pub fn savedadstyles_get(&self, account_id: &str, saved_ad_style_id: &str) -> AccountSavedadstyleGetCall<'a, S> {
        AccountSavedadstyleGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _saved_ad_style_id: saved_ad_style_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all saved ad styles in the specified account.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account for which to list saved ad styles.
    pub fn savedadstyles_list(&self, account_id: &str) -> AccountSavedadstyleListCall<'a, S> {
        AccountSavedadstyleListCall {
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
    /// List all URL channels in the specified ad client for the specified account.
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
    /// Get information about the selected AdSense account.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account to get information about.
    pub fn get(&self, account_id: &str) -> AccountGetCall<'a, S> {
        AccountGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _tree: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all accounts available to this AdSense account.
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



/// A builder providing access to all methods supported on *adclient* resources.
/// It is not used directly, but through the [`AdSense`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adsense1d4 as adsense1d4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adsense1d4::{AdSense, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdSense::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.adclients();
/// # }
/// ```
pub struct AdclientMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdSense<S>,
}

impl<'a, S> client::MethodsBuilder for AdclientMethods<'a, S> {}

impl<'a, S> AdclientMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all ad clients in this AdSense account.
    pub fn list(&self) -> AdclientListCall<'a, S> {
        AdclientListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *adunit* resources.
/// It is not used directly, but through the [`AdSense`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adsense1d4 as adsense1d4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adsense1d4::{AdSense, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdSense::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `customchannels_list(...)`, `get(...)`, `get_ad_code(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.adunits();
/// # }
/// ```
pub struct AdunitMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdSense<S>,
}

impl<'a, S> client::MethodsBuilder for AdunitMethods<'a, S> {}

impl<'a, S> AdunitMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all custom channels which the specified ad unit belongs to.
    /// 
    /// # Arguments
    ///
    /// * `adClientId` - Ad client which contains the ad unit.
    /// * `adUnitId` - Ad unit for which to list custom channels.
    pub fn customchannels_list(&self, ad_client_id: &str, ad_unit_id: &str) -> AdunitCustomchannelListCall<'a, S> {
        AdunitCustomchannelListCall {
            hub: self.hub,
            _ad_client_id: ad_client_id.to_string(),
            _ad_unit_id: ad_unit_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified ad unit in the specified ad client.
    /// 
    /// # Arguments
    ///
    /// * `adClientId` - Ad client for which to get the ad unit.
    /// * `adUnitId` - Ad unit to retrieve.
    pub fn get(&self, ad_client_id: &str, ad_unit_id: &str) -> AdunitGetCall<'a, S> {
        AdunitGetCall {
            hub: self.hub,
            _ad_client_id: ad_client_id.to_string(),
            _ad_unit_id: ad_unit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get ad code for the specified ad unit.
    /// 
    /// # Arguments
    ///
    /// * `adClientId` - Ad client with contains the ad unit.
    /// * `adUnitId` - Ad unit to get the code for.
    pub fn get_ad_code(&self, ad_client_id: &str, ad_unit_id: &str) -> AdunitGetAdCodeCall<'a, S> {
        AdunitGetAdCodeCall {
            hub: self.hub,
            _ad_client_id: ad_client_id.to_string(),
            _ad_unit_id: ad_unit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all ad units in the specified ad client for this AdSense account.
    /// 
    /// # Arguments
    ///
    /// * `adClientId` - Ad client for which to list ad units.
    pub fn list(&self, ad_client_id: &str) -> AdunitListCall<'a, S> {
        AdunitListCall {
            hub: self.hub,
            _ad_client_id: ad_client_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _include_inactive: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *alert* resources.
/// It is not used directly, but through the [`AdSense`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adsense1d4 as adsense1d4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adsense1d4::{AdSense, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdSense::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.alerts();
/// # }
/// ```
pub struct AlertMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdSense<S>,
}

impl<'a, S> client::MethodsBuilder for AlertMethods<'a, S> {}

impl<'a, S> AlertMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Dismiss (delete) the specified alert from the publisher's AdSense account.
    /// 
    /// # Arguments
    ///
    /// * `alertId` - Alert to delete.
    pub fn delete(&self, alert_id: &str) -> AlertDeleteCall<'a, S> {
        AlertDeleteCall {
            hub: self.hub,
            _alert_id: alert_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the alerts for this AdSense account.
    pub fn list(&self) -> AlertListCall<'a, S> {
        AlertListCall {
            hub: self.hub,
            _locale: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *customchannel* resources.
/// It is not used directly, but through the [`AdSense`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adsense1d4 as adsense1d4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adsense1d4::{AdSense, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdSense::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `adunits_list(...)`, `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.customchannels();
/// # }
/// ```
pub struct CustomchannelMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdSense<S>,
}

impl<'a, S> client::MethodsBuilder for CustomchannelMethods<'a, S> {}

impl<'a, S> CustomchannelMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all ad units in the specified custom channel.
    /// 
    /// # Arguments
    ///
    /// * `adClientId` - Ad client which contains the custom channel.
    /// * `customChannelId` - Custom channel for which to list ad units.
    pub fn adunits_list(&self, ad_client_id: &str, custom_channel_id: &str) -> CustomchannelAdunitListCall<'a, S> {
        CustomchannelAdunitListCall {
            hub: self.hub,
            _ad_client_id: ad_client_id.to_string(),
            _custom_channel_id: custom_channel_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _include_inactive: Default::default(),
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
    /// * `adClientId` - Ad client which contains the custom channel.
    /// * `customChannelId` - Custom channel to retrieve.
    pub fn get(&self, ad_client_id: &str, custom_channel_id: &str) -> CustomchannelGetCall<'a, S> {
        CustomchannelGetCall {
            hub: self.hub,
            _ad_client_id: ad_client_id.to_string(),
            _custom_channel_id: custom_channel_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all custom channels in the specified ad client for this AdSense account.
    /// 
    /// # Arguments
    ///
    /// * `adClientId` - Ad client for which to list custom channels.
    pub fn list(&self, ad_client_id: &str) -> CustomchannelListCall<'a, S> {
        CustomchannelListCall {
            hub: self.hub,
            _ad_client_id: ad_client_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *metadata* resources.
/// It is not used directly, but through the [`AdSense`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adsense1d4 as adsense1d4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adsense1d4::{AdSense, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdSense::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `dimensions_list(...)` and `metrics_list(...)`
/// // to build up your call.
/// let rb = hub.metadata();
/// # }
/// ```
pub struct MetadataMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdSense<S>,
}

impl<'a, S> client::MethodsBuilder for MetadataMethods<'a, S> {}

impl<'a, S> MetadataMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the metadata for the dimensions available to this AdSense account.
    pub fn dimensions_list(&self) -> MetadataDimensionListCall<'a, S> {
        MetadataDimensionListCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the metadata for the metrics available to this AdSense account.
    pub fn metrics_list(&self) -> MetadataMetricListCall<'a, S> {
        MetadataMetricListCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *payment* resources.
/// It is not used directly, but through the [`AdSense`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adsense1d4 as adsense1d4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adsense1d4::{AdSense, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdSense::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.payments();
/// # }
/// ```
pub struct PaymentMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdSense<S>,
}

impl<'a, S> client::MethodsBuilder for PaymentMethods<'a, S> {}

impl<'a, S> PaymentMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the payments for this AdSense account.
    pub fn list(&self) -> PaymentListCall<'a, S> {
        PaymentListCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *report* resources.
/// It is not used directly, but through the [`AdSense`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adsense1d4 as adsense1d4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adsense1d4::{AdSense, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdSense::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `generate(...)`, `saved_generate(...)` and `saved_list(...)`
/// // to build up your call.
/// let rb = hub.reports();
/// # }
/// ```
pub struct ReportMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdSense<S>,
}

impl<'a, S> client::MethodsBuilder for ReportMethods<'a, S> {}

impl<'a, S> ReportMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generate an AdSense report based on the saved report ID sent in the query parameters.
    /// 
    /// # Arguments
    ///
    /// * `savedReportId` - The saved report to retrieve.
    pub fn saved_generate(&self, saved_report_id: &str) -> ReportSavedGenerateCall<'a, S> {
        ReportSavedGenerateCall {
            hub: self.hub,
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
    /// List all saved reports in this AdSense account.
    pub fn saved_list(&self) -> ReportSavedListCall<'a, S> {
        ReportSavedListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generate an AdSense report based on the report request sent in the query parameters. Returns the result as JSON; to retrieve output in CSV format specify "alt=csv" as a query parameter.
    /// 
    /// # Arguments
    ///
    /// * `startDate` - Start of the date range to report on in "YYYY-MM-DD" format, inclusive.
    /// * `endDate` - End of the date range to report on in "YYYY-MM-DD" format, inclusive.
    pub fn generate(&self, start_date: &str, end_date: &str) -> ReportGenerateCall<'a, S> {
        ReportGenerateCall {
            hub: self.hub,
            _start_date: start_date.to_string(),
            _end_date: end_date.to_string(),
            _use_timezone_reporting: Default::default(),
            _start_index: Default::default(),
            _sort: Default::default(),
            _metric: Default::default(),
            _max_results: Default::default(),
            _locale: Default::default(),
            _filter: Default::default(),
            _dimension: Default::default(),
            _currency: Default::default(),
            _account_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *savedadstyle* resources.
/// It is not used directly, but through the [`AdSense`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adsense1d4 as adsense1d4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adsense1d4::{AdSense, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdSense::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.savedadstyles();
/// # }
/// ```
pub struct SavedadstyleMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdSense<S>,
}

impl<'a, S> client::MethodsBuilder for SavedadstyleMethods<'a, S> {}

impl<'a, S> SavedadstyleMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a specific saved ad style from the user's account.
    /// 
    /// # Arguments
    ///
    /// * `savedAdStyleId` - Saved ad style to retrieve.
    pub fn get(&self, saved_ad_style_id: &str) -> SavedadstyleGetCall<'a, S> {
        SavedadstyleGetCall {
            hub: self.hub,
            _saved_ad_style_id: saved_ad_style_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all saved ad styles in the user's account.
    pub fn list(&self) -> SavedadstyleListCall<'a, S> {
        SavedadstyleListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *urlchannel* resources.
/// It is not used directly, but through the [`AdSense`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adsense1d4 as adsense1d4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adsense1d4::{AdSense, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdSense::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.urlchannels();
/// # }
/// ```
pub struct UrlchannelMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdSense<S>,
}

impl<'a, S> client::MethodsBuilder for UrlchannelMethods<'a, S> {}

impl<'a, S> UrlchannelMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all URL channels in the specified ad client for this AdSense account.
    /// 
    /// # Arguments
    ///
    /// * `adClientId` - Ad client for which to list URL channels.
    pub fn list(&self, ad_client_id: &str) -> UrlchannelListCall<'a, S> {
        UrlchannelListCall {
            hub: self.hub,
            _ad_client_id: ad_client_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



