use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`CloudMachineLearningEngine`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_ml1 as ml1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use ml1::{CloudMachineLearningEngine, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudMachineLearningEngine::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `explain(...)`, `get_config(...)`, `jobs_cancel(...)`, `jobs_create(...)`, `jobs_get(...)`, `jobs_get_iam_policy(...)`, `jobs_list(...)`, `jobs_patch(...)`, `jobs_set_iam_policy(...)`, `jobs_test_iam_permissions(...)`, `locations_get(...)`, `locations_list(...)`, `locations_operations_cancel(...)`, `locations_operations_get(...)`, `locations_studies_create(...)`, `locations_studies_delete(...)`, `locations_studies_get(...)`, `locations_studies_list(...)`, `locations_studies_trials_add_measurement(...)`, `locations_studies_trials_check_early_stopping_state(...)`, `locations_studies_trials_complete(...)`, `locations_studies_trials_create(...)`, `locations_studies_trials_delete(...)`, `locations_studies_trials_get(...)`, `locations_studies_trials_list(...)`, `locations_studies_trials_list_optimal_trials(...)`, `locations_studies_trials_stop(...)`, `locations_studies_trials_suggest(...)`, `models_create(...)`, `models_delete(...)`, `models_get(...)`, `models_get_iam_policy(...)`, `models_list(...)`, `models_patch(...)`, `models_set_iam_policy(...)`, `models_test_iam_permissions(...)`, `models_versions_create(...)`, `models_versions_delete(...)`, `models_versions_get(...)`, `models_versions_list(...)`, `models_versions_patch(...)`, `models_versions_set_default(...)`, `operations_cancel(...)`, `operations_get(...)`, `operations_list(...)` and `predict(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudMachineLearningEngine<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancels a running job.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the job to cancel.
    pub fn jobs_cancel(&self, request: GoogleCloudMlV1__CancelJobRequest, name: &str) -> ProjectJobCancelCall<'a, S> {
        ProjectJobCancelCall {
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
    /// Creates a training or a batch prediction job.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The project name.
    pub fn jobs_create(&self, request: GoogleCloudMlV1__Job, parent: &str) -> ProjectJobCreateCall<'a, S> {
        ProjectJobCreateCall {
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
    /// Describes a job.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the job to get the description of.
    pub fn jobs_get(&self, name: &str) -> ProjectJobGetCall<'a, S> {
        ProjectJobGetCall {
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
    pub fn jobs_get_iam_policy(&self, resource: &str) -> ProjectJobGetIamPolicyCall<'a, S> {
        ProjectJobGetIamPolicyCall {
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
    /// Lists the jobs in the project. If there are no jobs that match the request parameters, the list request returns an empty response body: {}.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the project for which to list jobs.
    pub fn jobs_list(&self, parent: &str) -> ProjectJobListCall<'a, S> {
        ProjectJobListCall {
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
    /// Updates a specific job resource. Currently the only supported fields to update are `labels`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The job name.
    pub fn jobs_patch(&self, request: GoogleCloudMlV1__Job, name: &str) -> ProjectJobPatchCall<'a, S> {
        ProjectJobPatchCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn jobs_set_iam_policy(&self, request: GoogleIamV1__SetIamPolicyRequest, resource: &str) -> ProjectJobSetIamPolicyCall<'a, S> {
        ProjectJobSetIamPolicyCall {
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
    pub fn jobs_test_iam_permissions(&self, request: GoogleIamV1__TestIamPermissionsRequest, resource: &str) -> ProjectJobTestIamPermissionCall<'a, S> {
        ProjectJobTestIamPermissionCall {
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
    /// Adds a measurement of the objective metrics to a trial. This measurement is assumed to have been taken before the trial is complete.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The trial name.
    pub fn locations_studies_trials_add_measurement(&self, request: GoogleCloudMlV1__AddTrialMeasurementRequest, name: &str) -> ProjectLocationStudyTrialAddMeasurementCall<'a, S> {
        ProjectLocationStudyTrialAddMeasurementCall {
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
    /// Checks whether a trial should stop or not. Returns a long-running operation. When the operation is successful, it will contain a CheckTrialEarlyStoppingStateResponse.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The trial name.
    pub fn locations_studies_trials_check_early_stopping_state(&self, request: GoogleCloudMlV1__CheckTrialEarlyStoppingStateRequest, name: &str) -> ProjectLocationStudyTrialCheckEarlyStoppingStateCall<'a, S> {
        ProjectLocationStudyTrialCheckEarlyStoppingStateCall {
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
    /// Marks a trial as complete.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The trial name.metat
    pub fn locations_studies_trials_complete(&self, request: GoogleCloudMlV1__CompleteTrialRequest, name: &str) -> ProjectLocationStudyTrialCompleteCall<'a, S> {
        ProjectLocationStudyTrialCompleteCall {
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
    /// Adds a user provided trial to a study.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the study that the trial belongs to.
    pub fn locations_studies_trials_create(&self, request: GoogleCloudMlV1__Trial, parent: &str) -> ProjectLocationStudyTrialCreateCall<'a, S> {
        ProjectLocationStudyTrialCreateCall {
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
    /// Deletes a trial.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The trial name.
    pub fn locations_studies_trials_delete(&self, name: &str) -> ProjectLocationStudyTrialDeleteCall<'a, S> {
        ProjectLocationStudyTrialDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a trial.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The trial name.
    pub fn locations_studies_trials_get(&self, name: &str) -> ProjectLocationStudyTrialGetCall<'a, S> {
        ProjectLocationStudyTrialGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the trials associated with a study.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the study that the trial belongs to.
    pub fn locations_studies_trials_list(&self, parent: &str) -> ProjectLocationStudyTrialListCall<'a, S> {
        ProjectLocationStudyTrialListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the pareto-optimal trials for multi-objective study or the optimal trials for single-objective study. The definition of pareto-optimal can be checked in wiki page. https://en.wikipedia.org/wiki/Pareto_efficiency
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the study that the pareto-optimal trial belongs to.
    pub fn locations_studies_trials_list_optimal_trials(&self, request: GoogleCloudMlV1__ListOptimalTrialsRequest, parent: &str) -> ProjectLocationStudyTrialListOptimalTrialCall<'a, S> {
        ProjectLocationStudyTrialListOptimalTrialCall {
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
    /// Stops a trial.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The trial name.
    pub fn locations_studies_trials_stop(&self, request: GoogleCloudMlV1__StopTrialRequest, name: &str) -> ProjectLocationStudyTrialStopCall<'a, S> {
        ProjectLocationStudyTrialStopCall {
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
    /// Adds one or more trials to a study, with parameter values suggested by AI Platform Vizier. Returns a long-running operation associated with the generation of trial suggestions. When this long-running operation succeeds, it will contain a SuggestTrialsResponse.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the study that the trial belongs to.
    pub fn locations_studies_trials_suggest(&self, request: GoogleCloudMlV1__SuggestTrialsRequest, parent: &str) -> ProjectLocationStudyTrialSuggestCall<'a, S> {
        ProjectLocationStudyTrialSuggestCall {
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
    /// Creates a study.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The project and location that the study belongs to. Format: projects/{project}/locations/{location}
    pub fn locations_studies_create(&self, request: GoogleCloudMlV1__Study, parent: &str) -> ProjectLocationStudyCreateCall<'a, S> {
        ProjectLocationStudyCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _study_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a study.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The study name.
    pub fn locations_studies_delete(&self, name: &str) -> ProjectLocationStudyDeleteCall<'a, S> {
        ProjectLocationStudyDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a study.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The study name.
    pub fn locations_studies_get(&self, name: &str) -> ProjectLocationStudyGetCall<'a, S> {
        ProjectLocationStudyGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the studies in a region for an associated project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and location that the study belongs to. Format: projects/{project}/locations/{location}
    pub fn locations_studies_list(&self, parent: &str) -> ProjectLocationStudyListCall<'a, S> {
        ProjectLocationStudyListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get the complete list of CMLE capabilities in a location, along with their location-specific properties.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the location.
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
    /// List all locations that provides at least one type of CMLE capability.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the project for which available locations are to be listed (since some locations might be whitelisted for specific projects).
    pub fn locations_list(&self, parent: &str) -> ProjectLocationListCall<'a, S> {
        ProjectLocationListCall {
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
    /// Creates a new version of a model from a trained TensorFlow model. If the version created in the cloud by this call is the first deployed version of the specified model, it will be made the default version of the model. When you add a version to a model that already has one or more versions, the default version does not automatically change. If you want a new version to be the default, you must call projects.models.versions.setDefault.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the model.
    pub fn models_versions_create(&self, request: GoogleCloudMlV1__Version, parent: &str) -> ProjectModelVersionCreateCall<'a, S> {
        ProjectModelVersionCreateCall {
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
    /// Deletes a model version. Each model can have multiple versions deployed and in use at any given time. Use this method to remove a single version. Note: You cannot delete the version that is set as the default version of the model unless it is the only remaining version.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the version. You can get the names of all the versions of a model by calling projects.models.versions.list.
    pub fn models_versions_delete(&self, name: &str) -> ProjectModelVersionDeleteCall<'a, S> {
        ProjectModelVersionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about a model version. Models can have multiple versions. You can call projects.models.versions.list to get the same information that this method returns for all of the versions of a model.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the version.
    pub fn models_versions_get(&self, name: &str) -> ProjectModelVersionGetCall<'a, S> {
        ProjectModelVersionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets basic information about all the versions of a model. If you expect that a model has many versions, or if you need to handle only a limited number of results at a time, you can request that the list be retrieved in batches (called pages). If there are no versions that match the request parameters, the list request returns an empty response body: {}.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the model for which to list the version.
    pub fn models_versions_list(&self, parent: &str) -> ProjectModelVersionListCall<'a, S> {
        ProjectModelVersionListCall {
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
    /// Updates the specified Version resource. Currently the only update-able fields are `description`, `requestLoggingConfig`, `autoScaling.minNodes`, and `manualScaling.nodes`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the model.
    pub fn models_versions_patch(&self, request: GoogleCloudMlV1__Version, name: &str) -> ProjectModelVersionPatchCall<'a, S> {
        ProjectModelVersionPatchCall {
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
    /// Designates a version to be the default for the model. The default version is used for prediction requests made against the model that don't specify a version. The first version to be created for a model is automatically set as the default. You must make any subsequent changes to the default version setting manually using this method.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the version to make the default for the model. You can get the names of all the versions of a model by calling projects.models.versions.list.
    pub fn models_versions_set_default(&self, request: GoogleCloudMlV1__SetDefaultVersionRequest, name: &str) -> ProjectModelVersionSetDefaultCall<'a, S> {
        ProjectModelVersionSetDefaultCall {
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
    /// Creates a model which will later contain one or more versions. You must add at least one version before you can request predictions from the model. Add versions by calling projects.models.versions.create.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The project name.
    pub fn models_create(&self, request: GoogleCloudMlV1__Model, parent: &str) -> ProjectModelCreateCall<'a, S> {
        ProjectModelCreateCall {
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
    /// Deletes a model. You can only delete a model if there are no versions in it. You can delete versions by calling projects.models.versions.delete.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the model.
    pub fn models_delete(&self, name: &str) -> ProjectModelDeleteCall<'a, S> {
        ProjectModelDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about a model, including its name, the description (if set), and the default version (if at least one version of the model has been deployed).
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the model.
    pub fn models_get(&self, name: &str) -> ProjectModelGetCall<'a, S> {
        ProjectModelGetCall {
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
    pub fn models_get_iam_policy(&self, resource: &str) -> ProjectModelGetIamPolicyCall<'a, S> {
        ProjectModelGetIamPolicyCall {
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
    /// Lists the models in a project. Each project can contain multiple models, and each model can have multiple versions. If there are no models that match the request parameters, the list request returns an empty response body: {}.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the project whose models are to be listed.
    pub fn models_list(&self, parent: &str) -> ProjectModelListCall<'a, S> {
        ProjectModelListCall {
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
    /// Updates a specific model resource. Currently the only supported fields to update are `description` and `default_version.name`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The project name.
    pub fn models_patch(&self, request: GoogleCloudMlV1__Model, name: &str) -> ProjectModelPatchCall<'a, S> {
        ProjectModelPatchCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn models_set_iam_policy(&self, request: GoogleIamV1__SetIamPolicyRequest, resource: &str) -> ProjectModelSetIamPolicyCall<'a, S> {
        ProjectModelSetIamPolicyCall {
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
    pub fn models_test_iam_permissions(&self, request: GoogleIamV1__TestIamPermissionsRequest, resource: &str) -> ProjectModelTestIamPermissionCall<'a, S> {
        ProjectModelTestIamPermissionCall {
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
    pub fn operations_cancel(&self, name: &str) -> ProjectOperationCancelCall<'a, S> {
        ProjectOperationCancelCall {
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
    /// Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `"/v1/{name=users/*}/operations"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn operations_list(&self, name: &str) -> ProjectOperationListCall<'a, S> {
        ProjectOperationListCall {
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
    /// Performs explanation on the data in the request. {% dynamic include "/ai-platform/includes/___explain-request" %} 
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of a model or a version. Authorization: requires the `predict` permission on the specified resource.
    pub fn explain(&self, request: GoogleCloudMlV1__ExplainRequest, name: &str) -> ProjectExplainCall<'a, S> {
        ProjectExplainCall {
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
    /// Get the service account information associated with your project. You need this information in order to grant the service account permissions for the Google Cloud Storage location where you put your model training code for training the model with Google Cloud Machine Learning.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The project name.
    pub fn get_config(&self, name: &str) -> ProjectGetConfigCall<'a, S> {
        ProjectGetConfigCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Performs online prediction on the data in the request. {% dynamic include "/ai-platform/includes/___predict-request" %} 
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of a model or a version. Authorization: requires the `predict` permission on the specified resource.
    pub fn predict(&self, request: GoogleCloudMlV1__PredictRequest, name: &str) -> ProjectPredictCall<'a, S> {
        ProjectPredictCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



