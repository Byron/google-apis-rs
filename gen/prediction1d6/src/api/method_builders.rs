use super::*;
/// A builder providing access to all methods supported on *hostedmodel* resources.
/// It is not used directly, but through the [`Prediction`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_prediction1d6 as prediction1d6;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use prediction1d6::{Prediction, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Prediction::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `predict(...)`
/// // to build up your call.
/// let rb = hub.hostedmodels();
/// # }
/// ```
pub struct HostedmodelMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Prediction<S>,
}

impl<'a, S> client::MethodsBuilder for HostedmodelMethods<'a, S> {}

impl<'a, S> HostedmodelMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Submit input and request an output against a hosted model.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project associated with the model.
    /// * `hostedModelName` - The name of a hosted model.
    pub fn predict(&self, request: Input, project: &str, hosted_model_name: &str) -> HostedmodelPredictCall<'a, S> {
        HostedmodelPredictCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _hosted_model_name: hosted_model_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *trainedmodel* resources.
/// It is not used directly, but through the [`Prediction`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_prediction1d6 as prediction1d6;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use prediction1d6::{Prediction, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Prediction::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `analyze(...)`, `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `predict(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.trainedmodels();
/// # }
/// ```
pub struct TrainedmodelMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Prediction<S>,
}

impl<'a, S> client::MethodsBuilder for TrainedmodelMethods<'a, S> {}

impl<'a, S> TrainedmodelMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get analysis of the model and the data the model was trained on.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project associated with the model.
    /// * `id` - The unique name for the predictive model.
    pub fn analyze(&self, project: &str, id: &str) -> TrainedmodelAnalyzeCall<'a, S> {
        TrainedmodelAnalyzeCall {
            hub: self.hub,
            _project: project.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete a trained model.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project associated with the model.
    /// * `id` - The unique name for the predictive model.
    pub fn delete(&self, project: &str, id: &str) -> TrainedmodelDeleteCall<'a, S> {
        TrainedmodelDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Check training status of your model.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project associated with the model.
    /// * `id` - The unique name for the predictive model.
    pub fn get(&self, project: &str, id: &str) -> TrainedmodelGetCall<'a, S> {
        TrainedmodelGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Train a Prediction API model.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project associated with the model.
    pub fn insert(&self, request: Insert, project: &str) -> TrainedmodelInsertCall<'a, S> {
        TrainedmodelInsertCall {
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
    /// List available models.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project associated with the model.
    pub fn list(&self, project: &str) -> TrainedmodelListCall<'a, S> {
        TrainedmodelListCall {
            hub: self.hub,
            _project: project.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Submit model id and request a prediction.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project associated with the model.
    /// * `id` - The unique name for the predictive model.
    pub fn predict(&self, request: Input, project: &str, id: &str) -> TrainedmodelPredictCall<'a, S> {
        TrainedmodelPredictCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Add new data to a trained model.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project associated with the model.
    /// * `id` - The unique name for the predictive model.
    pub fn update(&self, request: Update, project: &str, id: &str) -> TrainedmodelUpdateCall<'a, S> {
        TrainedmodelUpdateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



