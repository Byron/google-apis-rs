use super::*;
/// A builder providing access to all methods supported on *deployment* resources.
/// It is not used directly, but through the [`Manager`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_manager1_beta2 as manager1_beta2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use manager1_beta2::{Manager, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Manager::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.deployments();
/// # }
/// ```
pub struct DeploymentMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Manager<S>,
}

impl<'a, S> client::MethodsBuilder for DeploymentMethods<'a, S> {}

impl<'a, S> DeploymentMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// 
    /// 
    /// # Arguments
    ///
    /// * `projectId` - No description provided.
    /// * `region` - No description provided.
    /// * `deploymentName` - No description provided.
    pub fn delete(&self, project_id: &str, region: &str, deployment_name: &str) -> DeploymentDeleteCall<'a, S> {
        DeploymentDeleteCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _region: region.to_string(),
            _deployment_name: deployment_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// 
    /// 
    /// # Arguments
    ///
    /// * `projectId` - No description provided.
    /// * `region` - No description provided.
    /// * `deploymentName` - No description provided.
    pub fn get(&self, project_id: &str, region: &str, deployment_name: &str) -> DeploymentGetCall<'a, S> {
        DeploymentGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _region: region.to_string(),
            _deployment_name: deployment_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// 
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - No description provided.
    /// * `region` - No description provided.
    pub fn insert(&self, request: Deployment, project_id: &str, region: &str) -> DeploymentInsertCall<'a, S> {
        DeploymentInsertCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _region: region.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// 
    /// 
    /// # Arguments
    ///
    /// * `projectId` - No description provided.
    /// * `region` - No description provided.
    pub fn list(&self, project_id: &str, region: &str) -> DeploymentListCall<'a, S> {
        DeploymentListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _region: region.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *template* resources.
/// It is not used directly, but through the [`Manager`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_manager1_beta2 as manager1_beta2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use manager1_beta2::{Manager, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Manager::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.templates();
/// # }
/// ```
pub struct TemplateMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Manager<S>,
}

impl<'a, S> client::MethodsBuilder for TemplateMethods<'a, S> {}

impl<'a, S> TemplateMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// 
    /// 
    /// # Arguments
    ///
    /// * `projectId` - No description provided.
    /// * `templateName` - No description provided.
    pub fn delete(&self, project_id: &str, template_name: &str) -> TemplateDeleteCall<'a, S> {
        TemplateDeleteCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _template_name: template_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// 
    /// 
    /// # Arguments
    ///
    /// * `projectId` - No description provided.
    /// * `templateName` - No description provided.
    pub fn get(&self, project_id: &str, template_name: &str) -> TemplateGetCall<'a, S> {
        TemplateGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _template_name: template_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// 
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - No description provided.
    pub fn insert(&self, request: Template, project_id: &str) -> TemplateInsertCall<'a, S> {
        TemplateInsertCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// 
    /// 
    /// # Arguments
    ///
    /// * `projectId` - No description provided.
    pub fn list(&self, project_id: &str) -> TemplateListCall<'a, S> {
        TemplateListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



