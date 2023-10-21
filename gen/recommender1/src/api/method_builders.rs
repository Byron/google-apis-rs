use super::*;
/// A builder providing access to all methods supported on *billingAccount* resources.
/// It is not used directly, but through the [`Recommender`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_recommender1 as recommender1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use recommender1::{Recommender, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Recommender::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_insight_types_get_config(...)`, `locations_insight_types_insights_get(...)`, `locations_insight_types_insights_list(...)`, `locations_insight_types_insights_mark_accepted(...)`, `locations_insight_types_update_config(...)`, `locations_recommenders_get_config(...)`, `locations_recommenders_recommendations_get(...)`, `locations_recommenders_recommendations_list(...)`, `locations_recommenders_recommendations_mark_claimed(...)`, `locations_recommenders_recommendations_mark_dismissed(...)`, `locations_recommenders_recommendations_mark_failed(...)`, `locations_recommenders_recommendations_mark_succeeded(...)` and `locations_recommenders_update_config(...)`
/// // to build up your call.
/// let rb = hub.billing_accounts();
/// # }
/// ```
pub struct BillingAccountMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Recommender<S>,
}

impl<'a, S> client::MethodsBuilder for BillingAccountMethods<'a, S> {}

impl<'a, S> BillingAccountMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the requested insight. Requires the recommender.*.get IAM permission for the specified insight type.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the insight.
    pub fn locations_insight_types_insights_get(&self, name: &str) -> BillingAccountLocationInsightTypeInsightGetCall<'a, S> {
        BillingAccountLocationInsightTypeInsightGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists insights for the specified Cloud Resource. Requires the recommender.*.list IAM permission for the specified insight type.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The container resource on which to execute the request. Acceptable formats: * `projects/[PROJECT_NUMBER]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]` * `projects/[PROJECT_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]` * `billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]` * `folders/[FOLDER_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]` * `organizations/[ORGANIZATION_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]` LOCATION here refers to GCP Locations: https://cloud.google.com/about/locations/ INSIGHT_TYPE_ID refers to supported insight types: https://cloud.google.com/recommender/docs/insights/insight-types.
    pub fn locations_insight_types_insights_list(&self, parent: &str) -> BillingAccountLocationInsightTypeInsightListCall<'a, S> {
        BillingAccountLocationInsightTypeInsightListCall {
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
    /// Marks the Insight State as Accepted. Users can use this method to indicate to the Recommender API that they have applied some action based on the insight. This stops the insight content from being updated. MarkInsightAccepted can be applied to insights in ACTIVE state. Requires the recommender.*.update IAM permission for the specified insight.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the insight.
    pub fn locations_insight_types_insights_mark_accepted(&self, request: GoogleCloudRecommenderV1MarkInsightAcceptedRequest, name: &str) -> BillingAccountLocationInsightTypeInsightMarkAcceptedCall<'a, S> {
        BillingAccountLocationInsightTypeInsightMarkAcceptedCall {
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
    /// Gets the requested InsightTypeConfig. There is only one instance of the config for each InsightType.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the InsightTypeConfig to get. Acceptable formats: * `projects/[PROJECT_NUMBER]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]/config` * `projects/[PROJECT_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]/config` * `organizations/[ORGANIZATION_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]/config` * `billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]/config`
    pub fn locations_insight_types_get_config(&self, name: &str) -> BillingAccountLocationInsightTypeGetConfigCall<'a, S> {
        BillingAccountLocationInsightTypeGetConfigCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an InsightTypeConfig change. This will create a new revision of the config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Name of insight type config. Eg, projects/[PROJECT_NUMBER]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]/config
    pub fn locations_insight_types_update_config(&self, request: GoogleCloudRecommenderV1InsightTypeConfig, name: &str) -> BillingAccountLocationInsightTypeUpdateConfigCall<'a, S> {
        BillingAccountLocationInsightTypeUpdateConfigCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the requested recommendation. Requires the recommender.*.get IAM permission for the specified recommender.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the recommendation.
    pub fn locations_recommenders_recommendations_get(&self, name: &str) -> BillingAccountLocationRecommenderRecommendationGetCall<'a, S> {
        BillingAccountLocationRecommenderRecommendationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists recommendations for the specified Cloud Resource. Requires the recommender.*.list IAM permission for the specified recommender.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The container resource on which to execute the request. Acceptable formats: * `projects/[PROJECT_NUMBER]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]` * `projects/[PROJECT_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]` * `billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]` * `folders/[FOLDER_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]` * `organizations/[ORGANIZATION_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]` LOCATION here refers to GCP Locations: https://cloud.google.com/about/locations/ RECOMMENDER_ID refers to supported recommenders: https://cloud.google.com/recommender/docs/recommenders.
    pub fn locations_recommenders_recommendations_list(&self, parent: &str) -> BillingAccountLocationRecommenderRecommendationListCall<'a, S> {
        BillingAccountLocationRecommenderRecommendationListCall {
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
    /// Marks the Recommendation State as Claimed. Users can use this method to indicate to the Recommender API that they are starting to apply the recommendation themselves. This stops the recommendation content from being updated. Associated insights are frozen and placed in the ACCEPTED state. MarkRecommendationClaimed can be applied to recommendations in CLAIMED, SUCCEEDED, FAILED, or ACTIVE state. Requires the recommender.*.update IAM permission for the specified recommender.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the recommendation.
    pub fn locations_recommenders_recommendations_mark_claimed(&self, request: GoogleCloudRecommenderV1MarkRecommendationClaimedRequest, name: &str) -> BillingAccountLocationRecommenderRecommendationMarkClaimedCall<'a, S> {
        BillingAccountLocationRecommenderRecommendationMarkClaimedCall {
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
    /// Mark the Recommendation State as Dismissed. Users can use this method to indicate to the Recommender API that an ACTIVE recommendation has to be marked back as DISMISSED. MarkRecommendationDismissed can be applied to recommendations in ACTIVE state. Requires the recommender.*.update IAM permission for the specified recommender.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Name of the recommendation.
    pub fn locations_recommenders_recommendations_mark_dismissed(&self, request: GoogleCloudRecommenderV1MarkRecommendationDismissedRequest, name: &str) -> BillingAccountLocationRecommenderRecommendationMarkDismissedCall<'a, S> {
        BillingAccountLocationRecommenderRecommendationMarkDismissedCall {
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
    /// Marks the Recommendation State as Failed. Users can use this method to indicate to the Recommender API that they have applied the recommendation themselves, and the operation failed. This stops the recommendation content from being updated. Associated insights are frozen and placed in the ACCEPTED state. MarkRecommendationFailed can be applied to recommendations in ACTIVE, CLAIMED, SUCCEEDED, or FAILED state. Requires the recommender.*.update IAM permission for the specified recommender.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the recommendation.
    pub fn locations_recommenders_recommendations_mark_failed(&self, request: GoogleCloudRecommenderV1MarkRecommendationFailedRequest, name: &str) -> BillingAccountLocationRecommenderRecommendationMarkFailedCall<'a, S> {
        BillingAccountLocationRecommenderRecommendationMarkFailedCall {
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
    /// Marks the Recommendation State as Succeeded. Users can use this method to indicate to the Recommender API that they have applied the recommendation themselves, and the operation was successful. This stops the recommendation content from being updated. Associated insights are frozen and placed in the ACCEPTED state. MarkRecommendationSucceeded can be applied to recommendations in ACTIVE, CLAIMED, SUCCEEDED, or FAILED state. Requires the recommender.*.update IAM permission for the specified recommender.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the recommendation.
    pub fn locations_recommenders_recommendations_mark_succeeded(&self, request: GoogleCloudRecommenderV1MarkRecommendationSucceededRequest, name: &str) -> BillingAccountLocationRecommenderRecommendationMarkSucceededCall<'a, S> {
        BillingAccountLocationRecommenderRecommendationMarkSucceededCall {
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
    /// Gets the requested Recommender Config. There is only one instance of the config for each Recommender.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the Recommendation Config to get. Acceptable formats: * `projects/[PROJECT_NUMBER]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]/config` * `projects/[PROJECT_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]/config` * `organizations/[ORGANIZATION_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]/config` * `billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]/config`
    pub fn locations_recommenders_get_config(&self, name: &str) -> BillingAccountLocationRecommenderGetConfigCall<'a, S> {
        BillingAccountLocationRecommenderGetConfigCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a Recommender Config. This will create a new revision of the config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Name of recommender config. Eg, projects/[PROJECT_NUMBER]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]/config
    pub fn locations_recommenders_update_config(&self, request: GoogleCloudRecommenderV1RecommenderConfig, name: &str) -> BillingAccountLocationRecommenderUpdateConfigCall<'a, S> {
        BillingAccountLocationRecommenderUpdateConfigCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *folder* resources.
/// It is not used directly, but through the [`Recommender`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_recommender1 as recommender1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use recommender1::{Recommender, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Recommender::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_insight_types_insights_get(...)`, `locations_insight_types_insights_list(...)`, `locations_insight_types_insights_mark_accepted(...)`, `locations_recommenders_recommendations_get(...)`, `locations_recommenders_recommendations_list(...)`, `locations_recommenders_recommendations_mark_claimed(...)`, `locations_recommenders_recommendations_mark_dismissed(...)`, `locations_recommenders_recommendations_mark_failed(...)` and `locations_recommenders_recommendations_mark_succeeded(...)`
/// // to build up your call.
/// let rb = hub.folders();
/// # }
/// ```
pub struct FolderMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Recommender<S>,
}

impl<'a, S> client::MethodsBuilder for FolderMethods<'a, S> {}

impl<'a, S> FolderMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the requested insight. Requires the recommender.*.get IAM permission for the specified insight type.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the insight.
    pub fn locations_insight_types_insights_get(&self, name: &str) -> FolderLocationInsightTypeInsightGetCall<'a, S> {
        FolderLocationInsightTypeInsightGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists insights for the specified Cloud Resource. Requires the recommender.*.list IAM permission for the specified insight type.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The container resource on which to execute the request. Acceptable formats: * `projects/[PROJECT_NUMBER]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]` * `projects/[PROJECT_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]` * `billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]` * `folders/[FOLDER_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]` * `organizations/[ORGANIZATION_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]` LOCATION here refers to GCP Locations: https://cloud.google.com/about/locations/ INSIGHT_TYPE_ID refers to supported insight types: https://cloud.google.com/recommender/docs/insights/insight-types.
    pub fn locations_insight_types_insights_list(&self, parent: &str) -> FolderLocationInsightTypeInsightListCall<'a, S> {
        FolderLocationInsightTypeInsightListCall {
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
    /// Marks the Insight State as Accepted. Users can use this method to indicate to the Recommender API that they have applied some action based on the insight. This stops the insight content from being updated. MarkInsightAccepted can be applied to insights in ACTIVE state. Requires the recommender.*.update IAM permission for the specified insight.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the insight.
    pub fn locations_insight_types_insights_mark_accepted(&self, request: GoogleCloudRecommenderV1MarkInsightAcceptedRequest, name: &str) -> FolderLocationInsightTypeInsightMarkAcceptedCall<'a, S> {
        FolderLocationInsightTypeInsightMarkAcceptedCall {
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
    /// Gets the requested recommendation. Requires the recommender.*.get IAM permission for the specified recommender.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the recommendation.
    pub fn locations_recommenders_recommendations_get(&self, name: &str) -> FolderLocationRecommenderRecommendationGetCall<'a, S> {
        FolderLocationRecommenderRecommendationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists recommendations for the specified Cloud Resource. Requires the recommender.*.list IAM permission for the specified recommender.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The container resource on which to execute the request. Acceptable formats: * `projects/[PROJECT_NUMBER]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]` * `projects/[PROJECT_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]` * `billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]` * `folders/[FOLDER_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]` * `organizations/[ORGANIZATION_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]` LOCATION here refers to GCP Locations: https://cloud.google.com/about/locations/ RECOMMENDER_ID refers to supported recommenders: https://cloud.google.com/recommender/docs/recommenders.
    pub fn locations_recommenders_recommendations_list(&self, parent: &str) -> FolderLocationRecommenderRecommendationListCall<'a, S> {
        FolderLocationRecommenderRecommendationListCall {
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
    /// Marks the Recommendation State as Claimed. Users can use this method to indicate to the Recommender API that they are starting to apply the recommendation themselves. This stops the recommendation content from being updated. Associated insights are frozen and placed in the ACCEPTED state. MarkRecommendationClaimed can be applied to recommendations in CLAIMED, SUCCEEDED, FAILED, or ACTIVE state. Requires the recommender.*.update IAM permission for the specified recommender.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the recommendation.
    pub fn locations_recommenders_recommendations_mark_claimed(&self, request: GoogleCloudRecommenderV1MarkRecommendationClaimedRequest, name: &str) -> FolderLocationRecommenderRecommendationMarkClaimedCall<'a, S> {
        FolderLocationRecommenderRecommendationMarkClaimedCall {
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
    /// Mark the Recommendation State as Dismissed. Users can use this method to indicate to the Recommender API that an ACTIVE recommendation has to be marked back as DISMISSED. MarkRecommendationDismissed can be applied to recommendations in ACTIVE state. Requires the recommender.*.update IAM permission for the specified recommender.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Name of the recommendation.
    pub fn locations_recommenders_recommendations_mark_dismissed(&self, request: GoogleCloudRecommenderV1MarkRecommendationDismissedRequest, name: &str) -> FolderLocationRecommenderRecommendationMarkDismissedCall<'a, S> {
        FolderLocationRecommenderRecommendationMarkDismissedCall {
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
    /// Marks the Recommendation State as Failed. Users can use this method to indicate to the Recommender API that they have applied the recommendation themselves, and the operation failed. This stops the recommendation content from being updated. Associated insights are frozen and placed in the ACCEPTED state. MarkRecommendationFailed can be applied to recommendations in ACTIVE, CLAIMED, SUCCEEDED, or FAILED state. Requires the recommender.*.update IAM permission for the specified recommender.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the recommendation.
    pub fn locations_recommenders_recommendations_mark_failed(&self, request: GoogleCloudRecommenderV1MarkRecommendationFailedRequest, name: &str) -> FolderLocationRecommenderRecommendationMarkFailedCall<'a, S> {
        FolderLocationRecommenderRecommendationMarkFailedCall {
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
    /// Marks the Recommendation State as Succeeded. Users can use this method to indicate to the Recommender API that they have applied the recommendation themselves, and the operation was successful. This stops the recommendation content from being updated. Associated insights are frozen and placed in the ACCEPTED state. MarkRecommendationSucceeded can be applied to recommendations in ACTIVE, CLAIMED, SUCCEEDED, or FAILED state. Requires the recommender.*.update IAM permission for the specified recommender.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the recommendation.
    pub fn locations_recommenders_recommendations_mark_succeeded(&self, request: GoogleCloudRecommenderV1MarkRecommendationSucceededRequest, name: &str) -> FolderLocationRecommenderRecommendationMarkSucceededCall<'a, S> {
        FolderLocationRecommenderRecommendationMarkSucceededCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *organization* resources.
/// It is not used directly, but through the [`Recommender`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_recommender1 as recommender1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use recommender1::{Recommender, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Recommender::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_insight_types_get_config(...)`, `locations_insight_types_insights_get(...)`, `locations_insight_types_insights_list(...)`, `locations_insight_types_insights_mark_accepted(...)`, `locations_insight_types_update_config(...)`, `locations_recommenders_get_config(...)`, `locations_recommenders_recommendations_get(...)`, `locations_recommenders_recommendations_list(...)`, `locations_recommenders_recommendations_mark_claimed(...)`, `locations_recommenders_recommendations_mark_dismissed(...)`, `locations_recommenders_recommendations_mark_failed(...)`, `locations_recommenders_recommendations_mark_succeeded(...)` and `locations_recommenders_update_config(...)`
/// // to build up your call.
/// let rb = hub.organizations();
/// # }
/// ```
pub struct OrganizationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Recommender<S>,
}

impl<'a, S> client::MethodsBuilder for OrganizationMethods<'a, S> {}

impl<'a, S> OrganizationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the requested insight. Requires the recommender.*.get IAM permission for the specified insight type.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the insight.
    pub fn locations_insight_types_insights_get(&self, name: &str) -> OrganizationLocationInsightTypeInsightGetCall<'a, S> {
        OrganizationLocationInsightTypeInsightGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists insights for the specified Cloud Resource. Requires the recommender.*.list IAM permission for the specified insight type.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The container resource on which to execute the request. Acceptable formats: * `projects/[PROJECT_NUMBER]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]` * `projects/[PROJECT_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]` * `billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]` * `folders/[FOLDER_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]` * `organizations/[ORGANIZATION_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]` LOCATION here refers to GCP Locations: https://cloud.google.com/about/locations/ INSIGHT_TYPE_ID refers to supported insight types: https://cloud.google.com/recommender/docs/insights/insight-types.
    pub fn locations_insight_types_insights_list(&self, parent: &str) -> OrganizationLocationInsightTypeInsightListCall<'a, S> {
        OrganizationLocationInsightTypeInsightListCall {
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
    /// Marks the Insight State as Accepted. Users can use this method to indicate to the Recommender API that they have applied some action based on the insight. This stops the insight content from being updated. MarkInsightAccepted can be applied to insights in ACTIVE state. Requires the recommender.*.update IAM permission for the specified insight.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the insight.
    pub fn locations_insight_types_insights_mark_accepted(&self, request: GoogleCloudRecommenderV1MarkInsightAcceptedRequest, name: &str) -> OrganizationLocationInsightTypeInsightMarkAcceptedCall<'a, S> {
        OrganizationLocationInsightTypeInsightMarkAcceptedCall {
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
    /// Gets the requested InsightTypeConfig. There is only one instance of the config for each InsightType.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the InsightTypeConfig to get. Acceptable formats: * `projects/[PROJECT_NUMBER]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]/config` * `projects/[PROJECT_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]/config` * `organizations/[ORGANIZATION_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]/config` * `billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]/config`
    pub fn locations_insight_types_get_config(&self, name: &str) -> OrganizationLocationInsightTypeGetConfigCall<'a, S> {
        OrganizationLocationInsightTypeGetConfigCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an InsightTypeConfig change. This will create a new revision of the config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Name of insight type config. Eg, projects/[PROJECT_NUMBER]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]/config
    pub fn locations_insight_types_update_config(&self, request: GoogleCloudRecommenderV1InsightTypeConfig, name: &str) -> OrganizationLocationInsightTypeUpdateConfigCall<'a, S> {
        OrganizationLocationInsightTypeUpdateConfigCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the requested recommendation. Requires the recommender.*.get IAM permission for the specified recommender.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the recommendation.
    pub fn locations_recommenders_recommendations_get(&self, name: &str) -> OrganizationLocationRecommenderRecommendationGetCall<'a, S> {
        OrganizationLocationRecommenderRecommendationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists recommendations for the specified Cloud Resource. Requires the recommender.*.list IAM permission for the specified recommender.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The container resource on which to execute the request. Acceptable formats: * `projects/[PROJECT_NUMBER]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]` * `projects/[PROJECT_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]` * `billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]` * `folders/[FOLDER_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]` * `organizations/[ORGANIZATION_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]` LOCATION here refers to GCP Locations: https://cloud.google.com/about/locations/ RECOMMENDER_ID refers to supported recommenders: https://cloud.google.com/recommender/docs/recommenders.
    pub fn locations_recommenders_recommendations_list(&self, parent: &str) -> OrganizationLocationRecommenderRecommendationListCall<'a, S> {
        OrganizationLocationRecommenderRecommendationListCall {
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
    /// Marks the Recommendation State as Claimed. Users can use this method to indicate to the Recommender API that they are starting to apply the recommendation themselves. This stops the recommendation content from being updated. Associated insights are frozen and placed in the ACCEPTED state. MarkRecommendationClaimed can be applied to recommendations in CLAIMED, SUCCEEDED, FAILED, or ACTIVE state. Requires the recommender.*.update IAM permission for the specified recommender.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the recommendation.
    pub fn locations_recommenders_recommendations_mark_claimed(&self, request: GoogleCloudRecommenderV1MarkRecommendationClaimedRequest, name: &str) -> OrganizationLocationRecommenderRecommendationMarkClaimedCall<'a, S> {
        OrganizationLocationRecommenderRecommendationMarkClaimedCall {
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
    /// Mark the Recommendation State as Dismissed. Users can use this method to indicate to the Recommender API that an ACTIVE recommendation has to be marked back as DISMISSED. MarkRecommendationDismissed can be applied to recommendations in ACTIVE state. Requires the recommender.*.update IAM permission for the specified recommender.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Name of the recommendation.
    pub fn locations_recommenders_recommendations_mark_dismissed(&self, request: GoogleCloudRecommenderV1MarkRecommendationDismissedRequest, name: &str) -> OrganizationLocationRecommenderRecommendationMarkDismissedCall<'a, S> {
        OrganizationLocationRecommenderRecommendationMarkDismissedCall {
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
    /// Marks the Recommendation State as Failed. Users can use this method to indicate to the Recommender API that they have applied the recommendation themselves, and the operation failed. This stops the recommendation content from being updated. Associated insights are frozen and placed in the ACCEPTED state. MarkRecommendationFailed can be applied to recommendations in ACTIVE, CLAIMED, SUCCEEDED, or FAILED state. Requires the recommender.*.update IAM permission for the specified recommender.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the recommendation.
    pub fn locations_recommenders_recommendations_mark_failed(&self, request: GoogleCloudRecommenderV1MarkRecommendationFailedRequest, name: &str) -> OrganizationLocationRecommenderRecommendationMarkFailedCall<'a, S> {
        OrganizationLocationRecommenderRecommendationMarkFailedCall {
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
    /// Marks the Recommendation State as Succeeded. Users can use this method to indicate to the Recommender API that they have applied the recommendation themselves, and the operation was successful. This stops the recommendation content from being updated. Associated insights are frozen and placed in the ACCEPTED state. MarkRecommendationSucceeded can be applied to recommendations in ACTIVE, CLAIMED, SUCCEEDED, or FAILED state. Requires the recommender.*.update IAM permission for the specified recommender.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the recommendation.
    pub fn locations_recommenders_recommendations_mark_succeeded(&self, request: GoogleCloudRecommenderV1MarkRecommendationSucceededRequest, name: &str) -> OrganizationLocationRecommenderRecommendationMarkSucceededCall<'a, S> {
        OrganizationLocationRecommenderRecommendationMarkSucceededCall {
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
    /// Gets the requested Recommender Config. There is only one instance of the config for each Recommender.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the Recommendation Config to get. Acceptable formats: * `projects/[PROJECT_NUMBER]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]/config` * `projects/[PROJECT_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]/config` * `organizations/[ORGANIZATION_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]/config` * `billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]/config`
    pub fn locations_recommenders_get_config(&self, name: &str) -> OrganizationLocationRecommenderGetConfigCall<'a, S> {
        OrganizationLocationRecommenderGetConfigCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a Recommender Config. This will create a new revision of the config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Name of recommender config. Eg, projects/[PROJECT_NUMBER]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]/config
    pub fn locations_recommenders_update_config(&self, request: GoogleCloudRecommenderV1RecommenderConfig, name: &str) -> OrganizationLocationRecommenderUpdateConfigCall<'a, S> {
        OrganizationLocationRecommenderUpdateConfigCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Recommender`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_recommender1 as recommender1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use recommender1::{Recommender, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Recommender::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_insight_types_get_config(...)`, `locations_insight_types_insights_get(...)`, `locations_insight_types_insights_list(...)`, `locations_insight_types_insights_mark_accepted(...)`, `locations_insight_types_update_config(...)`, `locations_recommenders_get_config(...)`, `locations_recommenders_recommendations_get(...)`, `locations_recommenders_recommendations_list(...)`, `locations_recommenders_recommendations_mark_claimed(...)`, `locations_recommenders_recommendations_mark_dismissed(...)`, `locations_recommenders_recommendations_mark_failed(...)`, `locations_recommenders_recommendations_mark_succeeded(...)` and `locations_recommenders_update_config(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Recommender<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the requested insight. Requires the recommender.*.get IAM permission for the specified insight type.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the insight.
    pub fn locations_insight_types_insights_get(&self, name: &str) -> ProjectLocationInsightTypeInsightGetCall<'a, S> {
        ProjectLocationInsightTypeInsightGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists insights for the specified Cloud Resource. Requires the recommender.*.list IAM permission for the specified insight type.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The container resource on which to execute the request. Acceptable formats: * `projects/[PROJECT_NUMBER]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]` * `projects/[PROJECT_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]` * `billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]` * `folders/[FOLDER_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]` * `organizations/[ORGANIZATION_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]` LOCATION here refers to GCP Locations: https://cloud.google.com/about/locations/ INSIGHT_TYPE_ID refers to supported insight types: https://cloud.google.com/recommender/docs/insights/insight-types.
    pub fn locations_insight_types_insights_list(&self, parent: &str) -> ProjectLocationInsightTypeInsightListCall<'a, S> {
        ProjectLocationInsightTypeInsightListCall {
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
    /// Marks the Insight State as Accepted. Users can use this method to indicate to the Recommender API that they have applied some action based on the insight. This stops the insight content from being updated. MarkInsightAccepted can be applied to insights in ACTIVE state. Requires the recommender.*.update IAM permission for the specified insight.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the insight.
    pub fn locations_insight_types_insights_mark_accepted(&self, request: GoogleCloudRecommenderV1MarkInsightAcceptedRequest, name: &str) -> ProjectLocationInsightTypeInsightMarkAcceptedCall<'a, S> {
        ProjectLocationInsightTypeInsightMarkAcceptedCall {
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
    /// Gets the requested InsightTypeConfig. There is only one instance of the config for each InsightType.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the InsightTypeConfig to get. Acceptable formats: * `projects/[PROJECT_NUMBER]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]/config` * `projects/[PROJECT_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]/config` * `organizations/[ORGANIZATION_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]/config` * `billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]/config`
    pub fn locations_insight_types_get_config(&self, name: &str) -> ProjectLocationInsightTypeGetConfigCall<'a, S> {
        ProjectLocationInsightTypeGetConfigCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an InsightTypeConfig change. This will create a new revision of the config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Name of insight type config. Eg, projects/[PROJECT_NUMBER]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]/config
    pub fn locations_insight_types_update_config(&self, request: GoogleCloudRecommenderV1InsightTypeConfig, name: &str) -> ProjectLocationInsightTypeUpdateConfigCall<'a, S> {
        ProjectLocationInsightTypeUpdateConfigCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the requested recommendation. Requires the recommender.*.get IAM permission for the specified recommender.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the recommendation.
    pub fn locations_recommenders_recommendations_get(&self, name: &str) -> ProjectLocationRecommenderRecommendationGetCall<'a, S> {
        ProjectLocationRecommenderRecommendationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists recommendations for the specified Cloud Resource. Requires the recommender.*.list IAM permission for the specified recommender.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The container resource on which to execute the request. Acceptable formats: * `projects/[PROJECT_NUMBER]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]` * `projects/[PROJECT_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]` * `billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]` * `folders/[FOLDER_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]` * `organizations/[ORGANIZATION_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]` LOCATION here refers to GCP Locations: https://cloud.google.com/about/locations/ RECOMMENDER_ID refers to supported recommenders: https://cloud.google.com/recommender/docs/recommenders.
    pub fn locations_recommenders_recommendations_list(&self, parent: &str) -> ProjectLocationRecommenderRecommendationListCall<'a, S> {
        ProjectLocationRecommenderRecommendationListCall {
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
    /// Marks the Recommendation State as Claimed. Users can use this method to indicate to the Recommender API that they are starting to apply the recommendation themselves. This stops the recommendation content from being updated. Associated insights are frozen and placed in the ACCEPTED state. MarkRecommendationClaimed can be applied to recommendations in CLAIMED, SUCCEEDED, FAILED, or ACTIVE state. Requires the recommender.*.update IAM permission for the specified recommender.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the recommendation.
    pub fn locations_recommenders_recommendations_mark_claimed(&self, request: GoogleCloudRecommenderV1MarkRecommendationClaimedRequest, name: &str) -> ProjectLocationRecommenderRecommendationMarkClaimedCall<'a, S> {
        ProjectLocationRecommenderRecommendationMarkClaimedCall {
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
    /// Mark the Recommendation State as Dismissed. Users can use this method to indicate to the Recommender API that an ACTIVE recommendation has to be marked back as DISMISSED. MarkRecommendationDismissed can be applied to recommendations in ACTIVE state. Requires the recommender.*.update IAM permission for the specified recommender.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Name of the recommendation.
    pub fn locations_recommenders_recommendations_mark_dismissed(&self, request: GoogleCloudRecommenderV1MarkRecommendationDismissedRequest, name: &str) -> ProjectLocationRecommenderRecommendationMarkDismissedCall<'a, S> {
        ProjectLocationRecommenderRecommendationMarkDismissedCall {
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
    /// Marks the Recommendation State as Failed. Users can use this method to indicate to the Recommender API that they have applied the recommendation themselves, and the operation failed. This stops the recommendation content from being updated. Associated insights are frozen and placed in the ACCEPTED state. MarkRecommendationFailed can be applied to recommendations in ACTIVE, CLAIMED, SUCCEEDED, or FAILED state. Requires the recommender.*.update IAM permission for the specified recommender.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the recommendation.
    pub fn locations_recommenders_recommendations_mark_failed(&self, request: GoogleCloudRecommenderV1MarkRecommendationFailedRequest, name: &str) -> ProjectLocationRecommenderRecommendationMarkFailedCall<'a, S> {
        ProjectLocationRecommenderRecommendationMarkFailedCall {
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
    /// Marks the Recommendation State as Succeeded. Users can use this method to indicate to the Recommender API that they have applied the recommendation themselves, and the operation was successful. This stops the recommendation content from being updated. Associated insights are frozen and placed in the ACCEPTED state. MarkRecommendationSucceeded can be applied to recommendations in ACTIVE, CLAIMED, SUCCEEDED, or FAILED state. Requires the recommender.*.update IAM permission for the specified recommender.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the recommendation.
    pub fn locations_recommenders_recommendations_mark_succeeded(&self, request: GoogleCloudRecommenderV1MarkRecommendationSucceededRequest, name: &str) -> ProjectLocationRecommenderRecommendationMarkSucceededCall<'a, S> {
        ProjectLocationRecommenderRecommendationMarkSucceededCall {
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
    /// Gets the requested Recommender Config. There is only one instance of the config for each Recommender.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the Recommendation Config to get. Acceptable formats: * `projects/[PROJECT_NUMBER]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]/config` * `projects/[PROJECT_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]/config` * `organizations/[ORGANIZATION_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]/config` * `billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]/config`
    pub fn locations_recommenders_get_config(&self, name: &str) -> ProjectLocationRecommenderGetConfigCall<'a, S> {
        ProjectLocationRecommenderGetConfigCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a Recommender Config. This will create a new revision of the config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Name of recommender config. Eg, projects/[PROJECT_NUMBER]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]/config
    pub fn locations_recommenders_update_config(&self, request: GoogleCloudRecommenderV1RecommenderConfig, name: &str) -> ProjectLocationRecommenderUpdateConfigCall<'a, S> {
        ProjectLocationRecommenderUpdateConfigCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



