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


/// Representation of an individual survey object.
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
    /// Targeting-criteria message containing demographic information
    
    pub audience: Option<SurveyAudience>,
    /// Cost to run the survey and collect the necessary number of responses.
    
    pub cost: Option<SurveyCost>,
    /// Additional information to store on behalf of the API consumer and associate with this question. This binary blob is treated as opaque. This field is limited to 64K bytes.
    #[serde(rename="customerData")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub customer_data: Option<Vec<u8>>,
    /// Text description of the survey.
    
    pub description: Option<String>,
    /// List of email addresses for survey owners. Must contain at least the address of the user making the API call.
    
    pub owners: Option<Vec<String>>,
    /// List of questions defining the survey.
    
    pub questions: Option<Vec<SurveyQuestion>>,
    /// Reason for the survey being rejected. Only present if the survey state is rejected.
    #[serde(rename="rejectionReason")]
    
    pub rejection_reason: Option<SurveyRejection>,
    /// State that the survey is in.
    
    pub state: Option<String>,
    /// Unique survey ID, that is viewable in the URL of the Survey Creator UI
    #[serde(rename="surveyUrlId")]
    
    pub survey_url_id: Option<String>,
    /// Optional name that will be given to the survey.
    
    pub title: Option<String>,
    /// Number of responses desired for the survey.
    #[serde(rename="wantedResponseCount")]
    
    pub wanted_response_count: Option<i32>,
}

impl client::RequestValue for Survey {}
impl client::Resource for Survey {}
impl client::ResponseResult for Survey {}


/// Specifications for the target audience of a survey run through the API.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SurveyAudience {
    /// Optional list of age buckets to target. Supported age buckets are: ['18-24', '25-34', '35-44', '45-54', '55-64', '65+']
    
    pub ages: Option<Vec<String>>,
    /// Required country code that surveys should be targeted to. Accepts standard ISO 3166-1 2 character language codes. For instance, 'US' for the United States, and 'GB' for the United Kingdom.
    
    pub country: Option<String>,
    /// Country subdivision (states/provinces/etc) that surveys should be targeted to. For all countries except GB, ISO-3166-2 subdivision code is required (eg. 'US-OH' for Ohio, United States). For GB, NUTS 1 statistical region codes for the United Kingdom is required (eg. 'UK-UKC' for North East England).
    #[serde(rename="countrySubdivision")]
    
    pub country_subdivision: Option<String>,
    /// Optional gender to target.
    
    pub gender: Option<String>,
    /// Language code that surveys should be targeted to. For instance, 'en-US'. Surveys may target bilingual users by specifying a list of language codes (for example, 'de' and 'en-US'). In that case, all languages will be used for targeting users but the survey content (which is displayed) must match the first language listed. Accepts standard BCP47 language codes. See specification.
    
    pub languages: Option<Vec<String>>,
    /// Online population source where the respondents are sampled from.
    #[serde(rename="populationSource")]
    
    pub population_source: Option<String>,
}

impl client::Part for SurveyAudience {}


/// Message defining the cost to run a given survey through API.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SurveyCost {
    /// Cost per survey response in nano units of the given currency. To get the total cost for a survey, multiply this value by wanted_response_count.
    #[serde(rename="costPerResponseNanos")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub cost_per_response_nanos: Option<i64>,
    /// Currency code that the cost is given in.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// *Deprecated* Threshold to start a survey automatically if the quoted price is at most this value. When a survey has a Screener (threshold) question, it must go through an incidence pricing test to determine the final cost per response. Typically you will have to make a followup call to start the survey giving the final computed cost per response. If the survey has no threshold_answers, setting this property will return an error. By specifying this property, you indicate the max price per response you are willing to pay in advance of the incidence test. If the price turns out to be lower than the specified value, the survey will begin immediately and you will be charged at the rate determined by the incidence pricing test. If the price turns out to be greater than the specified value the survey will not be started and you will instead be notified what price was determined by the incidence test. At that point, you must raise the value of this property to be greater than or equal to that cost before attempting to start the survey again. This will immediately start the survey as long the incidence test was run within the last 21 days. This will no longer be available after June 2018.
    #[serde(rename="maxCostPerResponseNanos")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_cost_per_response_nanos: Option<i64>,
    /// Cost of survey in nano units of the given currency. DEPRECATED in favor of cost_per_response_nanos
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub nanos: Option<i64>,
}

impl client::Part for SurveyCost {}


/// Message defining the question specifications.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SurveyQuestion {
    /// The randomization option for multiple choice and multi-select questions. If not specified, this option defaults to randomize.
    #[serde(rename="answerOrder")]
    
    pub answer_order: Option<String>,
    /// Required list of answer options for a question.
    
    pub answers: Option<Vec<String>>,
    /// Option to allow open-ended text box for Single Answer and Multiple Answer question types. This can be used with SINGLE_ANSWER, SINGLE_ANSWER_WITH_IMAGE, MULTIPLE_ANSWERS, and MULTIPLE_ANSWERS_WITH_IMAGE question types.
    #[serde(rename="hasOther")]
    
    pub has_other: Option<bool>,
    /// For rating questions, the text for the higher end of the scale, such as 'Best'. For numeric questions, a string representing a floating-point that is the maximum allowed number for a response.
    #[serde(rename="highValueLabel")]
    
    pub high_value_label: Option<String>,
    /// no description provided
    
    pub images: Option<Vec<SurveyQuestionImage>>,
    /// Currently only support pinning an answer option to the last position.
    #[serde(rename="lastAnswerPositionPinned")]
    
    pub last_answer_position_pinned: Option<bool>,
    /// For rating questions, the text for the lower end of the scale, such as 'Worst'. For numeric questions, a string representing a floating-point that is the minimum allowed number for a response.
    #[serde(rename="lowValueLabel")]
    
    pub low_value_label: Option<String>,
    /// Option to force the user to pick one of the open text suggestions. This requires that suggestions are provided for this question.
    #[serde(rename="mustPickSuggestion")]
    
    pub must_pick_suggestion: Option<bool>,
    /// Number of stars to use for ratings questions.
    #[serde(rename="numStars")]
    
    pub num_stars: Option<String>,
    /// Placeholder text for an open text question.
    #[serde(rename="openTextPlaceholder")]
    
    pub open_text_placeholder: Option<String>,
    /// A list of suggested answers for open text question auto-complete. This is only valid if single_line_response is true.
    #[serde(rename="openTextSuggestions")]
    
    pub open_text_suggestions: Option<Vec<String>>,
    /// Required question text shown to the respondent.
    
    pub question: Option<String>,
    /// Used by the Rating Scale with Text question type. This text goes along with the question field that is presented to the respondent, and is the actual text that the respondent is asked to rate.
    #[serde(rename="sentimentText")]
    
    pub sentiment_text: Option<String>,
    /// Option to allow multiple line open text responses instead of a single line response. Note that we don't show auto-complete suggestions with multiple line responses.
    #[serde(rename="singleLineResponse")]
    
    pub single_line_response: Option<bool>,
    /// The threshold/screener answer options, which will screen a user into the rest of the survey. These will be a subset of the answer option strings.
    #[serde(rename="thresholdAnswers")]
    
    pub threshold_answers: Option<Vec<String>>,
    /// Required field defining the question type. For details about configuring different type of questions, consult the question configuration guide.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Optional unit of measurement for display (for example: hours, people, miles).
    #[serde(rename="unitOfMeasurementLabel")]
    
    pub unit_of_measurement_label: Option<String>,
    /// The YouTube video ID to be show in video questions.
    #[serde(rename="videoId")]
    
    pub video_id: Option<String>,
}

impl client::Part for SurveyQuestion {}


/// Container object for image data and alt_text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SurveyQuestionImage {
    /// The alt text property used in image tags is required for all images.
    #[serde(rename="altText")]
    
    pub alt_text: Option<String>,
    /// Inline jpeg, gif, tiff, bmp, or png image raw bytes for an image question types.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// The read-only URL for the hosted images.
    
    pub url: Option<String>,
}

impl client::Part for SurveyQuestionImage {}


/// Message representing why the survey was rejected from review, if it was.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SurveyRejection {
    /// A human-readable explanation of what was wrong with the survey.
    
    pub explanation: Option<String>,
    /// Which category of rejection this was. See the  Google Surveys Help Center for additional details on each category.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for SurveyRejection {}


/// Reference to the current results for a given survey.
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
    /// Human readable string describing the status of the request.
    
    pub status: Option<String>,
    /// External survey ID as viewable by survey owners in the editor view.
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
    /// *Deprecated* Threshold to start a survey automatically if the quoted prices is less than or equal to this value. See Survey.Cost for more details. This will no longer be available after June 2018.
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


