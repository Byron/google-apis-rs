use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`ManagedServiceForMicrosoftActiveDirectoryConsumerAPI`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_managedidentities1 as managedidentities1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use managedidentities1::{ManagedServiceForMicrosoftActiveDirectoryConsumerAPI, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ManagedServiceForMicrosoftActiveDirectoryConsumerAPI::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_get(...)`, `locations_global_domains_attach_trust(...)`, `locations_global_domains_backups_create(...)`, `locations_global_domains_backups_delete(...)`, `locations_global_domains_backups_get(...)`, `locations_global_domains_backups_get_iam_policy(...)`, `locations_global_domains_backups_list(...)`, `locations_global_domains_backups_patch(...)`, `locations_global_domains_backups_set_iam_policy(...)`, `locations_global_domains_backups_test_iam_permissions(...)`, `locations_global_domains_create(...)`, `locations_global_domains_delete(...)`, `locations_global_domains_detach_trust(...)`, `locations_global_domains_extend_schema(...)`, `locations_global_domains_get(...)`, `locations_global_domains_get_iam_policy(...)`, `locations_global_domains_get_ldapssettings(...)`, `locations_global_domains_list(...)`, `locations_global_domains_patch(...)`, `locations_global_domains_reconfigure_trust(...)`, `locations_global_domains_reset_admin_password(...)`, `locations_global_domains_restore(...)`, `locations_global_domains_set_iam_policy(...)`, `locations_global_domains_sql_integrations_get(...)`, `locations_global_domains_sql_integrations_list(...)`, `locations_global_domains_test_iam_permissions(...)`, `locations_global_domains_update_ldapssettings(...)`, `locations_global_domains_validate_trust(...)`, `locations_global_operations_cancel(...)`, `locations_global_operations_delete(...)`, `locations_global_operations_get(...)`, `locations_global_operations_list(...)`, `locations_global_peerings_create(...)`, `locations_global_peerings_delete(...)`, `locations_global_peerings_get(...)`, `locations_global_peerings_get_iam_policy(...)`, `locations_global_peerings_list(...)`, `locations_global_peerings_patch(...)`, `locations_global_peerings_set_iam_policy(...)`, `locations_global_peerings_test_iam_permissions(...)` and `locations_list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ManagedServiceForMicrosoftActiveDirectoryConsumerAPI<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a Backup for a domain.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The domain resource name using the form: `projects/{project_id}/locations/global/domains/{domain_name}`
    pub fn locations_global_domains_backups_create(&self, request: Backup, parent: &str) -> ProjectLocationGlobalDomainBackupCreateCall<'a, S> {
        ProjectLocationGlobalDomainBackupCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _backup_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes identified Backup.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The backup resource name using the form: `projects/{project_id}/locations/global/domains/{domain_name}/backups/{backup_id}`
    pub fn locations_global_domains_backups_delete(&self, name: &str) -> ProjectLocationGlobalDomainBackupDeleteCall<'a, S> {
        ProjectLocationGlobalDomainBackupDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single Backup.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The backup resource name using the form: `projects/{project_id}/locations/global/domains/{domain_name}/backups/{backup_id}`
    pub fn locations_global_domains_backups_get(&self, name: &str) -> ProjectLocationGlobalDomainBackupGetCall<'a, S> {
        ProjectLocationGlobalDomainBackupGetCall {
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
    pub fn locations_global_domains_backups_get_iam_policy(&self, resource: &str) -> ProjectLocationGlobalDomainBackupGetIamPolicyCall<'a, S> {
        ProjectLocationGlobalDomainBackupGetIamPolicyCall {
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
    /// Lists Backup in a given project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The domain resource name using the form: `projects/{project_id}/locations/global/domains/{domain_name}`
    pub fn locations_global_domains_backups_list(&self, parent: &str) -> ProjectLocationGlobalDomainBackupListCall<'a, S> {
        ProjectLocationGlobalDomainBackupListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the labels for specified Backup.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The unique name of the Backup in the form of `projects/{project_id}/locations/global/domains/{domain_name}/backups/{name}`
    pub fn locations_global_domains_backups_patch(&self, request: Backup, name: &str) -> ProjectLocationGlobalDomainBackupPatchCall<'a, S> {
        ProjectLocationGlobalDomainBackupPatchCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_global_domains_backups_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationGlobalDomainBackupSetIamPolicyCall<'a, S> {
        ProjectLocationGlobalDomainBackupSetIamPolicyCall {
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
    pub fn locations_global_domains_backups_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationGlobalDomainBackupTestIamPermissionCall<'a, S> {
        ProjectLocationGlobalDomainBackupTestIamPermissionCall {
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
    /// Gets details of a single sqlIntegration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. SQLIntegration resource name using the form: `projects/{project_id}/locations/global/domains/{domain}/sqlIntegrations/{name}`
    pub fn locations_global_domains_sql_integrations_get(&self, name: &str) -> ProjectLocationGlobalDomainSqlIntegrationGetCall<'a, S> {
        ProjectLocationGlobalDomainSqlIntegrationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists SqlIntegrations in a given domain.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the SqlIntegrations using the form: `projects/{project_id}/locations/global/domains/*`
    pub fn locations_global_domains_sql_integrations_list(&self, parent: &str) -> ProjectLocationGlobalDomainSqlIntegrationListCall<'a, S> {
        ProjectLocationGlobalDomainSqlIntegrationListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds an AD trust to a domain.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource domain name, project name and location using the form: `projects/{project_id}/locations/global/domains/{domain_name}`
    pub fn locations_global_domains_attach_trust(&self, request: AttachTrustRequest, name: &str) -> ProjectLocationGlobalDomainAttachTrustCall<'a, S> {
        ProjectLocationGlobalDomainAttachTrustCall {
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
    /// Creates a Microsoft AD domain.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource project name and location using the form: `projects/{project_id}/locations/global`
    pub fn locations_global_domains_create(&self, request: Domain, parent: &str) -> ProjectLocationGlobalDomainCreateCall<'a, S> {
        ProjectLocationGlobalDomainCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _domain_name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a domain.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The domain resource name using the form: `projects/{project_id}/locations/global/domains/{domain_name}`
    pub fn locations_global_domains_delete(&self, name: &str) -> ProjectLocationGlobalDomainDeleteCall<'a, S> {
        ProjectLocationGlobalDomainDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes an AD trust.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource domain name, project name, and location using the form: `projects/{project_id}/locations/global/domains/{domain_name}`
    pub fn locations_global_domains_detach_trust(&self, request: DetachTrustRequest, name: &str) -> ProjectLocationGlobalDomainDetachTrustCall<'a, S> {
        ProjectLocationGlobalDomainDetachTrustCall {
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
    /// Extend Schema for Domain
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `domain` - Required. The domain resource name using the form: `projects/{project_id}/locations/global/domains/{domain_name}`
    pub fn locations_global_domains_extend_schema(&self, request: ExtendSchemaRequest, domain: &str) -> ProjectLocationGlobalDomainExtendSchemaCall<'a, S> {
        ProjectLocationGlobalDomainExtendSchemaCall {
            hub: self.hub,
            _request: request,
            _domain: domain.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about a domain.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The domain resource name using the form: `projects/{project_id}/locations/global/domains/{domain_name}`
    pub fn locations_global_domains_get(&self, name: &str) -> ProjectLocationGlobalDomainGetCall<'a, S> {
        ProjectLocationGlobalDomainGetCall {
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
    pub fn locations_global_domains_get_iam_policy(&self, resource: &str) -> ProjectLocationGlobalDomainGetIamPolicyCall<'a, S> {
        ProjectLocationGlobalDomainGetIamPolicyCall {
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
    /// Gets the domain ldaps settings.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The domain resource name using the form: `projects/{project_id}/locations/global/domains/{domain_name}`
    pub fn locations_global_domains_get_ldapssettings(&self, name: &str) -> ProjectLocationGlobalDomainGetLdapssettingCall<'a, S> {
        ProjectLocationGlobalDomainGetLdapssettingCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists domains in a project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the domain location using the form: `projects/{project_id}/locations/global`
    pub fn locations_global_domains_list(&self, parent: &str) -> ProjectLocationGlobalDomainListCall<'a, S> {
        ProjectLocationGlobalDomainListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the metadata and configuration of a domain.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The unique name of the domain using the form: `projects/{project_id}/locations/global/domains/{domain_name}`.
    pub fn locations_global_domains_patch(&self, request: Domain, name: &str) -> ProjectLocationGlobalDomainPatchCall<'a, S> {
        ProjectLocationGlobalDomainPatchCall {
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
    /// Updates the DNS conditional forwarder.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource domain name, project name and location using the form: `projects/{project_id}/locations/global/domains/{domain_name}`
    pub fn locations_global_domains_reconfigure_trust(&self, request: ReconfigureTrustRequest, name: &str) -> ProjectLocationGlobalDomainReconfigureTrustCall<'a, S> {
        ProjectLocationGlobalDomainReconfigureTrustCall {
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
    /// Resets a domain's administrator password.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The domain resource name using the form: `projects/{project_id}/locations/global/domains/{domain_name}`
    pub fn locations_global_domains_reset_admin_password(&self, request: ResetAdminPasswordRequest, name: &str) -> ProjectLocationGlobalDomainResetAdminPasswordCall<'a, S> {
        ProjectLocationGlobalDomainResetAdminPasswordCall {
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
    /// RestoreDomain restores domain backup mentioned in the RestoreDomainRequest
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name for the domain to which the backup belongs
    pub fn locations_global_domains_restore(&self, request: RestoreDomainRequest, name: &str) -> ProjectLocationGlobalDomainRestoreCall<'a, S> {
        ProjectLocationGlobalDomainRestoreCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_global_domains_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationGlobalDomainSetIamPolicyCall<'a, S> {
        ProjectLocationGlobalDomainSetIamPolicyCall {
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
    pub fn locations_global_domains_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationGlobalDomainTestIamPermissionCall<'a, S> {
        ProjectLocationGlobalDomainTestIamPermissionCall {
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
    /// Patches a single ldaps settings.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the LDAPS settings. Uses the form: `projects/{project}/locations/{location}/domains/{domain}`.
    pub fn locations_global_domains_update_ldapssettings(&self, request: LDAPSSettings, name: &str) -> ProjectLocationGlobalDomainUpdateLdapssettingCall<'a, S> {
        ProjectLocationGlobalDomainUpdateLdapssettingCall {
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
    /// Validates a trust state, that the target domain is reachable, and that the target domain is able to accept incoming trust requests.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource domain name, project name, and location using the form: `projects/{project_id}/locations/global/domains/{domain_name}`
    pub fn locations_global_domains_validate_trust(&self, request: ValidateTrustRequest, name: &str) -> ProjectLocationGlobalDomainValidateTrustCall<'a, S> {
        ProjectLocationGlobalDomainValidateTrustCall {
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
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn locations_global_operations_cancel(&self, request: CancelOperationRequest, name: &str) -> ProjectLocationGlobalOperationCancelCall<'a, S> {
        ProjectLocationGlobalOperationCancelCall {
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
    pub fn locations_global_operations_delete(&self, name: &str) -> ProjectLocationGlobalOperationDeleteCall<'a, S> {
        ProjectLocationGlobalOperationDeleteCall {
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
    pub fn locations_global_operations_get(&self, name: &str) -> ProjectLocationGlobalOperationGetCall<'a, S> {
        ProjectLocationGlobalOperationGetCall {
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
    pub fn locations_global_operations_list(&self, name: &str) -> ProjectLocationGlobalOperationListCall<'a, S> {
        ProjectLocationGlobalOperationListCall {
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
    /// Creates a Peering for Managed AD instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource project name and location using the form: `projects/{project_id}/locations/global`
    pub fn locations_global_peerings_create(&self, request: Peering, parent: &str) -> ProjectLocationGlobalPeeringCreateCall<'a, S> {
        ProjectLocationGlobalPeeringCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _peering_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes identified Peering.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Peering resource name using the form: `projects/{project_id}/locations/global/peerings/{peering_id}`
    pub fn locations_global_peerings_delete(&self, name: &str) -> ProjectLocationGlobalPeeringDeleteCall<'a, S> {
        ProjectLocationGlobalPeeringDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single Peering.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Peering resource name using the form: `projects/{project_id}/locations/global/peerings/{peering_id}`
    pub fn locations_global_peerings_get(&self, name: &str) -> ProjectLocationGlobalPeeringGetCall<'a, S> {
        ProjectLocationGlobalPeeringGetCall {
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
    pub fn locations_global_peerings_get_iam_policy(&self, resource: &str) -> ProjectLocationGlobalPeeringGetIamPolicyCall<'a, S> {
        ProjectLocationGlobalPeeringGetIamPolicyCall {
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
    /// Lists Peerings in a given project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the peering location using the form: `projects/{project_id}/locations/global`
    pub fn locations_global_peerings_list(&self, parent: &str) -> ProjectLocationGlobalPeeringListCall<'a, S> {
        ProjectLocationGlobalPeeringListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the labels for specified Peering.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Unique name of the peering in this scope including projects and location using the form: `projects/{project_id}/locations/global/peerings/{peering_id}`.
    pub fn locations_global_peerings_patch(&self, request: Peering, name: &str) -> ProjectLocationGlobalPeeringPatchCall<'a, S> {
        ProjectLocationGlobalPeeringPatchCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_global_peerings_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationGlobalPeeringSetIamPolicyCall<'a, S> {
        ProjectLocationGlobalPeeringSetIamPolicyCall {
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
    pub fn locations_global_peerings_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationGlobalPeeringTestIamPermissionCall<'a, S> {
        ProjectLocationGlobalPeeringTestIamPermissionCall {
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



