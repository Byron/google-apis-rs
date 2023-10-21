use super::*;
/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the [`AdSenseHost`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adsensehost4d1 as adsensehost4d1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adsensehost4d1::{AdSenseHost, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdSenseHost::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `adclients_get(...)`, `adclients_list(...)`, `adunits_delete(...)`, `adunits_get(...)`, `adunits_get_ad_code(...)`, `adunits_insert(...)`, `adunits_list(...)`, `adunits_patch(...)`, `adunits_update(...)`, `get(...)`, `list(...)` and `reports_generate(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdSenseHost<S>,
}

impl<'a, S> client::MethodsBuilder for AccountMethods<'a, S> {}

impl<'a, S> AccountMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get information about one of the ad clients in the specified publisher's AdSense account.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account which contains the ad client.
    /// * `adClientId` - Ad client to get.
    pub fn adclients_get(&self, account_id: &str, ad_client_id: &str) -> AccountAdclientGetCall<'a, S> {
        AccountAdclientGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _ad_client_id: ad_client_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all hosted ad clients in the specified hosted account.
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
    /// Delete the specified ad unit from the specified publisher AdSense account.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account which contains the ad unit.
    /// * `adClientId` - Ad client for which to get ad unit.
    /// * `adUnitId` - Ad unit to delete.
    pub fn adunits_delete(&self, account_id: &str, ad_client_id: &str, ad_unit_id: &str) -> AccountAdunitDeleteCall<'a, S> {
        AccountAdunitDeleteCall {
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
    /// Get the specified host ad unit in this AdSense account.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account which contains the ad unit.
    /// * `adClientId` - Ad client for which to get ad unit.
    /// * `adUnitId` - Ad unit to get.
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
    /// Get ad code for the specified ad unit, attaching the specified host custom channels.
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
            _host_custom_channel_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Insert the supplied ad unit into the specified publisher AdSense account.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account which will contain the ad unit.
    /// * `adClientId` - Ad client into which to insert the ad unit.
    pub fn adunits_insert(&self, request: AdUnit, account_id: &str, ad_client_id: &str) -> AccountAdunitInsertCall<'a, S> {
        AccountAdunitInsertCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _ad_client_id: ad_client_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all ad units in the specified publisher's AdSense account.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account which contains the ad client.
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
    /// Update the supplied ad unit in the specified publisher AdSense account. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account which contains the ad client.
    /// * `adClientId` - Ad client which contains the ad unit.
    /// * `adUnitId` - Ad unit to get.
    pub fn adunits_patch(&self, request: AdUnit, account_id: &str, ad_client_id: &str, ad_unit_id: &str) -> AccountAdunitPatchCall<'a, S> {
        AccountAdunitPatchCall {
            hub: self.hub,
            _request: request,
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
    /// Update the supplied ad unit in the specified publisher AdSense account.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - Account which contains the ad client.
    /// * `adClientId` - Ad client which contains the ad unit.
    pub fn adunits_update(&self, request: AdUnit, account_id: &str, ad_client_id: &str) -> AccountAdunitUpdateCall<'a, S> {
        AccountAdunitUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _ad_client_id: ad_client_id.to_string(),
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
    /// * `accountId` - Hosted account upon which to report.
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
    /// Get information about the selected associated AdSense account.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - Account to get information about.
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
    /// List hosted accounts associated with this AdSense account by ad client id.
    /// 
    /// # Arguments
    ///
    /// * `filterAdClientId` - Ad clients to list accounts for.
    pub fn list(&self, filter_ad_client_id: &Vec<String>) -> AccountListCall<'a, S> {
        AccountListCall {
            hub: self.hub,
            _filter_ad_client_id: filter_ad_client_id.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *adclient* resources.
/// It is not used directly, but through the [`AdSenseHost`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adsensehost4d1 as adsensehost4d1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adsensehost4d1::{AdSenseHost, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdSenseHost::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.adclients();
/// # }
/// ```
pub struct AdclientMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdSenseHost<S>,
}

impl<'a, S> client::MethodsBuilder for AdclientMethods<'a, S> {}

impl<'a, S> AdclientMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get information about one of the ad clients in the Host AdSense account.
    /// 
    /// # Arguments
    ///
    /// * `adClientId` - Ad client to get.
    pub fn get(&self, ad_client_id: &str) -> AdclientGetCall<'a, S> {
        AdclientGetCall {
            hub: self.hub,
            _ad_client_id: ad_client_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all host ad clients in this AdSense account.
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



/// A builder providing access to all methods supported on *associationsession* resources.
/// It is not used directly, but through the [`AdSenseHost`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adsensehost4d1 as adsensehost4d1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adsensehost4d1::{AdSenseHost, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdSenseHost::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `start(...)` and `verify(...)`
/// // to build up your call.
/// let rb = hub.associationsessions();
/// # }
/// ```
pub struct AssociationsessionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdSenseHost<S>,
}

impl<'a, S> client::MethodsBuilder for AssociationsessionMethods<'a, S> {}

impl<'a, S> AssociationsessionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create an association session for initiating an association with an AdSense user.
    /// 
    /// # Arguments
    ///
    /// * `productCode` - Products to associate with the user.
    /// * `websiteUrl` - The URL of the user's hosted website.
    pub fn start(&self, product_code: &Vec<String>, website_url: &str) -> AssociationsessionStartCall<'a, S> {
        AssociationsessionStartCall {
            hub: self.hub,
            _product_code: product_code.clone(),
            _website_url: website_url.to_string(),
            _website_locale: Default::default(),
            _user_locale: Default::default(),
            _callback_url: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Verify an association session after the association callback returns from AdSense signup.
    /// 
    /// # Arguments
    ///
    /// * `token` - The token returned to the association callback URL.
    pub fn verify(&self, token: &str) -> AssociationsessionVerifyCall<'a, S> {
        AssociationsessionVerifyCall {
            hub: self.hub,
            _token: token.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *customchannel* resources.
/// It is not used directly, but through the [`AdSenseHost`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adsensehost4d1 as adsensehost4d1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adsensehost4d1::{AdSenseHost, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdSenseHost::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.customchannels();
/// # }
/// ```
pub struct CustomchannelMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdSenseHost<S>,
}

impl<'a, S> client::MethodsBuilder for CustomchannelMethods<'a, S> {}

impl<'a, S> CustomchannelMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete a specific custom channel from the host AdSense account.
    /// 
    /// # Arguments
    ///
    /// * `adClientId` - Ad client from which to delete the custom channel.
    /// * `customChannelId` - Custom channel to delete.
    pub fn delete(&self, ad_client_id: &str, custom_channel_id: &str) -> CustomchannelDeleteCall<'a, S> {
        CustomchannelDeleteCall {
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
    /// Get a specific custom channel from the host AdSense account.
    /// 
    /// # Arguments
    ///
    /// * `adClientId` - Ad client from which to get the custom channel.
    /// * `customChannelId` - Custom channel to get.
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
    /// Add a new custom channel to the host AdSense account.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `adClientId` - Ad client to which the new custom channel will be added.
    pub fn insert(&self, request: CustomChannel, ad_client_id: &str) -> CustomchannelInsertCall<'a, S> {
        CustomchannelInsertCall {
            hub: self.hub,
            _request: request,
            _ad_client_id: ad_client_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all host custom channels in this AdSense account.
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
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update a custom channel in the host AdSense account. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `adClientId` - Ad client in which the custom channel will be updated.
    /// * `customChannelId` - Custom channel to get.
    pub fn patch(&self, request: CustomChannel, ad_client_id: &str, custom_channel_id: &str) -> CustomchannelPatchCall<'a, S> {
        CustomchannelPatchCall {
            hub: self.hub,
            _request: request,
            _ad_client_id: ad_client_id.to_string(),
            _custom_channel_id: custom_channel_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update a custom channel in the host AdSense account.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `adClientId` - Ad client in which the custom channel will be updated.
    pub fn update(&self, request: CustomChannel, ad_client_id: &str) -> CustomchannelUpdateCall<'a, S> {
        CustomchannelUpdateCall {
            hub: self.hub,
            _request: request,
            _ad_client_id: ad_client_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *report* resources.
/// It is not used directly, but through the [`AdSenseHost`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adsensehost4d1 as adsensehost4d1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adsensehost4d1::{AdSenseHost, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdSenseHost::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `generate(...)`
/// // to build up your call.
/// let rb = hub.reports();
/// # }
/// ```
pub struct ReportMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdSenseHost<S>,
}

impl<'a, S> client::MethodsBuilder for ReportMethods<'a, S> {}

impl<'a, S> ReportMethods<'a, S> {
    
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
}



/// A builder providing access to all methods supported on *urlchannel* resources.
/// It is not used directly, but through the [`AdSenseHost`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adsensehost4d1 as adsensehost4d1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adsensehost4d1::{AdSenseHost, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdSenseHost::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.urlchannels();
/// # }
/// ```
pub struct UrlchannelMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdSenseHost<S>,
}

impl<'a, S> client::MethodsBuilder for UrlchannelMethods<'a, S> {}

impl<'a, S> UrlchannelMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete a URL channel from the host AdSense account.
    /// 
    /// # Arguments
    ///
    /// * `adClientId` - Ad client from which to delete the URL channel.
    /// * `urlChannelId` - URL channel to delete.
    pub fn delete(&self, ad_client_id: &str, url_channel_id: &str) -> UrlchannelDeleteCall<'a, S> {
        UrlchannelDeleteCall {
            hub: self.hub,
            _ad_client_id: ad_client_id.to_string(),
            _url_channel_id: url_channel_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Add a new URL channel to the host AdSense account.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `adClientId` - Ad client to which the new URL channel will be added.
    pub fn insert(&self, request: UrlChannel, ad_client_id: &str) -> UrlchannelInsertCall<'a, S> {
        UrlchannelInsertCall {
            hub: self.hub,
            _request: request,
            _ad_client_id: ad_client_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all host URL channels in the host AdSense account.
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



