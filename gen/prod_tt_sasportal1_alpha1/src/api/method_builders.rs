use super::*;
/// A builder providing access to all methods supported on *customer* resources.
/// It is not used directly, but through the [`SASPortalTesting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_prod_tt_sasportal1_alpha1 as prod_tt_sasportal1_alpha1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use prod_tt_sasportal1_alpha1::{SASPortalTesting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = SASPortalTesting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `deployments_create(...)`, `deployments_delete(...)`, `deployments_devices_create(...)`, `deployments_devices_create_signed(...)`, `deployments_devices_list(...)`, `deployments_get(...)`, `deployments_list(...)`, `deployments_move(...)`, `deployments_patch(...)`, `devices_create(...)`, `devices_create_signed(...)`, `devices_delete(...)`, `devices_get(...)`, `devices_list(...)`, `devices_move(...)`, `devices_patch(...)`, `devices_sign_device(...)`, `devices_update_signed(...)`, `get(...)`, `list(...)`, `nodes_create(...)`, `nodes_delete(...)`, `nodes_deployments_create(...)`, `nodes_deployments_list(...)`, `nodes_devices_create(...)`, `nodes_devices_create_signed(...)`, `nodes_devices_list(...)`, `nodes_get(...)`, `nodes_list(...)`, `nodes_move(...)`, `nodes_nodes_create(...)`, `nodes_nodes_list(...)`, `nodes_patch(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.customers();
/// # }
/// ```
pub struct CustomerMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a SASPortalTesting<S>,
}

impl<'a, S> client::MethodsBuilder for CustomerMethods<'a, S> {}

impl<'a, S> CustomerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a device under a node or customer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent resource.
    pub fn deployments_devices_create(&self, request: SasPortalDevice, parent: &str) -> CustomerDeploymentDeviceCreateCall<'a, S> {
        CustomerDeploymentDeviceCreateCall {
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
    /// Creates a signed device under a node or customer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent resource.
    pub fn deployments_devices_create_signed(&self, request: SasPortalCreateSignedDeviceRequest, parent: &str) -> CustomerDeploymentDeviceCreateSignedCall<'a, S> {
        CustomerDeploymentDeviceCreateSignedCall {
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
    /// Lists devices under a node or customer.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the parent resource.
    pub fn deployments_devices_list(&self, parent: &str) -> CustomerDeploymentDeviceListCall<'a, S> {
        CustomerDeploymentDeviceListCall {
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
    /// Creates a new deployment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource name where the deployment is to be created.
    pub fn deployments_create(&self, request: SasPortalDeployment, parent: &str) -> CustomerDeploymentCreateCall<'a, S> {
        CustomerDeploymentCreateCall {
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
    /// Deletes a deployment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the deployment.
    pub fn deployments_delete(&self, name: &str) -> CustomerDeploymentDeleteCall<'a, S> {
        CustomerDeploymentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a requested deployment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the deployment.
    pub fn deployments_get(&self, name: &str) -> CustomerDeploymentGetCall<'a, S> {
        CustomerDeploymentGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists deployments.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource name, for example, "nodes/1", customer/1/nodes/2.
    pub fn deployments_list(&self, parent: &str) -> CustomerDeploymentListCall<'a, S> {
        CustomerDeploymentListCall {
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
    /// Moves a deployment under another node or customer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the deployment to move.
    pub fn deployments_move(&self, request: SasPortalMoveDeploymentRequest, name: &str) -> CustomerDeploymentMoveCall<'a, S> {
        CustomerDeploymentMoveCall {
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
    /// Updates an existing deployment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name.
    pub fn deployments_patch(&self, request: SasPortalDeployment, name: &str) -> CustomerDeploymentPatchCall<'a, S> {
        CustomerDeploymentPatchCall {
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
    /// Creates a device under a node or customer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent resource.
    pub fn devices_create(&self, request: SasPortalDevice, parent: &str) -> CustomerDeviceCreateCall<'a, S> {
        CustomerDeviceCreateCall {
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
    /// Creates a signed device under a node or customer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent resource.
    pub fn devices_create_signed(&self, request: SasPortalCreateSignedDeviceRequest, parent: &str) -> CustomerDeviceCreateSignedCall<'a, S> {
        CustomerDeviceCreateSignedCall {
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
    /// Deletes a device.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the device.
    pub fn devices_delete(&self, name: &str) -> CustomerDeviceDeleteCall<'a, S> {
        CustomerDeviceDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details about a device.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the device.
    pub fn devices_get(&self, name: &str) -> CustomerDeviceGetCall<'a, S> {
        CustomerDeviceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists devices under a node or customer.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the parent resource.
    pub fn devices_list(&self, parent: &str) -> CustomerDeviceListCall<'a, S> {
        CustomerDeviceListCall {
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
    /// Moves a device under another node or customer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the device to move.
    pub fn devices_move(&self, request: SasPortalMoveDeviceRequest, name: &str) -> CustomerDeviceMoveCall<'a, S> {
        CustomerDeviceMoveCall {
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
    /// Updates a device.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource path name.
    pub fn devices_patch(&self, request: SasPortalDevice, name: &str) -> CustomerDevicePatchCall<'a, S> {
        CustomerDevicePatchCall {
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
    /// Signs a device.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource path name.
    pub fn devices_sign_device(&self, request: SasPortalSignDeviceRequest, name: &str) -> CustomerDeviceSignDeviceCall<'a, S> {
        CustomerDeviceSignDeviceCall {
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
    /// Updates a signed device.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the device to update.
    pub fn devices_update_signed(&self, request: SasPortalUpdateSignedDeviceRequest, name: &str) -> CustomerDeviceUpdateSignedCall<'a, S> {
        CustomerDeviceUpdateSignedCall {
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
    /// Creates a new deployment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource name where the deployment is to be created.
    pub fn nodes_deployments_create(&self, request: SasPortalDeployment, parent: &str) -> CustomerNodeDeploymentCreateCall<'a, S> {
        CustomerNodeDeploymentCreateCall {
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
    /// Lists deployments.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource name, for example, "nodes/1", customer/1/nodes/2.
    pub fn nodes_deployments_list(&self, parent: &str) -> CustomerNodeDeploymentListCall<'a, S> {
        CustomerNodeDeploymentListCall {
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
    /// Creates a device under a node or customer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent resource.
    pub fn nodes_devices_create(&self, request: SasPortalDevice, parent: &str) -> CustomerNodeDeviceCreateCall<'a, S> {
        CustomerNodeDeviceCreateCall {
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
    /// Creates a signed device under a node or customer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent resource.
    pub fn nodes_devices_create_signed(&self, request: SasPortalCreateSignedDeviceRequest, parent: &str) -> CustomerNodeDeviceCreateSignedCall<'a, S> {
        CustomerNodeDeviceCreateSignedCall {
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
    /// Lists devices under a node or customer.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the parent resource.
    pub fn nodes_devices_list(&self, parent: &str) -> CustomerNodeDeviceListCall<'a, S> {
        CustomerNodeDeviceListCall {
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
    /// Creates a new node.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource name where the node is to be created.
    pub fn nodes_nodes_create(&self, request: SasPortalNode, parent: &str) -> CustomerNodeNodeCreateCall<'a, S> {
        CustomerNodeNodeCreateCall {
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
    /// Lists nodes.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource name, for example, "nodes/1".
    pub fn nodes_nodes_list(&self, parent: &str) -> CustomerNodeNodeListCall<'a, S> {
        CustomerNodeNodeListCall {
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
    /// Creates a new node.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource name where the node is to be created.
    pub fn nodes_create(&self, request: SasPortalNode, parent: &str) -> CustomerNodeCreateCall<'a, S> {
        CustomerNodeCreateCall {
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
    /// Deletes a node.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the node.
    pub fn nodes_delete(&self, name: &str) -> CustomerNodeDeleteCall<'a, S> {
        CustomerNodeDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a requested node.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the node.
    pub fn nodes_get(&self, name: &str) -> CustomerNodeGetCall<'a, S> {
        CustomerNodeGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists nodes.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource name, for example, "nodes/1".
    pub fn nodes_list(&self, parent: &str) -> CustomerNodeListCall<'a, S> {
        CustomerNodeListCall {
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
    /// Moves a node under another node or customer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the node to move.
    pub fn nodes_move(&self, request: SasPortalMoveNodeRequest, name: &str) -> CustomerNodeMoveCall<'a, S> {
        CustomerNodeMoveCall {
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
    /// Updates an existing node.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name.
    pub fn nodes_patch(&self, request: SasPortalNode, name: &str) -> CustomerNodePatchCall<'a, S> {
        CustomerNodePatchCall {
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
    /// Returns a requested customer.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the customer.
    pub fn get(&self, name: &str) -> CustomerGetCall<'a, S> {
        CustomerGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of requested customers.
    pub fn list(&self) -> CustomerListCall<'a, S> {
        CustomerListCall {
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
    /// Updates an existing customer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name of the customer.
    pub fn patch(&self, request: SasPortalCustomer, name: &str) -> CustomerPatchCall<'a, S> {
        CustomerPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *deployment* resources.
/// It is not used directly, but through the [`SASPortalTesting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_prod_tt_sasportal1_alpha1 as prod_tt_sasportal1_alpha1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use prod_tt_sasportal1_alpha1::{SASPortalTesting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = SASPortalTesting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `devices_delete(...)`, `devices_get(...)`, `devices_move(...)`, `devices_patch(...)`, `devices_sign_device(...)`, `devices_update_signed(...)` and `get(...)`
/// // to build up your call.
/// let rb = hub.deployments();
/// # }
/// ```
pub struct DeploymentMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a SASPortalTesting<S>,
}

impl<'a, S> client::MethodsBuilder for DeploymentMethods<'a, S> {}

impl<'a, S> DeploymentMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a device.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the device.
    pub fn devices_delete(&self, name: &str) -> DeploymentDeviceDeleteCall<'a, S> {
        DeploymentDeviceDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details about a device.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the device.
    pub fn devices_get(&self, name: &str) -> DeploymentDeviceGetCall<'a, S> {
        DeploymentDeviceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Moves a device under another node or customer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the device to move.
    pub fn devices_move(&self, request: SasPortalMoveDeviceRequest, name: &str) -> DeploymentDeviceMoveCall<'a, S> {
        DeploymentDeviceMoveCall {
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
    /// Updates a device.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource path name.
    pub fn devices_patch(&self, request: SasPortalDevice, name: &str) -> DeploymentDevicePatchCall<'a, S> {
        DeploymentDevicePatchCall {
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
    /// Signs a device.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource path name.
    pub fn devices_sign_device(&self, request: SasPortalSignDeviceRequest, name: &str) -> DeploymentDeviceSignDeviceCall<'a, S> {
        DeploymentDeviceSignDeviceCall {
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
    /// Updates a signed device.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the device to update.
    pub fn devices_update_signed(&self, request: SasPortalUpdateSignedDeviceRequest, name: &str) -> DeploymentDeviceUpdateSignedCall<'a, S> {
        DeploymentDeviceUpdateSignedCall {
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
    /// Returns a requested deployment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the deployment.
    pub fn get(&self, name: &str) -> DeploymentGetCall<'a, S> {
        DeploymentGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *installer* resources.
/// It is not used directly, but through the [`SASPortalTesting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_prod_tt_sasportal1_alpha1 as prod_tt_sasportal1_alpha1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use prod_tt_sasportal1_alpha1::{SASPortalTesting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = SASPortalTesting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `generate_secret(...)` and `validate(...)`
/// // to build up your call.
/// let rb = hub.installer();
/// # }
/// ```
pub struct InstallerMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a SASPortalTesting<S>,
}

impl<'a, S> client::MethodsBuilder for InstallerMethods<'a, S> {}

impl<'a, S> InstallerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generates a secret to be used with the ValidateInstaller.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn generate_secret(&self, request: SasPortalGenerateSecretRequest) -> InstallerGenerateSecretCall<'a, S> {
        InstallerGenerateSecretCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Validates the identity of a Certified Professional Installer (CPI).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn validate(&self, request: SasPortalValidateInstallerRequest) -> InstallerValidateCall<'a, S> {
        InstallerValidateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *node* resources.
/// It is not used directly, but through the [`SASPortalTesting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_prod_tt_sasportal1_alpha1 as prod_tt_sasportal1_alpha1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use prod_tt_sasportal1_alpha1::{SASPortalTesting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = SASPortalTesting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `deployments_delete(...)`, `deployments_devices_create(...)`, `deployments_devices_create_signed(...)`, `deployments_devices_list(...)`, `deployments_get(...)`, `deployments_list(...)`, `deployments_move(...)`, `deployments_patch(...)`, `devices_create(...)`, `devices_create_signed(...)`, `devices_delete(...)`, `devices_get(...)`, `devices_list(...)`, `devices_move(...)`, `devices_patch(...)`, `devices_sign_device(...)`, `devices_update_signed(...)`, `get(...)`, `nodes_create(...)`, `nodes_delete(...)`, `nodes_deployments_create(...)`, `nodes_deployments_list(...)`, `nodes_devices_create(...)`, `nodes_devices_create_signed(...)`, `nodes_devices_list(...)`, `nodes_get(...)`, `nodes_list(...)`, `nodes_move(...)`, `nodes_nodes_create(...)`, `nodes_nodes_list(...)` and `nodes_patch(...)`
/// // to build up your call.
/// let rb = hub.nodes();
/// # }
/// ```
pub struct NodeMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a SASPortalTesting<S>,
}

impl<'a, S> client::MethodsBuilder for NodeMethods<'a, S> {}

impl<'a, S> NodeMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a device under a node or customer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent resource.
    pub fn deployments_devices_create(&self, request: SasPortalDevice, parent: &str) -> NodeDeploymentDeviceCreateCall<'a, S> {
        NodeDeploymentDeviceCreateCall {
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
    /// Creates a signed device under a node or customer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent resource.
    pub fn deployments_devices_create_signed(&self, request: SasPortalCreateSignedDeviceRequest, parent: &str) -> NodeDeploymentDeviceCreateSignedCall<'a, S> {
        NodeDeploymentDeviceCreateSignedCall {
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
    /// Lists devices under a node or customer.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the parent resource.
    pub fn deployments_devices_list(&self, parent: &str) -> NodeDeploymentDeviceListCall<'a, S> {
        NodeDeploymentDeviceListCall {
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
    /// Deletes a deployment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the deployment.
    pub fn deployments_delete(&self, name: &str) -> NodeDeploymentDeleteCall<'a, S> {
        NodeDeploymentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a requested deployment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the deployment.
    pub fn deployments_get(&self, name: &str) -> NodeDeploymentGetCall<'a, S> {
        NodeDeploymentGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists deployments.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource name, for example, "nodes/1", customer/1/nodes/2.
    pub fn deployments_list(&self, parent: &str) -> NodeDeploymentListCall<'a, S> {
        NodeDeploymentListCall {
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
    /// Moves a deployment under another node or customer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the deployment to move.
    pub fn deployments_move(&self, request: SasPortalMoveDeploymentRequest, name: &str) -> NodeDeploymentMoveCall<'a, S> {
        NodeDeploymentMoveCall {
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
    /// Updates an existing deployment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name.
    pub fn deployments_patch(&self, request: SasPortalDeployment, name: &str) -> NodeDeploymentPatchCall<'a, S> {
        NodeDeploymentPatchCall {
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
    /// Creates a device under a node or customer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent resource.
    pub fn devices_create(&self, request: SasPortalDevice, parent: &str) -> NodeDeviceCreateCall<'a, S> {
        NodeDeviceCreateCall {
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
    /// Creates a signed device under a node or customer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent resource.
    pub fn devices_create_signed(&self, request: SasPortalCreateSignedDeviceRequest, parent: &str) -> NodeDeviceCreateSignedCall<'a, S> {
        NodeDeviceCreateSignedCall {
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
    /// Deletes a device.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the device.
    pub fn devices_delete(&self, name: &str) -> NodeDeviceDeleteCall<'a, S> {
        NodeDeviceDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details about a device.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the device.
    pub fn devices_get(&self, name: &str) -> NodeDeviceGetCall<'a, S> {
        NodeDeviceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists devices under a node or customer.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the parent resource.
    pub fn devices_list(&self, parent: &str) -> NodeDeviceListCall<'a, S> {
        NodeDeviceListCall {
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
    /// Moves a device under another node or customer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the device to move.
    pub fn devices_move(&self, request: SasPortalMoveDeviceRequest, name: &str) -> NodeDeviceMoveCall<'a, S> {
        NodeDeviceMoveCall {
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
    /// Updates a device.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource path name.
    pub fn devices_patch(&self, request: SasPortalDevice, name: &str) -> NodeDevicePatchCall<'a, S> {
        NodeDevicePatchCall {
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
    /// Signs a device.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource path name.
    pub fn devices_sign_device(&self, request: SasPortalSignDeviceRequest, name: &str) -> NodeDeviceSignDeviceCall<'a, S> {
        NodeDeviceSignDeviceCall {
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
    /// Updates a signed device.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the device to update.
    pub fn devices_update_signed(&self, request: SasPortalUpdateSignedDeviceRequest, name: &str) -> NodeDeviceUpdateSignedCall<'a, S> {
        NodeDeviceUpdateSignedCall {
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
    /// Creates a new deployment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource name where the deployment is to be created.
    pub fn nodes_deployments_create(&self, request: SasPortalDeployment, parent: &str) -> NodeNodeDeploymentCreateCall<'a, S> {
        NodeNodeDeploymentCreateCall {
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
    /// Lists deployments.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource name, for example, "nodes/1", customer/1/nodes/2.
    pub fn nodes_deployments_list(&self, parent: &str) -> NodeNodeDeploymentListCall<'a, S> {
        NodeNodeDeploymentListCall {
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
    /// Creates a device under a node or customer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent resource.
    pub fn nodes_devices_create(&self, request: SasPortalDevice, parent: &str) -> NodeNodeDeviceCreateCall<'a, S> {
        NodeNodeDeviceCreateCall {
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
    /// Creates a signed device under a node or customer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent resource.
    pub fn nodes_devices_create_signed(&self, request: SasPortalCreateSignedDeviceRequest, parent: &str) -> NodeNodeDeviceCreateSignedCall<'a, S> {
        NodeNodeDeviceCreateSignedCall {
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
    /// Lists devices under a node or customer.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the parent resource.
    pub fn nodes_devices_list(&self, parent: &str) -> NodeNodeDeviceListCall<'a, S> {
        NodeNodeDeviceListCall {
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
    /// Creates a new node.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource name where the node is to be created.
    pub fn nodes_nodes_create(&self, request: SasPortalNode, parent: &str) -> NodeNodeNodeCreateCall<'a, S> {
        NodeNodeNodeCreateCall {
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
    /// Lists nodes.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource name, for example, "nodes/1".
    pub fn nodes_nodes_list(&self, parent: &str) -> NodeNodeNodeListCall<'a, S> {
        NodeNodeNodeListCall {
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
    /// Creates a new node.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource name where the node is to be created.
    pub fn nodes_create(&self, request: SasPortalNode, parent: &str) -> NodeNodeCreateCall<'a, S> {
        NodeNodeCreateCall {
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
    /// Deletes a node.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the node.
    pub fn nodes_delete(&self, name: &str) -> NodeNodeDeleteCall<'a, S> {
        NodeNodeDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a requested node.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the node.
    pub fn nodes_get(&self, name: &str) -> NodeNodeGetCall<'a, S> {
        NodeNodeGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists nodes.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource name, for example, "nodes/1".
    pub fn nodes_list(&self, parent: &str) -> NodeNodeListCall<'a, S> {
        NodeNodeListCall {
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
    /// Moves a node under another node or customer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the node to move.
    pub fn nodes_move(&self, request: SasPortalMoveNodeRequest, name: &str) -> NodeNodeMoveCall<'a, S> {
        NodeNodeMoveCall {
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
    /// Updates an existing node.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name.
    pub fn nodes_patch(&self, request: SasPortalNode, name: &str) -> NodeNodePatchCall<'a, S> {
        NodeNodePatchCall {
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
    /// Returns a requested node.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the node.
    pub fn get(&self, name: &str) -> NodeGetCall<'a, S> {
        NodeGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *policy* resources.
/// It is not used directly, but through the [`SASPortalTesting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_prod_tt_sasportal1_alpha1 as prod_tt_sasportal1_alpha1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use prod_tt_sasportal1_alpha1::{SASPortalTesting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = SASPortalTesting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `set(...)` and `test(...)`
/// // to build up your call.
/// let rb = hub.policies();
/// # }
/// ```
pub struct PolicyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a SASPortalTesting<S>,
}

impl<'a, S> client::MethodsBuilder for PolicyMethods<'a, S> {}

impl<'a, S> PolicyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn get(&self, request: SasPortalGetPolicyRequest) -> PolicyGetCall<'a, S> {
        PolicyGetCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn set(&self, request: SasPortalSetPolicyRequest) -> PolicySetCall<'a, S> {
        PolicySetCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn test(&self, request: SasPortalTestPermissionsRequest) -> PolicyTestCall<'a, S> {
        PolicyTestCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



