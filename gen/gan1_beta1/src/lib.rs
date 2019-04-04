// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *gan* crate version *1.0.8+20130205*, where *20130205* is the exact revision of the *gan:v1beta1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.8*.
//! 
//! Everything else about the *gan* *v1_beta1* API can be found at the
//! [official documentation site](https://developers.google.com/affiliate-network/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/gan1_beta1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Gan.html) ... 
//! 
//! * [advertisers](struct.Advertiser.html)
//!  * [*get*](struct.AdvertiserGetCall.html) and [*list*](struct.AdvertiserListCall.html)
//! * [cc offers](struct.CcOffer.html)
//!  * [*list*](struct.CcOfferListCall.html)
//! * [events](struct.Event.html)
//!  * [*list*](struct.EventListCall.html)
//! * [links](struct.Link.html)
//!  * [*get*](struct.LinkGetCall.html), [*insert*](struct.LinkInsertCall.html) and [*list*](struct.LinkListCall.html)
//! * [publishers](struct.Publisher.html)
//!  * [*get*](struct.PublisherGetCall.html) and [*list*](struct.PublisherListCall.html)
//! * [reports](struct.Report.html)
//!  * [*get*](struct.ReportGetCall.html)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Gan.html)**
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
//! let r = hub.links().insert(...).doit()
//! let r = hub.links().get(...).doit()
//! let r = hub.links().list(...).doit()
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
//! google-gan1_beta1 = "*"
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
//! extern crate google_gan1_beta1 as gan1_beta1;
//! use gan1_beta1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use gan1_beta1::Gan;
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
//! let mut hub = Gan::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.links().list("role", "roleId")
//!              .start_date_min("sadipscing")
//!              .start_date_max("aliquyam")
//!              .search_text("ea")
//!              .relationship_status("no")
//!              .add_promotion_type("justo")
//!              .page_token("justo")
//!              .max_results(67)
//!              .link_type("et")
//!              .create_date_min("diam")
//!              .create_date_max("ipsum")
//!              .authorship("Lorem")
//!              .add_asset_size("et")
//!              .add_advertiser_id("duo")
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

/// Central instance to access all Gan related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_gan1_beta1 as gan1_beta1;
/// use gan1_beta1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use gan1_beta1::Gan;
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
/// let mut hub = Gan::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.links().list("role", "roleId")
///              .start_date_min("Lorem")
///              .start_date_max("eos")
///              .search_text("erat")
///              .relationship_status("sadipscing")
///              .add_promotion_type("dolor")
///              .page_token("eirmod")
///              .max_results(58)
///              .link_type("amet")
///              .create_date_min("no")
///              .create_date_max("labore")
///              .authorship("eirmod")
///              .add_asset_size("dolore")
///              .add_advertiser_id("invidunt")
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
pub struct Gan<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for Gan<C, A> {}

impl<'a, C, A> Gan<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Gan<C, A> {
        Gan {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.8".to_string(),
            _base_url: "https://www.googleapis.com/gan/v1beta1/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn advertisers(&'a self) -> AdvertiserMethods<'a, C, A> {
        AdvertiserMethods { hub: &self }
    }
    pub fn cc_offers(&'a self) -> CcOfferMethods<'a, C, A> {
        CcOfferMethods { hub: &self }
    }
    pub fn events(&'a self) -> EventMethods<'a, C, A> {
        EventMethods { hub: &self }
    }
    pub fn links(&'a self) -> LinkMethods<'a, C, A> {
        LinkMethods { hub: &self }
    }
    pub fn publishers(&'a self) -> PublisherMethods<'a, C, A> {
        PublisherMethods { hub: &self }
    }
    pub fn reports(&'a self) -> ReportMethods<'a, C, A> {
        ReportMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.8`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/gan/v1beta1/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// An AdvertiserResource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list advertisers](struct.AdvertiserListCall.html) (none)
/// * [get advertisers](struct.AdvertiserGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Advertiser {
    /// The status of the requesting publisher's relationship this advertiser.
    pub status: Option<String>,
    /// Allows advertisers to submit product listings to Google Product Search.
    #[serde(rename="productFeedsEnabled")]
    pub product_feeds_enabled: Option<bool>,
    /// Phone that this advertiser would like publishers to contact them with.
    #[serde(rename="contactPhone")]
    pub contact_phone: Option<String>,
    /// Description of the website the advertiser advertises from.
    pub description: Option<String>,
    /// List of merchant center ids for this advertiser
    #[serde(rename="merchantCenterIds")]
    pub merchant_center_ids: Option<Vec<String>>,
    /// The longest possible length of a commission (how long the cookies on the customer's browser last before they expire).
    #[serde(rename="commissionDuration")]
    pub commission_duration: Option<i32>,
    /// Email that this advertiser would like publishers to contact them with.
    #[serde(rename="contactEmail")]
    pub contact_email: Option<String>,
    /// Date that this advertiser was approved as a Google Affiliate Network advertiser.
    #[serde(rename="joinDate")]
    pub join_date: Option<String>,
    /// The ID of this advertiser.
    pub id: Option<String>,
    /// Category that this advertiser belongs to. A valid list of categories can be found here: http://www.google.com/support/affiliatenetwork/advertiser/bin/answer.py?hl=en&answer=107581
    pub category: Option<String>,
    /// The default link id for this advertiser.
    #[serde(rename="defaultLinkId")]
    pub default_link_id: Option<String>,
    /// The kind for an advertiser.
    pub kind: Option<String>,
    /// URL of the website this advertiser advertises from.
    #[serde(rename="siteUrl")]
    pub site_url: Option<String>,
    /// The name of this advertiser.
    pub name: Option<String>,
    /// A rank based on commissions paid to publishers over the past 90 days. A number between 1 and 4 where 4 means the top quartile (most money paid) and 1 means the bottom quartile (least money paid).
    #[serde(rename="payoutRank")]
    pub payout_rank: Option<String>,
    /// The sum of fees paid to publishers divided by the total number of clicks over the past seven days. This value should be multiplied by 100 at the time of display.
    #[serde(rename="epcSevenDayAverage")]
    pub epc_seven_day_average: Option<Money>,
    /// The sum of fees paid to publishers divided by the total number of clicks over the past three months. This value should be multiplied by 100 at the time of display.
    #[serde(rename="epcNinetyDayAverage")]
    pub epc_ninety_day_average: Option<Money>,
    /// True if the advertiser allows publisher created links, otherwise false.
    #[serde(rename="allowPublisherCreatedLinks")]
    pub allow_publisher_created_links: Option<bool>,
    /// The requested advertiser.
    pub item: Option<Option<Box<Advertiser>>>,
    /// URL to the logo this advertiser uses on the Google Affiliate Network.
    #[serde(rename="logoUrl")]
    pub logo_url: Option<String>,
    /// List of redirect URLs for this advertiser
    #[serde(rename="redirectDomains")]
    pub redirect_domains: Option<Vec<String>>,
}

impl Resource for Advertiser {}
impl ResponseResult for Advertiser {}


/// For cards with rewards programs, detailed rules about how the program works.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CcOfferRewards {
    /// The kind of purchases covered by this rule.
    pub category: Option<String>,
    /// The number of units rewarded per purchase dollar.
    pub amount: Option<f64>,
    /// The minimum purchase amount in the given category before this rule applies.
    #[serde(rename="minRewardTier")]
    pub min_reward_tier: Option<f64>,
    /// Other limits, for example, if this rule only applies during an introductory period.
    #[serde(rename="additionalDetails")]
    pub additional_details: Option<String>,
    /// The maximum purchase amount in the given category for this rule to apply.
    #[serde(rename="maxRewardTier")]
    pub max_reward_tier: Option<f64>,
    /// How long rewards granted by this rule last.
    #[serde(rename="expirationMonths")]
    pub expiration_months: Option<f64>,
}

impl NestedType for CcOfferRewards {}
impl Part for CcOfferRewards {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list links](struct.LinkListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Links {
    /// The next page token.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The links.
    pub items: Option<Vec<Link>>,
    /// The kind for a page of links.
    pub kind: Option<String>,
}

impl ResponseResult for Links {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list publishers](struct.PublisherListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Publishers {
    /// The 'pageToken' to pass to the next request to get the next page, if there are more to retrieve.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The entity list.
    pub items: Option<Vec<Publisher>>,
    /// The kind for a page of entities.
    pub kind: Option<String>,
}

impl ResponseResult for Publishers {}


/// An ApiMoneyProto.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Money {
    /// The amount of money.
    pub amount: Option<f64>,
    /// The 3-letter code of the currency in question.
    #[serde(rename="currencyCode")]
    pub currency_code: Option<String>,
}

impl Part for Money {}


/// A PublisherResource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list publishers](struct.PublisherListCall.html) (none)
/// * [get publishers](struct.PublisherGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Publisher {
    /// The status of the requesting advertiser's relationship with this publisher.
    pub status: Option<String>,
    /// The kind for a publisher.
    pub kind: Option<String>,
    /// The name of this publisher.
    pub name: Option<String>,
    /// Classification that this publisher belongs to. See this link for all publisher classifications: http://www.google.com/support/affiliatenetwork/advertiser/bin/answer.py?hl=en&answer=107625&ctx=cb&src=cb&cbid=-k5fihzthfaik&cbrank=4
    pub classification: Option<String>,
    /// The sum of fees paid to this publisher divided by the total number of clicks over the past seven days. Values are multiplied by 100 for display purposes.
    #[serde(rename="epcSevenDayAverage")]
    pub epc_seven_day_average: Option<Money>,
    /// Websites that this publisher uses to advertise.
    pub sites: Option<Vec<String>>,
    /// The sum of fees paid to this publisher divided by the total number of clicks over the past three months. Values are multiplied by 100 for display purposes.
    #[serde(rename="epcNinetyDayAverage")]
    pub epc_ninety_day_average: Option<Money>,
    /// The requested publisher.
    pub item: Option<Option<Box<Publisher>>>,
    /// A rank based on commissions paid to this publisher over the past 90 days. A number between 1 and 4 where 4 means the top quartile (most money paid) and 1 means the bottom quartile (least money paid).
    #[serde(rename="payoutRank")]
    pub payout_rank: Option<String>,
    /// Date that this publisher was approved as a Google Affiliate Network publisher.
    #[serde(rename="joinDate")]
    pub join_date: Option<String>,
    /// The ID of this publisher.
    pub id: Option<String>,
}

impl Resource for Publisher {}
impl ResponseResult for Publisher {}


/// Special offers on the link.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LinkSpecialOffers {
    /// Price cut on the purchase
    #[serde(rename="priceCut")]
    pub price_cut: Option<Money>,
    /// Minimum purchase amount for price cut promotion
    #[serde(rename="priceCutMin")]
    pub price_cut_min: Option<Money>,
    /// Whether there is free shipping
    #[serde(rename="freeShipping")]
    pub free_shipping: Option<bool>,
    /// List of promotion code associated with the link
    #[serde(rename="promotionCodes")]
    pub promotion_codes: Option<Vec<String>>,
    /// Percent off on the purchase
    #[serde(rename="percentOff")]
    pub percent_off: Option<f64>,
    /// Minimum purchase amount for percent off promotion
    #[serde(rename="percentOffMin")]
    pub percent_off_min: Option<Money>,
    /// Whether there is a free gift
    #[serde(rename="freeGift")]
    pub free_gift: Option<bool>,
    /// Minimum purchase amount for free shipping promotion
    #[serde(rename="freeShippingMin")]
    pub free_shipping_min: Option<Money>,
}

impl NestedType for LinkSpecialOffers {}
impl Part for LinkSpecialOffers {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list events](struct.EventListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Events {
    /// The 'pageToken' to pass to the next request to get the next page, if there are more to retrieve.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The event list.
    pub items: Option<Vec<Event>>,
    /// The kind for a page of events.
    pub kind: Option<String>,
}

impl ResponseResult for Events {}


/// For cards with rewards programs, extra circumstances whereby additional rewards may be granted.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CcOfferBonusRewards {
    /// How many units of reward will be granted.
    pub amount: Option<f64>,
    /// The circumstances under which this rule applies, for example, booking a flight via Orbitz.
    pub details: Option<String>,
}

impl NestedType for CcOfferBonusRewards {}
impl Part for CcOfferBonusRewards {}


/// Fees for defaulting on your payments.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CcOfferDefaultFees {
    /// The type of charge, for example Purchases.
    pub category: Option<String>,
    /// The highest rate the issuer may charge for defaulting on debt in this category. Expressed as an absolute number, not as a percentage.
    #[serde(rename="maxRate")]
    pub max_rate: Option<f64>,
    /// The lowest rate the issuer may charge for defaulting on debt in this category. Expressed as an absolute number, not as a percentage.
    #[serde(rename="minRate")]
    pub min_rate: Option<f64>,
    /// Fixed or variable.
    #[serde(rename="rateType")]
    pub rate_type: Option<String>,
}

impl NestedType for CcOfferDefaultFees {}
impl Part for CcOfferDefaultFees {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list cc offers](struct.CcOfferListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CcOffers {
    /// The credit card offers.
    pub items: Option<Vec<CcOffer>>,
    /// The kind for a page of credit card offers.
    pub kind: Option<String>,
}

impl ResponseResult for CcOffers {}


/// A LinkResource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert links](struct.LinkInsertCall.html) (request|response)
/// * [get links](struct.LinkGetCall.html) (response)
/// * [list links](struct.LinkListCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    /// The link type.
    #[serde(rename="linkType")]
    pub link_type: Option<String>,
    /// Date that this link becomes active.
    #[serde(rename="startDate")]
    pub start_date: Option<String>,
    /// Date that this link becomes inactive.
    #[serde(rename="endDate")]
    pub end_date: Option<String>,
    /// Description.
    pub description: Option<String>,
    /// Availability.
    pub availability: Option<String>,
    /// image alt text.
    #[serde(rename="imageAltText")]
    pub image_alt_text: Option<String>,
    /// The advertiser id for the advertiser who owns this link.
    #[serde(rename="advertiserId")]
    pub advertiser_id: Option<String>,
    /// Tracking url for clicks.
    #[serde(rename="clickTrackingUrl")]
    pub click_tracking_url: Option<String>,
    /// Promotion Type
    #[serde(rename="promotionType")]
    pub promotion_type: Option<String>,
    /// Duration
    pub duration: Option<String>,
    /// Authorship
    pub authorship: Option<String>,
    /// The ID of this link.
    pub id: Option<String>,
    /// Flag for if this link is active.
    #[serde(rename="isActive")]
    pub is_active: Option<bool>,
    /// The destination URL for the link.
    #[serde(rename="destinationUrl")]
    pub destination_url: Option<String>,
    /// Special offers on the link.
    #[serde(rename="specialOffers")]
    pub special_offers: Option<LinkSpecialOffers>,
    /// The kind for one entity.
    pub kind: Option<String>,
    /// The logical name for this link.
    pub name: Option<String>,
    /// The sum of fees paid to publishers divided by the total number of clicks over the past seven days on this link. This value should be multiplied by 100 at the time of display.
    #[serde(rename="epcSevenDayAverage")]
    pub epc_seven_day_average: Option<Money>,
    /// Date that this link was created.
    #[serde(rename="createDate")]
    pub create_date: Option<String>,
    /// The sum of fees paid to publishers divided by the total number of clicks over the past three months on this link. This value should be multiplied by 100 at the time of display.
    #[serde(rename="epcNinetyDayAverage")]
    pub epc_ninety_day_average: Option<Money>,
    /// Tracking url for impressions.
    #[serde(rename="impressionTrackingUrl")]
    pub impression_tracking_url: Option<String>,
}

impl RequestValue for Link {}
impl Resource for Link {}
impl ResponseResult for Link {}


/// Products associated with the event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventProducts {
    /// Fee that the advertiser paid to the Google Affiliate Network for this product.
    #[serde(rename="networkFee")]
    pub network_fee: Option<Money>,
    /// Sku of this product.
    pub sku: Option<String>,
    /// Name of the category this product belongs to.
    #[serde(rename="categoryName")]
    pub category_name: Option<String>,
    /// Sku name of this product.
    #[serde(rename="skuName")]
    pub sku_name: Option<String>,
    /// Fee that the advertiser paid to the publisehr for this product.
    #[serde(rename="publisherFee")]
    pub publisher_fee: Option<Money>,
    /// Amount earned by the publisher on this product.
    pub earnings: Option<Money>,
    /// Price per unit of this product.
    #[serde(rename="unitPrice")]
    pub unit_price: Option<Money>,
    /// Id of the category this product belongs to.
    #[serde(rename="categoryId")]
    pub category_id: Option<String>,
    /// Quantity of this product bought/exchanged.
    pub quantity: Option<String>,
}

impl NestedType for EventProducts {}
impl Part for EventProducts {}


/// A ReportResource representing a report of a certain type either for an advertiser or publisher.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get reports](struct.ReportGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Report {
    /// The totals rows for the report
    pub totals_rows: Option<Vec<Vec<String>>>,
    /// The kind for a report.
    pub kind: Option<String>,
    /// The rows of data for the report
    pub rows: Option<Vec<Vec<String>>>,
    /// The end of the date range for this report, exclusive.
    pub end_date: Option<String>,
    /// The number of matching rows before paging is applied.
    pub matching_row_count: Option<String>,
    /// The column names for the report
    pub column_names: Option<Vec<String>>,
    /// The report type.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The start of the date range for this report, inclusive.
    pub start_date: Option<String>,
}

impl Resource for Report {}
impl ResponseResult for Report {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list advertisers](struct.AdvertiserListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Advertisers {
    /// The 'pageToken' to pass to the next request to get the next page, if there are more to retrieve.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The advertiser list.
    pub items: Option<Vec<Advertiser>>,
    /// The kind for a page of advertisers.
    pub kind: Option<String>,
}

impl ResponseResult for Advertisers {}


/// An EventResource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list events](struct.EventListCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    /// The order ID for this event. Only returned for conversion events.
    #[serde(rename="orderId")]
    pub order_id: Option<String>,
    /// Status of the event (active|canceled). Only returned for charge and conversion events.
    pub status: Option<String>,
    /// The date-time this event was last modified as a RFC 3339 date-time value.
    #[serde(rename="modifyDate")]
    pub modify_date: Option<String>,
    /// The name of the advertiser for this event.
    #[serde(rename="advertiserName")]
    pub advertiser_name: Option<String>,
    /// The charge ID for this event. Only returned for charge events.
    #[serde(rename="chargeId")]
    pub charge_id: Option<String>,
    /// Charge type of the event (other|slotting_fee|monthly_minimum|tier_bonus|debit|credit). Only returned for charge events.
    #[serde(rename="chargeType")]
    pub charge_type: Option<String>,
    /// The name of the publisher for this event.
    #[serde(rename="publisherName")]
    pub publisher_name: Option<String>,
    /// The date-time this event was initiated as a RFC 3339 date-time value.
    #[serde(rename="eventDate")]
    pub event_date: Option<String>,
    /// Fee that the advertiser paid to the Google Affiliate Network.
    #[serde(rename="networkFee")]
    pub network_fee: Option<Money>,
    /// The ID of advertiser for this event.
    #[serde(rename="advertiserId")]
    pub advertiser_id: Option<String>,
    /// The kind for one event.
    pub kind: Option<String>,
    /// The ID of the publisher for this event.
    #[serde(rename="publisherId")]
    pub publisher_id: Option<String>,
    /// The ID of the member attached to this event. Only returned for conversion events.
    #[serde(rename="memberId")]
    pub member_id: Option<String>,
    /// Fee that the advertiser paid to the publisher.
    #[serde(rename="publisherFee")]
    pub publisher_fee: Option<Money>,
    /// Earnings by the publisher.
    pub earnings: Option<Money>,
    /// Products associated with the event.
    pub products: Option<Vec<EventProducts>>,
    /// Amount of money exchanged during the transaction. Only returned for charge and conversion events.
    #[serde(rename="commissionableSales")]
    pub commissionable_sales: Option<Money>,
    /// Type of the event (action|transaction|charge).
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Resource for Event {}


/// A credit card offer. There are many possible result fields. We provide two different views of the data, or "projections." The "full" projection includes every result field. And the "summary" projection, which is the default, includes a smaller subset of the fields. The fields included in the summary projection are marked as such in their descriptions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list cc offers](struct.CcOfferListCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CcOffer {
    /// If you get coverage when you use the card for the given activity, this field describes it.
    #[serde(rename="luggageInsurance")]
    pub luggage_insurance: Option<String>,
    /// The low end for credit limits the issuer imposes on recipients of this card.
    #[serde(rename="creditLimitMin")]
    pub credit_limit_min: Option<f64>,
    /// The issuer's name for the card, including any trademark or service mark designators. A summary field.
    #[serde(rename="cardName")]
    pub card_name: Option<String>,
    /// The high end for credit limits the issuer imposes on recipients of this card.
    #[serde(rename="creditLimitMax")]
    pub credit_limit_max: Option<f64>,
    /// Text describing the grace period before finance charges apply. A summary field.
    #[serde(rename="gracePeriodDisplay")]
    pub grace_period_display: Option<String>,
    /// Any extra fees levied on card holders.
    #[serde(rename="additionalCardHolderFee")]
    pub additional_card_holder_fee: Option<String>,
    /// This offer's ID. A summary field.
    #[serde(rename="offerId")]
    pub offer_id: Option<String>,
    /// The lowest interest rate the issuer charges on this card. Expressed as an absolute number, not as a percentage.
    #[serde(rename="minPurchaseRate")]
    pub min_purchase_rate: Option<f64>,
    /// A list of what the issuer thinks are the most important benefits of the card. Usually summarizes the rewards program, if there is one. A summary field.
    #[serde(rename="cardBenefits")]
    pub card_benefits: Option<Vec<String>>,
    /// Fees for defaulting on your payments.
    #[serde(rename="defaultFees")]
    pub default_fees: Option<Vec<CcOfferDefaultFees>>,
    /// Whether a cash reward program lets you get cash back sooner than end of year or other longish period.
    #[serde(rename="offersImmediateCashReward")]
    pub offers_immediate_cash_reward: Option<bool>,
    /// If you get coverage when you use the card for the given activity, this field describes it.
    #[serde(rename="travelInsurance")]
    pub travel_insurance: Option<String>,
    /// Text describing the fee for a payment that doesn't clear. A summary field.
    #[serde(rename="returnedPaymentFee")]
    pub returned_payment_fee: Option<String>,
    /// Name of card issuer. A summary field.
    pub issuer: Option<String>,
    /// Whether this card is only available to existing customers of the issuer.
    #[serde(rename="existingCustomerOnly")]
    pub existing_customer_only: Option<bool>,
    /// Text describing the annual fee, including any difference for the first year. A summary field.
    #[serde(rename="annualFeeDisplay")]
    pub annual_fee_display: Option<String>,
    /// Fee for setting up the card.
    #[serde(rename="initialSetupAndProcessingFee")]
    pub initial_setup_and_processing_fee: Option<String>,
    /// Text describing how much missing the grace period will cost.
    #[serde(rename="minimumFinanceCharge")]
    pub minimum_finance_charge: Option<String>,
    /// Text describing any additional details for the purchase rate. A summary field.
    #[serde(rename="purchaseRateAdditionalDetails")]
    pub purchase_rate_additional_details: Option<String>,
    /// For cards with rewards programs, the unit of reward. For example, miles, cash back, points.
    #[serde(rename="rewardUnit")]
    pub reward_unit: Option<String>,
    /// Text describing the credit ratings required for recipients of this card, for example "Excellent/Good." A summary field.
    #[serde(rename="creditRatingDisplay")]
    pub credit_rating_display: Option<String>,
    /// If you get coverage when you use the card for the given activity, this field describes it.
    #[serde(rename="fraudLiability")]
    pub fraud_liability: Option<String>,
    /// Text describing the terms for cash advances. A summary field.
    #[serde(rename="cashAdvanceTerms")]
    pub cash_advance_terms: Option<String>,
    /// The link to the issuer's page for this card. A summary field.
    #[serde(rename="landingPageUrl")]
    pub landing_page_url: Option<String>,
    /// Text describing the terms for introductory period cash advances. A summary field.
    #[serde(rename="introCashAdvanceTerms")]
    pub intro_cash_advance_terms: Option<String>,
    /// The annual fee for the first year, if different from the ongoing fee. Optional.
    #[serde(rename="firstYearAnnualFee")]
    pub first_year_annual_fee: Option<f64>,
    /// Text describing the terms for introductory period purchases. A summary field.
    #[serde(rename="introPurchaseTerms")]
    pub intro_purchase_terms: Option<String>,
    /// For cards with rewards programs, detailed rules about how the program works.
    pub rewards: Option<Vec<CcOfferRewards>>,
    /// If you get coverage when you use the card for the given activity, this field describes it.
    #[serde(rename="extendedWarranty")]
    pub extended_warranty: Option<String>,
    /// The highest interest rate the issuer charges on this card. Expressed as an absolute number, not as a percentage.
    #[serde(rename="maxPurchaseRate")]
    pub max_purchase_rate: Option<f64>,
    /// The link to ping to register a click on this offer. A summary field.
    #[serde(rename="trackingUrl")]
    pub tracking_url: Option<String>,
    /// Text describing how much a late payment will cost, eg "up to $35." A summary field.
    #[serde(rename="latePaymentFee")]
    pub late_payment_fee: Option<String>,
    /// Fee for exceeding the card's charge limit.
    #[serde(rename="overLimitFee")]
    pub over_limit_fee: Option<String>,
    /// What kind of credit card this is, for example secured or unsecured.
    #[serde(rename="cardType")]
    pub card_type: Option<String>,
    /// Possible categories for this card, eg "Low Interest" or "Good." A summary field.
    #[serde(rename="approvedCategories")]
    pub approved_categories: Option<Vec<String>>,
    /// The company that redeems the rewards, if different from the issuer.
    #[serde(rename="rewardPartner")]
    pub reward_partner: Option<String>,
    /// Text describing the terms for introductory period balance transfers. A summary field.
    #[serde(rename="introBalanceTransferTerms")]
    pub intro_balance_transfer_terms: Option<String>,
    /// The largest number of units you may accumulate in a year.
    #[serde(rename="annualRewardMaximum")]
    pub annual_reward_maximum: Option<f64>,
    /// Fee for each transaction involving a foreign currency.
    #[serde(rename="foreignCurrencyTransactionFee")]
    pub foreign_currency_transaction_fee: Option<String>,
    /// The ongoing annual fee, in dollars.
    #[serde(rename="annualFee")]
    pub annual_fee: Option<f64>,
    /// If you get coverage when you use the card for the given activity, this field describes it.
    #[serde(rename="carRentalInsurance")]
    pub car_rental_insurance: Option<String>,
    /// How often variable rates are updated.
    #[serde(rename="variableRatesUpdateFrequency")]
    pub variable_rates_update_frequency: Option<String>,
    /// The kind for one credit card offer. A summary field.
    pub kind: Option<String>,
    /// More marketing copy about the card's benefits. A summary field.
    #[serde(rename="additionalCardBenefits")]
    pub additional_card_benefits: Option<Vec<String>>,
    /// The generic link to the issuer's site.
    #[serde(rename="issuerWebsite")]
    pub issuer_website: Option<String>,
    /// Text describing how the balance is computed. A summary field.
    #[serde(rename="balanceComputationMethod")]
    pub balance_computation_method: Option<String>,
    /// Text describing the purchase APR. A summary field.
    #[serde(rename="aprDisplay")]
    pub apr_display: Option<String>,
    /// The link to the image of the card that is shown on Connect Commerce. A summary field.
    #[serde(rename="imageUrl")]
    pub image_url: Option<String>,
    /// Categories in which the issuer does not wish the card to be displayed. A summary field.
    #[serde(rename="prohibitedCategories")]
    pub prohibited_categories: Option<Vec<String>>,
    /// When variable rates were last updated.
    #[serde(rename="variableRatesLastUpdated")]
    pub variable_rates_last_updated: Option<String>,
    /// The youngest a recipient of this card may be.
    #[serde(rename="ageMinimum")]
    pub age_minimum: Option<f64>,
    /// Which network (eg Visa) the card belongs to. A summary field.
    pub network: Option<String>,
    /// Fixed or variable.
    #[serde(rename="purchaseRateType")]
    pub purchase_rate_type: Option<String>,
    /// Fee for requesting a copy of your statement.
    #[serde(rename="statementCopyFee")]
    pub statement_copy_fee: Option<String>,
    /// For airline miles rewards, tells whether blackout dates apply to the miles.
    #[serde(rename="rewardsHaveBlackoutDates")]
    pub rewards_have_blackout_dates: Option<bool>,
    /// If you get coverage when you use the card for the given activity, this field describes it.
    #[serde(rename="emergencyInsurance")]
    pub emergency_insurance: Option<String>,
    /// If you get coverage when you use the card for the given activity, this field describes it.
    #[serde(rename="flightAccidentInsurance")]
    pub flight_accident_insurance: Option<String>,
    /// Whether accumulated rewards ever expire.
    #[serde(rename="rewardsExpire")]
    pub rewards_expire: Option<bool>,
    /// Text describing the terms for balance transfers. A summary field.
    #[serde(rename="balanceTransferTerms")]
    pub balance_transfer_terms: Option<String>,
    /// A notice that, if present, is referenced via an asterisk by many of the other summary fields. If this field is present, it will always start with an asterisk ("*"), and must be prominently displayed with the offer. A summary field.
    pub disclaimer: Option<String>,
    /// The Google Affiliate Network ID of the advertiser making this offer.
    #[serde(rename="issuerId")]
    pub issuer_id: Option<String>,
    /// Text describing the details of the age minimum restriction.
    #[serde(rename="ageMinimumDetails")]
    pub age_minimum_details: Option<String>,
    /// For cards with rewards programs, extra circumstances whereby additional rewards may be granted.
    #[serde(rename="bonusRewards")]
    pub bonus_rewards: Option<Vec<CcOfferBonusRewards>>,
}

impl Resource for CcOffer {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *publisher* resources.
/// It is not used directly, but through the `Gan` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_gan1_beta1 as gan1_beta1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use gan1_beta1::Gan;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Gan::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.publishers();
/// # }
/// ```
pub struct PublisherMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Gan<C, A>,
}

impl<'a, C, A> MethodsBuilder for PublisherMethods<'a, C, A> {}

impl<'a, C, A> PublisherMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves data about all publishers that the requesting advertiser/publisher has access to.
    /// 
    /// # Arguments
    ///
    /// * `role` - The role of the requester. Valid values: 'advertisers' or 'publishers'.
    /// * `roleId` - The ID of the requesting advertiser or publisher.
    pub fn list(&self, role: &str, role_id: &str) -> PublisherListCall<'a, C, A> {
        PublisherListCall {
            hub: self.hub,
            _role: role.to_string(),
            _role_id: role_id.to_string(),
            _relationship_status: Default::default(),
            _publisher_category: Default::default(),
            _page_token: Default::default(),
            _min_seven_day_epc: Default::default(),
            _min_payout_rank: Default::default(),
            _min_ninety_day_epc: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves data about a single advertiser if that the requesting advertiser/publisher has access to it. Only advertisers can look up publishers. Publishers can request information about themselves by omitting the publisherId query parameter.
    /// 
    /// # Arguments
    ///
    /// * `role` - The role of the requester. Valid values: 'advertisers' or 'publishers'.
    /// * `roleId` - The ID of the requesting advertiser or publisher.
    pub fn get(&self, role: &str, role_id: &str) -> PublisherGetCall<'a, C, A> {
        PublisherGetCall {
            hub: self.hub,
            _role: role.to_string(),
            _role_id: role_id.to_string(),
            _publisher_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *link* resources.
/// It is not used directly, but through the `Gan` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_gan1_beta1 as gan1_beta1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use gan1_beta1::Gan;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Gan::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.links();
/// # }
/// ```
pub struct LinkMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Gan<C, A>,
}

impl<'a, C, A> MethodsBuilder for LinkMethods<'a, C, A> {}

impl<'a, C, A> LinkMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new link.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `role` - The role of the requester. Valid values: 'advertisers' or 'publishers'.
    /// * `roleId` - The ID of the requesting advertiser or publisher.
    pub fn insert(&self, request: Link, role: &str, role_id: &str) -> LinkInsertCall<'a, C, A> {
        LinkInsertCall {
            hub: self.hub,
            _request: request,
            _role: role.to_string(),
            _role_id: role_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves data about a single link if the requesting advertiser/publisher has access to it. Advertisers can look up their own links. Publishers can look up visible links or links belonging to advertisers they are in a relationship with.
    /// 
    /// # Arguments
    ///
    /// * `role` - The role of the requester. Valid values: 'advertisers' or 'publishers'.
    /// * `roleId` - The ID of the requesting advertiser or publisher.
    /// * `linkId` - The ID of the link to look up.
    pub fn get(&self, role: &str, role_id: &str, link_id: &str) -> LinkGetCall<'a, C, A> {
        LinkGetCall {
            hub: self.hub,
            _role: role.to_string(),
            _role_id: role_id.to_string(),
            _link_id: link_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves all links that match the query parameters.
    /// 
    /// # Arguments
    ///
    /// * `role` - The role of the requester. Valid values: 'advertisers' or 'publishers'.
    /// * `roleId` - The ID of the requesting advertiser or publisher.
    pub fn list(&self, role: &str, role_id: &str) -> LinkListCall<'a, C, A> {
        LinkListCall {
            hub: self.hub,
            _role: role.to_string(),
            _role_id: role_id.to_string(),
            _start_date_min: Default::default(),
            _start_date_max: Default::default(),
            _search_text: Default::default(),
            _relationship_status: Default::default(),
            _promotion_type: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _link_type: Default::default(),
            _create_date_min: Default::default(),
            _create_date_max: Default::default(),
            _authorship: Default::default(),
            _asset_size: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *report* resources.
/// It is not used directly, but through the `Gan` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_gan1_beta1 as gan1_beta1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use gan1_beta1::Gan;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Gan::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.reports();
/// # }
/// ```
pub struct ReportMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Gan<C, A>,
}

impl<'a, C, A> MethodsBuilder for ReportMethods<'a, C, A> {}

impl<'a, C, A> ReportMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a report of the specified type.
    /// 
    /// # Arguments
    ///
    /// * `role` - The role of the requester. Valid values: 'advertisers' or 'publishers'.
    /// * `roleId` - The ID of the requesting advertiser or publisher.
    /// * `reportType` - The type of report being requested. Valid values: 'order_delta'. Required.
    pub fn get(&self, role: &str, role_id: &str, report_type: &str) -> ReportGetCall<'a, C, A> {
        ReportGetCall {
            hub: self.hub,
            _role: role.to_string(),
            _role_id: role_id.to_string(),
            _report_type: report_type.to_string(),
            _status: Default::default(),
            _start_index: Default::default(),
            _start_date: Default::default(),
            _publisher_id: Default::default(),
            _order_id: Default::default(),
            _max_results: Default::default(),
            _link_id: Default::default(),
            _event_type: Default::default(),
            _end_date: Default::default(),
            _calculate_totals: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *ccOffer* resources.
/// It is not used directly, but through the `Gan` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_gan1_beta1 as gan1_beta1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use gan1_beta1::Gan;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Gan::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.cc_offers();
/// # }
/// ```
pub struct CcOfferMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Gan<C, A>,
}

impl<'a, C, A> MethodsBuilder for CcOfferMethods<'a, C, A> {}

impl<'a, C, A> CcOfferMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves credit card offers for the given publisher.
    /// 
    /// # Arguments
    ///
    /// * `publisher` - The ID of the publisher in question.
    pub fn list(&self, publisher: &str) -> CcOfferListCall<'a, C, A> {
        CcOfferListCall {
            hub: self.hub,
            _publisher: publisher.to_string(),
            _projection: Default::default(),
            _advertiser: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *advertiser* resources.
/// It is not used directly, but through the `Gan` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_gan1_beta1 as gan1_beta1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use gan1_beta1::Gan;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Gan::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.advertisers();
/// # }
/// ```
pub struct AdvertiserMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Gan<C, A>,
}

impl<'a, C, A> MethodsBuilder for AdvertiserMethods<'a, C, A> {}

impl<'a, C, A> AdvertiserMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves data about a single advertiser if that the requesting advertiser/publisher has access to it. Only publishers can lookup advertisers. Advertisers can request information about themselves by omitting the advertiserId query parameter.
    /// 
    /// # Arguments
    ///
    /// * `role` - The role of the requester. Valid values: 'advertisers' or 'publishers'.
    /// * `roleId` - The ID of the requesting advertiser or publisher.
    pub fn get(&self, role: &str, role_id: &str) -> AdvertiserGetCall<'a, C, A> {
        AdvertiserGetCall {
            hub: self.hub,
            _role: role.to_string(),
            _role_id: role_id.to_string(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves data about all advertisers that the requesting advertiser/publisher has access to.
    /// 
    /// # Arguments
    ///
    /// * `role` - The role of the requester. Valid values: 'advertisers' or 'publishers'.
    /// * `roleId` - The ID of the requesting advertiser or publisher.
    pub fn list(&self, role: &str, role_id: &str) -> AdvertiserListCall<'a, C, A> {
        AdvertiserListCall {
            hub: self.hub,
            _role: role.to_string(),
            _role_id: role_id.to_string(),
            _relationship_status: Default::default(),
            _page_token: Default::default(),
            _min_seven_day_epc: Default::default(),
            _min_payout_rank: Default::default(),
            _min_ninety_day_epc: Default::default(),
            _max_results: Default::default(),
            _advertiser_category: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *event* resources.
/// It is not used directly, but through the `Gan` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_gan1_beta1 as gan1_beta1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use gan1_beta1::Gan;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Gan::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.events();
/// # }
/// ```
pub struct EventMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Gan<C, A>,
}

impl<'a, C, A> MethodsBuilder for EventMethods<'a, C, A> {}

impl<'a, C, A> EventMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves event data for a given advertiser/publisher.
    /// 
    /// # Arguments
    ///
    /// * `role` - The role of the requester. Valid values: 'advertisers' or 'publishers'.
    /// * `roleId` - The ID of the requesting advertiser or publisher.
    pub fn list(&self, role: &str, role_id: &str) -> EventListCall<'a, C, A> {
        EventListCall {
            hub: self.hub,
            _role: role.to_string(),
            _role_id: role_id.to_string(),
            _type_: Default::default(),
            _status: Default::default(),
            _sku: Default::default(),
            _publisher_id: Default::default(),
            _product_category: Default::default(),
            _page_token: Default::default(),
            _order_id: Default::default(),
            _modify_date_min: Default::default(),
            _modify_date_max: Default::default(),
            _member_id: Default::default(),
            _max_results: Default::default(),
            _link_id: Default::default(),
            _event_date_min: Default::default(),
            _event_date_max: Default::default(),
            _charge_type: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Retrieves data about all publishers that the requesting advertiser/publisher has access to.
///
/// A builder for the *list* method supported by a *publisher* resource.
/// It is not used directly, but through a `PublisherMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_gan1_beta1 as gan1_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use gan1_beta1::Gan;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Gan::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.publishers().list("role", "roleId")
///              .relationship_status("Lorem")
///              .publisher_category("sea")
///              .page_token("et")
///              .min_seven_day_epc(0.314282909129)
///              .min_payout_rank(-21)
///              .min_ninety_day_epc(0.610793242854)
///              .max_results(43)
///              .doit();
/// # }
/// ```
pub struct PublisherListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Gan<C, A>,
    _role: String,
    _role_id: String,
    _relationship_status: Option<String>,
    _publisher_category: Option<String>,
    _page_token: Option<String>,
    _min_seven_day_epc: Option<f64>,
    _min_payout_rank: Option<i32>,
    _min_ninety_day_epc: Option<f64>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for PublisherListCall<'a, C, A> {}

impl<'a, C, A> PublisherListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Publishers)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "gan.publishers.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(11 + self._additional_params.len());
        params.push(("role", self._role.to_string()));
        params.push(("roleId", self._role_id.to_string()));
        if let Some(value) = self._relationship_status {
            params.push(("relationshipStatus", value.to_string()));
        }
        if let Some(value) = self._publisher_category {
            params.push(("publisherCategory", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._min_seven_day_epc {
            params.push(("minSevenDayEpc", value.to_string()));
        }
        if let Some(value) = self._min_payout_rank {
            params.push(("minPayoutRank", value.to_string()));
        }
        if let Some(value) = self._min_ninety_day_epc {
            params.push(("minNinetyDayEpc", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        for &field in ["alt", "role", "roleId", "relationshipStatus", "publisherCategory", "pageToken", "minSevenDayEpc", "minPayoutRank", "minNinetyDayEpc", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{role}/{roleId}/publishers";
        
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

        for &(find_this, param_name) in [("{role}", "role"), ("{roleId}", "roleId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["roleId", "role"].iter() {
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


    /// The role of the requester. Valid values: 'advertisers' or 'publishers'.
    ///
    /// Sets the *role* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role(mut self, new_value: &str) -> PublisherListCall<'a, C, A> {
        self._role = new_value.to_string();
        self
    }
    /// The ID of the requesting advertiser or publisher.
    ///
    /// Sets the *role id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role_id(mut self, new_value: &str) -> PublisherListCall<'a, C, A> {
        self._role_id = new_value.to_string();
        self
    }
    /// Filters out all publishers for which do not have the given relationship status with the requesting publisher.
    ///
    /// Sets the *relationship status* query property to the given value.
    pub fn relationship_status(mut self, new_value: &str) -> PublisherListCall<'a, C, A> {
        self._relationship_status = Some(new_value.to_string());
        self
    }
    /// Caret(^) delimted list of publisher categories. Valid categories: (unclassified|community_and_content|shopping_and_promotion|loyalty_and_rewards|network|search_specialist|comparison_shopping|email). Filters out all publishers not in one of the given advertiser categories. Optional.
    ///
    /// Sets the *publisher category* query property to the given value.
    pub fn publisher_category(mut self, new_value: &str) -> PublisherListCall<'a, C, A> {
        self._publisher_category = Some(new_value.to_string());
        self
    }
    /// The value of 'nextPageToken' from the previous page. Optional.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> PublisherListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Filters out all publishers that have a seven day EPC average lower than the given value (inclusive). Min value 0.0. Optional.
    ///
    /// Sets the *min seven day epc* query property to the given value.
    pub fn min_seven_day_epc(mut self, new_value: f64) -> PublisherListCall<'a, C, A> {
        self._min_seven_day_epc = Some(new_value);
        self
    }
    /// A value between 1 and 4, where 1 represents the quartile of publishers with the lowest ranks and 4 represents the quartile of publishers with the highest ranks. Filters out all publishers with a lower rank than the given quartile. For example if a 2 was given only publishers with a payout rank of 25 or higher would be included. Optional.
    ///
    /// Sets the *min payout rank* query property to the given value.
    pub fn min_payout_rank(mut self, new_value: i32) -> PublisherListCall<'a, C, A> {
        self._min_payout_rank = Some(new_value);
        self
    }
    /// Filters out all publishers that have a ninety day EPC average lower than the given value (inclusive). Min value: 0.0. Optional.
    ///
    /// Sets the *min ninety day epc* query property to the given value.
    pub fn min_ninety_day_epc(mut self, new_value: f64) -> PublisherListCall<'a, C, A> {
        self._min_ninety_day_epc = Some(new_value);
        self
    }
    /// Max number of items to return in this page. Optional. Defaults to 20.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> PublisherListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PublisherListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PublisherListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Retrieves data about a single advertiser if that the requesting advertiser/publisher has access to it. Only advertisers can look up publishers. Publishers can request information about themselves by omitting the publisherId query parameter.
///
/// A builder for the *get* method supported by a *publisher* resource.
/// It is not used directly, but through a `PublisherMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_gan1_beta1 as gan1_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use gan1_beta1::Gan;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Gan::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.publishers().get("role", "roleId")
///              .publisher_id("et")
///              .doit();
/// # }
/// ```
pub struct PublisherGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Gan<C, A>,
    _role: String,
    _role_id: String,
    _publisher_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for PublisherGetCall<'a, C, A> {}

impl<'a, C, A> PublisherGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Publisher)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "gan.publishers.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("role", self._role.to_string()));
        params.push(("roleId", self._role_id.to_string()));
        if let Some(value) = self._publisher_id {
            params.push(("publisherId", value.to_string()));
        }
        for &field in ["alt", "role", "roleId", "publisherId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{role}/{roleId}/publisher";
        
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

        for &(find_this, param_name) in [("{role}", "role"), ("{roleId}", "roleId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["roleId", "role"].iter() {
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


    /// The role of the requester. Valid values: 'advertisers' or 'publishers'.
    ///
    /// Sets the *role* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role(mut self, new_value: &str) -> PublisherGetCall<'a, C, A> {
        self._role = new_value.to_string();
        self
    }
    /// The ID of the requesting advertiser or publisher.
    ///
    /// Sets the *role id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role_id(mut self, new_value: &str) -> PublisherGetCall<'a, C, A> {
        self._role_id = new_value.to_string();
        self
    }
    /// The ID of the publisher to look up. Optional.
    ///
    /// Sets the *publisher id* query property to the given value.
    pub fn publisher_id(mut self, new_value: &str) -> PublisherGetCall<'a, C, A> {
        self._publisher_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PublisherGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PublisherGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Inserts a new link.
///
/// A builder for the *insert* method supported by a *link* resource.
/// It is not used directly, but through a `LinkMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_gan1_beta1 as gan1_beta1;
/// use gan1_beta1::Link;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use gan1_beta1::Gan;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Gan::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Link::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.links().insert(req, "role", "roleId")
///              .doit();
/// # }
/// ```
pub struct LinkInsertCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Gan<C, A>,
    _request: Link,
    _role: String,
    _role_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for LinkInsertCall<'a, C, A> {}

impl<'a, C, A> LinkInsertCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Link)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "gan.links.insert",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("role", self._role.to_string()));
        params.push(("roleId", self._role_id.to_string()));
        for &field in ["alt", "role", "roleId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{role}/{roleId}/link";
        
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

        for &(find_this, param_name) in [("{role}", "role"), ("{roleId}", "roleId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["roleId", "role"].iter() {
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
    pub fn request(mut self, new_value: Link) -> LinkInsertCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The role of the requester. Valid values: 'advertisers' or 'publishers'.
    ///
    /// Sets the *role* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role(mut self, new_value: &str) -> LinkInsertCall<'a, C, A> {
        self._role = new_value.to_string();
        self
    }
    /// The ID of the requesting advertiser or publisher.
    ///
    /// Sets the *role id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role_id(mut self, new_value: &str) -> LinkInsertCall<'a, C, A> {
        self._role_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LinkInsertCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LinkInsertCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Retrieves data about a single link if the requesting advertiser/publisher has access to it. Advertisers can look up their own links. Publishers can look up visible links or links belonging to advertisers they are in a relationship with.
///
/// A builder for the *get* method supported by a *link* resource.
/// It is not used directly, but through a `LinkMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_gan1_beta1 as gan1_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use gan1_beta1::Gan;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Gan::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.links().get("role", "roleId", "linkId")
///              .doit();
/// # }
/// ```
pub struct LinkGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Gan<C, A>,
    _role: String,
    _role_id: String,
    _link_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for LinkGetCall<'a, C, A> {}

impl<'a, C, A> LinkGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Link)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "gan.links.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("role", self._role.to_string()));
        params.push(("roleId", self._role_id.to_string()));
        params.push(("linkId", self._link_id.to_string()));
        for &field in ["alt", "role", "roleId", "linkId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{role}/{roleId}/link/{linkId}";
        
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

        for &(find_this, param_name) in [("{role}", "role"), ("{roleId}", "roleId"), ("{linkId}", "linkId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["linkId", "roleId", "role"].iter() {
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


    /// The role of the requester. Valid values: 'advertisers' or 'publishers'.
    ///
    /// Sets the *role* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role(mut self, new_value: &str) -> LinkGetCall<'a, C, A> {
        self._role = new_value.to_string();
        self
    }
    /// The ID of the requesting advertiser or publisher.
    ///
    /// Sets the *role id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role_id(mut self, new_value: &str) -> LinkGetCall<'a, C, A> {
        self._role_id = new_value.to_string();
        self
    }
    /// The ID of the link to look up.
    ///
    /// Sets the *link id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn link_id(mut self, new_value: &str) -> LinkGetCall<'a, C, A> {
        self._link_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LinkGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LinkGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Retrieves all links that match the query parameters.
///
/// A builder for the *list* method supported by a *link* resource.
/// It is not used directly, but through a `LinkMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_gan1_beta1 as gan1_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use gan1_beta1::Gan;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Gan::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.links().list("role", "roleId")
///              .start_date_min("et")
///              .start_date_max("consetetur")
///              .search_text("amet.")
///              .relationship_status("voluptua.")
///              .add_promotion_type("Lorem")
///              .page_token("gubergren")
///              .max_results(81)
///              .link_type("sit")
///              .create_date_min("vero")
///              .create_date_max("diam")
///              .authorship("rebum.")
///              .add_asset_size("consetetur")
///              .add_advertiser_id("sadipscing")
///              .doit();
/// # }
/// ```
pub struct LinkListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Gan<C, A>,
    _role: String,
    _role_id: String,
    _start_date_min: Option<String>,
    _start_date_max: Option<String>,
    _search_text: Option<String>,
    _relationship_status: Option<String>,
    _promotion_type: Vec<String>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _link_type: Option<String>,
    _create_date_min: Option<String>,
    _create_date_max: Option<String>,
    _authorship: Option<String>,
    _asset_size: Vec<String>,
    _advertiser_id: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for LinkListCall<'a, C, A> {}

impl<'a, C, A> LinkListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Links)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "gan.links.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(17 + self._additional_params.len());
        params.push(("role", self._role.to_string()));
        params.push(("roleId", self._role_id.to_string()));
        if let Some(value) = self._start_date_min {
            params.push(("startDateMin", value.to_string()));
        }
        if let Some(value) = self._start_date_max {
            params.push(("startDateMax", value.to_string()));
        }
        if let Some(value) = self._search_text {
            params.push(("searchText", value.to_string()));
        }
        if let Some(value) = self._relationship_status {
            params.push(("relationshipStatus", value.to_string()));
        }
        if self._promotion_type.len() > 0 {
            for f in self._promotion_type.iter() {
                params.push(("promotionType", f.to_string()));
            }
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._link_type {
            params.push(("linkType", value.to_string()));
        }
        if let Some(value) = self._create_date_min {
            params.push(("createDateMin", value.to_string()));
        }
        if let Some(value) = self._create_date_max {
            params.push(("createDateMax", value.to_string()));
        }
        if let Some(value) = self._authorship {
            params.push(("authorship", value.to_string()));
        }
        if self._asset_size.len() > 0 {
            for f in self._asset_size.iter() {
                params.push(("assetSize", f.to_string()));
            }
        }
        if self._advertiser_id.len() > 0 {
            for f in self._advertiser_id.iter() {
                params.push(("advertiserId", f.to_string()));
            }
        }
        for &field in ["alt", "role", "roleId", "startDateMin", "startDateMax", "searchText", "relationshipStatus", "promotionType", "pageToken", "maxResults", "linkType", "createDateMin", "createDateMax", "authorship", "assetSize", "advertiserId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{role}/{roleId}/links";
        
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

        for &(find_this, param_name) in [("{role}", "role"), ("{roleId}", "roleId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["roleId", "role"].iter() {
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


    /// The role of the requester. Valid values: 'advertisers' or 'publishers'.
    ///
    /// Sets the *role* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role(mut self, new_value: &str) -> LinkListCall<'a, C, A> {
        self._role = new_value.to_string();
        self
    }
    /// The ID of the requesting advertiser or publisher.
    ///
    /// Sets the *role id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role_id(mut self, new_value: &str) -> LinkListCall<'a, C, A> {
        self._role_id = new_value.to_string();
        self
    }
    /// The beginning of the start date range.
    ///
    /// Sets the *start date min* query property to the given value.
    pub fn start_date_min(mut self, new_value: &str) -> LinkListCall<'a, C, A> {
        self._start_date_min = Some(new_value.to_string());
        self
    }
    /// The end of the start date range.
    ///
    /// Sets the *start date max* query property to the given value.
    pub fn start_date_max(mut self, new_value: &str) -> LinkListCall<'a, C, A> {
        self._start_date_max = Some(new_value.to_string());
        self
    }
    /// Field for full text search across title and merchandising text, supports link id search.
    ///
    /// Sets the *search text* query property to the given value.
    pub fn search_text(mut self, new_value: &str) -> LinkListCall<'a, C, A> {
        self._search_text = Some(new_value.to_string());
        self
    }
    /// The status of the relationship.
    ///
    /// Sets the *relationship status* query property to the given value.
    pub fn relationship_status(mut self, new_value: &str) -> LinkListCall<'a, C, A> {
        self._relationship_status = Some(new_value.to_string());
        self
    }
    /// The promotion type.
    ///
    /// Append the given value to the *promotion type* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_promotion_type(mut self, new_value: &str) -> LinkListCall<'a, C, A> {
        self._promotion_type.push(new_value.to_string());
        self
    }
    /// The value of 'nextPageToken' from the previous page. Optional.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> LinkListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Max number of items to return in this page. Optional. Defaults to 20.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> LinkListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// The type of the link.
    ///
    /// Sets the *link type* query property to the given value.
    pub fn link_type(mut self, new_value: &str) -> LinkListCall<'a, C, A> {
        self._link_type = Some(new_value.to_string());
        self
    }
    /// The beginning of the create date range.
    ///
    /// Sets the *create date min* query property to the given value.
    pub fn create_date_min(mut self, new_value: &str) -> LinkListCall<'a, C, A> {
        self._create_date_min = Some(new_value.to_string());
        self
    }
    /// The end of the create date range.
    ///
    /// Sets the *create date max* query property to the given value.
    pub fn create_date_max(mut self, new_value: &str) -> LinkListCall<'a, C, A> {
        self._create_date_max = Some(new_value.to_string());
        self
    }
    /// The role of the author of the link.
    ///
    /// Sets the *authorship* query property to the given value.
    pub fn authorship(mut self, new_value: &str) -> LinkListCall<'a, C, A> {
        self._authorship = Some(new_value.to_string());
        self
    }
    /// The size of the given asset.
    ///
    /// Append the given value to the *asset size* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_asset_size(mut self, new_value: &str) -> LinkListCall<'a, C, A> {
        self._asset_size.push(new_value.to_string());
        self
    }
    /// Limits the resulting links to the ones belonging to the listed advertisers.
    ///
    /// Append the given value to the *advertiser id* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_advertiser_id(mut self, new_value: &str) -> LinkListCall<'a, C, A> {
        self._advertiser_id.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LinkListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LinkListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Retrieves a report of the specified type.
///
/// A builder for the *get* method supported by a *report* resource.
/// It is not used directly, but through a `ReportMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_gan1_beta1 as gan1_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use gan1_beta1::Gan;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Gan::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.reports().get("role", "roleId", "reportType")
///              .status("consetetur")
///              .start_index(17)
///              .start_date("duo")
///              .add_publisher_id("aliquyam")
///              .add_order_id("Lorem")
///              .max_results(84)
///              .add_link_id("clita")
///              .event_type("consetetur")
///              .end_date("takimata")
///              .calculate_totals(true)
///              .add_advertiser_id("kasd")
///              .doit();
/// # }
/// ```
pub struct ReportGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Gan<C, A>,
    _role: String,
    _role_id: String,
    _report_type: String,
    _status: Option<String>,
    _start_index: Option<u32>,
    _start_date: Option<String>,
    _publisher_id: Vec<String>,
    _order_id: Vec<String>,
    _max_results: Option<u32>,
    _link_id: Vec<String>,
    _event_type: Option<String>,
    _end_date: Option<String>,
    _calculate_totals: Option<bool>,
    _advertiser_id: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for ReportGetCall<'a, C, A> {}

impl<'a, C, A> ReportGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Report)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "gan.reports.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(16 + self._additional_params.len());
        params.push(("role", self._role.to_string()));
        params.push(("roleId", self._role_id.to_string()));
        params.push(("reportType", self._report_type.to_string()));
        if let Some(value) = self._status {
            params.push(("status", value.to_string()));
        }
        if let Some(value) = self._start_index {
            params.push(("startIndex", value.to_string()));
        }
        if let Some(value) = self._start_date {
            params.push(("startDate", value.to_string()));
        }
        if self._publisher_id.len() > 0 {
            for f in self._publisher_id.iter() {
                params.push(("publisherId", f.to_string()));
            }
        }
        if self._order_id.len() > 0 {
            for f in self._order_id.iter() {
                params.push(("orderId", f.to_string()));
            }
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if self._link_id.len() > 0 {
            for f in self._link_id.iter() {
                params.push(("linkId", f.to_string()));
            }
        }
        if let Some(value) = self._event_type {
            params.push(("eventType", value.to_string()));
        }
        if let Some(value) = self._end_date {
            params.push(("endDate", value.to_string()));
        }
        if let Some(value) = self._calculate_totals {
            params.push(("calculateTotals", value.to_string()));
        }
        if self._advertiser_id.len() > 0 {
            for f in self._advertiser_id.iter() {
                params.push(("advertiserId", f.to_string()));
            }
        }
        for &field in ["alt", "role", "roleId", "reportType", "status", "startIndex", "startDate", "publisherId", "orderId", "maxResults", "linkId", "eventType", "endDate", "calculateTotals", "advertiserId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{role}/{roleId}/report/{reportType}";
        
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

        for &(find_this, param_name) in [("{role}", "role"), ("{roleId}", "roleId"), ("{reportType}", "reportType")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["reportType", "roleId", "role"].iter() {
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


    /// The role of the requester. Valid values: 'advertisers' or 'publishers'.
    ///
    /// Sets the *role* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role(mut self, new_value: &str) -> ReportGetCall<'a, C, A> {
        self._role = new_value.to_string();
        self
    }
    /// The ID of the requesting advertiser or publisher.
    ///
    /// Sets the *role id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role_id(mut self, new_value: &str) -> ReportGetCall<'a, C, A> {
        self._role_id = new_value.to_string();
        self
    }
    /// The type of report being requested. Valid values: 'order_delta'. Required.
    ///
    /// Sets the *report type* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn report_type(mut self, new_value: &str) -> ReportGetCall<'a, C, A> {
        self._report_type = new_value.to_string();
        self
    }
    /// Filters out all events that do not have the given status. Valid values: 'active', 'canceled', or 'invalid'. Optional.
    ///
    /// Sets the *status* query property to the given value.
    pub fn status(mut self, new_value: &str) -> ReportGetCall<'a, C, A> {
        self._status = Some(new_value.to_string());
        self
    }
    /// Offset on which to return results when paging. Optional.
    ///
    /// Sets the *start index* query property to the given value.
    pub fn start_index(mut self, new_value: u32) -> ReportGetCall<'a, C, A> {
        self._start_index = Some(new_value);
        self
    }
    /// The start date (inclusive), in RFC 3339 format, for the report data to be returned. Defaults to one day before endDate, if that is given, or yesterday. Optional.
    ///
    /// Sets the *start date* query property to the given value.
    pub fn start_date(mut self, new_value: &str) -> ReportGetCall<'a, C, A> {
        self._start_date = Some(new_value.to_string());
        self
    }
    /// The IDs of the publishers to look up, if applicable.
    ///
    /// Append the given value to the *publisher id* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_publisher_id(mut self, new_value: &str) -> ReportGetCall<'a, C, A> {
        self._publisher_id.push(new_value.to_string());
        self
    }
    /// Filters to capture one of the given order IDs. Optional.
    ///
    /// Append the given value to the *order id* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_order_id(mut self, new_value: &str) -> ReportGetCall<'a, C, A> {
        self._order_id.push(new_value.to_string());
        self
    }
    /// Max number of items to return in this page. Optional. Defaults to return all results.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> ReportGetCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Filters to capture one of given link IDs. Optional.
    ///
    /// Append the given value to the *link id* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_link_id(mut self, new_value: &str) -> ReportGetCall<'a, C, A> {
        self._link_id.push(new_value.to_string());
        self
    }
    /// Filters out all events that are not of the given type. Valid values: 'action', 'transaction', or 'charge'. Optional.
    ///
    /// Sets the *event type* query property to the given value.
    pub fn event_type(mut self, new_value: &str) -> ReportGetCall<'a, C, A> {
        self._event_type = Some(new_value.to_string());
        self
    }
    /// The end date (exclusive), in RFC 3339 format, for the report data to be returned. Defaults to one day after startDate, if that is given, or today. Optional.
    ///
    /// Sets the *end date* query property to the given value.
    pub fn end_date(mut self, new_value: &str) -> ReportGetCall<'a, C, A> {
        self._end_date = Some(new_value.to_string());
        self
    }
    /// Whether or not to calculate totals rows. Optional.
    ///
    /// Sets the *calculate totals* query property to the given value.
    pub fn calculate_totals(mut self, new_value: bool) -> ReportGetCall<'a, C, A> {
        self._calculate_totals = Some(new_value);
        self
    }
    /// The IDs of the advertisers to look up, if applicable.
    ///
    /// Append the given value to the *advertiser id* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_advertiser_id(mut self, new_value: &str) -> ReportGetCall<'a, C, A> {
        self._advertiser_id.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ReportGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ReportGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Retrieves credit card offers for the given publisher.
///
/// A builder for the *list* method supported by a *ccOffer* resource.
/// It is not used directly, but through a `CcOfferMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_gan1_beta1 as gan1_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use gan1_beta1::Gan;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Gan::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.cc_offers().list("publisher")
///              .projection("takimata")
///              .add_advertiser("At")
///              .doit();
/// # }
/// ```
pub struct CcOfferListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Gan<C, A>,
    _publisher: String,
    _projection: Option<String>,
    _advertiser: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for CcOfferListCall<'a, C, A> {}

impl<'a, C, A> CcOfferListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, CcOffers)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "gan.ccOffers.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("publisher", self._publisher.to_string()));
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if self._advertiser.len() > 0 {
            for f in self._advertiser.iter() {
                params.push(("advertiser", f.to_string()));
            }
        }
        for &field in ["alt", "publisher", "projection", "advertiser"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "publishers/{publisher}/ccOffers";
        
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

        for &(find_this, param_name) in [("{publisher}", "publisher")].iter() {
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
            for param_name in ["publisher"].iter() {
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


    /// The ID of the publisher in question.
    ///
    /// Sets the *publisher* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn publisher(mut self, new_value: &str) -> CcOfferListCall<'a, C, A> {
        self._publisher = new_value.to_string();
        self
    }
    /// The set of fields to return.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> CcOfferListCall<'a, C, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// The advertiser ID of a card issuer whose offers to include. Optional, may be repeated.
    ///
    /// Append the given value to the *advertiser* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_advertiser(mut self, new_value: &str) -> CcOfferListCall<'a, C, A> {
        self._advertiser.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CcOfferListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> CcOfferListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Retrieves data about a single advertiser if that the requesting advertiser/publisher has access to it. Only publishers can lookup advertisers. Advertisers can request information about themselves by omitting the advertiserId query parameter.
///
/// A builder for the *get* method supported by a *advertiser* resource.
/// It is not used directly, but through a `AdvertiserMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_gan1_beta1 as gan1_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use gan1_beta1::Gan;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Gan::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.advertisers().get("role", "roleId")
///              .advertiser_id("ea")
///              .doit();
/// # }
/// ```
pub struct AdvertiserGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Gan<C, A>,
    _role: String,
    _role_id: String,
    _advertiser_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for AdvertiserGetCall<'a, C, A> {}

impl<'a, C, A> AdvertiserGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Advertiser)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "gan.advertisers.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("role", self._role.to_string()));
        params.push(("roleId", self._role_id.to_string()));
        if let Some(value) = self._advertiser_id {
            params.push(("advertiserId", value.to_string()));
        }
        for &field in ["alt", "role", "roleId", "advertiserId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{role}/{roleId}/advertiser";
        
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

        for &(find_this, param_name) in [("{role}", "role"), ("{roleId}", "roleId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["roleId", "role"].iter() {
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


    /// The role of the requester. Valid values: 'advertisers' or 'publishers'.
    ///
    /// Sets the *role* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role(mut self, new_value: &str) -> AdvertiserGetCall<'a, C, A> {
        self._role = new_value.to_string();
        self
    }
    /// The ID of the requesting advertiser or publisher.
    ///
    /// Sets the *role id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role_id(mut self, new_value: &str) -> AdvertiserGetCall<'a, C, A> {
        self._role_id = new_value.to_string();
        self
    }
    /// The ID of the advertiser to look up. Optional.
    ///
    /// Sets the *advertiser id* query property to the given value.
    pub fn advertiser_id(mut self, new_value: &str) -> AdvertiserGetCall<'a, C, A> {
        self._advertiser_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AdvertiserGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> AdvertiserGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Retrieves data about all advertisers that the requesting advertiser/publisher has access to.
///
/// A builder for the *list* method supported by a *advertiser* resource.
/// It is not used directly, but through a `AdvertiserMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_gan1_beta1 as gan1_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use gan1_beta1::Gan;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Gan::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.advertisers().list("role", "roleId")
///              .relationship_status("dolore")
///              .page_token("nonumy")
///              .min_seven_day_epc(0.717377589745)
///              .min_payout_rank(-82)
///              .min_ninety_day_epc(0.484066452025)
///              .max_results(61)
///              .advertiser_category("consetetur")
///              .doit();
/// # }
/// ```
pub struct AdvertiserListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Gan<C, A>,
    _role: String,
    _role_id: String,
    _relationship_status: Option<String>,
    _page_token: Option<String>,
    _min_seven_day_epc: Option<f64>,
    _min_payout_rank: Option<i32>,
    _min_ninety_day_epc: Option<f64>,
    _max_results: Option<u32>,
    _advertiser_category: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for AdvertiserListCall<'a, C, A> {}

impl<'a, C, A> AdvertiserListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Advertisers)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "gan.advertisers.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(11 + self._additional_params.len());
        params.push(("role", self._role.to_string()));
        params.push(("roleId", self._role_id.to_string()));
        if let Some(value) = self._relationship_status {
            params.push(("relationshipStatus", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._min_seven_day_epc {
            params.push(("minSevenDayEpc", value.to_string()));
        }
        if let Some(value) = self._min_payout_rank {
            params.push(("minPayoutRank", value.to_string()));
        }
        if let Some(value) = self._min_ninety_day_epc {
            params.push(("minNinetyDayEpc", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._advertiser_category {
            params.push(("advertiserCategory", value.to_string()));
        }
        for &field in ["alt", "role", "roleId", "relationshipStatus", "pageToken", "minSevenDayEpc", "minPayoutRank", "minNinetyDayEpc", "maxResults", "advertiserCategory"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{role}/{roleId}/advertisers";
        
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

        for &(find_this, param_name) in [("{role}", "role"), ("{roleId}", "roleId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["roleId", "role"].iter() {
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


    /// The role of the requester. Valid values: 'advertisers' or 'publishers'.
    ///
    /// Sets the *role* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role(mut self, new_value: &str) -> AdvertiserListCall<'a, C, A> {
        self._role = new_value.to_string();
        self
    }
    /// The ID of the requesting advertiser or publisher.
    ///
    /// Sets the *role id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role_id(mut self, new_value: &str) -> AdvertiserListCall<'a, C, A> {
        self._role_id = new_value.to_string();
        self
    }
    /// Filters out all advertisers for which do not have the given relationship status with the requesting publisher.
    ///
    /// Sets the *relationship status* query property to the given value.
    pub fn relationship_status(mut self, new_value: &str) -> AdvertiserListCall<'a, C, A> {
        self._relationship_status = Some(new_value.to_string());
        self
    }
    /// The value of 'nextPageToken' from the previous page. Optional.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AdvertiserListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Filters out all advertisers that have a seven day EPC average lower than the given value (inclusive). Min value: 0.0. Optional.
    ///
    /// Sets the *min seven day epc* query property to the given value.
    pub fn min_seven_day_epc(mut self, new_value: f64) -> AdvertiserListCall<'a, C, A> {
        self._min_seven_day_epc = Some(new_value);
        self
    }
    /// A value between 1 and 4, where 1 represents the quartile of advertisers with the lowest ranks and 4 represents the quartile of advertisers with the highest ranks. Filters out all advertisers with a lower rank than the given quartile. For example if a 2 was given only advertisers with a payout rank of 25 or higher would be included. Optional.
    ///
    /// Sets the *min payout rank* query property to the given value.
    pub fn min_payout_rank(mut self, new_value: i32) -> AdvertiserListCall<'a, C, A> {
        self._min_payout_rank = Some(new_value);
        self
    }
    /// Filters out all advertisers that have a ninety day EPC average lower than the given value (inclusive). Min value: 0.0. Optional.
    ///
    /// Sets the *min ninety day epc* query property to the given value.
    pub fn min_ninety_day_epc(mut self, new_value: f64) -> AdvertiserListCall<'a, C, A> {
        self._min_ninety_day_epc = Some(new_value);
        self
    }
    /// Max number of items to return in this page. Optional. Defaults to 20.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> AdvertiserListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Caret(^) delimted list of advertiser categories. Valid categories are defined here: http://www.google.com/support/affiliatenetwork/advertiser/bin/answer.py?hl=en&answer=107581. Filters out all advertisers not in one of the given advertiser categories. Optional.
    ///
    /// Sets the *advertiser category* query property to the given value.
    pub fn advertiser_category(mut self, new_value: &str) -> AdvertiserListCall<'a, C, A> {
        self._advertiser_category = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AdvertiserListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> AdvertiserListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Retrieves event data for a given advertiser/publisher.
///
/// A builder for the *list* method supported by a *event* resource.
/// It is not used directly, but through a `EventMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_gan1_beta1 as gan1_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use gan1_beta1::Gan;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Gan::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.events().list("role", "roleId")
///              .type_("ea")
///              .status("gubergren")
///              .sku("aliquyam")
///              .publisher_id("eos")
///              .product_category("tempor")
///              .page_token("sea")
///              .order_id("labore")
///              .modify_date_min("ipsum")
///              .modify_date_max("aliquyam")
///              .member_id("dolores")
///              .max_results(3)
///              .link_id("diam")
///              .event_date_min("ut")
///              .event_date_max("justo")
///              .charge_type("est")
///              .advertiser_id("amet")
///              .doit();
/// # }
/// ```
pub struct EventListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Gan<C, A>,
    _role: String,
    _role_id: String,
    _type_: Option<String>,
    _status: Option<String>,
    _sku: Option<String>,
    _publisher_id: Option<String>,
    _product_category: Option<String>,
    _page_token: Option<String>,
    _order_id: Option<String>,
    _modify_date_min: Option<String>,
    _modify_date_max: Option<String>,
    _member_id: Option<String>,
    _max_results: Option<u32>,
    _link_id: Option<String>,
    _event_date_min: Option<String>,
    _event_date_max: Option<String>,
    _charge_type: Option<String>,
    _advertiser_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for EventListCall<'a, C, A> {}

impl<'a, C, A> EventListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Events)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "gan.events.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(20 + self._additional_params.len());
        params.push(("role", self._role.to_string()));
        params.push(("roleId", self._role_id.to_string()));
        if let Some(value) = self._type_ {
            params.push(("type", value.to_string()));
        }
        if let Some(value) = self._status {
            params.push(("status", value.to_string()));
        }
        if let Some(value) = self._sku {
            params.push(("sku", value.to_string()));
        }
        if let Some(value) = self._publisher_id {
            params.push(("publisherId", value.to_string()));
        }
        if let Some(value) = self._product_category {
            params.push(("productCategory", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._order_id {
            params.push(("orderId", value.to_string()));
        }
        if let Some(value) = self._modify_date_min {
            params.push(("modifyDateMin", value.to_string()));
        }
        if let Some(value) = self._modify_date_max {
            params.push(("modifyDateMax", value.to_string()));
        }
        if let Some(value) = self._member_id {
            params.push(("memberId", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._link_id {
            params.push(("linkId", value.to_string()));
        }
        if let Some(value) = self._event_date_min {
            params.push(("eventDateMin", value.to_string()));
        }
        if let Some(value) = self._event_date_max {
            params.push(("eventDateMax", value.to_string()));
        }
        if let Some(value) = self._charge_type {
            params.push(("chargeType", value.to_string()));
        }
        if let Some(value) = self._advertiser_id {
            params.push(("advertiserId", value.to_string()));
        }
        for &field in ["alt", "role", "roleId", "type", "status", "sku", "publisherId", "productCategory", "pageToken", "orderId", "modifyDateMin", "modifyDateMax", "memberId", "maxResults", "linkId", "eventDateMin", "eventDateMax", "chargeType", "advertiserId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{role}/{roleId}/events";
        
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

        for &(find_this, param_name) in [("{role}", "role"), ("{roleId}", "roleId")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["roleId", "role"].iter() {
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


    /// The role of the requester. Valid values: 'advertisers' or 'publishers'.
    ///
    /// Sets the *role* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role(mut self, new_value: &str) -> EventListCall<'a, C, A> {
        self._role = new_value.to_string();
        self
    }
    /// The ID of the requesting advertiser or publisher.
    ///
    /// Sets the *role id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role_id(mut self, new_value: &str) -> EventListCall<'a, C, A> {
        self._role_id = new_value.to_string();
        self
    }
    /// Filters out all events that are not of the given type. Valid values: 'action', 'transaction', 'charge'. Optional.
    ///
    /// Sets the *type* query property to the given value.
    pub fn type_(mut self, new_value: &str) -> EventListCall<'a, C, A> {
        self._type_ = Some(new_value.to_string());
        self
    }
    /// Filters out all events that do not have the given status. Valid values: 'active', 'canceled'. Optional.
    ///
    /// Sets the *status* query property to the given value.
    pub fn status(mut self, new_value: &str) -> EventListCall<'a, C, A> {
        self._status = Some(new_value.to_string());
        self
    }
    /// Caret(^) delimited list of SKUs. Filters out all events that do not reference one of the given SKU. Optional.
    ///
    /// Sets the *sku* query property to the given value.
    pub fn sku(mut self, new_value: &str) -> EventListCall<'a, C, A> {
        self._sku = Some(new_value.to_string());
        self
    }
    /// Caret(^) delimited list of publisher IDs. Filters out all events that do not reference one of the given publishers IDs. Only used when under advertiser role. Optional.
    ///
    /// Sets the *publisher id* query property to the given value.
    pub fn publisher_id(mut self, new_value: &str) -> EventListCall<'a, C, A> {
        self._publisher_id = Some(new_value.to_string());
        self
    }
    /// Caret(^) delimited list of product categories. Filters out all events that do not reference a product in one of the given product categories. Optional.
    ///
    /// Sets the *product category* query property to the given value.
    pub fn product_category(mut self, new_value: &str) -> EventListCall<'a, C, A> {
        self._product_category = Some(new_value.to_string());
        self
    }
    /// The value of 'nextPageToken' from the previous page. Optional.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> EventListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Caret(^) delimited list of order IDs. Filters out all events that do not reference one of the given order IDs. Optional.
    ///
    /// Sets the *order id* query property to the given value.
    pub fn order_id(mut self, new_value: &str) -> EventListCall<'a, C, A> {
        self._order_id = Some(new_value.to_string());
        self
    }
    /// Filters out all events modified earlier than given date. Optional. Defaults to 24 hours before the current modifyDateMax, if modifyDateMax is explicitly set.
    ///
    /// Sets the *modify date min* query property to the given value.
    pub fn modify_date_min(mut self, new_value: &str) -> EventListCall<'a, C, A> {
        self._modify_date_min = Some(new_value.to_string());
        self
    }
    /// Filters out all events modified later than given date. Optional. Defaults to 24 hours after modifyDateMin, if modifyDateMin is explicitly set.
    ///
    /// Sets the *modify date max* query property to the given value.
    pub fn modify_date_max(mut self, new_value: &str) -> EventListCall<'a, C, A> {
        self._modify_date_max = Some(new_value.to_string());
        self
    }
    /// Caret(^) delimited list of member IDs. Filters out all events that do not reference one of the given member IDs. Optional.
    ///
    /// Sets the *member id* query property to the given value.
    pub fn member_id(mut self, new_value: &str) -> EventListCall<'a, C, A> {
        self._member_id = Some(new_value.to_string());
        self
    }
    /// Max number of offers to return in this page. Optional. Defaults to 20.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> EventListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Caret(^) delimited list of link IDs. Filters out all events that do not reference one of the given link IDs. Optional.
    ///
    /// Sets the *link id* query property to the given value.
    pub fn link_id(mut self, new_value: &str) -> EventListCall<'a, C, A> {
        self._link_id = Some(new_value.to_string());
        self
    }
    /// Filters out all events earlier than given date. Optional. Defaults to 24 hours from current date/time.
    ///
    /// Sets the *event date min* query property to the given value.
    pub fn event_date_min(mut self, new_value: &str) -> EventListCall<'a, C, A> {
        self._event_date_min = Some(new_value.to_string());
        self
    }
    /// Filters out all events later than given date. Optional. Defaults to 24 hours after eventMin.
    ///
    /// Sets the *event date max* query property to the given value.
    pub fn event_date_max(mut self, new_value: &str) -> EventListCall<'a, C, A> {
        self._event_date_max = Some(new_value.to_string());
        self
    }
    /// Filters out all charge events that are not of the given charge type. Valid values: 'other', 'slotting_fee', 'monthly_minimum', 'tier_bonus', 'credit', 'debit'. Optional.
    ///
    /// Sets the *charge type* query property to the given value.
    pub fn charge_type(mut self, new_value: &str) -> EventListCall<'a, C, A> {
        self._charge_type = Some(new_value.to_string());
        self
    }
    /// Caret(^) delimited list of advertiser IDs. Filters out all events that do not reference one of the given advertiser IDs. Only used when under publishers role. Optional.
    ///
    /// Sets the *advertiser id* query property to the given value.
    pub fn advertiser_id(mut self, new_value: &str) -> EventListCall<'a, C, A> {
        self._advertiser_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> EventListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> EventListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


