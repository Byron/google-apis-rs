use super::*;
/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the [`Adsense`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adsense2 as adsense2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use adsense2::{Adsense, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Adsense::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `adclients_adunits_create(...)`, `adclients_adunits_get(...)`, `adclients_adunits_get_adcode(...)`, `adclients_adunits_list(...)`, `adclients_adunits_list_linked_custom_channels(...)`, `adclients_adunits_patch(...)`, `adclients_customchannels_create(...)`, `adclients_customchannels_delete(...)`, `adclients_customchannels_get(...)`, `adclients_customchannels_list(...)`, `adclients_customchannels_list_linked_ad_units(...)`, `adclients_customchannels_patch(...)`, `adclients_get(...)`, `adclients_get_adcode(...)`, `adclients_list(...)`, `adclients_urlchannels_get(...)`, `adclients_urlchannels_list(...)`, `alerts_list(...)`, `get(...)`, `get_ad_blocking_recovery_tag(...)`, `list(...)`, `list_child_accounts(...)`, `payments_list(...)`, `reports_generate(...)`, `reports_generate_csv(...)`, `reports_get_saved(...)`, `reports_saved_generate(...)`, `reports_saved_generate_csv(...)`, `reports_saved_list(...)`, `sites_get(...)` and `sites_list(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Adsense<S>,
}

impl<'a, S> client::MethodsBuilder for AccountMethods<'a, S> {}

impl<'a, S> AccountMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an ad unit. This method can only be used by projects enabled for the [AdSense for Platforms](https://developers.google.com/adsense/platforms/) product. Note that ad units can only be created for ad clients with an “AFC” product code. For more info see the [AdClient resource](https://developers.google.com/adsense/management/reference/rest/v2/accounts.adclients). For now, this method can only be used to create `DISPLAY` ad units. See: https://support.google.com/adsense/answer/9183566
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Ad client to create an ad unit under. Format: accounts/{account}/adclients/{adclient}
    pub fn adclients_adunits_create(&self, request: AdUnit, parent: &str) -> AccountAdclientAdunitCreateCall<'a, S> {
        AccountAdclientAdunitCreateCall {
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
    /// Gets an ad unit from a specified account and ad client.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. AdUnit to get information about. Format: accounts/{account}/adclients/{adclient}/adunits/{adunit}
    pub fn adclients_adunits_get(&self, name: &str) -> AccountAdclientAdunitGetCall<'a, S> {
        AccountAdclientAdunitGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the ad unit code for a given ad unit. For more information, see [About the AdSense code](https://support.google.com/adsense/answer/9274634) and [Where to place the ad code in your HTML](https://support.google.com/adsense/answer/9190028).
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the adunit for which to get the adcode. Format: accounts/{account}/adclients/{adclient}/adunits/{adunit}
    pub fn adclients_adunits_get_adcode(&self, name: &str) -> AccountAdclientAdunitGetAdcodeCall<'a, S> {
        AccountAdclientAdunitGetAdcodeCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all ad units under a specified account and ad client.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The ad client which owns the collection of ad units. Format: accounts/{account}/adclients/{adclient}
    pub fn adclients_adunits_list(&self, parent: &str) -> AccountAdclientAdunitListCall<'a, S> {
        AccountAdclientAdunitListCall {
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
    /// Lists all the custom channels available for an ad unit.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The ad unit which owns the collection of custom channels. Format: accounts/{account}/adclients/{adclient}/adunits/{adunit}
    pub fn adclients_adunits_list_linked_custom_channels(&self, parent: &str) -> AccountAdclientAdunitListLinkedCustomChannelCall<'a, S> {
        AccountAdclientAdunitListLinkedCustomChannelCall {
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
    /// Updates an ad unit. This method can only be used by projects enabled for the [AdSense for Platforms](https://developers.google.com/adsense/platforms/) product. For now, this method can only be used to update `DISPLAY` ad units. See: https://support.google.com/adsense/answer/9183566
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name of the ad unit. Format: accounts/{account}/adclients/{adclient}/adunits/{adunit}
    pub fn adclients_adunits_patch(&self, request: AdUnit, name: &str) -> AccountAdclientAdunitPatchCall<'a, S> {
        AccountAdclientAdunitPatchCall {
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
    /// Creates a custom channel. This method can only be used by projects enabled for the [AdSense for Platforms](https://developers.google.com/adsense/platforms/) product.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The ad client to create a custom channel under. Format: accounts/{account}/adclients/{adclient}
    pub fn adclients_customchannels_create(&self, request: CustomChannel, parent: &str) -> AccountAdclientCustomchannelCreateCall<'a, S> {
        AccountAdclientCustomchannelCreateCall {
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
    /// Deletes a custom channel. This method can only be used by projects enabled for the [AdSense for Platforms](https://developers.google.com/adsense/platforms/) product.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the custom channel to delete. Format: accounts/{account}/adclients/{adclient}/customchannels/{customchannel}
    pub fn adclients_customchannels_delete(&self, name: &str) -> AccountAdclientCustomchannelDeleteCall<'a, S> {
        AccountAdclientCustomchannelDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about the selected custom channel.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the custom channel. Format: accounts/{account}/adclients/{adclient}/customchannels/{customchannel}
    pub fn adclients_customchannels_get(&self, name: &str) -> AccountAdclientCustomchannelGetCall<'a, S> {
        AccountAdclientCustomchannelGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the custom channels available in an ad client.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The ad client which owns the collection of custom channels. Format: accounts/{account}/adclients/{adclient}
    pub fn adclients_customchannels_list(&self, parent: &str) -> AccountAdclientCustomchannelListCall<'a, S> {
        AccountAdclientCustomchannelListCall {
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
    /// Lists all the ad units available for a custom channel.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The custom channel which owns the collection of ad units. Format: accounts/{account}/adclients/{adclient}/customchannels/{customchannel}
    pub fn adclients_customchannels_list_linked_ad_units(&self, parent: &str) -> AccountAdclientCustomchannelListLinkedAdUnitCall<'a, S> {
        AccountAdclientCustomchannelListLinkedAdUnitCall {
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
    /// Updates a custom channel. This method can only be used by projects enabled for the [AdSense for Platforms](https://developers.google.com/adsense/platforms/) product.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name of the custom channel. Format: accounts/{account}/adclients/{adclient}/customchannels/{customchannel}
    pub fn adclients_customchannels_patch(&self, request: CustomChannel, name: &str) -> AccountAdclientCustomchannelPatchCall<'a, S> {
        AccountAdclientCustomchannelPatchCall {
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
    /// Gets information about the selected url channel.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the url channel to retrieve. Format: accounts/{account}/adclients/{adclient}/urlchannels/{urlchannel}
    pub fn adclients_urlchannels_get(&self, name: &str) -> AccountAdclientUrlchannelGetCall<'a, S> {
        AccountAdclientUrlchannelGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists active url channels.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The ad client which owns the collection of url channels. Format: accounts/{account}/adclients/{adclient}
    pub fn adclients_urlchannels_list(&self, parent: &str) -> AccountAdclientUrlchannelListCall<'a, S> {
        AccountAdclientUrlchannelListCall {
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
    /// Gets the ad client from the given resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the ad client to retrieve. Format: accounts/{account}/adclients/{adclient}
    pub fn adclients_get(&self, name: &str) -> AccountAdclientGetCall<'a, S> {
        AccountAdclientGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the AdSense code for a given ad client. This returns what was previously known as the 'auto ad code'. This is only supported for ad clients with a product_code of AFC. For more information, see [About the AdSense code](https://support.google.com/adsense/answer/9274634).
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the ad client for which to get the adcode. Format: accounts/{account}/adclients/{adclient}
    pub fn adclients_get_adcode(&self, name: &str) -> AccountAdclientGetAdcodeCall<'a, S> {
        AccountAdclientGetAdcodeCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the ad clients available in an account.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The account which owns the collection of ad clients. Format: accounts/{account}
    pub fn adclients_list(&self, parent: &str) -> AccountAdclientListCall<'a, S> {
        AccountAdclientListCall {
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
    /// Lists all the alerts available in an account.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The account which owns the collection of alerts. Format: accounts/{account}
    pub fn alerts_list(&self, parent: &str) -> AccountAlertListCall<'a, S> {
        AccountAlertListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _language_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the payments available for an account.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The account which owns the collection of payments. Format: accounts/{account}
    pub fn payments_list(&self, parent: &str) -> AccountPaymentListCall<'a, S> {
        AccountPaymentListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generates a saved report.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the saved report. Format: accounts/{account}/reports/{report}
    pub fn reports_saved_generate(&self, name: &str) -> AccountReportSavedGenerateCall<'a, S> {
        AccountReportSavedGenerateCall {
            hub: self.hub,
            _name: name.to_string(),
            _start_date_year: Default::default(),
            _start_date_month: Default::default(),
            _start_date_day: Default::default(),
            _reporting_time_zone: Default::default(),
            _language_code: Default::default(),
            _end_date_year: Default::default(),
            _end_date_month: Default::default(),
            _end_date_day: Default::default(),
            _date_range: Default::default(),
            _currency_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generates a csv formatted saved report.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the saved report. Format: accounts/{account}/reports/{report}
    pub fn reports_saved_generate_csv(&self, name: &str) -> AccountReportSavedGenerateCsvCall<'a, S> {
        AccountReportSavedGenerateCsvCall {
            hub: self.hub,
            _name: name.to_string(),
            _start_date_year: Default::default(),
            _start_date_month: Default::default(),
            _start_date_day: Default::default(),
            _reporting_time_zone: Default::default(),
            _language_code: Default::default(),
            _end_date_year: Default::default(),
            _end_date_month: Default::default(),
            _end_date_day: Default::default(),
            _date_range: Default::default(),
            _currency_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists saved reports.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The account which owns the collection of reports. Format: accounts/{account}
    pub fn reports_saved_list(&self, parent: &str) -> AccountReportSavedListCall<'a, S> {
        AccountReportSavedListCall {
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
    /// Generates an ad hoc report.
    /// 
    /// # Arguments
    ///
    /// * `account` - Required. The account which owns the collection of reports. Format: accounts/{account}
    pub fn reports_generate(&self, account: &str) -> AccountReportGenerateCall<'a, S> {
        AccountReportGenerateCall {
            hub: self.hub,
            _account: account.to_string(),
            _start_date_year: Default::default(),
            _start_date_month: Default::default(),
            _start_date_day: Default::default(),
            _reporting_time_zone: Default::default(),
            _order_by: Default::default(),
            _metrics: Default::default(),
            _limit: Default::default(),
            _language_code: Default::default(),
            _filters: Default::default(),
            _end_date_year: Default::default(),
            _end_date_month: Default::default(),
            _end_date_day: Default::default(),
            _dimensions: Default::default(),
            _date_range: Default::default(),
            _currency_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generates a csv formatted ad hoc report.
    /// 
    /// # Arguments
    ///
    /// * `account` - Required. The account which owns the collection of reports. Format: accounts/{account}
    pub fn reports_generate_csv(&self, account: &str) -> AccountReportGenerateCsvCall<'a, S> {
        AccountReportGenerateCsvCall {
            hub: self.hub,
            _account: account.to_string(),
            _start_date_year: Default::default(),
            _start_date_month: Default::default(),
            _start_date_day: Default::default(),
            _reporting_time_zone: Default::default(),
            _order_by: Default::default(),
            _metrics: Default::default(),
            _limit: Default::default(),
            _language_code: Default::default(),
            _filters: Default::default(),
            _end_date_year: Default::default(),
            _end_date_month: Default::default(),
            _end_date_day: Default::default(),
            _dimensions: Default::default(),
            _date_range: Default::default(),
            _currency_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the saved report from the given resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the saved report to retrieve. Format: accounts/{account}/reports/{report}
    pub fn reports_get_saved(&self, name: &str) -> AccountReportGetSavedCall<'a, S> {
        AccountReportGetSavedCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about the selected site.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the site. Format: accounts/{account}/sites/{site}
    pub fn sites_get(&self, name: &str) -> AccountSiteGetCall<'a, S> {
        AccountSiteGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the sites available in an account.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The account which owns the collection of sites. Format: accounts/{account}
    pub fn sites_list(&self, parent: &str) -> AccountSiteListCall<'a, S> {
        AccountSiteListCall {
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
    /// Gets information about the selected AdSense account.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Account to get information about. Format: accounts/{account}
    pub fn get(&self, name: &str) -> AccountGetCall<'a, S> {
        AccountGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the ad blocking recovery tag of an account.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the account to get the tag for. Format: accounts/{account}
    pub fn get_ad_blocking_recovery_tag(&self, name: &str) -> AccountGetAdBlockingRecoveryTagCall<'a, S> {
        AccountGetAdBlockingRecoveryTagCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all accounts available to this user.
    pub fn list(&self) -> AccountListCall<'a, S> {
        AccountListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all accounts directly managed by the given AdSense account.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent account, which owns the child accounts. Format: accounts/{account}
    pub fn list_child_accounts(&self, parent: &str) -> AccountListChildAccountCall<'a, S> {
        AccountListChildAccountCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



