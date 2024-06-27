use std::collections::HashMap;
use std::cell::RefCell;
use std::default::Default;
use std::collections::BTreeSet;
use std::error::Error as StdError;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;

use hyper::client::connect;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::time::sleep;
use tower_service;
use serde::{Serialize, Deserialize};

use crate::{client, client::GetToken, client::serde_with};

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Clone, Copy)]
pub enum Scope {
    /// See, edit, create, and delete your Google Ads accounts and data.
    Adword,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Adword => "https://www.googleapis.com/auth/adwords",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Adword
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Localservices related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_localservices1 as localservices1;
/// use localservices1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use localservices1::{Localservices, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Localservices::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.account_reports().search()
///              .start_date_year(-17)
///              .start_date_month(-55)
///              .start_date_day(-88)
///              .query("amet")
///              .page_token("duo")
///              .page_size(-50)
///              .end_date_year(-93)
///              .end_date_month(-37)
///              .end_date_day(-12)
///              .doit().await;
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::Io(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
#[derive(Clone)]
pub struct Localservices<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, S> client::Hub for Localservices<S> {}

impl<'a, S> Localservices<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> Localservices<S> {
        Localservices {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.5".to_string(),
            _base_url: "https://localservices.googleapis.com/".to_string(),
            _root_url: "https://localservices.googleapis.com/".to_string(),
        }
    }

    pub fn account_reports(&'a self) -> AccountReportMethods<'a, S> {
        AccountReportMethods { hub: &self }
    }
    pub fn detailed_lead_reports(&'a self) -> DetailedLeadReportMethods<'a, S> {
        DetailedLeadReportMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.5`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://localservices.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://localservices.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// An Account Report of a GLS account identified by their account id containing aggregate data gathered from a particular date range. Next ID: 18
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
    
    pub charge_status: Option<String>,
    /// Currency code.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Dispute status related to the lead.
    #[serde(rename="disputeStatus")]
    
    pub dispute_status: Option<String>,
    /// Location of the associated account's home city.
    
    pub geo: Option<String>,
    /// Unique identifier of a Detailed Lead Report.
    #[serde(rename="googleAdsLeadId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub google_ads_lead_id: Option<i64>,
    /// Lead category (e.g. hvac, plumber)
    #[serde(rename="leadCategory")]
    
    pub lead_category: Option<String>,
    /// Timestamp of when the lead was created.
    #[serde(rename="leadCreationTimestamp")]
    
    pub lead_creation_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Deprecated in favor of google_ads_lead_id. Unique identifier of a Detailed Lead Report.
    #[serde(rename="leadId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub lead_id: Option<i64>,
    /// Price of the lead (available only after it has been charged).
    #[serde(rename="leadPrice")]
    
    pub lead_price: Option<f64>,
    /// Lead type.
    #[serde(rename="leadType")]
    
    pub lead_type: Option<String>,
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeTimeZone {
    /// IANA Time Zone Database time zone, e.g. "America/New_York".
    
    pub id: Option<String>,
    /// Optional. IANA Time Zone Database version number, e.g. "2019a".
    
    pub version: Option<String>,
}

impl client::Part for GoogleTypeTimeZone {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *accountReport* resources.
/// It is not used directly, but through the [`Localservices`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_localservices1 as localservices1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use localservices1::{Localservices, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Localservices::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `search(...)`
/// // to build up your call.
/// let rb = hub.account_reports();
/// # }
/// ```
pub struct AccountReportMethods<'a, S>
    where S: 'a {

    hub: &'a Localservices<S>,
}

impl<'a, S> client::MethodsBuilder for AccountReportMethods<'a, S> {}

impl<'a, S> AccountReportMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get account reports containing aggregate account data of all linked GLS accounts. Caller needs to provide their manager customer id and the associated auth credential that allows them read permissions on their linked accounts.
    pub fn search(&self) -> AccountReportSearchCall<'a, S> {
        AccountReportSearchCall {
            hub: self.hub,
            _start_date_year: Default::default(),
            _start_date_month: Default::default(),
            _start_date_day: Default::default(),
            _query: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _end_date_year: Default::default(),
            _end_date_month: Default::default(),
            _end_date_day: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *detailedLeadReport* resources.
/// It is not used directly, but through the [`Localservices`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_localservices1 as localservices1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use localservices1::{Localservices, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Localservices::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `search(...)`
/// // to build up your call.
/// let rb = hub.detailed_lead_reports();
/// # }
/// ```
pub struct DetailedLeadReportMethods<'a, S>
    where S: 'a {

    hub: &'a Localservices<S>,
}

impl<'a, S> client::MethodsBuilder for DetailedLeadReportMethods<'a, S> {}

impl<'a, S> DetailedLeadReportMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get detailed lead reports containing leads that have been received by all linked GLS accounts. Caller needs to provide their manager customer id and the associated auth credential that allows them read permissions on their linked accounts.
    pub fn search(&self) -> DetailedLeadReportSearchCall<'a, S> {
        DetailedLeadReportSearchCall {
            hub: self.hub,
            _start_date_year: Default::default(),
            _start_date_month: Default::default(),
            _start_date_day: Default::default(),
            _query: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _end_date_year: Default::default(),
            _end_date_month: Default::default(),
            _end_date_day: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Get account reports containing aggregate account data of all linked GLS accounts. Caller needs to provide their manager customer id and the associated auth credential that allows them read permissions on their linked accounts.
///
/// A builder for the *search* method supported by a *accountReport* resource.
/// It is not used directly, but through a [`AccountReportMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_localservices1 as localservices1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use localservices1::{Localservices, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Localservices::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.account_reports().search()
///              .start_date_year(-16)
///              .start_date_month(-57)
///              .start_date_day(-50)
///              .query("ipsum")
///              .page_token("est")
///              .page_size(-62)
///              .end_date_year(-17)
///              .end_date_month(-99)
///              .end_date_day(-56)
///              .doit().await;
/// # }
/// ```
pub struct AccountReportSearchCall<'a, S>
    where S: 'a {

    hub: &'a Localservices<S>,
    _start_date_year: Option<i32>,
    _start_date_month: Option<i32>,
    _start_date_day: Option<i32>,
    _query: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _end_date_year: Option<i32>,
    _end_date_month: Option<i32>,
    _end_date_day: Option<i32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for AccountReportSearchCall<'a, S> {}

impl<'a, S> AccountReportSearchCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleAdsHomeservicesLocalservicesV1SearchAccountReportsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "localservices.accountReports.search",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "startDate.year", "startDate.month", "startDate.day", "query", "pageToken", "pageSize", "endDate.year", "endDate.month", "endDate.day"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(11 + self._additional_params.len());
        if let Some(value) = self._start_date_year.as_ref() {
            params.push("startDate.year", value.to_string());
        }
        if let Some(value) = self._start_date_month.as_ref() {
            params.push("startDate.month", value.to_string());
        }
        if let Some(value) = self._start_date_day.as_ref() {
            params.push("startDate.day", value.to_string());
        }
        if let Some(value) = self._query.as_ref() {
            params.push("query", value);
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._end_date_year.as_ref() {
            params.push("endDate.year", value.to_string());
        }
        if let Some(value) = self._end_date_month.as_ref() {
            params.push("endDate.month", value.to_string());
        }
        if let Some(value) = self._end_date_day.as_ref() {
            params.push("endDate.day", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/accountReports:search";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Adword.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    ///
    /// Sets the *start date.year* query property to the given value.
    pub fn start_date_year(mut self, new_value: i32) -> AccountReportSearchCall<'a, S> {
        self._start_date_year = Some(new_value);
        self
    }
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    ///
    /// Sets the *start date.month* query property to the given value.
    pub fn start_date_month(mut self, new_value: i32) -> AccountReportSearchCall<'a, S> {
        self._start_date_month = Some(new_value);
        self
    }
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    ///
    /// Sets the *start date.day* query property to the given value.
    pub fn start_date_day(mut self, new_value: i32) -> AccountReportSearchCall<'a, S> {
        self._start_date_day = Some(new_value);
        self
    }
    /// A query string for searching for account reports. Caller must provide a customer id of their MCC account with an associated Gaia Mint that allows read permission on their linked accounts. Search expressions are case insensitive. Example query: | Query | Description | |-------------------------|-----------------------------------------------| | manager_customer_id:123 | Get Account Report for Manager with id 123. | Required.
    ///
    /// Sets the *query* query property to the given value.
    pub fn query(mut self, new_value: &str) -> AccountReportSearchCall<'a, S> {
        self._query = Some(new_value.to_string());
        self
    }
    /// The `next_page_token` value returned from a previous request to SearchAccountReports that indicates where listing should continue. Optional.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AccountReportSearchCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of accounts to return. If the page size is unset, page size will default to 1000. Maximum page_size is 10000. Optional.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AccountReportSearchCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    ///
    /// Sets the *end date.year* query property to the given value.
    pub fn end_date_year(mut self, new_value: i32) -> AccountReportSearchCall<'a, S> {
        self._end_date_year = Some(new_value);
        self
    }
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    ///
    /// Sets the *end date.month* query property to the given value.
    pub fn end_date_month(mut self, new_value: i32) -> AccountReportSearchCall<'a, S> {
        self._end_date_month = Some(new_value);
        self
    }
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    ///
    /// Sets the *end date.day* query property to the given value.
    pub fn end_date_day(mut self, new_value: i32) -> AccountReportSearchCall<'a, S> {
        self._end_date_day = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountReportSearchCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> AccountReportSearchCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Adword`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> AccountReportSearchCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> AccountReportSearchCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> AccountReportSearchCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Get detailed lead reports containing leads that have been received by all linked GLS accounts. Caller needs to provide their manager customer id and the associated auth credential that allows them read permissions on their linked accounts.
///
/// A builder for the *search* method supported by a *detailedLeadReport* resource.
/// It is not used directly, but through a [`DetailedLeadReportMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_localservices1 as localservices1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use localservices1::{Localservices, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Localservices::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.detailed_lead_reports().search()
///              .start_date_year(-25)
///              .start_date_month(-86)
///              .start_date_day(-43)
///              .query("duo")
///              .page_token("sed")
///              .page_size(-61)
///              .end_date_year(-15)
///              .end_date_month(-13)
///              .end_date_day(-24)
///              .doit().await;
/// # }
/// ```
pub struct DetailedLeadReportSearchCall<'a, S>
    where S: 'a {

    hub: &'a Localservices<S>,
    _start_date_year: Option<i32>,
    _start_date_month: Option<i32>,
    _start_date_day: Option<i32>,
    _query: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _end_date_year: Option<i32>,
    _end_date_month: Option<i32>,
    _end_date_day: Option<i32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for DetailedLeadReportSearchCall<'a, S> {}

impl<'a, S> DetailedLeadReportSearchCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleAdsHomeservicesLocalservicesV1SearchDetailedLeadReportsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "localservices.detailedLeadReports.search",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "startDate.year", "startDate.month", "startDate.day", "query", "pageToken", "pageSize", "endDate.year", "endDate.month", "endDate.day"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(11 + self._additional_params.len());
        if let Some(value) = self._start_date_year.as_ref() {
            params.push("startDate.year", value.to_string());
        }
        if let Some(value) = self._start_date_month.as_ref() {
            params.push("startDate.month", value.to_string());
        }
        if let Some(value) = self._start_date_day.as_ref() {
            params.push("startDate.day", value.to_string());
        }
        if let Some(value) = self._query.as_ref() {
            params.push("query", value);
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._end_date_year.as_ref() {
            params.push("endDate.year", value.to_string());
        }
        if let Some(value) = self._end_date_month.as_ref() {
            params.push("endDate.month", value.to_string());
        }
        if let Some(value) = self._end_date_day.as_ref() {
            params.push("endDate.day", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/detailedLeadReports:search";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Adword.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    ///
    /// Sets the *start date.year* query property to the given value.
    pub fn start_date_year(mut self, new_value: i32) -> DetailedLeadReportSearchCall<'a, S> {
        self._start_date_year = Some(new_value);
        self
    }
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    ///
    /// Sets the *start date.month* query property to the given value.
    pub fn start_date_month(mut self, new_value: i32) -> DetailedLeadReportSearchCall<'a, S> {
        self._start_date_month = Some(new_value);
        self
    }
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    ///
    /// Sets the *start date.day* query property to the given value.
    pub fn start_date_day(mut self, new_value: i32) -> DetailedLeadReportSearchCall<'a, S> {
        self._start_date_day = Some(new_value);
        self
    }
    /// A query string for searching for account reports. Caller must provide a customer id of their MCC account with an associated Gaia Mint that allows read permission on their linked accounts. Search expressions are case insensitive. Example query: | Query | Description | |-------------------------|-----------------------------------------------| | manager_customer_id:123 | Get Detailed Lead Report for Manager with id | | | 123. | Required.
    ///
    /// Sets the *query* query property to the given value.
    pub fn query(mut self, new_value: &str) -> DetailedLeadReportSearchCall<'a, S> {
        self._query = Some(new_value.to_string());
        self
    }
    /// The `next_page_token` value returned from a previous request to SearchDetailedLeadReports that indicates where listing should continue. Optional.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> DetailedLeadReportSearchCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of accounts to return. If the page size is unset, page size will default to 1000. Maximum page_size is 10000. Optional.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> DetailedLeadReportSearchCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    ///
    /// Sets the *end date.year* query property to the given value.
    pub fn end_date_year(mut self, new_value: i32) -> DetailedLeadReportSearchCall<'a, S> {
        self._end_date_year = Some(new_value);
        self
    }
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    ///
    /// Sets the *end date.month* query property to the given value.
    pub fn end_date_month(mut self, new_value: i32) -> DetailedLeadReportSearchCall<'a, S> {
        self._end_date_month = Some(new_value);
        self
    }
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    ///
    /// Sets the *end date.day* query property to the given value.
    pub fn end_date_day(mut self, new_value: i32) -> DetailedLeadReportSearchCall<'a, S> {
        self._end_date_day = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DetailedLeadReportSearchCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> DetailedLeadReportSearchCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Adword`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> DetailedLeadReportSearchCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> DetailedLeadReportSearchCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> DetailedLeadReportSearchCall<'a, S> {
        self._scopes.clear();
        self
    }
}


