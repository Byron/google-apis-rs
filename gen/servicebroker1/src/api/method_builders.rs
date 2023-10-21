use super::*;
/// A builder providing access to all free methods, which are not associated with a particular resource.
/// It is not used directly, but through the [`ServiceBroker`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_servicebroker1 as servicebroker1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use servicebroker1::{ServiceBroker, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ServiceBroker::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_iam_policy(...)`, `set_iam_policy(...)` and `test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.methods();
/// # }
/// ```
pub struct MethodMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ServiceBroker<S>,
}

impl<'a, S> client::MethodsBuilder for MethodMethods<'a, S> {}

impl<'a, S> MethodMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource.
    /// Returns an empty policy if the resource exists and does not have a policy
    /// set.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested.
    ///                See the operation documentation for the appropriate value for this field.
    pub fn get_iam_policy(&self, resource: &str) -> MethodGetIamPolicyCall<'a, S> {
        MethodGetIamPolicyCall {
            hub: self.hub,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any
    /// existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified.
    ///                See the operation documentation for the appropriate value for this field.
    pub fn set_iam_policy(&self, request: GoogleIamV1__SetIamPolicyRequest, resource: &str) -> MethodSetIamPolicyCall<'a, S> {
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
    /// Returns permissions that a caller has on the specified resource.
    /// If the resource does not exist, this will return an empty set of
    /// permissions, not a NOT_FOUND error.
    /// 
    /// Note: This operation is designed to be used for building permission-aware
    /// UIs and command-line tools, not for authorization checking. This operation
    /// may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested.
    ///                See the operation documentation for the appropriate value for this field.
    pub fn test_iam_permissions(&self, request: GoogleIamV1__TestIamPermissionsRequest, resource: &str) -> MethodTestIamPermissionCall<'a, S> {
        MethodTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



