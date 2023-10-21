use super::*;
/// A builder providing access to all methods supported on *deployment* resources.
/// It is not used directly, but through the [`DeploymentManager`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_deploymentmanager2 as deploymentmanager2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use deploymentmanager2::{DeploymentManager, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DeploymentManager::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `cancel_preview(...)`, `delete(...)`, `get(...)`, `get_iam_policy(...)`, `insert(...)`, `list(...)`, `patch(...)`, `set_iam_policy(...)`, `stop(...)`, `test_iam_permissions(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.deployments();
/// # }
/// ```
pub struct DeploymentMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DeploymentManager<S>,
}

impl<'a, S> client::MethodsBuilder for DeploymentMethods<'a, S> {}

impl<'a, S> DeploymentMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancels and removes the preview currently associated with the deployment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project ID for this request.
    /// * `deployment` - The name of the deployment for this request.
    pub fn cancel_preview(&self, request: DeploymentsCancelPreviewRequest, project: &str, deployment: &str) -> DeploymentCancelPreviewCall<'a, S> {
        DeploymentCancelPreviewCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _deployment: deployment.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a deployment and all of the resources in the deployment.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project ID for this request.
    /// * `deployment` - The name of the deployment for this request.
    pub fn delete(&self, project: &str, deployment: &str) -> DeploymentDeleteCall<'a, S> {
        DeploymentDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _deployment: deployment.to_string(),
            _delete_policy: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about a specific deployment.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project ID for this request.
    /// * `deployment` - The name of the deployment for this request.
    pub fn get(&self, project: &str, deployment: &str) -> DeploymentGetCall<'a, S> {
        DeploymentGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _deployment: deployment.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. May be empty if no such policy or resource exists.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn get_iam_policy(&self, project: &str, resource: &str) -> DeploymentGetIamPolicyCall<'a, S> {
        DeploymentGetIamPolicyCall {
            hub: self.hub,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a deployment and all of the resources described by the deployment manifest.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project ID for this request.
    pub fn insert(&self, request: Deployment, project: &str) -> DeploymentInsertCall<'a, S> {
        DeploymentInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _preview: Default::default(),
            _create_policy: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all deployments for a given project.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project ID for this request.
    pub fn list(&self, project: &str) -> DeploymentListCall<'a, S> {
        DeploymentListCall {
            hub: self.hub,
            _project: project.to_string(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches a deployment and all of the resources described by the deployment manifest.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project ID for this request.
    /// * `deployment` - The name of the deployment for this request.
    pub fn patch(&self, request: Deployment, project: &str, deployment: &str) -> DeploymentPatchCall<'a, S> {
        DeploymentPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _deployment: deployment.to_string(),
            _preview: Default::default(),
            _delete_policy: Default::default(),
            _create_policy: Default::default(),
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
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_iam_policy(&self, request: GlobalSetPolicyRequest, project: &str, resource: &str) -> DeploymentSetIamPolicyCall<'a, S> {
        DeploymentSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Stops an ongoing operation. This does not roll back any work that has already been completed, but prevents any new work from being started.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project ID for this request.
    /// * `deployment` - The name of the deployment for this request.
    pub fn stop(&self, request: DeploymentsStopRequest, project: &str, deployment: &str) -> DeploymentStopCall<'a, S> {
        DeploymentStopCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _deployment: deployment.to_string(),
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
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn test_iam_permissions(&self, request: TestPermissionsRequest, project: &str, resource: &str) -> DeploymentTestIamPermissionCall<'a, S> {
        DeploymentTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a deployment and all of the resources described by the deployment manifest.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project ID for this request.
    /// * `deployment` - The name of the deployment for this request.
    pub fn update(&self, request: Deployment, project: &str, deployment: &str) -> DeploymentUpdateCall<'a, S> {
        DeploymentUpdateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _deployment: deployment.to_string(),
            _preview: Default::default(),
            _delete_policy: Default::default(),
            _create_policy: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *manifest* resources.
/// It is not used directly, but through the [`DeploymentManager`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_deploymentmanager2 as deploymentmanager2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use deploymentmanager2::{DeploymentManager, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DeploymentManager::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.manifests();
/// # }
/// ```
pub struct ManifestMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DeploymentManager<S>,
}

impl<'a, S> client::MethodsBuilder for ManifestMethods<'a, S> {}

impl<'a, S> ManifestMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about a specific manifest.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project ID for this request.
    /// * `deployment` - The name of the deployment for this request.
    /// * `manifest` - The name of the manifest for this request.
    pub fn get(&self, project: &str, deployment: &str, manifest: &str) -> ManifestGetCall<'a, S> {
        ManifestGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _deployment: deployment.to_string(),
            _manifest: manifest.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all manifests for a given deployment.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project ID for this request.
    /// * `deployment` - The name of the deployment for this request.
    pub fn list(&self, project: &str, deployment: &str) -> ManifestListCall<'a, S> {
        ManifestListCall {
            hub: self.hub,
            _project: project.to_string(),
            _deployment: deployment.to_string(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *operation* resources.
/// It is not used directly, but through the [`DeploymentManager`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_deploymentmanager2 as deploymentmanager2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use deploymentmanager2::{DeploymentManager, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DeploymentManager::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.operations();
/// # }
/// ```
pub struct OperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DeploymentManager<S>,
}

impl<'a, S> client::MethodsBuilder for OperationMethods<'a, S> {}

impl<'a, S> OperationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about a specific operation.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project ID for this request.
    /// * `operation` - The name of the operation for this request.
    pub fn get(&self, project: &str, operation: &str) -> OperationGetCall<'a, S> {
        OperationGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _operation: operation.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all operations for a project.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project ID for this request.
    pub fn list(&self, project: &str) -> OperationListCall<'a, S> {
        OperationListCall {
            hub: self.hub,
            _project: project.to_string(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *resource* resources.
/// It is not used directly, but through the [`DeploymentManager`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_deploymentmanager2 as deploymentmanager2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use deploymentmanager2::{DeploymentManager, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DeploymentManager::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.resources();
/// # }
/// ```
pub struct ResourceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DeploymentManager<S>,
}

impl<'a, S> client::MethodsBuilder for ResourceMethods<'a, S> {}

impl<'a, S> ResourceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about a single resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project ID for this request.
    /// * `deployment` - The name of the deployment for this request.
    /// * `resource` - The name of the resource for this request.
    pub fn get(&self, project: &str, deployment: &str, resource: &str) -> ResourceGetCall<'a, S> {
        ResourceGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _deployment: deployment.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all resources in a given deployment.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project ID for this request.
    /// * `deployment` - The name of the deployment for this request.
    pub fn list(&self, project: &str, deployment: &str) -> ResourceListCall<'a, S> {
        ResourceListCall {
            hub: self.hub,
            _project: project.to_string(),
            _deployment: deployment.to_string(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *type* resources.
/// It is not used directly, but through the [`DeploymentManager`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_deploymentmanager2 as deploymentmanager2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use deploymentmanager2::{DeploymentManager, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DeploymentManager::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.types();
/// # }
/// ```
pub struct TypeMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DeploymentManager<S>,
}

impl<'a, S> client::MethodsBuilder for TypeMethods<'a, S> {}

impl<'a, S> TypeMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all resource types for Deployment Manager.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project ID for this request.
    pub fn list(&self, project: &str) -> TypeListCall<'a, S> {
        TypeListCall {
            hub: self.hub,
            _project: project.to_string(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



