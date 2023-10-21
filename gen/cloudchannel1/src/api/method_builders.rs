use super::*;
/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the [`Cloudchannel`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudchannel1 as cloudchannel1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudchannel1::{Cloudchannel, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `channel_partner_links_channel_partner_repricing_configs_create(...)`, `channel_partner_links_channel_partner_repricing_configs_delete(...)`, `channel_partner_links_channel_partner_repricing_configs_get(...)`, `channel_partner_links_channel_partner_repricing_configs_list(...)`, `channel_partner_links_channel_partner_repricing_configs_patch(...)`, `channel_partner_links_create(...)`, `channel_partner_links_customers_create(...)`, `channel_partner_links_customers_delete(...)`, `channel_partner_links_customers_get(...)`, `channel_partner_links_customers_import(...)`, `channel_partner_links_customers_list(...)`, `channel_partner_links_customers_patch(...)`, `channel_partner_links_get(...)`, `channel_partner_links_list(...)`, `channel_partner_links_patch(...)`, `check_cloud_identity_accounts_exist(...)`, `customers_create(...)`, `customers_customer_repricing_configs_create(...)`, `customers_customer_repricing_configs_delete(...)`, `customers_customer_repricing_configs_get(...)`, `customers_customer_repricing_configs_list(...)`, `customers_customer_repricing_configs_patch(...)`, `customers_delete(...)`, `customers_entitlements_activate(...)`, `customers_entitlements_cancel(...)`, `customers_entitlements_change_offer(...)`, `customers_entitlements_change_parameters(...)`, `customers_entitlements_change_renewal_settings(...)`, `customers_entitlements_create(...)`, `customers_entitlements_get(...)`, `customers_entitlements_list(...)`, `customers_entitlements_lookup_offer(...)`, `customers_entitlements_start_paid_service(...)`, `customers_entitlements_suspend(...)`, `customers_get(...)`, `customers_import(...)`, `customers_list(...)`, `customers_list_purchasable_offers(...)`, `customers_list_purchasable_skus(...)`, `customers_patch(...)`, `customers_provision_cloud_identity(...)`, `customers_transfer_entitlements(...)`, `customers_transfer_entitlements_to_google(...)`, `list_subscribers(...)`, `list_transferable_offers(...)`, `list_transferable_skus(...)`, `offers_list(...)`, `register(...)`, `report_jobs_fetch_report_results(...)`, `reports_list(...)`, `reports_run(...)` and `unregister(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Cloudchannel<S>,
}

impl<'a, S> client::MethodsBuilder for AccountMethods<'a, S> {}

impl<'a, S> AccountMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a ChannelPartnerRepricingConfig. Call this method to set modifications for a specific ChannelPartner's bill. You can only create configs if the RepricingConfig.effective_invoice_month is a future month. If needed, you can create a config for the current month, with some restrictions. When creating a config for a future month, make sure there are no existing configs for that RepricingConfig.effective_invoice_month. The following restrictions are for creating configs in the current month. * This functionality is reserved for recovering from an erroneous config, and should not be used for regular business cases. * The new config will not modify exports used with other configs. Changes to the config may be immediate, but may take up to 24 hours. * There is a limit of ten configs for any ChannelPartner or RepricingConfig.effective_invoice_month. * The contained ChannelPartnerRepricingConfig.repricing_config vaule must be different from the value used in the current config for a ChannelPartner. Possible Error Codes: * PERMISSION_DENIED: If the account making the request and the account being queried are different. * INVALID_ARGUMENT: Missing or invalid required parameters in the request. Also displays if the updated config is for the current month or past months. * NOT_FOUND: The ChannelPartnerRepricingConfig specified does not exist or is not associated with the given account. * INTERNAL: Any non-user error related to technical issues in the backend. In this case, contact Cloud Channel support. Return Value: If successful, the updated ChannelPartnerRepricingConfig resource, otherwise returns an error.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the ChannelPartner that will receive the repricing config. Parent uses the format: accounts/{account_id}/channelPartnerLinks/{channel_partner_id}
    pub fn channel_partner_links_channel_partner_repricing_configs_create(&self, request: GoogleCloudChannelV1ChannelPartnerRepricingConfig, parent: &str) -> AccountChannelPartnerLinkChannelPartnerRepricingConfigCreateCall<'a, S> {
        AccountChannelPartnerLinkChannelPartnerRepricingConfigCreateCall {
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
    /// Deletes the given ChannelPartnerRepricingConfig permanently. You can only delete configs if their RepricingConfig.effective_invoice_month is set to a date after the current month. Possible error codes: * PERMISSION_DENIED: The account making the request does not own this customer. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * FAILED_PRECONDITION: The ChannelPartnerRepricingConfig is active or in the past. * NOT_FOUND: No ChannelPartnerRepricingConfig found for the name in the request.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the channel partner repricing config rule to delete.
    pub fn channel_partner_links_channel_partner_repricing_configs_delete(&self, name: &str) -> AccountChannelPartnerLinkChannelPartnerRepricingConfigDeleteCall<'a, S> {
        AccountChannelPartnerLinkChannelPartnerRepricingConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about how a Distributor modifies their bill before sending it to a ChannelPartner. Possible Error Codes: * PERMISSION_DENIED: If the account making the request and the account being queried are different. * NOT_FOUND: The ChannelPartnerRepricingConfig was not found. * INTERNAL: Any non-user error related to technical issues in the backend. In this case, contact Cloud Channel support. Return Value: If successful, the ChannelPartnerRepricingConfig resource, otherwise returns an error.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the ChannelPartnerRepricingConfig Format: accounts/{account_id}/channelPartnerLinks/{channel_partner_id}/channelPartnerRepricingConfigs/{id}.
    pub fn channel_partner_links_channel_partner_repricing_configs_get(&self, name: &str) -> AccountChannelPartnerLinkChannelPartnerRepricingConfigGetCall<'a, S> {
        AccountChannelPartnerLinkChannelPartnerRepricingConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists information about how a Reseller modifies their bill before sending it to a ChannelPartner. Possible Error Codes: * PERMISSION_DENIED: If the account making the request and the account being queried are different. * NOT_FOUND: The ChannelPartnerRepricingConfig specified does not exist or is not associated with the given account. * INTERNAL: Any non-user error related to technical issues in the backend. In this case, contact Cloud Channel support. Return Value: If successful, the ChannelPartnerRepricingConfig resources. The data for each resource is displayed in the ascending order of: * channel partner ID * RepricingConfig.effective_invoice_month * ChannelPartnerRepricingConfig.update_time If unsuccessful, returns an error.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the account's ChannelPartnerLink. Parent uses the format: accounts/{account_id}/channelPartnerLinks/{channel_partner_id}. Supports accounts/{account_id}/channelPartnerLinks/- to retrieve configs for all channel partners.
    pub fn channel_partner_links_channel_partner_repricing_configs_list(&self, parent: &str) -> AccountChannelPartnerLinkChannelPartnerRepricingConfigListCall<'a, S> {
        AccountChannelPartnerLinkChannelPartnerRepricingConfigListCall {
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
    /// Updates a ChannelPartnerRepricingConfig. Call this method to set modifications for a specific ChannelPartner's bill. This method overwrites the existing CustomerRepricingConfig. You can only update configs if the RepricingConfig.effective_invoice_month is a future month. To make changes to configs for the current month, use CreateChannelPartnerRepricingConfig, taking note of its restrictions. You cannot update the RepricingConfig.effective_invoice_month. When updating a config in the future: * This config must already exist. Possible Error Codes: * PERMISSION_DENIED: If the account making the request and the account being queried are different. * INVALID_ARGUMENT: Missing or invalid required parameters in the request. Also displays if the updated config is for the current month or past months. * NOT_FOUND: The ChannelPartnerRepricingConfig specified does not exist or is not associated with the given account. * INTERNAL: Any non-user error related to technical issues in the backend. In this case, contact Cloud Channel support. Return Value: If successful, the updated ChannelPartnerRepricingConfig resource, otherwise returns an error.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name of the ChannelPartnerRepricingConfig. Format: accounts/{account_id}/channelPartnerLinks/{channel_partner_id}/channelPartnerRepricingConfigs/{id}.
    pub fn channel_partner_links_channel_partner_repricing_configs_patch(&self, request: GoogleCloudChannelV1ChannelPartnerRepricingConfig, name: &str) -> AccountChannelPartnerLinkChannelPartnerRepricingConfigPatchCall<'a, S> {
        AccountChannelPartnerLinkChannelPartnerRepricingConfigPatchCall {
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
    /// Creates a new Customer resource under the reseller or distributor account. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: * Required request parameters are missing or invalid. * Domain field value doesn't match the primary email domain. Return value: The newly created Customer resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of reseller account in which to create the customer. Parent uses the format: accounts/{account_id}
    pub fn channel_partner_links_customers_create(&self, request: GoogleCloudChannelV1Customer, parent: &str) -> AccountChannelPartnerLinkCustomerCreateCall<'a, S> {
        AccountChannelPartnerLinkCustomerCreateCall {
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
    /// Deletes the given Customer permanently. Possible error codes: * PERMISSION_DENIED: The account making the request does not own this customer. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * FAILED_PRECONDITION: The customer has existing entitlements. * NOT_FOUND: No Customer resource found for the name in the request.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the customer to delete.
    pub fn channel_partner_links_customers_delete(&self, name: &str) -> AccountChannelPartnerLinkCustomerDeleteCall<'a, S> {
        AccountChannelPartnerLinkCustomerDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the requested Customer resource. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: The customer resource doesn't exist. Usually the result of an invalid name parameter. Return value: The Customer resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the customer to retrieve. Name uses the format: accounts/{account_id}/customers/{customer_id}
    pub fn channel_partner_links_customers_get(&self, name: &str) -> AccountChannelPartnerLinkCustomerGetCall<'a, S> {
        AccountChannelPartnerLinkCustomerGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Imports a Customer from the Cloud Identity associated with the provided Cloud Identity ID or domain before a TransferEntitlements call. If a linked Customer already exists and overwrite_if_exists is true, it will update that Customer's data. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * NOT_FOUND: Cloud Identity doesn't exist or was deleted. * INVALID_ARGUMENT: Required parameters are missing, or the auth_token is expired or invalid. * ALREADY_EXISTS: A customer already exists and has conflicting critical fields. Requires an overwrite. Return value: The Customer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the reseller's account. Parent takes the format: accounts/{account_id} or accounts/{account_id}/channelPartnerLinks/{channel_partner_id}
    pub fn channel_partner_links_customers_import(&self, request: GoogleCloudChannelV1ImportCustomerRequest, parent: &str) -> AccountChannelPartnerLinkCustomerImportCall<'a, S> {
        AccountChannelPartnerLinkCustomerImportCall {
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
    /// List Customers. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. Return value: List of Customers, or an empty list if there are no customers.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the reseller account to list customers from. Parent uses the format: accounts/{account_id}.
    pub fn channel_partner_links_customers_list(&self, parent: &str) -> AccountChannelPartnerLinkCustomerListCall<'a, S> {
        AccountChannelPartnerLinkCustomerListCall {
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
    /// Updates an existing Customer resource for the reseller or distributor. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: No Customer resource found for the name in the request. Return value: The updated Customer resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name of the customer. Format: accounts/{account_id}/customers/{customer_id}
    pub fn channel_partner_links_customers_patch(&self, request: GoogleCloudChannelV1Customer, name: &str) -> AccountChannelPartnerLinkCustomerPatchCall<'a, S> {
        AccountChannelPartnerLinkCustomerPatchCall {
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
    /// Initiates a channel partner link between a distributor and a reseller, or between resellers in an n-tier reseller channel. Invited partners need to follow the invite_link_uri provided in the response to accept. After accepting the invitation, a link is set up between the two parties. You must be a distributor to call this method. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * ALREADY_EXISTS: The ChannelPartnerLink sent in the request already exists. * NOT_FOUND: No Cloud Identity customer exists for provided domain. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The new ChannelPartnerLink resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Create a channel partner link for the provided reseller account's resource name. Parent uses the format: accounts/{account_id}
    pub fn channel_partner_links_create(&self, request: GoogleCloudChannelV1ChannelPartnerLink, parent: &str) -> AccountChannelPartnerLinkCreateCall<'a, S> {
        AccountChannelPartnerLinkCreateCall {
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
    /// Returns the requested ChannelPartnerLink resource. You must be a distributor to call this method. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: ChannelPartnerLink resource not found because of an invalid channel partner link name. Return value: The ChannelPartnerLink resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the channel partner link to retrieve. Name uses the format: accounts/{account_id}/channelPartnerLinks/{id} where {id} is the Cloud Identity ID of the partner.
    pub fn channel_partner_links_get(&self, name: &str) -> AccountChannelPartnerLinkGetCall<'a, S> {
        AccountChannelPartnerLinkGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List ChannelPartnerLinks belonging to a distributor. You must be a distributor to call this method. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. Return value: The list of the distributor account's ChannelPartnerLink resources.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the reseller account for listing channel partner links. Parent uses the format: accounts/{account_id}
    pub fn channel_partner_links_list(&self, parent: &str) -> AccountChannelPartnerLinkListCall<'a, S> {
        AccountChannelPartnerLinkListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _view: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a channel partner link. Distributors call this method to change a link's status. For example, to suspend a partner link. You must be a distributor to call this method. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: * Required request parameters are missing or invalid. * Link state cannot change from invited to active or suspended. * Cannot send reseller_cloud_identity_id, invite_url, or name in update mask. * NOT_FOUND: ChannelPartnerLink resource not found. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The updated ChannelPartnerLink resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the channel partner link to cancel. Name uses the format: accounts/{account_id}/channelPartnerLinks/{id} where {id} is the Cloud Identity ID of the partner.
    pub fn channel_partner_links_patch(&self, request: GoogleCloudChannelV1UpdateChannelPartnerLinkRequest, name: &str) -> AccountChannelPartnerLinkPatchCall<'a, S> {
        AccountChannelPartnerLinkPatchCall {
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
    /// Creates a CustomerRepricingConfig. Call this method to set modifications for a specific customer's bill. You can only create configs if the RepricingConfig.effective_invoice_month is a future month. If needed, you can create a config for the current month, with some restrictions. When creating a config for a future month, make sure there are no existing configs for that RepricingConfig.effective_invoice_month. The following restrictions are for creating configs in the current month. * This functionality is reserved for recovering from an erroneous config, and should not be used for regular business cases. * The new config will not modify exports used with other configs. Changes to the config may be immediate, but may take up to 24 hours. * There is a limit of ten configs for any RepricingConfig.EntitlementGranularity.entitlement or RepricingConfig.effective_invoice_month. * The contained CustomerRepricingConfig.repricing_config vaule must be different from the value used in the current config for a RepricingConfig.EntitlementGranularity.entitlement. Possible Error Codes: * PERMISSION_DENIED: If the account making the request and the account being queried are different. * INVALID_ARGUMENT: Missing or invalid required parameters in the request. Also displays if the updated config is for the current month or past months. * NOT_FOUND: The CustomerRepricingConfig specified does not exist or is not associated with the given account. * INTERNAL: Any non-user error related to technical issues in the backend. In this case, contact Cloud Channel support. Return Value: If successful, the updated CustomerRepricingConfig resource, otherwise returns an error.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the customer that will receive this repricing config. Parent uses the format: accounts/{account_id}/customers/{customer_id}
    pub fn customers_customer_repricing_configs_create(&self, request: GoogleCloudChannelV1CustomerRepricingConfig, parent: &str) -> AccountCustomerCustomerRepricingConfigCreateCall<'a, S> {
        AccountCustomerCustomerRepricingConfigCreateCall {
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
    /// Deletes the given CustomerRepricingConfig permanently. You can only delete configs if their RepricingConfig.effective_invoice_month is set to a date after the current month. Possible error codes: * PERMISSION_DENIED: The account making the request does not own this customer. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * FAILED_PRECONDITION: The CustomerRepricingConfig is active or in the past. * NOT_FOUND: No CustomerRepricingConfig found for the name in the request.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the customer repricing config rule to delete. Format: accounts/{account_id}/customers/{customer_id}/customerRepricingConfigs/{id}.
    pub fn customers_customer_repricing_configs_delete(&self, name: &str) -> AccountCustomerCustomerRepricingConfigDeleteCall<'a, S> {
        AccountCustomerCustomerRepricingConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about how a Reseller modifies their bill before sending it to a Customer. Possible Error Codes: * PERMISSION_DENIED: If the account making the request and the account being queried are different. * NOT_FOUND: The CustomerRepricingConfig was not found. * INTERNAL: Any non-user error related to technical issues in the backend. In this case, contact Cloud Channel support. Return Value: If successful, the CustomerRepricingConfig resource, otherwise returns an error.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the CustomerRepricingConfig. Format: accounts/{account_id}/customers/{customer_id}/customerRepricingConfigs/{id}.
    pub fn customers_customer_repricing_configs_get(&self, name: &str) -> AccountCustomerCustomerRepricingConfigGetCall<'a, S> {
        AccountCustomerCustomerRepricingConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists information about how a Reseller modifies their bill before sending it to a Customer. Possible Error Codes: * PERMISSION_DENIED: If the account making the request and the account being queried are different. * NOT_FOUND: The CustomerRepricingConfig specified does not exist or is not associated with the given account. * INTERNAL: Any non-user error related to technical issues in the backend. In this case, contact Cloud Channel support. Return Value: If successful, the CustomerRepricingConfig resources. The data for each resource is displayed in the ascending order of: * customer ID * RepricingConfig.EntitlementGranularity.entitlement * RepricingConfig.effective_invoice_month * CustomerRepricingConfig.update_time If unsuccessful, returns an error.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the customer. Parent uses the format: accounts/{account_id}/customers/{customer_id}. Supports accounts/{account_id}/customers/- to retrieve configs for all customers.
    pub fn customers_customer_repricing_configs_list(&self, parent: &str) -> AccountCustomerCustomerRepricingConfigListCall<'a, S> {
        AccountCustomerCustomerRepricingConfigListCall {
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
    /// Updates a CustomerRepricingConfig. Call this method to set modifications for a specific customer's bill. This method overwrites the existing CustomerRepricingConfig. You can only update configs if the RepricingConfig.effective_invoice_month is a future month. To make changes to configs for the current month, use CreateCustomerRepricingConfig, taking note of its restrictions. You cannot update the RepricingConfig.effective_invoice_month. When updating a config in the future: * This config must already exist. Possible Error Codes: * PERMISSION_DENIED: If the account making the request and the account being queried are different. * INVALID_ARGUMENT: Missing or invalid required parameters in the request. Also displays if the updated config is for the current month or past months. * NOT_FOUND: The CustomerRepricingConfig specified does not exist or is not associated with the given account. * INTERNAL: Any non-user error related to technical issues in the backend. In this case, contact Cloud Channel support. Return Value: If successful, the updated CustomerRepricingConfig resource, otherwise returns an error.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name of the CustomerRepricingConfig. Format: accounts/{account_id}/customers/{customer_id}/customerRepricingConfigs/{id}.
    pub fn customers_customer_repricing_configs_patch(&self, request: GoogleCloudChannelV1CustomerRepricingConfig, name: &str) -> AccountCustomerCustomerRepricingConfigPatchCall<'a, S> {
        AccountCustomerCustomerRepricingConfigPatchCall {
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
    /// Activates a previously suspended entitlement. Entitlements suspended for pending ToS acceptance can't be activated using this method. An entitlement activation is a long-running operation and it updates the state of the customer entitlement. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: Entitlement resource not found. * SUSPENSION_NOT_RESELLER_INITIATED: Can only activate reseller-initiated suspensions and entitlements that have accepted the TOS. * NOT_SUSPENDED: Can only activate suspended entitlements not in an ACTIVE state. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata will contain an instance of OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the entitlement to activate. Name uses the format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    pub fn customers_entitlements_activate(&self, request: GoogleCloudChannelV1ActivateEntitlementRequest, name: &str) -> AccountCustomerEntitlementActivateCall<'a, S> {
        AccountCustomerEntitlementActivateCall {
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
    /// Cancels a previously fulfilled entitlement. An entitlement cancellation is a long-running operation. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * FAILED_PRECONDITION: There are Google Cloud projects linked to the Google Cloud entitlement's Cloud Billing subaccount. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: Entitlement resource not found. * DELETION_TYPE_NOT_ALLOWED: Cancel is only allowed for Google Workspace add-ons, or entitlements for Google Cloud's development platform. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The response will contain google.protobuf.Empty on success. The Operation metadata will contain an instance of OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the entitlement to cancel. Name uses the format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    pub fn customers_entitlements_cancel(&self, request: GoogleCloudChannelV1CancelEntitlementRequest, name: &str) -> AccountCustomerEntitlementCancelCall<'a, S> {
        AccountCustomerEntitlementCancelCall {
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
    /// Updates the Offer for an existing customer entitlement. An entitlement update is a long-running operation and it updates the entitlement as a result of fulfillment. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: Offer or Entitlement resource not found. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata will contain an instance of OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the entitlement to update. Name uses the format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    pub fn customers_entitlements_change_offer(&self, request: GoogleCloudChannelV1ChangeOfferRequest, name: &str) -> AccountCustomerEntitlementChangeOfferCall<'a, S> {
        AccountCustomerEntitlementChangeOfferCall {
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
    /// Change parameters of the entitlement. An entitlement update is a long-running operation and it updates the entitlement as a result of fulfillment. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. For example, the number of seats being changed is greater than the allowed number of max seats, or decreasing seats for a commitment based plan. * NOT_FOUND: Entitlement resource not found. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata will contain an instance of OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the entitlement to update. Name uses the format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    pub fn customers_entitlements_change_parameters(&self, request: GoogleCloudChannelV1ChangeParametersRequest, name: &str) -> AccountCustomerEntitlementChangeParameterCall<'a, S> {
        AccountCustomerEntitlementChangeParameterCall {
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
    /// Updates the renewal settings for an existing customer entitlement. An entitlement update is a long-running operation and it updates the entitlement as a result of fulfillment. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: Entitlement resource not found. * NOT_COMMITMENT_PLAN: Renewal Settings are only applicable for a commitment plan. Can't enable or disable renewals for non-commitment plans. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata will contain an instance of OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the entitlement to update. Name uses the format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    pub fn customers_entitlements_change_renewal_settings(&self, request: GoogleCloudChannelV1ChangeRenewalSettingsRequest, name: &str) -> AccountCustomerEntitlementChangeRenewalSettingCall<'a, S> {
        AccountCustomerEntitlementChangeRenewalSettingCall {
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
    /// Creates an entitlement for a customer. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: * Required request parameters are missing or invalid. * There is already a customer entitlement for a SKU from the same product family. * INVALID_VALUE: Make sure the OfferId is valid. If it is, contact Google Channel support for further troubleshooting. * NOT_FOUND: The customer or offer resource was not found. * ALREADY_EXISTS: * The SKU was already purchased for the customer. * The customer's primary email already exists. Retry after changing the customer's primary contact email. * CONDITION_NOT_MET or FAILED_PRECONDITION: * The domain required for purchasing a SKU has not been verified. * A pre-requisite SKU required to purchase an Add-On SKU is missing. For example, Google Workspace Business Starter is required to purchase Vault or Drive. * (Developer accounts only) Reseller and resold domain must meet the following naming requirements: * Domain names must start with goog-test. * Domain names must include the reseller domain. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata will contain an instance of OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the reseller's customer account in which to create the entitlement. Parent uses the format: accounts/{account_id}/customers/{customer_id}
    pub fn customers_entitlements_create(&self, request: GoogleCloudChannelV1CreateEntitlementRequest, parent: &str) -> AccountCustomerEntitlementCreateCall<'a, S> {
        AccountCustomerEntitlementCreateCall {
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
    /// Returns the requested Entitlement resource. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: The customer entitlement was not found. Return value: The requested Entitlement resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the entitlement to retrieve. Name uses the format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    pub fn customers_entitlements_get(&self, name: &str) -> AccountCustomerEntitlementGetCall<'a, S> {
        AccountCustomerEntitlementGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists Entitlements belonging to a customer. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. Return value: A list of the customer's Entitlements.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the reseller's customer account to list entitlements for. Parent uses the format: accounts/{account_id}/customers/{customer_id}
    pub fn customers_entitlements_list(&self, parent: &str) -> AccountCustomerEntitlementListCall<'a, S> {
        AccountCustomerEntitlementListCall {
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
    /// Returns the requested Offer resource. Possible error codes: * PERMISSION_DENIED: The entitlement doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: Entitlement or offer was not found. Return value: The Offer resource.
    /// 
    /// # Arguments
    ///
    /// * `entitlement` - Required. The resource name of the entitlement to retrieve the Offer. Entitlement uses the format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    pub fn customers_entitlements_lookup_offer(&self, entitlement: &str) -> AccountCustomerEntitlementLookupOfferCall<'a, S> {
        AccountCustomerEntitlementLookupOfferCall {
            hub: self.hub,
            _entitlement: entitlement.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts paid service for a trial entitlement. Starts paid service for a trial entitlement immediately. This method is only applicable if a plan is set up for a trial entitlement but has some trial days remaining. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: Entitlement resource not found. * FAILED_PRECONDITION/NOT_IN_TRIAL: This method only works for entitlement on trial plans. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata will contain an instance of OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the entitlement to start a paid service for. Name uses the format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    pub fn customers_entitlements_start_paid_service(&self, request: GoogleCloudChannelV1StartPaidServiceRequest, name: &str) -> AccountCustomerEntitlementStartPaidServiceCall<'a, S> {
        AccountCustomerEntitlementStartPaidServiceCall {
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
    /// Suspends a previously fulfilled entitlement. An entitlement suspension is a long-running operation. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: Entitlement resource not found. * NOT_ACTIVE: Entitlement is not active. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata will contain an instance of OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the entitlement to suspend. Name uses the format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    pub fn customers_entitlements_suspend(&self, request: GoogleCloudChannelV1SuspendEntitlementRequest, name: &str) -> AccountCustomerEntitlementSuspendCall<'a, S> {
        AccountCustomerEntitlementSuspendCall {
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
    /// Creates a new Customer resource under the reseller or distributor account. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: * Required request parameters are missing or invalid. * Domain field value doesn't match the primary email domain. Return value: The newly created Customer resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of reseller account in which to create the customer. Parent uses the format: accounts/{account_id}
    pub fn customers_create(&self, request: GoogleCloudChannelV1Customer, parent: &str) -> AccountCustomerCreateCall<'a, S> {
        AccountCustomerCreateCall {
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
    /// Deletes the given Customer permanently. Possible error codes: * PERMISSION_DENIED: The account making the request does not own this customer. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * FAILED_PRECONDITION: The customer has existing entitlements. * NOT_FOUND: No Customer resource found for the name in the request.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the customer to delete.
    pub fn customers_delete(&self, name: &str) -> AccountCustomerDeleteCall<'a, S> {
        AccountCustomerDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the requested Customer resource. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: The customer resource doesn't exist. Usually the result of an invalid name parameter. Return value: The Customer resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the customer to retrieve. Name uses the format: accounts/{account_id}/customers/{customer_id}
    pub fn customers_get(&self, name: &str) -> AccountCustomerGetCall<'a, S> {
        AccountCustomerGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Imports a Customer from the Cloud Identity associated with the provided Cloud Identity ID or domain before a TransferEntitlements call. If a linked Customer already exists and overwrite_if_exists is true, it will update that Customer's data. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * NOT_FOUND: Cloud Identity doesn't exist or was deleted. * INVALID_ARGUMENT: Required parameters are missing, or the auth_token is expired or invalid. * ALREADY_EXISTS: A customer already exists and has conflicting critical fields. Requires an overwrite. Return value: The Customer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the reseller's account. Parent takes the format: accounts/{account_id} or accounts/{account_id}/channelPartnerLinks/{channel_partner_id}
    pub fn customers_import(&self, request: GoogleCloudChannelV1ImportCustomerRequest, parent: &str) -> AccountCustomerImportCall<'a, S> {
        AccountCustomerImportCall {
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
    /// List Customers. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. Return value: List of Customers, or an empty list if there are no customers.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the reseller account to list customers from. Parent uses the format: accounts/{account_id}.
    pub fn customers_list(&self, parent: &str) -> AccountCustomerListCall<'a, S> {
        AccountCustomerListCall {
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
    /// Lists the following: * Offers that you can purchase for a customer. * Offers that you can change for an entitlement. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller * INVALID_ARGUMENT: Required request parameters are missing or invalid.
    /// 
    /// # Arguments
    ///
    /// * `customer` - Required. The resource name of the customer to list Offers for. Format: accounts/{account_id}/customers/{customer_id}.
    pub fn customers_list_purchasable_offers(&self, customer: &str) -> AccountCustomerListPurchasableOfferCall<'a, S> {
        AccountCustomerListPurchasableOfferCall {
            hub: self.hub,
            _customer: customer.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _language_code: Default::default(),
            _create_entitlement_purchase_sku: Default::default(),
            _change_offer_purchase_new_sku: Default::default(),
            _change_offer_purchase_entitlement: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the following: * SKUs that you can purchase for a customer * SKUs that you can upgrade or downgrade for an entitlement. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid.
    /// 
    /// # Arguments
    ///
    /// * `customer` - Required. The resource name of the customer to list SKUs for. Format: accounts/{account_id}/customers/{customer_id}.
    pub fn customers_list_purchasable_skus(&self, customer: &str) -> AccountCustomerListPurchasableSkuCall<'a, S> {
        AccountCustomerListPurchasableSkuCall {
            hub: self.hub,
            _customer: customer.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _language_code: Default::default(),
            _create_entitlement_purchase_product: Default::default(),
            _change_offer_purchase_entitlement: Default::default(),
            _change_offer_purchase_change_type: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing Customer resource for the reseller or distributor. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: No Customer resource found for the name in the request. Return value: The updated Customer resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name of the customer. Format: accounts/{account_id}/customers/{customer_id}
    pub fn customers_patch(&self, request: GoogleCloudChannelV1Customer, name: &str) -> AccountCustomerPatchCall<'a, S> {
        AccountCustomerPatchCall {
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
    /// Creates a Cloud Identity for the given customer using the customer's information, or the information provided here. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: The customer was not found. * ALREADY_EXISTS: The customer's primary email already exists. Retry after changing the customer's primary contact email. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata contains an instance of OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `customer` - Required. Resource name of the customer. Format: accounts/{account_id}/customers/{customer_id}
    pub fn customers_provision_cloud_identity(&self, request: GoogleCloudChannelV1ProvisionCloudIdentityRequest, customer: &str) -> AccountCustomerProvisionCloudIdentityCall<'a, S> {
        AccountCustomerProvisionCloudIdentityCall {
            hub: self.hub,
            _request: request,
            _customer: customer.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Transfers customer entitlements to new reseller. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: The customer or offer resource was not found. * ALREADY_EXISTS: The SKU was already transferred for the customer. * CONDITION_NOT_MET or FAILED_PRECONDITION: * The SKU requires domain verification to transfer, but the domain is not verified. * An Add-On SKU (example, Vault or Drive) is missing the pre-requisite SKU (example, G Suite Basic). * (Developer accounts only) Reseller and resold domain must meet the following naming requirements: * Domain names must start with goog-test. * Domain names must include the reseller domain. * Specify all transferring entitlements. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata will contain an instance of OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the reseller's customer account that will receive transferred entitlements. Parent uses the format: accounts/{account_id}/customers/{customer_id}
    pub fn customers_transfer_entitlements(&self, request: GoogleCloudChannelV1TransferEntitlementsRequest, parent: &str) -> AccountCustomerTransferEntitlementCall<'a, S> {
        AccountCustomerTransferEntitlementCall {
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
    /// Transfers customer entitlements from their current reseller to Google. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: The customer or offer resource was not found. * ALREADY_EXISTS: The SKU was already transferred for the customer. * CONDITION_NOT_MET or FAILED_PRECONDITION: * The SKU requires domain verification to transfer, but the domain is not verified. * An Add-On SKU (example, Vault or Drive) is missing the pre-requisite SKU (example, G Suite Basic). * (Developer accounts only) Reseller and resold domain must meet the following naming requirements: * Domain names must start with goog-test. * Domain names must include the reseller domain. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The response will contain google.protobuf.Empty on success. The Operation metadata will contain an instance of OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the reseller's customer account where the entitlements transfer from. Parent uses the format: accounts/{account_id}/customers/{customer_id}
    pub fn customers_transfer_entitlements_to_google(&self, request: GoogleCloudChannelV1TransferEntitlementsToGoogleRequest, parent: &str) -> AccountCustomerTransferEntitlementsToGoogleCall<'a, S> {
        AccountCustomerTransferEntitlementsToGoogleCall {
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
    /// Lists the Offers the reseller can sell. Possible error codes: * INVALID_ARGUMENT: Required request parameters are missing or invalid.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the reseller account from which to list Offers. Parent uses the format: accounts/{account_id}.
    pub fn offers_list(&self, parent: &str) -> AccountOfferListCall<'a, S> {
        AccountOfferListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _language_code: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves data generated by CloudChannelReportsService.RunReportJob.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `reportJob` - Required. The report job created by CloudChannelReportsService.RunReportJob. Report_job uses the format: accounts/{account_id}/reportJobs/{report_job_id}
    pub fn report_jobs_fetch_report_results(&self, request: GoogleCloudChannelV1FetchReportResultsRequest, report_job: &str) -> AccountReportJobFetchReportResultCall<'a, S> {
        AccountReportJobFetchReportResultCall {
            hub: self.hub,
            _request: request,
            _report_job: report_job.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the reports that RunReportJob can run. These reports include an ID, a description, and the list of columns that will be in the result.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the partner account to list available reports for. Parent uses the format: accounts/{account_id}
    pub fn reports_list(&self, parent: &str) -> AccountReportListCall<'a, S> {
        AccountReportListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _language_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Begins generation of data for a given report. The report identifier is a UID (for example, `613bf59q`). Possible error codes: * PERMISSION_DENIED: The user doesn't have access to this report. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: The report identifier was not found. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata contains an instance of OperationMetadata. To get the results of report generation, call CloudChannelReportsService.FetchReportResults with the RunReportJobResponse.report_job.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The report's resource name. Specifies the account and report used to generate report data. The report_id identifier is a UID (for example, `613bf59q`). Name uses the format: accounts/{account_id}/reports/{report_id}
    pub fn reports_run(&self, request: GoogleCloudChannelV1RunReportJobRequest, name: &str) -> AccountReportRunCall<'a, S> {
        AccountReportRunCall {
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
    /// Confirms the existence of Cloud Identity accounts based on the domain and if the Cloud Identity accounts are owned by the reseller. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * INVALID_VALUE: Invalid domain value in the request. Return value: A list of CloudIdentityCustomerAccount resources for the domain (may be empty) Note: in the v1alpha1 version of the API, a NOT_FOUND error returns if no CloudIdentityCustomerAccount resources match the domain.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The reseller account's resource name. Parent uses the format: accounts/{account_id}
    pub fn check_cloud_identity_accounts_exist(&self, request: GoogleCloudChannelV1CheckCloudIdentityAccountsExistRequest, parent: &str) -> AccountCheckCloudIdentityAccountsExistCall<'a, S> {
        AccountCheckCloudIdentityAccountsExistCall {
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
    /// Lists service accounts with subscriber privileges on the Cloud Pub/Sub topic created for this Channel Services account. Possible error codes: * PERMISSION_DENIED: The reseller account making the request and the provided reseller account are different, or the impersonated user is not a super admin. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: The topic resource doesn't exist. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: A list of service email addresses.
    /// 
    /// # Arguments
    ///
    /// * `account` - Required. Resource name of the account.
    pub fn list_subscribers(&self, account: &str) -> AccountListSubscriberCall<'a, S> {
        AccountListSubscriberCall {
            hub: self.hub,
            _account: account.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List TransferableOffers of a customer based on Cloud Identity ID or Customer Name in the request. Use this method when a reseller gets the entitlement information of an unowned customer. The reseller should provide the customer's Cloud Identity ID or Customer Name. Possible error codes: * PERMISSION_DENIED: * The customer doesn't belong to the reseller and has no auth token. * The customer provided incorrect reseller information when generating auth token. * The reseller account making the request is different from the reseller account in the query. * INVALID_ARGUMENT: Required request parameters are missing or invalid. Return value: List of TransferableOffer for the given customer and SKU.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the reseller's account.
    pub fn list_transferable_offers(&self, request: GoogleCloudChannelV1ListTransferableOffersRequest, parent: &str) -> AccountListTransferableOfferCall<'a, S> {
        AccountListTransferableOfferCall {
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
    /// List TransferableSkus of a customer based on the Cloud Identity ID or Customer Name in the request. Use this method to list the entitlements information of an unowned customer. You should provide the customer's Cloud Identity ID or Customer Name. Possible error codes: * PERMISSION_DENIED: * The customer doesn't belong to the reseller and has no auth token. * The supplied auth token is invalid. * The reseller account making the request is different from the reseller account in the query. * INVALID_ARGUMENT: Required request parameters are missing or invalid. Return value: A list of the customer's TransferableSku.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The reseller account's resource name. Parent uses the format: accounts/{account_id}
    pub fn list_transferable_skus(&self, request: GoogleCloudChannelV1ListTransferableSkusRequest, parent: &str) -> AccountListTransferableSkuCall<'a, S> {
        AccountListTransferableSkuCall {
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
    /// Registers a service account with subscriber privileges on the Cloud Pub/Sub topic for this Channel Services account. After you create a subscriber, you get the events through SubscriberEvent Possible error codes: * PERMISSION_DENIED: The reseller account making the request and the provided reseller account are different, or the impersonated user is not a super admin. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The topic name with the registered service email address.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `account` - Required. Resource name of the account.
    pub fn register(&self, request: GoogleCloudChannelV1RegisterSubscriberRequest, account: &str) -> AccountRegisterCall<'a, S> {
        AccountRegisterCall {
            hub: self.hub,
            _request: request,
            _account: account.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Unregisters a service account with subscriber privileges on the Cloud Pub/Sub topic created for this Channel Services account. If there are no service accounts left with subscriber privileges, this deletes the topic. You can call ListSubscribers to check for these accounts. Possible error codes: * PERMISSION_DENIED: The reseller account making the request and the provided reseller account are different, or the impersonated user is not a super admin. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: The topic resource doesn't exist. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The topic name that unregistered the service email address. Returns a success response if the service email address wasn't registered with the topic.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `account` - Required. Resource name of the account.
    pub fn unregister(&self, request: GoogleCloudChannelV1UnregisterSubscriberRequest, account: &str) -> AccountUnregisterCall<'a, S> {
        AccountUnregisterCall {
            hub: self.hub,
            _request: request,
            _account: account.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *operation* resources.
/// It is not used directly, but through the [`Cloudchannel`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudchannel1 as cloudchannel1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudchannel1::{Cloudchannel, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `cancel(...)`, `delete(...)`, `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.operations();
/// # }
/// ```
pub struct OperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Cloudchannel<S>,
}

impl<'a, S> client::MethodsBuilder for OperationMethods<'a, S> {}

impl<'a, S> OperationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn cancel(&self, request: GoogleLongrunningCancelOperationRequest, name: &str) -> OperationCancelCall<'a, S> {
        OperationCancelCall {
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
    /// Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be deleted.
    pub fn delete(&self, name: &str) -> OperationDeleteCall<'a, S> {
        OperationDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn get(&self, name: &str) -> OperationGetCall<'a, S> {
        OperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `"/v1/{name=users/*}/operations"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn list(&self, name: &str) -> OperationListCall<'a, S> {
        OperationListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *product* resources.
/// It is not used directly, but through the [`Cloudchannel`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudchannel1 as cloudchannel1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudchannel1::{Cloudchannel, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)` and `skus_list(...)`
/// // to build up your call.
/// let rb = hub.products();
/// # }
/// ```
pub struct ProductMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Cloudchannel<S>,
}

impl<'a, S> client::MethodsBuilder for ProductMethods<'a, S> {}

impl<'a, S> ProductMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the SKUs for a product the reseller is authorized to sell. Possible error codes: * INVALID_ARGUMENT: Required request parameters are missing or invalid.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the Product to list SKUs for. Parent uses the format: products/{product_id}. Supports products/- to retrieve SKUs for all products.
    pub fn skus_list(&self, parent: &str) -> ProductSkuListCall<'a, S> {
        ProductSkuListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _language_code: Default::default(),
            _account: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the Products the reseller is authorized to sell. Possible error codes: * INVALID_ARGUMENT: Required request parameters are missing or invalid.
    pub fn list(&self) -> ProductListCall<'a, S> {
        ProductListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _language_code: Default::default(),
            _account: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



