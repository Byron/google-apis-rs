use super::*;
/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FieldMask {
    /// no description provided
    
    pub fields: Option<Vec<FieldMask>>,
    /// no description provided
    
    pub id: Option<i32>,
}

impl client::Part for FieldMask {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get mobileapppanels](MobileapppanelGetCall) (response)
/// * [update mobileapppanels](MobileapppanelUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MobileAppPanel {
    /// no description provided
    
    pub country: Option<String>,
    /// no description provided
    #[serde(rename="isPublicPanel")]
    
    pub is_public_panel: Option<bool>,
    /// no description provided
    
    pub language: Option<String>,
    /// no description provided
    #[serde(rename="mobileAppPanelId")]
    
    pub mobile_app_panel_id: Option<String>,
    /// no description provided
    
    pub name: Option<String>,
    /// no description provided
    
    pub owners: Option<Vec<String>>,
}

impl client::RequestValue for MobileAppPanel {}
impl client::Resource for MobileAppPanel {}
impl client::ResponseResult for MobileAppPanel {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list mobileapppanels](MobileapppanelListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MobileAppPanelsListResponse {
    /// no description provided
    #[serde(rename="pageInfo")]
    
    pub page_info: Option<PageInfo>,
    /// Unique request ID used for logging and debugging. Please include in any error reporting or troubleshooting requests.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
    /// An individual predefined panel of Opinion Rewards mobile users.
    
    pub resources: Option<Vec<MobileAppPanel>>,
    /// no description provided
    #[serde(rename="tokenPagination")]
    
    pub token_pagination: Option<TokenPagination>,
}

impl client::ResponseResult for MobileAppPanelsListResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PageInfo {
    /// no description provided
    #[serde(rename="resultPerPage")]
    
    pub result_per_page: Option<i32>,
    /// no description provided
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// no description provided
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
}

impl client::Part for PageInfo {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get results](ResultGetCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResultsGetRequest {
    /// no description provided
    #[serde(rename="resultMask")]
    
    pub result_mask: Option<ResultsMask>,
}

impl client::RequestValue for ResultsGetRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResultsMask {
    /// no description provided
    
    pub fields: Option<Vec<FieldMask>>,
    /// no description provided
    
    pub projection: Option<String>,
}

impl client::Part for ResultsMask {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete surveys](SurveyDeleteCall) (none)
/// * [get surveys](SurveyGetCall) (response)
/// * [insert surveys](SurveyInsertCall) (request|response)
/// * [list surveys](SurveyListCall) (none)
/// * [start surveys](SurveyStartCall) (none)
/// * [stop surveys](SurveyStopCall) (none)
/// * [update surveys](SurveyUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Survey {
    /// no description provided
    
    pub audience: Option<SurveyAudience>,
    /// no description provided
    
    pub cost: Option<SurveyCost>,
    /// no description provided
    #[serde(rename="customerData")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub customer_data: Option<Vec<u8>>,
    /// no description provided
    
    pub description: Option<String>,
    /// no description provided
    
    pub owners: Option<Vec<String>>,
    /// no description provided
    
    pub questions: Option<Vec<SurveyQuestion>>,
    /// no description provided
    #[serde(rename="rejectionReason")]
    
    pub rejection_reason: Option<SurveyRejection>,
    /// no description provided
    
    pub state: Option<String>,
    /// no description provided
    #[serde(rename="surveyUrlId")]
    
    pub survey_url_id: Option<String>,
    /// no description provided
    
    pub title: Option<String>,
    /// no description provided
    #[serde(rename="wantedResponseCount")]
    
    pub wanted_response_count: Option<i32>,
}

impl client::RequestValue for Survey {}
impl client::Resource for Survey {}
impl client::ResponseResult for Survey {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SurveyAudience {
    /// no description provided
    
    pub ages: Option<Vec<String>>,
    /// no description provided
    
    pub country: Option<String>,
    /// no description provided
    #[serde(rename="countrySubdivision")]
    
    pub country_subdivision: Option<String>,
    /// no description provided
    
    pub gender: Option<String>,
    /// no description provided
    
    pub languages: Option<Vec<String>>,
    /// no description provided
    #[serde(rename="mobileAppPanelId")]
    
    pub mobile_app_panel_id: Option<String>,
    /// no description provided
    #[serde(rename="populationSource")]
    
    pub population_source: Option<String>,
}

impl client::Part for SurveyAudience {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SurveyCost {
    /// no description provided
    #[serde(rename="costPerResponseNanos")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub cost_per_response_nanos: Option<i64>,
    /// no description provided
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// no description provided
    #[serde(rename="maxCostPerResponseNanos")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_cost_per_response_nanos: Option<i64>,
    /// no description provided
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub nanos: Option<i64>,
}

impl client::Part for SurveyCost {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SurveyQuestion {
    /// no description provided
    #[serde(rename="answerOrder")]
    
    pub answer_order: Option<String>,
    /// no description provided
    
    pub answers: Option<Vec<String>>,
    /// no description provided
    #[serde(rename="hasOther")]
    
    pub has_other: Option<bool>,
    /// no description provided
    #[serde(rename="highValueLabel")]
    
    pub high_value_label: Option<String>,
    /// no description provided
    
    pub images: Option<Vec<SurveyQuestionImage>>,
    /// no description provided
    #[serde(rename="lastAnswerPositionPinned")]
    
    pub last_answer_position_pinned: Option<bool>,
    /// no description provided
    #[serde(rename="lowValueLabel")]
    
    pub low_value_label: Option<String>,
    /// no description provided
    #[serde(rename="mustPickSuggestion")]
    
    pub must_pick_suggestion: Option<bool>,
    /// no description provided
    #[serde(rename="numStars")]
    
    pub num_stars: Option<String>,
    /// no description provided
    #[serde(rename="openTextPlaceholder")]
    
    pub open_text_placeholder: Option<String>,
    /// no description provided
    #[serde(rename="openTextSuggestions")]
    
    pub open_text_suggestions: Option<Vec<String>>,
    /// no description provided
    
    pub question: Option<String>,
    /// no description provided
    #[serde(rename="sentimentText")]
    
    pub sentiment_text: Option<String>,
    /// no description provided
    #[serde(rename="singleLineResponse")]
    
    pub single_line_response: Option<bool>,
    /// no description provided
    #[serde(rename="thresholdAnswers")]
    
    pub threshold_answers: Option<Vec<String>>,
    /// no description provided
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// no description provided
    #[serde(rename="unitOfMeasurementLabel")]
    
    pub unit_of_measurement_label: Option<String>,
    /// no description provided
    #[serde(rename="videoId")]
    
    pub video_id: Option<String>,
}

impl client::Part for SurveyQuestion {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SurveyQuestionImage {
    /// no description provided
    #[serde(rename="altText")]
    
    pub alt_text: Option<String>,
    /// no description provided
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// no description provided
    
    pub url: Option<String>,
}

impl client::Part for SurveyQuestionImage {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SurveyRejection {
    /// no description provided
    
    pub explanation: Option<String>,
    /// no description provided
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for SurveyRejection {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get results](ResultGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SurveyResults {
    /// no description provided
    
    pub status: Option<String>,
    /// no description provided
    #[serde(rename="surveyUrlId")]
    
    pub survey_url_id: Option<String>,
}

impl client::ResponseResult for SurveyResults {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete surveys](SurveyDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SurveysDeleteResponse {
    /// Unique request ID used for logging and debugging. Please include in any error reporting or troubleshooting requests.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::ResponseResult for SurveysDeleteResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list surveys](SurveyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SurveysListResponse {
    /// no description provided
    #[serde(rename="pageInfo")]
    
    pub page_info: Option<PageInfo>,
    /// Unique request ID used for logging and debugging. Please include in any error reporting or troubleshooting requests.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
    /// An individual survey resource.
    
    pub resources: Option<Vec<Survey>>,
    /// no description provided
    #[serde(rename="tokenPagination")]
    
    pub token_pagination: Option<TokenPagination>,
}

impl client::ResponseResult for SurveysListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [start surveys](SurveyStartCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SurveysStartRequest {
    /// Threshold to start a survey automically if the quoted prices is less than or equal to this value. See Survey.Cost for more details.
    #[serde(rename="maxCostPerResponseNanos")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_cost_per_response_nanos: Option<i64>,
}

impl client::RequestValue for SurveysStartRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [start surveys](SurveyStartCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SurveysStartResponse {
    /// Unique request ID used for logging and debugging. Please include in any error reporting or troubleshooting requests.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::ResponseResult for SurveysStartResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [stop surveys](SurveyStopCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SurveysStopResponse {
    /// Unique request ID used for logging and debugging. Please include in any error reporting or troubleshooting requests.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::ResponseResult for SurveysStopResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TokenPagination {
    /// no description provided
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// no description provided
    #[serde(rename="previousPageToken")]
    
    pub previous_page_token: Option<String>,
}

impl client::Part for TokenPagination {}


