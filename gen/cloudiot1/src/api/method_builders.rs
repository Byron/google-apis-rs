use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`CloudIot`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudiot1 as cloudiot1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudiot1::{CloudIot, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudIot::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_registries_bind_device_to_gateway(...)`, `locations_registries_create(...)`, `locations_registries_delete(...)`, `locations_registries_devices_config_versions_list(...)`, `locations_registries_devices_create(...)`, `locations_registries_devices_delete(...)`, `locations_registries_devices_get(...)`, `locations_registries_devices_list(...)`, `locations_registries_devices_modify_cloud_to_device_config(...)`, `locations_registries_devices_patch(...)`, `locations_registries_devices_send_command_to_device(...)`, `locations_registries_devices_states_list(...)`, `locations_registries_get(...)`, `locations_registries_get_iam_policy(...)`, `locations_registries_groups_devices_list(...)`, `locations_registries_groups_get_iam_policy(...)`, `locations_registries_groups_set_iam_policy(...)`, `locations_registries_groups_test_iam_permissions(...)`, `locations_registries_list(...)`, `locations_registries_patch(...)`, `locations_registries_set_iam_policy(...)`, `locations_registries_test_iam_permissions(...)` and `locations_registries_unbind_device_from_gateway(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudIot<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the last few versions of the device configuration in descending order (i.e.: newest first).
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the device. For example, `projects/p0/locations/us-central1/registries/registry0/devices/device0` or `projects/p0/locations/us-central1/registries/registry0/devices/{num_id}`.
    pub fn locations_registries_devices_config_versions_list(&self, name: &str) -> ProjectLocationRegistryDeviceConfigVersionListCall<'a, S> {
        ProjectLocationRegistryDeviceConfigVersionListCall {
            hub: self.hub,
            _name: name.to_string(),
            _num_versions: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the last few versions of the device state in descending order (i.e.: newest first).
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the device. For example, `projects/p0/locations/us-central1/registries/registry0/devices/device0` or `projects/p0/locations/us-central1/registries/registry0/devices/{num_id}`.
    pub fn locations_registries_devices_states_list(&self, name: &str) -> ProjectLocationRegistryDeviceStateListCall<'a, S> {
        ProjectLocationRegistryDeviceStateListCall {
            hub: self.hub,
            _name: name.to_string(),
            _num_states: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a device in a device registry.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the device registry where this device should be created. For example, `projects/example-project/locations/us-central1/registries/my-registry`.
    pub fn locations_registries_devices_create(&self, request: Device, parent: &str) -> ProjectLocationRegistryDeviceCreateCall<'a, S> {
        ProjectLocationRegistryDeviceCreateCall {
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
    /// * `name` - Required. The name of the device. For example, `projects/p0/locations/us-central1/registries/registry0/devices/device0` or `projects/p0/locations/us-central1/registries/registry0/devices/{num_id}`.
    pub fn locations_registries_devices_delete(&self, name: &str) -> ProjectLocationRegistryDeviceDeleteCall<'a, S> {
        ProjectLocationRegistryDeviceDeleteCall {
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
    /// * `name` - Required. The name of the device. For example, `projects/p0/locations/us-central1/registries/registry0/devices/device0` or `projects/p0/locations/us-central1/registries/registry0/devices/{num_id}`.
    pub fn locations_registries_devices_get(&self, name: &str) -> ProjectLocationRegistryDeviceGetCall<'a, S> {
        ProjectLocationRegistryDeviceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _field_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List devices in a device registry.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The device registry path. Required. For example, `projects/my-project/locations/us-central1/registries/my-registry`.
    pub fn locations_registries_devices_list(&self, parent: &str) -> ProjectLocationRegistryDeviceListCall<'a, S> {
        ProjectLocationRegistryDeviceListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _gateway_list_options_gateway_type: Default::default(),
            _gateway_list_options_associations_gateway_id: Default::default(),
            _gateway_list_options_associations_device_id: Default::default(),
            _field_mask: Default::default(),
            _device_num_ids: Default::default(),
            _device_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modifies the configuration for the device, which is eventually sent from the Cloud IoT Core servers. Returns the modified configuration version and its metadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the device. For example, `projects/p0/locations/us-central1/registries/registry0/devices/device0` or `projects/p0/locations/us-central1/registries/registry0/devices/{num_id}`.
    pub fn locations_registries_devices_modify_cloud_to_device_config(&self, request: ModifyCloudToDeviceConfigRequest, name: &str) -> ProjectLocationRegistryDeviceModifyCloudToDeviceConfigCall<'a, S> {
        ProjectLocationRegistryDeviceModifyCloudToDeviceConfigCall {
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
    /// * `name` - The resource path name. For example, `projects/p1/locations/us-central1/registries/registry0/devices/dev0` or `projects/p1/locations/us-central1/registries/registry0/devices/{num_id}`. When `name` is populated as a response from the service, it always ends in the device numeric ID.
    pub fn locations_registries_devices_patch(&self, request: Device, name: &str) -> ProjectLocationRegistryDevicePatchCall<'a, S> {
        ProjectLocationRegistryDevicePatchCall {
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
    /// Sends a command to the specified device. In order for a device to be able to receive commands, it must: 1) be connected to Cloud IoT Core using the MQTT protocol, and 2) be subscribed to the group of MQTT topics specified by /devices/{device-id}/commands/#. This subscription will receive commands at the top-level topic /devices/{device-id}/commands as well as commands for subfolders, like /devices/{device-id}/commands/subfolder. Note that subscribing to specific subfolders is not supported. If the command could not be delivered to the device, this method will return an error; in particular, if the device is not subscribed, this method will return FAILED_PRECONDITION. Otherwise, this method will return OK. If the subscription is QoS 1, at least once delivery will be guaranteed; for QoS 0, no acknowledgment will be expected from the device.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the device. For example, `projects/p0/locations/us-central1/registries/registry0/devices/device0` or `projects/p0/locations/us-central1/registries/registry0/devices/{num_id}`.
    pub fn locations_registries_devices_send_command_to_device(&self, request: SendCommandToDeviceRequest, name: &str) -> ProjectLocationRegistryDeviceSendCommandToDeviceCall<'a, S> {
        ProjectLocationRegistryDeviceSendCommandToDeviceCall {
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
    /// List devices in a device registry.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The device registry path. Required. For example, `projects/my-project/locations/us-central1/registries/my-registry`.
    pub fn locations_registries_groups_devices_list(&self, parent: &str) -> ProjectLocationRegistryGroupDeviceListCall<'a, S> {
        ProjectLocationRegistryGroupDeviceListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _gateway_list_options_gateway_type: Default::default(),
            _gateway_list_options_associations_gateway_id: Default::default(),
            _gateway_list_options_associations_device_id: Default::default(),
            _field_mask: Default::default(),
            _device_num_ids: Default::default(),
            _device_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_registries_groups_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectLocationRegistryGroupGetIamPolicyCall<'a, S> {
        ProjectLocationRegistryGroupGetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
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
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_registries_groups_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationRegistryGroupSetIamPolicyCall<'a, S> {
        ProjectLocationRegistryGroupSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_registries_groups_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationRegistryGroupTestIamPermissionCall<'a, S> {
        ProjectLocationRegistryGroupTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Associates the device with the gateway.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the registry. For example, `projects/example-project/locations/us-central1/registries/my-registry`.
    pub fn locations_registries_bind_device_to_gateway(&self, request: BindDeviceToGatewayRequest, parent: &str) -> ProjectLocationRegistryBindDeviceToGatewayCall<'a, S> {
        ProjectLocationRegistryBindDeviceToGatewayCall {
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
    /// Creates a device registry that contains devices.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The project and cloud region where this device registry must be created. For example, `projects/example-project/locations/us-central1`.
    pub fn locations_registries_create(&self, request: DeviceRegistry, parent: &str) -> ProjectLocationRegistryCreateCall<'a, S> {
        ProjectLocationRegistryCreateCall {
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
    /// Deletes a device registry configuration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the device registry. For example, `projects/example-project/locations/us-central1/registries/my-registry`.
    pub fn locations_registries_delete(&self, name: &str) -> ProjectLocationRegistryDeleteCall<'a, S> {
        ProjectLocationRegistryDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a device registry configuration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the device registry. For example, `projects/example-project/locations/us-central1/registries/my-registry`.
    pub fn locations_registries_get(&self, name: &str) -> ProjectLocationRegistryGetCall<'a, S> {
        ProjectLocationRegistryGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_registries_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectLocationRegistryGetIamPolicyCall<'a, S> {
        ProjectLocationRegistryGetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists device registries.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and cloud region path. For example, `projects/example-project/locations/us-central1`.
    pub fn locations_registries_list(&self, parent: &str) -> ProjectLocationRegistryListCall<'a, S> {
        ProjectLocationRegistryListCall {
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
    /// Updates a device registry configuration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource path name. For example, `projects/example-project/locations/us-central1/registries/my-registry`.
    pub fn locations_registries_patch(&self, request: DeviceRegistry, name: &str) -> ProjectLocationRegistryPatchCall<'a, S> {
        ProjectLocationRegistryPatchCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_registries_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationRegistrySetIamPolicyCall<'a, S> {
        ProjectLocationRegistrySetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_registries_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationRegistryTestIamPermissionCall<'a, S> {
        ProjectLocationRegistryTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the association between the device and the gateway.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the registry. For example, `projects/example-project/locations/us-central1/registries/my-registry`.
    pub fn locations_registries_unbind_device_from_gateway(&self, request: UnbindDeviceFromGatewayRequest, parent: &str) -> ProjectLocationRegistryUnbindDeviceFromGatewayCall<'a, S> {
        ProjectLocationRegistryUnbindDeviceFromGatewayCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



