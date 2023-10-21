use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`CloudDeploy`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_clouddeploy1 as clouddeploy1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use clouddeploy1::{CloudDeploy, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudDeploy::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_delivery_pipelines_create(...)`, `locations_delivery_pipelines_delete(...)`, `locations_delivery_pipelines_get(...)`, `locations_delivery_pipelines_get_iam_policy(...)`, `locations_delivery_pipelines_list(...)`, `locations_delivery_pipelines_patch(...)`, `locations_delivery_pipelines_releases_abandon(...)`, `locations_delivery_pipelines_releases_create(...)`, `locations_delivery_pipelines_releases_get(...)`, `locations_delivery_pipelines_releases_list(...)`, `locations_delivery_pipelines_releases_rollouts_approve(...)`, `locations_delivery_pipelines_releases_rollouts_create(...)`, `locations_delivery_pipelines_releases_rollouts_get(...)`, `locations_delivery_pipelines_releases_rollouts_job_runs_get(...)`, `locations_delivery_pipelines_releases_rollouts_job_runs_list(...)`, `locations_delivery_pipelines_releases_rollouts_list(...)`, `locations_delivery_pipelines_releases_rollouts_retry_job(...)`, `locations_delivery_pipelines_set_iam_policy(...)`, `locations_delivery_pipelines_test_iam_permissions(...)`, `locations_get(...)`, `locations_get_config(...)`, `locations_list(...)`, `locations_operations_cancel(...)`, `locations_operations_delete(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `locations_targets_create(...)`, `locations_targets_delete(...)`, `locations_targets_get(...)`, `locations_targets_get_iam_policy(...)`, `locations_targets_list(...)`, `locations_targets_patch(...)`, `locations_targets_set_iam_policy(...)` and `locations_targets_test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudDeploy<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single JobRun.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the `JobRun`. Format must be projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}/releases/{release_name}/rollouts/{rollout_name}/jobRuns/{job_run_name}.
    pub fn locations_delivery_pipelines_releases_rollouts_job_runs_get(&self, name: &str) -> ProjectLocationDeliveryPipelineReleaseRolloutJobRunGetCall<'a, S> {
        ProjectLocationDeliveryPipelineReleaseRolloutJobRunGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists JobRuns in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The `Rollout` which owns this collection of `JobRun` objects.
    pub fn locations_delivery_pipelines_releases_rollouts_job_runs_list(&self, parent: &str) -> ProjectLocationDeliveryPipelineReleaseRolloutJobRunListCall<'a, S> {
        ProjectLocationDeliveryPipelineReleaseRolloutJobRunListCall {
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
    /// Approves a Rollout.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the Rollout. Format is projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/ releases/{release}/rollouts/{rollout}.
    pub fn locations_delivery_pipelines_releases_rollouts_approve(&self, request: ApproveRolloutRequest, name: &str) -> ProjectLocationDeliveryPipelineReleaseRolloutApproveCall<'a, S> {
        ProjectLocationDeliveryPipelineReleaseRolloutApproveCall {
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
    /// Creates a new Rollout in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent collection in which the `Rollout` should be created. Format should be projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}/releases/{release_name}.
    pub fn locations_delivery_pipelines_releases_rollouts_create(&self, request: Rollout, parent: &str) -> ProjectLocationDeliveryPipelineReleaseRolloutCreateCall<'a, S> {
        ProjectLocationDeliveryPipelineReleaseRolloutCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _validate_only: Default::default(),
            _rollout_id: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single Rollout.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the `Rollout`. Format must be projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}/releases/{release_name}/rollouts/{rollout_name}.
    pub fn locations_delivery_pipelines_releases_rollouts_get(&self, name: &str) -> ProjectLocationDeliveryPipelineReleaseRolloutGetCall<'a, S> {
        ProjectLocationDeliveryPipelineReleaseRolloutGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists Rollouts in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The `Release` which owns this collection of `Rollout` objects.
    pub fn locations_delivery_pipelines_releases_rollouts_list(&self, parent: &str) -> ProjectLocationDeliveryPipelineReleaseRolloutListCall<'a, S> {
        ProjectLocationDeliveryPipelineReleaseRolloutListCall {
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
    /// Retries the specified Job in a Rollout.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `rollout` - Required. Name of the Rollout. Format is projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/ releases/{release}/rollouts/{rollout}.
    pub fn locations_delivery_pipelines_releases_rollouts_retry_job(&self, request: RetryJobRequest, rollout: &str) -> ProjectLocationDeliveryPipelineReleaseRolloutRetryJobCall<'a, S> {
        ProjectLocationDeliveryPipelineReleaseRolloutRetryJobCall {
            hub: self.hub,
            _request: request,
            _rollout: rollout.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Abandons a Release in the Delivery Pipeline.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the Release. Format is projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/ releases/{release}.
    pub fn locations_delivery_pipelines_releases_abandon(&self, request: AbandonReleaseRequest, name: &str) -> ProjectLocationDeliveryPipelineReleaseAbandonCall<'a, S> {
        ProjectLocationDeliveryPipelineReleaseAbandonCall {
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
    /// Creates a new Release in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent collection in which the `Release` should be created. Format should be projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}.
    pub fn locations_delivery_pipelines_releases_create(&self, request: Release, parent: &str) -> ProjectLocationDeliveryPipelineReleaseCreateCall<'a, S> {
        ProjectLocationDeliveryPipelineReleaseCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _validate_only: Default::default(),
            _request_id: Default::default(),
            _release_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single Release.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the `Release`. Format must be projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}/releases/{release_name}.
    pub fn locations_delivery_pipelines_releases_get(&self, name: &str) -> ProjectLocationDeliveryPipelineReleaseGetCall<'a, S> {
        ProjectLocationDeliveryPipelineReleaseGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists Releases in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The `DeliveryPipeline` which owns this collection of `Release` objects.
    pub fn locations_delivery_pipelines_releases_list(&self, parent: &str) -> ProjectLocationDeliveryPipelineReleaseListCall<'a, S> {
        ProjectLocationDeliveryPipelineReleaseListCall {
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
    /// Creates a new DeliveryPipeline in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent collection in which the `DeliveryPipeline` should be created. Format should be projects/{project_id}/locations/{location_name}.
    pub fn locations_delivery_pipelines_create(&self, request: DeliveryPipeline, parent: &str) -> ProjectLocationDeliveryPipelineCreateCall<'a, S> {
        ProjectLocationDeliveryPipelineCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _validate_only: Default::default(),
            _request_id: Default::default(),
            _delivery_pipeline_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single DeliveryPipeline.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the `DeliveryPipeline` to delete. Format should be projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}.
    pub fn locations_delivery_pipelines_delete(&self, name: &str) -> ProjectLocationDeliveryPipelineDeleteCall<'a, S> {
        ProjectLocationDeliveryPipelineDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _request_id: Default::default(),
            _force: Default::default(),
            _etag: Default::default(),
            _allow_missing: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single DeliveryPipeline.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the `DeliveryPipeline`. Format must be projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}.
    pub fn locations_delivery_pipelines_get(&self, name: &str) -> ProjectLocationDeliveryPipelineGetCall<'a, S> {
        ProjectLocationDeliveryPipelineGetCall {
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
    pub fn locations_delivery_pipelines_get_iam_policy(&self, resource: &str) -> ProjectLocationDeliveryPipelineGetIamPolicyCall<'a, S> {
        ProjectLocationDeliveryPipelineGetIamPolicyCall {
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
    /// Lists DeliveryPipelines in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns this collection of pipelines. Format must be projects/{project_id}/locations/{location_name}.
    pub fn locations_delivery_pipelines_list(&self, parent: &str) -> ProjectLocationDeliveryPipelineListCall<'a, S> {
        ProjectLocationDeliveryPipelineListCall {
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
    /// Updates the parameters of a single DeliveryPipeline.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Optional. Name of the `DeliveryPipeline`. Format is projects/{project}/ locations/{location}/deliveryPipelines/a-z{0,62}.
    pub fn locations_delivery_pipelines_patch(&self, request: DeliveryPipeline, name: &str) -> ProjectLocationDeliveryPipelinePatchCall<'a, S> {
        ProjectLocationDeliveryPipelinePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _request_id: Default::default(),
            _allow_missing: Default::default(),
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
    pub fn locations_delivery_pipelines_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationDeliveryPipelineSetIamPolicyCall<'a, S> {
        ProjectLocationDeliveryPipelineSetIamPolicyCall {
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
    pub fn locations_delivery_pipelines_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationDeliveryPipelineTestIamPermissionCall<'a, S> {
        ProjectLocationDeliveryPipelineTestIamPermissionCall {
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
    /// Creates a new Target in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent collection in which the `Target` should be created. Format should be projects/{project_id}/locations/{location_name}.
    pub fn locations_targets_create(&self, request: Target, parent: &str) -> ProjectLocationTargetCreateCall<'a, S> {
        ProjectLocationTargetCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _validate_only: Default::default(),
            _target_id: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single Target.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the `Target` to delete. Format should be projects/{project_id}/locations/{location_name}/targets/{target_name}.
    pub fn locations_targets_delete(&self, name: &str) -> ProjectLocationTargetDeleteCall<'a, S> {
        ProjectLocationTargetDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _request_id: Default::default(),
            _etag: Default::default(),
            _allow_missing: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single Target.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the `Target`. Format must be projects/{project_id}/locations/{location_name}/targets/{target_name}.
    pub fn locations_targets_get(&self, name: &str) -> ProjectLocationTargetGetCall<'a, S> {
        ProjectLocationTargetGetCall {
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
    pub fn locations_targets_get_iam_policy(&self, resource: &str) -> ProjectLocationTargetGetIamPolicyCall<'a, S> {
        ProjectLocationTargetGetIamPolicyCall {
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
    /// Lists Targets in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns this collection of targets. Format must be projects/{project_id}/locations/{location_name}.
    pub fn locations_targets_list(&self, parent: &str) -> ProjectLocationTargetListCall<'a, S> {
        ProjectLocationTargetListCall {
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
    /// Updates the parameters of a single Target.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Optional. Name of the `Target`. Format is projects/{project}/locations/{location}/targets/a-z{0,62}.
    pub fn locations_targets_patch(&self, request: Target, name: &str) -> ProjectLocationTargetPatchCall<'a, S> {
        ProjectLocationTargetPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _request_id: Default::default(),
            _allow_missing: Default::default(),
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
    pub fn locations_targets_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationTargetSetIamPolicyCall<'a, S> {
        ProjectLocationTargetSetIamPolicyCall {
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
    pub fn locations_targets_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationTargetTestIamPermissionCall<'a, S> {
        ProjectLocationTargetTestIamPermissionCall {
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
    /// Gets the configuration for a location.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of requested configuration.
    pub fn locations_get_config(&self, name: &str) -> ProjectLocationGetConfigCall<'a, S> {
        ProjectLocationGetConfigCall {
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



