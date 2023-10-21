use super::*;
/// A generic empty message that you can re-use to avoid defining duplicated
/// empty messages in your APIs. A typical example is to use it as the request
/// or the response type of an API method. For instance:
/// 
/// ````text
/// service Foo {
///   rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);
/// }
/// ````
/// 
/// The JSON representation for `Empty` is empty JSON object `{}`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete company relation users](UserDeleteCompanyRelationCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Source of traffic for the current request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TrafficSource {
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    #[serde(rename="trafficSourceId")]
    
    pub traffic_source_id: Option<String>,
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    #[serde(rename="trafficSubId")]
    
    pub traffic_sub_id: Option<String>,
}

impl client::Part for TrafficSource {}


/// Common data that is in each API request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RequestMetadata {
    /// Locale to use for the current request.
    
    pub locale: Option<String>,
    /// Values to use instead of the user's respective defaults for the current
    /// request. These are only honored by whitelisted products.
    #[serde(rename="userOverrides")]
    
    pub user_overrides: Option<UserOverrides>,
    /// Google Partners session ID.
    #[serde(rename="partnersSessionId")]
    
    pub partners_session_id: Option<String>,
    /// Experiment IDs the current request belongs to.
    #[serde(rename="experimentIds")]
    
    pub experiment_ids: Option<Vec<String>>,
    /// Source of traffic for the current request.
    #[serde(rename="trafficSource")]
    
    pub traffic_source: Option<TrafficSource>,
}

impl client::Part for RequestMetadata {}


/// Request message for CreateLead.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [leads create companies](CompanyLeadCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateLeadRequest {
    /// Current request metadata.
    #[serde(rename="requestMetadata")]
    
    pub request_metadata: Option<RequestMetadata>,
    /// The lead resource. The `LeadType` must not be `LEAD_TYPE_UNSPECIFIED`
    /// and either `email` or `phone_number` must be provided.
    
    pub lead: Option<Lead>,
    /// <a href="https://www.google.com/recaptcha/">reCaptcha</a> challenge info.
    #[serde(rename="recaptchaChallenge")]
    
    pub recaptcha_challenge: Option<RecaptchaChallenge>,
}

impl client::RequestValue for CreateLeadRequest {}


/// Key value data pair for an event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventData {
    /// Data type.
    
    pub key: Option<String>,
    /// Data values.
    
    pub values: Option<Vec<String>>,
}

impl client::Part for EventData {}


/// A user's information on a specific exam.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExamStatus {
    /// Whether this exam is in the state of warning.
    
    pub warning: Option<bool>,
    /// Date this exam is due to expire.
    
    pub expiration: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The date the user last passed this exam.
    #[serde(rename="lastPassed")]
    
    pub last_passed: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The type of the exam.
    #[serde(rename="examType")]
    
    pub exam_type: Option<String>,
    /// Whether this exam has been passed and not expired.
    
    pub passed: Option<bool>,
    /// The date the user last taken this exam.
    
    pub taken: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for ExamStatus {}


/// Response for ListOffer.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list offers](OfferListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOffersResponse {
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    
    pub response_metadata: Option<ResponseMetadata>,
    /// Reason why no Offers are available.
    #[serde(rename="noOfferReason")]
    
    pub no_offer_reason: Option<String>,
    /// Available Offers to be distributed.
    #[serde(rename="availableOffers")]
    
    pub available_offers: Option<Vec<AvailableOffer>>,
}

impl client::ResponseResult for ListOffersResponse {}


/// Offer info by country.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CountryOfferInfo {
    /// Country code for which offer codes may be requested.
    #[serde(rename="offerCountryCode")]
    
    pub offer_country_code: Option<String>,
    /// (localized) Spend X amount for that country's offer.
    #[serde(rename="spendXAmount")]
    
    pub spend_x_amount: Option<String>,
    /// Type of offer country is eligible for.
    #[serde(rename="offerType")]
    
    pub offer_type: Option<String>,
    /// (localized) Get Y amount for that country's offer.
    #[serde(rename="getYAmount")]
    
    pub get_y_amount: Option<String>,
}

impl client::Part for CountryOfferInfo {}


/// Response message for
/// ListCompanies.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list companies](CompanyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCompaniesResponse {
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    
    pub response_metadata: Option<ResponseMetadata>,
    /// The list of companies.
    
    pub companies: Option<Vec<Company>>,
    /// A token to retrieve next page of results.
    /// Pass this value in the `ListCompaniesRequest.page_token` field in the
    /// subsequent call to
    /// ListCompanies to retrieve the
    /// next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListCompaniesResponse {}


/// Customers qualified for an offer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OfferCustomer {
    /// URL to the customer's AdWords page.
    #[serde(rename="adwordsUrl")]
    
    pub adwords_url: Option<String>,
    /// Type of the offer
    #[serde(rename="offerType")]
    
    pub offer_type: Option<String>,
    /// External CID for the customer.
    #[serde(rename="externalCid")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub external_cid: Option<i64>,
    /// Country code of the customer.
    #[serde(rename="countryCode")]
    
    pub country_code: Option<String>,
    /// Time the customer was created.
    #[serde(rename="creationTime")]
    
    pub creation_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Days the customer is still eligible.
    #[serde(rename="eligibilityDaysLeft")]
    
    pub eligibility_days_left: Option<i32>,
    /// Formatted Get Y amount with currency code.
    #[serde(rename="getYAmount")]
    
    pub get_y_amount: Option<String>,
    /// Name of the customer.
    
    pub name: Option<String>,
    /// Formatted Spend X amount with currency code.
    #[serde(rename="spendXAmount")]
    
    pub spend_x_amount: Option<String>,
}

impl client::Part for OfferCustomer {}


/// Google Partners certification status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CertificationStatus {
    /// The type of the certification.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Number of people who are certified,
    #[serde(rename="userCount")]
    
    pub user_count: Option<i32>,
    /// Whether certification is passing.
    #[serde(rename="isCertified")]
    
    pub is_certified: Option<bool>,
    /// List of certification exam statuses.
    #[serde(rename="examStatuses")]
    
    pub exam_statuses: Option<Vec<CertificationExamStatus>>,
}

impl client::Part for CertificationStatus {}


/// The localized company information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocalizedCompanyInfo {
    /// Language code of the localized company info, as defined by
    /// <a href="https://tools.ietf.org/html/bcp47">BCP 47</a>
    /// (IETF BCP 47, "Tags for Identifying Languages").
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// List of country codes for the localized company info.
    #[serde(rename="countryCodes")]
    
    pub country_codes: Option<Vec<String>>,
    /// Localized brief description that the company uses to advertise themselves.
    
    pub overview: Option<String>,
    /// Localized display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
}

impl client::Part for LocalizedCompanyInfo {}


/// Response message for
/// LogUserEvent.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [log user events](UserEventLogCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogUserEventResponse {
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    
    pub response_metadata: Option<ResponseMetadata>,
}

impl client::ResponseResult for LogUserEventResponse {}


/// Response for ListOfferHistory.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [history list offers](OfferHistoryListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOffersHistoryResponse {
    /// True if the user has the option to show entire company history.
    #[serde(rename="canShowEntireCompany")]
    
    pub can_show_entire_company: Option<bool>,
    /// Number of results across all pages.
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
    /// True if this response is showing entire company history.
    #[serde(rename="showingEntireCompany")]
    
    pub showing_entire_company: Option<bool>,
    /// Historical offers meeting request.
    
    pub offers: Option<Vec<HistoricalOffer>>,
    /// Supply this token in a ListOffersHistoryRequest to retrieve the next page.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    
    pub response_metadata: Option<ResponseMetadata>,
}

impl client::ResponseResult for ListOffersHistoryResponse {}


/// Response message for
/// LogClientMessage.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [log client messages](ClientMessageLogCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogMessageResponse {
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    
    pub response_metadata: Option<ResponseMetadata>,
}

impl client::ResponseResult for LogMessageResponse {}


/// Agency specialization status
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpecializationStatus {
    /// The specialization this status is for.
    #[serde(rename="badgeSpecialization")]
    
    pub badge_specialization: Option<String>,
    /// State of agency specialization.
    #[serde(rename="badgeSpecializationState")]
    
    pub badge_specialization_state: Option<String>,
}

impl client::Part for SpecializationStatus {}


/// A user's information on a specific certification.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Certification {
    /// The date the user last achieved certification.
    #[serde(rename="lastAchieved")]
    
    pub last_achieved: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Whether this certification has been achieved.
    
    pub achieved: Option<bool>,
    /// Date this certification is due to expire.
    
    pub expiration: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Whether this certification is in the state of warning.
    
    pub warning: Option<bool>,
    /// The type of certification, the area of expertise.
    #[serde(rename="certificationType")]
    
    pub certification_type: Option<String>,
}

impl client::Part for Certification {}


/// A resource representing a user of the Partners platform.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update profile users](UserUpdateProfileCall) (none)
/// * [create company relation users](UserCreateCompanyRelationCall) (none)
/// * [delete company relation users](UserDeleteCompanyRelationCall) (none)
/// * [get users](UserGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// The profile information of a Partners user, contains all the directly
    /// editable user information.
    
    pub profile: Option<UserProfile>,
    /// This is the list of AdWords Manager Accounts the user has edit access to.
    /// If the user has edit access to multiple accounts, the user can choose the
    /// preferred account and we use this when a personal account is needed. Can
    /// be empty meaning the user has access to no accounts.
    /// @OutputOnly
    #[serde(rename="availableAdwordsManagerAccounts")]
    
    pub available_adwords_manager_accounts: Option<Vec<AdWordsManagerAccountInfo>>,
    /// The internal user ID.
    /// Only available for a whitelisted set of api clients.
    #[serde(rename="internalId")]
    
    pub internal_id: Option<String>,
    /// The list of exams the user ever taken. For each type of exam, only one
    /// entry is listed.
    #[serde(rename="examStatus")]
    
    pub exam_status: Option<Vec<ExamStatus>>,
    /// The ID of the user.
    
    pub id: Option<String>,
    /// Information about a user's external public profile outside Google Partners.
    #[serde(rename="publicProfile")]
    
    pub public_profile: Option<PublicProfile>,
    /// The email address used by the user used for company verification.
    /// @OutputOnly
    #[serde(rename="companyVerificationEmail")]
    
    pub company_verification_email: Option<String>,
    /// The company that the user is associated with.
    /// If not present, the user is not associated with any company.
    
    pub company: Option<CompanyRelation>,
    /// The most recent time the user interacted with the Partners site.
    /// @OutputOnly
    #[serde(rename="lastAccessTime")]
    
    pub last_access_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The list of emails the user has access to/can select as primary.
    /// @OutputOnly
    #[serde(rename="primaryEmails")]
    
    pub primary_emails: Option<Vec<String>>,
    /// The list of achieved certifications. These are calculated based on exam
    /// results and other requirements.
    /// @OutputOnly
    #[serde(rename="certificationStatus")]
    
    pub certification_status: Option<Vec<Certification>>,
    /// Whether or not the user has opted to share their Academy for Ads info with
    /// Google Partners.
    #[serde(rename="afaInfoShared")]
    
    pub afa_info_shared: Option<bool>,
}

impl client::Resource for User {}
impl client::ResponseResult for User {}


/// Response message for
/// ListAnalytics.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list analytics](AnalyticListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAnalyticsResponse {
    /// A token to retrieve next page of results.
    /// Pass this value in the `ListAnalyticsRequest.page_token` field in the
    /// subsequent call to
    /// ListAnalytics to retrieve the
    /// next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    
    pub response_metadata: Option<ResponseMetadata>,
    /// Aggregated information across the response's
    /// analytics.
    #[serde(rename="analyticsSummary")]
    
    pub analytics_summary: Option<AnalyticsSummary>,
    /// The list of analytics.
    /// Sorted in ascending order of
    /// Analytics.event_date.
    
    pub analytics: Option<Vec<Analytics>>,
}

impl client::ResponseResult for ListAnalyticsResponse {}


/// Response message for ListLeads.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list leads](LeadListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLeadsResponse {
    /// A token to retrieve next page of results.
    /// Pass this value in the `ListLeadsRequest.page_token` field in the
    /// subsequent call to
    /// ListLeads to retrieve the
    /// next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    
    pub response_metadata: Option<ResponseMetadata>,
    /// The total count of leads for the given company.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
    /// The list of leads.
    
    pub leads: Option<Vec<Lead>>,
}

impl client::ResponseResult for ListLeadsResponse {}


/// A company resource in the Google Partners API. Once certified, it qualifies
/// for being searched by advertisers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update companies](MethodUpdateCompanyCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Company {
    /// The public viewability status of the company's profile.
    #[serde(rename="profileStatus")]
    
    pub profile_status: Option<String>,
    /// The primary language code of the company, as defined by
    /// <a href="https://tools.ietf.org/html/bcp47">BCP 47</a>
    /// (IETF BCP 47, "Tags for Identifying Languages").
    #[serde(rename="primaryLanguageCode")]
    
    pub primary_language_code: Option<String>,
    /// The list of all company locations.
    /// If set, must include the
    /// primary_location
    /// in the list.
    
    pub locations: Option<Vec<Location>>,
    /// The minimum monthly budget that the company accepts for partner business,
    /// converted to the requested currency code.
    #[serde(rename="convertedMinMonthlyBudget")]
    
    pub converted_min_monthly_budget: Option<Money>,
    /// Industries the company can help with.
    
    pub industries: Option<Vec<String>>,
    /// URL of the company's website.
    #[serde(rename="websiteUrl")]
    
    pub website_url: Option<String>,
    /// URL of the company's additional websites used to verify the dynamic badges.
    /// These are stored as full URLs as entered by the user, but only the TLD will
    /// be used for the actual verification.
    #[serde(rename="additionalWebsites")]
    
    pub additional_websites: Option<Vec<String>>,
    /// The Primary AdWords Manager Account id.
    #[serde(rename="primaryAdwordsManagerAccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub primary_adwords_manager_account_id: Option<i64>,
    /// Whether the company's badge authority is in AWN
    #[serde(rename="badgeAuthorityInAwn")]
    
    pub badge_authority_in_awn: Option<bool>,
    /// The name of the company.
    
    pub name: Option<String>,
    /// The list of localized info for the company.
    #[serde(rename="localizedInfos")]
    
    pub localized_infos: Option<Vec<LocalizedCompanyInfo>>,
    /// The list of Google Partners certification statuses for the company.
    #[serde(rename="certificationStatuses")]
    
    pub certification_statuses: Option<Vec<CertificationStatus>>,
    /// The ID of the company.
    
    pub id: Option<String>,
    /// Basic information from the company's public profile.
    #[serde(rename="publicProfile")]
    
    pub public_profile: Option<PublicProfile>,
    /// The unconverted minimum monthly budget that the company accepts for partner
    /// business.
    #[serde(rename="originalMinMonthlyBudget")]
    
    pub original_min_monthly_budget: Option<Money>,
    /// Services the company can help with.
    
    pub services: Option<Vec<String>>,
    /// The primary location of the company.
    #[serde(rename="primaryLocation")]
    
    pub primary_location: Option<Location>,
    /// Information related to the ranking of the company within the list of
    /// companies.
    
    pub ranks: Option<Vec<Rank>>,
    /// The list of Google Partners specialization statuses for the company.
    #[serde(rename="specializationStatus")]
    
    pub specialization_status: Option<Vec<SpecializationStatus>>,
    /// Partner badge tier
    #[serde(rename="badgeTier")]
    
    pub badge_tier: Option<String>,
    /// Email domains that allow users with a matching email address to get
    /// auto-approved for associating with this company.
    #[serde(rename="autoApprovalEmailDomains")]
    
    pub auto_approval_email_domains: Option<Vec<String>>,
    /// Company type labels listed on the company's profile.
    #[serde(rename="companyTypes")]
    
    pub company_types: Option<Vec<String>>,
}

impl client::RequestValue for Company {}
impl client::ResponseResult for Company {}


/// Response message for CreateLead.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [leads create companies](CompanyLeadCreateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateLeadResponse {
    /// Lead that was created depending on the outcome of
    /// <a href="https://www.google.com/recaptcha/">reCaptcha</a> validation.
    
    pub lead: Option<Lead>,
    /// The outcome of <a href="https://www.google.com/recaptcha/">reCaptcha</a>
    /// validation.
    #[serde(rename="recaptchaStatus")]
    
    pub recaptcha_status: Option<String>,
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    
    pub response_metadata: Option<ResponseMetadata>,
}

impl client::ResponseResult for CreateLeadResponse {}


/// Response message for GetCompany.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get companies](CompanyGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetCompanyResponse {
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    
    pub response_metadata: Option<ResponseMetadata>,
    /// The company.
    
    pub company: Option<Company>,
}

impl client::ResponseResult for GetCompanyResponse {}


/// A location with address and geographic coordinates. May optionally contain a
/// detailed (multi-field) version of the address.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// Top-level administrative subdivision of this country.
    #[serde(rename="administrativeArea")]
    
    pub administrative_area: Option<String>,
    /// Generally refers to the city/town portion of an address.
    
    pub locality: Option<String>,
    /// The latitude and longitude of the location, in degrees.
    #[serde(rename="latLng")]
    
    pub lat_lng: Option<LatLng>,
    /// CLDR (Common Locale Data Repository) region code .
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
    /// Dependent locality or sublocality. Used for UK dependent localities, or
    /// neighborhoods or boroughs in other locations.
    #[serde(rename="dependentLocality")]
    
    pub dependent_locality: Option<String>,
    /// The single string version of the address.
    
    pub address: Option<String>,
    /// Values are frequently alphanumeric.
    #[serde(rename="postalCode")]
    
    pub postal_code: Option<String>,
    /// Use of this code is very country-specific, but will refer to a secondary
    /// classification code for sorting mail.
    #[serde(rename="sortingCode")]
    
    pub sorting_code: Option<String>,
    /// Language code of the address. Should be in BCP 47 format.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// The following address lines represent the most specific part of any
    /// address.
    #[serde(rename="addressLine")]
    
    pub address_line: Option<Vec<String>>,
}

impl client::Part for Location {}


/// Status for a Google Partners certification exam.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CertificationExamStatus {
    /// The number of people who have passed the certification exam.
    #[serde(rename="numberUsersPass")]
    
    pub number_users_pass: Option<i32>,
    /// The type of certification exam.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for CertificationExamStatus {}


/// A set of opt-ins for a user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OptIns {
    /// An opt-in about receiving email from Partners marketing teams. Includes
    /// member-only events and special promotional offers for Google products.
    #[serde(rename="marketComm")]
    
    pub market_comm: Option<bool>,
    /// An opt-in about receiving email regarding new features and products.
    #[serde(rename="specialOffers")]
    
    pub special_offers: Option<bool>,
    /// An opt-in about receiving email with customized AdWords campaign management
    /// tips.
    #[serde(rename="performanceSuggestions")]
    
    pub performance_suggestions: Option<bool>,
    /// An opt-in to receive special promotional gifts and material in the mail.
    #[serde(rename="physicalMail")]
    
    pub physical_mail: Option<bool>,
    /// An opt-in to allow recieivng phone calls about their Partners account.
    #[serde(rename="phoneContact")]
    
    pub phone_contact: Option<bool>,
}

impl client::Part for OptIns {}


/// Information related to ranking of results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Rank {
    /// The numerical value of the rank.
    
    pub value: Option<f64>,
    /// The type of rank.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Rank {}


/// Response message for
/// GetPartnersStatus.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get partnersstatus](MethodGetPartnersstatuCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetPartnersStatusResponse {
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    
    pub response_metadata: Option<ResponseMetadata>,
}

impl client::ResponseResult for GetPartnersStatusResponse {}


/// The profile information of a Partners user.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update profile users](UserUpdateProfileCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserProfile {
    /// A list of ids representing which channels the user selected they were in.
    
    pub channels: Option<Vec<String>>,
    /// Whether the user's public profile is visible to anyone with the URL.
    #[serde(rename="profilePublic")]
    
    pub profile_public: Option<bool>,
    /// A list of ids represnting which job categories the user selected.
    #[serde(rename="jobFunctions")]
    
    pub job_functions: Option<Vec<String>>,
    /// The user's given name.
    #[serde(rename="givenName")]
    
    pub given_name: Option<String>,
    /// The user's mailing address, contains multiple fields.
    
    pub address: Option<Location>,
    /// A list of ids representing which industries the user selected.
    
    pub industries: Option<Vec<String>>,
    /// The list of opt-ins for the user, related to communication preferences.
    #[serde(rename="emailOptIns")]
    
    pub email_opt_ins: Option<OptIns>,
    /// The user's family name.
    #[serde(rename="familyName")]
    
    pub family_name: Option<String>,
    /// The list of languages this user understands.
    
    pub languages: Option<Vec<String>>,
    /// A list of ids representing which markets the user was interested in.
    
    pub markets: Option<Vec<String>>,
    /// Whether or not to migrate the user's exam data to Academy for Ads.
    #[serde(rename="migrateToAfa")]
    
    pub migrate_to_afa: Option<bool>,
    /// If the user has edit access to multiple accounts, the user can choose the
    /// preferred account and it is used when a personal account is needed. Can
    /// be empty.
    #[serde(rename="adwordsManagerAccount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub adwords_manager_account: Option<i64>,
    /// The user's phone number.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
    /// The user's primary country, an ISO 2-character code.
    #[serde(rename="primaryCountryCode")]
    
    pub primary_country_code: Option<String>,
    /// The email address the user has selected on the Partners site as primary.
    #[serde(rename="emailAddress")]
    
    pub email_address: Option<String>,
}

impl client::RequestValue for UserProfile {}
impl client::ResponseResult for UserProfile {}


/// Historical information about a Google Partners Offer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HistoricalOffer {
    /// Time offer was first created.
    #[serde(rename="creationTime")]
    
    pub creation_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Status of the offer.
    
    pub status: Option<String>,
    /// Email address for client.
    #[serde(rename="clientEmail")]
    
    pub client_email: Option<String>,
    /// ID of client.
    #[serde(rename="clientId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub client_id: Option<i64>,
    /// Name of the client.
    #[serde(rename="clientName")]
    
    pub client_name: Option<String>,
    /// Time last action was taken.
    #[serde(rename="lastModifiedTime")]
    
    pub last_modified_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Client's AdWords page URL.
    #[serde(rename="adwordsUrl")]
    
    pub adwords_url: Option<String>,
    /// Type of offer.
    #[serde(rename="offerType")]
    
    pub offer_type: Option<String>,
    /// Name (First + Last) of the partners user to whom the incentive is allocated.
    #[serde(rename="senderName")]
    
    pub sender_name: Option<String>,
    /// Country Code for the offer country.
    #[serde(rename="offerCountryCode")]
    
    pub offer_country_code: Option<String>,
    /// Time this offer expires.
    #[serde(rename="expirationTime")]
    
    pub expiration_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Offer code.
    #[serde(rename="offerCode")]
    
    pub offer_code: Option<String>,
}

impl client::Part for HistoricalOffer {}


/// Request message for
/// LogUserEvent.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [log user events](UserEventLogCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogUserEventRequest {
    /// The URL where the event occurred.
    
    pub url: Option<String>,
    /// Current request metadata.
    #[serde(rename="requestMetadata")]
    
    pub request_metadata: Option<RequestMetadata>,
    /// List of event data for the event.
    #[serde(rename="eventDatas")]
    
    pub event_datas: Option<Vec<EventData>>,
    /// The scope of the event.
    #[serde(rename="eventScope")]
    
    pub event_scope: Option<String>,
    /// The category the action belongs to.
    #[serde(rename="eventCategory")]
    
    pub event_category: Option<String>,
    /// Advertiser lead information.
    
    pub lead: Option<Lead>,
    /// The action that occurred.
    #[serde(rename="eventAction")]
    
    pub event_action: Option<String>,
}

impl client::RequestValue for LogUserEventRequest {}


/// Values to use instead of the user's respective defaults. These are only
/// honored by whitelisted products.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserOverrides {
    /// IP address to use instead of the user's geo-located IP address.
    #[serde(rename="ipAddress")]
    
    pub ip_address: Option<String>,
    /// Logged-in user ID to impersonate instead of the user's ID.
    #[serde(rename="userId")]
    
    pub user_id: Option<String>,
}

impl client::Part for UserOverrides {}


/// Details of the analytics events for a `Company` within a single day.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyticsDataPoint {
    /// Location information of where these events occurred.
    #[serde(rename="eventLocations")]
    
    pub event_locations: Option<Vec<LatLng>>,
    /// Number of times the type of event occurred.
    /// Meaning depends on context (e.g. profile views, contacts, etc.).
    #[serde(rename="eventCount")]
    
    pub event_count: Option<i32>,
}

impl client::Part for AnalyticsDataPoint {}


/// Analytics data for a `Company` within a single day.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Analytics {
    /// Date on which these events occurred.
    #[serde(rename="eventDate")]
    
    pub event_date: Option<Date>,
    /// Instances of users viewing the `Company` profile
    /// on the specified date.
    #[serde(rename="profileViews")]
    
    pub profile_views: Option<AnalyticsDataPoint>,
    /// Instances of users seeing the `Company` in Google Partners Search results
    /// on the specified date.
    #[serde(rename="searchViews")]
    
    pub search_views: Option<AnalyticsDataPoint>,
    /// Instances of users contacting the `Company`
    /// on the specified date.
    
    pub contacts: Option<AnalyticsDataPoint>,
}

impl client::Part for Analytics {}


/// Information about a particular AdWords Manager Account.
/// Read more at https://support.google.com/adwords/answer/6139186
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdWordsManagerAccountInfo {
    /// The AdWords Manager Account id.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Name of the customer this account represents.
    #[serde(rename="customerName")]
    
    pub customer_name: Option<String>,
}

impl client::Part for AdWordsManagerAccountInfo {}


/// Basic information from a public profile.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PublicProfile {
    /// The URL to the main profile image of the public profile.
    #[serde(rename="profileImage")]
    
    pub profile_image: Option<String>,
    /// The display name of the public profile.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The URL to the main display image of the public profile. Being deprecated.
    #[serde(rename="displayImageUrl")]
    
    pub display_image_url: Option<String>,
    /// The ID which can be used to retrieve more details about the public profile.
    
    pub id: Option<String>,
    /// The URL of the public profile.
    
    pub url: Option<String>,
}

impl client::Part for PublicProfile {}


/// Common data that is in each API response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResponseMetadata {
    /// Debug information about this request.
    #[serde(rename="debugInfo")]
    
    pub debug_info: Option<DebugInfo>,
}

impl client::Part for ResponseMetadata {}


/// <a href="https://www.google.com/recaptcha/">reCaptcha</a> challenge info.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RecaptchaChallenge {
    /// The ID of the reCaptcha challenge.
    
    pub id: Option<String>,
    /// The response to the reCaptcha challenge.
    
    pub response: Option<String>,
}

impl client::Part for RecaptchaChallenge {}


/// Available Offers to be distributed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AvailableOffer {
    /// Level of this offer.
    #[serde(rename="offerLevel")]
    
    pub offer_level: Option<String>,
    /// Name of the offer.
    
    pub name: Option<String>,
    /// ID of this offer.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Whether or not the list of qualified customers is definitely complete.
    #[serde(rename="qualifiedCustomersComplete")]
    
    pub qualified_customers_complete: Option<bool>,
    /// Offer info by country.
    #[serde(rename="countryOfferInfos")]
    
    pub country_offer_infos: Option<Vec<CountryOfferInfo>>,
    /// Type of offer.
    #[serde(rename="offerType")]
    
    pub offer_type: Option<String>,
    /// The maximum age of an account [in days] to be eligible.
    #[serde(rename="maxAccountAge")]
    
    pub max_account_age: Option<i32>,
    /// Customers who qualify for this offer.
    #[serde(rename="qualifiedCustomer")]
    
    pub qualified_customer: Option<Vec<OfferCustomer>>,
    /// Terms of the offer.
    
    pub terms: Option<String>,
    /// Should special text be shown on the offers page.
    #[serde(rename="showSpecialOfferCopy")]
    
    pub show_special_offer_copy: Option<bool>,
    /// The number of codes for this offer that are available for distribution.
    
    pub available: Option<i32>,
    /// Description of the offer.
    
    pub description: Option<String>,
}

impl client::Part for AvailableOffer {}


/// An object representing a latitude/longitude pair. This is expressed as a pair
/// of doubles representing degrees latitude and degrees longitude. Unless
/// specified otherwise, this must conform to the
/// <a href="http://www.unoosa.org/pdf/icg/2012/template/WGS_84.pdf">WGS84
/// standard</a>. Values must be within normalized ranges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LatLng {
    /// The latitude in degrees. It must be in the range [-90.0, +90.0].
    
    pub latitude: Option<f64>,
    /// The longitude in degrees. It must be in the range [-180.0, +180.0].
    
    pub longitude: Option<f64>,
}

impl client::Part for LatLng {}


/// Represents an amount of money with its currency type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Money {
    /// The 3-letter currency code defined in ISO 4217.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Number of nano (10^-9) units of the amount.
    /// The value must be between -999,999,999 and +999,999,999 inclusive.
    /// If `units` is positive, `nanos` must be positive or zero.
    /// If `units` is zero, `nanos` can be positive, zero, or negative.
    /// If `units` is negative, `nanos` must be negative or zero.
    /// For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    
    pub nanos: Option<i32>,
    /// The whole units of the amount.
    /// For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub units: Option<i64>,
}

impl client::Part for Money {}


/// Analytics aggregated data for a `Company` for a given date range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyticsSummary {
    /// Aggregated number of profile views for the `Company` for given date range.
    #[serde(rename="profileViewsCount")]
    
    pub profile_views_count: Option<i32>,
    /// Aggregated number of times users saw the `Company`
    /// in Google Partners Search results for given date range.
    #[serde(rename="searchViewsCount")]
    
    pub search_views_count: Option<i32>,
    /// Aggregated number of times users contacted the `Company`
    /// for given date range.
    #[serde(rename="contactsCount")]
    
    pub contacts_count: Option<i32>,
}

impl client::Part for AnalyticsSummary {}


/// Request message for
/// LogClientMessage.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [log client messages](ClientMessageLogCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogMessageRequest {
    /// Map of client info, such as URL, browser navigator, browser platform, etc.
    #[serde(rename="clientInfo")]
    
    pub client_info: Option<HashMap<String, String>>,
    /// Current request metadata.
    #[serde(rename="requestMetadata")]
    
    pub request_metadata: Option<RequestMetadata>,
    /// Message level of client message.
    
    pub level: Option<String>,
    /// Details about the client message.
    
    pub details: Option<String>,
}

impl client::RequestValue for LogMessageRequest {}


/// A lead resource that represents an advertiser contact for a `Company`. These
/// are usually generated via Google Partner Search (the advertiser portal).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list leads](LeadListCall) (none)
/// * [update leads](MethodUpdateLeadCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Lead {
    /// The minimum monthly budget lead source is willing to spend.
    #[serde(rename="minMonthlyBudget")]
    
    pub min_monthly_budget: Option<Money>,
    /// First name of lead source.
    #[serde(rename="givenName")]
    
    pub given_name: Option<String>,
    /// Language code of the lead's language preference, as defined by
    /// <a href="https://tools.ietf.org/html/bcp47">BCP 47</a>
    /// (IETF BCP 47, "Tags for Identifying Languages").
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Website URL of lead source.
    #[serde(rename="websiteUrl")]
    
    pub website_url: Option<String>,
    /// The lead's state in relation to the company.
    
    pub state: Option<String>,
    /// List of reasons for using Google Partner Search and creating a lead.
    #[serde(rename="gpsMotivations")]
    
    pub gps_motivations: Option<Vec<String>>,
    /// Email address of lead source.
    
    pub email: Option<String>,
    /// Last name of lead source.
    #[serde(rename="familyName")]
    
    pub family_name: Option<String>,
    /// ID of the lead.
    
    pub id: Option<String>,
    /// Comments lead source gave.
    
    pub comments: Option<String>,
    /// Phone number of lead source.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
    /// The AdWords Customer ID of the lead.
    #[serde(rename="adwordsCustomerId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub adwords_customer_id: Option<i64>,
    /// Timestamp of when this lead was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Whether or not the lead signed up for marketing emails
    #[serde(rename="marketingOptIn")]
    
    pub marketing_opt_in: Option<bool>,
    /// Type of lead.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::RequestValue for Lead {}
impl client::Resource for Lead {}
impl client::ResponseResult for Lead {}


/// Debug information about this request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DebugInfo {
    /// Info about the server that serviced this request.
    #[serde(rename="serverInfo")]
    
    pub server_info: Option<String>,
    /// Server-side debug stack trace.
    #[serde(rename="serverTraceInfo")]
    
    pub server_trace_info: Option<String>,
    /// URL of the service that handled this request.
    #[serde(rename="serviceUrl")]
    
    pub service_url: Option<String>,
}

impl client::Part for DebugInfo {}


/// Response message for
/// ListUserStates.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list user states](UserStateListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListUserStatesResponse {
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    
    pub response_metadata: Option<ResponseMetadata>,
    /// User's states.
    #[serde(rename="userStates")]
    
    pub user_states: Option<Vec<String>>,
}

impl client::ResponseResult for ListUserStatesResponse {}


/// A CompanyRelation resource representing information about a user’s
/// affiliation and standing with a company in Partners.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create company relation users](UserCreateCompanyRelationCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompanyRelation {
    /// Indicates if the user is an admin for this company.
    #[serde(rename="companyAdmin")]
    
    pub company_admin: Option<bool>,
    /// The primary address for this company.
    
    pub address: Option<String>,
    /// The flag that indicates if the company is pending verification.
    #[serde(rename="isPending")]
    
    pub is_pending: Option<bool>,
    /// The timestamp of when affiliation was requested.
    /// @OutputOnly
    #[serde(rename="creationTime")]
    
    pub creation_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The primary location of the company.
    #[serde(rename="primaryAddress")]
    
    pub primary_address: Option<Location>,
    /// The state of relationship, in terms of approvals.
    
    pub state: Option<String>,
    /// The name (in the company's primary language) for the company.
    
    pub name: Option<String>,
    /// The AdWords manager account # associated this company.
    #[serde(rename="managerAccount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub manager_account: Option<i64>,
    /// The segment the company is classified as.
    
    pub segment: Option<Vec<String>>,
    /// The internal company ID.
    /// Only available for a whitelisted set of api clients.
    #[serde(rename="internalCompanyId")]
    
    pub internal_company_id: Option<String>,
    /// Whether the company is a Partner.
    #[serde(rename="badgeTier")]
    
    pub badge_tier: Option<String>,
    /// The list of Google Partners specialization statuses for the company.
    #[serde(rename="specializationStatus")]
    
    pub specialization_status: Option<Vec<SpecializationStatus>>,
    /// The phone number for the company's primary address.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
    /// The website URL for this company.
    
    pub website: Option<String>,
    /// The primary country code of the company.
    #[serde(rename="primaryCountryCode")]
    
    pub primary_country_code: Option<String>,
    /// The ID of the company. There may be no id if this is a
    /// pending company.5
    #[serde(rename="companyId")]
    
    pub company_id: Option<String>,
    /// The primary language code of the company.
    #[serde(rename="primaryLanguageCode")]
    
    pub primary_language_code: Option<String>,
    /// A URL to a profile photo, e.g. a G+ profile photo.
    #[serde(rename="logoUrl")]
    
    pub logo_url: Option<String>,
    /// The timestamp when the user was approved.
    /// @OutputOnly
    #[serde(rename="resolvedTimestamp")]
    
    pub resolved_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for CompanyRelation {}
impl client::ResponseResult for CompanyRelation {}


/// Represents a whole or partial calendar date, e.g. a birthday. The time of day
/// and time zone are either specified elsewhere or are not significant. The date
/// is relative to the Proleptic Gregorian Calendar. This can represent:
/// 
/// * A full date, with non-zero year, month and day values
/// * A month and day value, with a zero year, e.g. an anniversary
/// * A year on its own, with zero month and day values
/// * A year and month value, with a zero day, e.g. a credit card expiration date
/// 
/// Related types are google.type.TimeOfDay and `google.protobuf.Timestamp`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Date {
    /// Year of date. Must be from 1 to 9999, or 0 if specifying a date without
    /// a year.
    
    pub year: Option<i32>,
    /// Day of month. Must be from 1 to 31 and valid for the year and month, or 0
    /// if specifying a year by itself or a year and month where the day is not
    /// significant.
    
    pub day: Option<i32>,
    /// Month of year. Must be from 1 to 12, or 0 if specifying a year without a
    /// month and day.
    
    pub month: Option<i32>,
}

impl client::Part for Date {}


