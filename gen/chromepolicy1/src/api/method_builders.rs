use super::*;
/// A builder providing access to all methods supported on *customer* resources.
/// It is not used directly, but through the [`ChromePolicy`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_chromepolicy1 as chromepolicy1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use chromepolicy1::{ChromePolicy, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ChromePolicy::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `policies_groups_batch_delete(...)`, `policies_groups_batch_modify(...)`, `policies_groups_list_group_priority_ordering(...)`, `policies_groups_update_group_priority_ordering(...)`, `policies_networks_define_certificate(...)`, `policies_networks_define_network(...)`, `policies_networks_remove_certificate(...)`, `policies_networks_remove_network(...)`, `policies_orgunits_batch_inherit(...)`, `policies_orgunits_batch_modify(...)`, `policies_resolve(...)`, `policy_schemas_get(...)` and `policy_schemas_list(...)`
/// // to build up your call.
/// let rb = hub.customers();
/// # }
/// ```
pub struct CustomerMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ChromePolicy<S>,
}

impl<'a, S> client::MethodsBuilder for CustomerMethods<'a, S> {}

impl<'a, S> CustomerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete multiple policy values that are applied to a specific group. All targets must have the same target format. That is to say that they must point to the same target resource and must have the same keys specified in `additionalTargetKeyNames`, though the values for those keys may be different. On failure the request will return the error details as part of the google.rpc.Status.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `customer` - ID of the Google Workspace account or literal "my_customer" for the customer associated to the request.
    pub fn policies_groups_batch_delete(&self, request: GoogleChromePolicyVersionsV1BatchDeleteGroupPoliciesRequest, customer: &str) -> CustomerPolicyGroupBatchDeleteCall<'a, S> {
        CustomerPolicyGroupBatchDeleteCall {
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
    /// Modify multiple policy values that are applied to a specific group. All targets must have the same target format. That is to say that they must point to the same target resource and must have the same keys specified in `additionalTargetKeyNames`, though the values for those keys may be different. On failure the request will return the error details as part of the google.rpc.Status.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `customer` - ID of the Google Workspace account or literal "my_customer" for the customer associated to the request.
    pub fn policies_groups_batch_modify(&self, request: GoogleChromePolicyVersionsV1BatchModifyGroupPoliciesRequest, customer: &str) -> CustomerPolicyGroupBatchModifyCall<'a, S> {
        CustomerPolicyGroupBatchModifyCall {
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
    /// Retrieve a group priority ordering for an app. The target app must be supplied in `additionalTargetKeyNames` in the PolicyTargetKey. On failure the request will return the error details as part of the google.rpc.Status.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `customer` - Required. ID of the Google Workspace account or literal "my_customer" for the customer associated to the request.
    pub fn policies_groups_list_group_priority_ordering(&self, request: GoogleChromePolicyVersionsV1ListGroupPriorityOrderingRequest, customer: &str) -> CustomerPolicyGroupListGroupPriorityOrderingCall<'a, S> {
        CustomerPolicyGroupListGroupPriorityOrderingCall {
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
    /// Update a group priority ordering for an app. The target app must be supplied in `additionalTargetKeyNames` in the PolicyTargetKey. On failure the request will return the error details as part of the google.rpc.Status.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `customer` - Required. ID of the Google Workspace account or literal "my_customer" for the customer associated to the request.
    pub fn policies_groups_update_group_priority_ordering(&self, request: GoogleChromePolicyVersionsV1UpdateGroupPriorityOrderingRequest, customer: &str) -> CustomerPolicyGroupUpdateGroupPriorityOrderingCall<'a, S> {
        CustomerPolicyGroupUpdateGroupPriorityOrderingCall {
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
    /// Creates a certificate at a specified OU for a customer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `customer` - Required. The customer for which the certificate will apply.
    pub fn policies_networks_define_certificate(&self, request: GoogleChromePolicyVersionsV1DefineCertificateRequest, customer: &str) -> CustomerPolicyNetworkDefineCertificateCall<'a, S> {
        CustomerPolicyNetworkDefineCertificateCall {
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
    /// Define a new network.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `customer` - Required. The customer who will own this new network.
    pub fn policies_networks_define_network(&self, request: GoogleChromePolicyVersionsV1DefineNetworkRequest, customer: &str) -> CustomerPolicyNetworkDefineNetworkCall<'a, S> {
        CustomerPolicyNetworkDefineNetworkCall {
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
    /// Remove an existing certificate by guid.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `customer` - Required. The customer whose certificate will be removed.
    pub fn policies_networks_remove_certificate(&self, request: GoogleChromePolicyVersionsV1RemoveCertificateRequest, customer: &str) -> CustomerPolicyNetworkRemoveCertificateCall<'a, S> {
        CustomerPolicyNetworkRemoveCertificateCall {
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
    /// Remove an existing network by guid.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `customer` - Required. The customer whose network will be removed.
    pub fn policies_networks_remove_network(&self, request: GoogleChromePolicyVersionsV1RemoveNetworkRequest, customer: &str) -> CustomerPolicyNetworkRemoveNetworkCall<'a, S> {
        CustomerPolicyNetworkRemoveNetworkCall {
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
    /// Modify multiple policy values that are applied to a specific org unit so that they now inherit the value from a parent (if applicable). All targets must have the same target format. That is to say that they must point to the same target resource and must have the same keys specified in `additionalTargetKeyNames`, though the values for those keys may be different. On failure the request will return the error details as part of the google.rpc.Status.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `customer` - ID of the G Suite account or literal "my_customer" for the customer associated to the request.
    pub fn policies_orgunits_batch_inherit(&self, request: GoogleChromePolicyVersionsV1BatchInheritOrgUnitPoliciesRequest, customer: &str) -> CustomerPolicyOrgunitBatchInheritCall<'a, S> {
        CustomerPolicyOrgunitBatchInheritCall {
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
    /// Modify multiple policy values that are applied to a specific org unit. All targets must have the same target format. That is to say that they must point to the same target resource and must have the same keys specified in `additionalTargetKeyNames`, though the values for those keys may be different. On failure the request will return the error details as part of the google.rpc.Status.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `customer` - ID of the G Suite account or literal "my_customer" for the customer associated to the request.
    pub fn policies_orgunits_batch_modify(&self, request: GoogleChromePolicyVersionsV1BatchModifyOrgUnitPoliciesRequest, customer: &str) -> CustomerPolicyOrgunitBatchModifyCall<'a, S> {
        CustomerPolicyOrgunitBatchModifyCall {
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
    /// Gets the resolved policy values for a list of policies that match a search query.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `customer` - ID of the G Suite account or literal "my_customer" for the customer associated to the request.
    pub fn policies_resolve(&self, request: GoogleChromePolicyVersionsV1ResolveRequest, customer: &str) -> CustomerPolicyResolveCall<'a, S> {
        CustomerPolicyResolveCall {
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
    /// Get a specific policy schema for a customer by its resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The policy schema resource name to query.
    pub fn policy_schemas_get(&self, name: &str) -> CustomerPolicySchemaGetCall<'a, S> {
        CustomerPolicySchemaGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a list of policy schemas that match a specified filter value for a given customer.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The customer for which the listing request will apply.
    pub fn policy_schemas_list(&self, parent: &str) -> CustomerPolicySchemaListCall<'a, S> {
        CustomerPolicySchemaListCall {
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
}



/// A builder providing access to all methods supported on *media* resources.
/// It is not used directly, but through the [`ChromePolicy`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_chromepolicy1 as chromepolicy1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use chromepolicy1::{ChromePolicy, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ChromePolicy::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `upload(...)`
/// // to build up your call.
/// let rb = hub.media();
/// # }
/// ```
pub struct MediaMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ChromePolicy<S>,
}

impl<'a, S> client::MethodsBuilder for MediaMethods<'a, S> {}

impl<'a, S> MediaMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an enterprise file from the content provided by user. Returns a public download url for end user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `customer` - Required. The customer for which the file upload will apply.
    pub fn upload(&self, request: GoogleChromePolicyVersionsV1UploadPolicyFileRequest, customer: &str) -> MediaUploadCall<'a, S> {
        MediaUploadCall {
            hub: self.hub,
            _request: request,
            _customer: customer.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



