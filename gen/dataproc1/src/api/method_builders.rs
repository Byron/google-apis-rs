use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Dataproc`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dataproc1 as dataproc1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dataproc1::{Dataproc, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dataproc::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_autoscaling_policies_create(...)`, `locations_autoscaling_policies_delete(...)`, `locations_autoscaling_policies_get(...)`, `locations_autoscaling_policies_get_iam_policy(...)`, `locations_autoscaling_policies_list(...)`, `locations_autoscaling_policies_set_iam_policy(...)`, `locations_autoscaling_policies_test_iam_permissions(...)`, `locations_autoscaling_policies_update(...)`, `locations_batches_create(...)`, `locations_batches_delete(...)`, `locations_batches_get(...)`, `locations_batches_list(...)`, `locations_operations_cancel(...)`, `locations_operations_delete(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `locations_workflow_templates_create(...)`, `locations_workflow_templates_delete(...)`, `locations_workflow_templates_get(...)`, `locations_workflow_templates_get_iam_policy(...)`, `locations_workflow_templates_instantiate(...)`, `locations_workflow_templates_instantiate_inline(...)`, `locations_workflow_templates_list(...)`, `locations_workflow_templates_set_iam_policy(...)`, `locations_workflow_templates_test_iam_permissions(...)`, `locations_workflow_templates_update(...)`, `regions_autoscaling_policies_create(...)`, `regions_autoscaling_policies_delete(...)`, `regions_autoscaling_policies_get(...)`, `regions_autoscaling_policies_get_iam_policy(...)`, `regions_autoscaling_policies_list(...)`, `regions_autoscaling_policies_set_iam_policy(...)`, `regions_autoscaling_policies_test_iam_permissions(...)`, `regions_autoscaling_policies_update(...)`, `regions_clusters_create(...)`, `regions_clusters_delete(...)`, `regions_clusters_diagnose(...)`, `regions_clusters_get(...)`, `regions_clusters_get_iam_policy(...)`, `regions_clusters_inject_credentials(...)`, `regions_clusters_list(...)`, `regions_clusters_node_groups_create(...)`, `regions_clusters_node_groups_get(...)`, `regions_clusters_node_groups_resize(...)`, `regions_clusters_patch(...)`, `regions_clusters_repair(...)`, `regions_clusters_set_iam_policy(...)`, `regions_clusters_start(...)`, `regions_clusters_stop(...)`, `regions_clusters_test_iam_permissions(...)`, `regions_jobs_cancel(...)`, `regions_jobs_delete(...)`, `regions_jobs_get(...)`, `regions_jobs_get_iam_policy(...)`, `regions_jobs_list(...)`, `regions_jobs_patch(...)`, `regions_jobs_set_iam_policy(...)`, `regions_jobs_submit(...)`, `regions_jobs_submit_as_operation(...)`, `regions_jobs_test_iam_permissions(...)`, `regions_operations_cancel(...)`, `regions_operations_delete(...)`, `regions_operations_get(...)`, `regions_operations_get_iam_policy(...)`, `regions_operations_list(...)`, `regions_operations_set_iam_policy(...)`, `regions_operations_test_iam_permissions(...)`, `regions_workflow_templates_create(...)`, `regions_workflow_templates_delete(...)`, `regions_workflow_templates_get(...)`, `regions_workflow_templates_get_iam_policy(...)`, `regions_workflow_templates_instantiate(...)`, `regions_workflow_templates_instantiate_inline(...)`, `regions_workflow_templates_list(...)`, `regions_workflow_templates_set_iam_policy(...)`, `regions_workflow_templates_test_iam_permissions(...)` and `regions_workflow_templates_update(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dataproc<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates new autoscaling policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The "resource name" of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies.create, the resource name of the region has the following format: projects/{project_id}/regions/{region} For projects.locations.autoscalingPolicies.create, the resource name of the location has the following format: projects/{project_id}/locations/{location}
    pub fn locations_autoscaling_policies_create(&self, request: AutoscalingPolicy, parent: &str) -> ProjectLocationAutoscalingPolicyCreateCall<'a, S> {
        ProjectLocationAutoscalingPolicyCreateCall {
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
    /// Deletes an autoscaling policy. It is an error to delete an autoscaling policy that is in use by one or more clusters.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The "resource name" of the autoscaling policy, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies.delete, the resource name of the policy has the following format: projects/{project_id}/regions/{region}/autoscalingPolicies/{policy_id} For projects.locations.autoscalingPolicies.delete, the resource name of the policy has the following format: projects/{project_id}/locations/{location}/autoscalingPolicies/{policy_id}
    pub fn locations_autoscaling_policies_delete(&self, name: &str) -> ProjectLocationAutoscalingPolicyDeleteCall<'a, S> {
        ProjectLocationAutoscalingPolicyDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves autoscaling policy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The "resource name" of the autoscaling policy, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies.get, the resource name of the policy has the following format: projects/{project_id}/regions/{region}/autoscalingPolicies/{policy_id} For projects.locations.autoscalingPolicies.get, the resource name of the policy has the following format: projects/{project_id}/locations/{location}/autoscalingPolicies/{policy_id}
    pub fn locations_autoscaling_policies_get(&self, name: &str) -> ProjectLocationAutoscalingPolicyGetCall<'a, S> {
        ProjectLocationAutoscalingPolicyGetCall {
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
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_autoscaling_policies_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectLocationAutoscalingPolicyGetIamPolicyCall<'a, S> {
        ProjectLocationAutoscalingPolicyGetIamPolicyCall {
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
    /// Lists autoscaling policies in the project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The "resource name" of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies.list, the resource name of the region has the following format: projects/{project_id}/regions/{region} For projects.locations.autoscalingPolicies.list, the resource name of the location has the following format: projects/{project_id}/locations/{location}
    pub fn locations_autoscaling_policies_list(&self, parent: &str) -> ProjectLocationAutoscalingPolicyListCall<'a, S> {
        ProjectLocationAutoscalingPolicyListCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_autoscaling_policies_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationAutoscalingPolicySetIamPolicyCall<'a, S> {
        ProjectLocationAutoscalingPolicySetIamPolicyCall {
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
    pub fn locations_autoscaling_policies_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationAutoscalingPolicyTestIamPermissionCall<'a, S> {
        ProjectLocationAutoscalingPolicyTestIamPermissionCall {
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
    /// Updates (replaces) autoscaling policy.Disabled check for update_mask, because all updates will be full replacements.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The "resource name" of the autoscaling policy, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies, the resource name of the policy has the following format: projects/{project_id}/regions/{region}/autoscalingPolicies/{policy_id} For projects.locations.autoscalingPolicies, the resource name of the policy has the following format: projects/{project_id}/locations/{location}/autoscalingPolicies/{policy_id}
    pub fn locations_autoscaling_policies_update(&self, request: AutoscalingPolicy, name: &str) -> ProjectLocationAutoscalingPolicyUpdateCall<'a, S> {
        ProjectLocationAutoscalingPolicyUpdateCall {
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
    /// Creates a batch workload that executes asynchronously.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource where this batch will be created.
    pub fn locations_batches_create(&self, request: Batch, parent: &str) -> ProjectLocationBatchCreateCall<'a, S> {
        ProjectLocationBatchCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _batch_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the batch workload resource. If the batch is not in terminal state, the delete fails and the response returns FAILED_PRECONDITION.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The fully qualified name of the batch to retrieve in the format "projects/PROJECT_ID/locations/DATAPROC_REGION/batches/BATCH_ID"
    pub fn locations_batches_delete(&self, name: &str) -> ProjectLocationBatchDeleteCall<'a, S> {
        ProjectLocationBatchDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the batch workload resource representation.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The fully qualified name of the batch to retrieve in the format "projects/PROJECT_ID/locations/DATAPROC_REGION/batches/BATCH_ID"
    pub fn locations_batches_get(&self, name: &str) -> ProjectLocationBatchGetCall<'a, S> {
        ProjectLocationBatchGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists batch workloads.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns this collection of batches.
    pub fn locations_batches_list(&self, parent: &str) -> ProjectLocationBatchListCall<'a, S> {
        ProjectLocationBatchListCall {
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
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED.
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
    /// Creates new workflow template.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates.create, the resource name of the region has the following format: projects/{project_id}/regions/{region} For projects.locations.workflowTemplates.create, the resource name of the location has the following format: projects/{project_id}/locations/{location}
    pub fn locations_workflow_templates_create(&self, request: WorkflowTemplate, parent: &str) -> ProjectLocationWorkflowTemplateCreateCall<'a, S> {
        ProjectLocationWorkflowTemplateCreateCall {
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
    /// Deletes a workflow template. It does not cancel in-progress workflows.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates.delete, the resource name of the template has the following format: projects/{project_id}/regions/{region}/workflowTemplates/{template_id} For projects.locations.workflowTemplates.instantiate, the resource name of the template has the following format: projects/{project_id}/locations/{location}/workflowTemplates/{template_id}
    pub fn locations_workflow_templates_delete(&self, name: &str) -> ProjectLocationWorkflowTemplateDeleteCall<'a, S> {
        ProjectLocationWorkflowTemplateDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the latest workflow template.Can retrieve previously instantiated template by specifying optional version parameter.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates.get, the resource name of the template has the following format: projects/{project_id}/regions/{region}/workflowTemplates/{template_id} For projects.locations.workflowTemplates.get, the resource name of the template has the following format: projects/{project_id}/locations/{location}/workflowTemplates/{template_id}
    pub fn locations_workflow_templates_get(&self, name: &str) -> ProjectLocationWorkflowTemplateGetCall<'a, S> {
        ProjectLocationWorkflowTemplateGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _version: Default::default(),
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
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_workflow_templates_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectLocationWorkflowTemplateGetIamPolicyCall<'a, S> {
        ProjectLocationWorkflowTemplateGetIamPolicyCall {
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
    /// Instantiates a template and begins execution.The returned Operation can be used to track execution of workflow by polling operations.get. The Operation will complete when entire workflow is finished.The running workflow can be aborted via operations.cancel. This will cause any inflight jobs to be cancelled and workflow-owned clusters to be deleted.The Operation.metadata will be WorkflowMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#workflowmetadata). Also see Using WorkflowMetadata (https://cloud.google.com/dataproc/docs/concepts/workflows/debugging#using_workflowmetadata).On successful completion, Operation.response will be Empty.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates.instantiate, the resource name of the template has the following format: projects/{project_id}/regions/{region}/workflowTemplates/{template_id} For projects.locations.workflowTemplates.instantiate, the resource name of the template has the following format: projects/{project_id}/locations/{location}/workflowTemplates/{template_id}
    pub fn locations_workflow_templates_instantiate(&self, request: InstantiateWorkflowTemplateRequest, name: &str) -> ProjectLocationWorkflowTemplateInstantiateCall<'a, S> {
        ProjectLocationWorkflowTemplateInstantiateCall {
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
    /// Instantiates a template and begins execution.This method is equivalent to executing the sequence CreateWorkflowTemplate, InstantiateWorkflowTemplate, DeleteWorkflowTemplate.The returned Operation can be used to track execution of workflow by polling operations.get. The Operation will complete when entire workflow is finished.The running workflow can be aborted via operations.cancel. This will cause any inflight jobs to be cancelled and workflow-owned clusters to be deleted.The Operation.metadata will be WorkflowMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#workflowmetadata). Also see Using WorkflowMetadata (https://cloud.google.com/dataproc/docs/concepts/workflows/debugging#using_workflowmetadata).On successful completion, Operation.response will be Empty.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates,instantiateinline, the resource name of the region has the following format: projects/{project_id}/regions/{region} For projects.locations.workflowTemplates.instantiateinline, the resource name of the location has the following format: projects/{project_id}/locations/{location}
    pub fn locations_workflow_templates_instantiate_inline(&self, request: WorkflowTemplate, parent: &str) -> ProjectLocationWorkflowTemplateInstantiateInlineCall<'a, S> {
        ProjectLocationWorkflowTemplateInstantiateInlineCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists workflows that match the specified filter in the request.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates,list, the resource name of the region has the following format: projects/{project_id}/regions/{region} For projects.locations.workflowTemplates.list, the resource name of the location has the following format: projects/{project_id}/locations/{location}
    pub fn locations_workflow_templates_list(&self, parent: &str) -> ProjectLocationWorkflowTemplateListCall<'a, S> {
        ProjectLocationWorkflowTemplateListCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_workflow_templates_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationWorkflowTemplateSetIamPolicyCall<'a, S> {
        ProjectLocationWorkflowTemplateSetIamPolicyCall {
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
    pub fn locations_workflow_templates_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationWorkflowTemplateTestIamPermissionCall<'a, S> {
        ProjectLocationWorkflowTemplateTestIamPermissionCall {
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
    /// Updates (replaces) workflow template. The updated template must contain version that matches the current server version.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates, the resource name of the template has the following format: projects/{project_id}/regions/{region}/workflowTemplates/{template_id} For projects.locations.workflowTemplates, the resource name of the template has the following format: projects/{project_id}/locations/{location}/workflowTemplates/{template_id}
    pub fn locations_workflow_templates_update(&self, request: WorkflowTemplate, name: &str) -> ProjectLocationWorkflowTemplateUpdateCall<'a, S> {
        ProjectLocationWorkflowTemplateUpdateCall {
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
    /// Creates new autoscaling policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The "resource name" of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies.create, the resource name of the region has the following format: projects/{project_id}/regions/{region} For projects.locations.autoscalingPolicies.create, the resource name of the location has the following format: projects/{project_id}/locations/{location}
    pub fn regions_autoscaling_policies_create(&self, request: AutoscalingPolicy, parent: &str) -> ProjectRegionAutoscalingPolicyCreateCall<'a, S> {
        ProjectRegionAutoscalingPolicyCreateCall {
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
    /// Deletes an autoscaling policy. It is an error to delete an autoscaling policy that is in use by one or more clusters.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The "resource name" of the autoscaling policy, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies.delete, the resource name of the policy has the following format: projects/{project_id}/regions/{region}/autoscalingPolicies/{policy_id} For projects.locations.autoscalingPolicies.delete, the resource name of the policy has the following format: projects/{project_id}/locations/{location}/autoscalingPolicies/{policy_id}
    pub fn regions_autoscaling_policies_delete(&self, name: &str) -> ProjectRegionAutoscalingPolicyDeleteCall<'a, S> {
        ProjectRegionAutoscalingPolicyDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves autoscaling policy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The "resource name" of the autoscaling policy, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies.get, the resource name of the policy has the following format: projects/{project_id}/regions/{region}/autoscalingPolicies/{policy_id} For projects.locations.autoscalingPolicies.get, the resource name of the policy has the following format: projects/{project_id}/locations/{location}/autoscalingPolicies/{policy_id}
    pub fn regions_autoscaling_policies_get(&self, name: &str) -> ProjectRegionAutoscalingPolicyGetCall<'a, S> {
        ProjectRegionAutoscalingPolicyGetCall {
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
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn regions_autoscaling_policies_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectRegionAutoscalingPolicyGetIamPolicyCall<'a, S> {
        ProjectRegionAutoscalingPolicyGetIamPolicyCall {
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
    /// Lists autoscaling policies in the project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The "resource name" of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies.list, the resource name of the region has the following format: projects/{project_id}/regions/{region} For projects.locations.autoscalingPolicies.list, the resource name of the location has the following format: projects/{project_id}/locations/{location}
    pub fn regions_autoscaling_policies_list(&self, parent: &str) -> ProjectRegionAutoscalingPolicyListCall<'a, S> {
        ProjectRegionAutoscalingPolicyListCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn regions_autoscaling_policies_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectRegionAutoscalingPolicySetIamPolicyCall<'a, S> {
        ProjectRegionAutoscalingPolicySetIamPolicyCall {
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
    pub fn regions_autoscaling_policies_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectRegionAutoscalingPolicyTestIamPermissionCall<'a, S> {
        ProjectRegionAutoscalingPolicyTestIamPermissionCall {
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
    /// Updates (replaces) autoscaling policy.Disabled check for update_mask, because all updates will be full replacements.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The "resource name" of the autoscaling policy, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies, the resource name of the policy has the following format: projects/{project_id}/regions/{region}/autoscalingPolicies/{policy_id} For projects.locations.autoscalingPolicies, the resource name of the policy has the following format: projects/{project_id}/locations/{location}/autoscalingPolicies/{policy_id}
    pub fn regions_autoscaling_policies_update(&self, request: AutoscalingPolicy, name: &str) -> ProjectRegionAutoscalingPolicyUpdateCall<'a, S> {
        ProjectRegionAutoscalingPolicyUpdateCall {
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
    /// Creates a node group in a cluster. The returned Operation.metadata is NodeGroupOperationMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#nodegroupoperationmetadata).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource where this node group will be created. Format: projects/{project}/regions/{region}/clusters/{cluster}
    pub fn regions_clusters_node_groups_create(&self, request: NodeGroup, parent: &str) -> ProjectRegionClusterNodeGroupCreateCall<'a, S> {
        ProjectRegionClusterNodeGroupCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _node_group_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the resource representation for a node group in a cluster.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the node group to retrieve. Format: projects/{project}/regions/{region}/clusters/{cluster}/nodeGroups/{nodeGroup}
    pub fn regions_clusters_node_groups_get(&self, name: &str) -> ProjectRegionClusterNodeGroupGetCall<'a, S> {
        ProjectRegionClusterNodeGroupGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Resizes a node group in a cluster. The returned Operation.metadata is NodeGroupOperationMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#nodegroupoperationmetadata).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the node group to resize. Format: projects/{project}/regions/{region}/clusters/{cluster}/nodeGroups/{nodeGroup}
    pub fn regions_clusters_node_groups_resize(&self, request: ResizeNodeGroupRequest, name: &str) -> ProjectRegionClusterNodeGroupResizeCall<'a, S> {
        ProjectRegionClusterNodeGroupResizeCall {
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
    /// Creates a cluster in a project. The returned Operation.metadata will be ClusterOperationMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#clusteroperationmetadata).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. The ID of the Google Cloud Platform project that the cluster belongs to.
    /// * `region` - Required. The Dataproc region in which to handle the request.
    pub fn regions_clusters_create(&self, request: Cluster, project_id: &str, region: &str) -> ProjectRegionClusterCreateCall<'a, S> {
        ProjectRegionClusterCreateCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _action_on_failed_primary_workers: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a cluster in a project. The returned Operation.metadata will be ClusterOperationMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#clusteroperationmetadata).
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. The ID of the Google Cloud Platform project that the cluster belongs to.
    /// * `region` - Required. The Dataproc region in which to handle the request.
    /// * `clusterName` - Required. The cluster name.
    pub fn regions_clusters_delete(&self, project_id: &str, region: &str, cluster_name: &str) -> ProjectRegionClusterDeleteCall<'a, S> {
        ProjectRegionClusterDeleteCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _region: region.to_string(),
            _cluster_name: cluster_name.to_string(),
            _request_id: Default::default(),
            _cluster_uuid: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets cluster diagnostic information. The returned Operation.metadata will be ClusterOperationMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#clusteroperationmetadata). After the operation completes, Operation.response contains DiagnoseClusterResults (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#diagnoseclusterresults).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. The ID of the Google Cloud Platform project that the cluster belongs to.
    /// * `region` - Required. The Dataproc region in which to handle the request.
    /// * `clusterName` - Required. The cluster name.
    pub fn regions_clusters_diagnose(&self, request: DiagnoseClusterRequest, project_id: &str, region: &str, cluster_name: &str) -> ProjectRegionClusterDiagnoseCall<'a, S> {
        ProjectRegionClusterDiagnoseCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _region: region.to_string(),
            _cluster_name: cluster_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the resource representation for a cluster in a project.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. The ID of the Google Cloud Platform project that the cluster belongs to.
    /// * `region` - Required. The Dataproc region in which to handle the request.
    /// * `clusterName` - Required. The cluster name.
    pub fn regions_clusters_get(&self, project_id: &str, region: &str, cluster_name: &str) -> ProjectRegionClusterGetCall<'a, S> {
        ProjectRegionClusterGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _region: region.to_string(),
            _cluster_name: cluster_name.to_string(),
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
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn regions_clusters_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectRegionClusterGetIamPolicyCall<'a, S> {
        ProjectRegionClusterGetIamPolicyCall {
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
    /// Inject encrypted credentials into all of the VMs in a cluster.The target cluster must be a personal auth cluster assigned to the user who is issuing the RPC.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Required. The ID of the Google Cloud Platform project the cluster belongs to, of the form projects/.
    /// * `region` - Required. The region containing the cluster, of the form regions/.
    /// * `cluster` - Required. The cluster, in the form clusters/.
    pub fn regions_clusters_inject_credentials(&self, request: InjectCredentialsRequest, project: &str, region: &str, cluster: &str) -> ProjectRegionClusterInjectCredentialCall<'a, S> {
        ProjectRegionClusterInjectCredentialCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _cluster: cluster.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all regions/{region}/clusters in a project alphabetically.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. The ID of the Google Cloud Platform project that the cluster belongs to.
    /// * `region` - Required. The Dataproc region in which to handle the request.
    pub fn regions_clusters_list(&self, project_id: &str, region: &str) -> ProjectRegionClusterListCall<'a, S> {
        ProjectRegionClusterListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _region: region.to_string(),
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
    /// Updates a cluster in a project. The returned Operation.metadata will be ClusterOperationMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#clusteroperationmetadata). The cluster must be in a RUNNING state or an error is returned.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. The ID of the Google Cloud Platform project the cluster belongs to.
    /// * `region` - Required. The Dataproc region in which to handle the request.
    /// * `clusterName` - Required. The cluster name.
    pub fn regions_clusters_patch(&self, request: Cluster, project_id: &str, region: &str, cluster_name: &str) -> ProjectRegionClusterPatchCall<'a, S> {
        ProjectRegionClusterPatchCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _region: region.to_string(),
            _cluster_name: cluster_name.to_string(),
            _update_mask: Default::default(),
            _request_id: Default::default(),
            _graceful_decommission_timeout: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Repairs a cluster.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. The ID of the Google Cloud Platform project the cluster belongs to.
    /// * `region` - Required. The Dataproc region in which to handle the request.
    /// * `clusterName` - Required. The cluster name.
    pub fn regions_clusters_repair(&self, request: RepairClusterRequest, project_id: &str, region: &str, cluster_name: &str) -> ProjectRegionClusterRepairCall<'a, S> {
        ProjectRegionClusterRepairCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _region: region.to_string(),
            _cluster_name: cluster_name.to_string(),
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
    pub fn regions_clusters_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectRegionClusterSetIamPolicyCall<'a, S> {
        ProjectRegionClusterSetIamPolicyCall {
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
    /// Starts a cluster in a project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. The ID of the Google Cloud Platform project the cluster belongs to.
    /// * `region` - Required. The Dataproc region in which to handle the request.
    /// * `clusterName` - Required. The cluster name.
    pub fn regions_clusters_start(&self, request: StartClusterRequest, project_id: &str, region: &str, cluster_name: &str) -> ProjectRegionClusterStartCall<'a, S> {
        ProjectRegionClusterStartCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _region: region.to_string(),
            _cluster_name: cluster_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Stops a cluster in a project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. The ID of the Google Cloud Platform project the cluster belongs to.
    /// * `region` - Required. The Dataproc region in which to handle the request.
    /// * `clusterName` - Required. The cluster name.
    pub fn regions_clusters_stop(&self, request: StopClusterRequest, project_id: &str, region: &str, cluster_name: &str) -> ProjectRegionClusterStopCall<'a, S> {
        ProjectRegionClusterStopCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _region: region.to_string(),
            _cluster_name: cluster_name.to_string(),
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
    pub fn regions_clusters_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectRegionClusterTestIamPermissionCall<'a, S> {
        ProjectRegionClusterTestIamPermissionCall {
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
    /// Starts a job cancellation request. To access the job resource after cancellation, call regions/{region}/jobs.list (https://cloud.google.com/dataproc/docs/reference/rest/v1/projects.regions.jobs/list) or regions/{region}/jobs.get (https://cloud.google.com/dataproc/docs/reference/rest/v1/projects.regions.jobs/get).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. The ID of the Google Cloud Platform project that the job belongs to.
    /// * `region` - Required. The Dataproc region in which to handle the request.
    /// * `jobId` - Required. The job ID.
    pub fn regions_jobs_cancel(&self, request: CancelJobRequest, project_id: &str, region: &str, job_id: &str) -> ProjectRegionJobCancelCall<'a, S> {
        ProjectRegionJobCancelCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _region: region.to_string(),
            _job_id: job_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the job from the project. If the job is active, the delete fails, and the response returns FAILED_PRECONDITION.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. The ID of the Google Cloud Platform project that the job belongs to.
    /// * `region` - Required. The Dataproc region in which to handle the request.
    /// * `jobId` - Required. The job ID.
    pub fn regions_jobs_delete(&self, project_id: &str, region: &str, job_id: &str) -> ProjectRegionJobDeleteCall<'a, S> {
        ProjectRegionJobDeleteCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _region: region.to_string(),
            _job_id: job_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the resource representation for a job in a project.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. The ID of the Google Cloud Platform project that the job belongs to.
    /// * `region` - Required. The Dataproc region in which to handle the request.
    /// * `jobId` - Required. The job ID.
    pub fn regions_jobs_get(&self, project_id: &str, region: &str, job_id: &str) -> ProjectRegionJobGetCall<'a, S> {
        ProjectRegionJobGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _region: region.to_string(),
            _job_id: job_id.to_string(),
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
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn regions_jobs_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectRegionJobGetIamPolicyCall<'a, S> {
        ProjectRegionJobGetIamPolicyCall {
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
    /// Lists regions/{region}/jobs in a project.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. The ID of the Google Cloud Platform project that the job belongs to.
    /// * `region` - Required. The Dataproc region in which to handle the request.
    pub fn regions_jobs_list(&self, project_id: &str, region: &str) -> ProjectRegionJobListCall<'a, S> {
        ProjectRegionJobListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _region: region.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _job_state_matcher: Default::default(),
            _filter: Default::default(),
            _cluster_name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a job in a project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. The ID of the Google Cloud Platform project that the job belongs to.
    /// * `region` - Required. The Dataproc region in which to handle the request.
    /// * `jobId` - Required. The job ID.
    pub fn regions_jobs_patch(&self, request: Job, project_id: &str, region: &str, job_id: &str) -> ProjectRegionJobPatchCall<'a, S> {
        ProjectRegionJobPatchCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _region: region.to_string(),
            _job_id: job_id.to_string(),
            _update_mask: Default::default(),
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
    pub fn regions_jobs_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectRegionJobSetIamPolicyCall<'a, S> {
        ProjectRegionJobSetIamPolicyCall {
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
    /// Submits a job to a cluster.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. The ID of the Google Cloud Platform project that the job belongs to.
    /// * `region` - Required. The Dataproc region in which to handle the request.
    pub fn regions_jobs_submit(&self, request: SubmitJobRequest, project_id: &str, region: &str) -> ProjectRegionJobSubmitCall<'a, S> {
        ProjectRegionJobSubmitCall {
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
    /// Submits job to a cluster.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. The ID of the Google Cloud Platform project that the job belongs to.
    /// * `region` - Required. The Dataproc region in which to handle the request.
    pub fn regions_jobs_submit_as_operation(&self, request: SubmitJobRequest, project_id: &str, region: &str) -> ProjectRegionJobSubmitAsOperationCall<'a, S> {
        ProjectRegionJobSubmitAsOperationCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn regions_jobs_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectRegionJobTestIamPermissionCall<'a, S> {
        ProjectRegionJobTestIamPermissionCall {
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
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn regions_operations_cancel(&self, name: &str) -> ProjectRegionOperationCancelCall<'a, S> {
        ProjectRegionOperationCancelCall {
            hub: self.hub,
            _name: name.to_string(),
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
    pub fn regions_operations_delete(&self, name: &str) -> ProjectRegionOperationDeleteCall<'a, S> {
        ProjectRegionOperationDeleteCall {
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
    pub fn regions_operations_get(&self, name: &str) -> ProjectRegionOperationGetCall<'a, S> {
        ProjectRegionOperationGetCall {
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
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn regions_operations_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectRegionOperationGetIamPolicyCall<'a, S> {
        ProjectRegionOperationGetIamPolicyCall {
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
    /// Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns UNIMPLEMENTED.NOTE: the name binding allows API services to override the binding to use different resource name schemes, such as users/*/operations. To override the binding, API services can add a binding such as "/v1/{name=users/*}/operations" to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn regions_operations_list(&self, name: &str) -> ProjectRegionOperationListCall<'a, S> {
        ProjectRegionOperationListCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn regions_operations_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectRegionOperationSetIamPolicyCall<'a, S> {
        ProjectRegionOperationSetIamPolicyCall {
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
    pub fn regions_operations_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectRegionOperationTestIamPermissionCall<'a, S> {
        ProjectRegionOperationTestIamPermissionCall {
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
    /// Creates new workflow template.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates.create, the resource name of the region has the following format: projects/{project_id}/regions/{region} For projects.locations.workflowTemplates.create, the resource name of the location has the following format: projects/{project_id}/locations/{location}
    pub fn regions_workflow_templates_create(&self, request: WorkflowTemplate, parent: &str) -> ProjectRegionWorkflowTemplateCreateCall<'a, S> {
        ProjectRegionWorkflowTemplateCreateCall {
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
    /// Deletes a workflow template. It does not cancel in-progress workflows.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates.delete, the resource name of the template has the following format: projects/{project_id}/regions/{region}/workflowTemplates/{template_id} For projects.locations.workflowTemplates.instantiate, the resource name of the template has the following format: projects/{project_id}/locations/{location}/workflowTemplates/{template_id}
    pub fn regions_workflow_templates_delete(&self, name: &str) -> ProjectRegionWorkflowTemplateDeleteCall<'a, S> {
        ProjectRegionWorkflowTemplateDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the latest workflow template.Can retrieve previously instantiated template by specifying optional version parameter.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates.get, the resource name of the template has the following format: projects/{project_id}/regions/{region}/workflowTemplates/{template_id} For projects.locations.workflowTemplates.get, the resource name of the template has the following format: projects/{project_id}/locations/{location}/workflowTemplates/{template_id}
    pub fn regions_workflow_templates_get(&self, name: &str) -> ProjectRegionWorkflowTemplateGetCall<'a, S> {
        ProjectRegionWorkflowTemplateGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _version: Default::default(),
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
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn regions_workflow_templates_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectRegionWorkflowTemplateGetIamPolicyCall<'a, S> {
        ProjectRegionWorkflowTemplateGetIamPolicyCall {
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
    /// Instantiates a template and begins execution.The returned Operation can be used to track execution of workflow by polling operations.get. The Operation will complete when entire workflow is finished.The running workflow can be aborted via operations.cancel. This will cause any inflight jobs to be cancelled and workflow-owned clusters to be deleted.The Operation.metadata will be WorkflowMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#workflowmetadata). Also see Using WorkflowMetadata (https://cloud.google.com/dataproc/docs/concepts/workflows/debugging#using_workflowmetadata).On successful completion, Operation.response will be Empty.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates.instantiate, the resource name of the template has the following format: projects/{project_id}/regions/{region}/workflowTemplates/{template_id} For projects.locations.workflowTemplates.instantiate, the resource name of the template has the following format: projects/{project_id}/locations/{location}/workflowTemplates/{template_id}
    pub fn regions_workflow_templates_instantiate(&self, request: InstantiateWorkflowTemplateRequest, name: &str) -> ProjectRegionWorkflowTemplateInstantiateCall<'a, S> {
        ProjectRegionWorkflowTemplateInstantiateCall {
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
    /// Instantiates a template and begins execution.This method is equivalent to executing the sequence CreateWorkflowTemplate, InstantiateWorkflowTemplate, DeleteWorkflowTemplate.The returned Operation can be used to track execution of workflow by polling operations.get. The Operation will complete when entire workflow is finished.The running workflow can be aborted via operations.cancel. This will cause any inflight jobs to be cancelled and workflow-owned clusters to be deleted.The Operation.metadata will be WorkflowMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#workflowmetadata). Also see Using WorkflowMetadata (https://cloud.google.com/dataproc/docs/concepts/workflows/debugging#using_workflowmetadata).On successful completion, Operation.response will be Empty.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates,instantiateinline, the resource name of the region has the following format: projects/{project_id}/regions/{region} For projects.locations.workflowTemplates.instantiateinline, the resource name of the location has the following format: projects/{project_id}/locations/{location}
    pub fn regions_workflow_templates_instantiate_inline(&self, request: WorkflowTemplate, parent: &str) -> ProjectRegionWorkflowTemplateInstantiateInlineCall<'a, S> {
        ProjectRegionWorkflowTemplateInstantiateInlineCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists workflows that match the specified filter in the request.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates,list, the resource name of the region has the following format: projects/{project_id}/regions/{region} For projects.locations.workflowTemplates.list, the resource name of the location has the following format: projects/{project_id}/locations/{location}
    pub fn regions_workflow_templates_list(&self, parent: &str) -> ProjectRegionWorkflowTemplateListCall<'a, S> {
        ProjectRegionWorkflowTemplateListCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn regions_workflow_templates_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectRegionWorkflowTemplateSetIamPolicyCall<'a, S> {
        ProjectRegionWorkflowTemplateSetIamPolicyCall {
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
    pub fn regions_workflow_templates_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectRegionWorkflowTemplateTestIamPermissionCall<'a, S> {
        ProjectRegionWorkflowTemplateTestIamPermissionCall {
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
    /// Updates (replaces) workflow template. The updated template must contain version that matches the current server version.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates, the resource name of the template has the following format: projects/{project_id}/regions/{region}/workflowTemplates/{template_id} For projects.locations.workflowTemplates, the resource name of the template has the following format: projects/{project_id}/locations/{location}/workflowTemplates/{template_id}
    pub fn regions_workflow_templates_update(&self, request: WorkflowTemplate, name: &str) -> ProjectRegionWorkflowTemplateUpdateCall<'a, S> {
        ProjectRegionWorkflowTemplateUpdateCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



