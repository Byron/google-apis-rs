use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`CloudIAP`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_iap1 as iap1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use iap1::{CloudIAP, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudIAP::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `brands_create(...)`, `brands_get(...)`, `brands_identity_aware_proxy_clients_create(...)`, `brands_identity_aware_proxy_clients_delete(...)`, `brands_identity_aware_proxy_clients_get(...)`, `brands_identity_aware_proxy_clients_list(...)`, `brands_identity_aware_proxy_clients_reset_secret(...)`, `brands_list(...)`, `iap_tunnel_locations_dest_groups_create(...)`, `iap_tunnel_locations_dest_groups_delete(...)`, `iap_tunnel_locations_dest_groups_get(...)`, `iap_tunnel_locations_dest_groups_list(...)` and `iap_tunnel_locations_dest_groups_patch(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudIAP<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an Identity Aware Proxy (IAP) OAuth client. The client is owned by IAP. Requires that the brand for the project exists and that it is set for internal-only use.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Path to create the client in. In the following format: projects/{project_number/id}/brands/{brand}. The project must belong to a G Suite account.
    pub fn brands_identity_aware_proxy_clients_create(&self, request: IdentityAwareProxyClient, parent: &str) -> ProjectBrandIdentityAwareProxyClientCreateCall<'a, S> {
        ProjectBrandIdentityAwareProxyClientCreateCall {
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
    /// Deletes an Identity Aware Proxy (IAP) OAuth client. Useful for removing obsolete clients, managing the number of clients in a given project, and cleaning up after tests. Requires that the client is owned by IAP.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the Identity Aware Proxy client to be deleted. In the following format: projects/{project_number/id}/brands/{brand}/identityAwareProxyClients/{client_id}.
    pub fn brands_identity_aware_proxy_clients_delete(&self, name: &str) -> ProjectBrandIdentityAwareProxyClientDeleteCall<'a, S> {
        ProjectBrandIdentityAwareProxyClientDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an Identity Aware Proxy (IAP) OAuth client. Requires that the client is owned by IAP.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the Identity Aware Proxy client to be fetched. In the following format: projects/{project_number/id}/brands/{brand}/identityAwareProxyClients/{client_id}.
    pub fn brands_identity_aware_proxy_clients_get(&self, name: &str) -> ProjectBrandIdentityAwareProxyClientGetCall<'a, S> {
        ProjectBrandIdentityAwareProxyClientGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the existing clients for the brand.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Full brand path. In the following format: projects/{project_number/id}/brands/{brand}.
    pub fn brands_identity_aware_proxy_clients_list(&self, parent: &str) -> ProjectBrandIdentityAwareProxyClientListCall<'a, S> {
        ProjectBrandIdentityAwareProxyClientListCall {
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
    /// Resets an Identity Aware Proxy (IAP) OAuth client secret. Useful if the secret was compromised. Requires that the client is owned by IAP.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the Identity Aware Proxy client to that will have its secret reset. In the following format: projects/{project_number/id}/brands/{brand}/identityAwareProxyClients/{client_id}.
    pub fn brands_identity_aware_proxy_clients_reset_secret(&self, request: ResetIdentityAwareProxyClientSecretRequest, name: &str) -> ProjectBrandIdentityAwareProxyClientResetSecretCall<'a, S> {
        ProjectBrandIdentityAwareProxyClientResetSecretCall {
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
    /// Constructs a new OAuth brand for the project if one does not exist. The created brand is "internal only", meaning that OAuth clients created under it only accept requests from users who belong to the same Google Workspace organization as the project. The brand is created in an un-reviewed status. NOTE: The "internal only" status can be manually changed in the Google Cloud Console. Requires that a brand does not already exist for the project, and that the specified support email is owned by the caller.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. GCP Project number/id under which the brand is to be created. In the following format: projects/{project_number/id}.
    pub fn brands_create(&self, request: Brand, parent: &str) -> ProjectBrandCreateCall<'a, S> {
        ProjectBrandCreateCall {
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
    /// Retrieves the OAuth brand of the project.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the brand to be fetched. In the following format: projects/{project_number/id}/brands/{brand}.
    pub fn brands_get(&self, name: &str) -> ProjectBrandGetCall<'a, S> {
        ProjectBrandGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the existing brands for the project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. GCP Project number/id. In the following format: projects/{project_number/id}.
    pub fn brands_list(&self, parent: &str) -> ProjectBrandListCall<'a, S> {
        ProjectBrandListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new TunnelDestGroup.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Google Cloud Project ID and location. In the following format: `projects/{project_number/id}/iap_tunnel/locations/{location}`.
    pub fn iap_tunnel_locations_dest_groups_create(&self, request: TunnelDestGroup, parent: &str) -> ProjectIapTunnelLocationDestGroupCreateCall<'a, S> {
        ProjectIapTunnelLocationDestGroupCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _tunnel_dest_group_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a TunnelDestGroup.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the TunnelDestGroup to delete. In the following format: `projects/{project_number/id}/iap_tunnel/locations/{location}/destGroups/{dest_group}`.
    pub fn iap_tunnel_locations_dest_groups_delete(&self, name: &str) -> ProjectIapTunnelLocationDestGroupDeleteCall<'a, S> {
        ProjectIapTunnelLocationDestGroupDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an existing TunnelDestGroup.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the TunnelDestGroup to be fetched. In the following format: `projects/{project_number/id}/iap_tunnel/locations/{location}/destGroups/{dest_group}`.
    pub fn iap_tunnel_locations_dest_groups_get(&self, name: &str) -> ProjectIapTunnelLocationDestGroupGetCall<'a, S> {
        ProjectIapTunnelLocationDestGroupGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the existing TunnelDestGroups. To group across all locations, use a `-` as the location ID. For example: `/v1/projects/123/iap_tunnel/locations/-/destGroups`
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Google Cloud Project ID and location. In the following format: `projects/{project_number/id}/iap_tunnel/locations/{location}`. A `-` can be used for the location to group across all locations.
    pub fn iap_tunnel_locations_dest_groups_list(&self, parent: &str) -> ProjectIapTunnelLocationDestGroupListCall<'a, S> {
        ProjectIapTunnelLocationDestGroupListCall {
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
    /// Updates a TunnelDestGroup.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Immutable. Identifier for the TunnelDestGroup. Must be unique within the project and contain only lower case letters (a-z) and dashes (-).
    pub fn iap_tunnel_locations_dest_groups_patch(&self, request: TunnelDestGroup, name: &str) -> ProjectIapTunnelLocationDestGroupPatchCall<'a, S> {
        ProjectIapTunnelLocationDestGroupPatchCall {
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



/// A builder providing access to all free methods, which are not associated with a particular resource.
/// It is not used directly, but through the [`CloudIAP`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_iap1 as iap1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use iap1::{CloudIAP, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudIAP::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_iam_policy(...)`, `get_iap_settings(...)`, `set_iam_policy(...)`, `test_iam_permissions(...)` and `update_iap_settings(...)`
/// // to build up your call.
/// let rb = hub.methods();
/// # }
/// ```
pub struct MethodMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudIAP<S>,
}

impl<'a, S> client::MethodsBuilder for MethodMethods<'a, S> {}

impl<'a, S> MethodMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for an Identity-Aware Proxy protected resource. More information about managing access via IAP can be found at: https://cloud.google.com/iap/docs/managing-access#managing_access_via_the_api
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> MethodGetIamPolicyCall<'a, S> {
        MethodGetIamPolicyCall {
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
    /// Gets the IAP settings on a particular IAP protected resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name for which to retrieve the settings. Authorization: Requires the `getSettings` permission for the associated resource.
    pub fn get_iap_settings(&self, name: &str) -> MethodGetIapSettingCall<'a, S> {
        MethodGetIapSettingCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy for an Identity-Aware Proxy protected resource. Replaces any existing policy. More information about managing access via IAP can be found at: https://cloud.google.com/iap/docs/managing-access#managing_access_via_the_api
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> MethodSetIamPolicyCall<'a, S> {
        MethodSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the Identity-Aware Proxy protected resource. More information about managing access via IAP can be found at: https://cloud.google.com/iap/docs/managing-access#managing_access_via_the_api
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> MethodTestIamPermissionCall<'a, S> {
        MethodTestIamPermissionCall {
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
    /// Updates the IAP settings on a particular IAP protected resource. It replaces all fields unless the `update_mask` is set.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the IAP protected resource.
    pub fn update_iap_settings(&self, request: IapSettings, name: &str) -> MethodUpdateIapSettingCall<'a, S> {
        MethodUpdateIapSettingCall {
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



