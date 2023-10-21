use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`RecaptchaEnterprise`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_recaptchaenterprise1 as recaptchaenterprise1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use recaptchaenterprise1::{RecaptchaEnterprise, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = RecaptchaEnterprise::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `assessments_annotate(...)`, `assessments_create(...)`, `keys_create(...)`, `keys_delete(...)`, `keys_get(...)`, `keys_get_metrics(...)`, `keys_list(...)`, `keys_migrate(...)`, `keys_patch(...)`, `keys_retrieve_legacy_secret_key(...)`, `relatedaccountgroupmemberships_search(...)`, `relatedaccountgroups_list(...)` and `relatedaccountgroups_memberships_list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a RecaptchaEnterprise<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Annotates a previously created Assessment to provide additional information on whether the event turned out to be authentic or fraudulent.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the Assessment, in the format "projects/{project}/assessments/{assessment}".
    pub fn assessments_annotate(&self, request: GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequest, name: &str) -> ProjectAssessmentAnnotateCall<'a, S> {
        ProjectAssessmentAnnotateCall {
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
    /// Creates an Assessment of the likelihood an event is legitimate.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the project in which the assessment will be created, in the format "projects/{project}".
    pub fn assessments_create(&self, request: GoogleCloudRecaptchaenterpriseV1Assessment, parent: &str) -> ProjectAssessmentCreateCall<'a, S> {
        ProjectAssessmentCreateCall {
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
    /// Creates a new reCAPTCHA Enterprise key.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the project in which the key will be created, in the format "projects/{project}".
    pub fn keys_create(&self, request: GoogleCloudRecaptchaenterpriseV1Key, parent: &str) -> ProjectKeyCreateCall<'a, S> {
        ProjectKeyCreateCall {
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
    /// Deletes the specified key.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the key to be deleted, in the format "projects/{project}/keys/{key}".
    pub fn keys_delete(&self, name: &str) -> ProjectKeyDeleteCall<'a, S> {
        ProjectKeyDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified key.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the requested key, in the format "projects/{project}/keys/{key}".
    pub fn keys_get(&self, name: &str) -> ProjectKeyGetCall<'a, S> {
        ProjectKeyGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get some aggregated metrics for a Key. This data can be used to build dashboards.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the requested metrics, in the format "projects/{project}/keys/{key}/metrics".
    pub fn keys_get_metrics(&self, name: &str) -> ProjectKeyGetMetricCall<'a, S> {
        ProjectKeyGetMetricCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all keys that belong to a project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the project that contains the keys that will be listed, in the format "projects/{project}".
    pub fn keys_list(&self, parent: &str) -> ProjectKeyListCall<'a, S> {
        ProjectKeyListCall {
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
    /// Migrates an existing key from reCAPTCHA to reCAPTCHA Enterprise. Once a key is migrated, it can be used from either product. SiteVerify requests are billed as CreateAssessment calls. You must be authenticated as one of the current owners of the reCAPTCHA Site Key, and your user must have the reCAPTCHA Enterprise Admin IAM role in the destination project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the key to be migrated, in the format "projects/{project}/keys/{key}".
    pub fn keys_migrate(&self, request: GoogleCloudRecaptchaenterpriseV1MigrateKeyRequest, name: &str) -> ProjectKeyMigrateCall<'a, S> {
        ProjectKeyMigrateCall {
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
    /// Updates the specified key.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name for the Key in the format "projects/{project}/keys/{key}".
    pub fn keys_patch(&self, request: GoogleCloudRecaptchaenterpriseV1Key, name: &str) -> ProjectKeyPatchCall<'a, S> {
        ProjectKeyPatchCall {
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
    /// Returns the secret key related to the specified public key. You must use the legacy secret key only in a 3rd party integration with legacy reCAPTCHA.
    /// 
    /// # Arguments
    ///
    /// * `key` - Required. The public key name linked to the requested secret key in the format "projects/{project}/keys/{key}".
    pub fn keys_retrieve_legacy_secret_key(&self, key: &str) -> ProjectKeyRetrieveLegacySecretKeyCall<'a, S> {
        ProjectKeyRetrieveLegacySecretKeyCall {
            hub: self.hub,
            _key: key.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Search group memberships related to a given account.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Required. The name of the project to search related account group memberships from. Specify the project name in the following format: "projects/{project}".
    pub fn relatedaccountgroupmemberships_search(&self, request: GoogleCloudRecaptchaenterpriseV1SearchRelatedAccountGroupMembershipsRequest, project: &str) -> ProjectRelatedaccountgroupmembershipSearchCall<'a, S> {
        ProjectRelatedaccountgroupmembershipSearchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get memberships in a group of related accounts.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name for the related account group in the format `projects/{project}/relatedaccountgroups/{relatedaccountgroup}`.
    pub fn relatedaccountgroups_memberships_list(&self, parent: &str) -> ProjectRelatedaccountgroupMembershipListCall<'a, S> {
        ProjectRelatedaccountgroupMembershipListCall {
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
    /// List groups of related accounts.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the project to list related account groups from, in the format "projects/{project}".
    pub fn relatedaccountgroups_list(&self, parent: &str) -> ProjectRelatedaccountgroupListCall<'a, S> {
        ProjectRelatedaccountgroupListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



