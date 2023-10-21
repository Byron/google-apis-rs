use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`CloudHealthcare`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_healthcare1 as healthcare1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use healthcare1::{CloudHealthcare, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudHealthcare::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_datasets_consent_stores_attribute_definitions_create(...)`, `locations_datasets_consent_stores_attribute_definitions_delete(...)`, `locations_datasets_consent_stores_attribute_definitions_get(...)`, `locations_datasets_consent_stores_attribute_definitions_list(...)`, `locations_datasets_consent_stores_attribute_definitions_patch(...)`, `locations_datasets_consent_stores_check_data_access(...)`, `locations_datasets_consent_stores_consent_artifacts_create(...)`, `locations_datasets_consent_stores_consent_artifacts_delete(...)`, `locations_datasets_consent_stores_consent_artifacts_get(...)`, `locations_datasets_consent_stores_consent_artifacts_list(...)`, `locations_datasets_consent_stores_consents_activate(...)`, `locations_datasets_consent_stores_consents_create(...)`, `locations_datasets_consent_stores_consents_delete(...)`, `locations_datasets_consent_stores_consents_delete_revision(...)`, `locations_datasets_consent_stores_consents_get(...)`, `locations_datasets_consent_stores_consents_list(...)`, `locations_datasets_consent_stores_consents_list_revisions(...)`, `locations_datasets_consent_stores_consents_patch(...)`, `locations_datasets_consent_stores_consents_reject(...)`, `locations_datasets_consent_stores_consents_revoke(...)`, `locations_datasets_consent_stores_create(...)`, `locations_datasets_consent_stores_delete(...)`, `locations_datasets_consent_stores_evaluate_user_consents(...)`, `locations_datasets_consent_stores_get(...)`, `locations_datasets_consent_stores_get_iam_policy(...)`, `locations_datasets_consent_stores_list(...)`, `locations_datasets_consent_stores_patch(...)`, `locations_datasets_consent_stores_query_accessible_data(...)`, `locations_datasets_consent_stores_set_iam_policy(...)`, `locations_datasets_consent_stores_test_iam_permissions(...)`, `locations_datasets_consent_stores_user_data_mappings_archive(...)`, `locations_datasets_consent_stores_user_data_mappings_create(...)`, `locations_datasets_consent_stores_user_data_mappings_delete(...)`, `locations_datasets_consent_stores_user_data_mappings_get(...)`, `locations_datasets_consent_stores_user_data_mappings_list(...)`, `locations_datasets_consent_stores_user_data_mappings_patch(...)`, `locations_datasets_create(...)`, `locations_datasets_deidentify(...)`, `locations_datasets_delete(...)`, `locations_datasets_dicom_stores_create(...)`, `locations_datasets_dicom_stores_deidentify(...)`, `locations_datasets_dicom_stores_delete(...)`, `locations_datasets_dicom_stores_export(...)`, `locations_datasets_dicom_stores_get(...)`, `locations_datasets_dicom_stores_get_iam_policy(...)`, `locations_datasets_dicom_stores_import(...)`, `locations_datasets_dicom_stores_list(...)`, `locations_datasets_dicom_stores_patch(...)`, `locations_datasets_dicom_stores_search_for_instances(...)`, `locations_datasets_dicom_stores_search_for_series(...)`, `locations_datasets_dicom_stores_search_for_studies(...)`, `locations_datasets_dicom_stores_set_iam_policy(...)`, `locations_datasets_dicom_stores_store_instances(...)`, `locations_datasets_dicom_stores_studies_delete(...)`, `locations_datasets_dicom_stores_studies_retrieve_metadata(...)`, `locations_datasets_dicom_stores_studies_retrieve_study(...)`, `locations_datasets_dicom_stores_studies_search_for_instances(...)`, `locations_datasets_dicom_stores_studies_search_for_series(...)`, `locations_datasets_dicom_stores_studies_series_delete(...)`, `locations_datasets_dicom_stores_studies_series_instances_delete(...)`, `locations_datasets_dicom_stores_studies_series_instances_frames_retrieve_frames(...)`, `locations_datasets_dicom_stores_studies_series_instances_frames_retrieve_rendered(...)`, `locations_datasets_dicom_stores_studies_series_instances_retrieve_instance(...)`, `locations_datasets_dicom_stores_studies_series_instances_retrieve_metadata(...)`, `locations_datasets_dicom_stores_studies_series_instances_retrieve_rendered(...)`, `locations_datasets_dicom_stores_studies_series_retrieve_metadata(...)`, `locations_datasets_dicom_stores_studies_series_retrieve_series(...)`, `locations_datasets_dicom_stores_studies_series_search_for_instances(...)`, `locations_datasets_dicom_stores_studies_store_instances(...)`, `locations_datasets_dicom_stores_test_iam_permissions(...)`, `locations_datasets_fhir_stores_create(...)`, `locations_datasets_fhir_stores_deidentify(...)`, `locations_datasets_fhir_stores_delete(...)`, `locations_datasets_fhir_stores_export(...)`, `locations_datasets_fhir_stores_fhir__patient_everything(...)`, `locations_datasets_fhir_stores_fhir__resource_purge(...)`, `locations_datasets_fhir_stores_fhir__resource_validate(...)`, `locations_datasets_fhir_stores_fhir_capabilities(...)`, `locations_datasets_fhir_stores_fhir_create(...)`, `locations_datasets_fhir_stores_fhir_delete(...)`, `locations_datasets_fhir_stores_fhir_execute_bundle(...)`, `locations_datasets_fhir_stores_fhir_history(...)`, `locations_datasets_fhir_stores_fhir_patch(...)`, `locations_datasets_fhir_stores_fhir_read(...)`, `locations_datasets_fhir_stores_fhir_search(...)`, `locations_datasets_fhir_stores_fhir_search_type(...)`, `locations_datasets_fhir_stores_fhir_update(...)`, `locations_datasets_fhir_stores_fhir_vread(...)`, `locations_datasets_fhir_stores_get(...)`, `locations_datasets_fhir_stores_get_iam_policy(...)`, `locations_datasets_fhir_stores_import(...)`, `locations_datasets_fhir_stores_list(...)`, `locations_datasets_fhir_stores_patch(...)`, `locations_datasets_fhir_stores_set_iam_policy(...)`, `locations_datasets_fhir_stores_test_iam_permissions(...)`, `locations_datasets_get(...)`, `locations_datasets_get_iam_policy(...)`, `locations_datasets_hl7_v2_stores_create(...)`, `locations_datasets_hl7_v2_stores_delete(...)`, `locations_datasets_hl7_v2_stores_export(...)`, `locations_datasets_hl7_v2_stores_get(...)`, `locations_datasets_hl7_v2_stores_get_iam_policy(...)`, `locations_datasets_hl7_v2_stores_import(...)`, `locations_datasets_hl7_v2_stores_list(...)`, `locations_datasets_hl7_v2_stores_messages_create(...)`, `locations_datasets_hl7_v2_stores_messages_delete(...)`, `locations_datasets_hl7_v2_stores_messages_get(...)`, `locations_datasets_hl7_v2_stores_messages_ingest(...)`, `locations_datasets_hl7_v2_stores_messages_list(...)`, `locations_datasets_hl7_v2_stores_messages_patch(...)`, `locations_datasets_hl7_v2_stores_patch(...)`, `locations_datasets_hl7_v2_stores_set_iam_policy(...)`, `locations_datasets_hl7_v2_stores_test_iam_permissions(...)`, `locations_datasets_list(...)`, `locations_datasets_operations_cancel(...)`, `locations_datasets_operations_get(...)`, `locations_datasets_operations_list(...)`, `locations_datasets_patch(...)`, `locations_datasets_set_iam_policy(...)`, `locations_datasets_test_iam_permissions(...)`, `locations_get(...)`, `locations_list(...)` and `locations_services_nlp_analyze_entities(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudHealthcare<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new Attribute definition in the parent consent store.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the consent store that this Attribute definition belongs to.
    pub fn locations_datasets_consent_stores_attribute_definitions_create(&self, request: AttributeDefinition, parent: &str) -> ProjectLocationDatasetConsentStoreAttributeDefinitionCreateCall<'a, S> {
        ProjectLocationDatasetConsentStoreAttributeDefinitionCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _attribute_definition_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified Attribute definition. Fails if the Attribute definition is referenced by any User data mapping, or the latest revision of any Consent.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the Attribute definition to delete. To preserve referential integrity, Attribute definitions referenced by a User data mapping or the latest revision of a Consent cannot be deleted.
    pub fn locations_datasets_consent_stores_attribute_definitions_delete(&self, name: &str) -> ProjectLocationDatasetConsentStoreAttributeDefinitionDeleteCall<'a, S> {
        ProjectLocationDatasetConsentStoreAttributeDefinitionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified Attribute definition.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the Attribute definition to get.
    pub fn locations_datasets_consent_stores_attribute_definitions_get(&self, name: &str) -> ProjectLocationDatasetConsentStoreAttributeDefinitionGetCall<'a, S> {
        ProjectLocationDatasetConsentStoreAttributeDefinitionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the Attribute definitions in the specified consent store.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the consent store to retrieve Attribute definitions from.
    pub fn locations_datasets_consent_stores_attribute_definitions_list(&self, parent: &str) -> ProjectLocationDatasetConsentStoreAttributeDefinitionListCall<'a, S> {
        ProjectLocationDatasetConsentStoreAttributeDefinitionListCall {
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
    /// Updates the specified Attribute definition.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Resource name of the Attribute definition, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/attributeDefinitions/{attribute_definition_id}`. Cannot be changed after creation.
    pub fn locations_datasets_consent_stores_attribute_definitions_patch(&self, request: AttributeDefinition, name: &str) -> ProjectLocationDatasetConsentStoreAttributeDefinitionPatchCall<'a, S> {
        ProjectLocationDatasetConsentStoreAttributeDefinitionPatchCall {
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
    /// Creates a new Consent artifact in the parent consent store.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the consent store this Consent artifact belongs to.
    pub fn locations_datasets_consent_stores_consent_artifacts_create(&self, request: ConsentArtifact, parent: &str) -> ProjectLocationDatasetConsentStoreConsentArtifactCreateCall<'a, S> {
        ProjectLocationDatasetConsentStoreConsentArtifactCreateCall {
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
    /// Deletes the specified Consent artifact. Fails if the artifact is referenced by the latest revision of any Consent.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the Consent artifact to delete. To preserve referential integrity, Consent artifacts referenced by the latest revision of a Consent cannot be deleted.
    pub fn locations_datasets_consent_stores_consent_artifacts_delete(&self, name: &str) -> ProjectLocationDatasetConsentStoreConsentArtifactDeleteCall<'a, S> {
        ProjectLocationDatasetConsentStoreConsentArtifactDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified Consent artifact.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the Consent artifact to retrieve.
    pub fn locations_datasets_consent_stores_consent_artifacts_get(&self, name: &str) -> ProjectLocationDatasetConsentStoreConsentArtifactGetCall<'a, S> {
        ProjectLocationDatasetConsentStoreConsentArtifactGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the Consent artifacts in the specified consent store.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the consent store to retrieve consent artifacts from.
    pub fn locations_datasets_consent_stores_consent_artifacts_list(&self, parent: &str) -> ProjectLocationDatasetConsentStoreConsentArtifactListCall<'a, S> {
        ProjectLocationDatasetConsentStoreConsentArtifactListCall {
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
    /// Activates the latest revision of the specified Consent by committing a new revision with `state` updated to `ACTIVE`. If the latest revision of the specified Consent is in the `ACTIVE` state, no new revision is committed. A FAILED_PRECONDITION error occurs if the latest revision of the specified Consent is in the `REJECTED` or `REVOKED` state.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the Consent to activate, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consents/{consent_id}`. An INVALID_ARGUMENT error occurs if `revision_id` is specified in the name.
    pub fn locations_datasets_consent_stores_consents_activate(&self, request: ActivateConsentRequest, name: &str) -> ProjectLocationDatasetConsentStoreConsentActivateCall<'a, S> {
        ProjectLocationDatasetConsentStoreConsentActivateCall {
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
    /// Creates a new Consent in the parent consent store.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the consent store.
    pub fn locations_datasets_consent_stores_consents_create(&self, request: Consent, parent: &str) -> ProjectLocationDatasetConsentStoreConsentCreateCall<'a, S> {
        ProjectLocationDatasetConsentStoreConsentCreateCall {
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
    /// Deletes the Consent and its revisions. To keep a record of the Consent but mark it inactive, see [RevokeConsent]. To delete a revision of a Consent, see [DeleteConsentRevision]. This operation does not delete the related Consent artifact.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the Consent to delete, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consents/{consent_id}`. An INVALID_ARGUMENT error occurs if `revision_id` is specified in the name.
    pub fn locations_datasets_consent_stores_consents_delete(&self, name: &str) -> ProjectLocationDatasetConsentStoreConsentDeleteCall<'a, S> {
        ProjectLocationDatasetConsentStoreConsentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified revision of a Consent. An INVALID_ARGUMENT error occurs if the specified revision is the latest revision.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the Consent revision to delete, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consents/{consent_id}@{revision_id}`. An INVALID_ARGUMENT error occurs if `revision_id` is not specified in the name.
    pub fn locations_datasets_consent_stores_consents_delete_revision(&self, name: &str) -> ProjectLocationDatasetConsentStoreConsentDeleteRevisionCall<'a, S> {
        ProjectLocationDatasetConsentStoreConsentDeleteRevisionCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified revision of a Consent, or the latest revision if `revision_id` is not specified in the resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the Consent to retrieve, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consents/{consent_id}`. In order to retrieve a previous revision of the Consent, also provide the revision ID: `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consents/{consent_id}@{revision_id}`
    pub fn locations_datasets_consent_stores_consents_get(&self, name: &str) -> ProjectLocationDatasetConsentStoreConsentGetCall<'a, S> {
        ProjectLocationDatasetConsentStoreConsentGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the Consent in the given consent store, returning each Consent's latest revision.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the consent store to retrieve Consents from.
    pub fn locations_datasets_consent_stores_consents_list(&self, parent: &str) -> ProjectLocationDatasetConsentStoreConsentListCall<'a, S> {
        ProjectLocationDatasetConsentStoreConsentListCall {
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
    /// Lists the revisions of the specified Consent in reverse chronological order.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the Consent to retrieve revisions for.
    pub fn locations_datasets_consent_stores_consents_list_revisions(&self, name: &str) -> ProjectLocationDatasetConsentStoreConsentListRevisionCall<'a, S> {
        ProjectLocationDatasetConsentStoreConsentListRevisionCall {
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
    /// Updates the latest revision of the specified Consent by committing a new revision with the changes. A FAILED_PRECONDITION error occurs if the latest revision of the specified Consent is in the `REJECTED` or `REVOKED` state.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Resource name of the Consent, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consents/{consent_id}`. Cannot be changed after creation.
    pub fn locations_datasets_consent_stores_consents_patch(&self, request: Consent, name: &str) -> ProjectLocationDatasetConsentStoreConsentPatchCall<'a, S> {
        ProjectLocationDatasetConsentStoreConsentPatchCall {
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
    /// Rejects the latest revision of the specified Consent by committing a new revision with `state` updated to `REJECTED`. If the latest revision of the specified Consent is in the `REJECTED` state, no new revision is committed. A FAILED_PRECONDITION error occurs if the latest revision of the specified Consent is in the `ACTIVE` or `REVOKED` state.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the Consent to reject, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consents/{consent_id}`. An INVALID_ARGUMENT error occurs if `revision_id` is specified in the name.
    pub fn locations_datasets_consent_stores_consents_reject(&self, request: RejectConsentRequest, name: &str) -> ProjectLocationDatasetConsentStoreConsentRejectCall<'a, S> {
        ProjectLocationDatasetConsentStoreConsentRejectCall {
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
    /// Revokes the latest revision of the specified Consent by committing a new revision with `state` updated to `REVOKED`. If the latest revision of the specified Consent is in the `REVOKED` state, no new revision is committed. A FAILED_PRECONDITION error occurs if the latest revision of the given consent is in `DRAFT` or `REJECTED` state.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the Consent to revoke, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consents/{consent_id}`. An INVALID_ARGUMENT error occurs if `revision_id` is specified in the name.
    pub fn locations_datasets_consent_stores_consents_revoke(&self, request: RevokeConsentRequest, name: &str) -> ProjectLocationDatasetConsentStoreConsentRevokeCall<'a, S> {
        ProjectLocationDatasetConsentStoreConsentRevokeCall {
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
    /// Archives the specified User data mapping.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the User data mapping to archive.
    pub fn locations_datasets_consent_stores_user_data_mappings_archive(&self, request: ArchiveUserDataMappingRequest, name: &str) -> ProjectLocationDatasetConsentStoreUserDataMappingArchiveCall<'a, S> {
        ProjectLocationDatasetConsentStoreUserDataMappingArchiveCall {
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
    /// Creates a new User data mapping in the parent consent store.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the consent store.
    pub fn locations_datasets_consent_stores_user_data_mappings_create(&self, request: UserDataMapping, parent: &str) -> ProjectLocationDatasetConsentStoreUserDataMappingCreateCall<'a, S> {
        ProjectLocationDatasetConsentStoreUserDataMappingCreateCall {
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
    /// Deletes the specified User data mapping.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the User data mapping to delete.
    pub fn locations_datasets_consent_stores_user_data_mappings_delete(&self, name: &str) -> ProjectLocationDatasetConsentStoreUserDataMappingDeleteCall<'a, S> {
        ProjectLocationDatasetConsentStoreUserDataMappingDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified User data mapping.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the User data mapping to retrieve.
    pub fn locations_datasets_consent_stores_user_data_mappings_get(&self, name: &str) -> ProjectLocationDatasetConsentStoreUserDataMappingGetCall<'a, S> {
        ProjectLocationDatasetConsentStoreUserDataMappingGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the User data mappings in the specified consent store.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the consent store to retrieve User data mappings from.
    pub fn locations_datasets_consent_stores_user_data_mappings_list(&self, parent: &str) -> ProjectLocationDatasetConsentStoreUserDataMappingListCall<'a, S> {
        ProjectLocationDatasetConsentStoreUserDataMappingListCall {
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
    /// Updates the specified User data mapping.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Resource name of the User data mapping, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/userDataMappings/{user_data_mapping_id}`.
    pub fn locations_datasets_consent_stores_user_data_mappings_patch(&self, request: UserDataMapping, name: &str) -> ProjectLocationDatasetConsentStoreUserDataMappingPatchCall<'a, S> {
        ProjectLocationDatasetConsentStoreUserDataMappingPatchCall {
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
    /// Checks if a particular data_id of a User data mapping in the specified consent store is consented for the specified use.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `consentStore` - Required. Name of the consent store where the requested data_id is stored, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}`.
    pub fn locations_datasets_consent_stores_check_data_access(&self, request: CheckDataAccessRequest, consent_store: &str) -> ProjectLocationDatasetConsentStoreCheckDataAccesCall<'a, S> {
        ProjectLocationDatasetConsentStoreCheckDataAccesCall {
            hub: self.hub,
            _request: request,
            _consent_store: consent_store.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new consent store in the parent dataset. Attempting to create a consent store with the same ID as an existing store fails with an ALREADY_EXISTS error.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the dataset this consent store belongs to.
    pub fn locations_datasets_consent_stores_create(&self, request: ConsentStore, parent: &str) -> ProjectLocationDatasetConsentStoreCreateCall<'a, S> {
        ProjectLocationDatasetConsentStoreCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _consent_store_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified consent store and removes all the consent store's data.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the consent store to delete.
    pub fn locations_datasets_consent_stores_delete(&self, name: &str) -> ProjectLocationDatasetConsentStoreDeleteCall<'a, S> {
        ProjectLocationDatasetConsentStoreDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Evaluates the user's Consents for all matching User data mappings. Note: User data mappings are indexed asynchronously, which can cause a slight delay between the time mappings are created or updated and when they are included in EvaluateUserConsents results.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `consentStore` - Required. Name of the consent store to retrieve User data mappings from.
    pub fn locations_datasets_consent_stores_evaluate_user_consents(&self, request: EvaluateUserConsentsRequest, consent_store: &str) -> ProjectLocationDatasetConsentStoreEvaluateUserConsentCall<'a, S> {
        ProjectLocationDatasetConsentStoreEvaluateUserConsentCall {
            hub: self.hub,
            _request: request,
            _consent_store: consent_store.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified consent store.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the consent store to get.
    pub fn locations_datasets_consent_stores_get(&self, name: &str) -> ProjectLocationDatasetConsentStoreGetCall<'a, S> {
        ProjectLocationDatasetConsentStoreGetCall {
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
    pub fn locations_datasets_consent_stores_get_iam_policy(&self, resource: &str) -> ProjectLocationDatasetConsentStoreGetIamPolicyCall<'a, S> {
        ProjectLocationDatasetConsentStoreGetIamPolicyCall {
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
    /// Lists the consent stores in the specified dataset.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the dataset.
    pub fn locations_datasets_consent_stores_list(&self, parent: &str) -> ProjectLocationDatasetConsentStoreListCall<'a, S> {
        ProjectLocationDatasetConsentStoreListCall {
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
    /// Updates the specified consent store.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Resource name of the consent store, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}`. Cannot be changed after creation.
    pub fn locations_datasets_consent_stores_patch(&self, request: ConsentStore, name: &str) -> ProjectLocationDatasetConsentStorePatchCall<'a, S> {
        ProjectLocationDatasetConsentStorePatchCall {
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
    /// Queries all data_ids that are consented for a specified use in the given consent store and writes them to a specified destination. The returned Operation includes a progress counter for the number of User data mappings processed. If the request is successful, a detailed response is returned of type QueryAccessibleDataResponse, contained in the response field when the operation finishes. The metadata field type is OperationMetadata. Errors are logged to Cloud Logging (see [Viewing error logs in Cloud Logging](https://cloud.google.com/healthcare/docs/how-tos/logging)). For example, the following sample log entry shows a `failed to evaluate consent policy` error that occurred during a QueryAccessibleData call to consent store `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}`. ```json jsonPayload: { @type: "type.googleapis.com/google.cloud.healthcare.logging.QueryAccessibleDataLogEntry" error: { code: 9 message: "failed to evaluate consent policy" } resourceName: "projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consents/{consent_id}" } logName: "projects/{project_id}/logs/healthcare.googleapis.com%2Fquery_accessible_data" operation: { id: "projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/operations/{operation_id}" producer: "healthcare.googleapis.com/QueryAccessibleData" } receiveTimestamp: "TIMESTAMP" resource: { labels: { consent_store_id: "{consent_store_id}" dataset_id: "{dataset_id}" location: "{location_id}" project_id: "{project_id}" } type: "healthcare_consent_store" } severity: "ERROR" timestamp: "TIMESTAMP" ```
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `consentStore` - Required. Name of the consent store to retrieve User data mappings from.
    pub fn locations_datasets_consent_stores_query_accessible_data(&self, request: QueryAccessibleDataRequest, consent_store: &str) -> ProjectLocationDatasetConsentStoreQueryAccessibleDataCall<'a, S> {
        ProjectLocationDatasetConsentStoreQueryAccessibleDataCall {
            hub: self.hub,
            _request: request,
            _consent_store: consent_store.to_string(),
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
    pub fn locations_datasets_consent_stores_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationDatasetConsentStoreSetIamPolicyCall<'a, S> {
        ProjectLocationDatasetConsentStoreSetIamPolicyCall {
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
    pub fn locations_datasets_consent_stores_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationDatasetConsentStoreTestIamPermissionCall<'a, S> {
        ProjectLocationDatasetConsentStoreTestIamPermissionCall {
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
    /// RetrieveFrames returns instances associated with the given study, series, SOP Instance UID and frame numbers. See [RetrieveTransaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4}. For details on the implementation of RetrieveFrames, see [DICOM frames](https://cloud.google.com/healthcare/docs/dicom#dicom_frames) in the Cloud Healthcare API conformance statement. For samples that show how to call RetrieveFrames, see [Retrieving DICOM data](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#retrieving_dicom_data).
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the DICOM store that is being accessed. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`.
    /// * `dicomWebPath` - The path of the RetrieveFrames DICOMweb request. For example, `studies/{study_uid}/series/{series_uid}/instances/{instance_uid}/frames/{frame_list}`.
    pub fn locations_datasets_dicom_stores_studies_series_instances_frames_retrieve_frames(&self, parent: &str, dicom_web_path: &str) -> ProjectLocationDatasetDicomStoreStudySeriesInstanceFrameRetrieveFrameCall<'a, S> {
        ProjectLocationDatasetDicomStoreStudySeriesInstanceFrameRetrieveFrameCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _dicom_web_path: dicom_web_path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// RetrieveRenderedFrames returns instances associated with the given study, series, SOP Instance UID and frame numbers in an acceptable Rendered Media Type. See [RetrieveTransaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4). For details on the implementation of RetrieveRenderedFrames, see [Rendered resources](https://cloud.google.com/healthcare/docs/dicom#rendered_resources) in the Cloud Healthcare API conformance statement. For samples that show how to call RetrieveRenderedFrames, see [Retrieving consumer image formats](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#retrieving_consumer_image_formats).
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the DICOM store that is being accessed. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`.
    /// * `dicomWebPath` - The path of the RetrieveRenderedFrames DICOMweb request. For example, `studies/{study_uid}/series/{series_uid}/instances/{instance_uid}/frames/{frame_list}/rendered`.
    pub fn locations_datasets_dicom_stores_studies_series_instances_frames_retrieve_rendered(&self, parent: &str, dicom_web_path: &str) -> ProjectLocationDatasetDicomStoreStudySeriesInstanceFrameRetrieveRenderedCall<'a, S> {
        ProjectLocationDatasetDicomStoreStudySeriesInstanceFrameRetrieveRenderedCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _dicom_web_path: dicom_web_path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// DeleteInstance deletes an instance associated with the given study, series, and SOP Instance UID. Delete requests are equivalent to the GET requests specified in the Retrieve transaction. Study and series search results can take a few seconds to be updated after an instance is deleted using DeleteInstance. For samples that show how to call DeleteInstance, see [Deleting a study, series, or instance](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#deleting_a_study_series_or_instance).
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the DICOM store that is being accessed. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`.
    /// * `dicomWebPath` - The path of the DeleteInstance request. For example, `studies/{study_uid}/series/{series_uid}/instances/{instance_uid}`.
    pub fn locations_datasets_dicom_stores_studies_series_instances_delete(&self, parent: &str, dicom_web_path: &str) -> ProjectLocationDatasetDicomStoreStudySeriesInstanceDeleteCall<'a, S> {
        ProjectLocationDatasetDicomStoreStudySeriesInstanceDeleteCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _dicom_web_path: dicom_web_path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// RetrieveInstance returns instance associated with the given study, series, and SOP Instance UID. See [RetrieveTransaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4). For details on the implementation of RetrieveInstance, see [DICOM study/series/instances](https://cloud.google.com/healthcare/docs/dicom#dicom_studyseriesinstances) and [DICOM instances](https://cloud.google.com/healthcare/docs/dicom#dicom_instances) in the Cloud Healthcare API conformance statement. For samples that show how to call RetrieveInstance, see [Retrieving an instance](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#retrieving_an_instance).
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the DICOM store that is being accessed. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`.
    /// * `dicomWebPath` - The path of the RetrieveInstance DICOMweb request. For example, `studies/{study_uid}/series/{series_uid}/instances/{instance_uid}`.
    pub fn locations_datasets_dicom_stores_studies_series_instances_retrieve_instance(&self, parent: &str, dicom_web_path: &str) -> ProjectLocationDatasetDicomStoreStudySeriesInstanceRetrieveInstanceCall<'a, S> {
        ProjectLocationDatasetDicomStoreStudySeriesInstanceRetrieveInstanceCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _dicom_web_path: dicom_web_path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// RetrieveInstanceMetadata returns instance associated with the given study, series, and SOP Instance UID presented as metadata with the bulk data removed. See [RetrieveTransaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4). For details on the implementation of RetrieveInstanceMetadata, see [Metadata resources](https://cloud.google.com/healthcare/docs/dicom#metadata_resources) in the Cloud Healthcare API conformance statement. For samples that show how to call RetrieveInstanceMetadata, see [Retrieving metadata](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#retrieving_metadata).
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the DICOM store that is being accessed. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`.
    /// * `dicomWebPath` - The path of the RetrieveInstanceMetadata DICOMweb request. For example, `studies/{study_uid}/series/{series_uid}/instances/{instance_uid}/metadata`.
    pub fn locations_datasets_dicom_stores_studies_series_instances_retrieve_metadata(&self, parent: &str, dicom_web_path: &str) -> ProjectLocationDatasetDicomStoreStudySeriesInstanceRetrieveMetadataCall<'a, S> {
        ProjectLocationDatasetDicomStoreStudySeriesInstanceRetrieveMetadataCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _dicom_web_path: dicom_web_path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// RetrieveRenderedInstance returns instance associated with the given study, series, and SOP Instance UID in an acceptable Rendered Media Type. See [RetrieveTransaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4). For details on the implementation of RetrieveRenderedInstance, see [Rendered resources](https://cloud.google.com/healthcare/docs/dicom#rendered_resources) in the Cloud Healthcare API conformance statement. For samples that show how to call RetrieveRenderedInstance, see [Retrieving consumer image formats](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#retrieving_consumer_image_formats).
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the DICOM store that is being accessed. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`.
    /// * `dicomWebPath` - The path of the RetrieveRenderedInstance DICOMweb request. For example, `studies/{study_uid}/series/{series_uid}/instances/{instance_uid}/rendered`.
    pub fn locations_datasets_dicom_stores_studies_series_instances_retrieve_rendered(&self, parent: &str, dicom_web_path: &str) -> ProjectLocationDatasetDicomStoreStudySeriesInstanceRetrieveRenderedCall<'a, S> {
        ProjectLocationDatasetDicomStoreStudySeriesInstanceRetrieveRenderedCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _dicom_web_path: dicom_web_path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// DeleteSeries deletes all instances within the given study and series. Delete requests are equivalent to the GET requests specified in the Retrieve transaction. The method returns an Operation which will be marked successful when the deletion is complete. Warning: Instances cannot be inserted into a series that is being deleted by an operation until the operation completes. For samples that show how to call DeleteSeries, see [Deleting a study, series, or instance](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#deleting_a_study_series_or_instance).
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the DICOM store that is being accessed. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`.
    /// * `dicomWebPath` - The path of the DeleteSeries request. For example, `studies/{study_uid}/series/{series_uid}`.
    pub fn locations_datasets_dicom_stores_studies_series_delete(&self, parent: &str, dicom_web_path: &str) -> ProjectLocationDatasetDicomStoreStudySeriesDeleteCall<'a, S> {
        ProjectLocationDatasetDicomStoreStudySeriesDeleteCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _dicom_web_path: dicom_web_path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// RetrieveSeriesMetadata returns instance associated with the given study and series, presented as metadata with the bulk data removed. See [RetrieveTransaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4). For details on the implementation of RetrieveSeriesMetadata, see [Metadata resources](https://cloud.google.com/healthcare/docs/dicom#metadata_resources) in the Cloud Healthcare API conformance statement. For samples that show how to call RetrieveSeriesMetadata, see [Retrieving metadata](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#retrieving_metadata).
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the DICOM store that is being accessed. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`.
    /// * `dicomWebPath` - The path of the RetrieveSeriesMetadata DICOMweb request. For example, `studies/{study_uid}/series/{series_uid}/metadata`.
    pub fn locations_datasets_dicom_stores_studies_series_retrieve_metadata(&self, parent: &str, dicom_web_path: &str) -> ProjectLocationDatasetDicomStoreStudySeriesRetrieveMetadataCall<'a, S> {
        ProjectLocationDatasetDicomStoreStudySeriesRetrieveMetadataCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _dicom_web_path: dicom_web_path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// RetrieveSeries returns all instances within the given study and series. See [RetrieveTransaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4). For details on the implementation of RetrieveSeries, see [DICOM study/series/instances](https://cloud.google.com/healthcare/docs/dicom#dicom_studyseriesinstances) in the Cloud Healthcare API conformance statement. For samples that show how to call RetrieveSeries, see [Retrieving DICOM data](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#retrieving_dicom_data).
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the DICOM store that is being accessed. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`.
    /// * `dicomWebPath` - The path of the RetrieveSeries DICOMweb request. For example, `studies/{study_uid}/series/{series_uid}`.
    pub fn locations_datasets_dicom_stores_studies_series_retrieve_series(&self, parent: &str, dicom_web_path: &str) -> ProjectLocationDatasetDicomStoreStudySeriesRetrieveSeryCall<'a, S> {
        ProjectLocationDatasetDicomStoreStudySeriesRetrieveSeryCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _dicom_web_path: dicom_web_path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// SearchForInstances returns a list of matching instances. See [Search Transaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.6). For details on the implementation of SearchForInstances, see [Search transaction](https://cloud.google.com/healthcare/docs/dicom#search_transaction) in the Cloud Healthcare API conformance statement. For samples that show how to call SearchForInstances, see [Searching for studies, series, instances, and frames](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#searching_for_studies_series_instances_and_frames).
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the DICOM store that is being accessed. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`.
    /// * `dicomWebPath` - The path of the SearchForInstancesRequest DICOMweb request. For example, `instances`, `series/{series_uid}/instances`, or `studies/{study_uid}/instances`.
    pub fn locations_datasets_dicom_stores_studies_series_search_for_instances(&self, parent: &str, dicom_web_path: &str) -> ProjectLocationDatasetDicomStoreStudySeriesSearchForInstanceCall<'a, S> {
        ProjectLocationDatasetDicomStoreStudySeriesSearchForInstanceCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _dicom_web_path: dicom_web_path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// DeleteStudy deletes all instances within the given study. Delete requests are equivalent to the GET requests specified in the Retrieve transaction. The method returns an Operation which will be marked successful when the deletion is complete. Warning: Instances cannot be inserted into a study that is being deleted by an operation until the operation completes. For samples that show how to call DeleteStudy, see [Deleting a study, series, or instance](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#deleting_a_study_series_or_instance).
    /// 
    /// # Arguments
    ///
    /// * `parent` - No description provided.
    /// * `dicomWebPath` - The path of the DeleteStudy request. For example, `studies/{study_uid}`.
    pub fn locations_datasets_dicom_stores_studies_delete(&self, parent: &str, dicom_web_path: &str) -> ProjectLocationDatasetDicomStoreStudyDeleteCall<'a, S> {
        ProjectLocationDatasetDicomStoreStudyDeleteCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _dicom_web_path: dicom_web_path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// RetrieveStudyMetadata returns instance associated with the given study presented as metadata with the bulk data removed. See [RetrieveTransaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4). For details on the implementation of RetrieveStudyMetadata, see [Metadata resources](https://cloud.google.com/healthcare/docs/dicom#metadata_resources) in the Cloud Healthcare API conformance statement. For samples that show how to call RetrieveStudyMetadata, see [Retrieving metadata](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#retrieving_metadata).
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the DICOM store that is being accessed. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`.
    /// * `dicomWebPath` - The path of the RetrieveStudyMetadata DICOMweb request. For example, `studies/{study_uid}/metadata`.
    pub fn locations_datasets_dicom_stores_studies_retrieve_metadata(&self, parent: &str, dicom_web_path: &str) -> ProjectLocationDatasetDicomStoreStudyRetrieveMetadataCall<'a, S> {
        ProjectLocationDatasetDicomStoreStudyRetrieveMetadataCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _dicom_web_path: dicom_web_path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// RetrieveStudy returns all instances within the given study. See [RetrieveTransaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4). For details on the implementation of RetrieveStudy, see [DICOM study/series/instances](https://cloud.google.com/healthcare/docs/dicom#dicom_studyseriesinstances) in the Cloud Healthcare API conformance statement. For samples that show how to call RetrieveStudy, see [Retrieving DICOM data](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#retrieving_dicom_data).
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the DICOM store that is being accessed. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`.
    /// * `dicomWebPath` - The path of the RetrieveStudy DICOMweb request. For example, `studies/{study_uid}`.
    pub fn locations_datasets_dicom_stores_studies_retrieve_study(&self, parent: &str, dicom_web_path: &str) -> ProjectLocationDatasetDicomStoreStudyRetrieveStudyCall<'a, S> {
        ProjectLocationDatasetDicomStoreStudyRetrieveStudyCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _dicom_web_path: dicom_web_path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// SearchForInstances returns a list of matching instances. See [Search Transaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.6). For details on the implementation of SearchForInstances, see [Search transaction](https://cloud.google.com/healthcare/docs/dicom#search_transaction) in the Cloud Healthcare API conformance statement. For samples that show how to call SearchForInstances, see [Searching for studies, series, instances, and frames](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#searching_for_studies_series_instances_and_frames).
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the DICOM store that is being accessed. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`.
    /// * `dicomWebPath` - The path of the SearchForInstancesRequest DICOMweb request. For example, `instances`, `series/{series_uid}/instances`, or `studies/{study_uid}/instances`.
    pub fn locations_datasets_dicom_stores_studies_search_for_instances(&self, parent: &str, dicom_web_path: &str) -> ProjectLocationDatasetDicomStoreStudySearchForInstanceCall<'a, S> {
        ProjectLocationDatasetDicomStoreStudySearchForInstanceCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _dicom_web_path: dicom_web_path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// SearchForSeries returns a list of matching series. See [Search Transaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.6). For details on the implementation of SearchForSeries, see [Search transaction](https://cloud.google.com/healthcare/docs/dicom#search_transaction) in the Cloud Healthcare API conformance statement. For samples that show how to call SearchForSeries, see [Searching for studies, series, instances, and frames](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#searching_for_studies_series_instances_and_frames).
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the DICOM store that is being accessed. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`.
    /// * `dicomWebPath` - The path of the SearchForSeries DICOMweb request. For example, `series` or `studies/{study_uid}/series`.
    pub fn locations_datasets_dicom_stores_studies_search_for_series(&self, parent: &str, dicom_web_path: &str) -> ProjectLocationDatasetDicomStoreStudySearchForSeryCall<'a, S> {
        ProjectLocationDatasetDicomStoreStudySearchForSeryCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _dicom_web_path: dicom_web_path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// StoreInstances stores DICOM instances associated with study instance unique identifiers (SUID). See [Store Transaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.5). For details on the implementation of StoreInstances, see [Store transaction](https://cloud.google.com/healthcare/docs/dicom#store_transaction) in the Cloud Healthcare API conformance statement. For samples that show how to call StoreInstances, see [Storing DICOM data](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#storing_dicom_data).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The name of the DICOM store that is being accessed. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`.
    /// * `dicomWebPath` - The path of the StoreInstances DICOMweb request. For example, `studies/[{study_uid}]`. Note that the `study_uid` is optional.
    pub fn locations_datasets_dicom_stores_studies_store_instances(&self, request: HttpBody, parent: &str, dicom_web_path: &str) -> ProjectLocationDatasetDicomStoreStudyStoreInstanceCall<'a, S> {
        ProjectLocationDatasetDicomStoreStudyStoreInstanceCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _dicom_web_path: dicom_web_path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new DICOM store within the parent dataset.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The name of the dataset this DICOM store belongs to.
    pub fn locations_datasets_dicom_stores_create(&self, request: DicomStore, parent: &str) -> ProjectLocationDatasetDicomStoreCreateCall<'a, S> {
        ProjectLocationDatasetDicomStoreCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _dicom_store_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// De-identifies data from the source store and writes it to the destination store. The metadata field type is OperationMetadata. If the request is successful, the response field type is DeidentifyDicomStoreSummary. If errors occur, error is set. The LRO result may still be successful if de-identification fails for some DICOM instances. The output DICOM store will not contain these failed resources. Failed resource totals are tracked in Operation.metadata. Error details are also logged to Cloud Logging (see [Viewing error logs in Cloud Logging](https://cloud.google.com/healthcare/docs/how-tos/logging)).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `sourceStore` - Source DICOM store resource name. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`.
    pub fn locations_datasets_dicom_stores_deidentify(&self, request: DeidentifyDicomStoreRequest, source_store: &str) -> ProjectLocationDatasetDicomStoreDeidentifyCall<'a, S> {
        ProjectLocationDatasetDicomStoreDeidentifyCall {
            hub: self.hub,
            _request: request,
            _source_store: source_store.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified DICOM store and removes all images that are contained within it.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the DICOM store to delete.
    pub fn locations_datasets_dicom_stores_delete(&self, name: &str) -> ProjectLocationDatasetDicomStoreDeleteCall<'a, S> {
        ProjectLocationDatasetDicomStoreDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Exports data to the specified destination by copying it from the DICOM store. Errors are also logged to Cloud Logging. For more information, see [Viewing error logs in Cloud Logging](https://cloud.google.com/healthcare/docs/how-tos/logging). The metadata field type is OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The DICOM store resource name from which to export the data. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`.
    pub fn locations_datasets_dicom_stores_export(&self, request: ExportDicomDataRequest, name: &str) -> ProjectLocationDatasetDicomStoreExportCall<'a, S> {
        ProjectLocationDatasetDicomStoreExportCall {
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
    /// Gets the specified DICOM store.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the DICOM store to get.
    pub fn locations_datasets_dicom_stores_get(&self, name: &str) -> ProjectLocationDatasetDicomStoreGetCall<'a, S> {
        ProjectLocationDatasetDicomStoreGetCall {
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
    pub fn locations_datasets_dicom_stores_get_iam_policy(&self, resource: &str) -> ProjectLocationDatasetDicomStoreGetIamPolicyCall<'a, S> {
        ProjectLocationDatasetDicomStoreGetIamPolicyCall {
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
    /// Imports data into the DICOM store by copying it from the specified source. Errors are logged to Cloud Logging. For more information, see [Viewing error logs in Cloud Logging](https://cloud.google.com/healthcare/docs/how-tos/logging). The metadata field type is OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the DICOM store resource into which the data is imported. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`.
    pub fn locations_datasets_dicom_stores_import(&self, request: ImportDicomDataRequest, name: &str) -> ProjectLocationDatasetDicomStoreImportCall<'a, S> {
        ProjectLocationDatasetDicomStoreImportCall {
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
    /// Lists the DICOM stores in the given dataset.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Name of the dataset.
    pub fn locations_datasets_dicom_stores_list(&self, parent: &str) -> ProjectLocationDatasetDicomStoreListCall<'a, S> {
        ProjectLocationDatasetDicomStoreListCall {
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
    /// Updates the specified DICOM store.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Resource name of the DICOM store, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`.
    pub fn locations_datasets_dicom_stores_patch(&self, request: DicomStore, name: &str) -> ProjectLocationDatasetDicomStorePatchCall<'a, S> {
        ProjectLocationDatasetDicomStorePatchCall {
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
    /// SearchForInstances returns a list of matching instances. See [Search Transaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.6). For details on the implementation of SearchForInstances, see [Search transaction](https://cloud.google.com/healthcare/docs/dicom#search_transaction) in the Cloud Healthcare API conformance statement. For samples that show how to call SearchForInstances, see [Searching for studies, series, instances, and frames](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#searching_for_studies_series_instances_and_frames).
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the DICOM store that is being accessed. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`.
    /// * `dicomWebPath` - The path of the SearchForInstancesRequest DICOMweb request. For example, `instances`, `series/{series_uid}/instances`, or `studies/{study_uid}/instances`.
    pub fn locations_datasets_dicom_stores_search_for_instances(&self, parent: &str, dicom_web_path: &str) -> ProjectLocationDatasetDicomStoreSearchForInstanceCall<'a, S> {
        ProjectLocationDatasetDicomStoreSearchForInstanceCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _dicom_web_path: dicom_web_path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// SearchForSeries returns a list of matching series. See [Search Transaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.6). For details on the implementation of SearchForSeries, see [Search transaction](https://cloud.google.com/healthcare/docs/dicom#search_transaction) in the Cloud Healthcare API conformance statement. For samples that show how to call SearchForSeries, see [Searching for studies, series, instances, and frames](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#searching_for_studies_series_instances_and_frames).
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the DICOM store that is being accessed. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`.
    /// * `dicomWebPath` - The path of the SearchForSeries DICOMweb request. For example, `series` or `studies/{study_uid}/series`.
    pub fn locations_datasets_dicom_stores_search_for_series(&self, parent: &str, dicom_web_path: &str) -> ProjectLocationDatasetDicomStoreSearchForSeryCall<'a, S> {
        ProjectLocationDatasetDicomStoreSearchForSeryCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _dicom_web_path: dicom_web_path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// SearchForStudies returns a list of matching studies. See [Search Transaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.6). For details on the implementation of SearchForStudies, see [Search transaction](https://cloud.google.com/healthcare/docs/dicom#search_transaction) in the Cloud Healthcare API conformance statement. For samples that show how to call SearchForStudies, see [Searching for studies, series, instances, and frames](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#searching_for_studies_series_instances_and_frames).
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the DICOM store that is being accessed. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`.
    /// * `dicomWebPath` - The path of the SearchForStudies DICOMweb request. For example, `studies`.
    pub fn locations_datasets_dicom_stores_search_for_studies(&self, parent: &str, dicom_web_path: &str) -> ProjectLocationDatasetDicomStoreSearchForStudyCall<'a, S> {
        ProjectLocationDatasetDicomStoreSearchForStudyCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _dicom_web_path: dicom_web_path.to_string(),
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
    pub fn locations_datasets_dicom_stores_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationDatasetDicomStoreSetIamPolicyCall<'a, S> {
        ProjectLocationDatasetDicomStoreSetIamPolicyCall {
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
    /// StoreInstances stores DICOM instances associated with study instance unique identifiers (SUID). See [Store Transaction] (http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.5). For details on the implementation of StoreInstances, see [Store transaction](https://cloud.google.com/healthcare/docs/dicom#store_transaction) in the Cloud Healthcare API conformance statement. For samples that show how to call StoreInstances, see [Storing DICOM data](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#storing_dicom_data).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The name of the DICOM store that is being accessed. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`.
    /// * `dicomWebPath` - The path of the StoreInstances DICOMweb request. For example, `studies/[{study_uid}]`. Note that the `study_uid` is optional.
    pub fn locations_datasets_dicom_stores_store_instances(&self, request: HttpBody, parent: &str, dicom_web_path: &str) -> ProjectLocationDatasetDicomStoreStoreInstanceCall<'a, S> {
        ProjectLocationDatasetDicomStoreStoreInstanceCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _dicom_web_path: dicom_web_path.to_string(),
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
    pub fn locations_datasets_dicom_stores_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationDatasetDicomStoreTestIamPermissionCall<'a, S> {
        ProjectLocationDatasetDicomStoreTestIamPermissionCall {
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
    /// Retrieves a Patient resource and resources related to that patient. Implements the FHIR extended operation Patient-everything ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/patient-operations.html#everything), [STU3](http://hl7.org/implement/standards/fhir/STU3/patient-operations.html#everything), [R4](http://hl7.org/implement/standards/fhir/R4/patient-operations.html#everything)). On success, the response body contains a JSON-encoded representation of a `Bundle` resource of type `searchset`, containing the results of the operation. Errors generated by the FHIR store contain a JSON-encoded `OperationOutcome` resource describing the reason for the error. If the request cannot be mapped to a valid API method on a FHIR store, a generic GCP error might be returned instead. The resources in scope for the response are: * The patient resource itself. * All the resources directly referenced by the patient resource. * Resources directly referencing the patient resource that meet the inclusion criteria. The inclusion criteria are based on the membership rules in the patient compartment definition ([DSTU2](http://hl7.org/fhir/DSTU2/compartment-patient.html), [STU3](http://www.hl7.org/fhir/stu3/compartmentdefinition-patient.html), [R4](http://hl7.org/fhir/R4/compartmentdefinition-patient.html)), which details the eligible resource types and referencing search parameters. For samples that show how to call `Patient-everything`, see [Getting all patient compartment resources](https://cloud.google.com/healthcare/docs/how-tos/fhir-resources#getting_all_patient_compartment_resources).
    /// 
    /// # Arguments
    ///
    /// * `name` - Name of the `Patient` resource for which the information is required.
    pub fn locations_datasets_fhir_stores_fhir__patient_everything(&self, name: &str) -> ProjectLocationDatasetFhirStoreFhirPatientEverythingCall<'a, S> {
        ProjectLocationDatasetFhirStoreFhirPatientEverythingCall {
            hub: self.hub,
            _name: name.to_string(),
            _start: Default::default(),
            _end: Default::default(),
            __type: Default::default(),
            __since: Default::default(),
            __page_token: Default::default(),
            __count: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes all the historical versions of a resource (excluding the current version) from the FHIR store. To remove all versions of a resource, first delete the current version and then call this method. This is not a FHIR standard operation. For samples that show how to call `Resource-purge`, see [Deleting historical versions of a FHIR resource](https://cloud.google.com/healthcare/docs/how-tos/fhir-resources#deleting_historical_versions_of_a_fhir_resource).
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the resource to purge.
    pub fn locations_datasets_fhir_stores_fhir__resource_purge(&self, name: &str) -> ProjectLocationDatasetFhirStoreFhirResourcePurgeCall<'a, S> {
        ProjectLocationDatasetFhirStoreFhirResourcePurgeCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Validates an input FHIR resource's conformance to its profiles and the profiles configured on the FHIR store. Implements the FHIR extended operation $validate ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/resource-operations.html#validate), [STU3](http://hl7.org/implement/standards/fhir/STU3/resource-operations.html#validate), or [R4](http://hl7.org/implement/standards/fhir/R4/resource-operation-validate.html)). The request body must contain a JSON-encoded FHIR resource, and the request headers must contain `Content-Type: application/fhir+json`. The `Parameters` input syntax is not supported. The `profile` query parameter can be used to request that the resource only be validated against a specific profile. If a profile with the given URL cannot be found in the FHIR store then an error is returned. Errors generated by validation contain a JSON-encoded `OperationOutcome` resource describing the reason for the error. If the request cannot be mapped to a valid API method on a FHIR store, a generic GCP error might be returned instead.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The name of the FHIR store that holds the profiles being used for validation.
    /// * `type` - The FHIR resource type of the resource being validated. For a complete list, see the FHIR Resource Index ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/resourcelist.html), [STU3](http://hl7.org/implement/standards/fhir/STU3/resourcelist.html), or [R4](http://hl7.org/implement/standards/fhir/R4/resourcelist.html)). Must match the resource type in the provided content.
    pub fn locations_datasets_fhir_stores_fhir__resource_validate(&self, request: HttpBody, parent: &str, type_: &str) -> ProjectLocationDatasetFhirStoreFhirResourceValidateCall<'a, S> {
        ProjectLocationDatasetFhirStoreFhirResourceValidateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _type_: type_.to_string(),
            _profile: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the FHIR capability statement ([STU3](http://hl7.org/implement/standards/fhir/STU3/capabilitystatement.html), [R4](http://hl7.org/implement/standards/fhir/R4/capabilitystatement.html)), or the [conformance statement](http://hl7.org/implement/standards/fhir/DSTU2/conformance.html) in the DSTU2 case for the store, which contains a description of functionality supported by the server. Implements the FHIR standard capabilities interaction ([STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#capabilities), [R4](http://hl7.org/implement/standards/fhir/R4/http.html#capabilities)), or the [conformance interaction](http://hl7.org/implement/standards/fhir/DSTU2/http.html#conformance) in the DSTU2 case. On success, the response body contains a JSON-encoded representation of a `CapabilityStatement` resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Name of the FHIR store to retrieve the capabilities for.
    pub fn locations_datasets_fhir_stores_fhir_capabilities(&self, name: &str) -> ProjectLocationDatasetFhirStoreFhirCapabilityCall<'a, S> {
        ProjectLocationDatasetFhirStoreFhirCapabilityCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a FHIR resource. Implements the FHIR standard create interaction ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#create), [STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#create), [R4](http://hl7.org/implement/standards/fhir/R4/http.html#create)), which creates a new resource with a server-assigned resource ID. The request body must contain a JSON-encoded FHIR resource, and the request headers must contain `Content-Type: application/fhir+json`. On success, the response body contains a JSON-encoded representation of the resource as it was created on the server, including the server-assigned resource ID and version ID. Errors generated by the FHIR store contain a JSON-encoded `OperationOutcome` resource describing the reason for the error. If the request cannot be mapped to a valid API method on a FHIR store, a generic GCP error might be returned instead. For samples that show how to call `create`, see [Creating a FHIR resource](https://cloud.google.com/healthcare/docs/how-tos/fhir-resources#creating_a_fhir_resource).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The name of the FHIR store this resource belongs to.
    /// * `type` - The FHIR resource type to create, such as Patient or Observation. For a complete list, see the FHIR Resource Index ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/resourcelist.html), [STU3](http://hl7.org/implement/standards/fhir/STU3/resourcelist.html), [R4](http://hl7.org/implement/standards/fhir/R4/resourcelist.html)). Must match the resource type in the provided content.
    pub fn locations_datasets_fhir_stores_fhir_create(&self, request: HttpBody, parent: &str, type_: &str) -> ProjectLocationDatasetFhirStoreFhirCreateCall<'a, S> {
        ProjectLocationDatasetFhirStoreFhirCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _type_: type_.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a FHIR resource. Implements the FHIR standard delete interaction ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#delete), [STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#delete), [R4](http://hl7.org/implement/standards/fhir/R4/http.html#delete)). Note: Unless resource versioning is disabled by setting the disable_resource_versioning flag on the FHIR store, the deleted resources will be moved to a history repository that can still be retrieved through vread and related methods, unless they are removed by the purge method. For samples that show how to call `delete`, see [Deleting a FHIR resource](https://cloud.google.com/healthcare/docs/how-tos/fhir-resources#deleting_a_fhir_resource).
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the resource to delete.
    pub fn locations_datasets_fhir_stores_fhir_delete(&self, name: &str) -> ProjectLocationDatasetFhirStoreFhirDeleteCall<'a, S> {
        ProjectLocationDatasetFhirStoreFhirDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Executes all the requests in the given Bundle. Implements the FHIR standard batch/transaction interaction ([DSTU2](https://hl7.org/implement/standards/fhir/DSTU2/http.html#transaction), [STU3](https://hl7.org/implement/standards/fhir/STU3/http.html#transaction), [R4](https://hl7.org/implement/standards/fhir/R4/http.html#transaction)). Supports all interactions within a bundle, except search. This method accepts Bundles of type `batch` and `transaction`, processing them according to the batch processing rules ([DSTU2](https://hl7.org/implement/standards/fhir/DSTU2/http.html#2.1.0.16.1), [STU3](https://hl7.org/implement/standards/fhir/STU3/http.html#2.21.0.17.1), [R4](https://hl7.org/implement/standards/fhir/R4/http.html#brules)) and transaction processing rules ([DSTU2](https://hl7.org/implement/standards/fhir/DSTU2/http.html#2.1.0.16.2), [STU3](https://hl7.org/implement/standards/fhir/STU3/http.html#2.21.0.17.2), [R4](https://hl7.org/implement/standards/fhir/R4/http.html#trules)). The request body must contain a JSON-encoded FHIR `Bundle` resource, and the request headers must contain `Content-Type: application/fhir+json`. For a batch bundle or a successful transaction, the response body contains a JSON-encoded representation of a `Bundle` resource of type `batch-response` or `transaction-response` containing one entry for each entry in the request, with the outcome of processing the entry. In the case of an error for a transaction bundle, the response body contains a JSON-encoded `OperationOutcome` resource describing the reason for the error. If the request cannot be mapped to a valid API method on a FHIR store, a generic GCP error might be returned instead. This method checks permissions for each request in the bundle. The `executeBundle` permission is required to call this method, but you must also grant sufficient permissions to execute the individual requests in the bundle. For example, if the bundle contains a request to create a FHIR resource, the caller must also have been granted the `healthcare.fhirResources.create` permission. You can use audit logs to view the permissions for `executeBundle` and each request in the bundle. For more information, see [Viewing Cloud Audit logs](https://cloud.google.com/healthcare-api/docs/how-tos/audit-logging). For samples that show how to call `executeBundle`, see [Managing FHIR resources using FHIR bundles](https://cloud.google.com/healthcare/docs/how-tos/fhir-bundles).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Name of the FHIR store in which this bundle will be executed.
    pub fn locations_datasets_fhir_stores_fhir_execute_bundle(&self, request: HttpBody, parent: &str) -> ProjectLocationDatasetFhirStoreFhirExecuteBundleCall<'a, S> {
        ProjectLocationDatasetFhirStoreFhirExecuteBundleCall {
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
    /// Lists all the versions of a resource (including the current version and deleted versions) from the FHIR store. Implements the per-resource form of the FHIR standard history interaction ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#history), [STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#history), [R4](http://hl7.org/implement/standards/fhir/R4/http.html#history)). On success, the response body contains a JSON-encoded representation of a `Bundle` resource of type `history`, containing the version history sorted from most recent to oldest versions. Errors generated by the FHIR store contain a JSON-encoded `OperationOutcome` resource describing the reason for the error. If the request cannot be mapped to a valid API method on a FHIR store, a generic GCP error might be returned instead. For samples that show how to call `history`, see [Listing FHIR resource versions](https://cloud.google.com/healthcare/docs/how-tos/fhir-resources#listing_fhir_resource_versions).
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the resource to retrieve.
    pub fn locations_datasets_fhir_stores_fhir_history(&self, name: &str) -> ProjectLocationDatasetFhirStoreFhirHistoryCall<'a, S> {
        ProjectLocationDatasetFhirStoreFhirHistoryCall {
            hub: self.hub,
            _name: name.to_string(),
            __since: Default::default(),
            __page_token: Default::default(),
            __count: Default::default(),
            __at: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates part of an existing resource by applying the operations specified in a [JSON Patch](http://jsonpatch.com/) document. Implements the FHIR standard patch interaction ([STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#patch), [R4](http://hl7.org/implement/standards/fhir/R4/http.html#patch)). DSTU2 doesn't define a patch method, but the server supports it in the same way it supports STU3. The request body must contain a JSON Patch document, and the request headers must contain `Content-Type: application/json-patch+json`. On success, the response body contains a JSON-encoded representation of the updated resource, including the server-assigned version ID. Errors generated by the FHIR store contain a JSON-encoded `OperationOutcome` resource describing the reason for the error. If the request cannot be mapped to a valid API method on a FHIR store, a generic GCP error might be returned instead. For samples that show how to call `patch`, see [Patching a FHIR resource](https://cloud.google.com/healthcare/docs/how-tos/fhir-resources#patching_a_fhir_resource).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the resource to update.
    pub fn locations_datasets_fhir_stores_fhir_patch(&self, request: HttpBody, name: &str) -> ProjectLocationDatasetFhirStoreFhirPatchCall<'a, S> {
        ProjectLocationDatasetFhirStoreFhirPatchCall {
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
    /// Gets the contents of a FHIR resource. Implements the FHIR standard read interaction ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#read), [STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#read), [R4](http://hl7.org/implement/standards/fhir/R4/http.html#read)). Also supports the FHIR standard conditional read interaction ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#cread), [STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#cread), [R4](http://hl7.org/implement/standards/fhir/R4/http.html#cread)) specified by supplying an `If-Modified-Since` header with a date/time value or an `If-None-Match` header with an ETag value. On success, the response body contains a JSON-encoded representation of the resource. Errors generated by the FHIR store contain a JSON-encoded `OperationOutcome` resource describing the reason for the error. If the request cannot be mapped to a valid API method on a FHIR store, a generic GCP error might be returned instead. For samples that show how to call `read`, see [Getting a FHIR resource](https://cloud.google.com/healthcare/docs/how-tos/fhir-resources#getting_a_fhir_resource).
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the resource to retrieve.
    pub fn locations_datasets_fhir_stores_fhir_read(&self, name: &str) -> ProjectLocationDatasetFhirStoreFhirReadCall<'a, S> {
        ProjectLocationDatasetFhirStoreFhirReadCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Searches for resources in the given FHIR store according to criteria specified as query parameters. Implements the FHIR standard search interaction ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#search), [STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#search), [R4](http://hl7.org/implement/standards/fhir/R4/http.html#search)) using the search semantics described in the FHIR Search specification ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/search.html), [STU3](http://hl7.org/implement/standards/fhir/STU3/search.html), [R4](http://hl7.org/implement/standards/fhir/R4/search.html)). Supports four methods of search defined by the specification: * `GET [base]?[parameters]` to search across all resources. * `GET [base]/[type]?[parameters]` to search resources of a specified type. * `POST [base]/_search?[parameters]` as an alternate form having the same semantics as the `GET` method across all resources. * `POST [base]/[type]/_search?[parameters]` as an alternate form having the same semantics as the `GET` method for the specified type. The `GET` and `POST` methods do not support compartment searches. The `POST` method does not support `application/x-www-form-urlencoded` search parameters. On success, the response body contains a JSON-encoded representation of a `Bundle` resource of type `searchset`, containing the results of the search. Errors generated by the FHIR store contain a JSON-encoded `OperationOutcome` resource describing the reason for the error. If the request cannot be mapped to a valid API method on a FHIR store, a generic GCP error might be returned instead. The server's capability statement, retrieved through capabilities, indicates what search parameters are supported on each FHIR resource. A list of all search parameters defined by the specification can be found in the FHIR Search Parameter Registry ([STU3](http://hl7.org/implement/standards/fhir/STU3/searchparameter-registry.html), [R4](http://hl7.org/implement/standards/fhir/R4/searchparameter-registry.html)). FHIR search parameters for DSTU2 can be found on each resource's definition page. Supported search modifiers: `:missing`, `:exact`, `:contains`, `:text`, `:in`, `:not-in`, `:above`, `:below`, `:[type]`, `:not`, and `recurse` (DSTU2 and STU3) or `:iterate` (R4). Supported search result parameters: `_sort`, `_count`, `_include`, `_revinclude`, `_summary=text`, `_summary=data`, and `_elements`. The maximum number of search results returned defaults to 100, which can be overridden by the `_count` parameter up to a maximum limit of 1000. If there are additional results, the returned `Bundle` contains a link of `relation` "next", which has a `_page_token` parameter for an opaque pagination token that can be used to retrieve the next page. Resources with a total size larger than 5MB or a field count larger than 50,000 might not be fully searchable as the server might trim its generated search index in those cases. Note: FHIR resources are indexed asynchronously, so there might be a slight delay between the time a resource is created or changes and when the change is reflected in search results. For samples and detailed information, see [Searching for FHIR resources](https://cloud.google.com/healthcare/docs/how-tos/fhir-search) and [Advanced FHIR search features](https://cloud.google.com/healthcare/docs/how-tos/fhir-advanced-search).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Name of the FHIR store to retrieve resources from.
    pub fn locations_datasets_fhir_stores_fhir_search(&self, request: SearchResourcesRequest, parent: &str) -> ProjectLocationDatasetFhirStoreFhirSearchCall<'a, S> {
        ProjectLocationDatasetFhirStoreFhirSearchCall {
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
    /// Searches for resources in the given FHIR store according to criteria specified as query parameters. Implements the FHIR standard search interaction ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#search), [STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#search), [R4](http://hl7.org/implement/standards/fhir/R4/http.html#search)) using the search semantics described in the FHIR Search specification ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/search.html), [STU3](http://hl7.org/implement/standards/fhir/STU3/search.html), [R4](http://hl7.org/implement/standards/fhir/R4/search.html)). Supports four methods of search defined by the specification: * `GET [base]?[parameters]` to search across all resources. * `GET [base]/[type]?[parameters]` to search resources of a specified type. * `POST [base]/_search?[parameters]` as an alternate form having the same semantics as the `GET` method across all resources. * `POST [base]/[type]/_search?[parameters]` as an alternate form having the same semantics as the `GET` method for the specified type. The `GET` and `POST` methods do not support compartment searches. The `POST` method does not support `application/x-www-form-urlencoded` search parameters. On success, the response body contains a JSON-encoded representation of a `Bundle` resource of type `searchset`, containing the results of the search. Errors generated by the FHIR store contain a JSON-encoded `OperationOutcome` resource describing the reason for the error. If the request cannot be mapped to a valid API method on a FHIR store, a generic GCP error might be returned instead. The server's capability statement, retrieved through capabilities, indicates what search parameters are supported on each FHIR resource. A list of all search parameters defined by the specification can be found in the FHIR Search Parameter Registry ([STU3](http://hl7.org/implement/standards/fhir/STU3/searchparameter-registry.html), [R4](http://hl7.org/implement/standards/fhir/R4/searchparameter-registry.html)). FHIR search parameters for DSTU2 can be found on each resource's definition page. Supported search modifiers: `:missing`, `:exact`, `:contains`, `:text`, `:in`, `:not-in`, `:above`, `:below`, `:[type]`, `:not`, and `recurse` (DSTU2 and STU3) or `:iterate` (R4). Supported search result parameters: `_sort`, `_count`, `_include`, `_revinclude`, `_summary=text`, `_summary=data`, and `_elements`. The maximum number of search results returned defaults to 100, which can be overridden by the `_count` parameter up to a maximum limit of 1000. If there are additional results, the returned `Bundle` contains a link of `relation` "next", which has a `_page_token` parameter for an opaque pagination token that can be used to retrieve the next page. Resources with a total size larger than 5MB or a field count larger than 50,000 might not be fully searchable as the server might trim its generated search index in those cases. Note: FHIR resources are indexed asynchronously, so there might be a slight delay between the time a resource is created or changes and when the change is reflected in search results. For samples and detailed information, see [Searching for FHIR resources](https://cloud.google.com/healthcare/docs/how-tos/fhir-search) and [Advanced FHIR search features](https://cloud.google.com/healthcare/docs/how-tos/fhir-advanced-search).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Name of the FHIR store to retrieve resources from.
    /// * `resourceType` - The FHIR resource type to search, such as Patient or Observation. For a complete list, see the FHIR Resource Index ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/resourcelist.html), [STU3](http://hl7.org/implement/standards/fhir/STU3/resourcelist.html), [R4](http://hl7.org/implement/standards/fhir/R4/resourcelist.html)).
    pub fn locations_datasets_fhir_stores_fhir_search_type(&self, request: SearchResourcesRequest, parent: &str, resource_type: &str) -> ProjectLocationDatasetFhirStoreFhirSearchTypeCall<'a, S> {
        ProjectLocationDatasetFhirStoreFhirSearchTypeCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _resource_type: resource_type.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the entire contents of a resource. Implements the FHIR standard update interaction ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#update), [STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#update), [R4](http://hl7.org/implement/standards/fhir/R4/http.html#update)). If the specified resource does not exist and the FHIR store has enable_update_create set, creates the resource with the client-specified ID. It is strongly advised not to include or encode any sensitive data such as patient identifiers in client-specified resource IDs. Those IDs are part of the FHIR resource path recorded in Cloud Audit Logs and Pub/Sub notifications. Those IDs can also be contained in reference fields within other resources. The request body must contain a JSON-encoded FHIR resource, and the request headers must contain `Content-Type: application/fhir+json`. The resource must contain an `id` element having an identical value to the ID in the REST path of the request. On success, the response body contains a JSON-encoded representation of the updated resource, including the server-assigned version ID. Errors generated by the FHIR store contain a JSON-encoded `OperationOutcome` resource describing the reason for the error. If the request cannot be mapped to a valid API method on a FHIR store, a generic GCP error might be returned instead. For samples that show how to call `update`, see [Updating a FHIR resource](https://cloud.google.com/healthcare/docs/how-tos/fhir-resources#updating_a_fhir_resource).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the resource to update.
    pub fn locations_datasets_fhir_stores_fhir_update(&self, request: HttpBody, name: &str) -> ProjectLocationDatasetFhirStoreFhirUpdateCall<'a, S> {
        ProjectLocationDatasetFhirStoreFhirUpdateCall {
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
    /// Gets the contents of a version (current or historical) of a FHIR resource by version ID. Implements the FHIR standard vread interaction ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#vread), [STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#vread), [R4](http://hl7.org/implement/standards/fhir/R4/http.html#vread)). On success, the response body contains a JSON-encoded representation of the resource. Errors generated by the FHIR store contain a JSON-encoded `OperationOutcome` resource describing the reason for the error. If the request cannot be mapped to a valid API method on a FHIR store, a generic GCP error might be returned instead. For samples that show how to call `vread`, see [Retrieving a FHIR resource version](https://cloud.google.com/healthcare/docs/how-tos/fhir-resources#retrieving_a_fhir_resource_version).
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the resource version to retrieve.
    pub fn locations_datasets_fhir_stores_fhir_vread(&self, name: &str) -> ProjectLocationDatasetFhirStoreFhirVreadCall<'a, S> {
        ProjectLocationDatasetFhirStoreFhirVreadCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new FHIR store within the parent dataset.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The name of the dataset this FHIR store belongs to.
    pub fn locations_datasets_fhir_stores_create(&self, request: FhirStore, parent: &str) -> ProjectLocationDatasetFhirStoreCreateCall<'a, S> {
        ProjectLocationDatasetFhirStoreCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _fhir_store_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// De-identifies data from the source store and writes it to the destination store. The metadata field type is OperationMetadata. If the request is successful, the response field type is DeidentifyFhirStoreSummary. If errors occur, error is set. Error details are also logged to Cloud Logging (see [Viewing error logs in Cloud Logging](https://cloud.google.com/healthcare/docs/how-tos/logging)).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `sourceStore` - Source FHIR store resource name. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/fhirStores/{fhir_store_id}`.
    pub fn locations_datasets_fhir_stores_deidentify(&self, request: DeidentifyFhirStoreRequest, source_store: &str) -> ProjectLocationDatasetFhirStoreDeidentifyCall<'a, S> {
        ProjectLocationDatasetFhirStoreDeidentifyCall {
            hub: self.hub,
            _request: request,
            _source_store: source_store.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified FHIR store and removes all resources within it.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the FHIR store to delete.
    pub fn locations_datasets_fhir_stores_delete(&self, name: &str) -> ProjectLocationDatasetFhirStoreDeleteCall<'a, S> {
        ProjectLocationDatasetFhirStoreDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Export resources from the FHIR store to the specified destination. This method returns an Operation that can be used to track the status of the export by calling GetOperation. Immediate fatal errors appear in the error field, errors are also logged to Cloud Logging (see [Viewing error logs in Cloud Logging](https://cloud.google.com/healthcare/docs/how-tos/logging)). Otherwise, when the operation finishes, a detailed response of type ExportResourcesResponse is returned in the response field. The metadata field type for this operation is OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the FHIR store to export resource from, in the format of `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/fhirStores/{fhir_store_id}`.
    pub fn locations_datasets_fhir_stores_export(&self, request: ExportResourcesRequest, name: &str) -> ProjectLocationDatasetFhirStoreExportCall<'a, S> {
        ProjectLocationDatasetFhirStoreExportCall {
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
    /// Gets the configuration of the specified FHIR store.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the FHIR store to get.
    pub fn locations_datasets_fhir_stores_get(&self, name: &str) -> ProjectLocationDatasetFhirStoreGetCall<'a, S> {
        ProjectLocationDatasetFhirStoreGetCall {
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
    pub fn locations_datasets_fhir_stores_get_iam_policy(&self, resource: &str) -> ProjectLocationDatasetFhirStoreGetIamPolicyCall<'a, S> {
        ProjectLocationDatasetFhirStoreGetIamPolicyCall {
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
    /// Imports resources to the FHIR store by loading data from the specified sources. This method is optimized to load large quantities of data using import semantics that ignore some FHIR store configuration options and are not suitable for all use cases. It is primarily intended to load data into an empty FHIR store that is not being used by other clients. In cases where this method is not appropriate, consider using ExecuteBundle to load data. Every resource in the input must contain a client-supplied ID. Each resource is stored using the supplied ID regardless of the enable_update_create setting on the FHIR store. It is strongly advised not to include or encode any sensitive data such as patient identifiers in client-specified resource IDs. Those IDs are part of the FHIR resource path recorded in Cloud Audit Logs and Cloud Pub/Sub notifications. Those IDs can also be contained in reference fields within other resources. The import process does not enforce referential integrity, regardless of the disable_referential_integrity setting on the FHIR store. This allows the import of resources with arbitrary interdependencies without considering grouping or ordering, but if the input data contains invalid references or if some resources fail to be imported, the FHIR store might be left in a state that violates referential integrity. The import process does not trigger Pub/Sub notification or BigQuery streaming update, regardless of how those are configured on the FHIR store. If a resource with the specified ID already exists, the most recent version of the resource is overwritten without creating a new historical version, regardless of the disable_resource_versioning setting on the FHIR store. If transient failures occur during the import, it's possible that successfully imported resources will be overwritten more than once. The import operation is idempotent unless the input data contains multiple valid resources with the same ID but different contents. In that case, after the import completes, the store contains exactly one resource with that ID but there is no ordering guarantee on which version of the contents it will have. The operation result counters do not count duplicate IDs as an error and count one success for each resource in the input, which might result in a success count larger than the number of resources in the FHIR store. This often occurs when importing data organized in bundles produced by Patient-everything where each bundle contains its own copy of a resource such as Practitioner that might be referred to by many patients. If some resources fail to import, for example due to parsing errors, successfully imported resources are not rolled back. The location and format of the input data is specified by the parameters in ImportResourcesRequest. Note that if no format is specified, this method assumes the `BUNDLE` format. When using the `BUNDLE` format this method ignores the `Bundle.type` field, except that `history` bundles are rejected, and does not apply any of the bundle processing semantics for batch or transaction bundles. Unlike in ExecuteBundle, transaction bundles are not executed as a single transaction and bundle-internal references are not rewritten. The bundle is treated as a collection of resources to be written as provided in `Bundle.entry.resource`, ignoring `Bundle.entry.request`. As an example, this allows the import of `searchset` bundles produced by a FHIR search or Patient-everything operation. This method returns an Operation that can be used to track the status of the import by calling GetOperation. Immediate fatal errors appear in the error field, errors are also logged to Cloud Logging (see [Viewing error logs in Cloud Logging](https://cloud.google.com/healthcare/docs/how-tos/logging)). Otherwise, when the operation finishes, a detailed response of type ImportResourcesResponse is returned in the response field. The metadata field type for this operation is OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the FHIR store to import FHIR resources to, in the format of `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/fhirStores/{fhir_store_id}`.
    pub fn locations_datasets_fhir_stores_import(&self, request: ImportResourcesRequest, name: &str) -> ProjectLocationDatasetFhirStoreImportCall<'a, S> {
        ProjectLocationDatasetFhirStoreImportCall {
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
    /// Lists the FHIR stores in the given dataset.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Name of the dataset.
    pub fn locations_datasets_fhir_stores_list(&self, parent: &str) -> ProjectLocationDatasetFhirStoreListCall<'a, S> {
        ProjectLocationDatasetFhirStoreListCall {
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
    /// Updates the configuration of the specified FHIR store.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name of the FHIR store, of the form `projects/{project_id}/datasets/{dataset_id}/fhirStores/{fhir_store_id}`.
    pub fn locations_datasets_fhir_stores_patch(&self, request: FhirStore, name: &str) -> ProjectLocationDatasetFhirStorePatchCall<'a, S> {
        ProjectLocationDatasetFhirStorePatchCall {
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
    pub fn locations_datasets_fhir_stores_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationDatasetFhirStoreSetIamPolicyCall<'a, S> {
        ProjectLocationDatasetFhirStoreSetIamPolicyCall {
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
    pub fn locations_datasets_fhir_stores_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationDatasetFhirStoreTestIamPermissionCall<'a, S> {
        ProjectLocationDatasetFhirStoreTestIamPermissionCall {
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
    /// Parses and stores an HL7v2 message. This method triggers an asynchronous notification to any Pub/Sub topic configured in Hl7V2Store.Hl7V2NotificationConfig, if the filtering matches the message. If an MLLP adapter is configured to listen to a Pub/Sub topic, the adapter transmits the message when a notification is received.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The name of the dataset this message belongs to.
    pub fn locations_datasets_hl7_v2_stores_messages_create(&self, request: CreateMessageRequest, parent: &str) -> ProjectLocationDatasetHl7V2StoreMessageCreateCall<'a, S> {
        ProjectLocationDatasetHl7V2StoreMessageCreateCall {
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
    /// Deletes an HL7v2 message.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the HL7v2 message to delete.
    pub fn locations_datasets_hl7_v2_stores_messages_delete(&self, name: &str) -> ProjectLocationDatasetHl7V2StoreMessageDeleteCall<'a, S> {
        ProjectLocationDatasetHl7V2StoreMessageDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an HL7v2 message.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the HL7v2 message to retrieve.
    pub fn locations_datasets_hl7_v2_stores_messages_get(&self, name: &str) -> ProjectLocationDatasetHl7V2StoreMessageGetCall<'a, S> {
        ProjectLocationDatasetHl7V2StoreMessageGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Parses and stores an HL7v2 message. This method triggers an asynchronous notification to any Pub/Sub topic configured in Hl7V2Store.Hl7V2NotificationConfig, if the filtering matches the message. If an MLLP adapter is configured to listen to a Pub/Sub topic, the adapter transmits the message when a notification is received. If the method is successful, it generates a response containing an HL7v2 acknowledgment (`ACK`) message. If the method encounters an error, it returns a negative acknowledgment (`NACK`) message. This behavior is suitable for replying to HL7v2 interface systems that expect these acknowledgments.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The name of the HL7v2 store this message belongs to.
    pub fn locations_datasets_hl7_v2_stores_messages_ingest(&self, request: IngestMessageRequest, parent: &str) -> ProjectLocationDatasetHl7V2StoreMessageIngestCall<'a, S> {
        ProjectLocationDatasetHl7V2StoreMessageIngestCall {
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
    /// Lists all the messages in the given HL7v2 store with support for filtering. Note: HL7v2 messages are indexed asynchronously, so there might be a slight delay between the time a message is created and when it can be found through a filter.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Name of the HL7v2 store to retrieve messages from.
    pub fn locations_datasets_hl7_v2_stores_messages_list(&self, parent: &str) -> ProjectLocationDatasetHl7V2StoreMessageListCall<'a, S> {
        ProjectLocationDatasetHl7V2StoreMessageListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _view: Default::default(),
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
    /// Update the message. The contents of the message in Message.data and data extracted from the contents such as Message.create_time cannot be altered. Only the Message.labels field is allowed to be updated. The labels in the request are merged with the existing set of labels. Existing labels with the same keys are updated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Resource name of the Message, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/hl7V2Stores/{hl7_v2_store_id}/messages/{message_id}`. Assigned by the server.
    pub fn locations_datasets_hl7_v2_stores_messages_patch(&self, request: Message, name: &str) -> ProjectLocationDatasetHl7V2StoreMessagePatchCall<'a, S> {
        ProjectLocationDatasetHl7V2StoreMessagePatchCall {
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
    /// Creates a new HL7v2 store within the parent dataset.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The name of the dataset this HL7v2 store belongs to.
    pub fn locations_datasets_hl7_v2_stores_create(&self, request: Hl7V2Store, parent: &str) -> ProjectLocationDatasetHl7V2StoreCreateCall<'a, S> {
        ProjectLocationDatasetHl7V2StoreCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _hl7_v2_store_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified HL7v2 store and removes all messages that it contains.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the HL7v2 store to delete.
    pub fn locations_datasets_hl7_v2_stores_delete(&self, name: &str) -> ProjectLocationDatasetHl7V2StoreDeleteCall<'a, S> {
        ProjectLocationDatasetHl7V2StoreDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Exports the messages to a destination. To filter messages to be exported, define a filter using the start and end time, relative to the message generation time (MSH.7). This API returns an Operation that can be used to track the status of the job by calling GetOperation. Immediate fatal errors appear in the error field. Otherwise, when the operation finishes, a detailed response of type ExportMessagesResponse is returned in the response field. The metadata field type for this operation is OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the source HL7v2 store, in the format `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/hl7v2Stores/{hl7v2_store_id}`
    pub fn locations_datasets_hl7_v2_stores_export(&self, request: ExportMessagesRequest, name: &str) -> ProjectLocationDatasetHl7V2StoreExportCall<'a, S> {
        ProjectLocationDatasetHl7V2StoreExportCall {
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
    /// Gets the specified HL7v2 store.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the HL7v2 store to get.
    pub fn locations_datasets_hl7_v2_stores_get(&self, name: &str) -> ProjectLocationDatasetHl7V2StoreGetCall<'a, S> {
        ProjectLocationDatasetHl7V2StoreGetCall {
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
    pub fn locations_datasets_hl7_v2_stores_get_iam_policy(&self, resource: &str) -> ProjectLocationDatasetHl7V2StoreGetIamPolicyCall<'a, S> {
        ProjectLocationDatasetHl7V2StoreGetIamPolicyCall {
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
    /// Import messages to the HL7v2 store by loading data from the specified sources. This method is optimized to load large quantities of data using import semantics that ignore some HL7v2 store configuration options and are not suitable for all use cases. It is primarily intended to load data into an empty HL7v2 store that is not being used by other clients. An existing message will be overwritten if a duplicate message is imported. A duplicate message is a message with the same raw bytes as a message that already exists in this HL7v2 store. When a message is overwritten, its labels will also be overwritten. The import operation is idempotent unless the input data contains multiple valid messages with the same raw bytes but different labels. In that case, after the import completes, the store contains exactly one message with those raw bytes but there is no ordering guarantee on which version of the labels it has. The operation result counters do not count duplicated raw bytes as an error and count one success for each message in the input, which might result in a success count larger than the number of messages in the HL7v2 store. If some messages fail to import, for example due to parsing errors, successfully imported messages are not rolled back. This method returns an Operation that can be used to track the status of the import by calling GetOperation. Immediate fatal errors appear in the error field, errors are also logged to Cloud Logging (see [Viewing error logs in Cloud Logging](https://cloud.google.com/healthcare/docs/how-tos/logging)). Otherwise, when the operation finishes, a response of type ImportMessagesResponse is returned in the response field. The metadata field type for this operation is OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the target HL7v2 store, in the format `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/hl7v2Stores/{hl7v2_store_id}`
    pub fn locations_datasets_hl7_v2_stores_import(&self, request: ImportMessagesRequest, name: &str) -> ProjectLocationDatasetHl7V2StoreImportCall<'a, S> {
        ProjectLocationDatasetHl7V2StoreImportCall {
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
    /// Lists the HL7v2 stores in the given dataset.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Name of the dataset.
    pub fn locations_datasets_hl7_v2_stores_list(&self, parent: &str) -> ProjectLocationDatasetHl7V2StoreListCall<'a, S> {
        ProjectLocationDatasetHl7V2StoreListCall {
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
    /// Updates the HL7v2 store.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Resource name of the HL7v2 store, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/hl7V2Stores/{hl7v2_store_id}`.
    pub fn locations_datasets_hl7_v2_stores_patch(&self, request: Hl7V2Store, name: &str) -> ProjectLocationDatasetHl7V2StorePatchCall<'a, S> {
        ProjectLocationDatasetHl7V2StorePatchCall {
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
    pub fn locations_datasets_hl7_v2_stores_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationDatasetHl7V2StoreSetIamPolicyCall<'a, S> {
        ProjectLocationDatasetHl7V2StoreSetIamPolicyCall {
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
    pub fn locations_datasets_hl7_v2_stores_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationDatasetHl7V2StoreTestIamPermissionCall<'a, S> {
        ProjectLocationDatasetHl7V2StoreTestIamPermissionCall {
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
    pub fn locations_datasets_operations_cancel(&self, request: CancelOperationRequest, name: &str) -> ProjectLocationDatasetOperationCancelCall<'a, S> {
        ProjectLocationDatasetOperationCancelCall {
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
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn locations_datasets_operations_get(&self, name: &str) -> ProjectLocationDatasetOperationGetCall<'a, S> {
        ProjectLocationDatasetOperationGetCall {
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
    pub fn locations_datasets_operations_list(&self, name: &str) -> ProjectLocationDatasetOperationListCall<'a, S> {
        ProjectLocationDatasetOperationListCall {
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
    /// Creates a new health dataset. Results are returned through the Operation interface which returns either an `Operation.response` which contains a Dataset or `Operation.error`. The metadata field type is OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The name of the project where the server creates the dataset. For example, `projects/{project_id}/locations/{location_id}`.
    pub fn locations_datasets_create(&self, request: Dataset, parent: &str) -> ProjectLocationDatasetCreateCall<'a, S> {
        ProjectLocationDatasetCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _dataset_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new dataset containing de-identified data from the source dataset. The metadata field type is OperationMetadata. If the request is successful, the response field type is DeidentifySummary. If errors occur, error is set. The LRO result may still be successful if de-identification fails for some DICOM instances. The new de-identified dataset will not contain these failed resources. Failed resource totals are tracked in Operation.metadata. Error details are also logged to Cloud Logging. For more information, see [Viewing error logs in Cloud Logging](https://cloud.google.com/healthcare/docs/how-tos/logging).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `sourceDataset` - Source dataset resource name. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}`.
    pub fn locations_datasets_deidentify(&self, request: DeidentifyDatasetRequest, source_dataset: &str) -> ProjectLocationDatasetDeidentifyCall<'a, S> {
        ProjectLocationDatasetDeidentifyCall {
            hub: self.hub,
            _request: request,
            _source_dataset: source_dataset.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified health dataset and all data contained in the dataset. Deleting a dataset does not affect the sources from which the dataset was imported (if any).
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the dataset to delete. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}`.
    pub fn locations_datasets_delete(&self, name: &str) -> ProjectLocationDatasetDeleteCall<'a, S> {
        ProjectLocationDatasetDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets any metadata associated with a dataset.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the dataset to read. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}`.
    pub fn locations_datasets_get(&self, name: &str) -> ProjectLocationDatasetGetCall<'a, S> {
        ProjectLocationDatasetGetCall {
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
    pub fn locations_datasets_get_iam_policy(&self, resource: &str) -> ProjectLocationDatasetGetIamPolicyCall<'a, S> {
        ProjectLocationDatasetGetIamPolicyCall {
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
    /// Lists the health datasets in the current project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the project whose datasets should be listed. For example, `projects/{project_id}/locations/{location_id}`.
    pub fn locations_datasets_list(&self, parent: &str) -> ProjectLocationDatasetListCall<'a, S> {
        ProjectLocationDatasetListCall {
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
    /// Updates dataset metadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Resource name of the dataset, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}`.
    pub fn locations_datasets_patch(&self, request: Dataset, name: &str) -> ProjectLocationDatasetPatchCall<'a, S> {
        ProjectLocationDatasetPatchCall {
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
    pub fn locations_datasets_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationDatasetSetIamPolicyCall<'a, S> {
        ProjectLocationDatasetSetIamPolicyCall {
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
    pub fn locations_datasets_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationDatasetTestIamPermissionCall<'a, S> {
        ProjectLocationDatasetTestIamPermissionCall {
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
    /// Analyze heathcare entity in a document. Its response includes the recognized entity mentions and the relationships between them. AnalyzeEntities uses context aware models to detect entities.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `nlpService` - The resource name of the service of the form: "projects/{project_id}/locations/{location_id}/services/nlp".
    pub fn locations_services_nlp_analyze_entities(&self, request: AnalyzeEntitiesRequest, nlp_service: &str) -> ProjectLocationServiceNlpAnalyzeEntityCall<'a, S> {
        ProjectLocationServiceNlpAnalyzeEntityCall {
            hub: self.hub,
            _request: request,
            _nlp_service: nlp_service.to_string(),
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



