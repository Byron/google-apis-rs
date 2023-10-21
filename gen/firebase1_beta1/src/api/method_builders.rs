use super::*;
/// A builder providing access to all methods supported on *availableProject* resources.
/// It is not used directly, but through the [`FirebaseManagement`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_firebase1_beta1 as firebase1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use firebase1_beta1::{FirebaseManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = FirebaseManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.available_projects();
/// # }
/// ```
pub struct AvailableProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a FirebaseManagement<S>,
}

impl<'a, S> client::MethodsBuilder for AvailableProjectMethods<'a, S> {}

impl<'a, S> AvailableProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists each [Google Cloud Platform (GCP) `Project`] (https://cloud.google.com/resource-manager/reference/rest/v1/projects) that can have Firebase resources added to it. A Project will only be listed if: - The caller has sufficient [Google IAM](https://cloud.google.com/iam) permissions to call AddFirebase. - The Project is not already a FirebaseProject. - The Project is not in an Organization which has policies that prevent Firebase resources from being added. 
    pub fn list(&self) -> AvailableProjectListCall<'a, S> {
        AvailableProjectListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *operation* resources.
/// It is not used directly, but through the [`FirebaseManagement`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_firebase1_beta1 as firebase1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use firebase1_beta1::{FirebaseManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = FirebaseManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.operations();
/// # }
/// ```
pub struct OperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a FirebaseManagement<S>,
}

impl<'a, S> client::MethodsBuilder for OperationMethods<'a, S> {}

impl<'a, S> OperationMethods<'a, S> {
    
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
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`FirebaseManagement`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_firebase1_beta1 as firebase1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use firebase1_beta1::{FirebaseManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = FirebaseManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `add_firebase(...)`, `add_google_analytics(...)`, `android_apps_create(...)`, `android_apps_get(...)`, `android_apps_get_config(...)`, `android_apps_list(...)`, `android_apps_patch(...)`, `android_apps_remove(...)`, `android_apps_sha_create(...)`, `android_apps_sha_delete(...)`, `android_apps_sha_list(...)`, `android_apps_undelete(...)`, `available_locations_list(...)`, `default_location_finalize(...)`, `get(...)`, `get_admin_sdk_config(...)`, `get_analytics_details(...)`, `ios_apps_create(...)`, `ios_apps_get(...)`, `ios_apps_get_config(...)`, `ios_apps_list(...)`, `ios_apps_patch(...)`, `ios_apps_remove(...)`, `ios_apps_undelete(...)`, `list(...)`, `patch(...)`, `remove_analytics(...)`, `search_apps(...)`, `web_apps_create(...)`, `web_apps_get(...)`, `web_apps_get_config(...)`, `web_apps_list(...)`, `web_apps_patch(...)`, `web_apps_remove(...)` and `web_apps_undelete(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a FirebaseManagement<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a ShaCertificate to the specified AndroidApp.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The resource name of the parent AndroidApp to which to add a ShaCertificate, in the format: projects/PROJECT_IDENTIFIER/androidApps/ APP_ID Since an APP_ID is a unique identifier, the Unique Resource from Sub-Collection access pattern may be used here, in the format: projects/-/androidApps/APP_ID Refer to the `AndroidApp` [`name`](https://firebase.google.com/../projects.androidApps#AndroidApp.FIELDS.name) field for details about PROJECT_IDENTIFIER and APP_ID values.
    pub fn android_apps_sha_create(&self, request: ShaCertificate, parent: &str) -> ProjectAndroidAppShaCreateCall<'a, S> {
        ProjectAndroidAppShaCreateCall {
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
    /// Removes a ShaCertificate from the specified AndroidApp.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the ShaCertificate to remove from the parent AndroidApp, in the format: projects/PROJECT_IDENTIFIER/androidApps/APP_ID /sha/SHA_HASH Refer to the `ShaCertificate` [`name`](https://firebase.google.com/../projects.androidApps.sha#ShaCertificate.FIELDS.name) field for details about PROJECT_IDENTIFIER, APP_ID, and SHA_HASH values. You can obtain the full resource name of the `ShaCertificate` from the response of [`ListShaCertificates`](https://firebase.google.com/../projects.androidApps.sha/list) or the original [`CreateShaCertificate`](https://firebase.google.com/../projects.androidApps.sha/create).
    pub fn android_apps_sha_delete(&self, name: &str) -> ProjectAndroidAppShaDeleteCall<'a, S> {
        ProjectAndroidAppShaDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the SHA-1 and SHA-256 certificates for the specified AndroidApp.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The resource name of the parent AndroidApp for which to list each associated ShaCertificate, in the format: projects/PROJECT_IDENTIFIER /androidApps/APP_ID Since an APP_ID is a unique identifier, the Unique Resource from Sub-Collection access pattern may be used here, in the format: projects/-/androidApps/APP_ID Refer to the `AndroidApp` [`name`](https://firebase.google.com/../projects.androidApps#AndroidApp.FIELDS.name) field for details about PROJECT_IDENTIFIER and APP_ID values.
    pub fn android_apps_sha_list(&self, parent: &str) -> ProjectAndroidAppShaListCall<'a, S> {
        ProjectAndroidAppShaListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Requests the creation of a new AndroidApp in the specified FirebaseProject. The result of this call is an `Operation` which can be used to track the provisioning process. The `Operation` is automatically deleted after completion, so there is no need to call `DeleteOperation`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The resource name of the parent FirebaseProject in which to create an AndroidApp, in the format: projects/PROJECT_IDENTIFIER/androidApps Refer to the `FirebaseProject` [`name`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values.
    pub fn android_apps_create(&self, request: AndroidApp, parent: &str) -> ProjectAndroidAppCreateCall<'a, S> {
        ProjectAndroidAppCreateCall {
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
    /// Gets the specified AndroidApp.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the AndroidApp, in the format: projects/ PROJECT_IDENTIFIER/androidApps/APP_ID Since an APP_ID is a unique identifier, the Unique Resource from Sub-Collection access pattern may be used here, in the format: projects/-/androidApps/APP_ID Refer to the `AndroidApp` [`name`](https://firebase.google.com/../projects.androidApps#AndroidApp.FIELDS.name) field for details about PROJECT_IDENTIFIER and APP_ID values.
    pub fn android_apps_get(&self, name: &str) -> ProjectAndroidAppGetCall<'a, S> {
        ProjectAndroidAppGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the configuration artifact associated with the specified AndroidApp.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the AndroidApp configuration to download, in the format: projects/PROJECT_IDENTIFIER/androidApps/APP_ID/config Since an APP_ID is a unique identifier, the Unique Resource from Sub-Collection access pattern may be used here, in the format: projects/-/androidApps/APP_ID Refer to the `AndroidApp` [`name`](https://firebase.google.com/../projects.androidApps#AndroidApp.FIELDS.name) field for details about PROJECT_IDENTIFIER and APP_ID values.
    pub fn android_apps_get_config(&self, name: &str) -> ProjectAndroidAppGetConfigCall<'a, S> {
        ProjectAndroidAppGetConfigCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists each AndroidApp associated with the specified FirebaseProject. The elements are returned in no particular order, but will be a consistent view of the Apps when additional requests are made with a `pageToken`.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The resource name of the parent FirebaseProject for which to list each associated AndroidApp, in the format: projects/PROJECT_IDENTIFIER /androidApps Refer to the `FirebaseProject` [`name`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values.
    pub fn android_apps_list(&self, parent: &str) -> ProjectAndroidAppListCall<'a, S> {
        ProjectAndroidAppListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _show_deleted: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the attributes of the specified AndroidApp.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the AndroidApp, in the format: projects/ PROJECT_IDENTIFIER/androidApps/APP_ID * PROJECT_IDENTIFIER: the parent Project’s [`ProjectNumber`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its [`ProjectId`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.project_id). Learn more about using project identifiers in Google’s [AIP 2510 standard](https://google.aip.dev/cloud/2510). Note that the value for PROJECT_IDENTIFIER in any response body will be the `ProjectId`. * APP_ID: the globally unique, Firebase-assigned identifier for the App (see [`appId`](https://firebase.google.com/../projects.androidApps#AndroidApp.FIELDS.app_id)).
    pub fn android_apps_patch(&self, request: AndroidApp, name: &str) -> ProjectAndroidAppPatchCall<'a, S> {
        ProjectAndroidAppPatchCall {
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
    /// Removes the specified AndroidApp from the FirebaseProject.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the AndroidApp, in the format: projects/ PROJECT_IDENTIFIER/androidApps/APP_ID Since an APP_ID is a unique identifier, the Unique Resource from Sub-Collection access pattern may be used here, in the format: projects/-/androidApps/APP_ID Refer to the AndroidApp [name](https://firebase.google.com/../projects.androidApps#AndroidApp.FIELDS.name) field for details about PROJECT_IDENTIFIER and APP_ID values.
    pub fn android_apps_remove(&self, request: RemoveAndroidAppRequest, name: &str) -> ProjectAndroidAppRemoveCall<'a, S> {
        ProjectAndroidAppRemoveCall {
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
    /// Restores the specified AndroidApp to the FirebaseProject.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the AndroidApp, in the format: projects/ PROJECT_IDENTIFIER/androidApps/APP_ID Since an APP_ID is a unique identifier, the Unique Resource from Sub-Collection access pattern may be used here, in the format: projects/-/androidApps/APP_ID Refer to the AndroidApp [name](https://firebase.google.com/../projects.androidApps#AndroidApp.FIELDS.name) field for details about PROJECT_IDENTIFIER and APP_ID values.
    pub fn android_apps_undelete(&self, request: UndeleteAndroidAppRequest, name: &str) -> ProjectAndroidAppUndeleteCall<'a, S> {
        ProjectAndroidAppUndeleteCall {
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
    /// Lists the valid Google Cloud Platform (GCP) resource locations for the specified Project (including a FirebaseProject). One of these locations can be selected as the Project's [_default_ GCP resource location](https://firebase.google.com/docs/projects/locations), which is the geographical location where the Project's resources, such as Cloud Firestore, will be provisioned by default. However, if the default GCP resource location has already been set for the Project, then this setting cannot be changed. This call checks for any possible [location restrictions](https://cloud.google.com/resource-manager/docs/organization-policy/defining-locations) for the specified Project and, thus, might return a subset of all possible GCP resource locations. To list all GCP resource locations (regardless of any restrictions), call the endpoint without specifying a unique project identifier (that is, `/v1beta1/{parent=projects/-}/listAvailableLocations`). To call `ListAvailableLocations` with a specified project, a member must be at minimum a Viewer of the Project. Calls without a specified project do not require any specific project permissions.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The FirebaseProject for which to list GCP resource locations, in the format: projects/PROJECT_IDENTIFIER Refer to the `FirebaseProject` [`name`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values. If no unique project identifier is specified (that is, `projects/-`), the returned list does not take into account org-specific or project-specific location restrictions.
    pub fn available_locations_list(&self, parent: &str) -> ProjectAvailableLocationListCall<'a, S> {
        ProjectAvailableLocationListCall {
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
    /// Sets the default Google Cloud Platform (GCP) resource location for the specified FirebaseProject. This method creates an App Engine application with a [default Cloud Storage bucket](https://cloud.google.com/appengine/docs/standard/python/googlecloudstorageclient/setting-up-cloud-storage#activating_a_cloud_storage_bucket), located in the specified [`locationId`](#body.request_body.FIELDS.location_id). This location must be one of the available [GCP resource locations](https://firebase.google.com/docs/projects/locations). After the default GCP resource location is finalized, or if it was already set, it cannot be changed. The default GCP resource location for the specified `FirebaseProject` might already be set because either the underlying GCP `Project` already has an App Engine application or `FinalizeDefaultLocation` was previously called with a specified `locationId`. Any new calls to `FinalizeDefaultLocation` with a *different* specified `locationId` will return a 409 error. The result of this call is an [`Operation`](https://firebase.google.com/../../v1beta1/operations), which can be used to track the provisioning process. The [`response`](https://firebase.google.com/../../v1beta1/operations#Operation.FIELDS.response) type of the `Operation` is google.protobuf.Empty. The `Operation` can be polled by its `name` using GetOperation until `done` is true. When `done` is true, the `Operation` has either succeeded or failed. If the `Operation` has succeeded, its [`response`](https://firebase.google.com/../../v1beta1/operations#Operation.FIELDS.response) will be set to a google.protobuf.Empty; if the `Operation` has failed, its `error` will be set to a google.rpc.Status. The `Operation` is automatically deleted after completion, so there is no need to call DeleteOperation. All fields listed in the [request body](#request-body) are required. To call `FinalizeDefaultLocation`, a member must be an Owner of the Project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The resource name of the FirebaseProject for which the default GCP resource location will be set, in the format: projects/PROJECT_IDENTIFIER Refer to the `FirebaseProject` [`name`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values.
    pub fn default_location_finalize(&self, request: FinalizeDefaultLocationRequest, parent: &str) -> ProjectDefaultLocationFinalizeCall<'a, S> {
        ProjectDefaultLocationFinalizeCall {
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
    /// Requests the creation of a new IosApp in the specified FirebaseProject. The result of this call is an `Operation` which can be used to track the provisioning process. The `Operation` is automatically deleted after completion, so there is no need to call `DeleteOperation`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The resource name of the parent FirebaseProject in which to create an IosApp, in the format: projects/PROJECT_IDENTIFIER/iosApps Refer to the `FirebaseProject` [`name`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values.
    pub fn ios_apps_create(&self, request: IosApp, parent: &str) -> ProjectIosAppCreateCall<'a, S> {
        ProjectIosAppCreateCall {
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
    /// Gets the specified IosApp.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the IosApp, in the format: projects/PROJECT_IDENTIFIER /iosApps/APP_ID Since an APP_ID is a unique identifier, the Unique Resource from Sub-Collection access pattern may be used here, in the format: projects/-/iosApps/APP_ID Refer to the `IosApp` [`name`](https://firebase.google.com/../projects.iosApps#IosApp.FIELDS.name) field for details about PROJECT_IDENTIFIER and APP_ID values.
    pub fn ios_apps_get(&self, name: &str) -> ProjectIosAppGetCall<'a, S> {
        ProjectIosAppGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the configuration artifact associated with the specified IosApp.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the App configuration to download, in the format: projects/PROJECT_IDENTIFIER/iosApps/APP_ID/config Since an APP_ID is a unique identifier, the Unique Resource from Sub-Collection access pattern may be used here, in the format: projects/-/iosApps/APP_ID Refer to the `IosApp` [`name`](https://firebase.google.com/../projects.iosApps#IosApp.FIELDS.name) field for details about PROJECT_IDENTIFIER and APP_ID values.
    pub fn ios_apps_get_config(&self, name: &str) -> ProjectIosAppGetConfigCall<'a, S> {
        ProjectIosAppGetConfigCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists each IosApp associated with the specified FirebaseProject. The elements are returned in no particular order, but will be a consistent view of the Apps when additional requests are made with a `pageToken`.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The resource name of the parent FirebaseProject for which to list each associated IosApp, in the format: projects/PROJECT_IDENTIFIER/iosApps Refer to the `FirebaseProject` [`name`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values.
    pub fn ios_apps_list(&self, parent: &str) -> ProjectIosAppListCall<'a, S> {
        ProjectIosAppListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _show_deleted: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the attributes of the specified IosApp.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the IosApp, in the format: projects/PROJECT_IDENTIFIER /iosApps/APP_ID * PROJECT_IDENTIFIER: the parent Project’s [`ProjectNumber`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its [`ProjectId`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.project_id). Learn more about using project identifiers in Google’s [AIP 2510 standard](https://google.aip.dev/cloud/2510). Note that the value for PROJECT_IDENTIFIER in any response body will be the `ProjectId`. * APP_ID: the globally unique, Firebase-assigned identifier for the App (see [`appId`](https://firebase.google.com/../projects.iosApps#IosApp.FIELDS.app_id)).
    pub fn ios_apps_patch(&self, request: IosApp, name: &str) -> ProjectIosAppPatchCall<'a, S> {
        ProjectIosAppPatchCall {
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
    /// Removes the specified IosApp from the FirebaseProject.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the IosApp, in the format: projects/ PROJECT_IDENTIFIER/iosApps/APP_ID Since an APP_ID is a unique identifier, the Unique Resource from Sub-Collection access pattern may be used here, in the format: projects/-/iosApps/APP_ID Refer to the IosApp [name](https://firebase.google.com/../projects.iosApps#IosApp.FIELDS.name) field for details about PROJECT_IDENTIFIER and APP_ID values.
    pub fn ios_apps_remove(&self, request: RemoveIosAppRequest, name: &str) -> ProjectIosAppRemoveCall<'a, S> {
        ProjectIosAppRemoveCall {
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
    /// Restores the specified IosApp to the FirebaseProject.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the IosApp, in the format: projects/ PROJECT_IDENTIFIER/iosApps/APP_ID Since an APP_ID is a unique identifier, the Unique Resource from Sub-Collection access pattern may be used here, in the format: projects/-/iosApps/APP_ID Refer to the IosApp [name](https://firebase.google.com/../projects.iosApps#IosApp.FIELDS.name) field for details about PROJECT_IDENTIFIER and APP_ID values.
    pub fn ios_apps_undelete(&self, request: UndeleteIosAppRequest, name: &str) -> ProjectIosAppUndeleteCall<'a, S> {
        ProjectIosAppUndeleteCall {
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
    /// Requests the creation of a new WebApp in the specified FirebaseProject. The result of this call is an `Operation` which can be used to track the provisioning process. The `Operation` is automatically deleted after completion, so there is no need to call `DeleteOperation`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The resource name of the parent FirebaseProject in which to create a WebApp, in the format: projects/PROJECT_IDENTIFIER/webApps Refer to the `FirebaseProject` [`name`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values.
    pub fn web_apps_create(&self, request: WebApp, parent: &str) -> ProjectWebAppCreateCall<'a, S> {
        ProjectWebAppCreateCall {
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
    /// Gets the specified WebApp.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the WebApp, in the format: projects/PROJECT_IDENTIFIER /webApps/APP_ID Since an APP_ID is a unique identifier, the Unique Resource from Sub-Collection access pattern may be used here, in the format: projects/-/webApps/APP_ID Refer to the `WebApp` [`name`](https://firebase.google.com/../projects.webApps#WebApp.FIELDS.name) field for details about PROJECT_IDENTIFIER and APP_ID values.
    pub fn web_apps_get(&self, name: &str) -> ProjectWebAppGetCall<'a, S> {
        ProjectWebAppGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the configuration artifact associated with the specified WebApp.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the WebApp configuration to download, in the format: projects/PROJECT_IDENTIFIER/webApps/APP_ID/config Since an APP_ID is a unique identifier, the Unique Resource from Sub-Collection access pattern may be used here, in the format: projects/-/webApps/APP_ID Refer to the `WebApp` [`name`](https://firebase.google.com/../projects.webApps#WebApp.FIELDS.name) field for details about PROJECT_IDENTIFIER and APP_ID values.
    pub fn web_apps_get_config(&self, name: &str) -> ProjectWebAppGetConfigCall<'a, S> {
        ProjectWebAppGetConfigCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists each WebApp associated with the specified FirebaseProject. The elements are returned in no particular order, but will be a consistent view of the Apps when additional requests are made with a `pageToken`.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The resource name of the parent FirebaseProject for which to list each associated WebApp, in the format: projects/PROJECT_IDENTIFIER/webApps Refer to the `FirebaseProject` [`name`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values.
    pub fn web_apps_list(&self, parent: &str) -> ProjectWebAppListCall<'a, S> {
        ProjectWebAppListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _show_deleted: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the attributes of the specified WebApp.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the WebApp, in the format: projects/PROJECT_IDENTIFIER /webApps/APP_ID * PROJECT_IDENTIFIER: the parent Project’s [`ProjectNumber`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its [`ProjectId`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.project_id). Learn more about using project identifiers in Google’s [AIP 2510 standard](https://google.aip.dev/cloud/2510). Note that the value for PROJECT_IDENTIFIER in any response body will be the `ProjectId`. * APP_ID: the globally unique, Firebase-assigned identifier for the App (see [`appId`](https://firebase.google.com/../projects.webApps#WebApp.FIELDS.app_id)).
    pub fn web_apps_patch(&self, request: WebApp, name: &str) -> ProjectWebAppPatchCall<'a, S> {
        ProjectWebAppPatchCall {
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
    /// Removes the specified WebApp from the FirebaseProject.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the WebApp, in the format: projects/ PROJECT_IDENTIFIER/webApps/APP_ID Since an APP_ID is a unique identifier, the Unique Resource from Sub-Collection access pattern may be used here, in the format: projects/-/webApps/APP_ID Refer to the WebApp [name](https://firebase.google.com/../projects.webApps#WebApp.FIELDS.name) field for details about PROJECT_IDENTIFIER and APP_ID values.
    pub fn web_apps_remove(&self, request: RemoveWebAppRequest, name: &str) -> ProjectWebAppRemoveCall<'a, S> {
        ProjectWebAppRemoveCall {
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
    /// Restores the specified WebApp to the FirebaseProject.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the WebApp, in the format: projects/ PROJECT_IDENTIFIER/webApps/APP_ID Since an APP_ID is a unique identifier, the Unique Resource from Sub-Collection access pattern may be used here, in the format: projects/-/webApps/APP_ID Refer to the WebApp [name](https://firebase.google.com/../projects.webApps#WebApp.FIELDS.name) field for details about PROJECT_IDENTIFIER and APP_ID values.
    pub fn web_apps_undelete(&self, request: UndeleteWebAppRequest, name: &str) -> ProjectWebAppUndeleteCall<'a, S> {
        ProjectWebAppUndeleteCall {
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
    /// Adds Firebase resources to the specified existing \[Google Cloud Platform (GCP) `Project`\] (https://cloud.google.com/resource-manager/reference/rest/v1/projects). Since a FirebaseProject is actually also a GCP `Project`, a `FirebaseProject` has the same underlying GCP identifiers (`projectNumber` and `projectId`). This allows for easy interop with Google APIs. The result of this call is an [`Operation`](https://firebase.google.com/../../v1beta1/operations). Poll the `Operation` to track the provisioning process by calling GetOperation until [`done`](https://firebase.google.com/../../v1beta1/operations#Operation.FIELDS.done) is `true`. When `done` is `true`, the `Operation` has either succeeded or failed. If the `Operation` succeeded, its [`response`](https://firebase.google.com/../../v1beta1/operations#Operation.FIELDS.response) is set to a FirebaseProject; if the `Operation` failed, its [`error`](https://firebase.google.com/../../v1beta1/operations#Operation.FIELDS.error) is set to a google.rpc.Status. The `Operation` is automatically deleted after completion, so there is no need to call DeleteOperation. This method does not modify any billing account information on the underlying GCP `Project`. To call `AddFirebase`, a project member or service account must have the following permissions (the IAM roles of Editor and Owner contain these permissions): `firebase.projects.update`, `resourcemanager.projects.get`, `serviceusage.services.enable`, and `serviceusage.services.get`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The resource name of the GCP `Project` to which Firebase resources will be added, in the format: projects/PROJECT_IDENTIFIER Refer to the `FirebaseProject` [`name`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values. After calling `AddFirebase`, the unique Project identifiers ( [`projectNumber`](https://cloud.google.com/resource-manager/reference/rest/v1/projects#Project.FIELDS.project_number) and [`projectId`](https://cloud.google.com/resource-manager/reference/rest/v1/projects#Project.FIELDS.project_id)) of the underlying GCP `Project` are also the identifiers of the FirebaseProject.
    pub fn add_firebase(&self, request: AddFirebaseRequest, project: &str) -> ProjectAddFirebaseCall<'a, S> {
        ProjectAddFirebaseCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Links the specified FirebaseProject with an existing [Google Analytics account](http://www.google.com/analytics/). Using this call, you can either: - Specify an `analyticsAccountId` to provision a new Google Analytics property within the specified account and associate the new property with the `FirebaseProject`. - Specify an existing `analyticsPropertyId` to associate the property with the `FirebaseProject`. Note that when you call `AddGoogleAnalytics`: 1. The first check determines if any existing data streams in the Google Analytics property correspond to any existing Firebase Apps in the `FirebaseProject` (based on the `packageName` or `bundleId` associated with the data stream). Then, as applicable, the data streams and apps are linked. Note that this auto-linking only applies to `AndroidApps` and `IosApps`. 2. If no corresponding data streams are found for the Firebase Apps, new data streams are provisioned in the Google Analytics property for each of the Firebase Apps. Note that a new data stream is always provisioned for a Web App even if it was previously associated with a data stream in the Analytics property. Learn more about the hierarchy and structure of Google Analytics accounts in the [Analytics documentation](https://support.google.com/analytics/answer/9303323). The result of this call is an [`Operation`](https://firebase.google.com/../../v1beta1/operations). Poll the `Operation` to track the provisioning process by calling GetOperation until [`done`](https://firebase.google.com/../../v1beta1/operations#Operation.FIELDS.done) is `true`. When `done` is `true`, the `Operation` has either succeeded or failed. If the `Operation` succeeded, its [`response`](https://firebase.google.com/../../v1beta1/operations#Operation.FIELDS.response) is set to an AnalyticsDetails; if the `Operation` failed, its [`error`](https://firebase.google.com/../../v1beta1/operations#Operation.FIELDS.error) is set to a google.rpc.Status. To call `AddGoogleAnalytics`, a project member must be an Owner for the existing `FirebaseProject` and have the [`Edit` permission](https://support.google.com/analytics/answer/2884495) for the Google Analytics account. If the `FirebaseProject` already has Google Analytics enabled, and you call `AddGoogleAnalytics` using an `analyticsPropertyId` that’s different from the currently associated property, then the call will fail. Analytics may have already been enabled in the Firebase console or by specifying `timeZone` and `regionCode` in the call to [`AddFirebase`](https://firebase.google.com/../../v1beta1/projects/addFirebase).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The resource name of the FirebaseProject to link to an existing Google Analytics account, in the format: projects/PROJECT_IDENTIFIER Refer to the `FirebaseProject` [`name`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values.
    pub fn add_google_analytics(&self, request: AddGoogleAnalyticsRequest, parent: &str) -> ProjectAddGoogleAnalyticCall<'a, S> {
        ProjectAddGoogleAnalyticCall {
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
    /// Gets the specified FirebaseProject.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the FirebaseProject, in the format: projects/ PROJECT_IDENTIFIER Refer to the `FirebaseProject` [`name`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values.
    pub fn get(&self, name: &str) -> ProjectGetCall<'a, S> {
        ProjectGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the configuration artifact associated with the specified FirebaseProject, which can be used by servers to simplify initialization. Typically, this configuration is used with the Firebase Admin SDK [initializeApp](https://firebase.google.com/docs/admin/setup#initialize_the_sdk) command.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the FirebaseProject, in the format: projects/ PROJECT_IDENTIFIER/adminSdkConfig Refer to the `FirebaseProject` [`name`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values.
    pub fn get_admin_sdk_config(&self, name: &str) -> ProjectGetAdminSdkConfigCall<'a, S> {
        ProjectGetAdminSdkConfigCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the Google Analytics details currently associated with the specified FirebaseProject. If the `FirebaseProject` is not yet linked to Google Analytics, then the response to `GetAnalyticsDetails` is `NOT_FOUND`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the FirebaseProject, in the format: projects/ PROJECT_IDENTIFIER/analyticsDetails Refer to the `FirebaseProject` [`name`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values.
    pub fn get_analytics_details(&self, name: &str) -> ProjectGetAnalyticsDetailCall<'a, S> {
        ProjectGetAnalyticsDetailCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists each FirebaseProject accessible to the caller. The elements are returned in no particular order, but they will be a consistent view of the Projects when additional requests are made with a `pageToken`. This method is eventually consistent with Project mutations, which means newly provisioned Projects and recent modifications to existing Projects might not be reflected in the set of Projects. The list will include only ACTIVE Projects. Use GetFirebaseProject for consistent reads as well as for additional Project details.
    pub fn list(&self) -> ProjectListCall<'a, S> {
        ProjectListCall {
            hub: self.hub,
            _show_deleted: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the attributes of the specified FirebaseProject. All [query parameters](#query-parameters) are required.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the Project, in the format: projects/PROJECT_IDENTIFIER PROJECT_IDENTIFIER: the Project’s [`ProjectNumber`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its [`ProjectId`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.project_id). Learn more about using project identifiers in Google’s [AIP 2510 standard](https://google.aip.dev/cloud/2510). Note that the value for PROJECT_IDENTIFIER in any response body will be the `ProjectId`.
    pub fn patch(&self, request: FirebaseProject, name: &str) -> ProjectPatchCall<'a, S> {
        ProjectPatchCall {
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
    /// Unlinks the specified FirebaseProject from its Google Analytics account. This call removes the association of the specified `FirebaseProject` with its current Google Analytics property. However, this call does not delete the Google Analytics resources, such as the Google Analytics property or any data streams. These resources may be re-associated later to the `FirebaseProject` by calling [`AddGoogleAnalytics`](https://firebase.google.com/../../v1beta1/projects/addGoogleAnalytics) and specifying the same `analyticsPropertyId`. For Android Apps and iOS Apps, this call re-links data streams with their corresponding apps. However, for Web Apps, this call provisions a *new* data stream for each Web App. To call `RemoveAnalytics`, a project member must be an Owner for the `FirebaseProject`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The resource name of the FirebaseProject to unlink from its Google Analytics account, in the format: projects/PROJECT_IDENTIFIER Refer to the `FirebaseProject` [`name`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values.
    pub fn remove_analytics(&self, request: RemoveAnalyticsRequest, parent: &str) -> ProjectRemoveAnalyticCall<'a, S> {
        ProjectRemoveAnalyticCall {
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
    /// Lists all available Apps for the specified FirebaseProject. This is a convenience method. Typically, interaction with an App should be done using the platform-specific service, but some tool use-cases require a summary of all known Apps (such as for App selector interfaces).
    /// 
    /// # Arguments
    ///
    /// * `parent` - The parent FirebaseProject for which to list Apps, in the format: projects/ PROJECT_IDENTIFIER Refer to the `FirebaseProject` [`name`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values.
    pub fn search_apps(&self, parent: &str) -> ProjectSearchAppCall<'a, S> {
        ProjectSearchAppCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _show_deleted: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



