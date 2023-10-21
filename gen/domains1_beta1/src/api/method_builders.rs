use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`CloudDomains`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_domains1_beta1 as domains1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use domains1_beta1::{CloudDomains, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudDomains::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_get(...)`, `locations_list(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `locations_registrations_configure_contact_settings(...)`, `locations_registrations_configure_dns_settings(...)`, `locations_registrations_configure_management_settings(...)`, `locations_registrations_delete(...)`, `locations_registrations_export(...)`, `locations_registrations_get(...)`, `locations_registrations_get_iam_policy(...)`, `locations_registrations_import(...)`, `locations_registrations_list(...)`, `locations_registrations_patch(...)`, `locations_registrations_register(...)`, `locations_registrations_reset_authorization_code(...)`, `locations_registrations_retrieve_authorization_code(...)`, `locations_registrations_retrieve_importable_domains(...)`, `locations_registrations_retrieve_register_parameters(...)`, `locations_registrations_retrieve_transfer_parameters(...)`, `locations_registrations_search_domains(...)`, `locations_registrations_set_iam_policy(...)`, `locations_registrations_test_iam_permissions(...)` and `locations_registrations_transfer(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudDomains<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn locations_operations_get(&self, name: &str) -> ProjectLocationOperationGetCall<'a, S> {
        ProjectLocationOperationGetCall {
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
    pub fn locations_operations_list(&self, name: &str) -> ProjectLocationOperationListCall<'a, S> {
        ProjectLocationOperationListCall {
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
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a `Registration`'s contact settings. Some changes require confirmation by the domain's registrant contact .
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `registration` - Required. The name of the `Registration` whose contact settings are being updated, in the format `projects/*/locations/*/registrations/*`.
    pub fn locations_registrations_configure_contact_settings(&self, request: ConfigureContactSettingsRequest, registration: &str) -> ProjectLocationRegistrationConfigureContactSettingCall<'a, S> {
        ProjectLocationRegistrationConfigureContactSettingCall {
            hub: self.hub,
            _request: request,
            _registration: registration.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a `Registration`'s DNS settings.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `registration` - Required. The name of the `Registration` whose DNS settings are being updated, in the format `projects/*/locations/*/registrations/*`.
    pub fn locations_registrations_configure_dns_settings(&self, request: ConfigureDnsSettingsRequest, registration: &str) -> ProjectLocationRegistrationConfigureDnsSettingCall<'a, S> {
        ProjectLocationRegistrationConfigureDnsSettingCall {
            hub: self.hub,
            _request: request,
            _registration: registration.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a `Registration`'s management settings.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `registration` - Required. The name of the `Registration` whose management settings are being updated, in the format `projects/*/locations/*/registrations/*`.
    pub fn locations_registrations_configure_management_settings(&self, request: ConfigureManagementSettingsRequest, registration: &str) -> ProjectLocationRegistrationConfigureManagementSettingCall<'a, S> {
        ProjectLocationRegistrationConfigureManagementSettingCall {
            hub: self.hub,
            _request: request,
            _registration: registration.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a `Registration` resource. This method works on any `Registration` resource using [Subscription or Commitment billing](https://cloud.google.com/domains/pricing#billing-models), provided that the resource was created at least 1 day in the past. For `Registration` resources using [Monthly billing](https://cloud.google.com/domains/pricing#billing-models), this method works if: * `state` is `EXPORTED` with `expire_time` in the past * `state` is `REGISTRATION_FAILED` * `state` is `TRANSFER_FAILED` When an active registration is successfully deleted, you can continue to use the domain in [Google Domains](https://domains.google/) until it expires. The calling user becomes the domain’s sole owner in Google Domains, and permissions for the domain are subsequently managed there. The domain does not renew automatically unless the new owner sets up billing in Google Domains.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the `Registration` to delete, in the format `projects/*/locations/*/registrations/*`.
    pub fn locations_registrations_delete(&self, name: &str) -> ProjectLocationRegistrationDeleteCall<'a, S> {
        ProjectLocationRegistrationDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Exports a `Registration` resource, such that it is no longer managed by Cloud Domains. When an active domain is successfully exported, you can continue to use the domain in [Google Domains](https://domains.google/) until it expires. The calling user becomes the domain's sole owner in Google Domains, and permissions for the domain are subsequently managed there. The domain does not renew automatically unless the new owner sets up billing in Google Domains.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the `Registration` to export, in the format `projects/*/locations/*/registrations/*`.
    pub fn locations_registrations_export(&self, request: ExportRegistrationRequest, name: &str) -> ProjectLocationRegistrationExportCall<'a, S> {
        ProjectLocationRegistrationExportCall {
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
    /// Gets the details of a `Registration` resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the `Registration` to get, in the format `projects/*/locations/*/registrations/*`.
    pub fn locations_registrations_get(&self, name: &str) -> ProjectLocationRegistrationGetCall<'a, S> {
        ProjectLocationRegistrationGetCall {
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
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_registrations_get_iam_policy(&self, resource: &str) -> ProjectLocationRegistrationGetIamPolicyCall<'a, S> {
        ProjectLocationRegistrationGetIamPolicyCall {
            hub: self.hub,
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Imports a domain name from [Google Domains](https://domains.google/) for use in Cloud Domains. To transfer a domain from another registrar, use the `TransferDomain` method instead. Since individual users can own domains in Google Domains, the calling user must have ownership permission on the domain.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the Registration. Must be in the format `projects/*/locations/*`.
    pub fn locations_registrations_import(&self, request: ImportDomainRequest, parent: &str) -> ProjectLocationRegistrationImportCall<'a, S> {
        ProjectLocationRegistrationImportCall {
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
    /// Lists the `Registration` resources in a project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and location from which to list `Registration`s, specified in the format `projects/*/locations/*`.
    pub fn locations_registrations_list(&self, parent: &str) -> ProjectLocationRegistrationListCall<'a, S> {
        ProjectLocationRegistrationListCall {
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
    /// Updates select fields of a `Registration` resource, notably `labels`. To update other fields, use the appropriate custom update method: * To update management settings, see `ConfigureManagementSettings` * To update DNS configuration, see `ConfigureDnsSettings` * To update contact information, see `ConfigureContactSettings`
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Name of the `Registration` resource, in the format `projects/*/locations/*/registrations/`.
    pub fn locations_registrations_patch(&self, request: Registration, name: &str) -> ProjectLocationRegistrationPatchCall<'a, S> {
        ProjectLocationRegistrationPatchCall {
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
    /// Registers a new domain name and creates a corresponding `Registration` resource. Call `RetrieveRegisterParameters` first to check availability of the domain name and determine parameters like price that are needed to build a call to this method. A successful call creates a `Registration` resource in state `REGISTRATION_PENDING`, which resolves to `ACTIVE` within 1-2 minutes, indicating that the domain was successfully registered. If the resource ends up in state `REGISTRATION_FAILED`, it indicates that the domain was not registered successfully, and you can safely delete the resource and retry registration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the `Registration`. Must be in the format `projects/*/locations/*`.
    pub fn locations_registrations_register(&self, request: RegisterDomainRequest, parent: &str) -> ProjectLocationRegistrationRegisterCall<'a, S> {
        ProjectLocationRegistrationRegisterCall {
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
    /// Resets the authorization code of the `Registration` to a new random string. You can call this method only after 60 days have elapsed since the initial domain registration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `registration` - Required. The name of the `Registration` whose authorization code is being reset, in the format `projects/*/locations/*/registrations/*`.
    pub fn locations_registrations_reset_authorization_code(&self, request: ResetAuthorizationCodeRequest, registration: &str) -> ProjectLocationRegistrationResetAuthorizationCodeCall<'a, S> {
        ProjectLocationRegistrationResetAuthorizationCodeCall {
            hub: self.hub,
            _request: request,
            _registration: registration.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the authorization code of the `Registration` for the purpose of transferring the domain to another registrar. You can call this method only after 60 days have elapsed since the initial domain registration.
    /// 
    /// # Arguments
    ///
    /// * `registration` - Required. The name of the `Registration` whose authorization code is being retrieved, in the format `projects/*/locations/*/registrations/*`.
    pub fn locations_registrations_retrieve_authorization_code(&self, registration: &str) -> ProjectLocationRegistrationRetrieveAuthorizationCodeCall<'a, S> {
        ProjectLocationRegistrationRetrieveAuthorizationCodeCall {
            hub: self.hub,
            _registration: registration.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists domain names from [Google Domains](https://domains.google/) that can be imported to Cloud Domains using the `ImportDomain` method. Since individual users can own domains in Google Domains, the list of domains returned depends on the individual user making the call. Domains already managed by Cloud Domains are not returned.
    /// 
    /// # Arguments
    ///
    /// * `location` - Required. The location. Must be in the format `projects/*/locations/*`.
    pub fn locations_registrations_retrieve_importable_domains(&self, location: &str) -> ProjectLocationRegistrationRetrieveImportableDomainCall<'a, S> {
        ProjectLocationRegistrationRetrieveImportableDomainCall {
            hub: self.hub,
            _location: location.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets parameters needed to register a new domain name, including price and up-to-date availability. Use the returned values to call `RegisterDomain`.
    /// 
    /// # Arguments
    ///
    /// * `location` - Required. The location. Must be in the format `projects/*/locations/*`.
    pub fn locations_registrations_retrieve_register_parameters(&self, location: &str) -> ProjectLocationRegistrationRetrieveRegisterParameterCall<'a, S> {
        ProjectLocationRegistrationRetrieveRegisterParameterCall {
            hub: self.hub,
            _location: location.to_string(),
            _domain_name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets parameters needed to transfer a domain name from another registrar to Cloud Domains. For domains already managed by [Google Domains](https://domains.google/), use `ImportDomain` instead. Use the returned values to call `TransferDomain`.
    /// 
    /// # Arguments
    ///
    /// * `location` - Required. The location. Must be in the format `projects/*/locations/*`.
    pub fn locations_registrations_retrieve_transfer_parameters(&self, location: &str) -> ProjectLocationRegistrationRetrieveTransferParameterCall<'a, S> {
        ProjectLocationRegistrationRetrieveTransferParameterCall {
            hub: self.hub,
            _location: location.to_string(),
            _domain_name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Searches for available domain names similar to the provided query. Availability results from this method are approximate; call `RetrieveRegisterParameters` on a domain before registering to confirm availability.
    /// 
    /// # Arguments
    ///
    /// * `location` - Required. The location. Must be in the format `projects/*/locations/*`.
    pub fn locations_registrations_search_domains(&self, location: &str) -> ProjectLocationRegistrationSearchDomainCall<'a, S> {
        ProjectLocationRegistrationSearchDomainCall {
            hub: self.hub,
            _location: location.to_string(),
            _query: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_registrations_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationRegistrationSetIamPolicyCall<'a, S> {
        ProjectLocationRegistrationSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_registrations_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationRegistrationTestIamPermissionCall<'a, S> {
        ProjectLocationRegistrationTestIamPermissionCall {
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
    /// Transfers a domain name from another registrar to Cloud Domains. For domains already managed by [Google Domains](https://domains.google/), use `ImportDomain` instead. Before calling this method, go to the domain's current registrar to unlock the domain for transfer and retrieve the domain's transfer authorization code. Then call `RetrieveTransferParameters` to confirm that the domain is unlocked and to get values needed to build a call to this method. A successful call creates a `Registration` resource in state `TRANSFER_PENDING`. It can take several days to complete the transfer process. The registrant can often speed up this process by approving the transfer through the current registrar, either by clicking a link in an email from the registrar or by visiting the registrar's website. A few minutes after transfer approval, the resource transitions to state `ACTIVE`, indicating that the transfer was successful. If the transfer is rejected or the request expires without being approved, the resource can end up in state `TRANSFER_FAILED`. If transfer fails, you can safely delete the resource and retry the transfer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the `Registration`. Must be in the format `projects/*/locations/*`.
    pub fn locations_registrations_transfer(&self, request: TransferDomainRequest, parent: &str) -> ProjectLocationRegistrationTransferCall<'a, S> {
        ProjectLocationRegistrationTransferCall {
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
    /// Gets information about a location.
    /// 
    /// # Arguments
    ///
    /// * `name` - Resource name for the location.
    pub fn locations_get(&self, name: &str) -> ProjectLocationGetCall<'a, S> {
        ProjectLocationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists information about the supported locations for this service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource that owns the locations collection, if applicable.
    pub fn locations_list(&self, name: &str) -> ProjectLocationListCall<'a, S> {
        ProjectLocationListCall {
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



