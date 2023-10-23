use super::*;
/// A builder providing access to all methods supported on *applicationDetailService* resources.
/// It is not used directly, but through the [`Testing`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_testing1 as testing1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use testing1::{Testing, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Testing::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_apk_details(...)`
/// // to build up your call.
/// let rb = hub.application_detail_service();
/// # }
/// ```
pub struct ApplicationDetailServiceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Testing<S>,
}

impl<'a, S> client::MethodsBuilder for ApplicationDetailServiceMethods<'a, S> {}

impl<'a, S> ApplicationDetailServiceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the details of an Android application APK.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn get_apk_details(&self, request: FileReference) -> ApplicationDetailServiceGetApkDetailCall<'a, S> {
        ApplicationDetailServiceGetApkDetailCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Testing`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_testing1 as testing1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use testing1::{Testing, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Testing::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `test_matrices_cancel(...)`, `test_matrices_create(...)` and `test_matrices_get(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Testing<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancels unfinished test executions in a test matrix. This call returns immediately and cancellation proceeds asynchronously. If the matrix is already final, this operation will have no effect. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the Test Matrix does not exist
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Cloud project that owns the test.
    /// * `testMatrixId` - Test matrix that will be canceled.
    pub fn test_matrices_cancel(&self, project_id: &str, test_matrix_id: &str) -> ProjectTestMatriceCancelCall<'a, S> {
        ProjectTestMatriceCancelCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _test_matrix_id: test_matrix_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates and runs a matrix of tests according to the given specifications. Unsupported environments will be returned in the state UNSUPPORTED. A test matrix is limited to use at most 2000 devices in parallel. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed or if the matrix tries to use too many simultaneous devices.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - The GCE project under which this job will run.
    pub fn test_matrices_create(&self, request: TestMatrix, project_id: &str) -> ProjectTestMatriceCreateCall<'a, S> {
        ProjectTestMatriceCreateCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Checks the status of a test matrix. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the Test Matrix does not exist
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Cloud project that owns the test matrix.
    /// * `testMatrixId` - Unique test matrix id which was assigned by the service.
    pub fn test_matrices_get(&self, project_id: &str, test_matrix_id: &str) -> ProjectTestMatriceGetCall<'a, S> {
        ProjectTestMatriceGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _test_matrix_id: test_matrix_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *testEnvironmentCatalog* resources.
/// It is not used directly, but through the [`Testing`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_testing1 as testing1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use testing1::{Testing, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Testing::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.test_environment_catalog();
/// # }
/// ```
pub struct TestEnvironmentCatalogMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Testing<S>,
}

impl<'a, S> client::MethodsBuilder for TestEnvironmentCatalogMethods<'a, S> {}

impl<'a, S> TestEnvironmentCatalogMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the catalog of supported test environments. May return any of the following canonical error codes: - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the environment type does not exist - INTERNAL - if an internal error occurred
    /// 
    /// # Arguments
    ///
    /// * `environmentType` - Required. The type of environment that should be listed.
    pub fn get(&self, environment_type: &TestEnvironmentCatalogEnvironmentTypeEnum) -> TestEnvironmentCatalogGetCall<'a, S> {
        TestEnvironmentCatalogGetCall {
            hub: self.hub,
            _environment_type: environment_type.clone(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



