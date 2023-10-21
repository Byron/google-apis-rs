use super::*;
/// A builder providing access to all methods supported on *enterprise* resources.
/// It is not used directly, but through the [`AndroidManagement`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidmanagement1 as androidmanagement1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidmanagement1::{AndroidManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `applications_get(...)`, `create(...)`, `delete(...)`, `devices_delete(...)`, `devices_get(...)`, `devices_issue_command(...)`, `devices_list(...)`, `devices_operations_cancel(...)`, `devices_operations_delete(...)`, `devices_operations_get(...)`, `devices_operations_list(...)`, `devices_patch(...)`, `enrollment_tokens_create(...)`, `enrollment_tokens_delete(...)`, `enrollment_tokens_get(...)`, `enrollment_tokens_list(...)`, `get(...)`, `list(...)`, `patch(...)`, `policies_delete(...)`, `policies_get(...)`, `policies_list(...)`, `policies_patch(...)`, `web_apps_create(...)`, `web_apps_delete(...)`, `web_apps_get(...)`, `web_apps_list(...)`, `web_apps_patch(...)` and `web_tokens_create(...)`
/// // to build up your call.
/// let rb = hub.enterprises();
/// # }
/// ```
pub struct EnterpriseMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidManagement<S>,
}

impl<'a, S> client::MethodsBuilder for EnterpriseMethods<'a, S> {}

impl<'a, S> EnterpriseMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets info about an application.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the application in the form enterprises/{enterpriseId}/applications/{package_name}.
    pub fn applications_get(&self, name: &str) -> EnterpriseApplicationGetCall<'a, S> {
        EnterpriseApplicationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _language_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn devices_operations_cancel(&self, name: &str) -> EnterpriseDeviceOperationCancelCall<'a, S> {
        EnterpriseDeviceOperationCancelCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be deleted.
    pub fn devices_operations_delete(&self, name: &str) -> EnterpriseDeviceOperationDeleteCall<'a, S> {
        EnterpriseDeviceOperationDeleteCall {
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
    pub fn devices_operations_get(&self, name: &str) -> EnterpriseDeviceOperationGetCall<'a, S> {
        EnterpriseDeviceOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns UNIMPLEMENTED.NOTE: the name binding allows API services to override the binding to use different resource name schemes, such as users/*/operations. To override the binding, API services can add a binding such as "/v1/{name=users/*}/operations" to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn devices_operations_list(&self, name: &str) -> EnterpriseDeviceOperationListCall<'a, S> {
        EnterpriseDeviceOperationListCall {
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
    /// Deletes a device. This operation wipes the device. Deleted devices do not show up in enterprises.devices.list calls and a 404 is returned from enterprises.devices.get.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the device in the form enterprises/{enterpriseId}/devices/{deviceId}.
    pub fn devices_delete(&self, name: &str) -> EnterpriseDeviceDeleteCall<'a, S> {
        EnterpriseDeviceDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _wipe_reason_message: Default::default(),
            _wipe_data_flags: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a device. Deleted devices will respond with a 404 error.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the device in the form enterprises/{enterpriseId}/devices/{deviceId}.
    pub fn devices_get(&self, name: &str) -> EnterpriseDeviceGetCall<'a, S> {
        EnterpriseDeviceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Issues a command to a device. The Operation resource returned contains a Command in its metadata field. Use the get operation method to get the status of the command.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the device in the form enterprises/{enterpriseId}/devices/{deviceId}.
    pub fn devices_issue_command(&self, request: Command, name: &str) -> EnterpriseDeviceIssueCommandCall<'a, S> {
        EnterpriseDeviceIssueCommandCall {
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
    /// Lists devices for a given enterprise. Deleted devices are not returned in the response.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the enterprise in the form enterprises/{enterpriseId}.
    pub fn devices_list(&self, parent: &str) -> EnterpriseDeviceListCall<'a, S> {
        EnterpriseDeviceListCall {
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
    /// Updates a device.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the device in the form enterprises/{enterpriseId}/devices/{deviceId}.
    pub fn devices_patch(&self, request: Device, name: &str) -> EnterpriseDevicePatchCall<'a, S> {
        EnterpriseDevicePatchCall {
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
    /// Creates an enrollment token for a given enterprise. It's up to the caller's responsibility to manage the lifecycle of newly created tokens and deleting them when they're not intended to be used anymore. Once an enrollment token has been created, it's not possible to retrieve the token's content anymore using AM API. It is recommended for EMMs to securely store the token if it's intended to be reused.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The name of the enterprise in the form enterprises/{enterpriseId}.
    pub fn enrollment_tokens_create(&self, request: EnrollmentToken, parent: &str) -> EnterpriseEnrollmentTokenCreateCall<'a, S> {
        EnterpriseEnrollmentTokenCreateCall {
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
    /// Deletes an enrollment token. This operation invalidates the token, preventing its future use.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the enrollment token in the form enterprises/{enterpriseId}/enrollmentTokens/{enrollmentTokenId}.
    pub fn enrollment_tokens_delete(&self, name: &str) -> EnterpriseEnrollmentTokenDeleteCall<'a, S> {
        EnterpriseEnrollmentTokenDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an active, unexpired enrollment token. Only a partial view of EnrollmentToken is returned: all the fields but name and expiration_timestamp are empty. This method is meant to help manage active enrollment tokens lifecycle. For security reasons, it's recommended to delete active enrollment tokens as soon as they're not intended to be used anymore.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the enrollment token in the form enterprises/{enterpriseId}/enrollmentTokens/{enrollmentTokenId}.
    pub fn enrollment_tokens_get(&self, name: &str) -> EnterpriseEnrollmentTokenGetCall<'a, S> {
        EnterpriseEnrollmentTokenGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists active, unexpired enrollment tokens for a given enterprise. The list items contain only a partial view of EnrollmentToken: all the fields but name and expiration_timestamp are empty. This method is meant to help manage active enrollment tokens lifecycle. For security reasons, it's recommended to delete active enrollment tokens as soon as they're not intended to be used anymore.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the enterprise in the form enterprises/{enterpriseId}.
    pub fn enrollment_tokens_list(&self, parent: &str) -> EnterpriseEnrollmentTokenListCall<'a, S> {
        EnterpriseEnrollmentTokenListCall {
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
    /// Deletes a policy. This operation is only permitted if no devices are currently referencing the policy.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the policy in the form enterprises/{enterpriseId}/policies/{policyId}.
    pub fn policies_delete(&self, name: &str) -> EnterprisePolicyDeleteCall<'a, S> {
        EnterprisePolicyDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a policy.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the policy in the form enterprises/{enterpriseId}/policies/{policyId}.
    pub fn policies_get(&self, name: &str) -> EnterprisePolicyGetCall<'a, S> {
        EnterprisePolicyGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists policies for a given enterprise.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the enterprise in the form enterprises/{enterpriseId}.
    pub fn policies_list(&self, parent: &str) -> EnterprisePolicyListCall<'a, S> {
        EnterprisePolicyListCall {
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
    /// Updates or creates a policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the policy in the form enterprises/{enterpriseId}/policies/{policyId}.
    pub fn policies_patch(&self, request: Policy, name: &str) -> EnterprisePolicyPatchCall<'a, S> {
        EnterprisePolicyPatchCall {
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
    /// Creates a web app.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The name of the enterprise in the form enterprises/{enterpriseId}.
    pub fn web_apps_create(&self, request: WebApp, parent: &str) -> EnterpriseWebAppCreateCall<'a, S> {
        EnterpriseWebAppCreateCall {
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
    /// Deletes a web app.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the web app in the form enterprises/{enterpriseId}/webApps/{packageName}.
    pub fn web_apps_delete(&self, name: &str) -> EnterpriseWebAppDeleteCall<'a, S> {
        EnterpriseWebAppDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a web app.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the web app in the form enterprises/{enterpriseId}/webApp/{packageName}.
    pub fn web_apps_get(&self, name: &str) -> EnterpriseWebAppGetCall<'a, S> {
        EnterpriseWebAppGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists web apps for a given enterprise.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the enterprise in the form enterprises/{enterpriseId}.
    pub fn web_apps_list(&self, parent: &str) -> EnterpriseWebAppListCall<'a, S> {
        EnterpriseWebAppListCall {
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
    /// Updates a web app.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the web app in the form enterprises/{enterpriseId}/webApps/{packageName}.
    pub fn web_apps_patch(&self, request: WebApp, name: &str) -> EnterpriseWebAppPatchCall<'a, S> {
        EnterpriseWebAppPatchCall {
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
    /// Creates a web token to access an embeddable managed Google Play web UI for a given enterprise.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The name of the enterprise in the form enterprises/{enterpriseId}.
    pub fn web_tokens_create(&self, request: WebToken, parent: &str) -> EnterpriseWebTokenCreateCall<'a, S> {
        EnterpriseWebTokenCreateCall {
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
    /// Creates an enterprise. This is the last step in the enterprise signup flow.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: Enterprise) -> EnterpriseCreateCall<'a, S> {
        EnterpriseCreateCall {
            hub: self.hub,
            _request: request,
            _signup_url_name: Default::default(),
            _project_id: Default::default(),
            _enterprise_token: Default::default(),
            _agreement_accepted: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an enterprise. Only available for EMM-managed enterprises.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the enterprise in the form enterprises/{enterpriseId}.
    pub fn delete(&self, name: &str) -> EnterpriseDeleteCall<'a, S> {
        EnterpriseDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an enterprise.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the enterprise in the form enterprises/{enterpriseId}.
    pub fn get(&self, name: &str) -> EnterpriseGetCall<'a, S> {
        EnterpriseGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists EMM-managed enterprises. Only BASIC fields are returned.
    pub fn list(&self) -> EnterpriseListCall<'a, S> {
        EnterpriseListCall {
            hub: self.hub,
            _view: Default::default(),
            _project_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an enterprise.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the enterprise in the form enterprises/{enterpriseId}.
    pub fn patch(&self, request: Enterprise, name: &str) -> EnterprisePatchCall<'a, S> {
        EnterprisePatchCall {
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



/// A builder providing access to all methods supported on *signupUrl* resources.
/// It is not used directly, but through the [`AndroidManagement`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidmanagement1 as androidmanagement1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidmanagement1::{AndroidManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`
/// // to build up your call.
/// let rb = hub.signup_urls();
/// # }
/// ```
pub struct SignupUrlMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidManagement<S>,
}

impl<'a, S> client::MethodsBuilder for SignupUrlMethods<'a, S> {}

impl<'a, S> SignupUrlMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an enterprise signup URL.
    pub fn create(&self) -> SignupUrlCreateCall<'a, S> {
        SignupUrlCreateCall {
            hub: self.hub,
            _project_id: Default::default(),
            _callback_url: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



