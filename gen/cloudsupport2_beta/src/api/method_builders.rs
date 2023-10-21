use super::*;
/// A builder providing access to all methods supported on *attachment* resources.
/// It is not used directly, but through the [`CloudSupport`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudsupport2_beta as cloudsupport2_beta;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudsupport2_beta::{CloudSupport, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudSupport::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`
/// // to build up your call.
/// let rb = hub.attachments();
/// # }
/// ```
pub struct AttachmentMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudSupport<S>,
}

impl<'a, S> client::MethodsBuilder for AttachmentMethods<'a, S> {}

impl<'a, S> AttachmentMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a file attachment on a case or Cloud resource. The attachment object must have the following fields set: filename.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the case (or case parent) to which the attachment should be attached.
    pub fn create(&self, request: CreateAttachmentRequest, parent: &str) -> AttachmentCreateCall<'a, S> {
        AttachmentCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *caseClassification* resources.
/// It is not used directly, but through the [`CloudSupport`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudsupport2_beta as cloudsupport2_beta;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudsupport2_beta::{CloudSupport, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudSupport::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `search(...)`
/// // to build up your call.
/// let rb = hub.case_classifications();
/// # }
/// ```
pub struct CaseClassificationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudSupport<S>,
}

impl<'a, S> client::MethodsBuilder for CaseClassificationMethods<'a, S> {}

impl<'a, S> CaseClassificationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieve valid classifications to be used when creating a support case. The classications are hierarchical, with each classification containing all levels of the hierarchy, separated by " > ". For example "Technical Issue > Compute > Compute Engine".
    pub fn search(&self) -> CaseClassificationSearchCall<'a, S> {
        CaseClassificationSearchCall {
            hub: self.hub,
            _query: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *case* resources.
/// It is not used directly, but through the [`CloudSupport`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudsupport2_beta as cloudsupport2_beta;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudsupport2_beta::{CloudSupport, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudSupport::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `attachments_list(...)`, `close(...)`, `comments_create(...)`, `comments_list(...)`, `create(...)`, `escalate(...)`, `get(...)`, `list(...)`, `patch(...)` and `search(...)`
/// // to build up your call.
/// let rb = hub.cases();
/// # }
/// ```
pub struct CaseMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudSupport<S>,
}

impl<'a, S> client::MethodsBuilder for CaseMethods<'a, S> {}

impl<'a, S> CaseMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieve all attachments associated with a support case.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of Case object for which attachments should be listed.
    pub fn attachments_list(&self, parent: &str) -> CaseAttachmentListCall<'a, S> {
        CaseAttachmentListCall {
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
    /// Add a new comment to the specified Case. The comment object must have the following fields set: body.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of Case to which this comment should be added.
    pub fn comments_create(&self, request: Comment, parent: &str) -> CaseCommentCreateCall<'a, S> {
        CaseCommentCreateCall {
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
    /// Retrieve all Comments associated with the Case object.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of Case object for which comments should be listed.
    pub fn comments_list(&self, parent: &str) -> CaseCommentListCall<'a, S> {
        CaseCommentListCall {
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
    /// Close the specified case.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The fully qualified name of the case resource to be closed.
    pub fn close(&self, request: CloseCaseRequest, name: &str) -> CaseCloseCall<'a, S> {
        CaseCloseCall {
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
    /// Create a new case and associate it with the given Cloud resource. The case object must have the following fields set: display_name, description, classification, and severity.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the Cloud resource under which the case should be created.
    pub fn create(&self, request: Case, parent: &str) -> CaseCreateCall<'a, S> {
        CaseCreateCall {
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
    /// Escalate a case. Escalating a case will initiate the Cloud Support escalation management process. This operation is only available to certain Customer Care tiers. Go to https://cloud.google.com/support and look for 'Technical support escalations' in the feature list to find out which tiers are able to perform escalations.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The fully qualified name of the Case resource to be escalated.
    pub fn escalate(&self, request: EscalateCaseRequest, name: &str) -> CaseEscalateCall<'a, S> {
        CaseEscalateCall {
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
    /// Retrieve the specified case.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The fully qualified name of a case to be retrieved.
    pub fn get(&self, name: &str) -> CaseGetCall<'a, S> {
        CaseGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieve all cases under the specified parent. Note: Listing cases under an Organization returns only the cases directly parented by that organization. To retrieve all cases under an organization, including cases parented by projects under that organization, use `cases.search`.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The fully qualified name of parent resource to list cases under.
    pub fn list(&self, parent: &str) -> CaseListCall<'a, S> {
        CaseListCall {
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
    /// Update the specified case. Only a subset of fields can be updated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name for the case.
    pub fn patch(&self, request: Case, name: &str) -> CasePatchCall<'a, S> {
        CasePatchCall {
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
    /// Search cases using the specified query.
    pub fn search(&self) -> CaseSearchCall<'a, S> {
        CaseSearchCall {
            hub: self.hub,
            _query: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *media* resources.
/// It is not used directly, but through the [`CloudSupport`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudsupport2_beta as cloudsupport2_beta;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudsupport2_beta::{CloudSupport, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudSupport::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `download(...)` and `upload(...)`
/// // to build up your call.
/// let rb = hub.media();
/// # }
/// ```
pub struct MediaMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudSupport<S>,
}

impl<'a, S> client::MethodsBuilder for MediaMethods<'a, S> {}

impl<'a, S> MediaMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Download a file attachment on a case. Note: HTTP requests must append "?alt=media" to the URL.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the attachment to be downloaded.
    pub fn download(&self, name: &str) -> MediaDownloadCall<'a, S> {
        MediaDownloadCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a file attachment on a case or Cloud resource. The attachment object must have the following fields set: filename.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the case (or case parent) to which the attachment should be attached.
    pub fn upload(&self, request: CreateAttachmentRequest, parent: &str) -> MediaUploadCall<'a, S> {
        MediaUploadCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



