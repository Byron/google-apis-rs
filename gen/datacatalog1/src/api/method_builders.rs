use super::*;
/// A builder providing access to all methods supported on *catalog* resources.
/// It is not used directly, but through the [`DataCatalog`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_datacatalog1 as datacatalog1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use datacatalog1::{DataCatalog, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DataCatalog::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `search(...)`
/// // to build up your call.
/// let rb = hub.catalog();
/// # }
/// ```
pub struct CatalogMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DataCatalog<S>,
}

impl<'a, S> client::MethodsBuilder for CatalogMethods<'a, S> {}

impl<'a, S> CatalogMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Searches Data Catalog for multiple resources like entries and tags that match a query. This is a [Custom Method] (https://cloud.google.com/apis/design/custom_methods) that doesn't return all information on a resource, only its ID and high level fields. To get more information, you can subsequently call specific get methods. Note: Data Catalog search queries don't guarantee full recall. Results that match your query might not be returned, even in subsequent result pages. Additionally, returned (and not returned) results can vary if you repeat search queries. For more information, see [Data Catalog search syntax] (https://cloud.google.com/data-catalog/docs/how-to/search-reference).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn search(&self, request: GoogleCloudDatacatalogV1SearchCatalogRequest) -> CatalogSearchCall<'a, S> {
        CatalogSearchCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *entry* resources.
/// It is not used directly, but through the [`DataCatalog`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_datacatalog1 as datacatalog1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use datacatalog1::{DataCatalog, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DataCatalog::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `lookup(...)`
/// // to build up your call.
/// let rb = hub.entries();
/// # }
/// ```
pub struct EntryMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DataCatalog<S>,
}

impl<'a, S> client::MethodsBuilder for EntryMethods<'a, S> {}

impl<'a, S> EntryMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an entry by its target resource name. The resource name comes from the source Google Cloud Platform service.
    pub fn lookup(&self) -> EntryLookupCall<'a, S> {
        EntryLookupCall {
            hub: self.hub,
            _sql_resource: Default::default(),
            _linked_resource: Default::default(),
            _fully_qualified_name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`DataCatalog`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_datacatalog1 as datacatalog1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use datacatalog1::{DataCatalog, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DataCatalog::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_entry_groups_create(...)`, `locations_entry_groups_delete(...)`, `locations_entry_groups_entries_create(...)`, `locations_entry_groups_entries_delete(...)`, `locations_entry_groups_entries_get(...)`, `locations_entry_groups_entries_get_iam_policy(...)`, `locations_entry_groups_entries_import(...)`, `locations_entry_groups_entries_list(...)`, `locations_entry_groups_entries_modify_entry_contacts(...)`, `locations_entry_groups_entries_modify_entry_overview(...)`, `locations_entry_groups_entries_patch(...)`, `locations_entry_groups_entries_star(...)`, `locations_entry_groups_entries_tags_create(...)`, `locations_entry_groups_entries_tags_delete(...)`, `locations_entry_groups_entries_tags_list(...)`, `locations_entry_groups_entries_tags_patch(...)`, `locations_entry_groups_entries_test_iam_permissions(...)`, `locations_entry_groups_entries_unstar(...)`, `locations_entry_groups_get(...)`, `locations_entry_groups_get_iam_policy(...)`, `locations_entry_groups_list(...)`, `locations_entry_groups_patch(...)`, `locations_entry_groups_set_iam_policy(...)`, `locations_entry_groups_tags_create(...)`, `locations_entry_groups_tags_delete(...)`, `locations_entry_groups_tags_list(...)`, `locations_entry_groups_tags_patch(...)`, `locations_entry_groups_test_iam_permissions(...)`, `locations_operations_cancel(...)`, `locations_operations_delete(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `locations_tag_templates_create(...)`, `locations_tag_templates_delete(...)`, `locations_tag_templates_fields_create(...)`, `locations_tag_templates_fields_delete(...)`, `locations_tag_templates_fields_enum_values_rename(...)`, `locations_tag_templates_fields_patch(...)`, `locations_tag_templates_fields_rename(...)`, `locations_tag_templates_get(...)`, `locations_tag_templates_get_iam_policy(...)`, `locations_tag_templates_patch(...)`, `locations_tag_templates_set_iam_policy(...)`, `locations_tag_templates_test_iam_permissions(...)`, `locations_taxonomies_create(...)`, `locations_taxonomies_delete(...)`, `locations_taxonomies_export(...)`, `locations_taxonomies_get(...)`, `locations_taxonomies_get_iam_policy(...)`, `locations_taxonomies_import(...)`, `locations_taxonomies_list(...)`, `locations_taxonomies_patch(...)`, `locations_taxonomies_policy_tags_create(...)`, `locations_taxonomies_policy_tags_delete(...)`, `locations_taxonomies_policy_tags_get(...)`, `locations_taxonomies_policy_tags_get_iam_policy(...)`, `locations_taxonomies_policy_tags_list(...)`, `locations_taxonomies_policy_tags_patch(...)`, `locations_taxonomies_policy_tags_set_iam_policy(...)`, `locations_taxonomies_policy_tags_test_iam_permissions(...)`, `locations_taxonomies_replace(...)`, `locations_taxonomies_set_iam_policy(...)` and `locations_taxonomies_test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DataCatalog<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a tag and assigns it to: * An Entry if the method name is `projects.locations.entryGroups.entries.tags.create`. * Or EntryGroupif the method name is `projects.locations.entryGroups.tags.create`. Note: The project identified by the `parent` parameter for the [tag] (https://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.entryGroups.entries.tags/create#path-parameters) and the [tag template] (https://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.tagTemplates/create#path-parameters) used to create the tag must be in the same organization.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the resource to attach this tag to. Tags can be attached to entries or entry groups. An entry can have up to 1000 attached tags. Note: The tag and its child resources might not be stored in the location specified in its name.
    pub fn locations_entry_groups_entries_tags_create(&self, request: GoogleCloudDatacatalogV1Tag, parent: &str) -> ProjectLocationEntryGroupEntryTagCreateCall<'a, S> {
        ProjectLocationEntryGroupEntryTagCreateCall {
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
    /// Deletes a tag.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the tag to delete.
    pub fn locations_entry_groups_entries_tags_delete(&self, name: &str) -> ProjectLocationEntryGroupEntryTagDeleteCall<'a, S> {
        ProjectLocationEntryGroupEntryTagDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists tags assigned to an Entry. The columns in the response are lowercased.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the Data Catalog resource to list the tags of. The resource can be an Entry or an EntryGroup (without `/entries/{entries}` at the end).
    pub fn locations_entry_groups_entries_tags_list(&self, parent: &str) -> ProjectLocationEntryGroupEntryTagListCall<'a, S> {
        ProjectLocationEntryGroupEntryTagListCall {
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
    /// Updates an existing tag.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the tag in URL format where tag ID is a system-generated identifier. Note: The tag itself might not be stored in the location specified in its name.
    pub fn locations_entry_groups_entries_tags_patch(&self, request: GoogleCloudDatacatalogV1Tag, name: &str) -> ProjectLocationEntryGroupEntryTagPatchCall<'a, S> {
        ProjectLocationEntryGroupEntryTagPatchCall {
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
    /// Creates an entry. You can create entries only with 'FILESET', 'CLUSTER', 'DATA_STREAM', or custom types. Data Catalog automatically creates entries with other types during metadata ingestion from integrated systems. You must enable the Data Catalog API in the project identified by the `parent` parameter. For more information, see [Data Catalog resource project](https://cloud.google.com/data-catalog/docs/concepts/resource-project). An entry group can have a maximum of 100,000 entries.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the entry group this entry belongs to. Note: The entry itself and its child resources might not be stored in the location specified in its name.
    pub fn locations_entry_groups_entries_create(&self, request: GoogleCloudDatacatalogV1Entry, parent: &str) -> ProjectLocationEntryGroupEntryCreateCall<'a, S> {
        ProjectLocationEntryGroupEntryCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _entry_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing entry. You can delete only the entries created by the CreateEntry method. You must enable the Data Catalog API in the project identified by the `name` parameter. For more information, see [Data Catalog resource project](https://cloud.google.com/data-catalog/docs/concepts/resource-project).
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the entry to delete.
    pub fn locations_entry_groups_entries_delete(&self, name: &str) -> ProjectLocationEntryGroupEntryDeleteCall<'a, S> {
        ProjectLocationEntryGroupEntryDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an entry.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the entry to get.
    pub fn locations_entry_groups_entries_get(&self, name: &str) -> ProjectLocationEntryGroupEntryGetCall<'a, S> {
        ProjectLocationEntryGroupEntryGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. May return: * A`NOT_FOUND` error if the resource doesn't exist or you don't have the permission to view it. * An empty policy if the resource exists but doesn't have a set policy. Supported resources are: - Tag templates - Entry groups Note: This method doesn't get policies from Google Cloud Platform resources ingested into Data Catalog. To call this method, you must have the following Google IAM permissions: - `datacatalog.tagTemplates.getIamPolicy` to get policies on tag templates. - `datacatalog.entryGroups.getIamPolicy` to get policies on entry groups.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_entry_groups_entries_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectLocationEntryGroupEntryGetIamPolicyCall<'a, S> {
        ProjectLocationEntryGroupEntryGetIamPolicyCall {
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
    /// Imports entries from some source (e.g. dump in a Cloud Storage bucket) to the Data Catalog. Dump here is a snapshot of the third-party system state, that needs to be ingested in the Data Catalog. Import of entries is a sync operation that reconciles state of the third-party system and Data Catalog. ImportEntries is a long-running operation done in the background, so this method returns long-running operation resource. The resource can be queried with Operations.GetOperation which contains metadata and response.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Target entry group for ingested entries.
    pub fn locations_entry_groups_entries_import(&self, request: GoogleCloudDatacatalogV1ImportEntriesRequest, parent: &str) -> ProjectLocationEntryGroupEntryImportCall<'a, S> {
        ProjectLocationEntryGroupEntryImportCall {
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
    /// Lists entries. Note: Currently, this method can list only custom entries. To get a list of both custom and automatically created entries, use SearchCatalog.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the entry group that contains the entries to list. Can be provided in URL format.
    pub fn locations_entry_groups_entries_list(&self, parent: &str) -> ProjectLocationEntryGroupEntryListCall<'a, S> {
        ProjectLocationEntryGroupEntryListCall {
            hub: self.hub,
            _parent: parent.to_string(),
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
    /// Modifies contacts, part of the business context of an Entry. To call this method, you must have the `datacatalog.entries.updateContacts` IAM permission on the corresponding project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The full resource name of the entry.
    pub fn locations_entry_groups_entries_modify_entry_contacts(&self, request: GoogleCloudDatacatalogV1ModifyEntryContactsRequest, name: &str) -> ProjectLocationEntryGroupEntryModifyEntryContactCall<'a, S> {
        ProjectLocationEntryGroupEntryModifyEntryContactCall {
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
    /// Modifies entry overview, part of the business context of an Entry. To call this method, you must have the `datacatalog.entries.updateOverview` IAM permission on the corresponding project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The full resource name of the entry.
    pub fn locations_entry_groups_entries_modify_entry_overview(&self, request: GoogleCloudDatacatalogV1ModifyEntryOverviewRequest, name: &str) -> ProjectLocationEntryGroupEntryModifyEntryOverviewCall<'a, S> {
        ProjectLocationEntryGroupEntryModifyEntryOverviewCall {
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
    /// Updates an existing entry. You must enable the Data Catalog API in the project identified by the `entry.name` parameter. For more information, see [Data Catalog resource project](https://cloud.google.com/data-catalog/docs/concepts/resource-project).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name of an entry in URL format. Note: The entry itself and its child resources might not be stored in the location specified in its name.
    pub fn locations_entry_groups_entries_patch(&self, request: GoogleCloudDatacatalogV1Entry, name: &str) -> ProjectLocationEntryGroupEntryPatchCall<'a, S> {
        ProjectLocationEntryGroupEntryPatchCall {
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
    /// Marks an Entry as starred by the current user. Starring information is private to each user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the entry to mark as starred.
    pub fn locations_entry_groups_entries_star(&self, request: GoogleCloudDatacatalogV1StarEntryRequest, name: &str) -> ProjectLocationEntryGroupEntryStarCall<'a, S> {
        ProjectLocationEntryGroupEntryStarCall {
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
    /// Gets your permissions on a resource. Returns an empty set of permissions if the resource doesn't exist. Supported resources are: - Tag templates - Entry groups Note: This method gets policies only within Data Catalog and can't be used to get policies from BigQuery, Pub/Sub, Dataproc Metastore, and any external Google Cloud Platform resources ingested into Data Catalog. No Google IAM permissions are required to call this method.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_entry_groups_entries_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationEntryGroupEntryTestIamPermissionCall<'a, S> {
        ProjectLocationEntryGroupEntryTestIamPermissionCall {
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
    /// Marks an Entry as NOT starred by the current user. Starring information is private to each user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the entry to mark as **not** starred.
    pub fn locations_entry_groups_entries_unstar(&self, request: GoogleCloudDatacatalogV1UnstarEntryRequest, name: &str) -> ProjectLocationEntryGroupEntryUnstarCall<'a, S> {
        ProjectLocationEntryGroupEntryUnstarCall {
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
    /// Creates a tag and assigns it to: * An Entry if the method name is `projects.locations.entryGroups.entries.tags.create`. * Or EntryGroupif the method name is `projects.locations.entryGroups.tags.create`. Note: The project identified by the `parent` parameter for the [tag] (https://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.entryGroups.entries.tags/create#path-parameters) and the [tag template] (https://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.tagTemplates/create#path-parameters) used to create the tag must be in the same organization.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the resource to attach this tag to. Tags can be attached to entries or entry groups. An entry can have up to 1000 attached tags. Note: The tag and its child resources might not be stored in the location specified in its name.
    pub fn locations_entry_groups_tags_create(&self, request: GoogleCloudDatacatalogV1Tag, parent: &str) -> ProjectLocationEntryGroupTagCreateCall<'a, S> {
        ProjectLocationEntryGroupTagCreateCall {
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
    /// Deletes a tag.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the tag to delete.
    pub fn locations_entry_groups_tags_delete(&self, name: &str) -> ProjectLocationEntryGroupTagDeleteCall<'a, S> {
        ProjectLocationEntryGroupTagDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists tags assigned to an Entry. The columns in the response are lowercased.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the Data Catalog resource to list the tags of. The resource can be an Entry or an EntryGroup (without `/entries/{entries}` at the end).
    pub fn locations_entry_groups_tags_list(&self, parent: &str) -> ProjectLocationEntryGroupTagListCall<'a, S> {
        ProjectLocationEntryGroupTagListCall {
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
    /// Updates an existing tag.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the tag in URL format where tag ID is a system-generated identifier. Note: The tag itself might not be stored in the location specified in its name.
    pub fn locations_entry_groups_tags_patch(&self, request: GoogleCloudDatacatalogV1Tag, name: &str) -> ProjectLocationEntryGroupTagPatchCall<'a, S> {
        ProjectLocationEntryGroupTagPatchCall {
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
    /// Creates an entry group. An entry group contains logically related entries together with [Cloud Identity and Access Management](https://cloud.google.com/data-catalog/docs/concepts/iam) policies. These policies specify users who can create, edit, and view entries within entry groups. Data Catalog automatically creates entry groups with names that start with the `@` symbol for the following resources: * BigQuery entries (`@bigquery`) * Pub/Sub topics (`@pubsub`) * Dataproc Metastore services (`@dataproc_metastore_{SERVICE_NAME_HASH}`) You can create your own entry groups for Cloud Storage fileset entries and custom entries together with the corresponding IAM policies. User-created entry groups can’t contain the `@` symbol, it is reserved for automatically created groups. Entry groups, like entries, can be searched. A maximum of 10,000 entry groups may be created per organization across all locations. You must enable the Data Catalog API in the project identified by the `parent` parameter. For more information, see [Data Catalog resource project](https://cloud.google.com/data-catalog/docs/concepts/resource-project).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The names of the project and location that the new entry group belongs to. Note: The entry group itself and its child resources might not be stored in the location specified in its name.
    pub fn locations_entry_groups_create(&self, request: GoogleCloudDatacatalogV1EntryGroup, parent: &str) -> ProjectLocationEntryGroupCreateCall<'a, S> {
        ProjectLocationEntryGroupCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _entry_group_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an entry group. You must enable the Data Catalog API in the project identified by the `name` parameter. For more information, see [Data Catalog resource project](https://cloud.google.com/data-catalog/docs/concepts/resource-project).
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the entry group to delete.
    pub fn locations_entry_groups_delete(&self, name: &str) -> ProjectLocationEntryGroupDeleteCall<'a, S> {
        ProjectLocationEntryGroupDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _force: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an entry group.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the entry group to get.
    pub fn locations_entry_groups_get(&self, name: &str) -> ProjectLocationEntryGroupGetCall<'a, S> {
        ProjectLocationEntryGroupGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _read_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. May return: * A`NOT_FOUND` error if the resource doesn't exist or you don't have the permission to view it. * An empty policy if the resource exists but doesn't have a set policy. Supported resources are: - Tag templates - Entry groups Note: This method doesn't get policies from Google Cloud Platform resources ingested into Data Catalog. To call this method, you must have the following Google IAM permissions: - `datacatalog.tagTemplates.getIamPolicy` to get policies on tag templates. - `datacatalog.entryGroups.getIamPolicy` to get policies on entry groups.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_entry_groups_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectLocationEntryGroupGetIamPolicyCall<'a, S> {
        ProjectLocationEntryGroupGetIamPolicyCall {
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
    /// Lists entry groups.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the location that contains the entry groups to list. Can be provided as a URL.
    pub fn locations_entry_groups_list(&self, parent: &str) -> ProjectLocationEntryGroupListCall<'a, S> {
        ProjectLocationEntryGroupListCall {
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
    /// Updates an entry group. You must enable the Data Catalog API in the project identified by the `entry_group.name` parameter. For more information, see [Data Catalog resource project](https://cloud.google.com/data-catalog/docs/concepts/resource-project).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the entry group in URL format. Note: The entry group itself and its child resources might not be stored in the location specified in its name.
    pub fn locations_entry_groups_patch(&self, request: GoogleCloudDatacatalogV1EntryGroup, name: &str) -> ProjectLocationEntryGroupPatchCall<'a, S> {
        ProjectLocationEntryGroupPatchCall {
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
    /// Sets an access control policy for a resource. Replaces any existing policy. Supported resources are: - Tag templates - Entry groups Note: This method sets policies only within Data Catalog and can't be used to manage policies in BigQuery, Pub/Sub, Dataproc Metastore, and any external Google Cloud Platform resources synced with the Data Catalog. To call this method, you must have the following Google IAM permissions: - `datacatalog.tagTemplates.setIamPolicy` to set policies on tag templates. - `datacatalog.entryGroups.setIamPolicy` to set policies on entry groups.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_entry_groups_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationEntryGroupSetIamPolicyCall<'a, S> {
        ProjectLocationEntryGroupSetIamPolicyCall {
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
    /// Gets your permissions on a resource. Returns an empty set of permissions if the resource doesn't exist. Supported resources are: - Tag templates - Entry groups Note: This method gets policies only within Data Catalog and can't be used to get policies from BigQuery, Pub/Sub, Dataproc Metastore, and any external Google Cloud Platform resources ingested into Data Catalog. No Google IAM permissions are required to call this method.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_entry_groups_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationEntryGroupTestIamPermissionCall<'a, S> {
        ProjectLocationEntryGroupTestIamPermissionCall {
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
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn locations_operations_cancel(&self, name: &str) -> ProjectLocationOperationCancelCall<'a, S> {
        ProjectLocationOperationCancelCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be deleted.
    pub fn locations_operations_delete(&self, name: &str) -> ProjectLocationOperationDeleteCall<'a, S> {
        ProjectLocationOperationDeleteCall {
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
    pub fn locations_operations_get(&self, name: &str) -> ProjectLocationOperationGetCall<'a, S> {
        ProjectLocationOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `"/v1/{name=users/*}/operations"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn locations_operations_list(&self, name: &str) -> ProjectLocationOperationListCall<'a, S> {
        ProjectLocationOperationListCall {
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
    /// Renames an enum value in a tag template. Within a single enum field, enum values must be unique.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the enum field value.
    pub fn locations_tag_templates_fields_enum_values_rename(&self, request: GoogleCloudDatacatalogV1RenameTagTemplateFieldEnumValueRequest, name: &str) -> ProjectLocationTagTemplateFieldEnumValueRenameCall<'a, S> {
        ProjectLocationTagTemplateFieldEnumValueRenameCall {
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
    /// Creates a field in a tag template. You must enable the Data Catalog API in the project identified by the `parent` parameter. For more information, see [Data Catalog resource project](https://cloud.google.com/data-catalog/docs/concepts/resource-project).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the project and the template location [region](https://cloud.google.com/data-catalog/docs/concepts/regions).
    pub fn locations_tag_templates_fields_create(&self, request: GoogleCloudDatacatalogV1TagTemplateField, parent: &str) -> ProjectLocationTagTemplateFieldCreateCall<'a, S> {
        ProjectLocationTagTemplateFieldCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _tag_template_field_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a field in a tag template and all uses of this field from the tags based on this template. You must enable the Data Catalog API in the project identified by the `name` parameter. For more information, see [Data Catalog resource project](https://cloud.google.com/data-catalog/docs/concepts/resource-project).
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the tag template field to delete.
    pub fn locations_tag_templates_fields_delete(&self, name: &str) -> ProjectLocationTagTemplateFieldDeleteCall<'a, S> {
        ProjectLocationTagTemplateFieldDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _force: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a field in a tag template. You can't update the field type with this method. You must enable the Data Catalog API in the project identified by the `name` parameter. For more information, see [Data Catalog resource project](https://cloud.google.com/data-catalog/docs/concepts/resource-project).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the tag template field.
    pub fn locations_tag_templates_fields_patch(&self, request: GoogleCloudDatacatalogV1TagTemplateField, name: &str) -> ProjectLocationTagTemplateFieldPatchCall<'a, S> {
        ProjectLocationTagTemplateFieldPatchCall {
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
    /// Renames a field in a tag template. You must enable the Data Catalog API in the project identified by the `name` parameter. For more information, see [Data Catalog resource project] (https://cloud.google.com/data-catalog/docs/concepts/resource-project).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the tag template field.
    pub fn locations_tag_templates_fields_rename(&self, request: GoogleCloudDatacatalogV1RenameTagTemplateFieldRequest, name: &str) -> ProjectLocationTagTemplateFieldRenameCall<'a, S> {
        ProjectLocationTagTemplateFieldRenameCall {
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
    /// Creates a tag template. You must enable the Data Catalog API in the project identified by the `parent` parameter. For more information, see [Data Catalog resource project] (https://cloud.google.com/data-catalog/docs/concepts/resource-project).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the project and the template location [region](https://cloud.google.com/data-catalog/docs/concepts/regions).
    pub fn locations_tag_templates_create(&self, request: GoogleCloudDatacatalogV1TagTemplate, parent: &str) -> ProjectLocationTagTemplateCreateCall<'a, S> {
        ProjectLocationTagTemplateCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _tag_template_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a tag template and all tags that use it. You must enable the Data Catalog API in the project identified by the `name` parameter. For more information, see [Data Catalog resource project](https://cloud.google.com/data-catalog/docs/concepts/resource-project).
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the tag template to delete.
    pub fn locations_tag_templates_delete(&self, name: &str) -> ProjectLocationTagTemplateDeleteCall<'a, S> {
        ProjectLocationTagTemplateDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _force: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a tag template.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the tag template to get.
    pub fn locations_tag_templates_get(&self, name: &str) -> ProjectLocationTagTemplateGetCall<'a, S> {
        ProjectLocationTagTemplateGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. May return: * A`NOT_FOUND` error if the resource doesn't exist or you don't have the permission to view it. * An empty policy if the resource exists but doesn't have a set policy. Supported resources are: - Tag templates - Entry groups Note: This method doesn't get policies from Google Cloud Platform resources ingested into Data Catalog. To call this method, you must have the following Google IAM permissions: - `datacatalog.tagTemplates.getIamPolicy` to get policies on tag templates. - `datacatalog.entryGroups.getIamPolicy` to get policies on entry groups.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_tag_templates_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectLocationTagTemplateGetIamPolicyCall<'a, S> {
        ProjectLocationTagTemplateGetIamPolicyCall {
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
    /// Updates a tag template. You can't update template fields with this method. These fields are separate resources with their own create, update, and delete methods. You must enable the Data Catalog API in the project identified by the `tag_template.name` parameter. For more information, see [Data Catalog resource project](https://cloud.google.com/data-catalog/docs/concepts/resource-project).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the tag template in URL format. Note: The tag template itself and its child resources might not be stored in the location specified in its name.
    pub fn locations_tag_templates_patch(&self, request: GoogleCloudDatacatalogV1TagTemplate, name: &str) -> ProjectLocationTagTemplatePatchCall<'a, S> {
        ProjectLocationTagTemplatePatchCall {
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
    /// Sets an access control policy for a resource. Replaces any existing policy. Supported resources are: - Tag templates - Entry groups Note: This method sets policies only within Data Catalog and can't be used to manage policies in BigQuery, Pub/Sub, Dataproc Metastore, and any external Google Cloud Platform resources synced with the Data Catalog. To call this method, you must have the following Google IAM permissions: - `datacatalog.tagTemplates.setIamPolicy` to set policies on tag templates. - `datacatalog.entryGroups.setIamPolicy` to set policies on entry groups.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_tag_templates_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationTagTemplateSetIamPolicyCall<'a, S> {
        ProjectLocationTagTemplateSetIamPolicyCall {
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
    /// Gets your permissions on a resource. Returns an empty set of permissions if the resource doesn't exist. Supported resources are: - Tag templates - Entry groups Note: This method gets policies only within Data Catalog and can't be used to get policies from BigQuery, Pub/Sub, Dataproc Metastore, and any external Google Cloud Platform resources ingested into Data Catalog. No Google IAM permissions are required to call this method.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_tag_templates_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationTagTemplateTestIamPermissionCall<'a, S> {
        ProjectLocationTagTemplateTestIamPermissionCall {
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
    /// Creates a policy tag in a taxonomy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the taxonomy that the policy tag will belong to.
    pub fn locations_taxonomies_policy_tags_create(&self, request: GoogleCloudDatacatalogV1PolicyTag, parent: &str) -> ProjectLocationTaxonomyPolicyTagCreateCall<'a, S> {
        ProjectLocationTaxonomyPolicyTagCreateCall {
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
    /// Deletes a policy tag together with the following: * All of its descendant policy tags, if any * Policies associated with the policy tag and its descendants * References from BigQuery table schema of the policy tag and its descendants
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the policy tag to delete. Note: All of its descendant policy tags are also deleted.
    pub fn locations_taxonomies_policy_tags_delete(&self, name: &str) -> ProjectLocationTaxonomyPolicyTagDeleteCall<'a, S> {
        ProjectLocationTaxonomyPolicyTagDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a policy tag.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the policy tag.
    pub fn locations_taxonomies_policy_tags_get(&self, name: &str) -> ProjectLocationTaxonomyPolicyTagGetCall<'a, S> {
        ProjectLocationTaxonomyPolicyTagGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the IAM policy for a policy tag or a taxonomy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_taxonomies_policy_tags_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectLocationTaxonomyPolicyTagGetIamPolicyCall<'a, S> {
        ProjectLocationTaxonomyPolicyTagGetIamPolicyCall {
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
    /// Lists all policy tags in a taxonomy.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name of the taxonomy to list the policy tags of.
    pub fn locations_taxonomies_policy_tags_list(&self, parent: &str) -> ProjectLocationTaxonomyPolicyTagListCall<'a, S> {
        ProjectLocationTaxonomyPolicyTagListCall {
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
    /// Updates a policy tag, including its display name, description, and parent policy tag.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name of this policy tag in the URL format. The policy tag manager generates unique taxonomy IDs and policy tag IDs.
    pub fn locations_taxonomies_policy_tags_patch(&self, request: GoogleCloudDatacatalogV1PolicyTag, name: &str) -> ProjectLocationTaxonomyPolicyTagPatchCall<'a, S> {
        ProjectLocationTaxonomyPolicyTagPatchCall {
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
    /// Sets the IAM policy for a policy tag or a taxonomy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_taxonomies_policy_tags_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationTaxonomyPolicyTagSetIamPolicyCall<'a, S> {
        ProjectLocationTaxonomyPolicyTagSetIamPolicyCall {
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
    /// Returns your permissions on a specified policy tag or taxonomy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_taxonomies_policy_tags_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationTaxonomyPolicyTagTestIamPermissionCall<'a, S> {
        ProjectLocationTaxonomyPolicyTagTestIamPermissionCall {
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
    /// Creates a taxonomy in a specified project. The taxonomy is initially empty, that is, it doesn't contain policy tags.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the project that the taxonomy will belong to.
    pub fn locations_taxonomies_create(&self, request: GoogleCloudDatacatalogV1Taxonomy, parent: &str) -> ProjectLocationTaxonomyCreateCall<'a, S> {
        ProjectLocationTaxonomyCreateCall {
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
    /// Deletes a taxonomy, including all policy tags in this taxonomy, their associated policies, and the policy tags references from BigQuery columns.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the taxonomy to delete. Note: All policy tags in this taxonomy are also deleted.
    pub fn locations_taxonomies_delete(&self, name: &str) -> ProjectLocationTaxonomyDeleteCall<'a, S> {
        ProjectLocationTaxonomyDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Exports taxonomies in the requested type and returns them, including their policy tags. The requested taxonomies must belong to the same project. This method generates `SerializedTaxonomy` protocol buffers with nested policy tags that can be used as input for `ImportTaxonomies` calls.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name of the project that the exported taxonomies belong to.
    pub fn locations_taxonomies_export(&self, parent: &str) -> ProjectLocationTaxonomyExportCall<'a, S> {
        ProjectLocationTaxonomyExportCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _taxonomies: Default::default(),
            _serialized_taxonomies: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a taxonomy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the taxonomy to get.
    pub fn locations_taxonomies_get(&self, name: &str) -> ProjectLocationTaxonomyGetCall<'a, S> {
        ProjectLocationTaxonomyGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the IAM policy for a policy tag or a taxonomy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_taxonomies_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectLocationTaxonomyGetIamPolicyCall<'a, S> {
        ProjectLocationTaxonomyGetIamPolicyCall {
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
    /// Creates new taxonomies (including their policy tags) in a given project by importing from inlined or cross-regional sources. For a cross-regional source, new taxonomies are created by copying from a source in another region. For an inlined source, taxonomies and policy tags are created in bulk using nested protocol buffer structures.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of project that the imported taxonomies will belong to.
    pub fn locations_taxonomies_import(&self, request: GoogleCloudDatacatalogV1ImportTaxonomiesRequest, parent: &str) -> ProjectLocationTaxonomyImportCall<'a, S> {
        ProjectLocationTaxonomyImportCall {
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
    /// Lists all taxonomies in a project in a particular location that you have a permission to view.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name of the project to list the taxonomies of.
    pub fn locations_taxonomies_list(&self, parent: &str) -> ProjectLocationTaxonomyListCall<'a, S> {
        ProjectLocationTaxonomyListCall {
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
    /// Updates a taxonomy, including its display name, description, and activated policy types.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name of this taxonomy in URL format. Note: Policy tag manager generates unique taxonomy IDs.
    pub fn locations_taxonomies_patch(&self, request: GoogleCloudDatacatalogV1Taxonomy, name: &str) -> ProjectLocationTaxonomyPatchCall<'a, S> {
        ProjectLocationTaxonomyPatchCall {
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
    /// Replaces (updates) a taxonomy and all its policy tags. The taxonomy and its entire hierarchy of policy tags must be represented literally by `SerializedTaxonomy` and the nested `SerializedPolicyTag` messages. This operation automatically does the following: - Deletes the existing policy tags that are missing from the `SerializedPolicyTag`. - Creates policy tags that don't have resource names. They are considered new. - Updates policy tags with valid resources names accordingly.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name of the taxonomy to update.
    pub fn locations_taxonomies_replace(&self, request: GoogleCloudDatacatalogV1ReplaceTaxonomyRequest, name: &str) -> ProjectLocationTaxonomyReplaceCall<'a, S> {
        ProjectLocationTaxonomyReplaceCall {
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
    /// Sets the IAM policy for a policy tag or a taxonomy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_taxonomies_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationTaxonomySetIamPolicyCall<'a, S> {
        ProjectLocationTaxonomySetIamPolicyCall {
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
    /// Returns your permissions on a specified policy tag or taxonomy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_taxonomies_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationTaxonomyTestIamPermissionCall<'a, S> {
        ProjectLocationTaxonomyTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



