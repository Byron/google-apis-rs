use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`DataprocMetastore`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_metastore1_beta as metastore1_beta;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use metastore1_beta::{DataprocMetastore, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DataprocMetastore::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_federations_create(...)`, `locations_federations_delete(...)`, `locations_federations_get(...)`, `locations_federations_get_iam_policy(...)`, `locations_federations_list(...)`, `locations_federations_patch(...)`, `locations_federations_set_iam_policy(...)`, `locations_federations_test_iam_permissions(...)`, `locations_get(...)`, `locations_list(...)`, `locations_operations_delete(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `locations_services_alter_location(...)`, `locations_services_backups_create(...)`, `locations_services_backups_delete(...)`, `locations_services_backups_get(...)`, `locations_services_backups_get_iam_policy(...)`, `locations_services_backups_list(...)`, `locations_services_backups_set_iam_policy(...)`, `locations_services_backups_test_iam_permissions(...)`, `locations_services_create(...)`, `locations_services_databases_get_iam_policy(...)`, `locations_services_databases_set_iam_policy(...)`, `locations_services_databases_tables_get_iam_policy(...)`, `locations_services_databases_tables_set_iam_policy(...)`, `locations_services_databases_tables_test_iam_permissions(...)`, `locations_services_databases_test_iam_permissions(...)`, `locations_services_delete(...)`, `locations_services_export_metadata(...)`, `locations_services_get(...)`, `locations_services_get_iam_policy(...)`, `locations_services_list(...)`, `locations_services_metadata_imports_create(...)`, `locations_services_metadata_imports_get(...)`, `locations_services_metadata_imports_list(...)`, `locations_services_metadata_imports_patch(...)`, `locations_services_move_table_to_database(...)`, `locations_services_patch(...)`, `locations_services_query_metadata(...)`, `locations_services_remove_iam_policy(...)`, `locations_services_restore(...)`, `locations_services_set_iam_policy(...)` and `locations_services_test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DataprocMetastore<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a metastore federation in a project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The relative resource name of the location in which to create a federation service, in the following form:projects/{project_number}/locations/{location_id}.
    pub fn locations_federations_create(&self, request: Federation, parent: &str) -> ProjectLocationFederationCreateCall<'a, S> {
        ProjectLocationFederationCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _federation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single federation.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The relative resource name of the metastore federation to delete, in the following form:projects/{project_number}/locations/{location_id}/federations/{federation_id}.
    pub fn locations_federations_delete(&self, name: &str) -> ProjectLocationFederationDeleteCall<'a, S> {
        ProjectLocationFederationDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the details of a single federation.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The relative resource name of the metastore federation to retrieve, in the following form:projects/{project_number}/locations/{location_id}/federations/{federation_id}.
    pub fn locations_federations_get(&self, name: &str) -> ProjectLocationFederationGetCall<'a, S> {
        ProjectLocationFederationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_federations_get_iam_policy(&self, resource: &str) -> ProjectLocationFederationGetIamPolicyCall<'a, S> {
        ProjectLocationFederationGetIamPolicyCall {
            hub: self.hub,
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists federations in a project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The relative resource name of the location of metastore federations to list, in the following form: projects/{project_number}/locations/{location_id}.
    pub fn locations_federations_list(&self, parent: &str) -> ProjectLocationFederationListCall<'a, S> {
        ProjectLocationFederationListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the fields of a federation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The relative resource name of the federation, of the form: projects/{project_number}/locations/{location_id}/federations/{federation_id}`.
    pub fn locations_federations_patch(&self, request: Federation, name: &str) -> ProjectLocationFederationPatchCall<'a, S> {
        ProjectLocationFederationPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_federations_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationFederationSetIamPolicyCall<'a, S> {
        ProjectLocationFederationSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_federations_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationFederationTestIamPermissionCall<'a, S> {
        ProjectLocationFederationTestIamPermissionCall {
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
    /// Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED.
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
    /// Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns UNIMPLEMENTED.NOTE: the name binding allows API services to override the binding to use different resource name schemes, such as users/*/operations. To override the binding, API services can add a binding such as "/v1/{name=users/*}/operations" to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.
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
    /// Creates a new backup in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The relative resource name of the service in which to create a backup of the following form:projects/{project_number}/locations/{location_id}/services/{service_id}.
    pub fn locations_services_backups_create(&self, request: Backup, parent: &str) -> ProjectLocationServiceBackupCreateCall<'a, S> {
        ProjectLocationServiceBackupCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _backup_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single backup.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The relative resource name of the backup to delete, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id}.
    pub fn locations_services_backups_delete(&self, name: &str) -> ProjectLocationServiceBackupDeleteCall<'a, S> {
        ProjectLocationServiceBackupDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single backup.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The relative resource name of the backup to retrieve, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id}.
    pub fn locations_services_backups_get(&self, name: &str) -> ProjectLocationServiceBackupGetCall<'a, S> {
        ProjectLocationServiceBackupGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_services_backups_get_iam_policy(&self, resource: &str) -> ProjectLocationServiceBackupGetIamPolicyCall<'a, S> {
        ProjectLocationServiceBackupGetIamPolicyCall {
            hub: self.hub,
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists backups in a service.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The relative resource name of the service whose backups to list, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/backups.
    pub fn locations_services_backups_list(&self, parent: &str) -> ProjectLocationServiceBackupListCall<'a, S> {
        ProjectLocationServiceBackupListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_services_backups_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationServiceBackupSetIamPolicyCall<'a, S> {
        ProjectLocationServiceBackupSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_services_backups_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationServiceBackupTestIamPermissionCall<'a, S> {
        ProjectLocationServiceBackupTestIamPermissionCall {
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
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_services_databases_tables_get_iam_policy(&self, resource: &str) -> ProjectLocationServiceDatabaseTableGetIamPolicyCall<'a, S> {
        ProjectLocationServiceDatabaseTableGetIamPolicyCall {
            hub: self.hub,
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_services_databases_tables_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationServiceDatabaseTableSetIamPolicyCall<'a, S> {
        ProjectLocationServiceDatabaseTableSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_services_databases_tables_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationServiceDatabaseTableTestIamPermissionCall<'a, S> {
        ProjectLocationServiceDatabaseTableTestIamPermissionCall {
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
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_services_databases_get_iam_policy(&self, resource: &str) -> ProjectLocationServiceDatabaseGetIamPolicyCall<'a, S> {
        ProjectLocationServiceDatabaseGetIamPolicyCall {
            hub: self.hub,
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_services_databases_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationServiceDatabaseSetIamPolicyCall<'a, S> {
        ProjectLocationServiceDatabaseSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_services_databases_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationServiceDatabaseTestIamPermissionCall<'a, S> {
        ProjectLocationServiceDatabaseTestIamPermissionCall {
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
    /// Creates a new MetadataImport in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The relative resource name of the service in which to create a metastore import, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}.
    pub fn locations_services_metadata_imports_create(&self, request: MetadataImport, parent: &str) -> ProjectLocationServiceMetadataImportCreateCall<'a, S> {
        ProjectLocationServiceMetadataImportCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _metadata_import_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single import.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The relative resource name of the metadata import to retrieve, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/metadataImports/{import_id}.
    pub fn locations_services_metadata_imports_get(&self, name: &str) -> ProjectLocationServiceMetadataImportGetCall<'a, S> {
        ProjectLocationServiceMetadataImportGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists imports in a service.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The relative resource name of the service whose metadata imports to list, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/metadataImports.
    pub fn locations_services_metadata_imports_list(&self, parent: &str) -> ProjectLocationServiceMetadataImportListCall<'a, S> {
        ProjectLocationServiceMetadataImportListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a single import. Only the description field of MetadataImport is supported to be updated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The relative resource name of the metadata import, of the form:projects/{project_number}/locations/{location_id}/services/{service_id}/metadataImports/{metadata_import_id}.
    pub fn locations_services_metadata_imports_patch(&self, request: MetadataImport, name: &str) -> ProjectLocationServiceMetadataImportPatchCall<'a, S> {
        ProjectLocationServiceMetadataImportPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Alter metadata resource location. The metadata resource can be a database, table, or partition. This functionality only updates the parent directory for the respective metadata resource and does not transfer any existing data to the new location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `service` - Required. The relative resource name of the metastore service to mutate metadata, in the following format:projects/{project_id}/locations/{location_id}/services/{service_id}.
    pub fn locations_services_alter_location(&self, request: AlterMetadataResourceLocationRequest, service: &str) -> ProjectLocationServiceAlterLocationCall<'a, S> {
        ProjectLocationServiceAlterLocationCall {
            hub: self.hub,
            _request: request,
            _service: service.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a metastore service in a project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The relative resource name of the location in which to create a metastore service, in the following form:projects/{project_number}/locations/{location_id}.
    pub fn locations_services_create(&self, request: Service, parent: &str) -> ProjectLocationServiceCreateCall<'a, S> {
        ProjectLocationServiceCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _service_id: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single service.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The relative resource name of the metastore service to delete, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}.
    pub fn locations_services_delete(&self, name: &str) -> ProjectLocationServiceDeleteCall<'a, S> {
        ProjectLocationServiceDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Exports metadata from a service.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `service` - Required. The relative resource name of the metastore service to run export, in the following form:projects/{project_id}/locations/{location_id}/services/{service_id}.
    pub fn locations_services_export_metadata(&self, request: ExportMetadataRequest, service: &str) -> ProjectLocationServiceExportMetadataCall<'a, S> {
        ProjectLocationServiceExportMetadataCall {
            hub: self.hub,
            _request: request,
            _service: service.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the details of a single service.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The relative resource name of the metastore service to retrieve, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}.
    pub fn locations_services_get(&self, name: &str) -> ProjectLocationServiceGetCall<'a, S> {
        ProjectLocationServiceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_services_get_iam_policy(&self, resource: &str) -> ProjectLocationServiceGetIamPolicyCall<'a, S> {
        ProjectLocationServiceGetIamPolicyCall {
            hub: self.hub,
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists services in a project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The relative resource name of the location of metastore services to list, in the following form:projects/{project_number}/locations/{location_id}.
    pub fn locations_services_list(&self, parent: &str) -> ProjectLocationServiceListCall<'a, S> {
        ProjectLocationServiceListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Move a table to another database.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `service` - Required. The relative resource name of the metastore service to mutate metadata, in the following format:projects/{project_id}/locations/{location_id}/services/{service_id}.
    pub fn locations_services_move_table_to_database(&self, request: MoveTableToDatabaseRequest, service: &str) -> ProjectLocationServiceMoveTableToDatabaseCall<'a, S> {
        ProjectLocationServiceMoveTableToDatabaseCall {
            hub: self.hub,
            _request: request,
            _service: service.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the parameters of a single service.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}.
    pub fn locations_services_patch(&self, request: Service, name: &str) -> ProjectLocationServicePatchCall<'a, S> {
        ProjectLocationServicePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Query DPMS metadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `service` - Required. The relative resource name of the metastore service to query metadata, in the following format:projects/{project_id}/locations/{location_id}/services/{service_id}.
    pub fn locations_services_query_metadata(&self, request: QueryMetadataRequest, service: &str) -> ProjectLocationServiceQueryMetadataCall<'a, S> {
        ProjectLocationServiceQueryMetadataCall {
            hub: self.hub,
            _request: request,
            _service: service.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes the attached IAM policies for a resource
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - Required. The relative resource name of the dataplane resource to remove IAM policy, in the following form:projects/{project_id}/locations/{location_id}/services/{service_id}/databases/{database_id} or projects/{project_id}/locations/{location_id}/services/{service_id}/databases/{database_id}/tables/{table_id}.
    pub fn locations_services_remove_iam_policy(&self, request: RemoveIamPolicyRequest, resource: &str) -> ProjectLocationServiceRemoveIamPolicyCall<'a, S> {
        ProjectLocationServiceRemoveIamPolicyCall {
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
    /// Restores a service from a backup.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `service` - Required. The relative resource name of the metastore service to run restore, in the following form:projects/{project_id}/locations/{location_id}/services/{service_id}.
    pub fn locations_services_restore(&self, request: RestoreServiceRequest, service: &str) -> ProjectLocationServiceRestoreCall<'a, S> {
        ProjectLocationServiceRestoreCall {
            hub: self.hub,
            _request: request,
            _service: service.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_services_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationServiceSetIamPolicyCall<'a, S> {
        ProjectLocationServiceSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_services_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationServiceTestIamPermissionCall<'a, S> {
        ProjectLocationServiceTestIamPermissionCall {
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
    /// Gets information about a location.
    /// 
    /// # Arguments
    ///
    /// * `name` - Resource name for the location.
    pub fn locations_get(&self, name: &str) -> ProjectLocationGetCall<'a, S> {
        ProjectLocationGetCall {
            hub: self.hub,
            _name: name.to_string(),
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
    /// * `name` - The resource that owns the locations collection, if applicable.
    pub fn locations_list(&self, name: &str) -> ProjectLocationListCall<'a, S> {
        ProjectLocationListCall {
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
}



