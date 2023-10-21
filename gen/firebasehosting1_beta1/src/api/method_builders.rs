use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`FirebaseHosting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_firebasehosting1_beta1 as firebasehosting1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use firebasehosting1_beta1::{FirebaseHosting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = FirebaseHosting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `operations_get(...)`, `sites_channels_create(...)`, `sites_channels_delete(...)`, `sites_channels_get(...)`, `sites_channels_list(...)`, `sites_channels_patch(...)`, `sites_channels_releases_create(...)`, `sites_channels_releases_list(...)`, `sites_create(...)`, `sites_delete(...)`, `sites_domains_create(...)`, `sites_domains_delete(...)`, `sites_domains_get(...)`, `sites_domains_list(...)`, `sites_domains_update(...)`, `sites_get(...)`, `sites_get_config(...)`, `sites_list(...)`, `sites_patch(...)`, `sites_releases_create(...)`, `sites_releases_list(...)`, `sites_update_config(...)`, `sites_versions_clone(...)`, `sites_versions_create(...)`, `sites_versions_delete(...)`, `sites_versions_files_list(...)`, `sites_versions_list(...)`, `sites_versions_patch(...)` and `sites_versions_populate_files(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a FirebaseHosting<S>,
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
    pub fn operations_get(&self, name: &str) -> ProjectOperationGetCall<'a, S> {
        ProjectOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new release, which makes the content of the specified version actively display on the appropriate URL(s).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The site or channel to which the release belongs, in either of the following formats: - sites/SITE_ID - sites/SITE_ID/channels/CHANNEL_ID
    pub fn sites_channels_releases_create(&self, request: Release, parent: &str) -> ProjectSiteChannelReleaseCreateCall<'a, S> {
        ProjectSiteChannelReleaseCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _version_name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the releases that have been created for the specified site or channel. When used to list releases for a site, this list includes releases for both the default `live` channel and any active preview channels for the specified site.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The site or channel for which to list releases, in either of the following formats: - sites/SITE_ID - sites/SITE_ID/channels/CHANNEL_ID 
    pub fn sites_channels_releases_list(&self, parent: &str) -> ProjectSiteChannelReleaseListCall<'a, S> {
        ProjectSiteChannelReleaseListCall {
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
    /// Creates a new channel in the specified site.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The site in which to create this channel, in the format: sites/ SITE_ID
    pub fn sites_channels_create(&self, request: Channel, parent: &str) -> ProjectSiteChannelCreateCall<'a, S> {
        ProjectSiteChannelCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _channel_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified channel of the specified site. The `live` channel cannot be deleted.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The fully-qualified resource name for the channel, in the format: sites/SITE_ID/channels/CHANNEL_ID
    pub fn sites_channels_delete(&self, name: &str) -> ProjectSiteChannelDeleteCall<'a, S> {
        ProjectSiteChannelDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves information for the specified channel of the specified site.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The fully-qualified resource name for the channel, in the format: sites/SITE_ID/channels/CHANNEL_ID
    pub fn sites_channels_get(&self, name: &str) -> ProjectSiteChannelGetCall<'a, S> {
        ProjectSiteChannelGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the channels for the specified site. All sites have a default `live` channel.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The site for which to list channels, in the format: sites/SITE_ID
    pub fn sites_channels_list(&self, parent: &str) -> ProjectSiteChannelListCall<'a, S> {
        ProjectSiteChannelListCall {
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
    /// Updates information for the specified channel of the specified site. Implicitly creates the channel if it doesn't already exist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The fully-qualified resource name for the channel, in the format: sites/ SITE_ID/channels/CHANNEL_ID
    pub fn sites_channels_patch(&self, request: Channel, name: &str) -> ProjectSiteChannelPatchCall<'a, S> {
        ProjectSiteChannelPatchCall {
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
    /// Creates a domain mapping on the specified site.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent to create the domain association for, in the format: sites/site-name
    pub fn sites_domains_create(&self, request: Domain, parent: &str) -> ProjectSiteDomainCreateCall<'a, S> {
        ProjectSiteDomainCreateCall {
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
    /// Deletes the existing domain mapping on the specified site.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the domain association to delete.
    pub fn sites_domains_delete(&self, name: &str) -> ProjectSiteDomainDeleteCall<'a, S> {
        ProjectSiteDomainDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a domain mapping on the specified site.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the domain configuration to get.
    pub fn sites_domains_get(&self, name: &str) -> ProjectSiteDomainGetCall<'a, S> {
        ProjectSiteDomainGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the domains for the specified site.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent for which to list domains, in the format: sites/ site-name
    pub fn sites_domains_list(&self, parent: &str) -> ProjectSiteDomainListCall<'a, S> {
        ProjectSiteDomainListCall {
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
    /// Updates the specified domain mapping, creating the mapping as if it does not exist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the domain association to update or create, if an association doesn't already exist.
    pub fn sites_domains_update(&self, request: Domain, name: &str) -> ProjectSiteDomainUpdateCall<'a, S> {
        ProjectSiteDomainUpdateCall {
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
    /// Creates a new release, which makes the content of the specified version actively display on the appropriate URL(s).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The site or channel to which the release belongs, in either of the following formats: - sites/SITE_ID - sites/SITE_ID/channels/CHANNEL_ID
    pub fn sites_releases_create(&self, request: Release, parent: &str) -> ProjectSiteReleaseCreateCall<'a, S> {
        ProjectSiteReleaseCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _version_name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the releases that have been created for the specified site or channel. When used to list releases for a site, this list includes releases for both the default `live` channel and any active preview channels for the specified site.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The site or channel for which to list releases, in either of the following formats: - sites/SITE_ID - sites/SITE_ID/channels/CHANNEL_ID 
    pub fn sites_releases_list(&self, parent: &str) -> ProjectSiteReleaseListCall<'a, S> {
        ProjectSiteReleaseListCall {
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
    /// Lists the remaining files to be uploaded for the specified version.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The version for which to list files, in the format: sites/SITE_ID /versions/VERSION_ID
    pub fn sites_versions_files_list(&self, parent: &str) -> ProjectSiteVersionFileListCall<'a, S> {
        ProjectSiteVersionFileListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _status: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new version on the specified target site using the content of the specified version.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The target site for the cloned version, in the format: sites/ SITE_ID
    pub fn sites_versions_clone(&self, request: CloneVersionRequest, parent: &str) -> ProjectSiteVersionCloneCall<'a, S> {
        ProjectSiteVersionCloneCall {
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
    /// Creates a new version for the specified site.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The site in which to create the version, in the format: sites/ SITE_ID
    pub fn sites_versions_create(&self, request: Version, parent: &str) -> ProjectSiteVersionCreateCall<'a, S> {
        ProjectSiteVersionCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _version_id: Default::default(),
            _size_bytes: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified version.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The fully-qualified resource name for the version, in the format: sites/SITE_ID/versions/VERSION_ID
    pub fn sites_versions_delete(&self, name: &str) -> ProjectSiteVersionDeleteCall<'a, S> {
        ProjectSiteVersionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the versions that have been created for the specified site. This list includes versions for both the default `live` channel and any active preview channels for the specified site.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The site or channel for which to list versions, in either of the following formats: - sites/SITE_ID - sites/SITE_ID/channels/CHANNEL_ID 
    pub fn sites_versions_list(&self, parent: &str) -> ProjectSiteVersionListCall<'a, S> {
        ProjectSiteVersionListCall {
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
    /// Updates the specified metadata for the specified version. This method will fail with `FAILED_PRECONDITION` in the event of an invalid state transition. The supported [state](https://firebase.google.com/docs/hosting/../sites.versions#versionstatus) transitions for a version are from `CREATED` to `FINALIZED`. Use [`DeleteVersion`](delete) to set the status of a version to `DELETED`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The fully-qualified resource name for the version, in the format: sites/ SITE_ID/versions/VERSION_ID This name is provided in the response body when you call [`CreateVersion`](sites.versions/create).
    pub fn sites_versions_patch(&self, request: Version, name: &str) -> ProjectSiteVersionPatchCall<'a, S> {
        ProjectSiteVersionPatchCall {
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
    ///  Adds content files to the specified version. Each file must be under 2 GB.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The version to which to add files, in the format: sites/SITE_ID /versions/VERSION_ID
    pub fn sites_versions_populate_files(&self, request: PopulateVersionFilesRequest, parent: &str) -> ProjectSiteVersionPopulateFileCall<'a, S> {
        ProjectSiteVersionPopulateFileCall {
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
    /// Creates a new Hosting Site in the specified parent Firebase project. Note that Hosting sites can take several minutes to propagate through Firebase systems.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The Firebase project in which to create a Hosting site, in the format: projects/PROJECT_IDENTIFIER Refer to the `Site` [`name`](https://firebase.google.com/docs/hosting/../projects#Site.FIELDS.name) field for details about PROJECT_IDENTIFIER values.
    pub fn sites_create(&self, request: Site, parent: &str) -> ProjectSiteCreateCall<'a, S> {
        ProjectSiteCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _site_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified Hosting Site from the specified parent Firebase project.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The fully-qualified resource name for the Hosting site, in the format: projects/PROJECT_IDENTIFIER/sites/SITE_ID Refer to the `Site` [`name`](https://firebase.google.com/docs/hosting/../projects#Site.FIELDS.name) field for details about PROJECT_IDENTIFIER values.
    pub fn sites_delete(&self, name: &str) -> ProjectSiteDeleteCall<'a, S> {
        ProjectSiteDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified Hosting Site.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The fully-qualified resource name for the Hosting site, in the format: projects/PROJECT_IDENTIFIER/sites/SITE_ID Refer to the `Site` [`name`](https://firebase.google.com/docs/hosting/../projects#Site.FIELDS.name) field for details about PROJECT_IDENTIFIER values. Since a SITE_ID is a globally unique identifier, you can also use the unique sub-collection resource access pattern, in the format: projects/-/sites/SITE_ID
    pub fn sites_get(&self, name: &str) -> ProjectSiteGetCall<'a, S> {
        ProjectSiteGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the Hosting metadata for a specific site.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The site for which to get the SiteConfig, in the format: sites/ site-name/config
    pub fn sites_get_config(&self, name: &str) -> ProjectSiteGetConfigCall<'a, S> {
        ProjectSiteGetConfigCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists each Hosting Site associated with the specified parent Firebase project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The Firebase project for which to list sites, in the format: projects/PROJECT_IDENTIFIER Refer to the `Site` [`name`](https://firebase.google.com/docs/hosting/../projects#Site.FIELDS.name) field for details about PROJECT_IDENTIFIER values.
    pub fn sites_list(&self, parent: &str) -> ProjectSiteListCall<'a, S> {
        ProjectSiteListCall {
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
    /// Updates attributes of the specified Hosting Site.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The fully-qualified resource name of the Hosting site, in the format: projects/PROJECT_IDENTIFIER/sites/SITE_ID PROJECT_IDENTIFIER: the Firebase project's [`ProjectNumber`](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its [`ProjectId`](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects#FirebaseProject.FIELDS.project_id). Learn more about using project identifiers in Google's [AIP 2510 standard](https://google.aip.dev/cloud/2510).
    pub fn sites_patch(&self, request: Site, name: &str) -> ProjectSitePatchCall<'a, S> {
        ProjectSitePatchCall {
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
    /// Sets the Hosting metadata for a specific site.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The site for which to update the SiteConfig, in the format: sites/ site-name/config
    pub fn sites_update_config(&self, request: SiteConfig, name: &str) -> ProjectSiteUpdateConfigCall<'a, S> {
        ProjectSiteUpdateConfigCall {
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



/// A builder providing access to all methods supported on *site* resources.
/// It is not used directly, but through the [`FirebaseHosting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_firebasehosting1_beta1 as firebasehosting1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use firebasehosting1_beta1::{FirebaseHosting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = FirebaseHosting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `channels_create(...)`, `channels_delete(...)`, `channels_get(...)`, `channels_list(...)`, `channels_patch(...)`, `channels_releases_create(...)`, `channels_releases_list(...)`, `domains_create(...)`, `domains_delete(...)`, `domains_get(...)`, `domains_list(...)`, `domains_update(...)`, `get_config(...)`, `releases_create(...)`, `releases_list(...)`, `update_config(...)`, `versions_clone(...)`, `versions_create(...)`, `versions_delete(...)`, `versions_files_list(...)`, `versions_list(...)`, `versions_patch(...)` and `versions_populate_files(...)`
/// // to build up your call.
/// let rb = hub.sites();
/// # }
/// ```
pub struct SiteMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a FirebaseHosting<S>,
}

impl<'a, S> client::MethodsBuilder for SiteMethods<'a, S> {}

impl<'a, S> SiteMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new release, which makes the content of the specified version actively display on the appropriate URL(s).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The site or channel to which the release belongs, in either of the following formats: - sites/SITE_ID - sites/SITE_ID/channels/CHANNEL_ID
    pub fn channels_releases_create(&self, request: Release, parent: &str) -> SiteChannelReleaseCreateCall<'a, S> {
        SiteChannelReleaseCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _version_name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the releases that have been created for the specified site or channel. When used to list releases for a site, this list includes releases for both the default `live` channel and any active preview channels for the specified site.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The site or channel for which to list releases, in either of the following formats: - sites/SITE_ID - sites/SITE_ID/channels/CHANNEL_ID 
    pub fn channels_releases_list(&self, parent: &str) -> SiteChannelReleaseListCall<'a, S> {
        SiteChannelReleaseListCall {
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
    /// Creates a new channel in the specified site.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The site in which to create this channel, in the format: sites/ SITE_ID
    pub fn channels_create(&self, request: Channel, parent: &str) -> SiteChannelCreateCall<'a, S> {
        SiteChannelCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _channel_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified channel of the specified site. The `live` channel cannot be deleted.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The fully-qualified resource name for the channel, in the format: sites/SITE_ID/channels/CHANNEL_ID
    pub fn channels_delete(&self, name: &str) -> SiteChannelDeleteCall<'a, S> {
        SiteChannelDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves information for the specified channel of the specified site.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The fully-qualified resource name for the channel, in the format: sites/SITE_ID/channels/CHANNEL_ID
    pub fn channels_get(&self, name: &str) -> SiteChannelGetCall<'a, S> {
        SiteChannelGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the channels for the specified site. All sites have a default `live` channel.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The site for which to list channels, in the format: sites/SITE_ID
    pub fn channels_list(&self, parent: &str) -> SiteChannelListCall<'a, S> {
        SiteChannelListCall {
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
    /// Updates information for the specified channel of the specified site. Implicitly creates the channel if it doesn't already exist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The fully-qualified resource name for the channel, in the format: sites/ SITE_ID/channels/CHANNEL_ID
    pub fn channels_patch(&self, request: Channel, name: &str) -> SiteChannelPatchCall<'a, S> {
        SiteChannelPatchCall {
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
    /// Creates a domain mapping on the specified site.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent to create the domain association for, in the format: sites/site-name
    pub fn domains_create(&self, request: Domain, parent: &str) -> SiteDomainCreateCall<'a, S> {
        SiteDomainCreateCall {
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
    /// Deletes the existing domain mapping on the specified site.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the domain association to delete.
    pub fn domains_delete(&self, name: &str) -> SiteDomainDeleteCall<'a, S> {
        SiteDomainDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a domain mapping on the specified site.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the domain configuration to get.
    pub fn domains_get(&self, name: &str) -> SiteDomainGetCall<'a, S> {
        SiteDomainGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the domains for the specified site.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent for which to list domains, in the format: sites/ site-name
    pub fn domains_list(&self, parent: &str) -> SiteDomainListCall<'a, S> {
        SiteDomainListCall {
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
    /// Updates the specified domain mapping, creating the mapping as if it does not exist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the domain association to update or create, if an association doesn't already exist.
    pub fn domains_update(&self, request: Domain, name: &str) -> SiteDomainUpdateCall<'a, S> {
        SiteDomainUpdateCall {
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
    /// Creates a new release, which makes the content of the specified version actively display on the appropriate URL(s).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The site or channel to which the release belongs, in either of the following formats: - sites/SITE_ID - sites/SITE_ID/channels/CHANNEL_ID
    pub fn releases_create(&self, request: Release, parent: &str) -> SiteReleaseCreateCall<'a, S> {
        SiteReleaseCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _version_name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the releases that have been created for the specified site or channel. When used to list releases for a site, this list includes releases for both the default `live` channel and any active preview channels for the specified site.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The site or channel for which to list releases, in either of the following formats: - sites/SITE_ID - sites/SITE_ID/channels/CHANNEL_ID 
    pub fn releases_list(&self, parent: &str) -> SiteReleaseListCall<'a, S> {
        SiteReleaseListCall {
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
    /// Lists the remaining files to be uploaded for the specified version.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The version for which to list files, in the format: sites/SITE_ID /versions/VERSION_ID
    pub fn versions_files_list(&self, parent: &str) -> SiteVersionFileListCall<'a, S> {
        SiteVersionFileListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _status: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new version on the specified target site using the content of the specified version.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The target site for the cloned version, in the format: sites/ SITE_ID
    pub fn versions_clone(&self, request: CloneVersionRequest, parent: &str) -> SiteVersionCloneCall<'a, S> {
        SiteVersionCloneCall {
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
    /// Creates a new version for the specified site.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The site in which to create the version, in the format: sites/ SITE_ID
    pub fn versions_create(&self, request: Version, parent: &str) -> SiteVersionCreateCall<'a, S> {
        SiteVersionCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _version_id: Default::default(),
            _size_bytes: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified version.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The fully-qualified resource name for the version, in the format: sites/SITE_ID/versions/VERSION_ID
    pub fn versions_delete(&self, name: &str) -> SiteVersionDeleteCall<'a, S> {
        SiteVersionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the versions that have been created for the specified site. This list includes versions for both the default `live` channel and any active preview channels for the specified site.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The site or channel for which to list versions, in either of the following formats: - sites/SITE_ID - sites/SITE_ID/channels/CHANNEL_ID 
    pub fn versions_list(&self, parent: &str) -> SiteVersionListCall<'a, S> {
        SiteVersionListCall {
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
    /// Updates the specified metadata for the specified version. This method will fail with `FAILED_PRECONDITION` in the event of an invalid state transition. The supported [state](https://firebase.google.com/docs/hosting/../sites.versions#versionstatus) transitions for a version are from `CREATED` to `FINALIZED`. Use [`DeleteVersion`](delete) to set the status of a version to `DELETED`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The fully-qualified resource name for the version, in the format: sites/ SITE_ID/versions/VERSION_ID This name is provided in the response body when you call [`CreateVersion`](sites.versions/create).
    pub fn versions_patch(&self, request: Version, name: &str) -> SiteVersionPatchCall<'a, S> {
        SiteVersionPatchCall {
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
    ///  Adds content files to the specified version. Each file must be under 2 GB.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The version to which to add files, in the format: sites/SITE_ID /versions/VERSION_ID
    pub fn versions_populate_files(&self, request: PopulateVersionFilesRequest, parent: &str) -> SiteVersionPopulateFileCall<'a, S> {
        SiteVersionPopulateFileCall {
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
    /// Gets the Hosting metadata for a specific site.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The site for which to get the SiteConfig, in the format: sites/ site-name/config
    pub fn get_config(&self, name: &str) -> SiteGetConfigCall<'a, S> {
        SiteGetConfigCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the Hosting metadata for a specific site.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The site for which to update the SiteConfig, in the format: sites/ site-name/config
    pub fn update_config(&self, request: SiteConfig, name: &str) -> SiteUpdateConfigCall<'a, S> {
        SiteUpdateConfigCall {
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



