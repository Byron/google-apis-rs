use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`DatabaseMigrationService`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_datamigration1 as datamigration1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use datamigration1::{DatabaseMigrationService, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DatabaseMigrationService::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_connection_profiles_create(...)`, `locations_connection_profiles_delete(...)`, `locations_connection_profiles_get(...)`, `locations_connection_profiles_get_iam_policy(...)`, `locations_connection_profiles_list(...)`, `locations_connection_profiles_patch(...)`, `locations_connection_profiles_set_iam_policy(...)`, `locations_connection_profiles_test_iam_permissions(...)`, `locations_conversion_workspaces_apply(...)`, `locations_conversion_workspaces_commit(...)`, `locations_conversion_workspaces_convert(...)`, `locations_conversion_workspaces_create(...)`, `locations_conversion_workspaces_delete(...)`, `locations_conversion_workspaces_describe_conversion_workspace_revisions(...)`, `locations_conversion_workspaces_describe_database_entities(...)`, `locations_conversion_workspaces_get(...)`, `locations_conversion_workspaces_list(...)`, `locations_conversion_workspaces_mapping_rules_import(...)`, `locations_conversion_workspaces_patch(...)`, `locations_conversion_workspaces_rollback(...)`, `locations_conversion_workspaces_search_background_jobs(...)`, `locations_conversion_workspaces_seed(...)`, `locations_get(...)`, `locations_list(...)`, `locations_migration_jobs_create(...)`, `locations_migration_jobs_delete(...)`, `locations_migration_jobs_generate_ssh_script(...)`, `locations_migration_jobs_get(...)`, `locations_migration_jobs_get_iam_policy(...)`, `locations_migration_jobs_list(...)`, `locations_migration_jobs_patch(...)`, `locations_migration_jobs_promote(...)`, `locations_migration_jobs_restart(...)`, `locations_migration_jobs_resume(...)`, `locations_migration_jobs_set_iam_policy(...)`, `locations_migration_jobs_start(...)`, `locations_migration_jobs_stop(...)`, `locations_migration_jobs_test_iam_permissions(...)`, `locations_migration_jobs_verify(...)`, `locations_operations_cancel(...)`, `locations_operations_delete(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `locations_private_connections_create(...)`, `locations_private_connections_delete(...)`, `locations_private_connections_get(...)` and `locations_private_connections_list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DatabaseMigrationService<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new connection profile in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent, which owns this collection of connection profiles.
    pub fn locations_connection_profiles_create(&self, request: ConnectionProfile, parent: &str) -> ProjectLocationConnectionProfileCreateCall<'a, S> {
        ProjectLocationConnectionProfileCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _validate_only: Default::default(),
            _skip_validation: Default::default(),
            _request_id: Default::default(),
            _connection_profile_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single Database Migration Service connection profile. A connection profile can only be deleted if it is not in use by any active migration jobs.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the connection profile resource to delete.
    pub fn locations_connection_profiles_delete(&self, name: &str) -> ProjectLocationConnectionProfileDeleteCall<'a, S> {
        ProjectLocationConnectionProfileDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _request_id: Default::default(),
            _force: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single connection profile.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the connection profile resource to get.
    pub fn locations_connection_profiles_get(&self, name: &str) -> ProjectLocationConnectionProfileGetCall<'a, S> {
        ProjectLocationConnectionProfileGetCall {
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
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_connection_profiles_get_iam_policy(&self, resource: &str) -> ProjectLocationConnectionProfileGetIamPolicyCall<'a, S> {
        ProjectLocationConnectionProfileGetIamPolicyCall {
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
    /// Retrieves a list of all connection profiles in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns this collection of connection profiles.
    pub fn locations_connection_profiles_list(&self, parent: &str) -> ProjectLocationConnectionProfileListCall<'a, S> {
        ProjectLocationConnectionProfileListCall {
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
    /// Update the configuration of a single connection profile.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of this connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{connectionProfile}.
    pub fn locations_connection_profiles_patch(&self, request: ConnectionProfile, name: &str) -> ProjectLocationConnectionProfilePatchCall<'a, S> {
        ProjectLocationConnectionProfilePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _skip_validation: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_connection_profiles_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationConnectionProfileSetIamPolicyCall<'a, S> {
        ProjectLocationConnectionProfileSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_connection_profiles_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationConnectionProfileTestIamPermissionCall<'a, S> {
        ProjectLocationConnectionProfileTestIamPermissionCall {
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
    /// Imports the mapping rules for a given conversion workspace. Supports various formats of external rules files.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the conversion workspace resource to import the rules to in the form of: projects/{project}/locations/{location}/conversionWorkspaces/{conversion_workspace}.
    pub fn locations_conversion_workspaces_mapping_rules_import(&self, request: ImportMappingRulesRequest, parent: &str) -> ProjectLocationConversionWorkspaceMappingRuleImportCall<'a, S> {
        ProjectLocationConversionWorkspaceMappingRuleImportCall {
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
    /// Apply draft tree onto a specific destination database
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the conversion workspace resource to apply draft to destination for. in the form of: projects/{project}/locations/{location}/conversionWorkspaces/{conversion_workspace}.
    pub fn locations_conversion_workspaces_apply(&self, request: ApplyConversionWorkspaceRequest, name: &str) -> ProjectLocationConversionWorkspaceApplyCall<'a, S> {
        ProjectLocationConversionWorkspaceApplyCall {
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
    /// Marks all the data in the conversion workspace as committed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the conversion workspace resource to commit.
    pub fn locations_conversion_workspaces_commit(&self, request: CommitConversionWorkspaceRequest, name: &str) -> ProjectLocationConversionWorkspaceCommitCall<'a, S> {
        ProjectLocationConversionWorkspaceCommitCall {
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
    /// Creates a draft tree schema for the destination database.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Name of the conversion workspace resource to convert in the form of: projects/{project}/locations/{location}/conversionWorkspaces/{conversion_workspace}.
    pub fn locations_conversion_workspaces_convert(&self, request: ConvertConversionWorkspaceRequest, name: &str) -> ProjectLocationConversionWorkspaceConvertCall<'a, S> {
        ProjectLocationConversionWorkspaceConvertCall {
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
    /// Creates a new conversion workspace in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent, which owns this collection of conversion workspaces.
    pub fn locations_conversion_workspaces_create(&self, request: ConversionWorkspace, parent: &str) -> ProjectLocationConversionWorkspaceCreateCall<'a, S> {
        ProjectLocationConversionWorkspaceCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _conversion_workspace_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single conversion workspace.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the conversion workspace resource to delete.
    pub fn locations_conversion_workspaces_delete(&self, name: &str) -> ProjectLocationConversionWorkspaceDeleteCall<'a, S> {
        ProjectLocationConversionWorkspaceDeleteCall {
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
    /// Retrieves a list of committed revisions of a specific conversion workspace.
    /// 
    /// # Arguments
    ///
    /// * `conversionWorkspace` - Required. Name of the conversion workspace resource whose revisions are listed. in the form of: projects/{project}/locations/{location}/conversionWorkspaces/{conversion_workspace}.
    pub fn locations_conversion_workspaces_describe_conversion_workspace_revisions(&self, conversion_workspace: &str) -> ProjectLocationConversionWorkspaceDescribeConversionWorkspaceRevisionCall<'a, S> {
        ProjectLocationConversionWorkspaceDescribeConversionWorkspaceRevisionCall {
            hub: self.hub,
            _conversion_workspace: conversion_workspace.to_string(),
            _commit_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Use this method to describe the database entities tree for a specific conversion workspace and a specific tree type. The DB Entities are not a resource like conversion workspace or mapping rule, and they can not be created, updated or deleted like one. Instead they are simple data objects describing the structure of the client database.
    /// 
    /// # Arguments
    ///
    /// * `conversionWorkspace` - Required. Name of the conversion workspace resource whose DB entities are described in the form of: projects/{project}/locations/{location}/conversionWorkspaces/{conversion_workspace}.
    pub fn locations_conversion_workspaces_describe_database_entities(&self, conversion_workspace: &str) -> ProjectLocationConversionWorkspaceDescribeDatabaseEntityCall<'a, S> {
        ProjectLocationConversionWorkspaceDescribeDatabaseEntityCall {
            hub: self.hub,
            _conversion_workspace: conversion_workspace.to_string(),
            _uncommitted: Default::default(),
            _tree: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _commit_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single conversion workspace.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the conversion workspace resource to get.
    pub fn locations_conversion_workspaces_get(&self, name: &str) -> ProjectLocationConversionWorkspaceGetCall<'a, S> {
        ProjectLocationConversionWorkspaceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists conversion workspaces in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns this collection of conversion workspaces.
    pub fn locations_conversion_workspaces_list(&self, parent: &str) -> ProjectLocationConversionWorkspaceListCall<'a, S> {
        ProjectLocationConversionWorkspaceListCall {
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
    /// Updates the parameters of a single conversion workspace.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Full name of the workspace resource, in the form of: projects/{project}/locations/{location}/conversionWorkspaces/{conversion_workspace}.
    pub fn locations_conversion_workspaces_patch(&self, request: ConversionWorkspace, name: &str) -> ProjectLocationConversionWorkspacePatchCall<'a, S> {
        ProjectLocationConversionWorkspacePatchCall {
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
    /// Rollbacks a conversion workspace to the last committed spanshot.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the conversion workspace resource to rollback to.
    pub fn locations_conversion_workspaces_rollback(&self, request: RollbackConversionWorkspaceRequest, name: &str) -> ProjectLocationConversionWorkspaceRollbackCall<'a, S> {
        ProjectLocationConversionWorkspaceRollbackCall {
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
    /// Use this method to search/list the background jobs for a specific conversion workspace. The background jobs are not a resource like conversion workspace or mapping rule, and they can not be created, updated or deleted like one. Instead they are a way to expose the data plane jobs log.
    /// 
    /// # Arguments
    ///
    /// * `conversionWorkspace` - Required. Name of the conversion workspace resource whos jobs are listed. in the form of: projects/{project}/locations/{location}/conversionWorkspaces/{conversion_workspace}.
    pub fn locations_conversion_workspaces_search_background_jobs(&self, conversion_workspace: &str) -> ProjectLocationConversionWorkspaceSearchBackgroundJobCall<'a, S> {
        ProjectLocationConversionWorkspaceSearchBackgroundJobCall {
            hub: self.hub,
            _conversion_workspace: conversion_workspace.to_string(),
            _return_most_recent_per_job_type: Default::default(),
            _max_size: Default::default(),
            _completed_until_time: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Imports a snapshot of the source database into the conversion workspace.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Name of the conversion workspace resource to seed with new database structure. in the form of: projects/{project}/locations/{location}/conversionWorkspaces/{conversion_workspace}.
    pub fn locations_conversion_workspaces_seed(&self, request: SeedConversionWorkspaceRequest, name: &str) -> ProjectLocationConversionWorkspaceSeedCall<'a, S> {
        ProjectLocationConversionWorkspaceSeedCall {
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
    /// Creates a new migration job in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent, which owns this collection of migration jobs.
    pub fn locations_migration_jobs_create(&self, request: MigrationJob, parent: &str) -> ProjectLocationMigrationJobCreateCall<'a, S> {
        ProjectLocationMigrationJobCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _migration_job_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single migration job.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the migration job resource to delete.
    pub fn locations_migration_jobs_delete(&self, name: &str) -> ProjectLocationMigrationJobDeleteCall<'a, S> {
        ProjectLocationMigrationJobDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _request_id: Default::default(),
            _force: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generate a SSH configuration script to configure the reverse SSH connectivity.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `migrationJob` - Name of the migration job resource to generate the SSH script.
    pub fn locations_migration_jobs_generate_ssh_script(&self, request: GenerateSshScriptRequest, migration_job: &str) -> ProjectLocationMigrationJobGenerateSshScriptCall<'a, S> {
        ProjectLocationMigrationJobGenerateSshScriptCall {
            hub: self.hub,
            _request: request,
            _migration_job: migration_job.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single migration job.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the migration job resource to get.
    pub fn locations_migration_jobs_get(&self, name: &str) -> ProjectLocationMigrationJobGetCall<'a, S> {
        ProjectLocationMigrationJobGetCall {
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
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_migration_jobs_get_iam_policy(&self, resource: &str) -> ProjectLocationMigrationJobGetIamPolicyCall<'a, S> {
        ProjectLocationMigrationJobGetIamPolicyCall {
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
    /// Lists migration jobs in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns this collection of migrationJobs.
    pub fn locations_migration_jobs_list(&self, parent: &str) -> ProjectLocationMigrationJobListCall<'a, S> {
        ProjectLocationMigrationJobListCall {
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
    /// Updates the parameters of a single migration job.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name (URI) of this migration job resource, in the form of: projects/{project}/locations/{location}/migrationJobs/{migrationJob}.
    pub fn locations_migration_jobs_patch(&self, request: MigrationJob, name: &str) -> ProjectLocationMigrationJobPatchCall<'a, S> {
        ProjectLocationMigrationJobPatchCall {
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
    /// Promote a migration job, stopping replication to the destination and promoting the destination to be a standalone database.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Name of the migration job resource to promote.
    pub fn locations_migration_jobs_promote(&self, request: PromoteMigrationJobRequest, name: &str) -> ProjectLocationMigrationJobPromoteCall<'a, S> {
        ProjectLocationMigrationJobPromoteCall {
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
    /// Restart a stopped or failed migration job, resetting the destination instance to its original state and starting the migration process from scratch.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Name of the migration job resource to restart.
    pub fn locations_migration_jobs_restart(&self, request: RestartMigrationJobRequest, name: &str) -> ProjectLocationMigrationJobRestartCall<'a, S> {
        ProjectLocationMigrationJobRestartCall {
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
    /// Resume a migration job that is currently stopped and is resumable (was stopped during CDC phase).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Name of the migration job resource to resume.
    pub fn locations_migration_jobs_resume(&self, request: ResumeMigrationJobRequest, name: &str) -> ProjectLocationMigrationJobResumeCall<'a, S> {
        ProjectLocationMigrationJobResumeCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_migration_jobs_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationMigrationJobSetIamPolicyCall<'a, S> {
        ProjectLocationMigrationJobSetIamPolicyCall {
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
    /// Start an already created migration job.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Name of the migration job resource to start.
    pub fn locations_migration_jobs_start(&self, request: StartMigrationJobRequest, name: &str) -> ProjectLocationMigrationJobStartCall<'a, S> {
        ProjectLocationMigrationJobStartCall {
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
    /// Stops a running migration job.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Name of the migration job resource to stop.
    pub fn locations_migration_jobs_stop(&self, request: StopMigrationJobRequest, name: &str) -> ProjectLocationMigrationJobStopCall<'a, S> {
        ProjectLocationMigrationJobStopCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_migration_jobs_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationMigrationJobTestIamPermissionCall<'a, S> {
        ProjectLocationMigrationJobTestIamPermissionCall {
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
    /// Verify a migration job, making sure the destination can reach the source and that all configuration and prerequisites are met.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Name of the migration job resource to verify.
    pub fn locations_migration_jobs_verify(&self, request: VerifyMigrationJobRequest, name: &str) -> ProjectLocationMigrationJobVerifyCall<'a, S> {
        ProjectLocationMigrationJobVerifyCall {
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
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn locations_operations_cancel(&self, request: CancelOperationRequest, name: &str) -> ProjectLocationOperationCancelCall<'a, S> {
        ProjectLocationOperationCancelCall {
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
    /// Creates a new private connection in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent that owns the collection of PrivateConnections.
    pub fn locations_private_connections_create(&self, request: PrivateConnection, parent: &str) -> ProjectLocationPrivateConnectionCreateCall<'a, S> {
        ProjectLocationPrivateConnectionCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _skip_validation: Default::default(),
            _request_id: Default::default(),
            _private_connection_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single Database Migration Service private connection.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the private connection to delete.
    pub fn locations_private_connections_delete(&self, name: &str) -> ProjectLocationPrivateConnectionDeleteCall<'a, S> {
        ProjectLocationPrivateConnectionDeleteCall {
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
    /// Gets details of a single private connection.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the private connection to get.
    pub fn locations_private_connections_get(&self, name: &str) -> ProjectLocationPrivateConnectionGetCall<'a, S> {
        ProjectLocationPrivateConnectionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of private connections in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent that owns the collection of private connections.
    pub fn locations_private_connections_list(&self, parent: &str) -> ProjectLocationPrivateConnectionListCall<'a, S> {
        ProjectLocationPrivateConnectionListCall {
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



