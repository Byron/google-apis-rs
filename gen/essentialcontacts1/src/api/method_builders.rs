use super::*;
/// A builder providing access to all methods supported on *folder* resources.
/// It is not used directly, but through the [`Essentialcontacts`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_essentialcontacts1 as essentialcontacts1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use essentialcontacts1::{Essentialcontacts, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Essentialcontacts::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `contacts_compute(...)`, `contacts_create(...)`, `contacts_delete(...)`, `contacts_get(...)`, `contacts_list(...)`, `contacts_patch(...)` and `contacts_send_test_message(...)`
/// // to build up your call.
/// let rb = hub.folders();
/// # }
/// ```
pub struct FolderMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Essentialcontacts<S>,
}

impl<'a, S> client::MethodsBuilder for FolderMethods<'a, S> {}

impl<'a, S> FolderMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all contacts for the resource that are subscribed to the specified notification categories, including contacts inherited from any parent resources.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the resource to compute contacts for. Format: organizations/{organization_id}, folders/{folder_id} or projects/{project_id}
    pub fn contacts_compute(&self, parent: &str) -> FolderContactComputeCall<'a, S> {
        FolderContactComputeCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _notification_categories: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a new contact for a resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource to save this contact for. Format: organizations/{organization_id}, folders/{folder_id} or projects/{project_id}
    pub fn contacts_create(&self, request: GoogleCloudEssentialcontactsV1Contact, parent: &str) -> FolderContactCreateCall<'a, S> {
        FolderContactCreateCall {
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
    /// Deletes a contact.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the contact to delete. Format: organizations/{organization_id}/contacts/{contact_id}, folders/{folder_id}/contacts/{contact_id} or projects/{project_id}/contacts/{contact_id}
    pub fn contacts_delete(&self, name: &str) -> FolderContactDeleteCall<'a, S> {
        FolderContactDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a single contact.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the contact to retrieve. Format: organizations/{organization_id}/contacts/{contact_id}, folders/{folder_id}/contacts/{contact_id} or projects/{project_id}/contacts/{contact_id}
    pub fn contacts_get(&self, name: &str) -> FolderContactGetCall<'a, S> {
        FolderContactGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the contacts that have been set on a resource.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource name. Format: organizations/{organization_id}, folders/{folder_id} or projects/{project_id}
    pub fn contacts_list(&self, parent: &str) -> FolderContactListCall<'a, S> {
        FolderContactListCall {
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
    /// Updates a contact. Note: A contact's email address cannot be changed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The identifier for the contact. Format: {resource_type}/{resource_id}/contacts/{contact_id}
    pub fn contacts_patch(&self, request: GoogleCloudEssentialcontactsV1Contact, name: &str) -> FolderContactPatchCall<'a, S> {
        FolderContactPatchCall {
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
    /// Allows a contact admin to send a test message to contact to verify that it has been configured correctly.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - Required. The name of the resource to send the test message for. All contacts must either be set directly on this resource or inherited from another resource that is an ancestor of this one. Format: organizations/{organization_id}, folders/{folder_id} or projects/{project_id}
    pub fn contacts_send_test_message(&self, request: GoogleCloudEssentialcontactsV1SendTestMessageRequest, resource: &str) -> FolderContactSendTestMessageCall<'a, S> {
        FolderContactSendTestMessageCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *organization* resources.
/// It is not used directly, but through the [`Essentialcontacts`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_essentialcontacts1 as essentialcontacts1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use essentialcontacts1::{Essentialcontacts, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Essentialcontacts::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `contacts_compute(...)`, `contacts_create(...)`, `contacts_delete(...)`, `contacts_get(...)`, `contacts_list(...)`, `contacts_patch(...)` and `contacts_send_test_message(...)`
/// // to build up your call.
/// let rb = hub.organizations();
/// # }
/// ```
pub struct OrganizationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Essentialcontacts<S>,
}

impl<'a, S> client::MethodsBuilder for OrganizationMethods<'a, S> {}

impl<'a, S> OrganizationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all contacts for the resource that are subscribed to the specified notification categories, including contacts inherited from any parent resources.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the resource to compute contacts for. Format: organizations/{organization_id}, folders/{folder_id} or projects/{project_id}
    pub fn contacts_compute(&self, parent: &str) -> OrganizationContactComputeCall<'a, S> {
        OrganizationContactComputeCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _notification_categories: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a new contact for a resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource to save this contact for. Format: organizations/{organization_id}, folders/{folder_id} or projects/{project_id}
    pub fn contacts_create(&self, request: GoogleCloudEssentialcontactsV1Contact, parent: &str) -> OrganizationContactCreateCall<'a, S> {
        OrganizationContactCreateCall {
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
    /// Deletes a contact.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the contact to delete. Format: organizations/{organization_id}/contacts/{contact_id}, folders/{folder_id}/contacts/{contact_id} or projects/{project_id}/contacts/{contact_id}
    pub fn contacts_delete(&self, name: &str) -> OrganizationContactDeleteCall<'a, S> {
        OrganizationContactDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a single contact.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the contact to retrieve. Format: organizations/{organization_id}/contacts/{contact_id}, folders/{folder_id}/contacts/{contact_id} or projects/{project_id}/contacts/{contact_id}
    pub fn contacts_get(&self, name: &str) -> OrganizationContactGetCall<'a, S> {
        OrganizationContactGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the contacts that have been set on a resource.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource name. Format: organizations/{organization_id}, folders/{folder_id} or projects/{project_id}
    pub fn contacts_list(&self, parent: &str) -> OrganizationContactListCall<'a, S> {
        OrganizationContactListCall {
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
    /// Updates a contact. Note: A contact's email address cannot be changed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The identifier for the contact. Format: {resource_type}/{resource_id}/contacts/{contact_id}
    pub fn contacts_patch(&self, request: GoogleCloudEssentialcontactsV1Contact, name: &str) -> OrganizationContactPatchCall<'a, S> {
        OrganizationContactPatchCall {
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
    /// Allows a contact admin to send a test message to contact to verify that it has been configured correctly.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - Required. The name of the resource to send the test message for. All contacts must either be set directly on this resource or inherited from another resource that is an ancestor of this one. Format: organizations/{organization_id}, folders/{folder_id} or projects/{project_id}
    pub fn contacts_send_test_message(&self, request: GoogleCloudEssentialcontactsV1SendTestMessageRequest, resource: &str) -> OrganizationContactSendTestMessageCall<'a, S> {
        OrganizationContactSendTestMessageCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Essentialcontacts`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_essentialcontacts1 as essentialcontacts1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use essentialcontacts1::{Essentialcontacts, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Essentialcontacts::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `contacts_compute(...)`, `contacts_create(...)`, `contacts_delete(...)`, `contacts_get(...)`, `contacts_list(...)`, `contacts_patch(...)` and `contacts_send_test_message(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Essentialcontacts<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all contacts for the resource that are subscribed to the specified notification categories, including contacts inherited from any parent resources.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the resource to compute contacts for. Format: organizations/{organization_id}, folders/{folder_id} or projects/{project_id}
    pub fn contacts_compute(&self, parent: &str) -> ProjectContactComputeCall<'a, S> {
        ProjectContactComputeCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _notification_categories: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a new contact for a resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource to save this contact for. Format: organizations/{organization_id}, folders/{folder_id} or projects/{project_id}
    pub fn contacts_create(&self, request: GoogleCloudEssentialcontactsV1Contact, parent: &str) -> ProjectContactCreateCall<'a, S> {
        ProjectContactCreateCall {
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
    /// Deletes a contact.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the contact to delete. Format: organizations/{organization_id}/contacts/{contact_id}, folders/{folder_id}/contacts/{contact_id} or projects/{project_id}/contacts/{contact_id}
    pub fn contacts_delete(&self, name: &str) -> ProjectContactDeleteCall<'a, S> {
        ProjectContactDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a single contact.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the contact to retrieve. Format: organizations/{organization_id}/contacts/{contact_id}, folders/{folder_id}/contacts/{contact_id} or projects/{project_id}/contacts/{contact_id}
    pub fn contacts_get(&self, name: &str) -> ProjectContactGetCall<'a, S> {
        ProjectContactGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the contacts that have been set on a resource.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource name. Format: organizations/{organization_id}, folders/{folder_id} or projects/{project_id}
    pub fn contacts_list(&self, parent: &str) -> ProjectContactListCall<'a, S> {
        ProjectContactListCall {
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
    /// Updates a contact. Note: A contact's email address cannot be changed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The identifier for the contact. Format: {resource_type}/{resource_id}/contacts/{contact_id}
    pub fn contacts_patch(&self, request: GoogleCloudEssentialcontactsV1Contact, name: &str) -> ProjectContactPatchCall<'a, S> {
        ProjectContactPatchCall {
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
    /// Allows a contact admin to send a test message to contact to verify that it has been configured correctly.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - Required. The name of the resource to send the test message for. All contacts must either be set directly on this resource or inherited from another resource that is an ancestor of this one. Format: organizations/{organization_id}, folders/{folder_id} or projects/{project_id}
    pub fn contacts_send_test_message(&self, request: GoogleCloudEssentialcontactsV1SendTestMessageRequest, resource: &str) -> ProjectContactSendTestMessageCall<'a, S> {
        ProjectContactSendTestMessageCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



