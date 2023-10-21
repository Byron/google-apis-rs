use super::*;
/// A builder providing access to all methods supported on *customer* resources.
/// It is not used directly, but through the [`AndroidProvisioningPartner`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androiddeviceprovisioning1 as androiddeviceprovisioning1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androiddeviceprovisioning1::{AndroidProvisioningPartner, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidProvisioningPartner::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `configurations_create(...)`, `configurations_delete(...)`, `configurations_get(...)`, `configurations_list(...)`, `configurations_patch(...)`, `devices_apply_configuration(...)`, `devices_get(...)`, `devices_list(...)`, `devices_remove_configuration(...)`, `devices_unclaim(...)`, `dpcs_list(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.customers();
/// # }
/// ```
pub struct CustomerMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidProvisioningPartner<S>,
}

impl<'a, S> client::MethodsBuilder for CustomerMethods<'a, S> {}

impl<'a, S> CustomerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new configuration. Once created, a customer can apply the configuration to devices.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The customer that manages the configuration. An API resource name in the format `customers/[CUSTOMER_ID]`. This field has custom validation in CreateConfigurationRequestValidator
    pub fn configurations_create(&self, request: Configuration, parent: &str) -> CustomerConfigurationCreateCall<'a, S> {
        CustomerConfigurationCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an unused configuration. The API call fails if the customer has devices with the configuration applied.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The configuration to delete. An API resource name in the format `customers/[CUSTOMER_ID]/configurations/[CONFIGURATION_ID]`. If the configuration is applied to any devices, the API call fails.
    pub fn configurations_delete(&self, name: &str) -> CustomerConfigurationDeleteCall<'a, S> {
        CustomerConfigurationDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the details of a configuration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The configuration to get. An API resource name in the format `customers/[CUSTOMER_ID]/configurations/[CONFIGURATION_ID]`.
    pub fn configurations_get(&self, name: &str) -> CustomerConfigurationGetCall<'a, S> {
        CustomerConfigurationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists a customer's configurations.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The customer that manages the listed configurations. An API resource name in the format `customers/[CUSTOMER_ID]`.
    pub fn configurations_list(&self, parent: &str) -> CustomerConfigurationListCall<'a, S> {
        CustomerConfigurationListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a configuration's field values.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The API resource name in the format `customers/[CUSTOMER_ID]/configurations/[CONFIGURATION_ID]`. Assigned by the server.
    pub fn configurations_patch(&self, request: Configuration, name: &str) -> CustomerConfigurationPatchCall<'a, S> {
        CustomerConfigurationPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Applies a Configuration to the device to register the device for zero-touch enrollment. After applying a configuration to a device, the device automatically provisions itself on first boot, or next factory reset.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The customer managing the device. An API resource name in the format `customers/[CUSTOMER_ID]`.
    pub fn devices_apply_configuration(&self, request: CustomerApplyConfigurationRequest, parent: &str) -> CustomerDeviceApplyConfigurationCall<'a, S> {
        CustomerDeviceApplyConfigurationCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the details of a device.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The device to get. An API resource name in the format `customers/[CUSTOMER_ID]/devices/[DEVICE_ID]`.
    pub fn devices_get(&self, name: &str) -> CustomerDeviceGetCall<'a, S> {
        CustomerDeviceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists a customer's devices.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The customer managing the devices. An API resource name in the format `customers/[CUSTOMER_ID]`.
    pub fn devices_list(&self, parent: &str) -> CustomerDeviceListCall<'a, S> {
        CustomerDeviceListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes a configuration from device.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The customer managing the device in the format `customers/[CUSTOMER_ID]`.
    pub fn devices_remove_configuration(&self, request: CustomerRemoveConfigurationRequest, parent: &str) -> CustomerDeviceRemoveConfigurationCall<'a, S> {
        CustomerDeviceRemoveConfigurationCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Unclaims a device from a customer and removes it from zero-touch enrollment. After removing a device, a customer must contact their reseller to register the device into zero-touch enrollment again.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The customer managing the device. An API resource name in the format `customers/[CUSTOMER_ID]`.
    pub fn devices_unclaim(&self, request: CustomerUnclaimDeviceRequest, parent: &str) -> CustomerDeviceUnclaimCall<'a, S> {
        CustomerDeviceUnclaimCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the DPCs (device policy controllers) that support zero-touch enrollment.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The customer that can use the DPCs in configurations. An API resource name in the format `customers/[CUSTOMER_ID]`.
    pub fn dpcs_list(&self, parent: &str) -> CustomerDpcListCall<'a, S> {
        CustomerDpcListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the user's customer accounts.
    pub fn list(&self) -> CustomerListCall<'a, S> {
        CustomerListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *operation* resources.
/// It is not used directly, but through the [`AndroidProvisioningPartner`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androiddeviceprovisioning1 as androiddeviceprovisioning1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androiddeviceprovisioning1::{AndroidProvisioningPartner, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidProvisioningPartner::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.operations();
/// # }
/// ```
pub struct OperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidProvisioningPartner<S>,
}

impl<'a, S> client::MethodsBuilder for OperationMethods<'a, S> {}

impl<'a, S> OperationMethods<'a, S> {
    
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
        }
    }
}



/// A builder providing access to all methods supported on *partner* resources.
/// It is not used directly, but through the [`AndroidProvisioningPartner`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androiddeviceprovisioning1 as androiddeviceprovisioning1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androiddeviceprovisioning1::{AndroidProvisioningPartner, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidProvisioningPartner::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `customers_create(...)`, `customers_list(...)`, `devices_claim(...)`, `devices_claim_async(...)`, `devices_find_by_identifier(...)`, `devices_find_by_owner(...)`, `devices_get(...)`, `devices_metadata(...)`, `devices_unclaim(...)`, `devices_unclaim_async(...)`, `devices_update_metadata_async(...)`, `vendors_customers_list(...)` and `vendors_list(...)`
/// // to build up your call.
/// let rb = hub.partners();
/// # }
/// ```
pub struct PartnerMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidProvisioningPartner<S>,
}

impl<'a, S> client::MethodsBuilder for PartnerMethods<'a, S> {}

impl<'a, S> PartnerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a customer for zero-touch enrollment. After the method returns successfully, admin and owner roles can manage devices and EMM configs by calling API methods or using their zero-touch enrollment portal. The customer receives an email that welcomes them to zero-touch enrollment and explains how to sign into the portal.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource ID in the format `partners/[PARTNER_ID]` that identifies the reseller.
    pub fn customers_create(&self, request: CreateCustomerRequest, parent: &str) -> PartnerCustomerCreateCall<'a, S> {
        PartnerCustomerCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the customers that are enrolled to the reseller identified by the `partnerId` argument. This list includes customers that the reseller created and customers that enrolled themselves using the portal.
    /// 
    /// # Arguments
    ///
    /// * `partnerId` - Required. The ID of the reseller partner.
    pub fn customers_list(&self, partner_id: i64) -> PartnerCustomerListCall<'a, S> {
        PartnerCustomerListCall {
            hub: self.hub,
            _partner_id: partner_id,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Claims a device for a customer and adds it to zero-touch enrollment. If the device is already claimed by another customer, the call returns an error.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `partnerId` - Required. The ID of the reseller partner.
    pub fn devices_claim(&self, request: ClaimDeviceRequest, partner_id: i64) -> PartnerDeviceClaimCall<'a, S> {
        PartnerDeviceClaimCall {
            hub: self.hub,
            _request: request,
            _partner_id: partner_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Claims a batch of devices for a customer asynchronously. Adds the devices to zero-touch enrollment. To learn more, read [Long‑running batch operations](https://developers.google.com/zero-touch/guides/how-it-works#operations).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `partnerId` - Required. The ID of the reseller partner.
    pub fn devices_claim_async(&self, request: ClaimDevicesRequest, partner_id: i64) -> PartnerDeviceClaimAsyncCall<'a, S> {
        PartnerDeviceClaimAsyncCall {
            hub: self.hub,
            _request: request,
            _partner_id: partner_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Finds devices by hardware identifiers, such as IMEI.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `partnerId` - Required. The ID of the reseller partner.
    pub fn devices_find_by_identifier(&self, request: FindDevicesByDeviceIdentifierRequest, partner_id: i64) -> PartnerDeviceFindByIdentifierCall<'a, S> {
        PartnerDeviceFindByIdentifierCall {
            hub: self.hub,
            _request: request,
            _partner_id: partner_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Finds devices claimed for customers. The results only contain devices registered to the reseller that's identified by the `partnerId` argument. The customer's devices purchased from other resellers don't appear in the results.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `partnerId` - Required. The ID of the reseller partner.
    pub fn devices_find_by_owner(&self, request: FindDevicesByOwnerRequest, partner_id: i64) -> PartnerDeviceFindByOwnerCall<'a, S> {
        PartnerDeviceFindByOwnerCall {
            hub: self.hub,
            _request: request,
            _partner_id: partner_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a device.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The device API resource name in the format `partners/[PARTNER_ID]/devices/[DEVICE_ID]`.
    pub fn devices_get(&self, name: &str) -> PartnerDeviceGetCall<'a, S> {
        PartnerDeviceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates reseller metadata associated with the device. Android devices only.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `metadataOwnerId` - Required. The owner of the newly set metadata. Set this to the partner ID.
    /// * `deviceId` - Required. The ID of the device.
    pub fn devices_metadata(&self, request: UpdateDeviceMetadataRequest, metadata_owner_id: i64, device_id: i64) -> PartnerDeviceMetadataCall<'a, S> {
        PartnerDeviceMetadataCall {
            hub: self.hub,
            _request: request,
            _metadata_owner_id: metadata_owner_id,
            _device_id: device_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Unclaims a device from a customer and removes it from zero-touch enrollment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `partnerId` - Required. The ID of the reseller partner.
    pub fn devices_unclaim(&self, request: UnclaimDeviceRequest, partner_id: i64) -> PartnerDeviceUnclaimCall<'a, S> {
        PartnerDeviceUnclaimCall {
            hub: self.hub,
            _request: request,
            _partner_id: partner_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Unclaims a batch of devices for a customer asynchronously. Removes the devices from zero-touch enrollment. To learn more, read [Long‑running batch operations](https://developers.google.com/zero-touch/guides/how-it-works#operations).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `partnerId` - Required. The reseller partner ID.
    pub fn devices_unclaim_async(&self, request: UnclaimDevicesRequest, partner_id: i64) -> PartnerDeviceUnclaimAsyncCall<'a, S> {
        PartnerDeviceUnclaimAsyncCall {
            hub: self.hub,
            _request: request,
            _partner_id: partner_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the reseller metadata attached to a batch of devices. This method updates devices asynchronously and returns an `Operation` that can be used to track progress. Read [Long‑running batch operations](https://developers.google.com/zero-touch/guides/how-it-works#operations). Android Devices only.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `partnerId` - Required. The reseller partner ID.
    pub fn devices_update_metadata_async(&self, request: UpdateDeviceMetadataInBatchRequest, partner_id: i64) -> PartnerDeviceUpdateMetadataAsyncCall<'a, S> {
        PartnerDeviceUpdateMetadataAsyncCall {
            hub: self.hub,
            _request: request,
            _partner_id: partner_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the customers of the vendor.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name in the format `partners/[PARTNER_ID]/vendors/[VENDOR_ID]`.
    pub fn vendors_customers_list(&self, parent: &str) -> PartnerVendorCustomerListCall<'a, S> {
        PartnerVendorCustomerListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the vendors of the partner.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name in the format `partners/[PARTNER_ID]`.
    pub fn vendors_list(&self, parent: &str) -> PartnerVendorListCall<'a, S> {
        PartnerVendorListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



