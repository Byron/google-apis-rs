use super::*;
/// A builder providing access to all methods supported on *folder* resources.
/// It is not used directly, but through the [`ResourceSettings`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_resourcesettings1 as resourcesettings1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use resourcesettings1::{ResourceSettings, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ResourceSettings::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `settings_get(...)`, `settings_list(...)` and `settings_patch(...)`
/// // to build up your call.
/// let rb = hub.folders();
/// # }
/// ```
pub struct FolderMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ResourceSettings<S>,
}

impl<'a, S> client::MethodsBuilder for FolderMethods<'a, S> {}

impl<'a, S> FolderMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a specified setting. Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the setting does not exist.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the setting to get. See Setting for naming requirements.
    pub fn settings_get(&self, name: &str) -> FolderSettingGetCall<'a, S> {
        FolderSettingGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the settings that are available on the Cloud resource `parent`.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project, folder, or organization that is the parent resource for this setting. Must be in one of the following forms: * `projects/{project_number}` * `projects/{project_id}` * `folders/{folder_id}` * `organizations/{organization_id}`
    pub fn settings_list(&self, parent: &str) -> FolderSettingListCall<'a, S> {
        FolderSettingListCall {
            hub: self.hub,
            _parent: parent.to_string(),
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
    /// Updates a specified setting. Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the setting does not exist. Returns a `google.rpc.Status` with `google.rpc.Code.FAILED_PRECONDITION` if the setting is flagged as read only. Returns a `google.rpc.Status` with `google.rpc.Code.ABORTED` if the etag supplied in the request does not match the persisted etag of the setting value. On success, the response will contain only `name`, `local_value` and `etag`. The `metadata` and `effective_value` cannot be updated through this API. Note: the supplied setting will perform a full overwrite of the `local_value` field.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the setting. Must be in one of the following forms: * `projects/{project_number}/settings/{setting_name}` * `folders/{folder_id}/settings/{setting_name}` * `organizations/{organization_id}/settings/{setting_name}` For example, "/projects/123/settings/gcp-enableMyFeature"
    pub fn settings_patch(&self, request: GoogleCloudResourcesettingsV1Setting, name: &str) -> FolderSettingPatchCall<'a, S> {
        FolderSettingPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *organization* resources.
/// It is not used directly, but through the [`ResourceSettings`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_resourcesettings1 as resourcesettings1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use resourcesettings1::{ResourceSettings, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ResourceSettings::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `settings_get(...)`, `settings_list(...)` and `settings_patch(...)`
/// // to build up your call.
/// let rb = hub.organizations();
/// # }
/// ```
pub struct OrganizationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ResourceSettings<S>,
}

impl<'a, S> client::MethodsBuilder for OrganizationMethods<'a, S> {}

impl<'a, S> OrganizationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a specified setting. Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the setting does not exist.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the setting to get. See Setting for naming requirements.
    pub fn settings_get(&self, name: &str) -> OrganizationSettingGetCall<'a, S> {
        OrganizationSettingGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the settings that are available on the Cloud resource `parent`.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project, folder, or organization that is the parent resource for this setting. Must be in one of the following forms: * `projects/{project_number}` * `projects/{project_id}` * `folders/{folder_id}` * `organizations/{organization_id}`
    pub fn settings_list(&self, parent: &str) -> OrganizationSettingListCall<'a, S> {
        OrganizationSettingListCall {
            hub: self.hub,
            _parent: parent.to_string(),
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
    /// Updates a specified setting. Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the setting does not exist. Returns a `google.rpc.Status` with `google.rpc.Code.FAILED_PRECONDITION` if the setting is flagged as read only. Returns a `google.rpc.Status` with `google.rpc.Code.ABORTED` if the etag supplied in the request does not match the persisted etag of the setting value. On success, the response will contain only `name`, `local_value` and `etag`. The `metadata` and `effective_value` cannot be updated through this API. Note: the supplied setting will perform a full overwrite of the `local_value` field.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the setting. Must be in one of the following forms: * `projects/{project_number}/settings/{setting_name}` * `folders/{folder_id}/settings/{setting_name}` * `organizations/{organization_id}/settings/{setting_name}` For example, "/projects/123/settings/gcp-enableMyFeature"
    pub fn settings_patch(&self, request: GoogleCloudResourcesettingsV1Setting, name: &str) -> OrganizationSettingPatchCall<'a, S> {
        OrganizationSettingPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`ResourceSettings`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_resourcesettings1 as resourcesettings1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use resourcesettings1::{ResourceSettings, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ResourceSettings::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `settings_get(...)`, `settings_list(...)` and `settings_patch(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ResourceSettings<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a specified setting. Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the setting does not exist.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the setting to get. See Setting for naming requirements.
    pub fn settings_get(&self, name: &str) -> ProjectSettingGetCall<'a, S> {
        ProjectSettingGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the settings that are available on the Cloud resource `parent`.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project, folder, or organization that is the parent resource for this setting. Must be in one of the following forms: * `projects/{project_number}` * `projects/{project_id}` * `folders/{folder_id}` * `organizations/{organization_id}`
    pub fn settings_list(&self, parent: &str) -> ProjectSettingListCall<'a, S> {
        ProjectSettingListCall {
            hub: self.hub,
            _parent: parent.to_string(),
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
    /// Updates a specified setting. Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the setting does not exist. Returns a `google.rpc.Status` with `google.rpc.Code.FAILED_PRECONDITION` if the setting is flagged as read only. Returns a `google.rpc.Status` with `google.rpc.Code.ABORTED` if the etag supplied in the request does not match the persisted etag of the setting value. On success, the response will contain only `name`, `local_value` and `etag`. The `metadata` and `effective_value` cannot be updated through this API. Note: the supplied setting will perform a full overwrite of the `local_value` field.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the setting. Must be in one of the following forms: * `projects/{project_number}/settings/{setting_name}` * `folders/{folder_id}/settings/{setting_name}` * `organizations/{organization_id}/settings/{setting_name}` For example, "/projects/123/settings/gcp-enableMyFeature"
    pub fn settings_patch(&self, request: GoogleCloudResourcesettingsV1Setting, name: &str) -> ProjectSettingPatchCall<'a, S> {
        ProjectSettingPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



