use super::*;
/// A builder providing access to all methods supported on *mobileapppanel* resources.
/// It is not used directly, but through the [`ConsumerSurveys`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_consumersurveys2 as consumersurveys2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use consumersurveys2::{ConsumerSurveys, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ConsumerSurveys::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.mobileapppanels();
/// # }
/// ```
pub struct MobileapppanelMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ConsumerSurveys<S>,
}

impl<'a, S> client::MethodsBuilder for MobileapppanelMethods<'a, S> {}

impl<'a, S> MobileapppanelMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a MobileAppPanel that is available to the authenticated user.
    /// 
    /// # Arguments
    ///
    /// * `panelId` - External URL ID for the panel.
    pub fn get(&self, panel_id: &str) -> MobileapppanelGetCall<'a, S> {
        MobileapppanelGetCall {
            hub: self.hub,
            _panel_id: panel_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the MobileAppPanels available to the authenticated user.
    pub fn list(&self) -> MobileapppanelListCall<'a, S> {
        MobileapppanelListCall {
            hub: self.hub,
            _token: Default::default(),
            _start_index: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a MobileAppPanel. Currently the only property that can be updated is the owners property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `panelId` - External URL ID for the panel.
    pub fn update(&self, request: MobileAppPanel, panel_id: &str) -> MobileapppanelUpdateCall<'a, S> {
        MobileapppanelUpdateCall {
            hub: self.hub,
            _request: request,
            _panel_id: panel_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *result* resources.
/// It is not used directly, but through the [`ConsumerSurveys`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_consumersurveys2 as consumersurveys2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use consumersurveys2::{ConsumerSurveys, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ConsumerSurveys::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.results();
/// # }
/// ```
pub struct ResultMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ConsumerSurveys<S>,
}

impl<'a, S> client::MethodsBuilder for ResultMethods<'a, S> {}

impl<'a, S> ResultMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves any survey results that have been produced so far. Results are formatted as an Excel file. You must add "?alt=media" to the URL as an argument to get results.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `surveyUrlId` - External URL ID for the survey.
    pub fn get(&self, request: ResultsGetRequest, survey_url_id: &str) -> ResultGetCall<'a, S> {
        ResultGetCall {
            hub: self.hub,
            _request: request,
            _survey_url_id: survey_url_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *survey* resources.
/// It is not used directly, but through the [`ConsumerSurveys`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_consumersurveys2 as consumersurveys2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use consumersurveys2::{ConsumerSurveys, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ConsumerSurveys::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `start(...)`, `stop(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.surveys();
/// # }
/// ```
pub struct SurveyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ConsumerSurveys<S>,
}

impl<'a, S> client::MethodsBuilder for SurveyMethods<'a, S> {}

impl<'a, S> SurveyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes a survey from view in all user GET requests.
    /// 
    /// # Arguments
    ///
    /// * `surveyUrlId` - External URL ID for the survey.
    pub fn delete(&self, survey_url_id: &str) -> SurveyDeleteCall<'a, S> {
        SurveyDeleteCall {
            hub: self.hub,
            _survey_url_id: survey_url_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves information about the specified survey.
    /// 
    /// # Arguments
    ///
    /// * `surveyUrlId` - External URL ID for the survey.
    pub fn get(&self, survey_url_id: &str) -> SurveyGetCall<'a, S> {
        SurveyGetCall {
            hub: self.hub,
            _survey_url_id: survey_url_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a survey.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: Survey) -> SurveyInsertCall<'a, S> {
        SurveyInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the surveys owned by the authenticated user.
    pub fn list(&self) -> SurveyListCall<'a, S> {
        SurveyListCall {
            hub: self.hub,
            _token: Default::default(),
            _start_index: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Begins running a survey.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - No description provided.
    pub fn start(&self, request: SurveysStartRequest, resource_id: &str) -> SurveyStartCall<'a, S> {
        SurveyStartCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Stops a running survey.
    /// 
    /// # Arguments
    ///
    /// * `resourceId` - No description provided.
    pub fn stop(&self, resource_id: &str) -> SurveyStopCall<'a, S> {
        SurveyStopCall {
            hub: self.hub,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a survey. Currently the only property that can be updated is the owners property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `surveyUrlId` - External URL ID for the survey.
    pub fn update(&self, request: Survey, survey_url_id: &str) -> SurveyUpdateCall<'a, S> {
        SurveyUpdateCall {
            hub: self.hub,
            _request: request,
            _survey_url_id: survey_url_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



