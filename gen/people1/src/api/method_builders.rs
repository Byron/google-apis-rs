use super::*;
/// A builder providing access to all methods supported on *contactGroup* resources.
/// It is not used directly, but through the [`PeopleService`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_people1 as people1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use people1::{PeopleService, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PeopleService::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `batch_get(...)`, `create(...)`, `delete(...)`, `get(...)`, `list(...)`, `members_modify(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.contact_groups();
/// # }
/// ```
pub struct ContactGroupMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a PeopleService<S>,
}

impl<'a, S> client::MethodsBuilder for ContactGroupMethods<'a, S> {}

impl<'a, S> ContactGroupMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modify the members of a contact group owned by the authenticated user. The only system contact groups that can have members added are `contactGroups/myContacts` and `contactGroups/starred`. Other system contact groups are deprecated and can only have contacts removed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceName` - Required. The resource name of the contact group to modify.
    pub fn members_modify(&self, request: ModifyContactGroupMembersRequest, resource_name: &str) -> ContactGroupMemberModifyCall<'a, S> {
        ContactGroupMemberModifyCall {
            hub: self.hub,
            _request: request,
            _resource_name: resource_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a list of contact groups owned by the authenticated user by specifying a list of contact group resource names.
    pub fn batch_get(&self) -> ContactGroupBatchGetCall<'a, S> {
        ContactGroupBatchGetCall {
            hub: self.hub,
            _resource_names: Default::default(),
            _max_members: Default::default(),
            _group_fields: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a new contact group owned by the authenticated user. Created contact group names must be unique to the users contact groups. Attempting to create a group with a duplicate name will return a HTTP 409 error. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: CreateContactGroupRequest) -> ContactGroupCreateCall<'a, S> {
        ContactGroupCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete an existing contact group owned by the authenticated user by specifying a contact group resource name. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.
    /// 
    /// # Arguments
    ///
    /// * `resourceName` - Required. The resource name of the contact group to delete.
    pub fn delete(&self, resource_name: &str) -> ContactGroupDeleteCall<'a, S> {
        ContactGroupDeleteCall {
            hub: self.hub,
            _resource_name: resource_name.to_string(),
            _delete_contacts: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a specific contact group owned by the authenticated user by specifying a contact group resource name.
    /// 
    /// # Arguments
    ///
    /// * `resourceName` - Required. The resource name of the contact group to get.
    pub fn get(&self, resource_name: &str) -> ContactGroupGetCall<'a, S> {
        ContactGroupGetCall {
            hub: self.hub,
            _resource_name: resource_name.to_string(),
            _max_members: Default::default(),
            _group_fields: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all contact groups owned by the authenticated user. Members of the contact groups are not populated.
    pub fn list(&self) -> ContactGroupListCall<'a, S> {
        ContactGroupListCall {
            hub: self.hub,
            _sync_token: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _group_fields: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update the name of an existing contact group owned by the authenticated user. Updated contact group names must be unique to the users contact groups. Attempting to create a group with a duplicate name will return a HTTP 409 error. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceName` - The resource name for the contact group, assigned by the server. An ASCII string, in the form of `contactGroups/{contact_group_id}`.
    pub fn update(&self, request: UpdateContactGroupRequest, resource_name: &str) -> ContactGroupUpdateCall<'a, S> {
        ContactGroupUpdateCall {
            hub: self.hub,
            _request: request,
            _resource_name: resource_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *otherContact* resources.
/// It is not used directly, but through the [`PeopleService`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_people1 as people1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use people1::{PeopleService, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PeopleService::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `copy_other_contact_to_my_contacts_group(...)`, `list(...)` and `search(...)`
/// // to build up your call.
/// let rb = hub.other_contacts();
/// # }
/// ```
pub struct OtherContactMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a PeopleService<S>,
}

impl<'a, S> client::MethodsBuilder for OtherContactMethods<'a, S> {}

impl<'a, S> OtherContactMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Copies an "Other contact" to a new contact in the user's "myContacts" group Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceName` - Required. The resource name of the "Other contact" to copy.
    pub fn copy_other_contact_to_my_contacts_group(&self, request: CopyOtherContactToMyContactsGroupRequest, resource_name: &str) -> OtherContactCopyOtherContactToMyContactsGroupCall<'a, S> {
        OtherContactCopyOtherContactToMyContactsGroupCall {
            hub: self.hub,
            _request: request,
            _resource_name: resource_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all “Other contacts”, that is contacts that are not in a contact group. “Other contacts” are typically auto created contacts from interactions. Sync tokens expire 7 days after the full sync. A request with an expired sync token will get an error with an [google.rpc.ErrorInfo](https://cloud.google.com/apis/design/errors#error_info) with reason “EXPIRED_SYNC_TOKEN”. In the case of such an error clients should make a full sync request without a `sync_token`. The first page of a full sync request has an additional quota. If the quota is exceeded, a 429 error will be returned. This quota is fixed and can not be increased. When the `sync_token` is specified, resources deleted since the last sync will be returned as a person with `PersonMetadata.deleted` set to true. When the `page_token` or `sync_token` is specified, all other request parameters must match the first call. Writes may have a propagation delay of several minutes for sync requests. Incremental syncs are not intended for read-after-write use cases. See example usage at [List the user’s other contacts that have changed](https://developers.google.com/people/v1/other-contacts#list_the_users_other_contacts_that_have_changed).
    pub fn list(&self) -> OtherContactListCall<'a, S> {
        OtherContactListCall {
            hub: self.hub,
            _sync_token: Default::default(),
            _sources: Default::default(),
            _request_sync_token: Default::default(),
            _read_mask: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Provides a list of contacts in the authenticated user's other contacts that matches the search query. The query matches on a contact's `names`, `emailAddresses`, and `phoneNumbers` fields that are from the OTHER_CONTACT source. **IMPORTANT**: Before searching, clients should send a warmup request with an empty query to update the cache. See https://developers.google.com/people/v1/other-contacts#search_the_users_other_contacts
    pub fn search(&self) -> OtherContactSearchCall<'a, S> {
        OtherContactSearchCall {
            hub: self.hub,
            _read_mask: Default::default(),
            _query: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *person* resources.
/// It is not used directly, but through the [`PeopleService`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_people1 as people1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use people1::{PeopleService, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PeopleService::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `batch_create_contacts(...)`, `batch_delete_contacts(...)`, `batch_update_contacts(...)`, `connections_list(...)`, `create_contact(...)`, `delete_contact(...)`, `delete_contact_photo(...)`, `get(...)`, `get_batch_get(...)`, `list_directory_people(...)`, `search_contacts(...)`, `search_directory_people(...)`, `update_contact(...)` and `update_contact_photo(...)`
/// // to build up your call.
/// let rb = hub.people();
/// # }
/// ```
pub struct PersonMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a PeopleService<S>,
}

impl<'a, S> client::MethodsBuilder for PersonMethods<'a, S> {}

impl<'a, S> PersonMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Provides a list of the authenticated user’s contacts. Sync tokens expire 7 days after the full sync. A request with an expired sync token will get an error with an [google.rpc.ErrorInfo](https://cloud.google.com/apis/design/errors#error_info) with reason “EXPIRED_SYNC_TOKEN”. In the case of such an error clients should make a full sync request without a `sync_token`. The first page of a full sync request has an additional quota. If the quota is exceeded, a 429 error will be returned. This quota is fixed and can not be increased. When the `sync_token` is specified, resources deleted since the last sync will be returned as a person with `PersonMetadata.deleted` set to true. When the `page_token` or `sync_token` is specified, all other request parameters must match the first call. Writes may have a propagation delay of several minutes for sync requests. Incremental syncs are not intended for read-after-write use cases. See example usage at [List the user’s contacts that have changed](https://developers.google.com/people/v1/contacts#list_the_users_contacts_that_have_changed).
    /// 
    /// # Arguments
    ///
    /// * `resourceName` - Required. The resource name to return connections for. Only `people/me` is valid.
    pub fn connections_list(&self, resource_name: &str) -> PersonConnectionListCall<'a, S> {
        PersonConnectionListCall {
            hub: self.hub,
            _resource_name: resource_name.to_string(),
            _sync_token: Default::default(),
            _sources: Default::default(),
            _sort_order: Default::default(),
            _request_sync_token: Default::default(),
            _request_mask_include_field: Default::default(),
            _person_fields: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a batch of new contacts and return the PersonResponses for the newly Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn batch_create_contacts(&self, request: BatchCreateContactsRequest) -> PersonBatchCreateContactCall<'a, S> {
        PersonBatchCreateContactCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete a batch of contacts. Any non-contact data will not be deleted. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn batch_delete_contacts(&self, request: BatchDeleteContactsRequest) -> PersonBatchDeleteContactCall<'a, S> {
        PersonBatchDeleteContactCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update a batch of contacts and return a map of resource names to PersonResponses for the updated contacts. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn batch_update_contacts(&self, request: BatchUpdateContactsRequest) -> PersonBatchUpdateContactCall<'a, S> {
        PersonBatchUpdateContactCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a new contact and return the person resource for that contact. The request returns a 400 error if more than one field is specified on a field that is a singleton for contact sources: * biographies * birthdays * genders * names Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create_contact(&self, request: Person) -> PersonCreateContactCall<'a, S> {
        PersonCreateContactCall {
            hub: self.hub,
            _request: request,
            _sources: Default::default(),
            _person_fields: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete a contact person. Any non-contact data will not be deleted. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.
    /// 
    /// # Arguments
    ///
    /// * `resourceName` - Required. The resource name of the contact to delete.
    pub fn delete_contact(&self, resource_name: &str) -> PersonDeleteContactCall<'a, S> {
        PersonDeleteContactCall {
            hub: self.hub,
            _resource_name: resource_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete a contact's photo. Mutate requests for the same user should be done sequentially to avoid // lock contention.
    /// 
    /// # Arguments
    ///
    /// * `resourceName` - Required. The resource name of the contact whose photo will be deleted.
    pub fn delete_contact_photo(&self, resource_name: &str) -> PersonDeleteContactPhotoCall<'a, S> {
        PersonDeleteContactPhotoCall {
            hub: self.hub,
            _resource_name: resource_name.to_string(),
            _sources: Default::default(),
            _person_fields: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Provides information about a person by specifying a resource name. Use `people/me` to indicate the authenticated user. The request returns a 400 error if 'personFields' is not specified.
    /// 
    /// # Arguments
    ///
    /// * `resourceName` - Required. The resource name of the person to provide information about. - To get information about the authenticated user, specify `people/me`. - To get information about a google account, specify `people/{account_id}`. - To get information about a contact, specify the resource name that identifies the contact as returned by `people.connections.list`.
    pub fn get(&self, resource_name: &str) -> PersonGetCall<'a, S> {
        PersonGetCall {
            hub: self.hub,
            _resource_name: resource_name.to_string(),
            _sources: Default::default(),
            _request_mask_include_field: Default::default(),
            _person_fields: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Provides information about a list of specific people by specifying a list of requested resource names. Use `people/me` to indicate the authenticated user. The request returns a 400 error if 'personFields' is not specified.
    pub fn get_batch_get(&self) -> PersonGetBatchGetCall<'a, S> {
        PersonGetBatchGetCall {
            hub: self.hub,
            _sources: Default::default(),
            _resource_names: Default::default(),
            _request_mask_include_field: Default::default(),
            _person_fields: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Provides a list of domain profiles and domain contacts in the authenticated user’s domain directory. When the `sync_token` is specified, resources deleted since the last sync will be returned as a person with `PersonMetadata.deleted` set to true. When the `page_token` or `sync_token` is specified, all other request parameters must match the first call. Writes may have a propagation delay of several minutes for sync requests. Incremental syncs are not intended for read-after-write use cases. See example usage at [List the directory people that have changed](https://developers.google.com/people/v1/directory#list_the_directory_people_that_have_changed).
    pub fn list_directory_people(&self) -> PersonListDirectoryPersonCall<'a, S> {
        PersonListDirectoryPersonCall {
            hub: self.hub,
            _sync_token: Default::default(),
            _sources: Default::default(),
            _request_sync_token: Default::default(),
            _read_mask: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _merge_sources: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Provides a list of contacts in the authenticated user's grouped contacts that matches the search query. The query matches on a contact's `names`, `nickNames`, `emailAddresses`, `phoneNumbers`, and `organizations` fields that are from the CONTACT source. **IMPORTANT**: Before searching, clients should send a warmup request with an empty query to update the cache. See https://developers.google.com/people/v1/contacts#search_the_users_contacts
    pub fn search_contacts(&self) -> PersonSearchContactCall<'a, S> {
        PersonSearchContactCall {
            hub: self.hub,
            _sources: Default::default(),
            _read_mask: Default::default(),
            _query: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Provides a list of domain profiles and domain contacts in the authenticated user's domain directory that match the search query.
    pub fn search_directory_people(&self) -> PersonSearchDirectoryPersonCall<'a, S> {
        PersonSearchDirectoryPersonCall {
            hub: self.hub,
            _sources: Default::default(),
            _read_mask: Default::default(),
            _query: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _merge_sources: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update contact data for an existing contact person. Any non-contact data will not be modified. Any non-contact data in the person to update will be ignored. All fields specified in the `update_mask` will be replaced. The server returns a 400 error if `person.metadata.sources` is not specified for the contact to be updated or if there is no contact source. The server returns a 400 error with reason `"failedPrecondition"` if `person.metadata.sources.etag` is different than the contact's etag, which indicates the contact has changed since its data was read. Clients should get the latest person and merge their updates into the latest person. The server returns a 400 error if `memberships` are being updated and there are no contact group memberships specified on the person. The server returns a 400 error if more than one field is specified on a field that is a singleton for contact sources: * biographies * birthdays * genders * names Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceName` - The resource name for the person, assigned by the server. An ASCII string in the form of `people/{person_id}`.
    pub fn update_contact(&self, request: Person, resource_name: &str) -> PersonUpdateContactCall<'a, S> {
        PersonUpdateContactCall {
            hub: self.hub,
            _request: request,
            _resource_name: resource_name.to_string(),
            _update_person_fields: Default::default(),
            _sources: Default::default(),
            _person_fields: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update a contact's photo. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceName` - Required. Person resource name
    pub fn update_contact_photo(&self, request: UpdateContactPhotoRequest, resource_name: &str) -> PersonUpdateContactPhotoCall<'a, S> {
        PersonUpdateContactPhotoCall {
            hub: self.hub,
            _request: request,
            _resource_name: resource_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



