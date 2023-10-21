use super::*;
/// A builder providing access to all methods supported on *accessPolicy* resources.
/// It is not used directly, but through the [`AccessContextManager`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_accesscontextmanager1 as accesscontextmanager1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use accesscontextmanager1::{AccessContextManager, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AccessContextManager::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `access_levels_create(...)`, `access_levels_delete(...)`, `access_levels_get(...)`, `access_levels_list(...)`, `access_levels_patch(...)`, `access_levels_replace_all(...)`, `access_levels_test_iam_permissions(...)`, `authorized_orgs_descs_create(...)`, `authorized_orgs_descs_delete(...)`, `authorized_orgs_descs_get(...)`, `authorized_orgs_descs_list(...)`, `authorized_orgs_descs_patch(...)`, `create(...)`, `delete(...)`, `get(...)`, `get_iam_policy(...)`, `list(...)`, `patch(...)`, `service_perimeters_commit(...)`, `service_perimeters_create(...)`, `service_perimeters_delete(...)`, `service_perimeters_get(...)`, `service_perimeters_list(...)`, `service_perimeters_patch(...)`, `service_perimeters_replace_all(...)`, `service_perimeters_test_iam_permissions(...)`, `set_iam_policy(...)` and `test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.access_policies();
/// # }
/// ```
pub struct AccessPolicyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AccessContextManager<S>,
}

impl<'a, S> client::MethodsBuilder for AccessPolicyMethods<'a, S> {}

impl<'a, S> AccessPolicyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an access level. The long-running operation from this RPC has a successful status after the access level propagates to long-lasting storage. If access levels contain errors, an error response is returned for the first error encountered.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name for the access policy which owns this Access Level. Format: `accessPolicies/{policy_id}`
    pub fn access_levels_create(&self, request: AccessLevel, parent: &str) -> AccessPolicyAccessLevelCreateCall<'a, S> {
        AccessPolicyAccessLevelCreateCall {
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
    /// Deletes an access level based on the resource name. The long-running operation from this RPC has a successful status after the access level has been removed from long-lasting storage.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name for the Access Level. Format: `accessPolicies/{policy_id}/accessLevels/{access_level_id}`
    pub fn access_levels_delete(&self, name: &str) -> AccessPolicyAccessLevelDeleteCall<'a, S> {
        AccessPolicyAccessLevelDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an access level based on the resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name for the Access Level. Format: `accessPolicies/{policy_id}/accessLevels/{access_level_id}`
    pub fn access_levels_get(&self, name: &str) -> AccessPolicyAccessLevelGetCall<'a, S> {
        AccessPolicyAccessLevelGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _access_level_format: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all access levels for an access policy.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name for the access policy to list Access Levels from. Format: `accessPolicies/{policy_id}`
    pub fn access_levels_list(&self, parent: &str) -> AccessPolicyAccessLevelListCall<'a, S> {
        AccessPolicyAccessLevelListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _access_level_format: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an access level. The long-running operation from this RPC has a successful status after the changes to the access level propagate to long-lasting storage. If access levels contain errors, an error response is returned for the first error encountered.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Resource name for the `AccessLevel`. Format: `accessPolicies/{access_policy}/accessLevels/{access_level}`. The `access_level` component must begin with a letter, followed by alphanumeric characters or `_`. Its maximum length is 50 characters. After you create an `AccessLevel`, you cannot change its `name`.
    pub fn access_levels_patch(&self, request: AccessLevel, name: &str) -> AccessPolicyAccessLevelPatchCall<'a, S> {
        AccessPolicyAccessLevelPatchCall {
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
    /// Replaces all existing access levels in an access policy with the access levels provided. This is done atomically. The long-running operation from this RPC has a successful status after all replacements propagate to long-lasting storage. If the replacement contains errors, an error response is returned for the first error encountered. Upon error, the replacement is cancelled, and existing access levels are not affected. The Operation.response field contains ReplaceAccessLevelsResponse. Removing access levels contained in existing service perimeters result in an error.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name for the access policy which owns these Access Levels. Format: `accessPolicies/{policy_id}`
    pub fn access_levels_replace_all(&self, request: ReplaceAccessLevelsRequest, parent: &str) -> AccessPolicyAccessLevelReplaceAllCall<'a, S> {
        AccessPolicyAccessLevelReplaceAllCall {
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
    /// Returns the IAM permissions that the caller has on the specified Access Context Manager resource. The resource can be an AccessPolicy, AccessLevel, or ServicePerimeter. This method does not support other resources.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn access_levels_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> AccessPolicyAccessLevelTestIamPermissionCall<'a, S> {
        AccessPolicyAccessLevelTestIamPermissionCall {
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
    /// Creates a authorized orgs desc. The long-running operation from this RPC has a successful status after the authorized orgs desc propagates to long-lasting storage. If a authorized orgs desc contains errors, an error response is returned for the first error encountered. The name of this `AuthorizedOrgsDesc` will be assigned during creation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name for the access policy which owns this Authorized Orgs Desc. Format: `accessPolicies/{policy_id}`
    pub fn authorized_orgs_descs_create(&self, request: AuthorizedOrgsDesc, parent: &str) -> AccessPolicyAuthorizedOrgsDescCreateCall<'a, S> {
        AccessPolicyAuthorizedOrgsDescCreateCall {
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
    /// Deletes a authorized orgs desc based on the resource name. The long-running operation from this RPC has a successful status after the authorized orgs desc is removed from long-lasting storage.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name for the Authorized Orgs Desc. Format: `accessPolicies/{policy_id}/authorizedOrgsDesc/{authorized_orgs_desc_id}`
    pub fn authorized_orgs_descs_delete(&self, name: &str) -> AccessPolicyAuthorizedOrgsDescDeleteCall<'a, S> {
        AccessPolicyAuthorizedOrgsDescDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a authorized orgs desc based on the resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name for the Authorized Orgs Desc. Format: `accessPolicies/{policy_id}/authorizedOrgsDescs/{authorized_orgs_descs_id}`
    pub fn authorized_orgs_descs_get(&self, name: &str) -> AccessPolicyAuthorizedOrgsDescGetCall<'a, S> {
        AccessPolicyAuthorizedOrgsDescGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all authorized orgs descs for an access policy.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name for the access policy to list Authorized Orgs Desc from. Format: `accessPolicies/{policy_id}`
    pub fn authorized_orgs_descs_list(&self, parent: &str) -> AccessPolicyAuthorizedOrgsDescListCall<'a, S> {
        AccessPolicyAuthorizedOrgsDescListCall {
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
    /// Updates a authorized orgs desc. The long-running operation from this RPC has a successful status after the authorized orgs desc propagates to long-lasting storage. If a authorized orgs desc contains errors, an error response is returned for the first error encountered. Only the organization list in `AuthorizedOrgsDesc` can be updated. The name, authorization_type, asset_type and authorization_direction cannot be updated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Assigned by the server during creation. The last segment has an arbitrary length and has only URI unreserved characters (as defined by [RFC 3986 Section 2.3](https://tools.ietf.org/html/rfc3986#section-2.3)). Should not be specified by the client during creation. Example: "accessPolicies/122256/authorizedOrgs/b3-BhcX_Ud5N"
    pub fn authorized_orgs_descs_patch(&self, request: AuthorizedOrgsDesc, name: &str) -> AccessPolicyAuthorizedOrgsDescPatchCall<'a, S> {
        AccessPolicyAuthorizedOrgsDescPatchCall {
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
    /// Commits the dry-run specification for all the service perimeters in an access policy. A commit operation on a service perimeter involves copying its `spec` field to the `status` field of the service perimeter. Only service perimeters with `use_explicit_dry_run_spec` field set to true are affected by a commit operation. The long-running operation from this RPC has a successful status after the dry-run specifications for all the service perimeters have been committed. If a commit fails, it causes the long-running operation to return an error response and the entire commit operation is cancelled. When successful, the Operation.response field contains CommitServicePerimetersResponse. The `dry_run` and the `spec` fields are cleared after a successful commit operation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name for the parent Access Policy which owns all Service Perimeters in scope for the commit operation. Format: `accessPolicies/{policy_id}`
    pub fn service_perimeters_commit(&self, request: CommitServicePerimetersRequest, parent: &str) -> AccessPolicyServicePerimeterCommitCall<'a, S> {
        AccessPolicyServicePerimeterCommitCall {
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
    /// Creates a service perimeter. The long-running operation from this RPC has a successful status after the service perimeter propagates to long-lasting storage. If a service perimeter contains errors, an error response is returned for the first error encountered.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name for the access policy which owns this Service Perimeter. Format: `accessPolicies/{policy_id}`
    pub fn service_perimeters_create(&self, request: ServicePerimeter, parent: &str) -> AccessPolicyServicePerimeterCreateCall<'a, S> {
        AccessPolicyServicePerimeterCreateCall {
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
    /// Deletes a service perimeter based on the resource name. The long-running operation from this RPC has a successful status after the service perimeter is removed from long-lasting storage.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name for the Service Perimeter. Format: `accessPolicies/{policy_id}/servicePerimeters/{service_perimeter_id}`
    pub fn service_perimeters_delete(&self, name: &str) -> AccessPolicyServicePerimeterDeleteCall<'a, S> {
        AccessPolicyServicePerimeterDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a service perimeter based on the resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name for the Service Perimeter. Format: `accessPolicies/{policy_id}/servicePerimeters/{service_perimeters_id}`
    pub fn service_perimeters_get(&self, name: &str) -> AccessPolicyServicePerimeterGetCall<'a, S> {
        AccessPolicyServicePerimeterGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all service perimeters for an access policy.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name for the access policy to list Service Perimeters from. Format: `accessPolicies/{policy_id}`
    pub fn service_perimeters_list(&self, parent: &str) -> AccessPolicyServicePerimeterListCall<'a, S> {
        AccessPolicyServicePerimeterListCall {
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
    /// Updates a service perimeter. The long-running operation from this RPC has a successful status after the service perimeter propagates to long-lasting storage. If a service perimeter contains errors, an error response is returned for the first error encountered.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Resource name for the `ServicePerimeter`. Format: `accessPolicies/{access_policy}/servicePerimeters/{service_perimeter}`. The `service_perimeter` component must begin with a letter, followed by alphanumeric characters or `_`. After you create a `ServicePerimeter`, you cannot change its `name`.
    pub fn service_perimeters_patch(&self, request: ServicePerimeter, name: &str) -> AccessPolicyServicePerimeterPatchCall<'a, S> {
        AccessPolicyServicePerimeterPatchCall {
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
    /// Replace all existing service perimeters in an access policy with the service perimeters provided. This is done atomically. The long-running operation from this RPC has a successful status after all replacements propagate to long-lasting storage. Replacements containing errors result in an error response for the first error encountered. Upon an error, replacement are cancelled and existing service perimeters are not affected. The Operation.response field contains ReplaceServicePerimetersResponse.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name for the access policy which owns these Service Perimeters. Format: `accessPolicies/{policy_id}`
    pub fn service_perimeters_replace_all(&self, request: ReplaceServicePerimetersRequest, parent: &str) -> AccessPolicyServicePerimeterReplaceAllCall<'a, S> {
        AccessPolicyServicePerimeterReplaceAllCall {
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
    /// Returns the IAM permissions that the caller has on the specified Access Context Manager resource. The resource can be an AccessPolicy, AccessLevel, or ServicePerimeter. This method does not support other resources.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn service_perimeters_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> AccessPolicyServicePerimeterTestIamPermissionCall<'a, S> {
        AccessPolicyServicePerimeterTestIamPermissionCall {
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
    /// Creates an access policy. This method fails if the organization already has an access policy. The long-running operation has a successful status after the access policy propagates to long-lasting storage. Syntactic and basic semantic errors are returned in `metadata` as a BadRequest proto.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: AccessPolicy) -> AccessPolicyCreateCall<'a, S> {
        AccessPolicyCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an access policy based on the resource name. The long-running operation has a successful status after the access policy is removed from long-lasting storage.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name for the access policy to delete. Format `accessPolicies/{policy_id}`
    pub fn delete(&self, name: &str) -> AccessPolicyDeleteCall<'a, S> {
        AccessPolicyDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns an access policy based on the name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name for the access policy to get. Format `accessPolicies/{policy_id}`
    pub fn get(&self, name: &str) -> AccessPolicyGetCall<'a, S> {
        AccessPolicyGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the IAM policy for the specified Access Context Manager access policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> AccessPolicyGetIamPolicyCall<'a, S> {
        AccessPolicyGetIamPolicyCall {
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
    /// Lists all access policies in an organization.
    pub fn list(&self) -> AccessPolicyListCall<'a, S> {
        AccessPolicyListCall {
            hub: self.hub,
            _parent: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an access policy. The long-running operation from this RPC has a successful status after the changes to the access policy propagate to long-lasting storage.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name of the `AccessPolicy`. Format: `accessPolicies/{access_policy}`
    pub fn patch(&self, request: AccessPolicy, name: &str) -> AccessPolicyPatchCall<'a, S> {
        AccessPolicyPatchCall {
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
    /// Sets the IAM policy for the specified Access Context Manager access policy. This method replaces the existing IAM policy on the access policy. The IAM policy controls the set of users who can perform specific operations on the Access Context Manager access policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> AccessPolicySetIamPolicyCall<'a, S> {
        AccessPolicySetIamPolicyCall {
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
    /// Returns the IAM permissions that the caller has on the specified Access Context Manager resource. The resource can be an AccessPolicy, AccessLevel, or ServicePerimeter. This method does not support other resources.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> AccessPolicyTestIamPermissionCall<'a, S> {
        AccessPolicyTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *operation* resources.
/// It is not used directly, but through the [`AccessContextManager`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_accesscontextmanager1 as accesscontextmanager1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use accesscontextmanager1::{AccessContextManager, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AccessContextManager::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `cancel(...)`, `delete(...)`, `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.operations();
/// # }
/// ```
pub struct OperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AccessContextManager<S>,
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
    pub fn cancel(&self, request: CancelOperationRequest, name: &str) -> OperationCancelCall<'a, S> {
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



/// A builder providing access to all methods supported on *organization* resources.
/// It is not used directly, but through the [`AccessContextManager`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_accesscontextmanager1 as accesscontextmanager1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use accesscontextmanager1::{AccessContextManager, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AccessContextManager::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `gcp_user_access_bindings_create(...)`, `gcp_user_access_bindings_delete(...)`, `gcp_user_access_bindings_get(...)`, `gcp_user_access_bindings_list(...)` and `gcp_user_access_bindings_patch(...)`
/// // to build up your call.
/// let rb = hub.organizations();
/// # }
/// ```
pub struct OrganizationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AccessContextManager<S>,
}

impl<'a, S> client::MethodsBuilder for OrganizationMethods<'a, S> {}

impl<'a, S> OrganizationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a GcpUserAccessBinding. If the client specifies a name, the server ignores it. Fails if a resource already exists with the same group_key. Completion of this long-running operation does not necessarily signify that the new binding is deployed onto all affected users, which may take more time.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Example: "organizations/256"
    pub fn gcp_user_access_bindings_create(&self, request: GcpUserAccessBinding, parent: &str) -> OrganizationGcpUserAccessBindingCreateCall<'a, S> {
        OrganizationGcpUserAccessBindingCreateCall {
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
    /// Deletes a GcpUserAccessBinding. Completion of this long-running operation does not necessarily signify that the binding deletion is deployed onto all affected users, which may take more time.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Example: "organizations/256/gcpUserAccessBindings/b3-BhcX_Ud5N"
    pub fn gcp_user_access_bindings_delete(&self, name: &str) -> OrganizationGcpUserAccessBindingDeleteCall<'a, S> {
        OrganizationGcpUserAccessBindingDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the GcpUserAccessBinding with the given name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Example: "organizations/256/gcpUserAccessBindings/b3-BhcX_Ud5N"
    pub fn gcp_user_access_bindings_get(&self, name: &str) -> OrganizationGcpUserAccessBindingGetCall<'a, S> {
        OrganizationGcpUserAccessBindingGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all GcpUserAccessBindings for a Google Cloud organization.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Example: "organizations/256"
    pub fn gcp_user_access_bindings_list(&self, parent: &str) -> OrganizationGcpUserAccessBindingListCall<'a, S> {
        OrganizationGcpUserAccessBindingListCall {
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
    /// Updates a GcpUserAccessBinding. Completion of this long-running operation does not necessarily signify that the changed binding is deployed onto all affected users, which may take more time.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. Assigned by the server during creation. The last segment has an arbitrary length and has only URI unreserved characters (as defined by [RFC 3986 Section 2.3](https://tools.ietf.org/html/rfc3986#section-2.3)). Should not be specified by the client during creation. Example: "organizations/256/gcpUserAccessBindings/b3-BhcX_Ud5N"
    pub fn gcp_user_access_bindings_patch(&self, request: GcpUserAccessBinding, name: &str) -> OrganizationGcpUserAccessBindingPatchCall<'a, S> {
        OrganizationGcpUserAccessBindingPatchCall {
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



