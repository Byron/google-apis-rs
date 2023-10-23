use super::*;
/// An Account Report of a GLS account identified by their account id containing aggregate data gathered from a particular date range. Next ID: 18
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAdsHomeservicesLocalservicesV1AccountReport {
    /// Unique identifier of the GLS account.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// Aggregator specific information related to the account.
    #[serde(rename="aggregatorInfo")]
    
    pub aggregator_info: Option<GoogleAdsHomeservicesLocalservicesV1AggregatorInfo>,
    /// Average review rating score from 1-5 stars.
    #[serde(rename="averageFiveStarRating")]
    
    pub average_five_star_rating: Option<f64>,
    /// Average weekly budget in the currency code of the account.
    #[serde(rename="averageWeeklyBudget")]
    
    pub average_weekly_budget: Option<f64>,
    /// Business name of the account.
    #[serde(rename="businessName")]
    
    pub business_name: Option<String>,
    /// Currency code of the account.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Number of charged leads the account received in current specified period.
    #[serde(rename="currentPeriodChargedLeads")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub current_period_charged_leads: Option<i64>,
    /// Number of connected phone calls (duration over 30s) in current specified period.
    #[serde(rename="currentPeriodConnectedPhoneCalls")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub current_period_connected_phone_calls: Option<i64>,
    /// Number of phone calls in current specified period, including both connected and unconnected calls.
    #[serde(rename="currentPeriodPhoneCalls")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub current_period_phone_calls: Option<i64>,
    /// Total cost of the account in current specified period in the account's specified currency.
    #[serde(rename="currentPeriodTotalCost")]
    
    pub current_period_total_cost: Option<f64>,
    /// Number of impressions that customers have had in the past 2 days.
    #[serde(rename="impressionsLastTwoDays")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub impressions_last_two_days: Option<i64>,
    /// Phone lead responsiveness of the account for the past 90 days from current date. This is computed by taking the total number of connected calls from charged phone leads and dividing by the total number of calls received.
    #[serde(rename="phoneLeadResponsiveness")]
    
    pub phone_lead_responsiveness: Option<f64>,
    /// Number of charged leads the account received in previous specified period.
    #[serde(rename="previousPeriodChargedLeads")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub previous_period_charged_leads: Option<i64>,
    /// Number of connected phone calls (duration over 30s) in previous specified period.
    #[serde(rename="previousPeriodConnectedPhoneCalls")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub previous_period_connected_phone_calls: Option<i64>,
    /// Number of phone calls in previous specified period, including both connected and unconnected calls.
    #[serde(rename="previousPeriodPhoneCalls")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub previous_period_phone_calls: Option<i64>,
    /// Total cost of the account in previous specified period in the account's specified currency.
    #[serde(rename="previousPeriodTotalCost")]
    
    pub previous_period_total_cost: Option<f64>,
    /// Total number of reviews the account has up to current date.
    #[serde(rename="totalReview")]
    
    pub total_review: Option<i32>,
}

impl client::Part for GoogleAdsHomeservicesLocalservicesV1AccountReport {}


/// Conatiner for aggregator specific information if lead is for an aggregator GLS account.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAdsHomeservicesLocalservicesV1AggregatorInfo {
    /// Provider id (listed in aggregator system) which maps to a account id in GLS system.
    #[serde(rename="aggregatorProviderId")]
    
    pub aggregator_provider_id: Option<String>,
}

impl client::Part for GoogleAdsHomeservicesLocalservicesV1AggregatorInfo {}


/// Container for booking lead specific information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAdsHomeservicesLocalservicesV1BookingLead {
    /// Timestamp of when service is provided by advertiser.
    #[serde(rename="bookingAppointmentTimestamp")]
    
    pub booking_appointment_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Consumer email associated with the booking lead.
    #[serde(rename="consumerEmail")]
    
    pub consumer_email: Option<String>,
    /// Consumer phone number associated with the booking lead.
    #[serde(rename="consumerPhoneNumber")]
    
    pub consumer_phone_number: Option<String>,
    /// Name of the customer who created the lead.
    #[serde(rename="customerName")]
    
    pub customer_name: Option<String>,
    /// The job type of the specified lead.
    #[serde(rename="jobType")]
    
    pub job_type: Option<String>,
}

impl client::Part for GoogleAdsHomeservicesLocalservicesV1BookingLead {}


/// A Detailed Lead Report of a lead identified by their lead id and contains consumer, account, monetization, and lead data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAdsHomeservicesLocalservicesV1DetailedLeadReport {
    /// Identifies account that received the lead.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// Aggregator specific information related to the lead.
    #[serde(rename="aggregatorInfo")]
    
    pub aggregator_info: Option<GoogleAdsHomeservicesLocalservicesV1AggregatorInfo>,
    /// More information associated to only booking leads.
    #[serde(rename="bookingLead")]
    
    pub booking_lead: Option<GoogleAdsHomeservicesLocalservicesV1BookingLead>,
    /// Business name associated to the account.
    #[serde(rename="businessName")]
    
    pub business_name: Option<String>,
    /// Whether the lead has been charged.
    #[serde(rename="chargeStatus")]
    
    pub charge_status: Option<GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatusEnum>,
    /// Currency code.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Dispute status related to the lead.
    #[serde(rename="disputeStatus")]
    
    pub dispute_status: Option<String>,
    /// Location of the associated account's home city.
    
    pub geo: Option<String>,
    /// Lead category (e.g. hvac, plumber)
    #[serde(rename="leadCategory")]
    
    pub lead_category: Option<String>,
    /// Timestamp of when the lead was created.
    #[serde(rename="leadCreationTimestamp")]
    
    pub lead_creation_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Unique identifier of a Detailed Lead Report.
    #[serde(rename="leadId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub lead_id: Option<i64>,
    /// Price of the lead (available only after it has been charged).
    #[serde(rename="leadPrice")]
    
    pub lead_price: Option<f64>,
    /// Lead type.
    #[serde(rename="leadType")]
    
    pub lead_type: Option<GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadTypeEnum>,
    /// More information associated to only message leads.
    #[serde(rename="messageLead")]
    
    pub message_lead: Option<GoogleAdsHomeservicesLocalservicesV1MessageLead>,
    /// More information associated to only phone leads.
    #[serde(rename="phoneLead")]
    
    pub phone_lead: Option<GoogleAdsHomeservicesLocalservicesV1PhoneLead>,
    /// Timezone of the particular provider associated to a lead.
    
    pub timezone: Option<GoogleTypeTimeZone>,
}

impl client::Part for GoogleAdsHomeservicesLocalservicesV1DetailedLeadReport {}


/// Container for message lead specific information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAdsHomeservicesLocalservicesV1MessageLead {
    /// Consumer phone number associated with the message lead.
    #[serde(rename="consumerPhoneNumber")]
    
    pub consumer_phone_number: Option<String>,
    /// Name of the customer who created the lead.
    #[serde(rename="customerName")]
    
    pub customer_name: Option<String>,
    /// The job type of the specified lead.
    #[serde(rename="jobType")]
    
    pub job_type: Option<String>,
    /// The postal code of the customer who created the lead.
    #[serde(rename="postalCode")]
    
    pub postal_code: Option<String>,
}

impl client::Part for GoogleAdsHomeservicesLocalservicesV1MessageLead {}


/// Container for phone lead specific information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAdsHomeservicesLocalservicesV1PhoneLead {
    /// Timestamp of the phone call which resulted in a charged phone lead.
    #[serde(rename="chargedCallTimestamp")]
    
    pub charged_call_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Duration of the charged phone call in seconds.
    #[serde(rename="chargedConnectedCallDurationSeconds")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub charged_connected_call_duration_seconds: Option<client::chrono::Duration>,
    /// Consumer phone number associated with the phone lead.
    #[serde(rename="consumerPhoneNumber")]
    
    pub consumer_phone_number: Option<String>,
}

impl client::Part for GoogleAdsHomeservicesLocalservicesV1PhoneLead {}


/// A page of the response received from the SearchAccountReports method. A paginated response where more pages are available has `next_page_token` set. This token can be used in a subsequent request to retrieve the next request page.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search account reports](AccountReportSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAdsHomeservicesLocalservicesV1SearchAccountReportsResponse {
    /// List of account reports which maps 1:1 to a particular linked GLS account.
    #[serde(rename="accountReports")]
    
    pub account_reports: Option<Vec<GoogleAdsHomeservicesLocalservicesV1AccountReport>>,
    /// Pagination token to retrieve the next page of results. When `next_page_token` is not filled in, there is no next page and the list returned is the last page in the result set.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleAdsHomeservicesLocalservicesV1SearchAccountReportsResponse {}


/// A page of the response received from the SearchDetailedLeadReports method. A paginated response where more pages are available has `next_page_token` set. This token can be used in a subsequent request to retrieve the next request page.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search detailed lead reports](DetailedLeadReportSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleAdsHomeservicesLocalservicesV1SearchDetailedLeadReportsResponse {
    /// List of detailed lead reports uniquely identified by external lead id.
    #[serde(rename="detailedLeadReports")]
    
    pub detailed_lead_reports: Option<Vec<GoogleAdsHomeservicesLocalservicesV1DetailedLeadReport>>,
    /// Pagination token to retrieve the next page of results. When `next_page_token` is not filled in, there is no next page and the list returned is the last page in the result set.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleAdsHomeservicesLocalservicesV1SearchDetailedLeadReportsResponse {}


/// Represents a time zone from the [IANA Time Zone Database](https://www.iana.org/time-zones).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeTimeZone {
    /// IANA Time Zone Database time zone, e.g. "America/New_York".
    
    pub id: Option<String>,
    /// Optional. IANA Time Zone Database version number, e.g. "2019a".
    
    pub version: Option<String>,
}

impl client::Part for GoogleTypeTimeZone {}


