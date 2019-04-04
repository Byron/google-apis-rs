// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Shopping Content* crate version *1.0.8+20181009*, where *20181009* is the exact revision of the *content:v2sandbox* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.8*.
//! 
//! Everything else about the *Shopping Content* *v2_sandbox* API can be found at the
//! [official documentation site](https://developers.google.com/shopping-content).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/content2_sandbox).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.ShoppingContent.html) ... 
//! 
//! * orderinvoices
//!  * [*createchargeinvoice*](struct.OrderinvoiceCreatechargeinvoiceCall.html) and [*createrefundinvoice*](struct.OrderinvoiceCreaterefundinvoiceCall.html)
//! * orderpayments
//!  * [*notifyauthapproved*](struct.OrderpaymentNotifyauthapprovedCall.html), [*notifyauthdeclined*](struct.OrderpaymentNotifyauthdeclinedCall.html), [*notifycharge*](struct.OrderpaymentNotifychargeCall.html) and [*notifyrefund*](struct.OrderpaymentNotifyrefundCall.html)
//! * orderreturns
//!  * [*get*](struct.OrderreturnGetCall.html) and [*list*](struct.OrderreturnListCall.html)
//! * [orders](struct.Order.html)
//!  * [*acknowledge*](struct.OrderAcknowledgeCall.html), [*advancetestorder*](struct.OrderAdvancetestorderCall.html), [*cancel*](struct.OrderCancelCall.html), [*cancellineitem*](struct.OrderCancellineitemCall.html), [*canceltestorderbycustomer*](struct.OrderCanceltestorderbycustomerCall.html), [*createtestorder*](struct.OrderCreatetestorderCall.html), [*createtestreturn*](struct.OrderCreatetestreturnCall.html), [*custombatch*](struct.OrderCustombatchCall.html), [*get*](struct.OrderGetCall.html), [*getbymerchantorderid*](struct.OrderGetbymerchantorderidCall.html), [*gettestordertemplate*](struct.OrderGettestordertemplateCall.html), [*instorerefundlineitem*](struct.OrderInstorerefundlineitemCall.html), [*list*](struct.OrderListCall.html), [*refund*](struct.OrderRefundCall.html), [*rejectreturnlineitem*](struct.OrderRejectreturnlineitemCall.html), [*returnlineitem*](struct.OrderReturnlineitemCall.html), [*returnrefundlineitem*](struct.OrderReturnrefundlineitemCall.html), [*setlineitemmetadata*](struct.OrderSetlineitemmetadataCall.html), [*shiplineitems*](struct.OrderShiplineitemCall.html), [*updatelineitemshippingdetails*](struct.OrderUpdatelineitemshippingdetailCall.html), [*updatemerchantorderid*](struct.OrderUpdatemerchantorderidCall.html) and [*updateshipment*](struct.OrderUpdateshipmentCall.html)
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
//! * **[Hub](struct.ShoppingContent.html)**
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
//! let r = hub.orders().list(...).doit()
//! let r = hub.orders().updatemerchantorderid(...).doit()
//! let r = hub.orders().gettestordertemplate(...).doit()
//! let r = hub.orders().updatelineitemshippingdetails(...).doit()
//! let r = hub.orders().refund(...).doit()
//! let r = hub.orders().acknowledge(...).doit()
//! let r = hub.orders().instorerefundlineitem(...).doit()
//! let r = hub.orders().advancetestorder(...).doit()
//! let r = hub.orders().rejectreturnlineitem(...).doit()
//! let r = hub.orders().createtestorder(...).doit()
//! let r = hub.orders().cancellineitem(...).doit()
//! let r = hub.orders().canceltestorderbycustomer(...).doit()
//! let r = hub.orders().returnrefundlineitem(...).doit()
//! let r = hub.orders().get(...).doit()
//! let r = hub.orders().returnlineitem(...).doit()
//! let r = hub.orders().custombatch(...).doit()
//! let r = hub.orders().getbymerchantorderid(...).doit()
//! let r = hub.orders().shiplineitems(...).doit()
//! let r = hub.orders().createtestreturn(...).doit()
//! let r = hub.orders().updateshipment(...).doit()
//! let r = hub.orders().setlineitemmetadata(...).doit()
//! let r = hub.orders().cancel(...).doit()
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
//! google-content2_sandbox = "*"
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
//! extern crate google_content2_sandbox as content2_sandbox;
//! use content2_sandbox::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use content2_sandbox::ShoppingContent;
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
//! let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.orders().list("merchantId")
//!              .add_statuses("justo")
//!              .placed_date_start("amet.")
//!              .placed_date_end("erat")
//!              .page_token("labore")
//!              .order_by("sea")
//!              .max_results(11)
//!              .acknowledged(true)
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

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// Manage your product listings and accounts for Google Shopping
    Full,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Full => "https://www.googleapis.com/auth/content",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Full
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all ShoppingContent related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use content2_sandbox::ShoppingContent;
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
/// let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().list("merchantId")
///              .add_statuses("sadipscing")
///              .placed_date_start("aliquyam")
///              .placed_date_end("ea")
///              .page_token("no")
///              .order_by("justo")
///              .max_results(80)
///              .acknowledged(true)
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
pub struct ShoppingContent<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for ShoppingContent<C, A> {}

impl<'a, C, A> ShoppingContent<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> ShoppingContent<C, A> {
        ShoppingContent {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.8".to_string(),
            _base_url: "https://www.googleapis.com/content/v2sandbox/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn orderinvoices(&'a self) -> OrderinvoiceMethods<'a, C, A> {
        OrderinvoiceMethods { hub: &self }
    }
    pub fn orderpayments(&'a self) -> OrderpaymentMethods<'a, C, A> {
        OrderpaymentMethods { hub: &self }
    }
    pub fn orderreturns(&'a self) -> OrderreturnMethods<'a, C, A> {
        OrderreturnMethods { hub: &self }
    }
    pub fn orders(&'a self) -> OrderMethods<'a, C, A> {
        OrderMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.8`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/content/v2sandbox/`.
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
/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UnitInvoiceAdditionalCharge {
    /// [required] Amount of the additional charge.
    #[serde(rename="additionalChargeAmount")]
    pub additional_charge_amount: Option<Amount>,
    /// [required] Type of the additional charge.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Promotions applied to the additional charge.
    #[serde(rename="additionalChargePromotions")]
    pub additional_charge_promotions: Option<Vec<Promotion>>,
}

impl Part for UnitInvoiceAdditionalCharge {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntryUpdateShipment {
    /// Date on which the shipment has been delivered, in ISO 8601 format. Optional and can be provided only if status is delivered.
    #[serde(rename="deliveryDate")]
    pub delivery_date: Option<String>,
    /// New status for the shipment. Not updated if missing.
    pub status: Option<String>,
    /// The carrier handling the shipment. Not updated if missing. See shipments[].carrier in the  Orders resource representation for a list of acceptable values.
    pub carrier: Option<String>,
    /// The tracking id for the shipment. Not updated if missing.
    #[serde(rename="trackingId")]
    pub tracking_id: Option<String>,
    /// The ID of the shipment.
    #[serde(rename="shipmentId")]
    pub shipment_id: Option<String>,
}

impl Part for OrdersCustomBatchRequestEntryUpdateShipment {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderLegacyPromotionBenefit {
    /// The OfferId(s) that were purchased in this order and map to this specific benefit of the promotion.
    #[serde(rename="offerIds")]
    pub offer_ids: Option<Vec<String>>,
    /// The discount in the order price when the promotion is applied.
    pub discount: Option<Price>,
    /// Describes whether the promotion applies to products (e.g. 20% off) or to shipping (e.g. Free Shipping).
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The impact on tax when the promotion is applied.
    #[serde(rename="taxImpact")]
    pub tax_impact: Option<Price>,
    /// Further describes the benefit of the promotion. Note that we will expand on this enumeration as we support new promotion sub-types.
    #[serde(rename="subType")]
    pub sub_type: Option<String>,
}

impl Part for OrderLegacyPromotionBenefit {}


/// A list of errors returned by a failed batch entry.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Errors {
    /// The message of the first error in errors.
    pub message: Option<String>,
    /// The HTTP status of the first error in errors.
    pub code: Option<u32>,
    /// A list of errors.
    pub errors: Option<Vec<ErrorType>>,
}

impl Part for Errors {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderReturn {
    /// Quantity that is returned.
    pub quantity: Option<u32>,
    /// The reason for the return.
    pub reason: Option<String>,
    /// Date on which the item has been created, in ISO 8601 format.
    #[serde(rename="creationDate")]
    pub creation_date: Option<String>,
    /// The actor that created the refund.
    pub actor: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    pub reason_text: Option<String>,
}

impl Part for OrderReturn {}


/// An error returned by the API.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ErrorType {
    /// The error code.
    pub reason: Option<String>,
    /// A description of the error.
    pub message: Option<String>,
    /// The domain of the error.
    pub domain: Option<String>,
}

impl Part for ErrorType {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [updatelineitemshippingdetails orders](struct.OrderUpdatelineitemshippingdetailCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersUpdateLineItemShippingDetailsRequest {
    /// The ID of the operation. Unique across all operations for a given order.
    #[serde(rename="operationId")]
    pub operation_id: Option<String>,
    /// Updated delivery by date, in ISO 8601 format. If not specified only ship by date is updated.
    #[serde(rename="deliverByDate")]
    pub deliver_by_date: Option<String>,
    /// The ID of the product to set metadata. This is the REST ID used in the products service. Either lineItemId or productId is required.
    #[serde(rename="productId")]
    pub product_id: Option<String>,
    /// The ID of the line item to set metadata. Either lineItemId or productId is required.
    #[serde(rename="lineItemId")]
    pub line_item_id: Option<String>,
    /// Updated ship by date, in ISO 8601 format. If not specified only deliver by date is updated.
    #[serde(rename="shipByDate")]
    pub ship_by_date: Option<String>,
}

impl RequestValue for OrdersUpdateLineItemShippingDetailsRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderLineItemShippingDetailsMethod {
    /// Minimum transit time.
    #[serde(rename="minDaysInTransit")]
    pub min_days_in_transit: Option<u32>,
    /// The carrier for the shipping. Optional. See shipments[].carrier for a list of acceptable values.
    pub carrier: Option<String>,
    /// The name of the shipping method.
    #[serde(rename="methodName")]
    pub method_name: Option<String>,
    /// Maximum transit time.
    #[serde(rename="maxDaysInTransit")]
    pub max_days_in_transit: Option<u32>,
}

impl Part for OrderLineItemShippingDetailsMethod {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notifyauthapproved orderpayments](struct.OrderpaymentNotifyauthapprovedCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderpaymentsNotifyAuthApprovedRequest {
    /// no description provided
    #[serde(rename="authAmountPretax")]
    pub auth_amount_pretax: Option<Price>,
    /// no description provided
    #[serde(rename="authAmountTax")]
    pub auth_amount_tax: Option<Price>,
}

impl RequestValue for OrderpaymentsNotifyAuthApprovedRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [createrefundinvoice orderinvoices](struct.OrderinvoiceCreaterefundinvoiceCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderinvoicesCreateRefundInvoiceResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#orderinvoicesCreateRefundInvoiceResponse".
    pub kind: Option<String>,
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    pub execution_status: Option<String>,
}

impl ResponseResult for OrderinvoicesCreateRefundInvoiceResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntryUpdateLineItemShippingDetails {
    /// Updated delivery by date, in ISO 8601 format. If not specified only ship by date is updated.
    #[serde(rename="deliverByDate")]
    pub deliver_by_date: Option<String>,
    /// The ID of the product to set metadata. This is the REST ID used in the products service. Either lineItemId or productId is required.
    #[serde(rename="productId")]
    pub product_id: Option<String>,
    /// The ID of the line item to set metadata. Either lineItemId or productId is required.
    #[serde(rename="lineItemId")]
    pub line_item_id: Option<String>,
    /// Updated ship by date, in ISO 8601 format. If not specified only deliver by date is updated.
    #[serde(rename="shipByDate")]
    pub ship_by_date: Option<String>,
}

impl Part for OrdersCustomBatchRequestEntryUpdateLineItemShippingDetails {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UnitInvoiceTaxLine {
    /// [required] Type of the tax.
    #[serde(rename="taxType")]
    pub tax_type: Option<String>,
    /// [required] Tax amount for the tax type.
    #[serde(rename="taxAmount")]
    pub tax_amount: Option<Price>,
    /// Optional name of the tax type. This should only be provided if taxType is otherFeeTax.
    #[serde(rename="taxName")]
    pub tax_name: Option<String>,
}

impl Part for UnitInvoiceTaxLine {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [updateshipment orders](struct.OrderUpdateshipmentCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersUpdateShipmentResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersUpdateShipmentResponse".
    pub kind: Option<String>,
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    pub execution_status: Option<String>,
}

impl ResponseResult for OrdersUpdateShipmentResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instorerefundlineitem orders](struct.OrderInstorerefundlineitemCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersInStoreRefundLineItemRequest {
    /// The quantity to return and refund.
    pub quantity: Option<u32>,
    /// The amount that is refunded. Required.
    #[serde(rename="amountPretax")]
    pub amount_pretax: Option<Price>,
    /// The reason for the return.
    pub reason: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    pub reason_text: Option<String>,
    /// The ID of the line item to return. Either lineItemId or productId is required.
    #[serde(rename="lineItemId")]
    pub line_item_id: Option<String>,
    /// Tax amount that correspond to refund amount in amountPretax. Required.
    #[serde(rename="amountTax")]
    pub amount_tax: Option<Price>,
    /// The ID of the product to return. This is the REST ID used in the products service. Either lineItemId or productId is required.
    #[serde(rename="productId")]
    pub product_id: Option<String>,
    /// The ID of the operation. Unique across all operations for a given order.
    #[serde(rename="operationId")]
    pub operation_id: Option<String>,
}

impl RequestValue for OrdersInStoreRefundLineItemRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [createrefundinvoice orderinvoices](struct.OrderinvoiceCreaterefundinvoiceCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderinvoicesCreateRefundInvoiceRequest {
    /// [required] The ID of the operation, unique across all operations for a given order.
    #[serde(rename="operationId")]
    pub operation_id: Option<String>,
    /// Invoice details for different shipment groups.
    #[serde(rename="shipmentInvoices")]
    pub shipment_invoices: Option<Vec<ShipmentInvoice>>,
    /// [required] The ID of the invoice.
    #[serde(rename="invoiceId")]
    pub invoice_id: Option<String>,
    /// Option to create an invoice for a refund and mark all items within the invoice as returned. Exactly one of refundOnlyOption or returnOption must be provided.
    #[serde(rename="returnOption")]
    pub return_option: Option<OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceReturnOption>,
    /// Option to create a refund-only invoice. Exactly one of refundOnlyOption or returnOption must be provided.
    #[serde(rename="refundOnlyOption")]
    pub refund_only_option: Option<OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceRefundOption>,
}

impl RequestValue for OrderinvoicesCreateRefundInvoiceRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestOrderCustomer {
    /// Deprecated. Please use marketingRightsInfo instead.
    #[serde(rename="explicitMarketingPreference")]
    pub explicit_marketing_preference: Option<bool>,
    /// Full name of the customer.
    #[serde(rename="fullName")]
    pub full_name: Option<String>,
    /// Deprecated.
    pub email: Option<String>,
    /// Customer's marketing preferences.
    #[serde(rename="marketingRightsInfo")]
    pub marketing_rights_info: Option<TestOrderCustomerMarketingRightsInfo>,
}

impl Part for TestOrderCustomer {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [cancellineitem orders](struct.OrderCancellineitemCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCancelLineItemResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersCancelLineItemResponse".
    pub kind: Option<String>,
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    pub execution_status: Option<String>,
}

impl ResponseResult for OrdersCancelLineItemResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderAddress {
    /// City, town or commune. May also include dependent localities or sublocalities (e.g. neighborhoods or suburbs).
    pub locality: Option<String>,
    /// CLDR country code (e.g. "US").
    pub country: Option<String>,
    /// Strings representing the lines of the printed label for mailing the order, for example:
    /// John Smith
    /// 1600 Amphitheatre Parkway
    /// Mountain View, CA, 94043
    /// United States
    #[serde(rename="fullAddress")]
    pub full_address: Option<Vec<String>>,
    /// Street-level part of the address.
    #[serde(rename="streetAddress")]
    pub street_address: Option<Vec<String>>,
    /// Postal Code or ZIP (e.g. "94043").
    #[serde(rename="postalCode")]
    pub postal_code: Option<String>,
    /// Name of the recipient.
    #[serde(rename="recipientName")]
    pub recipient_name: Option<String>,
    /// Whether the address is a post office box.
    #[serde(rename="isPostOfficeBox")]
    pub is_post_office_box: Option<bool>,
    /// Top-level administrative subdivision of the country. For example, a state like California ("CA") or a province like Quebec ("QC").
    pub region: Option<String>,
}

impl Part for OrderAddress {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderLineItemProduct {
    /// The CLDR territory code of the target country of the product.
    #[serde(rename="targetCountry")]
    pub target_country: Option<i64>,
    /// URL to the cached image shown to the user when order was placed.
    #[serde(rename="shownImage")]
    pub shown_image: Option<String>,
    /// The two-letter ISO 639-1 language code for the item.
    #[serde(rename="contentLanguage")]
    pub content_language: Option<String>,
    /// The title of the product.
    pub title: Option<String>,
    /// Manufacturer Part Number (MPN) of the item.
    pub mpn: Option<String>,
    /// Variant attributes for the item. These are dimensions of the product, such as color, gender, material, pattern, and size. You can find a comprehensive list of variant attributes here.
    #[serde(rename="variantAttributes")]
    pub variant_attributes: Option<Vec<OrderLineItemProductVariantAttribute>>,
    /// Brand of the item.
    pub brand: Option<String>,
    /// URL of an image of the item.
    #[serde(rename="imageLink")]
    pub image_link: Option<String>,
    /// An identifier of the item.
    #[serde(rename="offerId")]
    pub offer_id: Option<String>,
    /// The REST id of the product.
    pub id: Option<String>,
    /// Shared identifier for all variants of the same product.
    #[serde(rename="itemGroupId")]
    pub item_group_id: Option<String>,
    /// Global Trade Item Number (GTIN) of the item.
    pub gtin: Option<String>,
    /// Condition or state of the item.
    pub condition: Option<String>,
    /// Price of the item.
    pub price: Option<Price>,
    /// The item's channel (online or local).
    pub channel: Option<String>,
}

impl Part for OrderLineItemProduct {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestOrderCustomerMarketingRightsInfo {
    /// Last know user use selection regards marketing preferences. In certain cases selection might not be known, so this field would be empty.
    #[serde(rename="explicitMarketingPreference")]
    pub explicit_marketing_preference: Option<String>,
    /// Timestamp when last time marketing preference was updated. Could be empty, if user wasn't offered a selection yet.
    #[serde(rename="lastUpdatedTimestamp")]
    pub last_updated_timestamp: Option<String>,
}

impl Part for TestOrderCustomerMarketingRightsInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomerReturnReason {
    /// no description provided
    #[serde(rename="reasonCode")]
    pub reason_code: Option<String>,
    /// no description provided
    pub description: Option<String>,
}

impl Part for CustomerReturnReason {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notifyauthdeclined orderpayments](struct.OrderpaymentNotifyauthdeclinedCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderpaymentsNotifyAuthDeclinedResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#orderpaymentsNotifyAuthDeclinedResponse".
    pub kind: Option<String>,
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    pub execution_status: Option<String>,
}

impl ResponseResult for OrderpaymentsNotifyAuthDeclinedResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [getbymerchantorderid orders](struct.OrderGetbymerchantorderidCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersGetByMerchantOrderIdResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersGetByMerchantOrderIdResponse".
    pub kind: Option<String>,
    /// The requested order.
    pub order: Option<Order>,
}

impl ResponseResult for OrdersGetByMerchantOrderIdResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [setlineitemmetadata orders](struct.OrderSetlineitemmetadataCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersSetLineItemMetadataResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersSetLineItemMetadataResponse".
    pub kind: Option<String>,
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    pub execution_status: Option<String>,
}

impl ResponseResult for OrdersSetLineItemMetadataResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [cancel orders](struct.OrderCancelCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCancelRequest {
    /// The reason for the cancellation.
    pub reason: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    pub reason_text: Option<String>,
    /// The ID of the operation. Unique across all operations for a given order.
    #[serde(rename="operationId")]
    pub operation_id: Option<String>,
}

impl RequestValue for OrdersCancelRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderCustomerMarketingRightsInfo {
    /// Last known user selection regarding marketing preferences. In certain cases this selection might not be known, so this field would be empty.
    #[serde(rename="explicitMarketingPreference")]
    pub explicit_marketing_preference: Option<String>,
    /// Timestamp when last time marketing preference was updated. Could be empty, if user wasn't offered a selection yet.
    #[serde(rename="lastUpdatedTimestamp")]
    pub last_updated_timestamp: Option<String>,
    /// Email address that can be used for marketing purposes. This field is only filled when explicitMarketingPreference is equal to 'granted'.
    #[serde(rename="marketingEmailAddress")]
    pub marketing_email_address: Option<String>,
}

impl Part for OrderCustomerMarketingRightsInfo {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list orders](struct.OrderListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersListResponse {
    /// The token for the retrieval of the next page of orders.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersListResponse".
    pub kind: Option<String>,
    /// no description provided
    pub resources: Option<Vec<Order>>,
}

impl ResponseResult for OrdersListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get orderreturns](struct.OrderreturnGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MerchantOrderReturn {
    /// no description provided
    #[serde(rename="orderId")]
    pub order_id: Option<String>,
    /// no description provided
    #[serde(rename="merchantOrderId")]
    pub merchant_order_id: Option<String>,
    /// no description provided
    #[serde(rename="orderReturnId")]
    pub order_return_id: Option<String>,
    /// no description provided
    #[serde(rename="creationDate")]
    pub creation_date: Option<String>,
    /// no description provided
    #[serde(rename="returnShipments")]
    pub return_shipments: Option<Vec<ReturnShipment>>,
    /// no description provided
    #[serde(rename="returnItems")]
    pub return_items: Option<Vec<MerchantOrderReturnItem>>,
}

impl ResponseResult for MerchantOrderReturn {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderShipmentLineItemShipment {
    /// The ID of the product to ship. This is the REST ID used in the products service. Either lineItemId or productId is required.
    #[serde(rename="productId")]
    pub product_id: Option<String>,
    /// The id of the line item that is shipped. Either lineItemId or productId is required.
    #[serde(rename="lineItemId")]
    pub line_item_id: Option<String>,
    /// The quantity that is shipped.
    pub quantity: Option<u32>,
}

impl Part for OrderShipmentLineItemShipment {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [returnlineitem orders](struct.OrderReturnlineitemCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersReturnLineItemResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersReturnLineItemResponse".
    pub kind: Option<String>,
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    pub execution_status: Option<String>,
}

impl ResponseResult for OrdersReturnLineItemResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderCustomer {
    /// Deprecated. Please use marketingRightsInfo instead.
    #[serde(rename="explicitMarketingPreference")]
    pub explicit_marketing_preference: Option<bool>,
    /// Full name of the customer.
    #[serde(rename="fullName")]
    pub full_name: Option<String>,
    /// Deprecated.
    pub email: Option<String>,
    /// Customer's marketing preferences.
    #[serde(rename="marketingRightsInfo")]
    pub marketing_rights_info: Option<OrderCustomerMarketingRightsInfo>,
}

impl Part for OrderCustomer {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [refund orders](struct.OrderRefundCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersRefundRequest {
    /// The amount that is refunded. Either amount or amountPretax and amountTax should be filled.
    #[serde(rename="amountPretax")]
    pub amount_pretax: Option<Price>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    pub reason_text: Option<String>,
    /// The reason for the refund.
    pub reason: Option<String>,
    /// Tax amount that correspond to refund amount in amountPretax.
    #[serde(rename="amountTax")]
    pub amount_tax: Option<Price>,
    /// Deprecated. Please use amountPretax and amountTax instead.
    pub amount: Option<Price>,
    /// The ID of the operation. Unique across all operations for a given order.
    #[serde(rename="operationId")]
    pub operation_id: Option<String>,
}

impl RequestValue for OrdersRefundRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [rejectreturnlineitem orders](struct.OrderRejectreturnlineitemCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersRejectReturnLineItemResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersRejectReturnLineItemResponse".
    pub kind: Option<String>,
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    pub execution_status: Option<String>,
}

impl ResponseResult for OrdersRejectReturnLineItemResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notifyrefund orderpayments](struct.OrderpaymentNotifyrefundCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderpaymentsNotifyRefundResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#orderpaymentsNotifyRefundResponse".
    pub kind: Option<String>,
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    pub execution_status: Option<String>,
}

impl ResponseResult for OrderpaymentsNotifyRefundResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instorerefundlineitem orders](struct.OrderInstorerefundlineitemCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersInStoreRefundLineItemResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersInStoreRefundLineItemResponse".
    pub kind: Option<String>,
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    pub execution_status: Option<String>,
}

impl ResponseResult for OrdersInStoreRefundLineItemResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntry {
    /// The ID of the order. Required for all methods beside getByMerchantOrderId.
    #[serde(rename="orderId")]
    pub order_id: Option<String>,
    /// Required for rejectReturnLineItem method.
    #[serde(rename="rejectReturnLineItem")]
    pub reject_return_line_item: Option<OrdersCustomBatchRequestEntryRejectReturnLineItem>,
    /// Required for updateLineItemShippingDate method.
    #[serde(rename="updateLineItemShippingDetails")]
    pub update_line_item_shipping_details: Option<OrdersCustomBatchRequestEntryUpdateLineItemShippingDetails>,
    /// Required for inStoreReturnLineItem method.
    #[serde(rename="inStoreRefundLineItem")]
    pub in_store_refund_line_item: Option<OrdersCustomBatchRequestEntryInStoreRefundLineItem>,
    /// Required for updateShipment method.
    #[serde(rename="updateShipment")]
    pub update_shipment: Option<OrdersCustomBatchRequestEntryUpdateShipment>,
    /// Required for cancelLineItem method.
    #[serde(rename="cancelLineItem")]
    pub cancel_line_item: Option<OrdersCustomBatchRequestEntryCancelLineItem>,
    /// Required for refund method.
    pub refund: Option<OrdersCustomBatchRequestEntryRefund>,
    /// Required for cancel method.
    pub cancel: Option<OrdersCustomBatchRequestEntryCancel>,
    /// The ID of the managing account.
    #[serde(rename="merchantId")]
    pub merchant_id: Option<String>,
    /// Required for returnLineItem method.
    #[serde(rename="returnLineItem")]
    pub return_line_item: Option<OrdersCustomBatchRequestEntryReturnLineItem>,
    /// The merchant order id. Required for updateMerchantOrderId and getByMerchantOrderId methods.
    #[serde(rename="merchantOrderId")]
    pub merchant_order_id: Option<String>,
    /// Required for returnRefundLineItem method.
    #[serde(rename="returnRefundLineItem")]
    pub return_refund_line_item: Option<OrdersCustomBatchRequestEntryReturnRefundLineItem>,
    /// Required for shipLineItems method.
    #[serde(rename="shipLineItems")]
    pub ship_line_items: Option<OrdersCustomBatchRequestEntryShipLineItems>,
    /// Required for setLineItemMetadata method.
    #[serde(rename="setLineItemMetadata")]
    pub set_line_item_metadata: Option<OrdersCustomBatchRequestEntrySetLineItemMetadata>,
    /// An entry ID, unique within the batch request.
    #[serde(rename="batchId")]
    pub batch_id: Option<u32>,
    /// The method to apply.
    pub method: Option<String>,
    /// The ID of the operation. Unique across all operations for a given order. Required for all methods beside get and getByMerchantOrderId.
    #[serde(rename="operationId")]
    pub operation_id: Option<String>,
}

impl Part for OrdersCustomBatchRequestEntry {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [createtestreturn orders](struct.OrderCreatetestreturnCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCreateTestReturnResponse {
    /// The ID of the newly created test order return.
    #[serde(rename="returnId")]
    pub return_id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersCreateTestReturnResponse".
    pub kind: Option<String>,
}

impl ResponseResult for OrdersCreateTestReturnResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntryCancelLineItem {
    /// Deprecated. Please use amountPretax and amountTax instead.
    pub amount: Option<Price>,
    /// Amount to refund for the cancelation. Optional. If not set, Google will calculate the default based on the price and tax of the items involved. The amount must not be larger than the net amount left on the order.
    #[serde(rename="amountPretax")]
    pub amount_pretax: Option<Price>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    pub reason_text: Option<String>,
    /// The ID of the line item to cancel. Either lineItemId or productId is required.
    #[serde(rename="lineItemId")]
    pub line_item_id: Option<String>,
    /// The reason for the cancellation.
    pub reason: Option<String>,
    /// Tax amount that correspond to cancellation amount in amountPretax.
    #[serde(rename="amountTax")]
    pub amount_tax: Option<Price>,
    /// The quantity to cancel.
    pub quantity: Option<u32>,
    /// The ID of the product to cancel. This is the REST ID used in the products service. Either lineItemId or productId is required.
    #[serde(rename="productId")]
    pub product_id: Option<String>,
}

impl Part for OrdersCustomBatchRequestEntryCancelLineItem {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [acknowledge orders](struct.OrderAcknowledgeCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersAcknowledgeRequest {
    /// The ID of the operation. Unique across all operations for a given order.
    #[serde(rename="operationId")]
    pub operation_id: Option<String>,
}

impl RequestValue for OrdersAcknowledgeRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderMerchantProvidedAnnotation {
    /// Key for additional merchant provided (as key-value pairs) annotation about the line item.
    pub key: Option<String>,
    /// Value for additional merchant provided (as key-value pairs) annotation about the line item.
    pub value: Option<String>,
}

impl Part for OrderMerchantProvidedAnnotation {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [refund orders](struct.OrderRefundCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersRefundResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersRefundResponse".
    pub kind: Option<String>,
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    pub execution_status: Option<String>,
}

impl ResponseResult for OrdersRefundResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntryRefund {
    /// The reason for the refund.
    pub reason: Option<String>,
    /// Deprecated. Please use amountPretax and amountTax instead.
    pub amount: Option<Price>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    pub reason_text: Option<String>,
    /// Tax amount that correspond to refund amount in amountPretax.
    #[serde(rename="amountTax")]
    pub amount_tax: Option<Price>,
    /// The amount that is refunded. Either amount or amountPretax and amountTax should be filled.
    #[serde(rename="amountPretax")]
    pub amount_pretax: Option<Price>,
}

impl Part for OrdersCustomBatchRequestEntryRefund {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [updatemerchantorderid orders](struct.OrderUpdatemerchantorderidCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersUpdateMerchantOrderIdRequest {
    /// The merchant order id to be assigned to the order. Must be unique per merchant.
    #[serde(rename="merchantOrderId")]
    pub merchant_order_id: Option<String>,
    /// The ID of the operation. Unique across all operations for a given order.
    #[serde(rename="operationId")]
    pub operation_id: Option<String>,
}

impl RequestValue for OrdersUpdateMerchantOrderIdRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InvoiceSummaryAdditionalChargeSummary {
    /// [required] Total additional charge for this type.
    #[serde(rename="totalAmount")]
    pub total_amount: Option<Amount>,
    /// [required] Type of the additional charge.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for InvoiceSummaryAdditionalChargeSummary {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [shiplineitems orders](struct.OrderShiplineitemCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersShipLineItemsResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersShipLineItemsResponse".
    pub kind: Option<String>,
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    pub execution_status: Option<String>,
}

impl ResponseResult for OrdersShipLineItemsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notifycharge orderpayments](struct.OrderpaymentNotifychargeCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderpaymentsNotifyChargeResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#orderpaymentsNotifyChargeResponse".
    pub kind: Option<String>,
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    pub execution_status: Option<String>,
}

impl ResponseResult for OrderpaymentsNotifyChargeResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestOrder {
    /// The details of the customer who placed the order.
    pub customer: Option<TestOrderCustomer>,
    /// Deprecated. The details of the merchant provided promotions applied to the order. More details about the program are here.
    pub promotions: Option<Vec<OrderLegacyPromotion>>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#testOrder".
    pub kind: Option<String>,
    /// Line items that are ordered. At least one line item must be provided.
    #[serde(rename="lineItems")]
    pub line_items: Option<Vec<TestOrderLineItem>>,
    /// Determines if test order must be pulled by merchant or pushed to merchant via push integration.
    #[serde(rename="notificationMode")]
    pub notification_mode: Option<String>,
    /// Identifier of one of the predefined delivery addresses for the delivery.
    #[serde(rename="predefinedDeliveryAddress")]
    pub predefined_delivery_address: Option<String>,
    /// The total cost of shipping for all items.
    #[serde(rename="shippingCost")]
    pub shipping_cost: Option<Price>,
    /// The requested shipping option.
    #[serde(rename="shippingOption")]
    pub shipping_option: Option<String>,
    /// Whether the orderinvoices service should support this order.
    #[serde(rename="enableOrderinvoices")]
    pub enable_orderinvoices: Option<bool>,
    /// The tax for the total shipping cost.
    #[serde(rename="shippingCostTax")]
    pub shipping_cost_tax: Option<Price>,
    /// The details of the payment method.
    #[serde(rename="paymentMethod")]
    pub payment_method: Option<TestOrderPaymentMethod>,
}

impl Part for TestOrder {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReturnShipment {
    /// no description provided
    #[serde(rename="returnMethodType")]
    pub return_method_type: Option<String>,
    /// no description provided
    #[serde(rename="creationDate")]
    pub creation_date: Option<String>,
    /// no description provided
    #[serde(rename="shipmentTrackingInfos")]
    pub shipment_tracking_infos: Option<Vec<ShipmentTrackingInfo>>,
    /// no description provided
    #[serde(rename="shipmentId")]
    pub shipment_id: Option<String>,
}

impl Part for ReturnShipment {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [createtestorder orders](struct.OrderCreatetestorderCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCreateTestOrderRequest {
    /// The  CLDR territory code of the country of the test order to create. Affects the currency and addresses of orders created via template_name, or the addresses of orders created via test_order.
    /// 
    /// Acceptable values are:  
    /// - "US" 
    /// - "FR"  Defaults to US.
    pub country: Option<String>,
    /// The test order to create.
    #[serde(rename="testOrder")]
    pub test_order: Option<TestOrder>,
    /// The test order template to use. Specify as an alternative to testOrder as a shortcut for retrieving a template and then creating an order using that template.
    #[serde(rename="templateName")]
    pub template_name: Option<String>,
}

impl RequestValue for OrdersCreateTestOrderRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [canceltestorderbycustomer orders](struct.OrderCanceltestorderbycustomerCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCancelTestOrderByCustomerRequest {
    /// The reason for the cancellation.
    pub reason: Option<String>,
}

impl RequestValue for OrdersCancelTestOrderByCustomerRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderPaymentMethod {
    /// The card expiration month (January = 1, February = 2 etc.).
    #[serde(rename="expirationMonth")]
    pub expiration_month: Option<i32>,
    /// The card expiration year (4-digit, e.g. 2015).
    #[serde(rename="expirationYear")]
    pub expiration_year: Option<i32>,
    /// The billing phone number.
    #[serde(rename="phoneNumber")]
    pub phone_number: Option<String>,
    /// The billing address.
    #[serde(rename="billingAddress")]
    pub billing_address: Option<OrderAddress>,
    /// The type of instrument.
    /// 
    /// Acceptable values are:  
    /// - "AMEX" 
    /// - "DISCOVER" 
    /// - "JCB" 
    /// - "MASTERCARD" 
    /// - "UNIONPAY" 
    /// - "VISA" 
    /// - ""
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The last four digits of the card number.
    #[serde(rename="lastFourDigits")]
    pub last_four_digits: Option<String>,
}

impl Part for OrderPaymentMethod {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [advancetestorder orders](struct.OrderAdvancetestorderCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersAdvanceTestOrderResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersAdvanceTestOrderResponse".
    pub kind: Option<String>,
}

impl ResponseResult for OrdersAdvanceTestOrderResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntryReturnLineItem {
    /// The reason for the return.
    pub reason: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    pub reason_text: Option<String>,
    /// The ID of the product to return. This is the REST ID used in the products service. Either lineItemId or productId is required.
    #[serde(rename="productId")]
    pub product_id: Option<String>,
    /// The ID of the line item to return. Either lineItemId or productId is required.
    #[serde(rename="lineItemId")]
    pub line_item_id: Option<String>,
    /// The quantity to return.
    pub quantity: Option<u32>,
}

impl Part for OrdersCustomBatchRequestEntryReturnLineItem {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [createtestreturn orders](struct.OrderCreatetestreturnCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCreateTestReturnRequest {
    /// Returned items.
    pub items: Option<Vec<OrdersCustomBatchRequestEntryCreateTestReturnReturnItem>>,
}

impl RequestValue for OrdersCreateTestReturnRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShipmentInvoiceLineItemInvoice {
    /// [required] Invoice details for a single unit.
    #[serde(rename="unitInvoice")]
    pub unit_invoice: Option<UnitInvoice>,
    /// [required] Unit IDs to define specific units within the line item.
    #[serde(rename="shipmentUnitIds")]
    pub shipment_unit_ids: Option<Vec<String>>,
    /// ID of the line item. Either lineItemId or productId must be set.
    #[serde(rename="lineItemId")]
    pub line_item_id: Option<String>,
    /// ID of the product. This is the REST ID used in the products service. Either lineItemId or productId must be set.
    #[serde(rename="productId")]
    pub product_id: Option<String>,
}

impl Part for ShipmentInvoiceLineItemInvoice {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notifycharge orderpayments](struct.OrderpaymentNotifychargeCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderpaymentsNotifyChargeRequest {
    /// Whether charge was successful.
    #[serde(rename="chargeState")]
    pub charge_state: Option<String>,
    /// Deprecated. Please use invoiceIds instead.
    #[serde(rename="invoiceId")]
    pub invoice_id: Option<String>,
    /// Invoice IDs from the orderinvoices service that correspond to the charge.
    #[serde(rename="invoiceIds")]
    pub invoice_ids: Option<Vec<String>>,
}

impl RequestValue for OrderpaymentsNotifyChargeRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch orders](struct.OrderCustombatchCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequest {
    /// The request entries to be processed in the batch.
    pub entries: Option<Vec<OrdersCustomBatchRequestEntry>>,
}

impl RequestValue for OrdersCustomBatchRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [returnrefundlineitem orders](struct.OrderReturnrefundlineitemCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersReturnRefundLineItemResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersReturnRefundLineItemResponse".
    pub kind: Option<String>,
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    pub execution_status: Option<String>,
}

impl ResponseResult for OrdersReturnRefundLineItemResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [returnrefundlineitem orders](struct.OrderReturnrefundlineitemCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersReturnRefundLineItemRequest {
    /// The quantity to return and refund.
    pub quantity: Option<u32>,
    /// The amount that is refunded. If omitted, refundless return is assumed (same as calling returnLineItem method). Optional, but if filled then both amountPretax and amountTax must be set.
    #[serde(rename="amountPretax")]
    pub amount_pretax: Option<Price>,
    /// The reason for the return.
    pub reason: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    pub reason_text: Option<String>,
    /// The ID of the line item to return. Either lineItemId or productId is required.
    #[serde(rename="lineItemId")]
    pub line_item_id: Option<String>,
    /// Tax amount that correspond to refund amount in amountPretax.
    #[serde(rename="amountTax")]
    pub amount_tax: Option<Price>,
    /// The ID of the product to return. This is the REST ID used in the products service. Either lineItemId or productId is required.
    #[serde(rename="productId")]
    pub product_id: Option<String>,
    /// The ID of the operation. Unique across all operations for a given order.
    #[serde(rename="operationId")]
    pub operation_id: Option<String>,
}

impl RequestValue for OrdersReturnRefundLineItemRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [shiplineitems orders](struct.OrderShiplineitemCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersShipLineItemsRequest {
    /// Line items to ship.
    #[serde(rename="lineItems")]
    pub line_items: Option<Vec<OrderShipmentLineItemShipment>>,
    /// Deprecated. Please use shipmentInfo instead. The carrier handling the shipment. See shipments[].carrier in the  Orders resource representation for a list of acceptable values.
    pub carrier: Option<String>,
    /// Deprecated. Please use shipmentInfo instead. The tracking id for the shipment.
    #[serde(rename="trackingId")]
    pub tracking_id: Option<String>,
    /// ID of the shipment group. Required for orders that use the orderinvoices service.
    #[serde(rename="shipmentGroupId")]
    pub shipment_group_id: Option<String>,
    /// Deprecated. Please use shipmentInfo instead. The ID of the shipment.
    #[serde(rename="shipmentId")]
    pub shipment_id: Option<String>,
    /// Shipment information. This field is repeated because a single line item can be shipped in several packages (and have several tracking IDs).
    #[serde(rename="shipmentInfos")]
    pub shipment_infos: Option<Vec<OrdersCustomBatchRequestEntryShipLineItemsShipmentInfo>>,
    /// The ID of the operation. Unique across all operations for a given order.
    #[serde(rename="operationId")]
    pub operation_id: Option<String>,
}

impl RequestValue for OrdersShipLineItemsRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [updatelineitemshippingdetails orders](struct.OrderUpdatelineitemshippingdetailCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersUpdateLineItemShippingDetailsResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersUpdateLineItemShippingDetailsResponse".
    pub kind: Option<String>,
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    pub execution_status: Option<String>,
}

impl ResponseResult for OrdersUpdateLineItemShippingDetailsResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderShipment {
    /// The status of the shipment.
    pub status: Option<String>,
    /// Date on which the shipment has been created, in ISO 8601 format.
    #[serde(rename="creationDate")]
    pub creation_date: Option<String>,
    /// The carrier handling the shipment.
    /// 
    /// Acceptable values are:  
    /// - "gsx" 
    /// - "ups" 
    /// - "usps" 
    /// - "fedex" 
    /// - "dhl" 
    /// - "ecourier" 
    /// - "cxt" 
    /// - "google" 
    /// - "ontrac" 
    /// - "emsy" 
    /// - "ont" 
    /// - "deliv" 
    /// - "dynamex" 
    /// - "lasership" 
    /// - "mpx" 
    /// - "uds"
    pub carrier: Option<String>,
    /// The tracking id for the shipment.
    #[serde(rename="trackingId")]
    pub tracking_id: Option<String>,
    /// Date on which the shipment has been delivered, in ISO 8601 format. Present only if status is delivered
    #[serde(rename="deliveryDate")]
    pub delivery_date: Option<String>,
    /// The line items that are shipped.
    #[serde(rename="lineItems")]
    pub line_items: Option<Vec<OrderShipmentLineItemShipment>>,
    /// The id of the shipment.
    pub id: Option<String>,
}

impl Part for OrderShipment {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntryRejectReturnLineItem {
    /// The reason for the return.
    pub reason: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    pub reason_text: Option<String>,
    /// The ID of the product to return. This is the REST ID used in the products service. Either lineItemId or productId is required.
    #[serde(rename="productId")]
    pub product_id: Option<String>,
    /// The ID of the line item to return. Either lineItemId or productId is required.
    #[serde(rename="lineItemId")]
    pub line_item_id: Option<String>,
    /// The quantity to return and refund.
    pub quantity: Option<u32>,
}

impl Part for OrdersCustomBatchRequestEntryRejectReturnLineItem {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestOrderLineItem {
    /// Details of the return policy for the line item.
    #[serde(rename="returnInfo")]
    pub return_info: Option<OrderLineItemReturnInfo>,
    /// Product data from the time of the order placement.
    pub product: Option<TestOrderLineItemProduct>,
    /// Unit tax for the line item.
    #[serde(rename="unitTax")]
    pub unit_tax: Option<Price>,
    /// Number of items ordered.
    #[serde(rename="quantityOrdered")]
    pub quantity_ordered: Option<u32>,
    /// Details of the requested shipping for the line item.
    #[serde(rename="shippingDetails")]
    pub shipping_details: Option<OrderLineItemShippingDetails>,
}

impl Part for TestOrderLineItem {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [updatemerchantorderid orders](struct.OrderUpdatemerchantorderidCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersUpdateMerchantOrderIdResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersUpdateMerchantOrderIdResponse".
    pub kind: Option<String>,
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    pub execution_status: Option<String>,
}

impl ResponseResult for OrdersUpdateMerchantOrderIdResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notifyauthapproved orderpayments](struct.OrderpaymentNotifyauthapprovedCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderpaymentsNotifyAuthApprovedResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#orderpaymentsNotifyAuthApprovedResponse".
    pub kind: Option<String>,
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    pub execution_status: Option<String>,
}

impl ResponseResult for OrderpaymentsNotifyAuthApprovedResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceRefundOption {
    /// [required] Reason for the refund.
    pub reason: Option<String>,
    /// Optional description of the refund reason.
    pub description: Option<String>,
}

impl Part for OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceRefundOption {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderLineItemShippingDetails {
    /// The delivery by date, in ISO 8601 format.
    #[serde(rename="deliverByDate")]
    pub deliver_by_date: Option<String>,
    /// Details of the shipping method.
    pub method: Option<OrderLineItemShippingDetailsMethod>,
    /// The ship by date, in ISO 8601 format.
    #[serde(rename="shipByDate")]
    pub ship_by_date: Option<String>,
}

impl Part for OrderLineItemShippingDetails {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Price {
    /// The currency of the price.
    pub currency: Option<String>,
    /// The price represented as a number.
    pub value: Option<String>,
}

impl Part for Price {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [rejectreturnlineitem orders](struct.OrderRejectreturnlineitemCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersRejectReturnLineItemRequest {
    /// The reason for the return.
    pub reason: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    pub reason_text: Option<String>,
    /// The ID of the line item to return. Either lineItemId or productId is required.
    #[serde(rename="lineItemId")]
    pub line_item_id: Option<String>,
    /// The ID of the operation. Unique across all operations for a given order.
    #[serde(rename="operationId")]
    pub operation_id: Option<String>,
    /// The ID of the product to return. This is the REST ID used in the products service. Either lineItemId or productId is required.
    #[serde(rename="productId")]
    pub product_id: Option<String>,
    /// The quantity to return and refund.
    pub quantity: Option<u32>,
}

impl RequestValue for OrdersRejectReturnLineItemRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UnitInvoice {
    /// Promotions applied to a unit.
    pub promotions: Option<Vec<Promotion>>,
    /// [required] Price of the unit, before applying taxes.
    #[serde(rename="unitPricePretax")]
    pub unit_price_pretax: Option<Price>,
    /// Tax amounts to apply to the unit price.
    #[serde(rename="unitPriceTaxes")]
    pub unit_price_taxes: Option<Vec<UnitInvoiceTaxLine>>,
    /// Additional charges for a unit, e.g. shipping costs.
    #[serde(rename="additionalCharges")]
    pub additional_charges: Option<Vec<UnitInvoiceAdditionalCharge>>,
}

impl Part for UnitInvoice {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MerchantOrderReturnItem {
    /// no description provided
    #[serde(rename="itemId")]
    pub item_id: Option<String>,
    /// no description provided
    pub product: Option<OrderLineItemProduct>,
    /// no description provided
    #[serde(rename="merchantReturnReason")]
    pub merchant_return_reason: Option<RefundReason>,
    /// no description provided
    #[serde(rename="customerReturnReason")]
    pub customer_return_reason: Option<CustomerReturnReason>,
    /// no description provided
    pub state: Option<String>,
    /// no description provided
    #[serde(rename="returnShipmentIds")]
    pub return_shipment_ids: Option<Vec<String>>,
}

impl Part for MerchantOrderReturnItem {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntryShipLineItemsShipmentInfo {
    /// The carrier handling the shipment. See shipments[].carrier in the  Orders resource representation for a list of acceptable values.
    pub carrier: Option<String>,
    /// The tracking id for the shipment.
    #[serde(rename="trackingId")]
    pub tracking_id: Option<String>,
    /// The ID of the shipment.
    #[serde(rename="shipmentId")]
    pub shipment_id: Option<String>,
}

impl Part for OrdersCustomBatchRequestEntryShipLineItemsShipmentInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestOrderPaymentMethod {
    /// The card expiration month (January = 1, February = 2 etc.).
    #[serde(rename="expirationMonth")]
    pub expiration_month: Option<i32>,
    /// The last four digits of the card number.
    #[serde(rename="lastFourDigits")]
    pub last_four_digits: Option<String>,
    /// The card expiration year (4-digit, e.g. 2015).
    #[serde(rename="expirationYear")]
    pub expiration_year: Option<i32>,
    /// The type of instrument. Note that real orders might have different values than the four values accepted by createTestOrder.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The billing address.
    #[serde(rename="predefinedBillingAddress")]
    pub predefined_billing_address: Option<String>,
}

impl Part for TestOrderPaymentMethod {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [setlineitemmetadata orders](struct.OrderSetlineitemmetadataCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersSetLineItemMetadataRequest {
    /// The ID of the operation. Unique across all operations for a given order.
    #[serde(rename="operationId")]
    pub operation_id: Option<String>,
    /// no description provided
    pub annotations: Option<Vec<OrderMerchantProvidedAnnotation>>,
    /// The ID of the line item to set metadata. Either lineItemId or productId is required.
    #[serde(rename="lineItemId")]
    pub line_item_id: Option<String>,
    /// The ID of the product to set metadata. This is the REST ID used in the products service. Either lineItemId or productId is required.
    #[serde(rename="productId")]
    pub product_id: Option<String>,
}

impl RequestValue for OrdersSetLineItemMetadataRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderLineItemProductVariantAttribute {
    /// The dimension of the variant.
    pub dimension: Option<String>,
    /// The value for the dimension.
    pub value: Option<String>,
}

impl Part for OrderLineItemProductVariantAttribute {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [returnlineitem orders](struct.OrderReturnlineitemCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersReturnLineItemRequest {
    /// The reason for the return.
    pub reason: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    pub reason_text: Option<String>,
    /// The ID of the line item to return. Either lineItemId or productId is required.
    #[serde(rename="lineItemId")]
    pub line_item_id: Option<String>,
    /// The ID of the operation. Unique across all operations for a given order.
    #[serde(rename="operationId")]
    pub operation_id: Option<String>,
    /// The ID of the product to return. This is the REST ID used in the products service. Either lineItemId or productId is required.
    #[serde(rename="productId")]
    pub product_id: Option<String>,
    /// The quantity to return.
    pub quantity: Option<u32>,
}

impl RequestValue for OrdersReturnLineItemRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderCancellation {
    /// The quantity that was canceled.
    pub quantity: Option<u32>,
    /// The reason for the cancellation. Orders that are cancelled with a noInventory reason will lead to the removal of the product from Shopping Actions until you make an update to that product. This will not affect your Shopping ads.
    pub reason: Option<String>,
    /// Date on which the cancellation has been created, in ISO 8601 format.
    #[serde(rename="creationDate")]
    pub creation_date: Option<String>,
    /// The actor that created the cancellation.
    pub actor: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    pub reason_text: Option<String>,
}

impl Part for OrderCancellation {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntryCancel {
    /// The reason for the cancellation.
    pub reason: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    pub reason_text: Option<String>,
}

impl Part for OrdersCustomBatchRequestEntryCancel {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notifyrefund orderpayments](struct.OrderpaymentNotifyrefundCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderpaymentsNotifyRefundRequest {
    /// Deprecated. Please use invoiceIds instead.
    #[serde(rename="invoiceId")]
    pub invoice_id: Option<String>,
    /// Whether refund was successful.
    #[serde(rename="refundState")]
    pub refund_state: Option<String>,
    /// Invoice IDs from the orderinvoices service that correspond to the refund.
    #[serde(rename="invoiceIds")]
    pub invoice_ids: Option<Vec<String>>,
}

impl RequestValue for OrderpaymentsNotifyRefundRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch orders](struct.OrderCustombatchCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersCustomBatchResponse".
    pub kind: Option<String>,
    /// The result of the execution of the batch requests.
    pub entries: Option<Vec<OrdersCustomBatchResponseEntry>>,
}

impl ResponseResult for OrdersCustomBatchResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [canceltestorderbycustomer orders](struct.OrderCanceltestorderbycustomerCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCancelTestOrderByCustomerResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersCancelTestOrderByCustomerResponse".
    pub kind: Option<String>,
}

impl ResponseResult for OrdersCancelTestOrderByCustomerResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntryCreateTestReturnReturnItem {
    /// The ID of the line item to return.
    #[serde(rename="lineItemId")]
    pub line_item_id: Option<String>,
    /// Quantity that is returned.
    pub quantity: Option<u32>,
}

impl Part for OrdersCustomBatchRequestEntryCreateTestReturnReturnItem {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notifyauthdeclined orderpayments](struct.OrderpaymentNotifyauthdeclinedCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderpaymentsNotifyAuthDeclinedRequest {
    /// Reason why payment authorization was declined.
    #[serde(rename="declineReason")]
    pub decline_reason: Option<String>,
}

impl RequestValue for OrderpaymentsNotifyAuthDeclinedRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShipmentTrackingInfo {
    /// no description provided
    #[serde(rename="trackingNumber")]
    pub tracking_number: Option<String>,
    /// no description provided
    pub carrier: Option<String>,
}

impl Part for ShipmentTrackingInfo {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [cancellineitem orders](struct.OrderCancellineitemCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCancelLineItemRequest {
    /// The ID of the product to cancel. This is the REST ID used in the products service. Either lineItemId or productId is required.
    #[serde(rename="productId")]
    pub product_id: Option<String>,
    /// Deprecated. Please use amountPretax and amountTax instead.
    pub amount: Option<Price>,
    /// Amount to refund for the cancelation. Optional. If not set, Google will calculate the default based on the price and tax of the items involved. The amount must not be larger than the net amount left on the order.
    #[serde(rename="amountPretax")]
    pub amount_pretax: Option<Price>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    pub reason_text: Option<String>,
    /// The ID of the line item to cancel. Either lineItemId or productId is required.
    #[serde(rename="lineItemId")]
    pub line_item_id: Option<String>,
    /// The reason for the cancellation.
    pub reason: Option<String>,
    /// Tax amount that correspond to cancellation amount in amountPretax.
    #[serde(rename="amountTax")]
    pub amount_tax: Option<Price>,
    /// The ID of the operation. Unique across all operations for a given order.
    #[serde(rename="operationId")]
    pub operation_id: Option<String>,
    /// The quantity to cancel.
    pub quantity: Option<u32>,
}

impl RequestValue for OrdersCancelLineItemRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntrySetLineItemMetadata {
    /// no description provided
    pub annotations: Option<Vec<OrderMerchantProvidedAnnotation>>,
    /// The ID of the line item to set metadata. Either lineItemId or productId is required.
    #[serde(rename="lineItemId")]
    pub line_item_id: Option<String>,
    /// The ID of the product to set metadata. This is the REST ID used in the products service. Either lineItemId or productId is required.
    #[serde(rename="productId")]
    pub product_id: Option<String>,
}

impl Part for OrdersCustomBatchRequestEntrySetLineItemMetadata {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceReturnOption {
    /// [required] Reason for the return.
    pub reason: Option<String>,
    /// Optional description of the return reason.
    pub description: Option<String>,
}

impl Part for OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceReturnOption {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [gettestordertemplate orders](struct.OrderGettestordertemplateCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersGetTestOrderTemplateResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersGetTestOrderTemplateResponse".
    pub kind: Option<String>,
    /// The requested test order template.
    pub template: Option<TestOrder>,
}

impl ResponseResult for OrdersGetTestOrderTemplateResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Promotion {
    /// [required] Amount of the promotion. The values here are the promotion applied to the unit price pretax and to the total of the tax amounts.
    #[serde(rename="promotionAmount")]
    pub promotion_amount: Option<Amount>,
    /// [required] ID of the promotion.
    #[serde(rename="promotionId")]
    pub promotion_id: Option<String>,
}

impl Part for Promotion {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [createchargeinvoice orderinvoices](struct.OrderinvoiceCreatechargeinvoiceCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderinvoicesCreateChargeInvoiceRequest {
    /// [required] Invoice summary.
    #[serde(rename="invoiceSummary")]
    pub invoice_summary: Option<InvoiceSummary>,
    /// [required] The ID of the invoice.
    #[serde(rename="invoiceId")]
    pub invoice_id: Option<String>,
    /// [required] Invoice details per line item.
    #[serde(rename="lineItemInvoices")]
    pub line_item_invoices: Option<Vec<ShipmentInvoiceLineItemInvoice>>,
    /// [required] ID of the shipment group.
    #[serde(rename="shipmentGroupId")]
    pub shipment_group_id: Option<String>,
    /// [required] The ID of the operation, unique across all operations for a given order.
    #[serde(rename="operationId")]
    pub operation_id: Option<String>,
}

impl RequestValue for OrderinvoicesCreateChargeInvoiceRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list orderreturns](struct.OrderreturnListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderreturnsListResponse {
    /// The token for the retrieval of the next page of returns.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#orderreturnsListResponse".
    pub kind: Option<String>,
    /// no description provided
    pub resources: Option<Vec<MerchantOrderReturn>>,
}

impl ResponseResult for OrderreturnsListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list orders](struct.OrderListCall.html) (none)
/// * [updatemerchantorderid orders](struct.OrderUpdatemerchantorderidCall.html) (none)
/// * [gettestordertemplate orders](struct.OrderGettestordertemplateCall.html) (none)
/// * [updatelineitemshippingdetails orders](struct.OrderUpdatelineitemshippingdetailCall.html) (none)
/// * [refund orders](struct.OrderRefundCall.html) (none)
/// * [acknowledge orders](struct.OrderAcknowledgeCall.html) (none)
/// * [instorerefundlineitem orders](struct.OrderInstorerefundlineitemCall.html) (none)
/// * [advancetestorder orders](struct.OrderAdvancetestorderCall.html) (none)
/// * [rejectreturnlineitem orders](struct.OrderRejectreturnlineitemCall.html) (none)
/// * [createtestorder orders](struct.OrderCreatetestorderCall.html) (none)
/// * [cancellineitem orders](struct.OrderCancellineitemCall.html) (none)
/// * [canceltestorderbycustomer orders](struct.OrderCanceltestorderbycustomerCall.html) (none)
/// * [returnrefundlineitem orders](struct.OrderReturnrefundlineitemCall.html) (none)
/// * [get orders](struct.OrderGetCall.html) (response)
/// * [returnlineitem orders](struct.OrderReturnlineitemCall.html) (none)
/// * [custombatch orders](struct.OrderCustombatchCall.html) (none)
/// * [getbymerchantorderid orders](struct.OrderGetbymerchantorderidCall.html) (none)
/// * [shiplineitems orders](struct.OrderShiplineitemCall.html) (none)
/// * [createtestreturn orders](struct.OrderCreatetestreturnCall.html) (none)
/// * [updateshipment orders](struct.OrderUpdateshipmentCall.html) (none)
/// * [setlineitemmetadata orders](struct.OrderSetlineitemmetadataCall.html) (none)
/// * [cancel orders](struct.OrderCancelCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Order {
    /// Shipments of the order.
    pub shipments: Option<Vec<OrderShipment>>,
    /// The details of the customer who placed the order.
    pub customer: Option<OrderCustomer>,
    /// The total cost of shipping for all items.
    #[serde(rename="shippingCost")]
    pub shipping_cost: Option<Price>,
    /// The channel type of the order: "purchaseOnGoogle" or "googleExpress".
    #[serde(rename="channelType")]
    pub channel_type: Option<String>,
    /// The net amount for the order. For example, if an order was originally for a grand total of $100 and a refund was issued for $20, the net amount will be $80.
    #[serde(rename="netAmount")]
    pub net_amount: Option<Price>,
    /// The REST id of the order. Globally unique.
    pub id: Option<String>,
    /// Deprecated. The details of the merchant provided promotions applied to the order. More details about the program are here.
    pub promotions: Option<Vec<OrderLegacyPromotion>>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#order".
    pub kind: Option<String>,
    /// Line items that are ordered.
    #[serde(rename="lineItems")]
    pub line_items: Option<Vec<OrderLineItem>>,
    /// Refunds for the order.
    pub refunds: Option<Vec<OrderRefund>>,
    /// Whether the order was acknowledged.
    pub acknowledged: Option<bool>,
    /// The status of the payment.
    #[serde(rename="paymentStatus")]
    pub payment_status: Option<String>,
    /// no description provided
    #[serde(rename="merchantId")]
    pub merchant_id: Option<String>,
    /// Merchant-provided id of the order.
    #[serde(rename="merchantOrderId")]
    pub merchant_order_id: Option<String>,
    /// The requested shipping option.
    #[serde(rename="shippingOption")]
    pub shipping_option: Option<String>,
    /// The status of the order.
    pub status: Option<String>,
    /// The details for the delivery.
    #[serde(rename="deliveryDetails")]
    pub delivery_details: Option<OrderDeliveryDetails>,
    /// The date when the order was placed, in ISO 8601 format.
    #[serde(rename="placedDate")]
    pub placed_date: Option<String>,
    /// The tax for the total shipping cost.
    #[serde(rename="shippingCostTax")]
    pub shipping_cost_tax: Option<Price>,
    /// The details of the payment method.
    #[serde(rename="paymentMethod")]
    pub payment_method: Option<OrderPaymentMethod>,
}

impl Resource for Order {}
impl ResponseResult for Order {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderLineItem {
    /// Product data from the time of the order placement.
    pub product: Option<OrderLineItemProduct>,
    /// Number of items delivered.
    #[serde(rename="quantityDelivered")]
    pub quantity_delivered: Option<u32>,
    /// Details of the return policy for the line item.
    #[serde(rename="returnInfo")]
    pub return_info: Option<OrderLineItemReturnInfo>,
    /// Total price for the line item. For example, if two items for $10 are purchased, the total price will be $20.
    pub price: Option<Price>,
    /// Number of items pending.
    #[serde(rename="quantityPending")]
    pub quantity_pending: Option<u32>,
    /// Total tax amount for the line item. For example, if two items are purchased, and each have a cost tax of $2, the total tax amount will be $4.
    pub tax: Option<Price>,
    /// Cancellations of the line item.
    pub cancellations: Option<Vec<OrderCancellation>>,
    /// Number of items canceled.
    #[serde(rename="quantityCanceled")]
    pub quantity_canceled: Option<u32>,
    /// The id of the line item.
    pub id: Option<String>,
    /// Returns of the line item.
    pub returns: Option<Vec<OrderReturn>>,
    /// Number of items shipped.
    #[serde(rename="quantityShipped")]
    pub quantity_shipped: Option<u32>,
    /// Annotations that are attached to the line item.
    pub annotations: Option<Vec<OrderMerchantProvidedAnnotation>>,
    /// Number of items returned.
    #[serde(rename="quantityReturned")]
    pub quantity_returned: Option<u32>,
    /// Number of items ordered.
    #[serde(rename="quantityOrdered")]
    pub quantity_ordered: Option<u32>,
    /// Details of the requested shipping for the line item.
    #[serde(rename="shippingDetails")]
    pub shipping_details: Option<OrderLineItemShippingDetails>,
}

impl Part for OrderLineItem {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderRefund {
    /// The amount that is refunded.
    pub amount: Option<Price>,
    /// Date on which the item has been created, in ISO 8601 format.
    #[serde(rename="creationDate")]
    pub creation_date: Option<String>,
    /// The reason for the refund.
    pub reason: Option<String>,
    /// The actor that created the refund.
    pub actor: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    pub reason_text: Option<String>,
}

impl Part for OrderRefund {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [cancel orders](struct.OrderCancelCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCancelResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersCancelResponse".
    pub kind: Option<String>,
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    pub execution_status: Option<String>,
}

impl ResponseResult for OrdersCancelResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderLegacyPromotion {
    /// The full title of the promotion.
    #[serde(rename="longTitle")]
    pub long_title: Option<String>,
    /// Optional. The text code that corresponds to the promotion when applied on the retailer?s website.
    #[serde(rename="genericRedemptionCode")]
    pub generic_redemption_code: Option<String>,
    /// no description provided
    pub benefits: Option<Vec<OrderLegacyPromotionBenefit>>,
    /// Indicates that the promotion is valid online.
    #[serde(rename="redemptionChannel")]
    pub redemption_channel: Option<String>,
    /// Whether the promotion is applicable to all products or only specific products.
    #[serde(rename="productApplicability")]
    pub product_applicability: Option<String>,
    /// The date and time frame when the promotion is active and ready for validation review. Note that the promotion live time may be delayed for a few hours due to the validation review.
    /// Start date and end date are separated by a forward slash (/). The start date is specified by the format (YYYY-MM-DD), followed by the letter ?T?, the time of the day when the sale starts (in Greenwich Mean Time, GMT), followed by an expression of the time zone for the sale. The end date is in the same format.
    #[serde(rename="effectiveDates")]
    pub effective_dates: Option<String>,
    /// The unique ID of the promotion.
    pub id: Option<String>,
}

impl Part for OrderLegacyPromotion {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestOrderLineItemProduct {
    /// The CLDR territory code of the target country of the product.
    #[serde(rename="targetCountry")]
    pub target_country: Option<i64>,
    /// The two-letter ISO 639-1 language code for the item.
    #[serde(rename="contentLanguage")]
    pub content_language: Option<String>,
    /// The title of the product.
    pub title: Option<String>,
    /// Manufacturer Part Number (MPN) of the item. Optional.
    pub mpn: Option<String>,
    /// Variant attributes for the item. Optional.
    #[serde(rename="variantAttributes")]
    pub variant_attributes: Option<Vec<OrderLineItemProductVariantAttribute>>,
    /// Brand of the item.
    pub brand: Option<String>,
    /// URL of an image of the item.
    #[serde(rename="imageLink")]
    pub image_link: Option<String>,
    /// An identifier of the item.
    #[serde(rename="offerId")]
    pub offer_id: Option<String>,
    /// Shared identifier for all variants of the same product. Optional.
    #[serde(rename="itemGroupId")]
    pub item_group_id: Option<String>,
    /// Global Trade Item Number (GTIN) of the item. Optional.
    pub gtin: Option<String>,
    /// Condition or state of the item.
    pub condition: Option<String>,
    /// The price for the product.
    pub price: Option<Price>,
    /// The item's channel.
    pub channel: Option<String>,
}

impl Part for TestOrderLineItemProduct {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntryInStoreRefundLineItem {
    /// The amount that is refunded. Required.
    #[serde(rename="amountPretax")]
    pub amount_pretax: Option<Price>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    pub reason_text: Option<String>,
    /// The ID of the line item to return. Either lineItemId or productId is required.
    #[serde(rename="lineItemId")]
    pub line_item_id: Option<String>,
    /// Tax amount that correspond to refund amount in amountPretax. Required.
    #[serde(rename="amountTax")]
    pub amount_tax: Option<Price>,
    /// The quantity to return and refund.
    pub quantity: Option<u32>,
    /// The reason for the return.
    pub reason: Option<String>,
    /// The ID of the product to return. This is the REST ID used in the products service. Either lineItemId or productId is required.
    #[serde(rename="productId")]
    pub product_id: Option<String>,
}

impl Part for OrdersCustomBatchRequestEntryInStoreRefundLineItem {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [createtestorder orders](struct.OrderCreatetestorderCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCreateTestOrderResponse {
    /// The ID of the newly created test order.
    #[serde(rename="orderId")]
    pub order_id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersCreateTestOrderResponse".
    pub kind: Option<String>,
}

impl ResponseResult for OrdersCreateTestOrderResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderDeliveryDetails {
    /// The phone number of the person receiving the delivery.
    #[serde(rename="phoneNumber")]
    pub phone_number: Option<String>,
    /// The delivery address
    pub address: Option<OrderAddress>,
}

impl Part for OrderDeliveryDetails {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntryShipLineItems {
    /// Line items to ship.
    #[serde(rename="lineItems")]
    pub line_items: Option<Vec<OrderShipmentLineItemShipment>>,
    /// Deprecated. Please use shipmentInfo instead. The carrier handling the shipment. See shipments[].carrier in the  Orders resource representation for a list of acceptable values.
    pub carrier: Option<String>,
    /// Deprecated. Please use shipmentInfo instead. The tracking id for the shipment.
    #[serde(rename="trackingId")]
    pub tracking_id: Option<String>,
    /// ID of the shipment group. Required for orders that use the orderinvoices service.
    #[serde(rename="shipmentGroupId")]
    pub shipment_group_id: Option<String>,
    /// Deprecated. Please use shipmentInfo instead. The ID of the shipment.
    #[serde(rename="shipmentId")]
    pub shipment_id: Option<String>,
    /// Shipment information. This field is repeated because a single line item can be shipped in several packages (and have several tracking IDs).
    #[serde(rename="shipmentInfos")]
    pub shipment_infos: Option<Vec<OrdersCustomBatchRequestEntryShipLineItemsShipmentInfo>>,
}

impl Part for OrdersCustomBatchRequestEntryShipLineItems {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [acknowledge orders](struct.OrderAcknowledgeCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersAcknowledgeResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersAcknowledgeResponse".
    pub kind: Option<String>,
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    pub execution_status: Option<String>,
}

impl ResponseResult for OrdersAcknowledgeResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShipmentInvoice {
    /// [required] Invoice summary.
    #[serde(rename="invoiceSummary")]
    pub invoice_summary: Option<InvoiceSummary>,
    /// [required] Invoice details per line item.
    #[serde(rename="lineItemInvoices")]
    pub line_item_invoices: Option<Vec<ShipmentInvoiceLineItemInvoice>>,
    /// [required] ID of the shipment group.
    #[serde(rename="shipmentGroupId")]
    pub shipment_group_id: Option<String>,
}

impl Part for ShipmentInvoice {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Amount {
    /// [required] Value before taxes.
    pub pretax: Option<Price>,
    /// [required] Tax value.
    pub tax: Option<Price>,
}

impl Part for Amount {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RefundReason {
    /// no description provided
    #[serde(rename="reasonCode")]
    pub reason_code: Option<String>,
    /// no description provided
    pub description: Option<String>,
}

impl Part for RefundReason {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [createchargeinvoice orderinvoices](struct.OrderinvoiceCreatechargeinvoiceCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderinvoicesCreateChargeInvoiceResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#orderinvoicesCreateChargeInvoiceResponse".
    pub kind: Option<String>,
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    pub execution_status: Option<String>,
}

impl ResponseResult for OrderinvoicesCreateChargeInvoiceResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntryReturnRefundLineItem {
    /// The amount that is refunded. If omitted, refundless return is assumed (same as calling returnLineItem method). Optional, but if filled then both amountPretax and amountTax must be set.
    #[serde(rename="amountPretax")]
    pub amount_pretax: Option<Price>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    pub reason_text: Option<String>,
    /// The ID of the line item to return. Either lineItemId or productId is required.
    #[serde(rename="lineItemId")]
    pub line_item_id: Option<String>,
    /// Tax amount that correspond to refund amount in amountPretax.
    #[serde(rename="amountTax")]
    pub amount_tax: Option<Price>,
    /// The quantity to return and refund.
    pub quantity: Option<u32>,
    /// The reason for the return.
    pub reason: Option<String>,
    /// The ID of the product to return. This is the REST ID used in the products service. Either lineItemId or productId is required.
    #[serde(rename="productId")]
    pub product_id: Option<String>,
}

impl Part for OrdersCustomBatchRequestEntryReturnRefundLineItem {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchResponseEntry {
    /// The ID of the request entry this entry responds to.
    #[serde(rename="batchId")]
    pub batch_id: Option<u32>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersCustomBatchResponseEntry".
    pub kind: Option<String>,
    /// A list of errors defined if and only if the request failed.
    pub errors: Option<Errors>,
    /// The retrieved order. Only defined if the method is get and if the request was successful.
    pub order: Option<Order>,
    /// The status of the execution. Only defined if  
    /// - the request was successful; and 
    /// - the method is not get, getByMerchantOrderId, or one of the test methods.
    #[serde(rename="executionStatus")]
    pub execution_status: Option<String>,
}

impl Part for OrdersCustomBatchResponseEntry {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [updateshipment orders](struct.OrderUpdateshipmentCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersUpdateShipmentRequest {
    /// New status for the shipment. Not updated if missing.
    pub status: Option<String>,
    /// The carrier handling the shipment. Not updated if missing. See shipments[].carrier in the  Orders resource representation for a list of acceptable values.
    pub carrier: Option<String>,
    /// The tracking id for the shipment. Not updated if missing.
    #[serde(rename="trackingId")]
    pub tracking_id: Option<String>,
    /// The ID of the shipment.
    #[serde(rename="shipmentId")]
    pub shipment_id: Option<String>,
    /// Date on which the shipment has been delivered, in ISO 8601 format. Optional and can be provided only if status is delivered.
    #[serde(rename="deliveryDate")]
    pub delivery_date: Option<String>,
    /// The ID of the operation. Unique across all operations for a given order.
    #[serde(rename="operationId")]
    pub operation_id: Option<String>,
}

impl RequestValue for OrdersUpdateShipmentRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderLineItemReturnInfo {
    /// URL of the item return policy.
    #[serde(rename="policyUrl")]
    pub policy_url: Option<String>,
    /// Whether the item is returnable.
    #[serde(rename="isReturnable")]
    pub is_returnable: Option<bool>,
    /// How many days later the item can be returned.
    #[serde(rename="daysToReturn")]
    pub days_to_return: Option<i32>,
}

impl Part for OrderLineItemReturnInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InvoiceSummary {
    /// [required] Total price for the product.
    #[serde(rename="productTotal")]
    pub product_total: Option<Amount>,
    /// [required] Google balance on this invoice. A negative amount means Google is paying, a positive one means Google is receiving money. Note: the sum of merchant_balance, customer_balance and google_balance must always be zero.
    #[serde(rename="googleBalance")]
    pub google_balance: Option<Amount>,
    /// [required] Merchant balance on this invoice. A negative amount means the merchant is paying, a positive one means the merchant is receiving money. Note: the sum of merchant_balance, customer_balance and google_balance must always be zero.
    #[serde(rename="merchantBalance")]
    pub merchant_balance: Option<Amount>,
    /// Summary for each promotion.
    #[serde(rename="promotionSummaries")]
    pub promotion_summaries: Option<Vec<Promotion>>,
    /// [required] Customer balance on this invoice. A negative amount means the customer is paying, a positive one means the customer is receiving money. Note: the sum of merchant_balance, customer_balance and google_balance must always be zero.
    /// 
    /// Furthermore the absolute value of this amount is expected to be equal to the sum of product amount and additional charges, minus promotions.
    #[serde(rename="customerBalance")]
    pub customer_balance: Option<Amount>,
    /// Summary of the total amounts of the additional charges.
    #[serde(rename="additionalChargeSummaries")]
    pub additional_charge_summaries: Option<Vec<InvoiceSummaryAdditionalChargeSummary>>,
}

impl Part for InvoiceSummary {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *orderpayment* resources.
/// It is not used directly, but through the `ShoppingContent` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_content2_sandbox as content2_sandbox;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use content2_sandbox::ShoppingContent;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `notifyauthapproved(...)`, `notifyauthdeclined(...)`, `notifycharge(...)` and `notifyrefund(...)`
/// // to build up your call.
/// let rb = hub.orderpayments();
/// # }
/// ```
pub struct OrderpaymentMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
}

impl<'a, C, A> MethodsBuilder for OrderpaymentMethods<'a, C, A> {}

impl<'a, C, A> OrderpaymentMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Notify about failure to authorize user's payment method.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order for which payment authorization was declined.
    pub fn notifyauthdeclined(&self, request: OrderpaymentsNotifyAuthDeclinedRequest, merchant_id: &str, order_id: &str) -> OrderpaymentNotifyauthdeclinedCall<'a, C, A> {
        OrderpaymentNotifyauthdeclinedCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Notify about successfully authorizing user's payment method for a given amount.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order for for which payment authorization is happening.
    pub fn notifyauthapproved(&self, request: OrderpaymentsNotifyAuthApprovedRequest, merchant_id: &str, order_id: &str) -> OrderpaymentNotifyauthapprovedCall<'a, C, A> {
        OrderpaymentNotifyauthapprovedCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Notify about charge on user's selected payments method.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order for which charge is happening.
    pub fn notifycharge(&self, request: OrderpaymentsNotifyChargeRequest, merchant_id: &str, order_id: &str) -> OrderpaymentNotifychargeCall<'a, C, A> {
        OrderpaymentNotifychargeCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Notify about refund on user's selected payments method.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order for which charge is happening.
    pub fn notifyrefund(&self, request: OrderpaymentsNotifyRefundRequest, merchant_id: &str, order_id: &str) -> OrderpaymentNotifyrefundCall<'a, C, A> {
        OrderpaymentNotifyrefundCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *orderreturn* resources.
/// It is not used directly, but through the `ShoppingContent` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_content2_sandbox as content2_sandbox;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use content2_sandbox::ShoppingContent;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.orderreturns();
/// # }
/// ```
pub struct OrderreturnMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
}

impl<'a, C, A> MethodsBuilder for OrderreturnMethods<'a, C, A> {}

impl<'a, C, A> OrderreturnMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an order return from your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `returnId` - Merchant order return ID generated by Google.
    pub fn get(&self, merchant_id: &str, return_id: &str) -> OrderreturnGetCall<'a, C, A> {
        OrderreturnGetCall {
            hub: self.hub,
            _merchant_id: merchant_id.to_string(),
            _return_id: return_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists order returns in your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    pub fn list(&self, merchant_id: &str) -> OrderreturnListCall<'a, C, A> {
        OrderreturnListCall {
            hub: self.hub,
            _merchant_id: merchant_id.to_string(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _created_start_date: Default::default(),
            _created_end_date: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *orderinvoice* resources.
/// It is not used directly, but through the `ShoppingContent` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_content2_sandbox as content2_sandbox;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use content2_sandbox::ShoppingContent;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `createchargeinvoice(...)` and `createrefundinvoice(...)`
/// // to build up your call.
/// let rb = hub.orderinvoices();
/// # }
/// ```
pub struct OrderinvoiceMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
}

impl<'a, C, A> MethodsBuilder for OrderinvoiceMethods<'a, C, A> {}

impl<'a, C, A> OrderinvoiceMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a refund invoice for one or more shipment groups, and triggers a refund for non-facilitated payment orders. This can only be used for line items that have previously been charged using createChargeInvoice. All amounts (except for the summary) are incremental with respect to the previous invoice.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn createrefundinvoice(&self, request: OrderinvoicesCreateRefundInvoiceRequest, merchant_id: &str, order_id: &str) -> OrderinvoiceCreaterefundinvoiceCall<'a, C, A> {
        OrderinvoiceCreaterefundinvoiceCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a charge invoice for a shipment group, and triggers a charge capture for non-facilitated payment orders.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn createchargeinvoice(&self, request: OrderinvoicesCreateChargeInvoiceRequest, merchant_id: &str, order_id: &str) -> OrderinvoiceCreatechargeinvoiceCall<'a, C, A> {
        OrderinvoiceCreatechargeinvoiceCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *order* resources.
/// It is not used directly, but through the `ShoppingContent` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_content2_sandbox as content2_sandbox;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use content2_sandbox::ShoppingContent;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `acknowledge(...)`, `advancetestorder(...)`, `cancel(...)`, `cancellineitem(...)`, `canceltestorderbycustomer(...)`, `createtestorder(...)`, `createtestreturn(...)`, `custombatch(...)`, `get(...)`, `getbymerchantorderid(...)`, `gettestordertemplate(...)`, `instorerefundlineitem(...)`, `list(...)`, `refund(...)`, `rejectreturnlineitem(...)`, `returnlineitem(...)`, `returnrefundlineitem(...)`, `setlineitemmetadata(...)`, `shiplineitems(...)`, `updatelineitemshippingdetails(...)`, `updatemerchantorderid(...)` and `updateshipment(...)`
/// // to build up your call.
/// let rb = hub.orders();
/// # }
/// ```
pub struct OrderMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
}

impl<'a, C, A> MethodsBuilder for OrderMethods<'a, C, A> {}

impl<'a, C, A> OrderMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the orders in your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    pub fn list(&self, merchant_id: &str) -> OrderListCall<'a, C, A> {
        OrderListCall {
            hub: self.hub,
            _merchant_id: merchant_id.to_string(),
            _statuses: Default::default(),
            _placed_date_start: Default::default(),
            _placed_date_end: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _acknowledged: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the merchant order ID for a given order.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn updatemerchantorderid(&self, request: OrdersUpdateMerchantOrderIdRequest, merchant_id: &str, order_id: &str) -> OrderUpdatemerchantorderidCall<'a, C, A> {
        OrderUpdatemerchantorderidCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sandbox only. Retrieves an order template that can be used to quickly create a new order in sandbox.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that should manage the order. This cannot be a multi-client account.
    /// * `templateName` - The name of the template to retrieve.
    pub fn gettestordertemplate(&self, merchant_id: &str, template_name: &str) -> OrderGettestordertemplateCall<'a, C, A> {
        OrderGettestordertemplateCall {
            hub: self.hub,
            _merchant_id: merchant_id.to_string(),
            _template_name: template_name.to_string(),
            _country: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deprecated, please use returnRefundLineItem instead.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order to refund.
    pub fn refund(&self, request: OrdersRefundRequest, merchant_id: &str, order_id: &str) -> OrderRefundCall<'a, C, A> {
        OrderRefundCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Marks an order as acknowledged.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn acknowledge(&self, request: OrdersAcknowledgeRequest, merchant_id: &str, order_id: &str) -> OrderAcknowledgeCall<'a, C, A> {
        OrderAcknowledgeCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sandbox only. Moves a test order from state "inProgress" to state "pendingShipment".
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the test order to modify.
    pub fn advancetestorder(&self, merchant_id: &str, order_id: &str) -> OrderAdvancetestorderCall<'a, C, A> {
        OrderAdvancetestorderCall {
            hub: self.hub,
            _merchant_id: merchant_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Rejects return on an line item.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn rejectreturnlineitem(&self, request: OrdersRejectReturnLineItemRequest, merchant_id: &str, order_id: &str) -> OrderRejectreturnlineitemCall<'a, C, A> {
        OrderRejectreturnlineitemCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sandbox only. Creates a test order.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that should manage the order. This cannot be a multi-client account.
    pub fn createtestorder(&self, request: OrdersCreateTestOrderRequest, merchant_id: &str) -> OrderCreatetestorderCall<'a, C, A> {
        OrderCreatetestorderCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Marks line item(s) as shipped.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn shiplineitems(&self, request: OrdersShipLineItemsRequest, merchant_id: &str, order_id: &str) -> OrderShiplineitemCall<'a, C, A> {
        OrderShiplineitemCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sandbox only. Cancels a test order for customer-initiated cancellation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the test order to cancel.
    pub fn canceltestorderbycustomer(&self, request: OrdersCancelTestOrderByCustomerRequest, merchant_id: &str, order_id: &str) -> OrderCanceltestorderbycustomerCall<'a, C, A> {
        OrderCanceltestorderbycustomerCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns and refunds a line item. Note that this method can only be called on fully shipped orders.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn returnrefundlineitem(&self, request: OrdersReturnRefundLineItemRequest, merchant_id: &str, order_id: &str) -> OrderReturnrefundlineitemCall<'a, C, A> {
        OrderReturnrefundlineitemCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an order from your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn get(&self, merchant_id: &str, order_id: &str) -> OrderGetCall<'a, C, A> {
        OrderGetCall {
            hub: self.hub,
            _merchant_id: merchant_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a line item.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn returnlineitem(&self, request: OrdersReturnLineItemRequest, merchant_id: &str, order_id: &str) -> OrderReturnlineitemCall<'a, C, A> {
        OrderReturnlineitemCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates ship by and delivery by dates for a line item.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn updatelineitemshippingdetails(&self, request: OrdersUpdateLineItemShippingDetailsRequest, merchant_id: &str, order_id: &str) -> OrderUpdatelineitemshippingdetailCall<'a, C, A> {
        OrderUpdatelineitemshippingdetailCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves or modifies multiple orders in a single request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn custombatch(&self, request: OrdersCustomBatchRequest) -> OrderCustombatchCall<'a, C, A> {
        OrderCustombatchCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Notifies that item return and refund was handled directly by merchant outside of Google payments processing (e.g. cash refund done in store).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn instorerefundlineitem(&self, request: OrdersInStoreRefundLineItemRequest, merchant_id: &str, order_id: &str) -> OrderInstorerefundlineitemCall<'a, C, A> {
        OrderInstorerefundlineitemCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancels a line item, making a full refund.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn cancellineitem(&self, request: OrdersCancelLineItemRequest, merchant_id: &str, order_id: &str) -> OrderCancellineitemCall<'a, C, A> {
        OrderCancellineitemCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sandbox only. Creates a test return.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn createtestreturn(&self, request: OrdersCreateTestReturnRequest, merchant_id: &str, order_id: &str) -> OrderCreatetestreturnCall<'a, C, A> {
        OrderCreatetestreturnCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a shipment's status, carrier, and/or tracking ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn updateshipment(&self, request: OrdersUpdateShipmentRequest, merchant_id: &str, order_id: &str) -> OrderUpdateshipmentCall<'a, C, A> {
        OrderUpdateshipmentCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an order using merchant order id.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `merchantOrderId` - The merchant order id to be looked for.
    pub fn getbymerchantorderid(&self, merchant_id: &str, merchant_order_id: &str) -> OrderGetbymerchantorderidCall<'a, C, A> {
        OrderGetbymerchantorderidCall {
            hub: self.hub,
            _merchant_id: merchant_id.to_string(),
            _merchant_order_id: merchant_order_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets (overrides) merchant provided annotations on the line item.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn setlineitemmetadata(&self, request: OrdersSetLineItemMetadataRequest, merchant_id: &str, order_id: &str) -> OrderSetlineitemmetadataCall<'a, C, A> {
        OrderSetlineitemmetadataCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancels all line items in an order, making a full refund.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order to cancel.
    pub fn cancel(&self, request: OrdersCancelRequest, merchant_id: &str, order_id: &str) -> OrderCancelCall<'a, C, A> {
        OrderCancelCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Notify about failure to authorize user's payment method.
///
/// A builder for the *notifyauthdeclined* method supported by a *orderpayment* resource.
/// It is not used directly, but through a `OrderpaymentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::OrderpaymentsNotifyAuthDeclinedRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrderpaymentsNotifyAuthDeclinedRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orderpayments().notifyauthdeclined(req, "merchantId", "orderId")
///              .doit();
/// # }
/// ```
pub struct OrderpaymentNotifyauthdeclinedCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _request: OrderpaymentsNotifyAuthDeclinedRequest,
    _merchant_id: String,
    _order_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderpaymentNotifyauthdeclinedCall<'a, C, A> {}

impl<'a, C, A> OrderpaymentNotifyauthdeclinedCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrderpaymentsNotifyAuthDeclinedResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orderpayments.notifyauthdeclined",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        params.push(("orderId", self._order_id.to_string()));
        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/orderpayments/{orderId}/notifyAuthDeclined";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
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
            for param_name in ["orderId", "merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
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
    pub fn request(mut self, new_value: OrderpaymentsNotifyAuthDeclinedRequest) -> OrderpaymentNotifyauthdeclinedCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderpaymentNotifyauthdeclinedCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The ID of the order for which payment authorization was declined.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderpaymentNotifyauthdeclinedCall<'a, C, A> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderpaymentNotifyauthdeclinedCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderpaymentNotifyauthdeclinedCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderpaymentNotifyauthdeclinedCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Notify about successfully authorizing user's payment method for a given amount.
///
/// A builder for the *notifyauthapproved* method supported by a *orderpayment* resource.
/// It is not used directly, but through a `OrderpaymentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::OrderpaymentsNotifyAuthApprovedRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrderpaymentsNotifyAuthApprovedRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orderpayments().notifyauthapproved(req, "merchantId", "orderId")
///              .doit();
/// # }
/// ```
pub struct OrderpaymentNotifyauthapprovedCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _request: OrderpaymentsNotifyAuthApprovedRequest,
    _merchant_id: String,
    _order_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderpaymentNotifyauthapprovedCall<'a, C, A> {}

impl<'a, C, A> OrderpaymentNotifyauthapprovedCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrderpaymentsNotifyAuthApprovedResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orderpayments.notifyauthapproved",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        params.push(("orderId", self._order_id.to_string()));
        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/orderpayments/{orderId}/notifyAuthApproved";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
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
            for param_name in ["orderId", "merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
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
    pub fn request(mut self, new_value: OrderpaymentsNotifyAuthApprovedRequest) -> OrderpaymentNotifyauthapprovedCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderpaymentNotifyauthapprovedCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The ID of the order for for which payment authorization is happening.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderpaymentNotifyauthapprovedCall<'a, C, A> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderpaymentNotifyauthapprovedCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderpaymentNotifyauthapprovedCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderpaymentNotifyauthapprovedCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Notify about charge on user's selected payments method.
///
/// A builder for the *notifycharge* method supported by a *orderpayment* resource.
/// It is not used directly, but through a `OrderpaymentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::OrderpaymentsNotifyChargeRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrderpaymentsNotifyChargeRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orderpayments().notifycharge(req, "merchantId", "orderId")
///              .doit();
/// # }
/// ```
pub struct OrderpaymentNotifychargeCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _request: OrderpaymentsNotifyChargeRequest,
    _merchant_id: String,
    _order_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderpaymentNotifychargeCall<'a, C, A> {}

impl<'a, C, A> OrderpaymentNotifychargeCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrderpaymentsNotifyChargeResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orderpayments.notifycharge",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        params.push(("orderId", self._order_id.to_string()));
        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/orderpayments/{orderId}/notifyCharge";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
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
            for param_name in ["orderId", "merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
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
    pub fn request(mut self, new_value: OrderpaymentsNotifyChargeRequest) -> OrderpaymentNotifychargeCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderpaymentNotifychargeCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The ID of the order for which charge is happening.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderpaymentNotifychargeCall<'a, C, A> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderpaymentNotifychargeCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderpaymentNotifychargeCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderpaymentNotifychargeCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Notify about refund on user's selected payments method.
///
/// A builder for the *notifyrefund* method supported by a *orderpayment* resource.
/// It is not used directly, but through a `OrderpaymentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::OrderpaymentsNotifyRefundRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrderpaymentsNotifyRefundRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orderpayments().notifyrefund(req, "merchantId", "orderId")
///              .doit();
/// # }
/// ```
pub struct OrderpaymentNotifyrefundCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _request: OrderpaymentsNotifyRefundRequest,
    _merchant_id: String,
    _order_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderpaymentNotifyrefundCall<'a, C, A> {}

impl<'a, C, A> OrderpaymentNotifyrefundCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrderpaymentsNotifyRefundResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orderpayments.notifyrefund",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        params.push(("orderId", self._order_id.to_string()));
        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/orderpayments/{orderId}/notifyRefund";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
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
            for param_name in ["orderId", "merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
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
    pub fn request(mut self, new_value: OrderpaymentsNotifyRefundRequest) -> OrderpaymentNotifyrefundCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderpaymentNotifyrefundCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The ID of the order for which charge is happening.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderpaymentNotifyrefundCall<'a, C, A> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderpaymentNotifyrefundCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderpaymentNotifyrefundCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderpaymentNotifyrefundCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves an order return from your Merchant Center account.
///
/// A builder for the *get* method supported by a *orderreturn* resource.
/// It is not used directly, but through a `OrderreturnMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orderreturns().get("merchantId", "returnId")
///              .doit();
/// # }
/// ```
pub struct OrderreturnGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _merchant_id: String,
    _return_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderreturnGetCall<'a, C, A> {}

impl<'a, C, A> OrderreturnGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, MerchantOrderReturn)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orderreturns.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        params.push(("returnId", self._return_id.to_string()));
        for &field in ["alt", "merchantId", "returnId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/orderreturns/{returnId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{returnId}", "returnId")].iter() {
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
            for param_name in ["returnId", "merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

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


    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderreturnGetCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// Merchant order return ID generated by Google.
    ///
    /// Sets the *return id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn return_id(mut self, new_value: &str) -> OrderreturnGetCall<'a, C, A> {
        self._return_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderreturnGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderreturnGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderreturnGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists order returns in your Merchant Center account.
///
/// A builder for the *list* method supported by a *orderreturn* resource.
/// It is not used directly, but through a `OrderreturnMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orderreturns().list("merchantId")
///              .page_token("sadipscing")
///              .order_by("dolor")
///              .max_results(62)
///              .created_start_date("elitr")
///              .created_end_date("amet")
///              .doit();
/// # }
/// ```
pub struct OrderreturnListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _merchant_id: String,
    _page_token: Option<String>,
    _order_by: Option<String>,
    _max_results: Option<u32>,
    _created_start_date: Option<String>,
    _created_end_date: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderreturnListCall<'a, C, A> {}

impl<'a, C, A> OrderreturnListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrderreturnsListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orderreturns.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(8 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._order_by {
            params.push(("orderBy", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._created_start_date {
            params.push(("createdStartDate", value.to_string()));
        }
        if let Some(value) = self._created_end_date {
            params.push(("createdEndDate", value.to_string()));
        }
        for &field in ["alt", "merchantId", "pageToken", "orderBy", "maxResults", "createdStartDate", "createdEndDate"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/orderreturns";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId")].iter() {
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
            for param_name in ["merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

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


    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderreturnListCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The token returned by the previous request.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> OrderreturnListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Return the results in the specified order.
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> OrderreturnListCall<'a, C, A> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// The maximum number of order returns to return in the response, used for paging. The default value is 25 returns per page, and the maximum allowed value is 250 returns per page.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> OrderreturnListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Obtains order returns created after this date (inclusively), in ISO 8601 format.
    ///
    /// Sets the *created start date* query property to the given value.
    pub fn created_start_date(mut self, new_value: &str) -> OrderreturnListCall<'a, C, A> {
        self._created_start_date = Some(new_value.to_string());
        self
    }
    /// Obtains order returns created before this date (inclusively), in ISO 8601 format.
    ///
    /// Sets the *created end date* query property to the given value.
    pub fn created_end_date(mut self, new_value: &str) -> OrderreturnListCall<'a, C, A> {
        self._created_end_date = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderreturnListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderreturnListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderreturnListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a refund invoice for one or more shipment groups, and triggers a refund for non-facilitated payment orders. This can only be used for line items that have previously been charged using createChargeInvoice. All amounts (except for the summary) are incremental with respect to the previous invoice.
///
/// A builder for the *createrefundinvoice* method supported by a *orderinvoice* resource.
/// It is not used directly, but through a `OrderinvoiceMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::OrderinvoicesCreateRefundInvoiceRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrderinvoicesCreateRefundInvoiceRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orderinvoices().createrefundinvoice(req, "merchantId", "orderId")
///              .doit();
/// # }
/// ```
pub struct OrderinvoiceCreaterefundinvoiceCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _request: OrderinvoicesCreateRefundInvoiceRequest,
    _merchant_id: String,
    _order_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderinvoiceCreaterefundinvoiceCall<'a, C, A> {}

impl<'a, C, A> OrderinvoiceCreaterefundinvoiceCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrderinvoicesCreateRefundInvoiceResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orderinvoices.createrefundinvoice",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        params.push(("orderId", self._order_id.to_string()));
        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/orderinvoices/{orderId}/createRefundInvoice";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
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
            for param_name in ["orderId", "merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
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
    pub fn request(mut self, new_value: OrderinvoicesCreateRefundInvoiceRequest) -> OrderinvoiceCreaterefundinvoiceCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderinvoiceCreaterefundinvoiceCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderinvoiceCreaterefundinvoiceCall<'a, C, A> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderinvoiceCreaterefundinvoiceCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderinvoiceCreaterefundinvoiceCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderinvoiceCreaterefundinvoiceCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a charge invoice for a shipment group, and triggers a charge capture for non-facilitated payment orders.
///
/// A builder for the *createchargeinvoice* method supported by a *orderinvoice* resource.
/// It is not used directly, but through a `OrderinvoiceMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::OrderinvoicesCreateChargeInvoiceRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrderinvoicesCreateChargeInvoiceRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orderinvoices().createchargeinvoice(req, "merchantId", "orderId")
///              .doit();
/// # }
/// ```
pub struct OrderinvoiceCreatechargeinvoiceCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _request: OrderinvoicesCreateChargeInvoiceRequest,
    _merchant_id: String,
    _order_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderinvoiceCreatechargeinvoiceCall<'a, C, A> {}

impl<'a, C, A> OrderinvoiceCreatechargeinvoiceCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrderinvoicesCreateChargeInvoiceResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orderinvoices.createchargeinvoice",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        params.push(("orderId", self._order_id.to_string()));
        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/orderinvoices/{orderId}/createChargeInvoice";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
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
            for param_name in ["orderId", "merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
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
    pub fn request(mut self, new_value: OrderinvoicesCreateChargeInvoiceRequest) -> OrderinvoiceCreatechargeinvoiceCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderinvoiceCreatechargeinvoiceCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderinvoiceCreatechargeinvoiceCall<'a, C, A> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderinvoiceCreatechargeinvoiceCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderinvoiceCreatechargeinvoiceCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderinvoiceCreatechargeinvoiceCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists the orders in your Merchant Center account.
///
/// A builder for the *list* method supported by a *order* resource.
/// It is not used directly, but through a `OrderMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().list("merchantId")
///              .add_statuses("aliquyam")
///              .placed_date_start("accusam")
///              .placed_date_end("Lorem")
///              .page_token("sea")
///              .order_by("et")
///              .max_results(31)
///              .acknowledged(true)
///              .doit();
/// # }
/// ```
pub struct OrderListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _merchant_id: String,
    _statuses: Vec<String>,
    _placed_date_start: Option<String>,
    _placed_date_end: Option<String>,
    _page_token: Option<String>,
    _order_by: Option<String>,
    _max_results: Option<u32>,
    _acknowledged: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderListCall<'a, C, A> {}

impl<'a, C, A> OrderListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrdersListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orders.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(10 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        if self._statuses.len() > 0 {
            for f in self._statuses.iter() {
                params.push(("statuses", f.to_string()));
            }
        }
        if let Some(value) = self._placed_date_start {
            params.push(("placedDateStart", value.to_string()));
        }
        if let Some(value) = self._placed_date_end {
            params.push(("placedDateEnd", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._order_by {
            params.push(("orderBy", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._acknowledged {
            params.push(("acknowledged", value.to_string()));
        }
        for &field in ["alt", "merchantId", "statuses", "placedDateStart", "placedDateEnd", "pageToken", "orderBy", "maxResults", "acknowledged"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/orders";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId")].iter() {
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
            for param_name in ["merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

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


    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderListCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// Obtains orders that match any of the specified statuses. Multiple values can be specified with comma separation. Additionally, please note that active is a shortcut for pendingShipment and partiallyShipped, and completed is a shortcut for shipped , partiallyDelivered, delivered, partiallyReturned, returned, and canceled.
    ///
    /// Append the given value to the *statuses* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_statuses(mut self, new_value: &str) -> OrderListCall<'a, C, A> {
        self._statuses.push(new_value.to_string());
        self
    }
    /// Obtains orders placed after this date (inclusively), in ISO 8601 format.
    ///
    /// Sets the *placed date start* query property to the given value.
    pub fn placed_date_start(mut self, new_value: &str) -> OrderListCall<'a, C, A> {
        self._placed_date_start = Some(new_value.to_string());
        self
    }
    /// Obtains orders placed before this date (exclusively), in ISO 8601 format.
    ///
    /// Sets the *placed date end* query property to the given value.
    pub fn placed_date_end(mut self, new_value: &str) -> OrderListCall<'a, C, A> {
        self._placed_date_end = Some(new_value.to_string());
        self
    }
    /// The token returned by the previous request.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> OrderListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The ordering of the returned list. The only supported value are placedDate desc and placedDate asc for now, which returns orders sorted by placement date. "placedDate desc" stands for listing orders by placement date, from oldest to most recent. "placedDate asc" stands for listing orders by placement date, from most recent to oldest. In future releases we'll support other sorting criteria.
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> OrderListCall<'a, C, A> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// The maximum number of orders to return in the response, used for paging. The default value is 25 orders per page, and the maximum allowed value is 250 orders per page.
    /// Known issue: All List calls will return all Orders without limit regardless of the value of this field.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> OrderListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Obtains orders that match the acknowledgement status. When set to true, obtains orders that have been acknowledged. When false, obtains orders that have not been acknowledged.
    /// We recommend using this filter set to false, in conjunction with the acknowledge call, such that only un-acknowledged orders are returned.
    ///
    /// Sets the *acknowledged* query property to the given value.
    pub fn acknowledged(mut self, new_value: bool) -> OrderListCall<'a, C, A> {
        self._acknowledged = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates the merchant order ID for a given order.
///
/// A builder for the *updatemerchantorderid* method supported by a *order* resource.
/// It is not used directly, but through a `OrderMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::OrdersUpdateMerchantOrderIdRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersUpdateMerchantOrderIdRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().updatemerchantorderid(req, "merchantId", "orderId")
///              .doit();
/// # }
/// ```
pub struct OrderUpdatemerchantorderidCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _request: OrdersUpdateMerchantOrderIdRequest,
    _merchant_id: String,
    _order_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderUpdatemerchantorderidCall<'a, C, A> {}

impl<'a, C, A> OrderUpdatemerchantorderidCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrdersUpdateMerchantOrderIdResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orders.updatemerchantorderid",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        params.push(("orderId", self._order_id.to_string()));
        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/updateMerchantOrderId";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
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
            for param_name in ["orderId", "merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
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
    pub fn request(mut self, new_value: OrdersUpdateMerchantOrderIdRequest) -> OrderUpdatemerchantorderidCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderUpdatemerchantorderidCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderUpdatemerchantorderidCall<'a, C, A> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderUpdatemerchantorderidCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderUpdatemerchantorderidCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderUpdatemerchantorderidCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Sandbox only. Retrieves an order template that can be used to quickly create a new order in sandbox.
///
/// A builder for the *gettestordertemplate* method supported by a *order* resource.
/// It is not used directly, but through a `OrderMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().gettestordertemplate("merchantId", "templateName")
///              .country("et")
///              .doit();
/// # }
/// ```
pub struct OrderGettestordertemplateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _merchant_id: String,
    _template_name: String,
    _country: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderGettestordertemplateCall<'a, C, A> {}

impl<'a, C, A> OrderGettestordertemplateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrdersGetTestOrderTemplateResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orders.gettestordertemplate",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        params.push(("templateName", self._template_name.to_string()));
        if let Some(value) = self._country {
            params.push(("country", value.to_string()));
        }
        for &field in ["alt", "merchantId", "templateName", "country"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/testordertemplates/{templateName}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{templateName}", "templateName")].iter() {
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
            for param_name in ["templateName", "merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

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


    /// The ID of the account that should manage the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderGettestordertemplateCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The name of the template to retrieve.
    ///
    /// Sets the *template name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn template_name(mut self, new_value: &str) -> OrderGettestordertemplateCall<'a, C, A> {
        self._template_name = new_value.to_string();
        self
    }
    /// The country of the template to retrieve. Defaults to US.
    ///
    /// Sets the *country* query property to the given value.
    pub fn country(mut self, new_value: &str) -> OrderGettestordertemplateCall<'a, C, A> {
        self._country = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderGettestordertemplateCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderGettestordertemplateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderGettestordertemplateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deprecated, please use returnRefundLineItem instead.
///
/// A builder for the *refund* method supported by a *order* resource.
/// It is not used directly, but through a `OrderMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::OrdersRefundRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersRefundRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().refund(req, "merchantId", "orderId")
///              .doit();
/// # }
/// ```
pub struct OrderRefundCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _request: OrdersRefundRequest,
    _merchant_id: String,
    _order_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderRefundCall<'a, C, A> {}

impl<'a, C, A> OrderRefundCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrdersRefundResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orders.refund",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        params.push(("orderId", self._order_id.to_string()));
        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/refund";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
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
            for param_name in ["orderId", "merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
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
    pub fn request(mut self, new_value: OrdersRefundRequest) -> OrderRefundCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderRefundCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The ID of the order to refund.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderRefundCall<'a, C, A> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderRefundCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderRefundCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderRefundCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Marks an order as acknowledged.
///
/// A builder for the *acknowledge* method supported by a *order* resource.
/// It is not used directly, but through a `OrderMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::OrdersAcknowledgeRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersAcknowledgeRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().acknowledge(req, "merchantId", "orderId")
///              .doit();
/// # }
/// ```
pub struct OrderAcknowledgeCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _request: OrdersAcknowledgeRequest,
    _merchant_id: String,
    _order_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderAcknowledgeCall<'a, C, A> {}

impl<'a, C, A> OrderAcknowledgeCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrdersAcknowledgeResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orders.acknowledge",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        params.push(("orderId", self._order_id.to_string()));
        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/acknowledge";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
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
            for param_name in ["orderId", "merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
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
    pub fn request(mut self, new_value: OrdersAcknowledgeRequest) -> OrderAcknowledgeCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderAcknowledgeCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderAcknowledgeCall<'a, C, A> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderAcknowledgeCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderAcknowledgeCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderAcknowledgeCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Sandbox only. Moves a test order from state "inProgress" to state "pendingShipment".
///
/// A builder for the *advancetestorder* method supported by a *order* resource.
/// It is not used directly, but through a `OrderMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().advancetestorder("merchantId", "orderId")
///              .doit();
/// # }
/// ```
pub struct OrderAdvancetestorderCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _merchant_id: String,
    _order_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderAdvancetestorderCall<'a, C, A> {}

impl<'a, C, A> OrderAdvancetestorderCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrdersAdvanceTestOrderResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orders.advancetestorder",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        params.push(("orderId", self._order_id.to_string()));
        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/testorders/{orderId}/advance";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
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
            for param_name in ["orderId", "merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

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


    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderAdvancetestorderCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The ID of the test order to modify.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderAdvancetestorderCall<'a, C, A> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderAdvancetestorderCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderAdvancetestorderCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderAdvancetestorderCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Rejects return on an line item.
///
/// A builder for the *rejectreturnlineitem* method supported by a *order* resource.
/// It is not used directly, but through a `OrderMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::OrdersRejectReturnLineItemRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersRejectReturnLineItemRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().rejectreturnlineitem(req, "merchantId", "orderId")
///              .doit();
/// # }
/// ```
pub struct OrderRejectreturnlineitemCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _request: OrdersRejectReturnLineItemRequest,
    _merchant_id: String,
    _order_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderRejectreturnlineitemCall<'a, C, A> {}

impl<'a, C, A> OrderRejectreturnlineitemCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrdersRejectReturnLineItemResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orders.rejectreturnlineitem",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        params.push(("orderId", self._order_id.to_string()));
        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/rejectReturnLineItem";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
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
            for param_name in ["orderId", "merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
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
    pub fn request(mut self, new_value: OrdersRejectReturnLineItemRequest) -> OrderRejectreturnlineitemCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderRejectreturnlineitemCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderRejectreturnlineitemCall<'a, C, A> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderRejectreturnlineitemCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderRejectreturnlineitemCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderRejectreturnlineitemCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Sandbox only. Creates a test order.
///
/// A builder for the *createtestorder* method supported by a *order* resource.
/// It is not used directly, but through a `OrderMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::OrdersCreateTestOrderRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersCreateTestOrderRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().createtestorder(req, "merchantId")
///              .doit();
/// # }
/// ```
pub struct OrderCreatetestorderCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _request: OrdersCreateTestOrderRequest,
    _merchant_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderCreatetestorderCall<'a, C, A> {}

impl<'a, C, A> OrderCreatetestorderCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrdersCreateTestOrderResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orders.createtestorder",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        for &field in ["alt", "merchantId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/testorders";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId")].iter() {
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
            for param_name in ["merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
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
    pub fn request(mut self, new_value: OrdersCreateTestOrderRequest) -> OrderCreatetestorderCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the account that should manage the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderCreatetestorderCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderCreatetestorderCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderCreatetestorderCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderCreatetestorderCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Marks line item(s) as shipped.
///
/// A builder for the *shiplineitems* method supported by a *order* resource.
/// It is not used directly, but through a `OrderMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::OrdersShipLineItemsRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersShipLineItemsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().shiplineitems(req, "merchantId", "orderId")
///              .doit();
/// # }
/// ```
pub struct OrderShiplineitemCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _request: OrdersShipLineItemsRequest,
    _merchant_id: String,
    _order_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderShiplineitemCall<'a, C, A> {}

impl<'a, C, A> OrderShiplineitemCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrdersShipLineItemsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orders.shiplineitems",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        params.push(("orderId", self._order_id.to_string()));
        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/shipLineItems";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
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
            for param_name in ["orderId", "merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
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
    pub fn request(mut self, new_value: OrdersShipLineItemsRequest) -> OrderShiplineitemCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderShiplineitemCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderShiplineitemCall<'a, C, A> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderShiplineitemCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderShiplineitemCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderShiplineitemCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Sandbox only. Cancels a test order for customer-initiated cancellation.
///
/// A builder for the *canceltestorderbycustomer* method supported by a *order* resource.
/// It is not used directly, but through a `OrderMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::OrdersCancelTestOrderByCustomerRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersCancelTestOrderByCustomerRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().canceltestorderbycustomer(req, "merchantId", "orderId")
///              .doit();
/// # }
/// ```
pub struct OrderCanceltestorderbycustomerCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _request: OrdersCancelTestOrderByCustomerRequest,
    _merchant_id: String,
    _order_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderCanceltestorderbycustomerCall<'a, C, A> {}

impl<'a, C, A> OrderCanceltestorderbycustomerCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrdersCancelTestOrderByCustomerResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orders.canceltestorderbycustomer",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        params.push(("orderId", self._order_id.to_string()));
        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/testorders/{orderId}/cancelByCustomer";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
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
            for param_name in ["orderId", "merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
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
    pub fn request(mut self, new_value: OrdersCancelTestOrderByCustomerRequest) -> OrderCanceltestorderbycustomerCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderCanceltestorderbycustomerCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The ID of the test order to cancel.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderCanceltestorderbycustomerCall<'a, C, A> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderCanceltestorderbycustomerCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderCanceltestorderbycustomerCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderCanceltestorderbycustomerCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns and refunds a line item. Note that this method can only be called on fully shipped orders.
///
/// A builder for the *returnrefundlineitem* method supported by a *order* resource.
/// It is not used directly, but through a `OrderMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::OrdersReturnRefundLineItemRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersReturnRefundLineItemRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().returnrefundlineitem(req, "merchantId", "orderId")
///              .doit();
/// # }
/// ```
pub struct OrderReturnrefundlineitemCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _request: OrdersReturnRefundLineItemRequest,
    _merchant_id: String,
    _order_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderReturnrefundlineitemCall<'a, C, A> {}

impl<'a, C, A> OrderReturnrefundlineitemCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrdersReturnRefundLineItemResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orders.returnrefundlineitem",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        params.push(("orderId", self._order_id.to_string()));
        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/returnRefundLineItem";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
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
            for param_name in ["orderId", "merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
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
    pub fn request(mut self, new_value: OrdersReturnRefundLineItemRequest) -> OrderReturnrefundlineitemCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderReturnrefundlineitemCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderReturnrefundlineitemCall<'a, C, A> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderReturnrefundlineitemCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderReturnrefundlineitemCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderReturnrefundlineitemCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves an order from your Merchant Center account.
///
/// A builder for the *get* method supported by a *order* resource.
/// It is not used directly, but through a `OrderMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().get("merchantId", "orderId")
///              .doit();
/// # }
/// ```
pub struct OrderGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _merchant_id: String,
    _order_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderGetCall<'a, C, A> {}

impl<'a, C, A> OrderGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Order)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orders.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        params.push(("orderId", self._order_id.to_string()));
        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
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
            for param_name in ["orderId", "merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

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


    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderGetCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderGetCall<'a, C, A> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a line item.
///
/// A builder for the *returnlineitem* method supported by a *order* resource.
/// It is not used directly, but through a `OrderMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::OrdersReturnLineItemRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersReturnLineItemRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().returnlineitem(req, "merchantId", "orderId")
///              .doit();
/// # }
/// ```
pub struct OrderReturnlineitemCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _request: OrdersReturnLineItemRequest,
    _merchant_id: String,
    _order_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderReturnlineitemCall<'a, C, A> {}

impl<'a, C, A> OrderReturnlineitemCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrdersReturnLineItemResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orders.returnlineitem",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        params.push(("orderId", self._order_id.to_string()));
        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/returnLineItem";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
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
            for param_name in ["orderId", "merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
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
    pub fn request(mut self, new_value: OrdersReturnLineItemRequest) -> OrderReturnlineitemCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderReturnlineitemCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderReturnlineitemCall<'a, C, A> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderReturnlineitemCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderReturnlineitemCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderReturnlineitemCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates ship by and delivery by dates for a line item.
///
/// A builder for the *updatelineitemshippingdetails* method supported by a *order* resource.
/// It is not used directly, but through a `OrderMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::OrdersUpdateLineItemShippingDetailsRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersUpdateLineItemShippingDetailsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().updatelineitemshippingdetails(req, "merchantId", "orderId")
///              .doit();
/// # }
/// ```
pub struct OrderUpdatelineitemshippingdetailCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _request: OrdersUpdateLineItemShippingDetailsRequest,
    _merchant_id: String,
    _order_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderUpdatelineitemshippingdetailCall<'a, C, A> {}

impl<'a, C, A> OrderUpdatelineitemshippingdetailCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrdersUpdateLineItemShippingDetailsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orders.updatelineitemshippingdetails",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        params.push(("orderId", self._order_id.to_string()));
        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/updateLineItemShippingDetails";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
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
            for param_name in ["orderId", "merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
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
    pub fn request(mut self, new_value: OrdersUpdateLineItemShippingDetailsRequest) -> OrderUpdatelineitemshippingdetailCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderUpdatelineitemshippingdetailCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderUpdatelineitemshippingdetailCall<'a, C, A> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderUpdatelineitemshippingdetailCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderUpdatelineitemshippingdetailCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderUpdatelineitemshippingdetailCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves or modifies multiple orders in a single request.
///
/// A builder for the *custombatch* method supported by a *order* resource.
/// It is not used directly, but through a `OrderMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::OrdersCustomBatchRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersCustomBatchRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().custombatch(req)
///              .doit();
/// # }
/// ```
pub struct OrderCustombatchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _request: OrdersCustomBatchRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderCustombatchCall<'a, C, A> {}

impl<'a, C, A> OrderCustombatchCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrdersCustomBatchResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orders.custombatch",
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

        let mut url = self.hub._base_url.clone() + "orders/batch";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
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
    pub fn request(mut self, new_value: OrdersCustomBatchRequest) -> OrderCustombatchCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderCustombatchCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderCustombatchCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderCustombatchCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Notifies that item return and refund was handled directly by merchant outside of Google payments processing (e.g. cash refund done in store).
///
/// A builder for the *instorerefundlineitem* method supported by a *order* resource.
/// It is not used directly, but through a `OrderMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::OrdersInStoreRefundLineItemRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersInStoreRefundLineItemRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().instorerefundlineitem(req, "merchantId", "orderId")
///              .doit();
/// # }
/// ```
pub struct OrderInstorerefundlineitemCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _request: OrdersInStoreRefundLineItemRequest,
    _merchant_id: String,
    _order_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderInstorerefundlineitemCall<'a, C, A> {}

impl<'a, C, A> OrderInstorerefundlineitemCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrdersInStoreRefundLineItemResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orders.instorerefundlineitem",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        params.push(("orderId", self._order_id.to_string()));
        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/inStoreRefundLineItem";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
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
            for param_name in ["orderId", "merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
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
    pub fn request(mut self, new_value: OrdersInStoreRefundLineItemRequest) -> OrderInstorerefundlineitemCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderInstorerefundlineitemCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderInstorerefundlineitemCall<'a, C, A> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderInstorerefundlineitemCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderInstorerefundlineitemCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderInstorerefundlineitemCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Cancels a line item, making a full refund.
///
/// A builder for the *cancellineitem* method supported by a *order* resource.
/// It is not used directly, but through a `OrderMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::OrdersCancelLineItemRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersCancelLineItemRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().cancellineitem(req, "merchantId", "orderId")
///              .doit();
/// # }
/// ```
pub struct OrderCancellineitemCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _request: OrdersCancelLineItemRequest,
    _merchant_id: String,
    _order_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderCancellineitemCall<'a, C, A> {}

impl<'a, C, A> OrderCancellineitemCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrdersCancelLineItemResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orders.cancellineitem",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        params.push(("orderId", self._order_id.to_string()));
        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/cancelLineItem";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
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
            for param_name in ["orderId", "merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
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
    pub fn request(mut self, new_value: OrdersCancelLineItemRequest) -> OrderCancellineitemCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderCancellineitemCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderCancellineitemCall<'a, C, A> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderCancellineitemCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderCancellineitemCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderCancellineitemCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Sandbox only. Creates a test return.
///
/// A builder for the *createtestreturn* method supported by a *order* resource.
/// It is not used directly, but through a `OrderMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::OrdersCreateTestReturnRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersCreateTestReturnRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().createtestreturn(req, "merchantId", "orderId")
///              .doit();
/// # }
/// ```
pub struct OrderCreatetestreturnCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _request: OrdersCreateTestReturnRequest,
    _merchant_id: String,
    _order_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderCreatetestreturnCall<'a, C, A> {}

impl<'a, C, A> OrderCreatetestreturnCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrdersCreateTestReturnResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orders.createtestreturn",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        params.push(("orderId", self._order_id.to_string()));
        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/testreturn";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
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
            for param_name in ["orderId", "merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
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
    pub fn request(mut self, new_value: OrdersCreateTestReturnRequest) -> OrderCreatetestreturnCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderCreatetestreturnCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderCreatetestreturnCall<'a, C, A> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderCreatetestreturnCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderCreatetestreturnCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderCreatetestreturnCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates a shipment's status, carrier, and/or tracking ID.
///
/// A builder for the *updateshipment* method supported by a *order* resource.
/// It is not used directly, but through a `OrderMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::OrdersUpdateShipmentRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersUpdateShipmentRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().updateshipment(req, "merchantId", "orderId")
///              .doit();
/// # }
/// ```
pub struct OrderUpdateshipmentCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _request: OrdersUpdateShipmentRequest,
    _merchant_id: String,
    _order_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderUpdateshipmentCall<'a, C, A> {}

impl<'a, C, A> OrderUpdateshipmentCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrdersUpdateShipmentResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orders.updateshipment",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        params.push(("orderId", self._order_id.to_string()));
        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/updateShipment";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
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
            for param_name in ["orderId", "merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
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
    pub fn request(mut self, new_value: OrdersUpdateShipmentRequest) -> OrderUpdateshipmentCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderUpdateshipmentCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderUpdateshipmentCall<'a, C, A> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderUpdateshipmentCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderUpdateshipmentCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderUpdateshipmentCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves an order using merchant order id.
///
/// A builder for the *getbymerchantorderid* method supported by a *order* resource.
/// It is not used directly, but through a `OrderMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().getbymerchantorderid("merchantId", "merchantOrderId")
///              .doit();
/// # }
/// ```
pub struct OrderGetbymerchantorderidCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _merchant_id: String,
    _merchant_order_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderGetbymerchantorderidCall<'a, C, A> {}

impl<'a, C, A> OrderGetbymerchantorderidCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrdersGetByMerchantOrderIdResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orders.getbymerchantorderid",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        params.push(("merchantOrderId", self._merchant_order_id.to_string()));
        for &field in ["alt", "merchantId", "merchantOrderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/ordersbymerchantid/{merchantOrderId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{merchantOrderId}", "merchantOrderId")].iter() {
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
            for param_name in ["merchantOrderId", "merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

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


    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderGetbymerchantorderidCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The merchant order id to be looked for.
    ///
    /// Sets the *merchant order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_order_id(mut self, new_value: &str) -> OrderGetbymerchantorderidCall<'a, C, A> {
        self._merchant_order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderGetbymerchantorderidCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderGetbymerchantorderidCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderGetbymerchantorderidCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Sets (overrides) merchant provided annotations on the line item.
///
/// A builder for the *setlineitemmetadata* method supported by a *order* resource.
/// It is not used directly, but through a `OrderMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::OrdersSetLineItemMetadataRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersSetLineItemMetadataRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().setlineitemmetadata(req, "merchantId", "orderId")
///              .doit();
/// # }
/// ```
pub struct OrderSetlineitemmetadataCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _request: OrdersSetLineItemMetadataRequest,
    _merchant_id: String,
    _order_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderSetlineitemmetadataCall<'a, C, A> {}

impl<'a, C, A> OrderSetlineitemmetadataCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrdersSetLineItemMetadataResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orders.setlineitemmetadata",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        params.push(("orderId", self._order_id.to_string()));
        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/setLineItemMetadata";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
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
            for param_name in ["orderId", "merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
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
    pub fn request(mut self, new_value: OrdersSetLineItemMetadataRequest) -> OrderSetlineitemmetadataCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderSetlineitemmetadataCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderSetlineitemmetadataCall<'a, C, A> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderSetlineitemmetadataCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderSetlineitemmetadataCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderSetlineitemmetadataCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Cancels all line items in an order, making a full refund.
///
/// A builder for the *cancel* method supported by a *order* resource.
/// It is not used directly, but through a `OrderMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::OrdersCancelRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use content2_sandbox::ShoppingContent;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ShoppingContent::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersCancelRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().cancel(req, "merchantId", "orderId")
///              .doit();
/// # }
/// ```
pub struct OrderCancelCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ShoppingContent<C, A>,
    _request: OrdersCancelRequest,
    _merchant_id: String,
    _order_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrderCancelCall<'a, C, A> {}

impl<'a, C, A> OrderCancelCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, OrdersCancelResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "content.orders.cancel",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("merchantId", self._merchant_id.to_string()));
        params.push(("orderId", self._order_id.to_string()));
        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/cancel";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
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
            for param_name in ["orderId", "merchantId"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
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
    pub fn request(mut self, new_value: OrdersCancelRequest) -> OrderCancelCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: &str) -> OrderCancelCall<'a, C, A> {
        self._merchant_id = new_value.to_string();
        self
    }
    /// The ID of the order to cancel.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderCancelCall<'a, C, A> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OrderCancelCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> OrderCancelCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> OrderCancelCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


