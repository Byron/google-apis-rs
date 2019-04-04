// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Partners* crate version *1.0.8+20180925*, where *20180925* is the exact revision of the *partners:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.8*.
//! 
//! Everything else about the *Partners* *v2* API can be found at the
//! [official documentation site](https://developers.google.com/partners/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/partners2).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Partners.html) ... 
//! 
//! * analytics
//!  * [*list*](struct.AnalyticListCall.html)
//! * client messages
//!  * [*log*](struct.ClientMessageLogCall.html)
//! * [companies](struct.Company.html)
//!  * [*get*](struct.CompanyGetCall.html), [*leads create*](struct.CompanyLeadCreateCall.html) and [*list*](struct.CompanyListCall.html)
//! * [leads](struct.Lead.html)
//!  * [*list*](struct.LeadListCall.html)
//! * offers
//!  * [*history list*](struct.OfferHistoryListCall.html) and [*list*](struct.OfferListCall.html)
//! * user events
//!  * [*log*](struct.UserEventLogCall.html)
//! * user states
//!  * [*list*](struct.UserStateListCall.html)
//! * [users](struct.User.html)
//!  * [*create company relation*](struct.UserCreateCompanyRelationCall.html), [*delete company relation*](struct.UserDeleteCompanyRelationCall.html), [*get*](struct.UserGetCall.html) and [*update profile*](struct.UserUpdateProfileCall.html)
//! 
//! Other activities are ...
//! 
//! * [get partnersstatus](struct.MethodGetPartnersstatuCall.html)
//! * [update companies](struct.MethodUpdateCompanyCall.html)
//! * [update leads](struct.MethodUpdateLeadCall.html)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Partners.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](trait.MethodsBuilder.html) which in turn
//!       allow access to individual [*Call Builders*](trait.CallBuilder.html)
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit()
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.users().create_company_relation(...).doit()
//! let r = hub.users().get(...).doit()
//! let r = hub.users().update_profile(...).doit()
//! let r = hub.users().delete_company_relation(...).doit()
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-partners2 = "*"
//! # This project intentionally uses an old version of Hyper. See
//! # https://github.com/Byron/google-apis-rs/issues/173 for more
//! # information.
//! hyper = "^0.10"
//! hyper-rustls = "^0.6"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! yup-oauth2 = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_partners2 as partners2;
//! use partners2::CompanyRelation;
//! use partners2::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use partners2::Partners;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
//!                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = CompanyRelation::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.users().create_company_relation(req, "userId")
//!              .request_metadata_user_overrides_user_id("justo")
//!              .request_metadata_user_overrides_ip_address("amet.")
//!              .request_metadata_traffic_source_traffic_sub_id("erat")
//!              .request_metadata_traffic_source_traffic_source_id("labore")
//!              .request_metadata_partners_session_id("sea")
//!              .request_metadata_locale("nonumy")
//!              .add_request_metadata_experiment_ids("dolores")
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](enum.Result.html), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](trait.ResponseResult.html), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](trait.Delegate.html) to the 
//! [Method Builder](trait.CallBuilder.html) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [enocodable](trait.RequestValue.html) and 
//! [decodable](trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;
use std::time::Duration;

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, Error, CallBuilder, Hub, ReadSeek, Part,
              ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, MethodsBuilder,
              Resource, ErrorResponse, remove_json_null_values};


// ##############
// UTILITIES ###
// ############




// ########
// HUB ###
// ######

/// Central instance to access all Partners related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_partners2 as partners2;
/// use partners2::CompanyRelation;
/// use partners2::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use partners2::Partners;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CompanyRelation::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().create_company_relation(req, "userId")
///              .request_metadata_user_overrides_user_id("sadipscing")
///              .request_metadata_user_overrides_ip_address("aliquyam")
///              .request_metadata_traffic_source_traffic_sub_id("ea")
///              .request_metadata_traffic_source_traffic_source_id("no")
///              .request_metadata_partners_session_id("justo")
///              .request_metadata_locale("justo")
///              .add_request_metadata_experiment_ids("et")
///              .doit();
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
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
pub struct Partners<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for Partners<C, A> {}

impl<'a, C, A> Partners<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Partners<C, A> {
        Partners {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.8".to_string(),
            _base_url: "https://partners.googleapis.com/".to_string(),
            _root_url: "https://partners.googleapis.com/".to_string(),
        }
    }

    pub fn analytics(&'a self) -> AnalyticMethods<'a, C, A> {
        AnalyticMethods { hub: &self }
    }
    pub fn client_messages(&'a self) -> ClientMessageMethods<'a, C, A> {
        ClientMessageMethods { hub: &self }
    }
    pub fn companies(&'a self) -> CompanyMethods<'a, C, A> {
        CompanyMethods { hub: &self }
    }
    pub fn leads(&'a self) -> LeadMethods<'a, C, A> {
        LeadMethods { hub: &self }
    }
    pub fn methods(&'a self) -> MethodMethods<'a, C, A> {
        MethodMethods { hub: &self }
    }
    pub fn offers(&'a self) -> OfferMethods<'a, C, A> {
        OfferMethods { hub: &self }
    }
    pub fn user_events(&'a self) -> UserEventMethods<'a, C, A> {
        UserEventMethods { hub: &self }
    }
    pub fn user_states(&'a self) -> UserStateMethods<'a, C, A> {
        UserStateMethods { hub: &self }
    }
    pub fn users(&'a self) -> UserMethods<'a, C, A> {
        UserMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.8`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://partners.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://partners.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Response message for
/// LogClientMessage.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [log client messages](struct.ClientMessageLogCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogMessageResponse {
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    pub response_metadata: Option<ResponseMetadata>,
}

impl ResponseResult for LogMessageResponse {}


/// Values to use instead of the user's respective defaults. These are only
/// honored by whitelisted products.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserOverrides {
    /// IP address to use instead of the user's geo-located IP address.
    #[serde(rename="ipAddress")]
    pub ip_address: Option<String>,
    /// Logged-in user ID to impersonate instead of the user's ID.
    #[serde(rename="userId")]
    pub user_id: Option<String>,
}

impl Part for UserOverrides {}


/// A lead resource that represents an advertiser contact for a `Company`. These
/// are usually generated via Google Partner Search (the advertiser portal).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list leads](struct.LeadListCall.html) (none)
/// * [update leads](struct.MethodUpdateLeadCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Lead {
    /// Whether or not the lead signed up for marketing emails
    #[serde(rename="marketingOptIn")]
    pub marketing_opt_in: Option<bool>,
    /// Language code of the lead's language preference, as defined by
    /// <a href="https://tools.ietf.org/html/bcp47">BCP 47</a>
    /// (IETF BCP 47, "Tags for Identifying Languages").
    #[serde(rename="languageCode")]
    pub language_code: Option<String>,
    /// List of reasons for using Google Partner Search and creating a lead.
    #[serde(rename="gpsMotivations")]
    pub gps_motivations: Option<Vec<String>>,
    /// ID of the lead.
    pub id: Option<String>,
    /// Last name of lead source.
    #[serde(rename="familyName")]
    pub family_name: Option<String>,
    /// The minimum monthly budget lead source is willing to spend.
    #[serde(rename="minMonthlyBudget")]
    pub min_monthly_budget: Option<Money>,
    /// Comments lead source gave.
    pub comments: Option<String>,
    /// Email address of lead source.
    pub email: Option<String>,
    /// Website URL of lead source.
    #[serde(rename="websiteUrl")]
    pub website_url: Option<String>,
    /// The lead's state in relation to the company.
    pub state: Option<String>,
    /// Phone number of lead source.
    #[serde(rename="phoneNumber")]
    pub phone_number: Option<String>,
    /// The AdWords Customer ID of the lead.
    #[serde(rename="adwordsCustomerId")]
    pub adwords_customer_id: Option<String>,
    /// First name of lead source.
    #[serde(rename="givenName")]
    pub given_name: Option<String>,
    /// Type of lead.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Timestamp of when this lead was created.
    #[serde(rename="createTime")]
    pub create_time: Option<String>,
}

impl RequestValue for Lead {}
impl Resource for Lead {}
impl ResponseResult for Lead {}


/// Debug information about this request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DebugInfo {
    /// Server-side debug stack trace.
    #[serde(rename="serverTraceInfo")]
    pub server_trace_info: Option<String>,
    /// URL of the service that handled this request.
    #[serde(rename="serviceUrl")]
    pub service_url: Option<String>,
    /// Info about the server that serviced this request.
    #[serde(rename="serverInfo")]
    pub server_info: Option<String>,
}

impl Part for DebugInfo {}


/// Available Offers to be distributed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AvailableOffer {
    /// The number of codes for this offer that are available for distribution.
    pub available: Option<i32>,
    /// Offer info by country.
    #[serde(rename="countryOfferInfos")]
    pub country_offer_infos: Option<Vec<CountryOfferInfo>>,
    /// Customers who qualify for this offer.
    #[serde(rename="qualifiedCustomer")]
    pub qualified_customer: Option<Vec<OfferCustomer>>,
    /// Terms of the offer.
    pub terms: Option<String>,
    /// Name of the offer.
    pub name: Option<String>,
    /// The maximum age of an account [in days] to be eligible.
    #[serde(rename="maxAccountAge")]
    pub max_account_age: Option<i32>,
    /// Level of this offer.
    #[serde(rename="offerLevel")]
    pub offer_level: Option<String>,
    /// Should special text be shown on the offers page.
    #[serde(rename="showSpecialOfferCopy")]
    pub show_special_offer_copy: Option<bool>,
    /// Type of offer.
    #[serde(rename="offerType")]
    pub offer_type: Option<String>,
    /// ID of this offer.
    pub id: Option<String>,
    /// Whether or not the list of qualified customers is definitely complete.
    #[serde(rename="qualifiedCustomersComplete")]
    pub qualified_customers_complete: Option<bool>,
    /// Description of the offer.
    pub description: Option<String>,
}

impl Part for AvailableOffer {}


/// Response for ListOffer.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list offers](struct.OfferListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOffersResponse {
    /// Reason why no Offers are available.
    #[serde(rename="noOfferReason")]
    pub no_offer_reason: Option<String>,
    /// Available Offers to be distributed.
    #[serde(rename="availableOffers")]
    pub available_offers: Option<Vec<AvailableOffer>>,
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    pub response_metadata: Option<ResponseMetadata>,
}

impl ResponseResult for ListOffersResponse {}


/// Response message for ListLeads.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list leads](struct.LeadListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLeadsResponse {
    /// A token to retrieve next page of results.
    /// Pass this value in the `ListLeadsRequest.page_token` field in the
    /// subsequent call to
    /// ListLeads to retrieve the
    /// next page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The total count of leads for the given company.
    #[serde(rename="totalSize")]
    pub total_size: Option<i32>,
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    pub response_metadata: Option<ResponseMetadata>,
    /// The list of leads.
    pub leads: Option<Vec<Lead>>,
}

impl ResponseResult for ListLeadsResponse {}


/// The localized company information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocalizedCompanyInfo {
    /// List of country codes for the localized company info.
    #[serde(rename="countryCodes")]
    pub country_codes: Option<Vec<String>>,
    /// Language code of the localized company info, as defined by
    /// <a href="https://tools.ietf.org/html/bcp47">BCP 47</a>
    /// (IETF BCP 47, "Tags for Identifying Languages").
    #[serde(rename="languageCode")]
    pub language_code: Option<String>,
    /// Localized display name.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// Localized brief description that the company uses to advertise themselves.
    pub overview: Option<String>,
}

impl Part for LocalizedCompanyInfo {}


/// A location with address and geographic coordinates. May optionally contain a
/// detailed (multi-field) version of the address.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// Language code of the address. Should be in BCP 47 format.
    #[serde(rename="languageCode")]
    pub language_code: Option<String>,
    /// The following address lines represent the most specific part of any
    /// address.
    #[serde(rename="addressLine")]
    pub address_line: Option<Vec<String>>,
    /// Generally refers to the city/town portion of an address.
    pub locality: Option<String>,
    /// Use of this code is very country-specific, but will refer to a secondary
    /// classification code for sorting mail.
    #[serde(rename="sortingCode")]
    pub sorting_code: Option<String>,
    /// The latitude and longitude of the location, in degrees.
    #[serde(rename="latLng")]
    pub lat_lng: Option<LatLng>,
    /// Dependent locality or sublocality. Used for UK dependent localities, or
    /// neighborhoods or boroughs in other locations.
    #[serde(rename="dependentLocality")]
    pub dependent_locality: Option<String>,
    /// CLDR (Common Locale Data Repository) region code .
    #[serde(rename="regionCode")]
    pub region_code: Option<String>,
    /// Top-level administrative subdivision of this country.
    #[serde(rename="administrativeArea")]
    pub administrative_area: Option<String>,
    /// The single string version of the address.
    pub address: Option<String>,
    /// Values are frequently alphanumeric.
    #[serde(rename="postalCode")]
    pub postal_code: Option<String>,
}

impl Part for Location {}


/// Response message for
/// ListCompanies.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list companies](struct.CompanyListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCompaniesResponse {
    /// A token to retrieve next page of results.
    /// Pass this value in the `ListCompaniesRequest.page_token` field in the
    /// subsequent call to
    /// ListCompanies to retrieve the
    /// next page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The list of companies.
    pub companies: Option<Vec<Company>>,
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    pub response_metadata: Option<ResponseMetadata>,
}

impl ResponseResult for ListCompaniesResponse {}


/// Key value data pair for an event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventData {
    /// Data values.
    pub values: Option<Vec<String>>,
    /// Data type.
    pub key: Option<String>,
}

impl Part for EventData {}


/// A generic empty message that you can re-use to avoid defining duplicated
/// empty messages in your APIs. A typical example is to use it as the request
/// or the response type of an API method. For instance:
/// 
///     service Foo {
///       rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);
///     }
/// 
/// The JSON representation for `Empty` is empty JSON object `{}`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete company relation users](struct.UserDeleteCompanyRelationCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl ResponseResult for Empty {}


/// A set of opt-ins for a user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OptIns {
    /// An opt-in about receiving email regarding new features and products.
    #[serde(rename="specialOffers")]
    pub special_offers: Option<bool>,
    /// An opt-in about receiving email from Partners marketing teams. Includes
    /// member-only events and special promotional offers for Google products.
    #[serde(rename="marketComm")]
    pub market_comm: Option<bool>,
    /// An opt-in to allow recieivng phone calls about their Partners account.
    #[serde(rename="phoneContact")]
    pub phone_contact: Option<bool>,
    /// An opt-in to receive special promotional gifts and material in the mail.
    #[serde(rename="physicalMail")]
    pub physical_mail: Option<bool>,
    /// An opt-in about receiving email with customized AdWords campaign management
    /// tips.
    #[serde(rename="performanceSuggestions")]
    pub performance_suggestions: Option<bool>,
}

impl Part for OptIns {}


/// Details of the analytics events for a `Company` within a single day.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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

impl Part for AnalyticsDataPoint {}


/// Response message for
/// ListAnalytics.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list analytics](struct.AnalyticListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAnalyticsResponse {
    /// A token to retrieve next page of results.
    /// Pass this value in the `ListAnalyticsRequest.page_token` field in the
    /// subsequent call to
    /// ListAnalytics to retrieve the
    /// next page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Aggregated information across the response's
    /// analytics.
    #[serde(rename="analyticsSummary")]
    pub analytics_summary: Option<AnalyticsSummary>,
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    pub response_metadata: Option<ResponseMetadata>,
    /// The list of analytics.
    /// Sorted in ascending order of
    /// Analytics.event_date.
    pub analytics: Option<Vec<Analytics>>,
}

impl ResponseResult for ListAnalyticsResponse {}


/// Agency specialization status
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpecializationStatus {
    /// The specialization this status is for.
    #[serde(rename="badgeSpecialization")]
    pub badge_specialization: Option<String>,
    /// State of agency specialization.
    #[serde(rename="badgeSpecializationState")]
    pub badge_specialization_state: Option<String>,
}

impl Part for SpecializationStatus {}


/// A company resource in the Google Partners API. Once certified, it qualifies
/// for being searched by advertisers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update companies](struct.MethodUpdateCompanyCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Company {
    /// Company type labels listed on the company's profile.
    #[serde(rename="companyTypes")]
    pub company_types: Option<Vec<String>>,
    /// The unconverted minimum monthly budget that the company accepts for partner
    /// business.
    #[serde(rename="originalMinMonthlyBudget")]
    pub original_min_monthly_budget: Option<Money>,
    /// The list of Google Partners specialization statuses for the company.
    #[serde(rename="specializationStatus")]
    pub specialization_status: Option<Vec<SpecializationStatus>>,
    /// The public viewability status of the company's profile.
    #[serde(rename="profileStatus")]
    pub profile_status: Option<String>,
    /// The list of all company locations.
    /// If set, must include the
    /// primary_location
    /// in the list.
    pub locations: Option<Vec<Location>>,
    /// Services the company can help with.
    pub services: Option<Vec<String>>,
    /// URL of the company's additional websites used to verify the dynamic badges.
    /// These are stored as full URLs as entered by the user, but only the TLD will
    /// be used for the actual verification.
    #[serde(rename="additionalWebsites")]
    pub additional_websites: Option<Vec<String>>,
    /// The primary location of the company.
    #[serde(rename="primaryLocation")]
    pub primary_location: Option<Location>,
    /// Basic information from the company's public profile.
    #[serde(rename="publicProfile")]
    pub public_profile: Option<PublicProfile>,
    /// The ID of the company.
    pub id: Option<String>,
    /// Industries the company can help with.
    pub industries: Option<Vec<String>>,
    /// URL of the company's website.
    #[serde(rename="websiteUrl")]
    pub website_url: Option<String>,
    /// Email domains that allow users with a matching email address to get
    /// auto-approved for associating with this company.
    #[serde(rename="autoApprovalEmailDomains")]
    pub auto_approval_email_domains: Option<Vec<String>>,
    /// The name of the company.
    pub name: Option<String>,
    /// The list of localized info for the company.
    #[serde(rename="localizedInfos")]
    pub localized_infos: Option<Vec<LocalizedCompanyInfo>>,
    /// Partner badge tier
    #[serde(rename="badgeTier")]
    pub badge_tier: Option<String>,
    /// Information related to the ranking of the company within the list of
    /// companies.
    pub ranks: Option<Vec<Rank>>,
    /// The Primary AdWords Manager Account id.
    #[serde(rename="primaryAdwordsManagerAccountId")]
    pub primary_adwords_manager_account_id: Option<String>,
    /// The primary language code of the company, as defined by
    /// <a href="https://tools.ietf.org/html/bcp47">BCP 47</a>
    /// (IETF BCP 47, "Tags for Identifying Languages").
    #[serde(rename="primaryLanguageCode")]
    pub primary_language_code: Option<String>,
    /// The list of Google Partners certification statuses for the company.
    #[serde(rename="certificationStatuses")]
    pub certification_statuses: Option<Vec<CertificationStatus>>,
    /// Whether the company's badge authority is in AWN
    #[serde(rename="badgeAuthorityInAwn")]
    pub badge_authority_in_awn: Option<bool>,
    /// The minimum monthly budget that the company accepts for partner business,
    /// converted to the requested currency code.
    #[serde(rename="convertedMinMonthlyBudget")]
    pub converted_min_monthly_budget: Option<Money>,
}

impl RequestValue for Company {}
impl ResponseResult for Company {}


/// Request message for
/// LogClientMessage.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [log client messages](struct.ClientMessageLogCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogMessageRequest {
    /// Map of client info, such as URL, browser navigator, browser platform, etc.
    #[serde(rename="clientInfo")]
    pub client_info: Option<HashMap<String, String>>,
    /// Current request metadata.
    #[serde(rename="requestMetadata")]
    pub request_metadata: Option<RequestMetadata>,
    /// Details about the client message.
    pub details: Option<String>,
    /// Message level of client message.
    pub level: Option<String>,
}

impl RequestValue for LogMessageRequest {}


/// Information about a particular AdWords Manager Account.
/// Read more at https://support.google.com/adwords/answer/6139186
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdWordsManagerAccountInfo {
    /// The AdWords Manager Account id.
    pub id: Option<String>,
    /// Name of the customer this account represents.
    #[serde(rename="customerName")]
    pub customer_name: Option<String>,
}

impl Part for AdWordsManagerAccountInfo {}


/// Response for ListOfferHistory.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [history list offers](struct.OfferHistoryListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOffersHistoryResponse {
    /// Supply this token in a ListOffersHistoryRequest to retrieve the next page.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// True if this response is showing entire company history.
    #[serde(rename="showingEntireCompany")]
    pub showing_entire_company: Option<bool>,
    /// True if the user has the option to show entire company history.
    #[serde(rename="canShowEntireCompany")]
    pub can_show_entire_company: Option<bool>,
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    pub response_metadata: Option<ResponseMetadata>,
    /// Historical offers meeting request.
    pub offers: Option<Vec<HistoricalOffer>>,
    /// Number of results across all pages.
    #[serde(rename="totalResults")]
    pub total_results: Option<i32>,
}

impl ResponseResult for ListOffersHistoryResponse {}


/// Offer info by country.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CountryOfferInfo {
    /// (localized) Spend X amount for that country's offer.
    #[serde(rename="spendXAmount")]
    pub spend_x_amount: Option<String>,
    /// Country code for which offer codes may be requested.
    #[serde(rename="offerCountryCode")]
    pub offer_country_code: Option<i64>,
    /// Type of offer country is eligible for.
    #[serde(rename="offerType")]
    pub offer_type: Option<String>,
    /// (localized) Get Y amount for that country's offer.
    #[serde(rename="getYAmount")]
    pub get_y_amount: Option<String>,
}

impl Part for CountryOfferInfo {}


/// Response message for GetCompany.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get companies](struct.CompanyGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetCompanyResponse {
    /// The company.
    pub company: Option<Company>,
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    pub response_metadata: Option<ResponseMetadata>,
}

impl ResponseResult for GetCompanyResponse {}


/// Common data that is in each API request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RequestMetadata {
    /// Locale to use for the current request.
    pub locale: Option<String>,
    /// Google Partners session ID.
    #[serde(rename="partnersSessionId")]
    pub partners_session_id: Option<String>,
    /// Experiment IDs the current request belongs to.
    #[serde(rename="experimentIds")]
    pub experiment_ids: Option<Vec<String>>,
    /// Values to use instead of the user's respective defaults for the current
    /// request. These are only honored by whitelisted products.
    #[serde(rename="userOverrides")]
    pub user_overrides: Option<UserOverrides>,
    /// Source of traffic for the current request.
    #[serde(rename="trafficSource")]
    pub traffic_source: Option<TrafficSource>,
}

impl Part for RequestMetadata {}


/// A resource representing a user of the Partners platform.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create company relation users](struct.UserCreateCompanyRelationCall.html) (none)
/// * [get users](struct.UserGetCall.html) (response)
/// * [update profile users](struct.UserUpdateProfileCall.html) (none)
/// * [delete company relation users](struct.UserDeleteCompanyRelationCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// The profile information of a Partners user, contains all the directly
    /// editable user information.
    pub profile: Option<UserProfile>,
    /// The email address used by the user used for company verification.
    /// @OutputOnly
    #[serde(rename="companyVerificationEmail")]
    pub company_verification_email: Option<String>,
    /// The list of achieved certifications. These are calculated based on exam
    /// results and other requirements.
    /// @OutputOnly
    #[serde(rename="certificationStatus")]
    pub certification_status: Option<Vec<Certification>>,
    /// The internal user ID.
    /// Only available for a whitelisted set of api clients.
    #[serde(rename="internalId")]
    pub internal_id: Option<String>,
    /// This is the list of AdWords Manager Accounts the user has edit access to.
    /// If the user has edit access to multiple accounts, the user can choose the
    /// preferred account and we use this when a personal account is needed. Can
    /// be empty meaning the user has access to no accounts.
    /// @OutputOnly
    #[serde(rename="availableAdwordsManagerAccounts")]
    pub available_adwords_manager_accounts: Option<Vec<AdWordsManagerAccountInfo>>,
    /// The company that the user is associated with.
    /// If not present, the user is not associated with any company.
    pub company: Option<CompanyRelation>,
    /// The most recent time the user interacted with the Partners site.
    /// @OutputOnly
    #[serde(rename="lastAccessTime")]
    pub last_access_time: Option<String>,
    /// The list of exams the user ever taken. For each type of exam, only one
    /// entry is listed.
    #[serde(rename="examStatus")]
    pub exam_status: Option<Vec<ExamStatus>>,
    /// The list of emails the user has access to/can select as primary.
    /// @OutputOnly
    #[serde(rename="primaryEmails")]
    pub primary_emails: Option<Vec<String>>,
    /// Information about a user's external public profile outside Google Partners.
    #[serde(rename="publicProfile")]
    pub public_profile: Option<PublicProfile>,
    /// The ID of the user.
    pub id: Option<String>,
    /// Whether or not the user has opted to share their Academy for Ads info with
    /// Google Partners.
    #[serde(rename="afaInfoShared")]
    pub afa_info_shared: Option<bool>,
}

impl Resource for User {}
impl ResponseResult for User {}


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

impl Part for Date {}


/// The profile information of a Partners user.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update profile users](struct.UserUpdateProfileCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserProfile {
    /// A list of ids representing which industries the user selected.
    pub industries: Option<Vec<String>>,
    /// The user's primary country, an ISO 2-character code.
    #[serde(rename="primaryCountryCode")]
    pub primary_country_code: Option<i64>,
    /// If the user has edit access to multiple accounts, the user can choose the
    /// preferred account and it is used when a personal account is needed. Can
    /// be empty.
    #[serde(rename="adwordsManagerAccount")]
    pub adwords_manager_account: Option<String>,
    /// A list of ids represnting which job categories the user selected.
    #[serde(rename="jobFunctions")]
    pub job_functions: Option<Vec<String>>,
    /// The list of opt-ins for the user, related to communication preferences.
    #[serde(rename="emailOptIns")]
    pub email_opt_ins: Option<OptIns>,
    /// The user's family name.
    #[serde(rename="familyName")]
    pub family_name: Option<String>,
    /// Whether or not to migrate the user's exam data to Academy for Ads.
    #[serde(rename="migrateToAfa")]
    pub migrate_to_afa: Option<bool>,
    /// The list of languages this user understands.
    pub languages: Option<Vec<String>>,
    /// The user's phone number.
    #[serde(rename="phoneNumber")]
    pub phone_number: Option<String>,
    /// The user's mailing address, contains multiple fields.
    pub address: Option<Location>,
    /// The email address the user has selected on the Partners site as primary.
    #[serde(rename="emailAddress")]
    pub email_address: Option<String>,
    /// The user's given name.
    #[serde(rename="givenName")]
    pub given_name: Option<String>,
    /// A list of ids representing which markets the user was interested in.
    pub markets: Option<Vec<String>>,
    /// Whether the user's public profile is visible to anyone with the URL.
    #[serde(rename="profilePublic")]
    pub profile_public: Option<bool>,
    /// A list of ids representing which channels the user selected they were in.
    pub channels: Option<Vec<String>>,
}

impl RequestValue for UserProfile {}
impl ResponseResult for UserProfile {}


/// Response message for
/// GetPartnersStatus.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get partnersstatus](struct.MethodGetPartnersstatuCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetPartnersStatusResponse {
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    pub response_metadata: Option<ResponseMetadata>,
}

impl ResponseResult for GetPartnersStatusResponse {}


/// Response message for
/// LogUserEvent.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [log user events](struct.UserEventLogCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogUserEventResponse {
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    pub response_metadata: Option<ResponseMetadata>,
}

impl ResponseResult for LogUserEventResponse {}


/// Source of traffic for the current request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TrafficSource {
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    #[serde(rename="trafficSubId")]
    pub traffic_sub_id: Option<String>,
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    #[serde(rename="trafficSourceId")]
    pub traffic_source_id: Option<String>,
}

impl Part for TrafficSource {}


/// Google Partners certification status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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

impl Part for CertificationStatus {}


/// Common data that is in each API response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResponseMetadata {
    /// Debug information about this request.
    #[serde(rename="debugInfo")]
    pub debug_info: Option<DebugInfo>,
}

impl Part for ResponseMetadata {}


/// Analytics aggregated data for a `Company` for a given date range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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

impl Part for AnalyticsSummary {}


/// Request message for
/// LogUserEvent.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [log user events](struct.UserEventLogCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogUserEventRequest {
    /// Current request metadata.
    #[serde(rename="requestMetadata")]
    pub request_metadata: Option<RequestMetadata>,
    /// Advertiser lead information.
    pub lead: Option<Lead>,
    /// The URL where the event occurred.
    pub url: Option<String>,
    /// List of event data for the event.
    #[serde(rename="eventDatas")]
    pub event_datas: Option<Vec<EventData>>,
    /// The category the action belongs to.
    #[serde(rename="eventCategory")]
    pub event_category: Option<String>,
    /// The scope of the event.
    #[serde(rename="eventScope")]
    pub event_scope: Option<String>,
    /// The action that occurred.
    #[serde(rename="eventAction")]
    pub event_action: Option<String>,
}

impl RequestValue for LogUserEventRequest {}


/// Customers qualified for an offer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OfferCustomer {
    /// Name of the customer.
    pub name: Option<String>,
    /// Country code of the customer.
    #[serde(rename="countryCode")]
    pub country_code: Option<String>,
    /// Formatted Spend X amount with currency code.
    #[serde(rename="spendXAmount")]
    pub spend_x_amount: Option<String>,
    /// URL to the customer's AdWords page.
    #[serde(rename="adwordsUrl")]
    pub adwords_url: Option<String>,
    /// Time the customer was created.
    #[serde(rename="creationTime")]
    pub creation_time: Option<String>,
    /// External CID for the customer.
    #[serde(rename="externalCid")]
    pub external_cid: Option<String>,
    /// Days the customer is still eligible.
    #[serde(rename="eligibilityDaysLeft")]
    pub eligibility_days_left: Option<i32>,
    /// Formatted Get Y amount with currency code.
    #[serde(rename="getYAmount")]
    pub get_y_amount: Option<String>,
    /// Type of the offer
    #[serde(rename="offerType")]
    pub offer_type: Option<String>,
}

impl Part for OfferCustomer {}


/// <a href="https://www.google.com/recaptcha/">reCaptcha</a> challenge info.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RecaptchaChallenge {
    /// The ID of the reCaptcha challenge.
    pub id: Option<String>,
    /// The response to the reCaptcha challenge.
    pub response: Option<String>,
}

impl Part for RecaptchaChallenge {}


/// A user's information on a specific exam.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExamStatus {
    /// The type of the exam.
    #[serde(rename="examType")]
    pub exam_type: Option<String>,
    /// Whether this exam is in the state of warning.
    pub warning: Option<bool>,
    /// Date this exam is due to expire.
    pub expiration: Option<String>,
    /// Whether this exam has been passed and not expired.
    pub passed: Option<bool>,
    /// The date the user last taken this exam.
    pub taken: Option<String>,
    /// The date the user last passed this exam.
    #[serde(rename="lastPassed")]
    pub last_passed: Option<String>,
}

impl Part for ExamStatus {}


/// Status for a Google Partners certification exam.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CertificationExamStatus {
    /// The number of people who have passed the certification exam.
    #[serde(rename="numberUsersPass")]
    pub number_users_pass: Option<i32>,
    /// The type of certification exam.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for CertificationExamStatus {}


/// Response message for CreateLead.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [leads create companies](struct.CompanyLeadCreateCall.html) (response)
/// 
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

impl ResponseResult for CreateLeadResponse {}


/// Basic information from a public profile.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PublicProfile {
    /// The URL of the public profile.
    pub url: Option<String>,
    /// The URL to the main profile image of the public profile.
    #[serde(rename="profileImage")]
    pub profile_image: Option<String>,
    /// The URL to the main display image of the public profile. Being deprecated.
    #[serde(rename="displayImageUrl")]
    pub display_image_url: Option<String>,
    /// The display name of the public profile.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// The ID which can be used to retrieve more details about the public profile.
    pub id: Option<String>,
}

impl Part for PublicProfile {}


/// A user's information on a specific certification.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Certification {
    /// Whether this certification has been achieved.
    pub achieved: Option<bool>,
    /// The date the user last achieved certification.
    #[serde(rename="lastAchieved")]
    pub last_achieved: Option<String>,
    /// Whether this certification is in the state of warning.
    pub warning: Option<bool>,
    /// Date this certification is due to expire.
    pub expiration: Option<String>,
    /// The type of certification, the area of expertise.
    #[serde(rename="certificationType")]
    pub certification_type: Option<String>,
}

impl Part for Certification {}


/// Represents an amount of money with its currency type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Money {
    /// Number of nano (10^-9) units of the amount.
    /// The value must be between -999,999,999 and +999,999,999 inclusive.
    /// If `units` is positive, `nanos` must be positive or zero.
    /// If `units` is zero, `nanos` can be positive, zero, or negative.
    /// If `units` is negative, `nanos` must be negative or zero.
    /// For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    pub nanos: Option<i32>,
    /// The whole units of the amount.
    /// For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    pub units: Option<String>,
    /// The 3-letter currency code defined in ISO 4217.
    #[serde(rename="currencyCode")]
    pub currency_code: Option<String>,
}

impl Part for Money {}


/// An object representing a latitude/longitude pair. This is expressed as a pair
/// of doubles representing degrees latitude and degrees longitude. Unless
/// specified otherwise, this must conform to the
/// <a href="http://www.unoosa.org/pdf/icg/2012/template/WGS_84.pdf">WGS84
/// standard</a>. Values must be within normalized ranges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LatLng {
    /// The latitude in degrees. It must be in the range [-90.0, +90.0].
    pub latitude: Option<f64>,
    /// The longitude in degrees. It must be in the range [-180.0, +180.0].
    pub longitude: Option<f64>,
}

impl Part for LatLng {}


/// Information related to ranking of results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Rank {
    /// The type of rank.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The numerical value of the rank.
    pub value: Option<f64>,
}

impl Part for Rank {}


/// Analytics data for a `Company` within a single day.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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

impl Part for Analytics {}


/// Request message for CreateLead.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [leads create companies](struct.CompanyLeadCreateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateLeadRequest {
    /// Current request metadata.
    #[serde(rename="requestMetadata")]
    pub request_metadata: Option<RequestMetadata>,
    /// <a href="https://www.google.com/recaptcha/">reCaptcha</a> challenge info.
    #[serde(rename="recaptchaChallenge")]
    pub recaptcha_challenge: Option<RecaptchaChallenge>,
    /// The lead resource. The `LeadType` must not be `LEAD_TYPE_UNSPECIFIED`
    /// and either `email` or `phone_number` must be provided.
    pub lead: Option<Lead>,
}

impl RequestValue for CreateLeadRequest {}


/// A CompanyRelation resource representing information about a user's
/// affiliation and standing with a company in Partners.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create company relation users](struct.UserCreateCompanyRelationCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompanyRelation {
    /// The AdWords manager account # associated this company.
    #[serde(rename="managerAccount")]
    pub manager_account: Option<String>,
    /// The website URL for this company.
    pub website: Option<String>,
    /// The list of Google Partners specialization statuses for the company.
    #[serde(rename="specializationStatus")]
    pub specialization_status: Option<Vec<SpecializationStatus>>,
    /// The internal company ID.
    /// Only available for a whitelisted set of api clients.
    #[serde(rename="internalCompanyId")]
    pub internal_company_id: Option<String>,
    /// The primary location of the company.
    #[serde(rename="primaryAddress")]
    pub primary_address: Option<Location>,
    /// Indicates if the user is an admin for this company.
    #[serde(rename="companyAdmin")]
    pub company_admin: Option<bool>,
    /// The primary address for this company.
    pub address: Option<String>,
    /// The segment the company is classified as.
    pub segment: Option<Vec<String>>,
    /// The primary country code of the company.
    #[serde(rename="primaryCountryCode")]
    pub primary_country_code: Option<i64>,
    /// The name (in the company's primary language) for the company.
    pub name: Option<String>,
    /// The ID of the company. There may be no id if this is a
    /// pending company.5
    #[serde(rename="companyId")]
    pub company_id: Option<String>,
    /// The flag that indicates if the company is pending verification.
    #[serde(rename="isPending")]
    pub is_pending: Option<bool>,
    /// The timestamp of when affiliation was requested.
    /// @OutputOnly
    #[serde(rename="creationTime")]
    pub creation_time: Option<String>,
    /// Whether the company is a Partner.
    #[serde(rename="badgeTier")]
    pub badge_tier: Option<String>,
    /// The state of relationship, in terms of approvals.
    pub state: Option<String>,
    /// The phone number for the company's primary address.
    #[serde(rename="phoneNumber")]
    pub phone_number: Option<String>,
    /// A URL to a profile photo, e.g. a G+ profile photo.
    #[serde(rename="logoUrl")]
    pub logo_url: Option<String>,
    /// The primary language code of the company.
    #[serde(rename="primaryLanguageCode")]
    pub primary_language_code: Option<String>,
    /// The timestamp when the user was approved.
    /// @OutputOnly
    #[serde(rename="resolvedTimestamp")]
    pub resolved_timestamp: Option<String>,
}

impl RequestValue for CompanyRelation {}
impl ResponseResult for CompanyRelation {}


/// Response message for
/// ListUserStates.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list user states](struct.UserStateListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListUserStatesResponse {
    /// User's states.
    #[serde(rename="userStates")]
    pub user_states: Option<Vec<String>>,
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    pub response_metadata: Option<ResponseMetadata>,
}

impl ResponseResult for ListUserStatesResponse {}


/// Historical information about a Google Partners Offer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HistoricalOffer {
    /// Status of the offer.
    pub status: Option<String>,
    /// Offer code.
    #[serde(rename="offerCode")]
    pub offer_code: Option<String>,
    /// Country Code for the offer country.
    #[serde(rename="offerCountryCode")]
    pub offer_country_code: Option<i64>,
    /// Client's AdWords page URL.
    #[serde(rename="adwordsUrl")]
    pub adwords_url: Option<String>,
    /// Time offer was first created.
    #[serde(rename="creationTime")]
    pub creation_time: Option<String>,
    /// ID of client.
    #[serde(rename="clientId")]
    pub client_id: Option<String>,
    /// Email address for client.
    #[serde(rename="clientEmail")]
    pub client_email: Option<String>,
    /// Time last action was taken.
    #[serde(rename="lastModifiedTime")]
    pub last_modified_time: Option<String>,
    /// Type of offer.
    #[serde(rename="offerType")]
    pub offer_type: Option<String>,
    /// Name (First + Last) of the partners user to whom the incentive is allocated.
    #[serde(rename="senderName")]
    pub sender_name: Option<String>,
    /// Time this offer expires.
    #[serde(rename="expirationTime")]
    pub expiration_time: Option<String>,
    /// Name of the client.
    #[serde(rename="clientName")]
    pub client_name: Option<String>,
}

impl Part for HistoricalOffer {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all free methods, which are not associated with a particular resource.
/// It is not used directly, but through the `Partners` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_partners2 as partners2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use partners2::Partners;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_partnersstatus(...)`, `update_companies(...)` and `update_leads(...)`
/// // to build up your call.
/// let rb = hub.methods();
/// # }
/// ```
pub struct MethodMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
}

impl<'a, C, A> MethodsBuilder for MethodMethods<'a, C, A> {}

impl<'a, C, A> MethodMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update company.
    /// Should only be called within the context of an authorized logged in user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update_companies(&self, request: Company) -> MethodUpdateCompanyCall<'a, C, A> {
        MethodUpdateCompanyCall {
            hub: self.hub,
            _request: request,
            _update_mask: Default::default(),
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets Partners Status of the logged in user's agency.
    /// Should only be called if the logged in user is the admin of the agency.
    pub fn get_partnersstatus(&self) -> MethodGetPartnersstatuCall<'a, C, A> {
        MethodGetPartnersstatuCall {
            hub: self.hub,
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified lead.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update_leads(&self, request: Lead) -> MethodUpdateLeadCall<'a, C, A> {
        MethodUpdateLeadCall {
            hub: self.hub,
            _request: request,
            _update_mask: Default::default(),
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *userEvent* resources.
/// It is not used directly, but through the `Partners` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_partners2 as partners2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use partners2::Partners;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `log(...)`
/// // to build up your call.
/// let rb = hub.user_events();
/// # }
/// ```
pub struct UserEventMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
}

impl<'a, C, A> MethodsBuilder for UserEventMethods<'a, C, A> {}

impl<'a, C, A> UserEventMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Logs a user event.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn log(&self, request: LogUserEventRequest) -> UserEventLogCall<'a, C, A> {
        UserEventLogCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *clientMessage* resources.
/// It is not used directly, but through the `Partners` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_partners2 as partners2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use partners2::Partners;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `log(...)`
/// // to build up your call.
/// let rb = hub.client_messages();
/// # }
/// ```
pub struct ClientMessageMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
}

impl<'a, C, A> MethodsBuilder for ClientMessageMethods<'a, C, A> {}

impl<'a, C, A> ClientMessageMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Logs a generic message from the client, such as
    /// `Failed to render component`, `Profile page is running slow`,
    /// `More than 500 users have accessed this result.`, etc.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn log(&self, request: LogMessageRequest) -> ClientMessageLogCall<'a, C, A> {
        ClientMessageLogCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *company* resources.
/// It is not used directly, but through the `Partners` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_partners2 as partners2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use partners2::Partners;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `leads_create(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.companies();
/// # }
/// ```
pub struct CompanyMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
}

impl<'a, C, A> MethodsBuilder for CompanyMethods<'a, C, A> {}

impl<'a, C, A> CompanyMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a company.
    /// 
    /// # Arguments
    ///
    /// * `companyId` - The ID of the company to retrieve.
    pub fn get(&self, company_id: &str) -> CompanyGetCall<'a, C, A> {
        CompanyGetCall {
            hub: self.hub,
            _company_id: company_id.to_string(),
            _view: Default::default(),
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _order_by: Default::default(),
            _currency_code: Default::default(),
            _address: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an advertiser lead for the given company ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `companyId` - The ID of the company to contact.
    pub fn leads_create(&self, request: CreateLeadRequest, company_id: &str) -> CompanyLeadCreateCall<'a, C, A> {
        CompanyLeadCreateCall {
            hub: self.hub,
            _request: request,
            _company_id: company_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists companies.
    pub fn list(&self) -> CompanyListCall<'a, C, A> {
        CompanyListCall {
            hub: self.hub,
            _website_url: Default::default(),
            _view: Default::default(),
            _specializations: Default::default(),
            _services: Default::default(),
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _min_monthly_budget_units: Default::default(),
            _min_monthly_budget_nanos: Default::default(),
            _min_monthly_budget_currency_code: Default::default(),
            _max_monthly_budget_units: Default::default(),
            _max_monthly_budget_nanos: Default::default(),
            _max_monthly_budget_currency_code: Default::default(),
            _language_codes: Default::default(),
            _industries: Default::default(),
            _gps_motivations: Default::default(),
            _company_name: Default::default(),
            _address: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *lead* resources.
/// It is not used directly, but through the `Partners` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_partners2 as partners2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use partners2::Partners;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.leads();
/// # }
/// ```
pub struct LeadMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
}

impl<'a, C, A> MethodsBuilder for LeadMethods<'a, C, A> {}

impl<'a, C, A> LeadMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists advertiser leads for a user's associated company.
    /// Should only be called within the context of an authorized logged in user.
    pub fn list(&self) -> LeadListCall<'a, C, A> {
        LeadListCall {
            hub: self.hub,
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *analytic* resources.
/// It is not used directly, but through the `Partners` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_partners2 as partners2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use partners2::Partners;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.analytics();
/// # }
/// ```
pub struct AnalyticMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
}

impl<'a, C, A> MethodsBuilder for AnalyticMethods<'a, C, A> {}

impl<'a, C, A> AnalyticMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists analytics data for a user's associated company.
    /// Should only be called within the context of an authorized logged in user.
    pub fn list(&self) -> AnalyticListCall<'a, C, A> {
        AnalyticListCall {
            hub: self.hub,
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *offer* resources.
/// It is not used directly, but through the `Partners` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_partners2 as partners2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use partners2::Partners;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `history_list(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.offers();
/// # }
/// ```
pub struct OfferMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
}

impl<'a, C, A> MethodsBuilder for OfferMethods<'a, C, A> {}

impl<'a, C, A> OfferMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the Historical Offers for the current user (or user's entire company)
    pub fn history_list(&self) -> OfferHistoryListCall<'a, C, A> {
        OfferHistoryListCall {
            hub: self.hub,
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _entire_company: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the Offers available for the current user
    pub fn list(&self) -> OfferListCall<'a, C, A> {
        OfferListCall {
            hub: self.hub,
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *userState* resources.
/// It is not used directly, but through the `Partners` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_partners2 as partners2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use partners2::Partners;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.user_states();
/// # }
/// ```
pub struct UserStateMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
}

impl<'a, C, A> MethodsBuilder for UserStateMethods<'a, C, A> {}

impl<'a, C, A> UserStateMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists states for current user.
    pub fn list(&self) -> UserStateListCall<'a, C, A> {
        UserStateListCall {
            hub: self.hub,
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *user* resources.
/// It is not used directly, but through the `Partners` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_partners2 as partners2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use partners2::Partners;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create_company_relation(...)`, `delete_company_relation(...)`, `get(...)` and `update_profile(...)`
/// // to build up your call.
/// let rb = hub.users();
/// # }
/// ```
pub struct UserMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
}

impl<'a, C, A> MethodsBuilder for UserMethods<'a, C, A> {}

impl<'a, C, A> UserMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a user's profile. A user can only update their own profile and
    /// should only be called within the context of a logged in user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update_profile(&self, request: UserProfile) -> UserUpdateProfileCall<'a, C, A> {
        UserUpdateProfileCall {
            hub: self.hub,
            _request: request,
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a user's company relation. Affiliates the user to a company.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - The ID of the user. Can be set to <code>me</code> to mean
    ///              the currently authenticated user.
    pub fn create_company_relation(&self, request: CompanyRelation, user_id: &str) -> UserCreateCompanyRelationCall<'a, C, A> {
        UserCreateCompanyRelationCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a user.
    /// 
    /// # Arguments
    ///
    /// * `userId` - Identifier of the user. Can be set to <code>me</code> to mean the currently
    ///              authenticated user.
    pub fn get(&self, user_id: &str) -> UserGetCall<'a, C, A> {
        UserGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _user_view: Default::default(),
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a user's company relation. Unaffiliaites the user from a company.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The ID of the user. Can be set to <code>me</code> to mean
    ///              the currently authenticated user.
    pub fn delete_company_relation(&self, user_id: &str) -> UserDeleteCompanyRelationCall<'a, C, A> {
        UserDeleteCompanyRelationCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Update company.
/// Should only be called within the context of an authorized logged in user.
///
/// A builder for the *updateCompanies* method.
/// It is not used directly, but through a `MethodMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_partners2 as partners2;
/// use partners2::Company;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use partners2::Partners;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Company::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().update_companies(req)
///              .update_mask("et")
///              .request_metadata_user_overrides_user_id("diam")
///              .request_metadata_user_overrides_ip_address("ipsum")
///              .request_metadata_traffic_source_traffic_sub_id("Lorem")
///              .request_metadata_traffic_source_traffic_source_id("et")
///              .request_metadata_partners_session_id("duo")
///              .request_metadata_locale("aliquyam")
///              .add_request_metadata_experiment_ids("sea")
///              .doit();
/// # }
/// ```
pub struct MethodUpdateCompanyCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
    _request: Company,
    _update_mask: Option<String>,
    _request_metadata_user_overrides_user_id: Option<String>,
    _request_metadata_user_overrides_ip_address: Option<String>,
    _request_metadata_traffic_source_traffic_sub_id: Option<String>,
    _request_metadata_traffic_source_traffic_source_id: Option<String>,
    _request_metadata_partners_session_id: Option<String>,
    _request_metadata_locale: Option<String>,
    _request_metadata_experiment_ids: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for MethodUpdateCompanyCall<'a, C, A> {}

impl<'a, C, A> MethodUpdateCompanyCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Company)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "partners.updateCompanies",
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(11 + self._additional_params.len());
        if let Some(value) = self._update_mask {
            params.push(("updateMask", value.to_string()));
        }
        if let Some(value) = self._request_metadata_user_overrides_user_id {
            params.push(("requestMetadata.userOverrides.userId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address {
            params.push(("requestMetadata.userOverrides.ipAddress", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id {
            params.push(("requestMetadata.trafficSource.trafficSubId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id {
            params.push(("requestMetadata.trafficSource.trafficSourceId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_partners_session_id {
            params.push(("requestMetadata.partnersSessionId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_locale {
            params.push(("requestMetadata.locale", value.to_string()));
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push(("requestMetadata.experimentIds", f.to_string()));
            }
        }
        for &field in ["alt", "updateMask", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/companies";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }


        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Company) -> MethodUpdateCompanyCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Standard field mask for the set of fields to be updated.
    /// Required with at least 1 value in FieldMask's paths.
    ///
    /// Sets the *update mask* query property to the given value.
    pub fn update_mask(mut self, new_value: &str) -> MethodUpdateCompanyCall<'a, C, A> {
        self._update_mask = Some(new_value.to_string());
        self
    }
    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> MethodUpdateCompanyCall<'a, C, A> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> MethodUpdateCompanyCall<'a, C, A> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> MethodUpdateCompanyCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> MethodUpdateCompanyCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> MethodUpdateCompanyCall<'a, C, A> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> MethodUpdateCompanyCall<'a, C, A> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> MethodUpdateCompanyCall<'a, C, A> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MethodUpdateCompanyCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> MethodUpdateCompanyCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Gets Partners Status of the logged in user's agency.
/// Should only be called if the logged in user is the admin of the agency.
///
/// A builder for the *getPartnersstatus* method.
/// It is not used directly, but through a `MethodMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_partners2 as partners2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use partners2::Partners;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().get_partnersstatus()
///              .request_metadata_user_overrides_user_id("Lorem")
///              .request_metadata_user_overrides_ip_address("eos")
///              .request_metadata_traffic_source_traffic_sub_id("erat")
///              .request_metadata_traffic_source_traffic_source_id("sadipscing")
///              .request_metadata_partners_session_id("dolor")
///              .request_metadata_locale("eirmod")
///              .add_request_metadata_experiment_ids("elitr")
///              .doit();
/// # }
/// ```
pub struct MethodGetPartnersstatuCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
    _request_metadata_user_overrides_user_id: Option<String>,
    _request_metadata_user_overrides_ip_address: Option<String>,
    _request_metadata_traffic_source_traffic_sub_id: Option<String>,
    _request_metadata_traffic_source_traffic_source_id: Option<String>,
    _request_metadata_partners_session_id: Option<String>,
    _request_metadata_locale: Option<String>,
    _request_metadata_experiment_ids: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for MethodGetPartnersstatuCall<'a, C, A> {}

impl<'a, C, A> MethodGetPartnersstatuCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GetPartnersStatusResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "partners.getPartnersstatus",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
        if let Some(value) = self._request_metadata_user_overrides_user_id {
            params.push(("requestMetadata.userOverrides.userId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address {
            params.push(("requestMetadata.userOverrides.ipAddress", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id {
            params.push(("requestMetadata.trafficSource.trafficSubId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id {
            params.push(("requestMetadata.trafficSource.trafficSourceId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_partners_session_id {
            params.push(("requestMetadata.partnersSessionId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_locale {
            params.push(("requestMetadata.locale", value.to_string()));
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push(("requestMetadata.experimentIds", f.to_string()));
            }
        }
        for &field in ["alt", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/partnersstatus";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }


        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> MethodGetPartnersstatuCall<'a, C, A> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> MethodGetPartnersstatuCall<'a, C, A> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> MethodGetPartnersstatuCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> MethodGetPartnersstatuCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> MethodGetPartnersstatuCall<'a, C, A> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> MethodGetPartnersstatuCall<'a, C, A> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> MethodGetPartnersstatuCall<'a, C, A> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MethodGetPartnersstatuCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> MethodGetPartnersstatuCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Updates the specified lead.
///
/// A builder for the *updateLeads* method.
/// It is not used directly, but through a `MethodMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_partners2 as partners2;
/// use partners2::Lead;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use partners2::Partners;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Lead::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().update_leads(req)
///              .update_mask("amet")
///              .request_metadata_user_overrides_user_id("no")
///              .request_metadata_user_overrides_ip_address("labore")
///              .request_metadata_traffic_source_traffic_sub_id("eirmod")
///              .request_metadata_traffic_source_traffic_source_id("dolore")
///              .request_metadata_partners_session_id("invidunt")
///              .request_metadata_locale("aliquyam")
///              .add_request_metadata_experiment_ids("accusam")
///              .doit();
/// # }
/// ```
pub struct MethodUpdateLeadCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
    _request: Lead,
    _update_mask: Option<String>,
    _request_metadata_user_overrides_user_id: Option<String>,
    _request_metadata_user_overrides_ip_address: Option<String>,
    _request_metadata_traffic_source_traffic_sub_id: Option<String>,
    _request_metadata_traffic_source_traffic_source_id: Option<String>,
    _request_metadata_partners_session_id: Option<String>,
    _request_metadata_locale: Option<String>,
    _request_metadata_experiment_ids: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for MethodUpdateLeadCall<'a, C, A> {}

impl<'a, C, A> MethodUpdateLeadCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Lead)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "partners.updateLeads",
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(11 + self._additional_params.len());
        if let Some(value) = self._update_mask {
            params.push(("updateMask", value.to_string()));
        }
        if let Some(value) = self._request_metadata_user_overrides_user_id {
            params.push(("requestMetadata.userOverrides.userId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address {
            params.push(("requestMetadata.userOverrides.ipAddress", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id {
            params.push(("requestMetadata.trafficSource.trafficSubId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id {
            params.push(("requestMetadata.trafficSource.trafficSourceId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_partners_session_id {
            params.push(("requestMetadata.partnersSessionId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_locale {
            params.push(("requestMetadata.locale", value.to_string()));
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push(("requestMetadata.experimentIds", f.to_string()));
            }
        }
        for &field in ["alt", "updateMask", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/leads";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }


        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Lead) -> MethodUpdateLeadCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Standard field mask for the set of fields to be updated.
    /// Required with at least 1 value in FieldMask's paths.
    /// Only `state` and `adwords_customer_id` are currently supported.
    ///
    /// Sets the *update mask* query property to the given value.
    pub fn update_mask(mut self, new_value: &str) -> MethodUpdateLeadCall<'a, C, A> {
        self._update_mask = Some(new_value.to_string());
        self
    }
    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> MethodUpdateLeadCall<'a, C, A> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> MethodUpdateLeadCall<'a, C, A> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> MethodUpdateLeadCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> MethodUpdateLeadCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> MethodUpdateLeadCall<'a, C, A> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> MethodUpdateLeadCall<'a, C, A> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> MethodUpdateLeadCall<'a, C, A> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MethodUpdateLeadCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> MethodUpdateLeadCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Logs a user event.
///
/// A builder for the *log* method supported by a *userEvent* resource.
/// It is not used directly, but through a `UserEventMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_partners2 as partners2;
/// use partners2::LogUserEventRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use partners2::Partners;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = LogUserEventRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.user_events().log(req)
///              .doit();
/// # }
/// ```
pub struct UserEventLogCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
    _request: LogUserEventRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for UserEventLogCall<'a, C, A> {}

impl<'a, C, A> UserEventLogCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, LogUserEventResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "partners.userEvents.log",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/userEvents:log";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }


        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: LogUserEventRequest) -> UserEventLogCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> UserEventLogCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserEventLogCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Logs a generic message from the client, such as
/// `Failed to render component`, `Profile page is running slow`,
/// `More than 500 users have accessed this result.`, etc.
///
/// A builder for the *log* method supported by a *clientMessage* resource.
/// It is not used directly, but through a `ClientMessageMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_partners2 as partners2;
/// use partners2::LogMessageRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use partners2::Partners;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = LogMessageRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.client_messages().log(req)
///              .doit();
/// # }
/// ```
pub struct ClientMessageLogCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
    _request: LogMessageRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for ClientMessageLogCall<'a, C, A> {}

impl<'a, C, A> ClientMessageLogCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, LogMessageResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "partners.clientMessages.log",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/clientMessages:log";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }


        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: LogMessageRequest) -> ClientMessageLogCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ClientMessageLogCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ClientMessageLogCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Gets a company.
///
/// A builder for the *get* method supported by a *company* resource.
/// It is not used directly, but through a `CompanyMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_partners2 as partners2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use partners2::Partners;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.companies().get("companyId")
///              .view("sea")
///              .request_metadata_user_overrides_user_id("et")
///              .request_metadata_user_overrides_ip_address("duo")
///              .request_metadata_traffic_source_traffic_sub_id("et")
///              .request_metadata_traffic_source_traffic_source_id("eirmod")
///              .request_metadata_partners_session_id("sanctus")
///              .request_metadata_locale("et")
///              .add_request_metadata_experiment_ids("amet")
///              .order_by("et")
///              .currency_code("consetetur")
///              .address("ut")
///              .doit();
/// # }
/// ```
pub struct CompanyGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
    _company_id: String,
    _view: Option<String>,
    _request_metadata_user_overrides_user_id: Option<String>,
    _request_metadata_user_overrides_ip_address: Option<String>,
    _request_metadata_traffic_source_traffic_sub_id: Option<String>,
    _request_metadata_traffic_source_traffic_source_id: Option<String>,
    _request_metadata_partners_session_id: Option<String>,
    _request_metadata_locale: Option<String>,
    _request_metadata_experiment_ids: Vec<String>,
    _order_by: Option<String>,
    _currency_code: Option<String>,
    _address: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for CompanyGetCall<'a, C, A> {}

impl<'a, C, A> CompanyGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GetCompanyResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "partners.companies.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(14 + self._additional_params.len());
        params.push(("companyId", self._company_id.to_string()));
        if let Some(value) = self._view {
            params.push(("view", value.to_string()));
        }
        if let Some(value) = self._request_metadata_user_overrides_user_id {
            params.push(("requestMetadata.userOverrides.userId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address {
            params.push(("requestMetadata.userOverrides.ipAddress", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id {
            params.push(("requestMetadata.trafficSource.trafficSubId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id {
            params.push(("requestMetadata.trafficSource.trafficSourceId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_partners_session_id {
            params.push(("requestMetadata.partnersSessionId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_locale {
            params.push(("requestMetadata.locale", value.to_string()));
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push(("requestMetadata.experimentIds", f.to_string()));
            }
        }
        if let Some(value) = self._order_by {
            params.push(("orderBy", value.to_string()));
        }
        if let Some(value) = self._currency_code {
            params.push(("currencyCode", value.to_string()));
        }
        if let Some(value) = self._address {
            params.push(("address", value.to_string()));
        }
        for &field in ["alt", "companyId", "view", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds", "orderBy", "currencyCode", "address"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/companies/{companyId}";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{companyId}", "companyId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["companyId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the company to retrieve.
    ///
    /// Sets the *company id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn company_id(mut self, new_value: &str) -> CompanyGetCall<'a, C, A> {
        self._company_id = new_value.to_string();
        self
    }
    /// The view of `Company` resource to be returned. This must not be
    /// `COMPANY_VIEW_UNSPECIFIED`.
    ///
    /// Sets the *view* query property to the given value.
    pub fn view(mut self, new_value: &str) -> CompanyGetCall<'a, C, A> {
        self._view = Some(new_value.to_string());
        self
    }
    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> CompanyGetCall<'a, C, A> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> CompanyGetCall<'a, C, A> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> CompanyGetCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> CompanyGetCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> CompanyGetCall<'a, C, A> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> CompanyGetCall<'a, C, A> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> CompanyGetCall<'a, C, A> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
        self
    }
    /// How to order addresses within the returned company. Currently, only
    /// `address` and `address desc` is supported which will sorted by closest to
    /// farthest in distance from given address and farthest to closest distance
    /// from given address respectively.
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> CompanyGetCall<'a, C, A> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// If the company's budget is in a different currency code than this one, then
    /// the converted budget is converted to this currency code.
    ///
    /// Sets the *currency code* query property to the given value.
    pub fn currency_code(mut self, new_value: &str) -> CompanyGetCall<'a, C, A> {
        self._currency_code = Some(new_value.to_string());
        self
    }
    /// The address to use for sorting the company's addresses by proximity.
    /// If not given, the geo-located address of the request is used.
    /// Used when order_by is set.
    ///
    /// Sets the *address* query property to the given value.
    pub fn address(mut self, new_value: &str) -> CompanyGetCall<'a, C, A> {
        self._address = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CompanyGetCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CompanyGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Creates an advertiser lead for the given company ID.
///
/// A builder for the *leads.create* method supported by a *company* resource.
/// It is not used directly, but through a `CompanyMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_partners2 as partners2;
/// use partners2::CreateLeadRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use partners2::Partners;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CreateLeadRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.companies().leads_create(req, "companyId")
///              .doit();
/// # }
/// ```
pub struct CompanyLeadCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
    _request: CreateLeadRequest,
    _company_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for CompanyLeadCreateCall<'a, C, A> {}

impl<'a, C, A> CompanyLeadCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, CreateLeadResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "partners.companies.leads.create",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("companyId", self._company_id.to_string()));
        for &field in ["alt", "companyId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/companies/{companyId}/leads";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{companyId}", "companyId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["companyId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: CreateLeadRequest) -> CompanyLeadCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the company to contact.
    ///
    /// Sets the *company id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn company_id(mut self, new_value: &str) -> CompanyLeadCreateCall<'a, C, A> {
        self._company_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CompanyLeadCreateCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CompanyLeadCreateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Lists companies.
///
/// A builder for the *list* method supported by a *company* resource.
/// It is not used directly, but through a `CompanyMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_partners2 as partners2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use partners2::Partners;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.companies().list()
///              .website_url("sed")
///              .view("dolor")
///              .add_specializations("dolor")
///              .add_services("dolor")
///              .request_metadata_user_overrides_user_id("et")
///              .request_metadata_user_overrides_ip_address("consetetur")
///              .request_metadata_traffic_source_traffic_sub_id("amet.")
///              .request_metadata_traffic_source_traffic_source_id("voluptua.")
///              .request_metadata_partners_session_id("Lorem")
///              .request_metadata_locale("gubergren")
///              .add_request_metadata_experiment_ids("justo")
///              .page_token("sit")
///              .page_size(-26)
///              .order_by("diam")
///              .min_monthly_budget_units("rebum.")
///              .min_monthly_budget_nanos(-45)
///              .min_monthly_budget_currency_code("sadipscing")
///              .max_monthly_budget_units("vero")
///              .max_monthly_budget_nanos(-95)
///              .max_monthly_budget_currency_code("invidunt")
///              .add_language_codes("consetetur")
///              .add_industries("dolore")
///              .add_gps_motivations("duo")
///              .company_name("aliquyam")
///              .address("Lorem")
///              .doit();
/// # }
/// ```
pub struct CompanyListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
    _website_url: Option<String>,
    _view: Option<String>,
    _specializations: Vec<String>,
    _services: Vec<String>,
    _request_metadata_user_overrides_user_id: Option<String>,
    _request_metadata_user_overrides_ip_address: Option<String>,
    _request_metadata_traffic_source_traffic_sub_id: Option<String>,
    _request_metadata_traffic_source_traffic_source_id: Option<String>,
    _request_metadata_partners_session_id: Option<String>,
    _request_metadata_locale: Option<String>,
    _request_metadata_experiment_ids: Vec<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _order_by: Option<String>,
    _min_monthly_budget_units: Option<String>,
    _min_monthly_budget_nanos: Option<i32>,
    _min_monthly_budget_currency_code: Option<String>,
    _max_monthly_budget_units: Option<String>,
    _max_monthly_budget_nanos: Option<i32>,
    _max_monthly_budget_currency_code: Option<String>,
    _language_codes: Vec<String>,
    _industries: Vec<String>,
    _gps_motivations: Vec<String>,
    _company_name: Option<String>,
    _address: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for CompanyListCall<'a, C, A> {}

impl<'a, C, A> CompanyListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListCompaniesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "partners.companies.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(27 + self._additional_params.len());
        if let Some(value) = self._website_url {
            params.push(("websiteUrl", value.to_string()));
        }
        if let Some(value) = self._view {
            params.push(("view", value.to_string()));
        }
        if self._specializations.len() > 0 {
            for f in self._specializations.iter() {
                params.push(("specializations", f.to_string()));
            }
        }
        if self._services.len() > 0 {
            for f in self._services.iter() {
                params.push(("services", f.to_string()));
            }
        }
        if let Some(value) = self._request_metadata_user_overrides_user_id {
            params.push(("requestMetadata.userOverrides.userId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address {
            params.push(("requestMetadata.userOverrides.ipAddress", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id {
            params.push(("requestMetadata.trafficSource.trafficSubId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id {
            params.push(("requestMetadata.trafficSource.trafficSourceId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_partners_session_id {
            params.push(("requestMetadata.partnersSessionId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_locale {
            params.push(("requestMetadata.locale", value.to_string()));
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push(("requestMetadata.experimentIds", f.to_string()));
            }
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._order_by {
            params.push(("orderBy", value.to_string()));
        }
        if let Some(value) = self._min_monthly_budget_units {
            params.push(("minMonthlyBudget.units", value.to_string()));
        }
        if let Some(value) = self._min_monthly_budget_nanos {
            params.push(("minMonthlyBudget.nanos", value.to_string()));
        }
        if let Some(value) = self._min_monthly_budget_currency_code {
            params.push(("minMonthlyBudget.currencyCode", value.to_string()));
        }
        if let Some(value) = self._max_monthly_budget_units {
            params.push(("maxMonthlyBudget.units", value.to_string()));
        }
        if let Some(value) = self._max_monthly_budget_nanos {
            params.push(("maxMonthlyBudget.nanos", value.to_string()));
        }
        if let Some(value) = self._max_monthly_budget_currency_code {
            params.push(("maxMonthlyBudget.currencyCode", value.to_string()));
        }
        if self._language_codes.len() > 0 {
            for f in self._language_codes.iter() {
                params.push(("languageCodes", f.to_string()));
            }
        }
        if self._industries.len() > 0 {
            for f in self._industries.iter() {
                params.push(("industries", f.to_string()));
            }
        }
        if self._gps_motivations.len() > 0 {
            for f in self._gps_motivations.iter() {
                params.push(("gpsMotivations", f.to_string()));
            }
        }
        if let Some(value) = self._company_name {
            params.push(("companyName", value.to_string()));
        }
        if let Some(value) = self._address {
            params.push(("address", value.to_string()));
        }
        for &field in ["alt", "websiteUrl", "view", "specializations", "services", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds", "pageToken", "pageSize", "orderBy", "minMonthlyBudget.units", "minMonthlyBudget.nanos", "minMonthlyBudget.currencyCode", "maxMonthlyBudget.units", "maxMonthlyBudget.nanos", "maxMonthlyBudget.currencyCode", "languageCodes", "industries", "gpsMotivations", "companyName", "address"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/companies";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }


        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Website URL that will help to find a better matched company.
    /// .
    ///
    /// Sets the *website url* query property to the given value.
    pub fn website_url(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._website_url = Some(new_value.to_string());
        self
    }
    /// The view of the `Company` resource to be returned. This must not be
    /// `COMPANY_VIEW_UNSPECIFIED`.
    ///
    /// Sets the *view* query property to the given value.
    pub fn view(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._view = Some(new_value.to_string());
        self
    }
    /// List of specializations that the returned agencies should provide. If this
    /// is not empty, any returned agency must have at least one of these
    /// specializations, or one of the services in the "services" field.
    ///
    /// Append the given value to the *specializations* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_specializations(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._specializations.push(new_value.to_string());
        self
    }
    /// List of services that the returned agencies should provide. If this is
    /// not empty, any returned agency must have at least one of these services,
    /// or one of the specializations in the "specializations" field.
    ///
    /// Append the given value to the *services* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_services(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._services.push(new_value.to_string());
        self
    }
    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
        self
    }
    /// A token identifying a page of results that the server returns.
    /// Typically, this is the value of `ListCompaniesResponse.next_page_token`
    /// returned from the previous call to
    /// ListCompanies.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Requested page size. Server may return fewer companies than requested.
    /// If unspecified, server picks an appropriate default.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> CompanyListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// How to order addresses within the returned companies. Currently, only
    /// `address` and `address desc` is supported which will sorted by closest to
    /// farthest in distance from given address and farthest to closest distance
    /// from given address respectively.
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// The whole units of the amount.
    /// For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    ///
    /// Sets the *min monthly budget.units* query property to the given value.
    pub fn min_monthly_budget_units(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._min_monthly_budget_units = Some(new_value.to_string());
        self
    }
    /// Number of nano (10^-9) units of the amount.
    /// The value must be between -999,999,999 and +999,999,999 inclusive.
    /// If `units` is positive, `nanos` must be positive or zero.
    /// If `units` is zero, `nanos` can be positive, zero, or negative.
    /// If `units` is negative, `nanos` must be negative or zero.
    /// For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    ///
    /// Sets the *min monthly budget.nanos* query property to the given value.
    pub fn min_monthly_budget_nanos(mut self, new_value: i32) -> CompanyListCall<'a, C, A> {
        self._min_monthly_budget_nanos = Some(new_value);
        self
    }
    /// The 3-letter currency code defined in ISO 4217.
    ///
    /// Sets the *min monthly budget.currency code* query property to the given value.
    pub fn min_monthly_budget_currency_code(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._min_monthly_budget_currency_code = Some(new_value.to_string());
        self
    }
    /// The whole units of the amount.
    /// For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    ///
    /// Sets the *max monthly budget.units* query property to the given value.
    pub fn max_monthly_budget_units(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._max_monthly_budget_units = Some(new_value.to_string());
        self
    }
    /// Number of nano (10^-9) units of the amount.
    /// The value must be between -999,999,999 and +999,999,999 inclusive.
    /// If `units` is positive, `nanos` must be positive or zero.
    /// If `units` is zero, `nanos` can be positive, zero, or negative.
    /// If `units` is negative, `nanos` must be negative or zero.
    /// For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    ///
    /// Sets the *max monthly budget.nanos* query property to the given value.
    pub fn max_monthly_budget_nanos(mut self, new_value: i32) -> CompanyListCall<'a, C, A> {
        self._max_monthly_budget_nanos = Some(new_value);
        self
    }
    /// The 3-letter currency code defined in ISO 4217.
    ///
    /// Sets the *max monthly budget.currency code* query property to the given value.
    pub fn max_monthly_budget_currency_code(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._max_monthly_budget_currency_code = Some(new_value.to_string());
        self
    }
    /// List of language codes that company can support. Only primary language
    /// subtags are accepted as defined by
    /// <a href="https://tools.ietf.org/html/bcp47">BCP 47</a>
    /// (IETF BCP 47, "Tags for Identifying Languages").
    ///
    /// Append the given value to the *language codes* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_language_codes(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._language_codes.push(new_value.to_string());
        self
    }
    /// List of industries the company can help with.
    ///
    /// Append the given value to the *industries* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_industries(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._industries.push(new_value.to_string());
        self
    }
    /// List of reasons for using Google Partner Search to get companies.
    ///
    /// Append the given value to the *gps motivations* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_gps_motivations(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._gps_motivations.push(new_value.to_string());
        self
    }
    /// Company name to search for.
    ///
    /// Sets the *company name* query property to the given value.
    pub fn company_name(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._company_name = Some(new_value.to_string());
        self
    }
    /// The address to use when searching for companies.
    /// If not given, the geo-located address of the request is used.
    ///
    /// Sets the *address* query property to the given value.
    pub fn address(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._address = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CompanyListCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CompanyListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Lists advertiser leads for a user's associated company.
/// Should only be called within the context of an authorized logged in user.
///
/// A builder for the *list* method supported by a *lead* resource.
/// It is not used directly, but through a `LeadMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_partners2 as partners2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use partners2::Partners;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.leads().list()
///              .request_metadata_user_overrides_user_id("et")
///              .request_metadata_user_overrides_ip_address("clita")
///              .request_metadata_traffic_source_traffic_sub_id("consetetur")
///              .request_metadata_traffic_source_traffic_source_id("takimata")
///              .request_metadata_partners_session_id("nonumy")
///              .request_metadata_locale("kasd")
///              .add_request_metadata_experiment_ids("sanctus")
///              .page_token("takimata")
///              .page_size(-27)
///              .order_by("labore")
///              .doit();
/// # }
/// ```
pub struct LeadListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
    _request_metadata_user_overrides_user_id: Option<String>,
    _request_metadata_user_overrides_ip_address: Option<String>,
    _request_metadata_traffic_source_traffic_sub_id: Option<String>,
    _request_metadata_traffic_source_traffic_source_id: Option<String>,
    _request_metadata_partners_session_id: Option<String>,
    _request_metadata_locale: Option<String>,
    _request_metadata_experiment_ids: Vec<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _order_by: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for LeadListCall<'a, C, A> {}

impl<'a, C, A> LeadListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListLeadsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "partners.leads.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(12 + self._additional_params.len());
        if let Some(value) = self._request_metadata_user_overrides_user_id {
            params.push(("requestMetadata.userOverrides.userId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address {
            params.push(("requestMetadata.userOverrides.ipAddress", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id {
            params.push(("requestMetadata.trafficSource.trafficSubId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id {
            params.push(("requestMetadata.trafficSource.trafficSourceId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_partners_session_id {
            params.push(("requestMetadata.partnersSessionId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_locale {
            params.push(("requestMetadata.locale", value.to_string()));
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push(("requestMetadata.experimentIds", f.to_string()));
            }
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._order_by {
            params.push(("orderBy", value.to_string()));
        }
        for &field in ["alt", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds", "pageToken", "pageSize", "orderBy"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/leads";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }


        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> LeadListCall<'a, C, A> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> LeadListCall<'a, C, A> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> LeadListCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> LeadListCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> LeadListCall<'a, C, A> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> LeadListCall<'a, C, A> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> LeadListCall<'a, C, A> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
        self
    }
    /// A token identifying a page of results that the server returns.
    /// Typically, this is the value of `ListLeadsResponse.next_page_token`
    /// returned from the previous call to
    /// ListLeads.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> LeadListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Requested page size. Server may return fewer leads than requested.
    /// If unspecified, server picks an appropriate default.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> LeadListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// How to order Leads. Currently, only `create_time`
    /// and `create_time desc` are supported
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> LeadListCall<'a, C, A> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LeadListCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> LeadListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Lists analytics data for a user's associated company.
/// Should only be called within the context of an authorized logged in user.
///
/// A builder for the *list* method supported by a *analytic* resource.
/// It is not used directly, but through a `AnalyticMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_partners2 as partners2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use partners2::Partners;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.analytics().list()
///              .request_metadata_user_overrides_user_id("invidunt")
///              .request_metadata_user_overrides_ip_address("ea")
///              .request_metadata_traffic_source_traffic_sub_id("sadipscing")
///              .request_metadata_traffic_source_traffic_source_id("rebum.")
///              .request_metadata_partners_session_id("dolore")
///              .request_metadata_locale("nonumy")
///              .add_request_metadata_experiment_ids("sed")
///              .page_token("aliquyam")
///              .page_size(-53)
///              .doit();
/// # }
/// ```
pub struct AnalyticListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
    _request_metadata_user_overrides_user_id: Option<String>,
    _request_metadata_user_overrides_ip_address: Option<String>,
    _request_metadata_traffic_source_traffic_sub_id: Option<String>,
    _request_metadata_traffic_source_traffic_source_id: Option<String>,
    _request_metadata_partners_session_id: Option<String>,
    _request_metadata_locale: Option<String>,
    _request_metadata_experiment_ids: Vec<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for AnalyticListCall<'a, C, A> {}

impl<'a, C, A> AnalyticListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListAnalyticsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "partners.analytics.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(11 + self._additional_params.len());
        if let Some(value) = self._request_metadata_user_overrides_user_id {
            params.push(("requestMetadata.userOverrides.userId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address {
            params.push(("requestMetadata.userOverrides.ipAddress", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id {
            params.push(("requestMetadata.trafficSource.trafficSubId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id {
            params.push(("requestMetadata.trafficSource.trafficSourceId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_partners_session_id {
            params.push(("requestMetadata.partnersSessionId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_locale {
            params.push(("requestMetadata.locale", value.to_string()));
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push(("requestMetadata.experimentIds", f.to_string()));
            }
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        for &field in ["alt", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/analytics";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }


        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> AnalyticListCall<'a, C, A> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> AnalyticListCall<'a, C, A> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> AnalyticListCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> AnalyticListCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> AnalyticListCall<'a, C, A> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> AnalyticListCall<'a, C, A> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> AnalyticListCall<'a, C, A> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
        self
    }
    /// A token identifying a page of results that the server returns.
    /// Typically, this is the value of `ListAnalyticsResponse.next_page_token`
    /// returned from the previous call to
    /// ListAnalytics.
    /// Will be a date string in `YYYY-MM-DD` format representing the end date
    /// of the date range of results to return.
    /// If unspecified or set to "", default value is the current date.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AnalyticListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Requested page size. Server may return fewer analytics than requested.
    /// If unspecified or set to 0, default value is 30.
    /// Specifies the number of days in the date range when querying analytics.
    /// The `page_token` represents the end date of the date range
    /// and the start date is calculated using the `page_size` as the number
    /// of days BEFORE the end date.
    /// Must be a non-negative integer.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AnalyticListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AnalyticListCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> AnalyticListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Lists the Historical Offers for the current user (or user's entire company)
///
/// A builder for the *history.list* method supported by a *offer* resource.
/// It is not used directly, but through a `OfferMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_partners2 as partners2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use partners2::Partners;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.offers().history_list()
///              .request_metadata_user_overrides_user_id("eirmod")
///              .request_metadata_user_overrides_ip_address("consetetur")
///              .request_metadata_traffic_source_traffic_sub_id("labore")
///              .request_metadata_traffic_source_traffic_source_id("sed")
///              .request_metadata_partners_session_id("ea")
///              .request_metadata_locale("gubergren")
///              .add_request_metadata_experiment_ids("aliquyam")
///              .page_token("eos")
///              .page_size(-38)
///              .order_by("sea")
///              .entire_company(false)
///              .doit();
/// # }
/// ```
pub struct OfferHistoryListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
    _request_metadata_user_overrides_user_id: Option<String>,
    _request_metadata_user_overrides_ip_address: Option<String>,
    _request_metadata_traffic_source_traffic_sub_id: Option<String>,
    _request_metadata_traffic_source_traffic_source_id: Option<String>,
    _request_metadata_partners_session_id: Option<String>,
    _request_metadata_locale: Option<String>,
    _request_metadata_experiment_ids: Vec<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _order_by: Option<String>,
    _entire_company: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for OfferHistoryListCall<'a, C, A> {}

impl<'a, C, A> OfferHistoryListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListOffersHistoryResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "partners.offers.history.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(13 + self._additional_params.len());
        if let Some(value) = self._request_metadata_user_overrides_user_id {
            params.push(("requestMetadata.userOverrides.userId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address {
            params.push(("requestMetadata.userOverrides.ipAddress", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id {
            params.push(("requestMetadata.trafficSource.trafficSubId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id {
            params.push(("requestMetadata.trafficSource.trafficSourceId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_partners_session_id {
            params.push(("requestMetadata.partnersSessionId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_locale {
            params.push(("requestMetadata.locale", value.to_string()));
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push(("requestMetadata.experimentIds", f.to_string()));
            }
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._order_by {
            params.push(("orderBy", value.to_string()));
        }
        if let Some(value) = self._entire_company {
            params.push(("entireCompany", value.to_string()));
        }
        for &field in ["alt", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds", "pageToken", "pageSize", "orderBy", "entireCompany"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/offers/history";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }


        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> OfferHistoryListCall<'a, C, A> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> OfferHistoryListCall<'a, C, A> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> OfferHistoryListCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> OfferHistoryListCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> OfferHistoryListCall<'a, C, A> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> OfferHistoryListCall<'a, C, A> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> OfferHistoryListCall<'a, C, A> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
        self
    }
    /// Token to retrieve a specific page.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> OfferHistoryListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of rows to return per page.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> OfferHistoryListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// Comma-separated list of fields to order by, e.g.: "foo,bar,baz".
    /// Use "foo desc" to sort descending.
    /// List of valid field names is: name, offer_code, expiration_time, status,
    ///     last_modified_time, sender_name, creation_time, country_code,
    ///     offer_type.
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> OfferHistoryListCall<'a, C, A> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// if true, show history for the entire company.  Requires user to be admin.
    ///
    /// Sets the *entire company* query property to the given value.
    pub fn entire_company(mut self, new_value: bool) -> OfferHistoryListCall<'a, C, A> {
        self._entire_company = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OfferHistoryListCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> OfferHistoryListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Lists the Offers available for the current user
///
/// A builder for the *list* method supported by a *offer* resource.
/// It is not used directly, but through a `OfferMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_partners2 as partners2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use partners2::Partners;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.offers().list()
///              .request_metadata_user_overrides_user_id("ipsum")
///              .request_metadata_user_overrides_ip_address("aliquyam")
///              .request_metadata_traffic_source_traffic_sub_id("dolores")
///              .request_metadata_traffic_source_traffic_source_id("sit")
///              .request_metadata_partners_session_id("diam")
///              .request_metadata_locale("ut")
///              .add_request_metadata_experiment_ids("justo")
///              .doit();
/// # }
/// ```
pub struct OfferListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
    _request_metadata_user_overrides_user_id: Option<String>,
    _request_metadata_user_overrides_ip_address: Option<String>,
    _request_metadata_traffic_source_traffic_sub_id: Option<String>,
    _request_metadata_traffic_source_traffic_source_id: Option<String>,
    _request_metadata_partners_session_id: Option<String>,
    _request_metadata_locale: Option<String>,
    _request_metadata_experiment_ids: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for OfferListCall<'a, C, A> {}

impl<'a, C, A> OfferListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListOffersResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "partners.offers.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
        if let Some(value) = self._request_metadata_user_overrides_user_id {
            params.push(("requestMetadata.userOverrides.userId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address {
            params.push(("requestMetadata.userOverrides.ipAddress", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id {
            params.push(("requestMetadata.trafficSource.trafficSubId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id {
            params.push(("requestMetadata.trafficSource.trafficSourceId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_partners_session_id {
            params.push(("requestMetadata.partnersSessionId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_locale {
            params.push(("requestMetadata.locale", value.to_string()));
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push(("requestMetadata.experimentIds", f.to_string()));
            }
        }
        for &field in ["alt", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/offers";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }


        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> OfferListCall<'a, C, A> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> OfferListCall<'a, C, A> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> OfferListCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> OfferListCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> OfferListCall<'a, C, A> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> OfferListCall<'a, C, A> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> OfferListCall<'a, C, A> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OfferListCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> OfferListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Lists states for current user.
///
/// A builder for the *list* method supported by a *userState* resource.
/// It is not used directly, but through a `UserStateMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_partners2 as partners2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use partners2::Partners;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.user_states().list()
///              .request_metadata_user_overrides_user_id("est")
///              .request_metadata_user_overrides_ip_address("amet")
///              .request_metadata_traffic_source_traffic_sub_id("accusam")
///              .request_metadata_traffic_source_traffic_source_id("clita")
///              .request_metadata_partners_session_id("diam")
///              .request_metadata_locale("justo")
///              .add_request_metadata_experiment_ids("est")
///              .doit();
/// # }
/// ```
pub struct UserStateListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
    _request_metadata_user_overrides_user_id: Option<String>,
    _request_metadata_user_overrides_ip_address: Option<String>,
    _request_metadata_traffic_source_traffic_sub_id: Option<String>,
    _request_metadata_traffic_source_traffic_source_id: Option<String>,
    _request_metadata_partners_session_id: Option<String>,
    _request_metadata_locale: Option<String>,
    _request_metadata_experiment_ids: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for UserStateListCall<'a, C, A> {}

impl<'a, C, A> UserStateListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListUserStatesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "partners.userStates.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
        if let Some(value) = self._request_metadata_user_overrides_user_id {
            params.push(("requestMetadata.userOverrides.userId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address {
            params.push(("requestMetadata.userOverrides.ipAddress", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id {
            params.push(("requestMetadata.trafficSource.trafficSubId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id {
            params.push(("requestMetadata.trafficSource.trafficSourceId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_partners_session_id {
            params.push(("requestMetadata.partnersSessionId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_locale {
            params.push(("requestMetadata.locale", value.to_string()));
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push(("requestMetadata.experimentIds", f.to_string()));
            }
        }
        for &field in ["alt", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/userStates";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }


        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> UserStateListCall<'a, C, A> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> UserStateListCall<'a, C, A> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> UserStateListCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> UserStateListCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> UserStateListCall<'a, C, A> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> UserStateListCall<'a, C, A> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> UserStateListCall<'a, C, A> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> UserStateListCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserStateListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Updates a user's profile. A user can only update their own profile and
/// should only be called within the context of a logged in user.
///
/// A builder for the *updateProfile* method supported by a *user* resource.
/// It is not used directly, but through a `UserMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_partners2 as partners2;
/// use partners2::UserProfile;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use partners2::Partners;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = UserProfile::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().update_profile(req)
///              .request_metadata_user_overrides_user_id("clita")
///              .request_metadata_user_overrides_ip_address("invidunt")
///              .request_metadata_traffic_source_traffic_sub_id("ut")
///              .request_metadata_traffic_source_traffic_source_id("dolores")
///              .request_metadata_partners_session_id("eos")
///              .request_metadata_locale("voluptua.")
///              .add_request_metadata_experiment_ids("duo")
///              .doit();
/// # }
/// ```
pub struct UserUpdateProfileCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
    _request: UserProfile,
    _request_metadata_user_overrides_user_id: Option<String>,
    _request_metadata_user_overrides_ip_address: Option<String>,
    _request_metadata_traffic_source_traffic_sub_id: Option<String>,
    _request_metadata_traffic_source_traffic_source_id: Option<String>,
    _request_metadata_partners_session_id: Option<String>,
    _request_metadata_locale: Option<String>,
    _request_metadata_experiment_ids: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for UserUpdateProfileCall<'a, C, A> {}

impl<'a, C, A> UserUpdateProfileCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, UserProfile)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "partners.users.updateProfile",
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(10 + self._additional_params.len());
        if let Some(value) = self._request_metadata_user_overrides_user_id {
            params.push(("requestMetadata.userOverrides.userId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address {
            params.push(("requestMetadata.userOverrides.ipAddress", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id {
            params.push(("requestMetadata.trafficSource.trafficSubId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id {
            params.push(("requestMetadata.trafficSource.trafficSourceId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_partners_session_id {
            params.push(("requestMetadata.partnersSessionId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_locale {
            params.push(("requestMetadata.locale", value.to_string()));
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push(("requestMetadata.experimentIds", f.to_string()));
            }
        }
        for &field in ["alt", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/users/profile";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }


        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: UserProfile) -> UserUpdateProfileCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> UserUpdateProfileCall<'a, C, A> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> UserUpdateProfileCall<'a, C, A> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> UserUpdateProfileCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> UserUpdateProfileCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> UserUpdateProfileCall<'a, C, A> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> UserUpdateProfileCall<'a, C, A> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> UserUpdateProfileCall<'a, C, A> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> UserUpdateProfileCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserUpdateProfileCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Creates a user's company relation. Affiliates the user to a company.
///
/// A builder for the *createCompanyRelation* method supported by a *user* resource.
/// It is not used directly, but through a `UserMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_partners2 as partners2;
/// use partners2::CompanyRelation;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use partners2::Partners;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CompanyRelation::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().create_company_relation(req, "userId")
///              .request_metadata_user_overrides_user_id("aliquyam")
///              .request_metadata_user_overrides_ip_address("ea")
///              .request_metadata_traffic_source_traffic_sub_id("ea")
///              .request_metadata_traffic_source_traffic_source_id("et")
///              .request_metadata_partners_session_id("dolor")
///              .request_metadata_locale("diam")
///              .add_request_metadata_experiment_ids("kasd")
///              .doit();
/// # }
/// ```
pub struct UserCreateCompanyRelationCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
    _request: CompanyRelation,
    _user_id: String,
    _request_metadata_user_overrides_user_id: Option<String>,
    _request_metadata_user_overrides_ip_address: Option<String>,
    _request_metadata_traffic_source_traffic_sub_id: Option<String>,
    _request_metadata_traffic_source_traffic_source_id: Option<String>,
    _request_metadata_partners_session_id: Option<String>,
    _request_metadata_locale: Option<String>,
    _request_metadata_experiment_ids: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for UserCreateCompanyRelationCall<'a, C, A> {}

impl<'a, C, A> UserCreateCompanyRelationCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, CompanyRelation)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "partners.users.createCompanyRelation",
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(11 + self._additional_params.len());
        params.push(("userId", self._user_id.to_string()));
        if let Some(value) = self._request_metadata_user_overrides_user_id {
            params.push(("requestMetadata.userOverrides.userId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address {
            params.push(("requestMetadata.userOverrides.ipAddress", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id {
            params.push(("requestMetadata.trafficSource.trafficSubId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id {
            params.push(("requestMetadata.trafficSource.trafficSourceId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_partners_session_id {
            params.push(("requestMetadata.partnersSessionId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_locale {
            params.push(("requestMetadata.locale", value.to_string()));
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push(("requestMetadata.experimentIds", f.to_string()));
            }
        }
        for &field in ["alt", "userId", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/users/{userId}/companyRelation";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{userId}", "userId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["userId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: CompanyRelation) -> UserCreateCompanyRelationCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the user. Can be set to <code>me</code> to mean
    /// the currently authenticated user.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> UserCreateCompanyRelationCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> UserCreateCompanyRelationCall<'a, C, A> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> UserCreateCompanyRelationCall<'a, C, A> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> UserCreateCompanyRelationCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> UserCreateCompanyRelationCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> UserCreateCompanyRelationCall<'a, C, A> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> UserCreateCompanyRelationCall<'a, C, A> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> UserCreateCompanyRelationCall<'a, C, A> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> UserCreateCompanyRelationCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserCreateCompanyRelationCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Gets a user.
///
/// A builder for the *get* method supported by a *user* resource.
/// It is not used directly, but through a `UserMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_partners2 as partners2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use partners2::Partners;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().get("userId")
///              .user_view("rebum.")
///              .request_metadata_user_overrides_user_id("Lorem")
///              .request_metadata_user_overrides_ip_address("clita")
///              .request_metadata_traffic_source_traffic_sub_id("invidunt")
///              .request_metadata_traffic_source_traffic_source_id("eirmod")
///              .request_metadata_partners_session_id("At")
///              .request_metadata_locale("consetetur")
///              .add_request_metadata_experiment_ids("et")
///              .doit();
/// # }
/// ```
pub struct UserGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
    _user_id: String,
    _user_view: Option<String>,
    _request_metadata_user_overrides_user_id: Option<String>,
    _request_metadata_user_overrides_ip_address: Option<String>,
    _request_metadata_traffic_source_traffic_sub_id: Option<String>,
    _request_metadata_traffic_source_traffic_source_id: Option<String>,
    _request_metadata_partners_session_id: Option<String>,
    _request_metadata_locale: Option<String>,
    _request_metadata_experiment_ids: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for UserGetCall<'a, C, A> {}

impl<'a, C, A> UserGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, User)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "partners.users.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(11 + self._additional_params.len());
        params.push(("userId", self._user_id.to_string()));
        if let Some(value) = self._user_view {
            params.push(("userView", value.to_string()));
        }
        if let Some(value) = self._request_metadata_user_overrides_user_id {
            params.push(("requestMetadata.userOverrides.userId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address {
            params.push(("requestMetadata.userOverrides.ipAddress", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id {
            params.push(("requestMetadata.trafficSource.trafficSubId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id {
            params.push(("requestMetadata.trafficSource.trafficSourceId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_partners_session_id {
            params.push(("requestMetadata.partnersSessionId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_locale {
            params.push(("requestMetadata.locale", value.to_string()));
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push(("requestMetadata.experimentIds", f.to_string()));
            }
        }
        for &field in ["alt", "userId", "userView", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/users/{userId}";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{userId}", "userId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["userId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Identifier of the user. Can be set to <code>me</code> to mean the currently
    /// authenticated user.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> UserGetCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// Specifies what parts of the user information to return.
    ///
    /// Sets the *user view* query property to the given value.
    pub fn user_view(mut self, new_value: &str) -> UserGetCall<'a, C, A> {
        self._user_view = Some(new_value.to_string());
        self
    }
    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> UserGetCall<'a, C, A> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> UserGetCall<'a, C, A> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> UserGetCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> UserGetCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> UserGetCall<'a, C, A> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> UserGetCall<'a, C, A> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> UserGetCall<'a, C, A> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> UserGetCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Deletes a user's company relation. Unaffiliaites the user from a company.
///
/// A builder for the *deleteCompanyRelation* method supported by a *user* resource.
/// It is not used directly, but through a `UserMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_partners2 as partners2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use partners2::Partners;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Partners::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().delete_company_relation("userId")
///              .request_metadata_user_overrides_user_id("sit")
///              .request_metadata_user_overrides_ip_address("takimata")
///              .request_metadata_traffic_source_traffic_sub_id("elitr")
///              .request_metadata_traffic_source_traffic_source_id("nonumy")
///              .request_metadata_partners_session_id("rebum.")
///              .request_metadata_locale("Lorem")
///              .add_request_metadata_experiment_ids("Lorem")
///              .doit();
/// # }
/// ```
pub struct UserDeleteCompanyRelationCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
    _user_id: String,
    _request_metadata_user_overrides_user_id: Option<String>,
    _request_metadata_user_overrides_ip_address: Option<String>,
    _request_metadata_traffic_source_traffic_sub_id: Option<String>,
    _request_metadata_traffic_source_traffic_source_id: Option<String>,
    _request_metadata_partners_session_id: Option<String>,
    _request_metadata_locale: Option<String>,
    _request_metadata_experiment_ids: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for UserDeleteCompanyRelationCall<'a, C, A> {}

impl<'a, C, A> UserDeleteCompanyRelationCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "partners.users.deleteCompanyRelation",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(10 + self._additional_params.len());
        params.push(("userId", self._user_id.to_string()));
        if let Some(value) = self._request_metadata_user_overrides_user_id {
            params.push(("requestMetadata.userOverrides.userId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address {
            params.push(("requestMetadata.userOverrides.ipAddress", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id {
            params.push(("requestMetadata.trafficSource.trafficSubId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id {
            params.push(("requestMetadata.trafficSource.trafficSourceId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_partners_session_id {
            params.push(("requestMetadata.partnersSessionId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_locale {
            params.push(("requestMetadata.locale", value.to_string()));
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push(("requestMetadata.experimentIds", f.to_string()));
            }
        }
        for &field in ["alt", "userId", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/users/{userId}/companyRelation";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{userId}", "userId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["userId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the user. Can be set to <code>me</code> to mean
    /// the currently authenticated user.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> UserDeleteCompanyRelationCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> UserDeleteCompanyRelationCall<'a, C, A> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> UserDeleteCompanyRelationCall<'a, C, A> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> UserDeleteCompanyRelationCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> UserDeleteCompanyRelationCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> UserDeleteCompanyRelationCall<'a, C, A> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> UserDeleteCompanyRelationCall<'a, C, A> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> UserDeleteCompanyRelationCall<'a, C, A> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> UserDeleteCompanyRelationCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserDeleteCompanyRelationCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


