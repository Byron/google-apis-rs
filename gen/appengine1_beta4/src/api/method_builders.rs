use super::*;
/// A builder providing access to all methods supported on *app* resources.
/// It is not used directly, but through the [`Appengine`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_appengine1_beta4 as appengine1_beta4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use appengine1_beta4::{Appengine, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Appengine::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `get(...)`, `locations_get(...)`, `locations_list(...)`, `modules_delete(...)`, `modules_get(...)`, `modules_list(...)`, `modules_patch(...)`, `modules_versions_create(...)`, `modules_versions_delete(...)`, `modules_versions_get(...)`, `modules_versions_instances_debug(...)`, `modules_versions_instances_delete(...)`, `modules_versions_instances_get(...)`, `modules_versions_instances_list(...)`, `modules_versions_list(...)`, `modules_versions_patch(...)`, `operations_get(...)`, `operations_list(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.apps();
/// # }
/// ```
pub struct AppMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Appengine<S>,
}

impl<'a, S> client::MethodsBuilder for AppMethods<'a, S> {}

impl<'a, S> AppMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about a location.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `name`. Resource name for the location.
    /// * `locationsId` - Part of `name`. See documentation of `appsId`.
    pub fn locations_get(&self, apps_id: &str, locations_id: &str) -> AppLocationGetCall<'a, S> {
        AppLocationGetCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _locations_id: locations_id.to_string(),
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
    /// * `appsId` - Part of `name`. The resource that owns the locations collection, if applicable.
    pub fn locations_list(&self, apps_id: &str) -> AppLocationListCall<'a, S> {
        AppLocationListCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
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
    /// Enables debugging on a VM instance. This allows you to use the SSH command to connect to the virtual machine where the instance lives. While in "debug mode", the instance continues to serve live traffic. You should delete the instance when you are done debugging and then allow the system to take over and determine if another instance should be started.Only applicable for instances in App Engine flexible environment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `appsId` - Part of `name`. Name of the resource requested. Example: apps/myapp/modules/default/versions/v1/instances/instance-1.
    /// * `modulesId` - Part of `name`. See documentation of `appsId`.
    /// * `versionsId` - Part of `name`. See documentation of `appsId`.
    /// * `instancesId` - Part of `name`. See documentation of `appsId`.
    pub fn modules_versions_instances_debug(&self, request: DebugInstanceRequest, apps_id: &str, modules_id: &str, versions_id: &str, instances_id: &str) -> AppModuleVersionInstanceDebugCall<'a, S> {
        AppModuleVersionInstanceDebugCall {
            hub: self.hub,
            _request: request,
            _apps_id: apps_id.to_string(),
            _modules_id: modules_id.to_string(),
            _versions_id: versions_id.to_string(),
            _instances_id: instances_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Stops a running instance.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `name`. Name of the resource requested. Example: apps/myapp/modules/default/versions/v1/instances/instance-1.
    /// * `modulesId` - Part of `name`. See documentation of `appsId`.
    /// * `versionsId` - Part of `name`. See documentation of `appsId`.
    /// * `instancesId` - Part of `name`. See documentation of `appsId`.
    pub fn modules_versions_instances_delete(&self, apps_id: &str, modules_id: &str, versions_id: &str, instances_id: &str) -> AppModuleVersionInstanceDeleteCall<'a, S> {
        AppModuleVersionInstanceDeleteCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _modules_id: modules_id.to_string(),
            _versions_id: versions_id.to_string(),
            _instances_id: instances_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets instance information.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `name`. Name of the resource requested. Example: apps/myapp/modules/default/versions/v1/instances/instance-1.
    /// * `modulesId` - Part of `name`. See documentation of `appsId`.
    /// * `versionsId` - Part of `name`. See documentation of `appsId`.
    /// * `instancesId` - Part of `name`. See documentation of `appsId`.
    pub fn modules_versions_instances_get(&self, apps_id: &str, modules_id: &str, versions_id: &str, instances_id: &str) -> AppModuleVersionInstanceGetCall<'a, S> {
        AppModuleVersionInstanceGetCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _modules_id: modules_id.to_string(),
            _versions_id: versions_id.to_string(),
            _instances_id: instances_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the instances of a version.Tip: To aggregate details about instances over time, see the Stackdriver Monitoring API (https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list).
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `name`. Name of the resource requested. Example: apps/myapp/modules/default/versions/v1.
    /// * `modulesId` - Part of `name`. See documentation of `appsId`.
    /// * `versionsId` - Part of `name`. See documentation of `appsId`.
    pub fn modules_versions_instances_list(&self, apps_id: &str, modules_id: &str, versions_id: &str) -> AppModuleVersionInstanceListCall<'a, S> {
        AppModuleVersionInstanceListCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _modules_id: modules_id.to_string(),
            _versions_id: versions_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deploys code and resource files to a new version.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `appsId` - Part of `name`. Name of the resource to update. Example: apps/myapp/modules/default.
    /// * `modulesId` - Part of `name`. See documentation of `appsId`.
    pub fn modules_versions_create(&self, request: Version, apps_id: &str, modules_id: &str) -> AppModuleVersionCreateCall<'a, S> {
        AppModuleVersionCreateCall {
            hub: self.hub,
            _request: request,
            _apps_id: apps_id.to_string(),
            _modules_id: modules_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing version.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `name`. Name of the resource requested. Example: apps/myapp/modules/default/versions/v1.
    /// * `modulesId` - Part of `name`. See documentation of `appsId`.
    /// * `versionsId` - Part of `name`. See documentation of `appsId`.
    pub fn modules_versions_delete(&self, apps_id: &str, modules_id: &str, versions_id: &str) -> AppModuleVersionDeleteCall<'a, S> {
        AppModuleVersionDeleteCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _modules_id: modules_id.to_string(),
            _versions_id: versions_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified Version resource. By default, only a BASIC_VIEW will be returned. Specify the FULL_VIEW parameter to get the full resource.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `name`. Name of the resource requested. Example: apps/myapp/modules/default/versions/v1.
    /// * `modulesId` - Part of `name`. See documentation of `appsId`.
    /// * `versionsId` - Part of `name`. See documentation of `appsId`.
    pub fn modules_versions_get(&self, apps_id: &str, modules_id: &str, versions_id: &str) -> AppModuleVersionGetCall<'a, S> {
        AppModuleVersionGetCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _modules_id: modules_id.to_string(),
            _versions_id: versions_id.to_string(),
            _view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the versions of a module.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `name`. Name of the resource requested. Example: apps/myapp/modules/default.
    /// * `modulesId` - Part of `name`. See documentation of `appsId`.
    pub fn modules_versions_list(&self, apps_id: &str, modules_id: &str) -> AppModuleVersionListCall<'a, S> {
        AppModuleVersionListCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _modules_id: modules_id.to_string(),
            _view: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified Version resource. You can specify the following fields depending on the App Engine environment and type of scaling that the version resource uses:
    /// serving_status (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta4/apps.modules.versions#Version.FIELDS.serving_status):  For Version resources that use basic scaling, manual scaling, or run in  the App Engine flexible environment.
    /// instance_class (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta4/apps.modules.versions#Version.FIELDS.instance_class):  For Version resources that run in the App Engine standard environment.
    /// automatic_scaling.min_idle_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta4/apps.modules.versions#Version.FIELDS.automatic_scaling):  For Version resources that use automatic scaling and run in the App  Engine standard environment.
    /// automatic_scaling.max_idle_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta4/apps.modules.versions#Version.FIELDS.automatic_scaling):  For Version resources that use automatic scaling and run in the App  Engine standard environment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `appsId` - Part of `name`. Name of the resource to update. Example: apps/myapp/modules/default/versions/1.
    /// * `modulesId` - Part of `name`. See documentation of `appsId`.
    /// * `versionsId` - Part of `name`. See documentation of `appsId`.
    pub fn modules_versions_patch(&self, request: Version, apps_id: &str, modules_id: &str, versions_id: &str) -> AppModuleVersionPatchCall<'a, S> {
        AppModuleVersionPatchCall {
            hub: self.hub,
            _request: request,
            _apps_id: apps_id.to_string(),
            _modules_id: modules_id.to_string(),
            _versions_id: versions_id.to_string(),
            _mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified module and all enclosed versions.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `name`. Name of the resource requested. Example: apps/myapp/modules/default.
    /// * `modulesId` - Part of `name`. See documentation of `appsId`.
    pub fn modules_delete(&self, apps_id: &str, modules_id: &str) -> AppModuleDeleteCall<'a, S> {
        AppModuleDeleteCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _modules_id: modules_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the current configuration of the specified module.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `name`. Name of the resource requested. Example: apps/myapp/modules/default.
    /// * `modulesId` - Part of `name`. See documentation of `appsId`.
    pub fn modules_get(&self, apps_id: &str, modules_id: &str) -> AppModuleGetCall<'a, S> {
        AppModuleGetCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _modules_id: modules_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the modules in the application.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `name`. Name of the resource requested. Example: apps/myapp.
    pub fn modules_list(&self, apps_id: &str) -> AppModuleListCall<'a, S> {
        AppModuleListCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the configuration of the specified module.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `appsId` - Part of `name`. Name of the resource to update. Example: apps/myapp/modules/default.
    /// * `modulesId` - Part of `name`. See documentation of `appsId`.
    pub fn modules_patch(&self, request: Module, apps_id: &str, modules_id: &str) -> AppModulePatchCall<'a, S> {
        AppModulePatchCall {
            hub: self.hub,
            _request: request,
            _apps_id: apps_id.to_string(),
            _modules_id: modules_id.to_string(),
            _migrate_traffic: Default::default(),
            _mask: Default::default(),
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
    /// * `appsId` - Part of `name`. The name of the operation resource.
    /// * `operationsId` - Part of `name`. See documentation of `appsId`.
    pub fn operations_get(&self, apps_id: &str, operations_id: &str) -> AppOperationGetCall<'a, S> {
        AppOperationGetCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _operations_id: operations_id.to_string(),
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
    /// * `appsId` - Part of `name`. The name of the operation's parent resource.
    pub fn operations_list(&self, apps_id: &str) -> AppOperationListCall<'a, S> {
        AppOperationListCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
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
    /// Creates an App Engine application for a Google Cloud Platform project. Required fields:
    /// id - The ID of the target Cloud Platform project.
    /// location - The region (https://cloud.google.com/appengine/docs/locations) where you want the App Engine application located.For more information about App Engine applications, see Managing Projects, Applications, and Billing (https://cloud.google.com/appengine/docs/python/console/).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: Application) -> AppCreateCall<'a, S> {
        AppCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about an application.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `name`. Name of the application to get. Example: apps/myapp.
    pub fn get(&self, apps_id: &str) -> AppGetCall<'a, S> {
        AppGetCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _ensure_resources_exist: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified Application resource. You can update the following fields:
    /// auth_domain (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta4/apps#Application.FIELDS.auth_domain)
    /// default_cookie_expiration (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta4/apps#Application.FIELDS.default_cookie_expiration)
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `appsId` - Part of `name`. Name of the Application resource to update. Example: apps/myapp.
    pub fn patch(&self, request: Application, apps_id: &str) -> AppPatchCall<'a, S> {
        AppPatchCall {
            hub: self.hub,
            _request: request,
            _apps_id: apps_id.to_string(),
            _mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



