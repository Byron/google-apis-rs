use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`ContainerAnalysis`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_containeranalysis1 as containeranalysis1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use containeranalysis1::{ContainerAnalysis, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ContainerAnalysis::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `notes_batch_create(...)`, `notes_create(...)`, `notes_delete(...)`, `notes_get(...)`, `notes_get_iam_policy(...)`, `notes_list(...)`, `notes_occurrences_list(...)`, `notes_patch(...)`, `notes_set_iam_policy(...)`, `notes_test_iam_permissions(...)`, `occurrences_batch_create(...)`, `occurrences_create(...)`, `occurrences_delete(...)`, `occurrences_get(...)`, `occurrences_get_iam_policy(...)`, `occurrences_get_notes(...)`, `occurrences_get_vulnerability_summary(...)`, `occurrences_list(...)`, `occurrences_patch(...)`, `occurrences_set_iam_policy(...)` and `occurrences_test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ContainerAnalysis<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists occurrences referencing the specified note. Provider projects can use this method to get all occurrences across consumer projects referencing the specified note.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the note to list occurrences for in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`.
    pub fn notes_occurrences_list(&self, name: &str) -> ProjectNoteOccurrenceListCall<'a, S> {
        ProjectNoteOccurrenceListCall {
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
    /// Creates new notes in batch.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the project in the form of `projects/[PROJECT_ID]`, under which the notes are to be created.
    pub fn notes_batch_create(&self, request: BatchCreateNotesRequest, parent: &str) -> ProjectNoteBatchCreateCall<'a, S> {
        ProjectNoteBatchCreateCall {
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
    /// Creates a new note.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the project in the form of `projects/[PROJECT_ID]`, under which the note is to be created.
    pub fn notes_create(&self, request: Note, parent: &str) -> ProjectNoteCreateCall<'a, S> {
        ProjectNoteCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _note_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified note.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the note in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`.
    pub fn notes_delete(&self, name: &str) -> ProjectNoteDeleteCall<'a, S> {
        ProjectNoteDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified note.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the note in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`.
    pub fn notes_get(&self, name: &str) -> ProjectNoteGetCall<'a, S> {
        ProjectNoteGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a note or an occurrence resource. Requires `containeranalysis.notes.setIamPolicy` or `containeranalysis.occurrences.setIamPolicy` permission if the resource is a note or occurrence, respectively. The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn notes_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectNoteGetIamPolicyCall<'a, S> {
        ProjectNoteGetIamPolicyCall {
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
    /// Lists notes for the specified project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the project to list notes for in the form of `projects/[PROJECT_ID]`.
    pub fn notes_list(&self, parent: &str) -> ProjectNoteListCall<'a, S> {
        ProjectNoteListCall {
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
    /// Updates the specified note.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the note in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`.
    pub fn notes_patch(&self, request: Note, name: &str) -> ProjectNotePatchCall<'a, S> {
        ProjectNotePatchCall {
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
    /// Sets the access control policy on the specified note or occurrence. Requires `containeranalysis.notes.setIamPolicy` or `containeranalysis.occurrences.setIamPolicy` permission if the resource is a note or an occurrence, respectively. The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn notes_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectNoteSetIamPolicyCall<'a, S> {
        ProjectNoteSetIamPolicyCall {
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
    /// Returns the permissions that a caller has on the specified note or occurrence. Requires list permission on the project (for example, `containeranalysis.notes.list`). The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn notes_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectNoteTestIamPermissionCall<'a, S> {
        ProjectNoteTestIamPermissionCall {
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
    /// Creates new occurrences in batch.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the project in the form of `projects/[PROJECT_ID]`, under which the occurrences are to be created.
    pub fn occurrences_batch_create(&self, request: BatchCreateOccurrencesRequest, parent: &str) -> ProjectOccurrenceBatchCreateCall<'a, S> {
        ProjectOccurrenceBatchCreateCall {
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
    /// Creates a new occurrence.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the project in the form of `projects/[PROJECT_ID]`, under which the occurrence is to be created.
    pub fn occurrences_create(&self, request: Occurrence, parent: &str) -> ProjectOccurrenceCreateCall<'a, S> {
        ProjectOccurrenceCreateCall {
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
    /// Deletes the specified occurrence. For example, use this method to delete an occurrence when the occurrence is no longer applicable for the given resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`.
    pub fn occurrences_delete(&self, name: &str) -> ProjectOccurrenceDeleteCall<'a, S> {
        ProjectOccurrenceDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified occurrence.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`.
    pub fn occurrences_get(&self, name: &str) -> ProjectOccurrenceGetCall<'a, S> {
        ProjectOccurrenceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a note or an occurrence resource. Requires `containeranalysis.notes.setIamPolicy` or `containeranalysis.occurrences.setIamPolicy` permission if the resource is a note or occurrence, respectively. The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn occurrences_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectOccurrenceGetIamPolicyCall<'a, S> {
        ProjectOccurrenceGetIamPolicyCall {
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
    /// Gets the note attached to the specified occurrence. Consumer projects can use this method to get a note that belongs to a provider project.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`.
    pub fn occurrences_get_notes(&self, name: &str) -> ProjectOccurrenceGetNoteCall<'a, S> {
        ProjectOccurrenceGetNoteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a summary of the number and severity of occurrences.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the project to get a vulnerability summary for in the form of `projects/[PROJECT_ID]`.
    pub fn occurrences_get_vulnerability_summary(&self, parent: &str) -> ProjectOccurrenceGetVulnerabilitySummaryCall<'a, S> {
        ProjectOccurrenceGetVulnerabilitySummaryCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists occurrences for the specified project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the project to list occurrences for in the form of `projects/[PROJECT_ID]`.
    pub fn occurrences_list(&self, parent: &str) -> ProjectOccurrenceListCall<'a, S> {
        ProjectOccurrenceListCall {
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
    /// Updates the specified occurrence.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`.
    pub fn occurrences_patch(&self, request: Occurrence, name: &str) -> ProjectOccurrencePatchCall<'a, S> {
        ProjectOccurrencePatchCall {
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
    /// Sets the access control policy on the specified note or occurrence. Requires `containeranalysis.notes.setIamPolicy` or `containeranalysis.occurrences.setIamPolicy` permission if the resource is a note or an occurrence, respectively. The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn occurrences_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectOccurrenceSetIamPolicyCall<'a, S> {
        ProjectOccurrenceSetIamPolicyCall {
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
    /// Returns the permissions that a caller has on the specified note or occurrence. Requires list permission on the project (for example, `containeranalysis.notes.list`). The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn occurrences_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectOccurrenceTestIamPermissionCall<'a, S> {
        ProjectOccurrenceTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



