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
/// extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().list(97)
///              .add_statuses("ea")
///              .placed_date_start("ipsum")
///              .placed_date_end("invidunt")
///              .page_token("amet")
///              .order_by("duo")
///              .max_results(51)
///              .acknowledged(false)
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
pub struct ShoppingContent<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, S> client::Hub for ShoppingContent<S> {}

impl<'a, S> ShoppingContent<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> ShoppingContent<S> {
        ShoppingContent {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.2-beta-1".to_string(),
            _base_url: "https://www.googleapis.com/content/v2sandbox/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn orderinvoices(&'a self) -> OrderinvoiceMethods<'a, S> {
        OrderinvoiceMethods { hub: &self }
    }
    pub fn orderpayments(&'a self) -> OrderpaymentMethods<'a, S> {
        OrderpaymentMethods { hub: &self }
    }
    pub fn orderreturns(&'a self) -> OrderreturnMethods<'a, S> {
        OrderreturnMethods { hub: &self }
    }
    pub fn orders(&'a self) -> OrderMethods<'a, S> {
        OrderMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.2-beta-1`.
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
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Amount {
    /// [required] Value before taxes.
    
    pub pretax: Option<Price>,
    /// [required] Tax value.
    
    pub tax: Option<Price>,
}

impl client::Part for Amount {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomerReturnReason {
    /// no description provided
    
    pub description: Option<String>,
    /// no description provided
    #[serde(rename="reasonCode")]
    
    pub reason_code: Option<String>,
}

impl client::Part for CustomerReturnReason {}


/// An error returned by the API.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Error {
    /// The domain of the error.
    
    pub domain: Option<String>,
    /// A description of the error.
    
    pub message: Option<String>,
    /// The error code.
    
    pub reason: Option<String>,
}

impl client::Part for Error {}


/// A list of errors returned by a failed batch entry.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Errors {
    /// The HTTP status of the first error in errors.
    
    pub code: Option<u32>,
    /// A list of errors.
    
    pub errors: Option<Vec<Error>>,
    /// The message of the first error in errors.
    
    pub message: Option<String>,
}

impl client::Part for Errors {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InvoiceSummary {
    /// Summary of the total amounts of the additional charges.
    #[serde(rename="additionalChargeSummaries")]
    
    pub additional_charge_summaries: Option<Vec<InvoiceSummaryAdditionalChargeSummary>>,
    /// [required] Customer balance on this invoice. A negative amount means the customer is paying, a positive one means the customer is receiving money. Note: the sum of merchant_balance, customer_balance and google_balance must always be zero.
    /// 
    /// Furthermore the absolute value of this amount is expected to be equal to the sum of product amount and additional charges, minus promotions.
    #[serde(rename="customerBalance")]
    
    pub customer_balance: Option<Amount>,
    /// [required] Google balance on this invoice. A negative amount means Google is paying, a positive one means Google is receiving money. Note: the sum of merchant_balance, customer_balance and google_balance must always be zero.
    #[serde(rename="googleBalance")]
    
    pub google_balance: Option<Amount>,
    /// [required] Merchant balance on this invoice. A negative amount means the merchant is paying, a positive one means the merchant is receiving money. Note: the sum of merchant_balance, customer_balance and google_balance must always be zero.
    #[serde(rename="merchantBalance")]
    
    pub merchant_balance: Option<Amount>,
    /// [required] Total price for the product.
    #[serde(rename="productTotal")]
    
    pub product_total: Option<Amount>,
    /// Summary for each promotion.
    #[serde(rename="promotionSummaries")]
    
    pub promotion_summaries: Option<Vec<Promotion>>,
}

impl client::Part for InvoiceSummary {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InvoiceSummaryAdditionalChargeSummary {
    /// [required] Total additional charge for this type.
    #[serde(rename="totalAmount")]
    
    pub total_amount: Option<Amount>,
    /// [required] Type of the additional charge.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for InvoiceSummaryAdditionalChargeSummary {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get orderreturns](OrderreturnGetCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MerchantOrderReturn {
    /// no description provided
    #[serde(rename="creationDate")]
    
    pub creation_date: Option<String>,
    /// no description provided
    #[serde(rename="merchantOrderId")]
    
    pub merchant_order_id: Option<String>,
    /// no description provided
    #[serde(rename="orderId")]
    
    pub order_id: Option<String>,
    /// no description provided
    #[serde(rename="orderReturnId")]
    
    pub order_return_id: Option<String>,
    /// no description provided
    #[serde(rename="returnItems")]
    
    pub return_items: Option<Vec<MerchantOrderReturnItem>>,
    /// no description provided
    #[serde(rename="returnShipments")]
    
    pub return_shipments: Option<Vec<ReturnShipment>>,
}

impl client::ResponseResult for MerchantOrderReturn {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MerchantOrderReturnItem {
    /// no description provided
    #[serde(rename="customerReturnReason")]
    
    pub customer_return_reason: Option<CustomerReturnReason>,
    /// no description provided
    #[serde(rename="itemId")]
    
    pub item_id: Option<String>,
    /// no description provided
    #[serde(rename="merchantReturnReason")]
    
    pub merchant_return_reason: Option<RefundReason>,
    /// no description provided
    
    pub product: Option<OrderLineItemProduct>,
    /// no description provided
    #[serde(rename="returnShipmentIds")]
    
    pub return_shipment_ids: Option<Vec<String>>,
    /// no description provided
    
    pub state: Option<String>,
}

impl client::Part for MerchantOrderReturnItem {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [acknowledge orders](OrderAcknowledgeCall) (none)
/// * [advancetestorder orders](OrderAdvancetestorderCall) (none)
/// * [cancel orders](OrderCancelCall) (none)
/// * [cancellineitem orders](OrderCancellineitemCall) (none)
/// * [canceltestorderbycustomer orders](OrderCanceltestorderbycustomerCall) (none)
/// * [createtestorder orders](OrderCreatetestorderCall) (none)
/// * [createtestreturn orders](OrderCreatetestreturnCall) (none)
/// * [custombatch orders](OrderCustombatchCall) (none)
/// * [get orders](OrderGetCall) (response)
/// * [getbymerchantorderid orders](OrderGetbymerchantorderidCall) (none)
/// * [gettestordertemplate orders](OrderGettestordertemplateCall) (none)
/// * [instorerefundlineitem orders](OrderInstorerefundlineitemCall) (none)
/// * [list orders](OrderListCall) (none)
/// * [refund orders](OrderRefundCall) (none)
/// * [rejectreturnlineitem orders](OrderRejectreturnlineitemCall) (none)
/// * [returnlineitem orders](OrderReturnlineitemCall) (none)
/// * [returnrefundlineitem orders](OrderReturnrefundlineitemCall) (none)
/// * [setlineitemmetadata orders](OrderSetlineitemmetadataCall) (none)
/// * [shiplineitems orders](OrderShiplineitemCall) (none)
/// * [updatelineitemshippingdetails orders](OrderUpdatelineitemshippingdetailCall) (none)
/// * [updatemerchantorderid orders](OrderUpdatemerchantorderidCall) (none)
/// * [updateshipment orders](OrderUpdateshipmentCall) (none)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Order {
    /// Whether the order was acknowledged.
    
    pub acknowledged: Option<bool>,
    /// The channel type of the order: "purchaseOnGoogle" or "googleExpress".
    #[serde(rename="channelType")]
    
    pub channel_type: Option<String>,
    /// The details of the customer who placed the order.
    
    pub customer: Option<OrderCustomer>,
    /// The details for the delivery.
    #[serde(rename="deliveryDetails")]
    
    pub delivery_details: Option<OrderDeliveryDetails>,
    /// The REST id of the order. Globally unique.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#order".
    
    pub kind: Option<String>,
    /// Line items that are ordered.
    #[serde(rename="lineItems")]
    
    pub line_items: Option<Vec<OrderLineItem>>,
    /// no description provided
    #[serde(rename="merchantId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub merchant_id: Option<u64>,
    /// Merchant-provided id of the order.
    #[serde(rename="merchantOrderId")]
    
    pub merchant_order_id: Option<String>,
    /// The net amount for the order. For example, if an order was originally for a grand total of $100 and a refund was issued for $20, the net amount will be $80.
    #[serde(rename="netAmount")]
    
    pub net_amount: Option<Price>,
    /// The details of the payment method.
    #[serde(rename="paymentMethod")]
    
    pub payment_method: Option<OrderPaymentMethod>,
    /// The status of the payment.
    #[serde(rename="paymentStatus")]
    
    pub payment_status: Option<String>,
    /// The date when the order was placed, in ISO 8601 format.
    #[serde(rename="placedDate")]
    
    pub placed_date: Option<String>,
    /// Deprecated. The details of the merchant provided promotions applied to the order. More details about the program are here.
    
    pub promotions: Option<Vec<OrderLegacyPromotion>>,
    /// Refunds for the order.
    
    pub refunds: Option<Vec<OrderRefund>>,
    /// Shipments of the order.
    
    pub shipments: Option<Vec<OrderShipment>>,
    /// The total cost of shipping for all items.
    #[serde(rename="shippingCost")]
    
    pub shipping_cost: Option<Price>,
    /// The tax for the total shipping cost.
    #[serde(rename="shippingCostTax")]
    
    pub shipping_cost_tax: Option<Price>,
    /// The requested shipping option.
    #[serde(rename="shippingOption")]
    
    pub shipping_option: Option<String>,
    /// The status of the order.
    
    pub status: Option<String>,
}

impl client::Resource for Order {}
impl client::ResponseResult for Order {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderAddress {
    /// CLDR country code (e.g. "US").
    
    pub country: Option<String>,
    /// Strings representing the lines of the printed label for mailing the order, for example:
    /// John Smith
    /// 1600 Amphitheatre Parkway
    /// Mountain View, CA, 94043
    /// United States
    #[serde(rename="fullAddress")]
    
    pub full_address: Option<Vec<String>>,
    /// Whether the address is a post office box.
    #[serde(rename="isPostOfficeBox")]
    
    pub is_post_office_box: Option<bool>,
    /// City, town or commune. May also include dependent localities or sublocalities (e.g. neighborhoods or suburbs).
    
    pub locality: Option<String>,
    /// Postal Code or ZIP (e.g. "94043").
    #[serde(rename="postalCode")]
    
    pub postal_code: Option<String>,
    /// Name of the recipient.
    #[serde(rename="recipientName")]
    
    pub recipient_name: Option<String>,
    /// Top-level administrative subdivision of the country. For example, a state like California ("CA") or a province like Quebec ("QC").
    
    pub region: Option<String>,
    /// Street-level part of the address.
    #[serde(rename="streetAddress")]
    
    pub street_address: Option<Vec<String>>,
}

impl client::Part for OrderAddress {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderCancellation {
    /// The actor that created the cancellation.
    
    pub actor: Option<String>,
    /// Date on which the cancellation has been created, in ISO 8601 format.
    #[serde(rename="creationDate")]
    
    pub creation_date: Option<String>,
    /// The quantity that was canceled.
    
    pub quantity: Option<u32>,
    /// The reason for the cancellation. Orders that are cancelled with a noInventory reason will lead to the removal of the product from Shopping Actions until you make an update to that product. This will not affect your Shopping ads.
    
    pub reason: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    
    pub reason_text: Option<String>,
}

impl client::Part for OrderCancellation {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderCustomer {
    /// Deprecated.
    
    pub email: Option<String>,
    /// Deprecated. Please use marketingRightsInfo instead.
    #[serde(rename="explicitMarketingPreference")]
    
    pub explicit_marketing_preference: Option<bool>,
    /// Full name of the customer.
    #[serde(rename="fullName")]
    
    pub full_name: Option<String>,
    /// Customer's marketing preferences.
    #[serde(rename="marketingRightsInfo")]
    
    pub marketing_rights_info: Option<OrderCustomerMarketingRightsInfo>,
}

impl client::Part for OrderCustomer {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
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

impl client::Part for OrderCustomerMarketingRightsInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderDeliveryDetails {
    /// The delivery address
    
    pub address: Option<OrderAddress>,
    /// The phone number of the person receiving the delivery.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
}

impl client::Part for OrderDeliveryDetails {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderLegacyPromotion {
    /// no description provided
    
    pub benefits: Option<Vec<OrderLegacyPromotionBenefit>>,
    /// The date and time frame when the promotion is active and ready for validation review. Note that the promotion live time may be delayed for a few hours due to the validation review.
    /// Start date and end date are separated by a forward slash (/). The start date is specified by the format (YYYY-MM-DD), followed by the letter ?T?, the time of the day when the sale starts (in Greenwich Mean Time, GMT), followed by an expression of the time zone for the sale. The end date is in the same format.
    #[serde(rename="effectiveDates")]
    
    pub effective_dates: Option<String>,
    /// Optional. The text code that corresponds to the promotion when applied on the retailer?s website.
    #[serde(rename="genericRedemptionCode")]
    
    pub generic_redemption_code: Option<String>,
    /// The unique ID of the promotion.
    
    pub id: Option<String>,
    /// The full title of the promotion.
    #[serde(rename="longTitle")]
    
    pub long_title: Option<String>,
    /// Whether the promotion is applicable to all products or only specific products.
    #[serde(rename="productApplicability")]
    
    pub product_applicability: Option<String>,
    /// Indicates that the promotion is valid online.
    #[serde(rename="redemptionChannel")]
    
    pub redemption_channel: Option<String>,
}

impl client::Part for OrderLegacyPromotion {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderLegacyPromotionBenefit {
    /// The discount in the order price when the promotion is applied.
    
    pub discount: Option<Price>,
    /// The OfferId(s) that were purchased in this order and map to this specific benefit of the promotion.
    #[serde(rename="offerIds")]
    
    pub offer_ids: Option<Vec<String>>,
    /// Further describes the benefit of the promotion. Note that we will expand on this enumeration as we support new promotion sub-types.
    #[serde(rename="subType")]
    
    pub sub_type: Option<String>,
    /// The impact on tax when the promotion is applied.
    #[serde(rename="taxImpact")]
    
    pub tax_impact: Option<Price>,
    /// Describes whether the promotion applies to products (e.g. 20% off) or to shipping (e.g. Free Shipping).
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for OrderLegacyPromotionBenefit {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderLineItem {
    /// Annotations that are attached to the line item.
    
    pub annotations: Option<Vec<OrderMerchantProvidedAnnotation>>,
    /// Cancellations of the line item.
    
    pub cancellations: Option<Vec<OrderCancellation>>,
    /// The id of the line item.
    
    pub id: Option<String>,
    /// Total price for the line item. For example, if two items for $10 are purchased, the total price will be $20.
    
    pub price: Option<Price>,
    /// Product data from the time of the order placement.
    
    pub product: Option<OrderLineItemProduct>,
    /// Number of items canceled.
    #[serde(rename="quantityCanceled")]
    
    pub quantity_canceled: Option<u32>,
    /// Number of items delivered.
    #[serde(rename="quantityDelivered")]
    
    pub quantity_delivered: Option<u32>,
    /// Number of items ordered.
    #[serde(rename="quantityOrdered")]
    
    pub quantity_ordered: Option<u32>,
    /// Number of items pending.
    #[serde(rename="quantityPending")]
    
    pub quantity_pending: Option<u32>,
    /// Number of items returned.
    #[serde(rename="quantityReturned")]
    
    pub quantity_returned: Option<u32>,
    /// Number of items shipped.
    #[serde(rename="quantityShipped")]
    
    pub quantity_shipped: Option<u32>,
    /// Details of the return policy for the line item.
    #[serde(rename="returnInfo")]
    
    pub return_info: Option<OrderLineItemReturnInfo>,
    /// Returns of the line item.
    
    pub returns: Option<Vec<OrderReturn>>,
    /// Details of the requested shipping for the line item.
    #[serde(rename="shippingDetails")]
    
    pub shipping_details: Option<OrderLineItemShippingDetails>,
    /// Total tax amount for the line item. For example, if two items are purchased, and each have a cost tax of $2, the total tax amount will be $4.
    
    pub tax: Option<Price>,
}

impl client::Part for OrderLineItem {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderLineItemProduct {
    /// Brand of the item.
    
    pub brand: Option<String>,
    /// The item's channel (online or local).
    
    pub channel: Option<String>,
    /// Condition or state of the item.
    
    pub condition: Option<String>,
    /// The two-letter ISO 639-1 language code for the item.
    #[serde(rename="contentLanguage")]
    
    pub content_language: Option<String>,
    /// Global Trade Item Number (GTIN) of the item.
    
    pub gtin: Option<String>,
    /// The REST id of the product.
    
    pub id: Option<String>,
    /// URL of an image of the item.
    #[serde(rename="imageLink")]
    
    pub image_link: Option<String>,
    /// Shared identifier for all variants of the same product.
    #[serde(rename="itemGroupId")]
    
    pub item_group_id: Option<String>,
    /// Manufacturer Part Number (MPN) of the item.
    
    pub mpn: Option<String>,
    /// An identifier of the item.
    #[serde(rename="offerId")]
    
    pub offer_id: Option<String>,
    /// Price of the item.
    
    pub price: Option<Price>,
    /// URL to the cached image shown to the user when order was placed.
    #[serde(rename="shownImage")]
    
    pub shown_image: Option<String>,
    /// The CLDR territory code of the target country of the product.
    #[serde(rename="targetCountry")]
    
    pub target_country: Option<String>,
    /// The title of the product.
    
    pub title: Option<String>,
    /// Variant attributes for the item. These are dimensions of the product, such as color, gender, material, pattern, and size. You can find a comprehensive list of variant attributes here.
    #[serde(rename="variantAttributes")]
    
    pub variant_attributes: Option<Vec<OrderLineItemProductVariantAttribute>>,
}

impl client::Part for OrderLineItemProduct {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderLineItemProductVariantAttribute {
    /// The dimension of the variant.
    
    pub dimension: Option<String>,
    /// The value for the dimension.
    
    pub value: Option<String>,
}

impl client::Part for OrderLineItemProductVariantAttribute {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderLineItemReturnInfo {
    /// How many days later the item can be returned.
    #[serde(rename="daysToReturn")]
    
    pub days_to_return: Option<i32>,
    /// Whether the item is returnable.
    #[serde(rename="isReturnable")]
    
    pub is_returnable: Option<bool>,
    /// URL of the item return policy.
    #[serde(rename="policyUrl")]
    
    pub policy_url: Option<String>,
}

impl client::Part for OrderLineItemReturnInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
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

impl client::Part for OrderLineItemShippingDetails {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderLineItemShippingDetailsMethod {
    /// The carrier for the shipping. Optional. See shipments[].carrier for a list of acceptable values.
    
    pub carrier: Option<String>,
    /// Maximum transit time.
    #[serde(rename="maxDaysInTransit")]
    
    pub max_days_in_transit: Option<u32>,
    /// The name of the shipping method.
    #[serde(rename="methodName")]
    
    pub method_name: Option<String>,
    /// Minimum transit time.
    #[serde(rename="minDaysInTransit")]
    
    pub min_days_in_transit: Option<u32>,
}

impl client::Part for OrderLineItemShippingDetailsMethod {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderMerchantProvidedAnnotation {
    /// Key for additional merchant provided (as key-value pairs) annotation about the line item.
    
    pub key: Option<String>,
    /// Value for additional merchant provided (as key-value pairs) annotation about the line item.
    
    pub value: Option<String>,
}

impl client::Part for OrderMerchantProvidedAnnotation {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderPaymentMethod {
    /// The billing address.
    #[serde(rename="billingAddress")]
    
    pub billing_address: Option<OrderAddress>,
    /// The card expiration month (January = 1, February = 2 etc.).
    #[serde(rename="expirationMonth")]
    
    pub expiration_month: Option<i32>,
    /// The card expiration year (4-digit, e.g. 2015).
    #[serde(rename="expirationYear")]
    
    pub expiration_year: Option<i32>,
    /// The last four digits of the card number.
    #[serde(rename="lastFourDigits")]
    
    pub last_four_digits: Option<String>,
    /// The billing phone number.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
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
}

impl client::Part for OrderPaymentMethod {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderRefund {
    /// The actor that created the refund.
    
    pub actor: Option<String>,
    /// The amount that is refunded.
    
    pub amount: Option<Price>,
    /// Date on which the item has been created, in ISO 8601 format.
    #[serde(rename="creationDate")]
    
    pub creation_date: Option<String>,
    /// The reason for the refund.
    
    pub reason: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    
    pub reason_text: Option<String>,
}

impl client::Part for OrderRefund {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderReturn {
    /// The actor that created the refund.
    
    pub actor: Option<String>,
    /// Date on which the item has been created, in ISO 8601 format.
    #[serde(rename="creationDate")]
    
    pub creation_date: Option<String>,
    /// Quantity that is returned.
    
    pub quantity: Option<u32>,
    /// The reason for the return.
    
    pub reason: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    
    pub reason_text: Option<String>,
}

impl client::Part for OrderReturn {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderShipment {
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
    /// Date on which the shipment has been created, in ISO 8601 format.
    #[serde(rename="creationDate")]
    
    pub creation_date: Option<String>,
    /// Date on which the shipment has been delivered, in ISO 8601 format. Present only if status is delivered
    #[serde(rename="deliveryDate")]
    
    pub delivery_date: Option<String>,
    /// The id of the shipment.
    
    pub id: Option<String>,
    /// The line items that are shipped.
    #[serde(rename="lineItems")]
    
    pub line_items: Option<Vec<OrderShipmentLineItemShipment>>,
    /// The status of the shipment.
    
    pub status: Option<String>,
    /// The tracking id for the shipment.
    #[serde(rename="trackingId")]
    
    pub tracking_id: Option<String>,
}

impl client::Part for OrderShipment {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderShipmentLineItemShipment {
    /// The id of the line item that is shipped. Either lineItemId or productId is required.
    #[serde(rename="lineItemId")]
    
    pub line_item_id: Option<String>,
    /// The ID of the product to ship. This is the REST ID used in the products service. Either lineItemId or productId is required.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// The quantity that is shipped.
    
    pub quantity: Option<u32>,
}

impl client::Part for OrderShipmentLineItemShipment {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [createchargeinvoice orderinvoices](OrderinvoiceCreatechargeinvoiceCall) (request)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderinvoicesCreateChargeInvoiceRequest {
    /// [required] The ID of the invoice.
    #[serde(rename="invoiceId")]
    
    pub invoice_id: Option<String>,
    /// [required] Invoice summary.
    #[serde(rename="invoiceSummary")]
    
    pub invoice_summary: Option<InvoiceSummary>,
    /// [required] Invoice details per line item.
    #[serde(rename="lineItemInvoices")]
    
    pub line_item_invoices: Option<Vec<ShipmentInvoiceLineItemInvoice>>,
    /// [required] The ID of the operation, unique across all operations for a given order.
    #[serde(rename="operationId")]
    
    pub operation_id: Option<String>,
    /// [required] ID of the shipment group.
    #[serde(rename="shipmentGroupId")]
    
    pub shipment_group_id: Option<String>,
}

impl client::RequestValue for OrderinvoicesCreateChargeInvoiceRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [createchargeinvoice orderinvoices](OrderinvoiceCreatechargeinvoiceCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderinvoicesCreateChargeInvoiceResponse {
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    
    pub execution_status: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#orderinvoicesCreateChargeInvoiceResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for OrderinvoicesCreateChargeInvoiceResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [createrefundinvoice orderinvoices](OrderinvoiceCreaterefundinvoiceCall) (request)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderinvoicesCreateRefundInvoiceRequest {
    /// [required] The ID of the invoice.
    #[serde(rename="invoiceId")]
    
    pub invoice_id: Option<String>,
    /// [required] The ID of the operation, unique across all operations for a given order.
    #[serde(rename="operationId")]
    
    pub operation_id: Option<String>,
    /// Option to create a refund-only invoice. Exactly one of refundOnlyOption or returnOption must be provided.
    #[serde(rename="refundOnlyOption")]
    
    pub refund_only_option: Option<OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceRefundOption>,
    /// Option to create an invoice for a refund and mark all items within the invoice as returned. Exactly one of refundOnlyOption or returnOption must be provided.
    #[serde(rename="returnOption")]
    
    pub return_option: Option<OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceReturnOption>,
    /// Invoice details for different shipment groups.
    #[serde(rename="shipmentInvoices")]
    
    pub shipment_invoices: Option<Vec<ShipmentInvoice>>,
}

impl client::RequestValue for OrderinvoicesCreateRefundInvoiceRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [createrefundinvoice orderinvoices](OrderinvoiceCreaterefundinvoiceCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderinvoicesCreateRefundInvoiceResponse {
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    
    pub execution_status: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#orderinvoicesCreateRefundInvoiceResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for OrderinvoicesCreateRefundInvoiceResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceRefundOption {
    /// Optional description of the refund reason.
    
    pub description: Option<String>,
    /// [required] Reason for the refund.
    
    pub reason: Option<String>,
}

impl client::Part for OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceRefundOption {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceReturnOption {
    /// Optional description of the return reason.
    
    pub description: Option<String>,
    /// [required] Reason for the return.
    
    pub reason: Option<String>,
}

impl client::Part for OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceReturnOption {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notifyauthapproved orderpayments](OrderpaymentNotifyauthapprovedCall) (request)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderpaymentsNotifyAuthApprovedRequest {
    /// no description provided
    #[serde(rename="authAmountPretax")]
    
    pub auth_amount_pretax: Option<Price>,
    /// no description provided
    #[serde(rename="authAmountTax")]
    
    pub auth_amount_tax: Option<Price>,
}

impl client::RequestValue for OrderpaymentsNotifyAuthApprovedRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notifyauthapproved orderpayments](OrderpaymentNotifyauthapprovedCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderpaymentsNotifyAuthApprovedResponse {
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    
    pub execution_status: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#orderpaymentsNotifyAuthApprovedResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for OrderpaymentsNotifyAuthApprovedResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notifyauthdeclined orderpayments](OrderpaymentNotifyauthdeclinedCall) (request)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderpaymentsNotifyAuthDeclinedRequest {
    /// Reason why payment authorization was declined.
    #[serde(rename="declineReason")]
    
    pub decline_reason: Option<String>,
}

impl client::RequestValue for OrderpaymentsNotifyAuthDeclinedRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notifyauthdeclined orderpayments](OrderpaymentNotifyauthdeclinedCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderpaymentsNotifyAuthDeclinedResponse {
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    
    pub execution_status: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#orderpaymentsNotifyAuthDeclinedResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for OrderpaymentsNotifyAuthDeclinedResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notifycharge orderpayments](OrderpaymentNotifychargeCall) (request)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
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

impl client::RequestValue for OrderpaymentsNotifyChargeRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notifycharge orderpayments](OrderpaymentNotifychargeCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderpaymentsNotifyChargeResponse {
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    
    pub execution_status: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#orderpaymentsNotifyChargeResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for OrderpaymentsNotifyChargeResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notifyrefund orderpayments](OrderpaymentNotifyrefundCall) (request)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderpaymentsNotifyRefundRequest {
    /// Deprecated. Please use invoiceIds instead.
    #[serde(rename="invoiceId")]
    
    pub invoice_id: Option<String>,
    /// Invoice IDs from the orderinvoices service that correspond to the refund.
    #[serde(rename="invoiceIds")]
    
    pub invoice_ids: Option<Vec<String>>,
    /// Whether refund was successful.
    #[serde(rename="refundState")]
    
    pub refund_state: Option<String>,
}

impl client::RequestValue for OrderpaymentsNotifyRefundRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notifyrefund orderpayments](OrderpaymentNotifyrefundCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderpaymentsNotifyRefundResponse {
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    
    pub execution_status: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#orderpaymentsNotifyRefundResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for OrderpaymentsNotifyRefundResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list orderreturns](OrderreturnListCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderreturnsListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#orderreturnsListResponse".
    
    pub kind: Option<String>,
    /// The token for the retrieval of the next page of returns.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// no description provided
    
    pub resources: Option<Vec<MerchantOrderReturn>>,
}

impl client::ResponseResult for OrderreturnsListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [acknowledge orders](OrderAcknowledgeCall) (request)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersAcknowledgeRequest {
    /// The ID of the operation. Unique across all operations for a given order.
    #[serde(rename="operationId")]
    
    pub operation_id: Option<String>,
}

impl client::RequestValue for OrdersAcknowledgeRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [acknowledge orders](OrderAcknowledgeCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersAcknowledgeResponse {
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    
    pub execution_status: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersAcknowledgeResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for OrdersAcknowledgeResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [advancetestorder orders](OrderAdvancetestorderCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersAdvanceTestOrderResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersAdvanceTestOrderResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for OrdersAdvanceTestOrderResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [cancellineitem orders](OrderCancellineitemCall) (request)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCancelLineItemRequest {
    /// Deprecated. Please use amountPretax and amountTax instead.
    
    pub amount: Option<Price>,
    /// Amount to refund for the cancelation. Optional. If not set, Google will calculate the default based on the price and tax of the items involved. The amount must not be larger than the net amount left on the order.
    #[serde(rename="amountPretax")]
    
    pub amount_pretax: Option<Price>,
    /// Tax amount that correspond to cancellation amount in amountPretax.
    #[serde(rename="amountTax")]
    
    pub amount_tax: Option<Price>,
    /// The ID of the line item to cancel. Either lineItemId or productId is required.
    #[serde(rename="lineItemId")]
    
    pub line_item_id: Option<String>,
    /// The ID of the operation. Unique across all operations for a given order.
    #[serde(rename="operationId")]
    
    pub operation_id: Option<String>,
    /// The ID of the product to cancel. This is the REST ID used in the products service. Either lineItemId or productId is required.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// The quantity to cancel.
    
    pub quantity: Option<u32>,
    /// The reason for the cancellation.
    
    pub reason: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    
    pub reason_text: Option<String>,
}

impl client::RequestValue for OrdersCancelLineItemRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [cancellineitem orders](OrderCancellineitemCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCancelLineItemResponse {
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    
    pub execution_status: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersCancelLineItemResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for OrdersCancelLineItemResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [cancel orders](OrderCancelCall) (request)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCancelRequest {
    /// The ID of the operation. Unique across all operations for a given order.
    #[serde(rename="operationId")]
    
    pub operation_id: Option<String>,
    /// The reason for the cancellation.
    
    pub reason: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    
    pub reason_text: Option<String>,
}

impl client::RequestValue for OrdersCancelRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [cancel orders](OrderCancelCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCancelResponse {
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    
    pub execution_status: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersCancelResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for OrdersCancelResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [canceltestorderbycustomer orders](OrderCanceltestorderbycustomerCall) (request)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCancelTestOrderByCustomerRequest {
    /// The reason for the cancellation.
    
    pub reason: Option<String>,
}

impl client::RequestValue for OrdersCancelTestOrderByCustomerRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [canceltestorderbycustomer orders](OrderCanceltestorderbycustomerCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCancelTestOrderByCustomerResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersCancelTestOrderByCustomerResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for OrdersCancelTestOrderByCustomerResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [createtestorder orders](OrderCreatetestorderCall) (request)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCreateTestOrderRequest {
    /// The  CLDR territory code of the country of the test order to create. Affects the currency and addresses of orders created via template_name, or the addresses of orders created via test_order.
    /// 
    /// Acceptable values are:  
    /// - "US" 
    /// - "FR"  Defaults to US.
    
    pub country: Option<String>,
    /// The test order template to use. Specify as an alternative to testOrder as a shortcut for retrieving a template and then creating an order using that template.
    #[serde(rename="templateName")]
    
    pub template_name: Option<String>,
    /// The test order to create.
    #[serde(rename="testOrder")]
    
    pub test_order: Option<TestOrder>,
}

impl client::RequestValue for OrdersCreateTestOrderRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [createtestorder orders](OrderCreatetestorderCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCreateTestOrderResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersCreateTestOrderResponse".
    
    pub kind: Option<String>,
    /// The ID of the newly created test order.
    #[serde(rename="orderId")]
    
    pub order_id: Option<String>,
}

impl client::ResponseResult for OrdersCreateTestOrderResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [createtestreturn orders](OrderCreatetestreturnCall) (request)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCreateTestReturnRequest {
    /// Returned items.
    
    pub items: Option<Vec<OrdersCustomBatchRequestEntryCreateTestReturnReturnItem>>,
}

impl client::RequestValue for OrdersCreateTestReturnRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [createtestreturn orders](OrderCreatetestreturnCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCreateTestReturnResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersCreateTestReturnResponse".
    
    pub kind: Option<String>,
    /// The ID of the newly created test order return.
    #[serde(rename="returnId")]
    
    pub return_id: Option<String>,
}

impl client::ResponseResult for OrdersCreateTestReturnResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch orders](OrderCustombatchCall) (request)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequest {
    /// The request entries to be processed in the batch.
    
    pub entries: Option<Vec<OrdersCustomBatchRequestEntry>>,
}

impl client::RequestValue for OrdersCustomBatchRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntry {
    /// An entry ID, unique within the batch request.
    #[serde(rename="batchId")]
    
    pub batch_id: Option<u32>,
    /// Required for cancel method.
    
    pub cancel: Option<OrdersCustomBatchRequestEntryCancel>,
    /// Required for cancelLineItem method.
    #[serde(rename="cancelLineItem")]
    
    pub cancel_line_item: Option<OrdersCustomBatchRequestEntryCancelLineItem>,
    /// Required for inStoreReturnLineItem method.
    #[serde(rename="inStoreRefundLineItem")]
    
    pub in_store_refund_line_item: Option<OrdersCustomBatchRequestEntryInStoreRefundLineItem>,
    /// The ID of the managing account.
    #[serde(rename="merchantId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub merchant_id: Option<u64>,
    /// The merchant order id. Required for updateMerchantOrderId and getByMerchantOrderId methods.
    #[serde(rename="merchantOrderId")]
    
    pub merchant_order_id: Option<String>,
    /// The method to apply.
    
    pub method: Option<String>,
    /// The ID of the operation. Unique across all operations for a given order. Required for all methods beside get and getByMerchantOrderId.
    #[serde(rename="operationId")]
    
    pub operation_id: Option<String>,
    /// The ID of the order. Required for all methods beside getByMerchantOrderId.
    #[serde(rename="orderId")]
    
    pub order_id: Option<String>,
    /// Required for refund method.
    
    pub refund: Option<OrdersCustomBatchRequestEntryRefund>,
    /// Required for rejectReturnLineItem method.
    #[serde(rename="rejectReturnLineItem")]
    
    pub reject_return_line_item: Option<OrdersCustomBatchRequestEntryRejectReturnLineItem>,
    /// Required for returnLineItem method.
    #[serde(rename="returnLineItem")]
    
    pub return_line_item: Option<OrdersCustomBatchRequestEntryReturnLineItem>,
    /// Required for returnRefundLineItem method.
    #[serde(rename="returnRefundLineItem")]
    
    pub return_refund_line_item: Option<OrdersCustomBatchRequestEntryReturnRefundLineItem>,
    /// Required for setLineItemMetadata method.
    #[serde(rename="setLineItemMetadata")]
    
    pub set_line_item_metadata: Option<OrdersCustomBatchRequestEntrySetLineItemMetadata>,
    /// Required for shipLineItems method.
    #[serde(rename="shipLineItems")]
    
    pub ship_line_items: Option<OrdersCustomBatchRequestEntryShipLineItems>,
    /// Required for updateLineItemShippingDate method.
    #[serde(rename="updateLineItemShippingDetails")]
    
    pub update_line_item_shipping_details: Option<OrdersCustomBatchRequestEntryUpdateLineItemShippingDetails>,
    /// Required for updateShipment method.
    #[serde(rename="updateShipment")]
    
    pub update_shipment: Option<OrdersCustomBatchRequestEntryUpdateShipment>,
}

impl client::Part for OrdersCustomBatchRequestEntry {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntryCancel {
    /// The reason for the cancellation.
    
    pub reason: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    
    pub reason_text: Option<String>,
}

impl client::Part for OrdersCustomBatchRequestEntryCancel {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntryCancelLineItem {
    /// Deprecated. Please use amountPretax and amountTax instead.
    
    pub amount: Option<Price>,
    /// Amount to refund for the cancelation. Optional. If not set, Google will calculate the default based on the price and tax of the items involved. The amount must not be larger than the net amount left on the order.
    #[serde(rename="amountPretax")]
    
    pub amount_pretax: Option<Price>,
    /// Tax amount that correspond to cancellation amount in amountPretax.
    #[serde(rename="amountTax")]
    
    pub amount_tax: Option<Price>,
    /// The ID of the line item to cancel. Either lineItemId or productId is required.
    #[serde(rename="lineItemId")]
    
    pub line_item_id: Option<String>,
    /// The ID of the product to cancel. This is the REST ID used in the products service. Either lineItemId or productId is required.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// The quantity to cancel.
    
    pub quantity: Option<u32>,
    /// The reason for the cancellation.
    
    pub reason: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    
    pub reason_text: Option<String>,
}

impl client::Part for OrdersCustomBatchRequestEntryCancelLineItem {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntryCreateTestReturnReturnItem {
    /// The ID of the line item to return.
    #[serde(rename="lineItemId")]
    
    pub line_item_id: Option<String>,
    /// Quantity that is returned.
    
    pub quantity: Option<u32>,
}

impl client::Part for OrdersCustomBatchRequestEntryCreateTestReturnReturnItem {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntryInStoreRefundLineItem {
    /// The amount that is refunded. Required.
    #[serde(rename="amountPretax")]
    
    pub amount_pretax: Option<Price>,
    /// Tax amount that correspond to refund amount in amountPretax. Required.
    #[serde(rename="amountTax")]
    
    pub amount_tax: Option<Price>,
    /// The ID of the line item to return. Either lineItemId or productId is required.
    #[serde(rename="lineItemId")]
    
    pub line_item_id: Option<String>,
    /// The ID of the product to return. This is the REST ID used in the products service. Either lineItemId or productId is required.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// The quantity to return and refund.
    
    pub quantity: Option<u32>,
    /// The reason for the return.
    
    pub reason: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    
    pub reason_text: Option<String>,
}

impl client::Part for OrdersCustomBatchRequestEntryInStoreRefundLineItem {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntryRefund {
    /// Deprecated. Please use amountPretax and amountTax instead.
    
    pub amount: Option<Price>,
    /// The amount that is refunded. Either amount or amountPretax and amountTax should be filled.
    #[serde(rename="amountPretax")]
    
    pub amount_pretax: Option<Price>,
    /// Tax amount that correspond to refund amount in amountPretax.
    #[serde(rename="amountTax")]
    
    pub amount_tax: Option<Price>,
    /// The reason for the refund.
    
    pub reason: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    
    pub reason_text: Option<String>,
}

impl client::Part for OrdersCustomBatchRequestEntryRefund {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntryRejectReturnLineItem {
    /// The ID of the line item to return. Either lineItemId or productId is required.
    #[serde(rename="lineItemId")]
    
    pub line_item_id: Option<String>,
    /// The ID of the product to return. This is the REST ID used in the products service. Either lineItemId or productId is required.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// The quantity to return and refund.
    
    pub quantity: Option<u32>,
    /// The reason for the return.
    
    pub reason: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    
    pub reason_text: Option<String>,
}

impl client::Part for OrdersCustomBatchRequestEntryRejectReturnLineItem {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntryReturnLineItem {
    /// The ID of the line item to return. Either lineItemId or productId is required.
    #[serde(rename="lineItemId")]
    
    pub line_item_id: Option<String>,
    /// The ID of the product to return. This is the REST ID used in the products service. Either lineItemId or productId is required.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// The quantity to return.
    
    pub quantity: Option<u32>,
    /// The reason for the return.
    
    pub reason: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    
    pub reason_text: Option<String>,
}

impl client::Part for OrdersCustomBatchRequestEntryReturnLineItem {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntryReturnRefundLineItem {
    /// The amount that is refunded. If omitted, refundless return is assumed (same as calling returnLineItem method). Optional, but if filled then both amountPretax and amountTax must be set.
    #[serde(rename="amountPretax")]
    
    pub amount_pretax: Option<Price>,
    /// Tax amount that correspond to refund amount in amountPretax.
    #[serde(rename="amountTax")]
    
    pub amount_tax: Option<Price>,
    /// The ID of the line item to return. Either lineItemId or productId is required.
    #[serde(rename="lineItemId")]
    
    pub line_item_id: Option<String>,
    /// The ID of the product to return. This is the REST ID used in the products service. Either lineItemId or productId is required.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// The quantity to return and refund.
    
    pub quantity: Option<u32>,
    /// The reason for the return.
    
    pub reason: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    
    pub reason_text: Option<String>,
}

impl client::Part for OrdersCustomBatchRequestEntryReturnRefundLineItem {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
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

impl client::Part for OrdersCustomBatchRequestEntrySetLineItemMetadata {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntryShipLineItems {
    /// Deprecated. Please use shipmentInfo instead. The carrier handling the shipment. See shipments[].carrier in the  Orders resource representation for a list of acceptable values.
    
    pub carrier: Option<String>,
    /// Line items to ship.
    #[serde(rename="lineItems")]
    
    pub line_items: Option<Vec<OrderShipmentLineItemShipment>>,
    /// ID of the shipment group. Required for orders that use the orderinvoices service.
    #[serde(rename="shipmentGroupId")]
    
    pub shipment_group_id: Option<String>,
    /// Deprecated. Please use shipmentInfo instead. The ID of the shipment.
    #[serde(rename="shipmentId")]
    
    pub shipment_id: Option<String>,
    /// Shipment information. This field is repeated because a single line item can be shipped in several packages (and have several tracking IDs).
    #[serde(rename="shipmentInfos")]
    
    pub shipment_infos: Option<Vec<OrdersCustomBatchRequestEntryShipLineItemsShipmentInfo>>,
    /// Deprecated. Please use shipmentInfo instead. The tracking id for the shipment.
    #[serde(rename="trackingId")]
    
    pub tracking_id: Option<String>,
}

impl client::Part for OrdersCustomBatchRequestEntryShipLineItems {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntryShipLineItemsShipmentInfo {
    /// The carrier handling the shipment. See shipments[].carrier in the  Orders resource representation for a list of acceptable values.
    
    pub carrier: Option<String>,
    /// The ID of the shipment.
    #[serde(rename="shipmentId")]
    
    pub shipment_id: Option<String>,
    /// The tracking id for the shipment.
    #[serde(rename="trackingId")]
    
    pub tracking_id: Option<String>,
}

impl client::Part for OrdersCustomBatchRequestEntryShipLineItemsShipmentInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntryUpdateLineItemShippingDetails {
    /// Updated delivery by date, in ISO 8601 format. If not specified only ship by date is updated.
    #[serde(rename="deliverByDate")]
    
    pub deliver_by_date: Option<String>,
    /// The ID of the line item to set metadata. Either lineItemId or productId is required.
    #[serde(rename="lineItemId")]
    
    pub line_item_id: Option<String>,
    /// The ID of the product to set metadata. This is the REST ID used in the products service. Either lineItemId or productId is required.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// Updated ship by date, in ISO 8601 format. If not specified only deliver by date is updated.
    #[serde(rename="shipByDate")]
    
    pub ship_by_date: Option<String>,
}

impl client::Part for OrdersCustomBatchRequestEntryUpdateLineItemShippingDetails {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchRequestEntryUpdateShipment {
    /// The carrier handling the shipment. Not updated if missing. See shipments[].carrier in the  Orders resource representation for a list of acceptable values.
    
    pub carrier: Option<String>,
    /// Date on which the shipment has been delivered, in ISO 8601 format. Optional and can be provided only if status is delivered.
    #[serde(rename="deliveryDate")]
    
    pub delivery_date: Option<String>,
    /// The ID of the shipment.
    #[serde(rename="shipmentId")]
    
    pub shipment_id: Option<String>,
    /// New status for the shipment. Not updated if missing.
    
    pub status: Option<String>,
    /// The tracking id for the shipment. Not updated if missing.
    #[serde(rename="trackingId")]
    
    pub tracking_id: Option<String>,
}

impl client::Part for OrdersCustomBatchRequestEntryUpdateShipment {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch orders](OrderCustombatchCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchResponse {
    /// The result of the execution of the batch requests.
    
    pub entries: Option<Vec<OrdersCustomBatchResponseEntry>>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersCustomBatchResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for OrdersCustomBatchResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCustomBatchResponseEntry {
    /// The ID of the request entry this entry responds to.
    #[serde(rename="batchId")]
    
    pub batch_id: Option<u32>,
    /// A list of errors defined if and only if the request failed.
    
    pub errors: Option<Errors>,
    /// The status of the execution. Only defined if  
    /// - the request was successful; and 
    /// - the method is not get, getByMerchantOrderId, or one of the test methods.
    #[serde(rename="executionStatus")]
    
    pub execution_status: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersCustomBatchResponseEntry".
    
    pub kind: Option<String>,
    /// The retrieved order. Only defined if the method is get and if the request was successful.
    
    pub order: Option<Order>,
}

impl client::Part for OrdersCustomBatchResponseEntry {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [getbymerchantorderid orders](OrderGetbymerchantorderidCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersGetByMerchantOrderIdResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersGetByMerchantOrderIdResponse".
    
    pub kind: Option<String>,
    /// The requested order.
    
    pub order: Option<Order>,
}

impl client::ResponseResult for OrdersGetByMerchantOrderIdResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [gettestordertemplate orders](OrderGettestordertemplateCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersGetTestOrderTemplateResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersGetTestOrderTemplateResponse".
    
    pub kind: Option<String>,
    /// The requested test order template.
    
    pub template: Option<TestOrder>,
}

impl client::ResponseResult for OrdersGetTestOrderTemplateResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instorerefundlineitem orders](OrderInstorerefundlineitemCall) (request)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersInStoreRefundLineItemRequest {
    /// The amount that is refunded. Required.
    #[serde(rename="amountPretax")]
    
    pub amount_pretax: Option<Price>,
    /// Tax amount that correspond to refund amount in amountPretax. Required.
    #[serde(rename="amountTax")]
    
    pub amount_tax: Option<Price>,
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
    /// The reason for the return.
    
    pub reason: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    
    pub reason_text: Option<String>,
}

impl client::RequestValue for OrdersInStoreRefundLineItemRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instorerefundlineitem orders](OrderInstorerefundlineitemCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersInStoreRefundLineItemResponse {
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    
    pub execution_status: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersInStoreRefundLineItemResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for OrdersInStoreRefundLineItemResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list orders](OrderListCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersListResponse".
    
    pub kind: Option<String>,
    /// The token for the retrieval of the next page of orders.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// no description provided
    
    pub resources: Option<Vec<Order>>,
}

impl client::ResponseResult for OrdersListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [refund orders](OrderRefundCall) (request)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersRefundRequest {
    /// Deprecated. Please use amountPretax and amountTax instead.
    
    pub amount: Option<Price>,
    /// The amount that is refunded. Either amount or amountPretax and amountTax should be filled.
    #[serde(rename="amountPretax")]
    
    pub amount_pretax: Option<Price>,
    /// Tax amount that correspond to refund amount in amountPretax.
    #[serde(rename="amountTax")]
    
    pub amount_tax: Option<Price>,
    /// The ID of the operation. Unique across all operations for a given order.
    #[serde(rename="operationId")]
    
    pub operation_id: Option<String>,
    /// The reason for the refund.
    
    pub reason: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    
    pub reason_text: Option<String>,
}

impl client::RequestValue for OrdersRefundRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [refund orders](OrderRefundCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersRefundResponse {
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    
    pub execution_status: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersRefundResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for OrdersRefundResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [rejectreturnlineitem orders](OrderRejectreturnlineitemCall) (request)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersRejectReturnLineItemRequest {
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
    /// The reason for the return.
    
    pub reason: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    
    pub reason_text: Option<String>,
}

impl client::RequestValue for OrdersRejectReturnLineItemRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [rejectreturnlineitem orders](OrderRejectreturnlineitemCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersRejectReturnLineItemResponse {
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    
    pub execution_status: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersRejectReturnLineItemResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for OrdersRejectReturnLineItemResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [returnlineitem orders](OrderReturnlineitemCall) (request)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersReturnLineItemRequest {
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
    /// The reason for the return.
    
    pub reason: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    
    pub reason_text: Option<String>,
}

impl client::RequestValue for OrdersReturnLineItemRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [returnlineitem orders](OrderReturnlineitemCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersReturnLineItemResponse {
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    
    pub execution_status: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersReturnLineItemResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for OrdersReturnLineItemResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [returnrefundlineitem orders](OrderReturnrefundlineitemCall) (request)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersReturnRefundLineItemRequest {
    /// The amount that is refunded. If omitted, refundless return is assumed (same as calling returnLineItem method). Optional, but if filled then both amountPretax and amountTax must be set.
    #[serde(rename="amountPretax")]
    
    pub amount_pretax: Option<Price>,
    /// Tax amount that correspond to refund amount in amountPretax.
    #[serde(rename="amountTax")]
    
    pub amount_tax: Option<Price>,
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
    /// The reason for the return.
    
    pub reason: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    
    pub reason_text: Option<String>,
}

impl client::RequestValue for OrdersReturnRefundLineItemRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [returnrefundlineitem orders](OrderReturnrefundlineitemCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersReturnRefundLineItemResponse {
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    
    pub execution_status: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersReturnRefundLineItemResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for OrdersReturnRefundLineItemResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [setlineitemmetadata orders](OrderSetlineitemmetadataCall) (request)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersSetLineItemMetadataRequest {
    /// no description provided
    
    pub annotations: Option<Vec<OrderMerchantProvidedAnnotation>>,
    /// The ID of the line item to set metadata. Either lineItemId or productId is required.
    #[serde(rename="lineItemId")]
    
    pub line_item_id: Option<String>,
    /// The ID of the operation. Unique across all operations for a given order.
    #[serde(rename="operationId")]
    
    pub operation_id: Option<String>,
    /// The ID of the product to set metadata. This is the REST ID used in the products service. Either lineItemId or productId is required.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
}

impl client::RequestValue for OrdersSetLineItemMetadataRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [setlineitemmetadata orders](OrderSetlineitemmetadataCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersSetLineItemMetadataResponse {
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    
    pub execution_status: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersSetLineItemMetadataResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for OrdersSetLineItemMetadataResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [shiplineitems orders](OrderShiplineitemCall) (request)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersShipLineItemsRequest {
    /// Deprecated. Please use shipmentInfo instead. The carrier handling the shipment. See shipments[].carrier in the  Orders resource representation for a list of acceptable values.
    
    pub carrier: Option<String>,
    /// Line items to ship.
    #[serde(rename="lineItems")]
    
    pub line_items: Option<Vec<OrderShipmentLineItemShipment>>,
    /// The ID of the operation. Unique across all operations for a given order.
    #[serde(rename="operationId")]
    
    pub operation_id: Option<String>,
    /// ID of the shipment group. Required for orders that use the orderinvoices service.
    #[serde(rename="shipmentGroupId")]
    
    pub shipment_group_id: Option<String>,
    /// Deprecated. Please use shipmentInfo instead. The ID of the shipment.
    #[serde(rename="shipmentId")]
    
    pub shipment_id: Option<String>,
    /// Shipment information. This field is repeated because a single line item can be shipped in several packages (and have several tracking IDs).
    #[serde(rename="shipmentInfos")]
    
    pub shipment_infos: Option<Vec<OrdersCustomBatchRequestEntryShipLineItemsShipmentInfo>>,
    /// Deprecated. Please use shipmentInfo instead. The tracking id for the shipment.
    #[serde(rename="trackingId")]
    
    pub tracking_id: Option<String>,
}

impl client::RequestValue for OrdersShipLineItemsRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [shiplineitems orders](OrderShiplineitemCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersShipLineItemsResponse {
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    
    pub execution_status: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersShipLineItemsResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for OrdersShipLineItemsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [updatelineitemshippingdetails orders](OrderUpdatelineitemshippingdetailCall) (request)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersUpdateLineItemShippingDetailsRequest {
    /// Updated delivery by date, in ISO 8601 format. If not specified only ship by date is updated.
    #[serde(rename="deliverByDate")]
    
    pub deliver_by_date: Option<String>,
    /// The ID of the line item to set metadata. Either lineItemId or productId is required.
    #[serde(rename="lineItemId")]
    
    pub line_item_id: Option<String>,
    /// The ID of the operation. Unique across all operations for a given order.
    #[serde(rename="operationId")]
    
    pub operation_id: Option<String>,
    /// The ID of the product to set metadata. This is the REST ID used in the products service. Either lineItemId or productId is required.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// Updated ship by date, in ISO 8601 format. If not specified only deliver by date is updated.
    #[serde(rename="shipByDate")]
    
    pub ship_by_date: Option<String>,
}

impl client::RequestValue for OrdersUpdateLineItemShippingDetailsRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [updatelineitemshippingdetails orders](OrderUpdatelineitemshippingdetailCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersUpdateLineItemShippingDetailsResponse {
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    
    pub execution_status: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersUpdateLineItemShippingDetailsResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for OrdersUpdateLineItemShippingDetailsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [updatemerchantorderid orders](OrderUpdatemerchantorderidCall) (request)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersUpdateMerchantOrderIdRequest {
    /// The merchant order id to be assigned to the order. Must be unique per merchant.
    #[serde(rename="merchantOrderId")]
    
    pub merchant_order_id: Option<String>,
    /// The ID of the operation. Unique across all operations for a given order.
    #[serde(rename="operationId")]
    
    pub operation_id: Option<String>,
}

impl client::RequestValue for OrdersUpdateMerchantOrderIdRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [updatemerchantorderid orders](OrderUpdatemerchantorderidCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersUpdateMerchantOrderIdResponse {
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    
    pub execution_status: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersUpdateMerchantOrderIdResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for OrdersUpdateMerchantOrderIdResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [updateshipment orders](OrderUpdateshipmentCall) (request)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersUpdateShipmentRequest {
    /// The carrier handling the shipment. Not updated if missing. See shipments[].carrier in the  Orders resource representation for a list of acceptable values.
    
    pub carrier: Option<String>,
    /// Date on which the shipment has been delivered, in ISO 8601 format. Optional and can be provided only if status is delivered.
    #[serde(rename="deliveryDate")]
    
    pub delivery_date: Option<String>,
    /// The ID of the operation. Unique across all operations for a given order.
    #[serde(rename="operationId")]
    
    pub operation_id: Option<String>,
    /// The ID of the shipment.
    #[serde(rename="shipmentId")]
    
    pub shipment_id: Option<String>,
    /// New status for the shipment. Not updated if missing.
    
    pub status: Option<String>,
    /// The tracking id for the shipment. Not updated if missing.
    #[serde(rename="trackingId")]
    
    pub tracking_id: Option<String>,
}

impl client::RequestValue for OrdersUpdateShipmentRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [updateshipment orders](OrderUpdateshipmentCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrdersUpdateShipmentResponse {
    /// The status of the execution.
    #[serde(rename="executionStatus")]
    
    pub execution_status: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#ordersUpdateShipmentResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for OrdersUpdateShipmentResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Price {
    /// The currency of the price.
    
    pub currency: Option<String>,
    /// The price represented as a number.
    
    pub value: Option<String>,
}

impl client::Part for Price {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Promotion {
    /// [required] Amount of the promotion. The values here are the promotion applied to the unit price pretax and to the total of the tax amounts.
    #[serde(rename="promotionAmount")]
    
    pub promotion_amount: Option<Amount>,
    /// [required] ID of the promotion.
    #[serde(rename="promotionId")]
    
    pub promotion_id: Option<String>,
}

impl client::Part for Promotion {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RefundReason {
    /// no description provided
    
    pub description: Option<String>,
    /// no description provided
    #[serde(rename="reasonCode")]
    
    pub reason_code: Option<String>,
}

impl client::Part for RefundReason {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReturnShipment {
    /// no description provided
    #[serde(rename="creationDate")]
    
    pub creation_date: Option<String>,
    /// no description provided
    #[serde(rename="returnMethodType")]
    
    pub return_method_type: Option<String>,
    /// no description provided
    #[serde(rename="shipmentId")]
    
    pub shipment_id: Option<String>,
    /// no description provided
    #[serde(rename="shipmentTrackingInfos")]
    
    pub shipment_tracking_infos: Option<Vec<ShipmentTrackingInfo>>,
}

impl client::Part for ReturnShipment {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
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

impl client::Part for ShipmentInvoice {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShipmentInvoiceLineItemInvoice {
    /// ID of the line item. Either lineItemId or productId must be set.
    #[serde(rename="lineItemId")]
    
    pub line_item_id: Option<String>,
    /// ID of the product. This is the REST ID used in the products service. Either lineItemId or productId must be set.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// [required] Unit IDs to define specific units within the line item.
    #[serde(rename="shipmentUnitIds")]
    
    pub shipment_unit_ids: Option<Vec<String>>,
    /// [required] Invoice details for a single unit.
    #[serde(rename="unitInvoice")]
    
    pub unit_invoice: Option<UnitInvoice>,
}

impl client::Part for ShipmentInvoiceLineItemInvoice {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShipmentTrackingInfo {
    /// no description provided
    
    pub carrier: Option<String>,
    /// no description provided
    #[serde(rename="trackingNumber")]
    
    pub tracking_number: Option<String>,
}

impl client::Part for ShipmentTrackingInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestOrder {
    /// The details of the customer who placed the order.
    
    pub customer: Option<TestOrderCustomer>,
    /// Whether the orderinvoices service should support this order.
    #[serde(rename="enableOrderinvoices")]
    
    pub enable_orderinvoices: Option<bool>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#testOrder".
    
    pub kind: Option<String>,
    /// Line items that are ordered. At least one line item must be provided.
    #[serde(rename="lineItems")]
    
    pub line_items: Option<Vec<TestOrderLineItem>>,
    /// Determines if test order must be pulled by merchant or pushed to merchant via push integration.
    #[serde(rename="notificationMode")]
    
    pub notification_mode: Option<String>,
    /// The details of the payment method.
    #[serde(rename="paymentMethod")]
    
    pub payment_method: Option<TestOrderPaymentMethod>,
    /// Identifier of one of the predefined delivery addresses for the delivery.
    #[serde(rename="predefinedDeliveryAddress")]
    
    pub predefined_delivery_address: Option<String>,
    /// Deprecated. The details of the merchant provided promotions applied to the order. More details about the program are here.
    
    pub promotions: Option<Vec<OrderLegacyPromotion>>,
    /// The total cost of shipping for all items.
    #[serde(rename="shippingCost")]
    
    pub shipping_cost: Option<Price>,
    /// The tax for the total shipping cost.
    #[serde(rename="shippingCostTax")]
    
    pub shipping_cost_tax: Option<Price>,
    /// The requested shipping option.
    #[serde(rename="shippingOption")]
    
    pub shipping_option: Option<String>,
}

impl client::Part for TestOrder {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestOrderCustomer {
    /// Deprecated.
    
    pub email: Option<String>,
    /// Deprecated. Please use marketingRightsInfo instead.
    #[serde(rename="explicitMarketingPreference")]
    
    pub explicit_marketing_preference: Option<bool>,
    /// Full name of the customer.
    #[serde(rename="fullName")]
    
    pub full_name: Option<String>,
    /// Customer's marketing preferences.
    #[serde(rename="marketingRightsInfo")]
    
    pub marketing_rights_info: Option<TestOrderCustomerMarketingRightsInfo>,
}

impl client::Part for TestOrderCustomer {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestOrderCustomerMarketingRightsInfo {
    /// Last know user use selection regards marketing preferences. In certain cases selection might not be known, so this field would be empty.
    #[serde(rename="explicitMarketingPreference")]
    
    pub explicit_marketing_preference: Option<String>,
    /// Timestamp when last time marketing preference was updated. Could be empty, if user wasn't offered a selection yet.
    #[serde(rename="lastUpdatedTimestamp")]
    
    pub last_updated_timestamp: Option<String>,
}

impl client::Part for TestOrderCustomerMarketingRightsInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestOrderLineItem {
    /// Product data from the time of the order placement.
    
    pub product: Option<TestOrderLineItemProduct>,
    /// Number of items ordered.
    #[serde(rename="quantityOrdered")]
    
    pub quantity_ordered: Option<u32>,
    /// Details of the return policy for the line item.
    #[serde(rename="returnInfo")]
    
    pub return_info: Option<OrderLineItemReturnInfo>,
    /// Details of the requested shipping for the line item.
    #[serde(rename="shippingDetails")]
    
    pub shipping_details: Option<OrderLineItemShippingDetails>,
    /// Unit tax for the line item.
    #[serde(rename="unitTax")]
    
    pub unit_tax: Option<Price>,
}

impl client::Part for TestOrderLineItem {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestOrderLineItemProduct {
    /// Brand of the item.
    
    pub brand: Option<String>,
    /// The item's channel.
    
    pub channel: Option<String>,
    /// Condition or state of the item.
    
    pub condition: Option<String>,
    /// The two-letter ISO 639-1 language code for the item.
    #[serde(rename="contentLanguage")]
    
    pub content_language: Option<String>,
    /// Global Trade Item Number (GTIN) of the item. Optional.
    
    pub gtin: Option<String>,
    /// URL of an image of the item.
    #[serde(rename="imageLink")]
    
    pub image_link: Option<String>,
    /// Shared identifier for all variants of the same product. Optional.
    #[serde(rename="itemGroupId")]
    
    pub item_group_id: Option<String>,
    /// Manufacturer Part Number (MPN) of the item. Optional.
    
    pub mpn: Option<String>,
    /// An identifier of the item.
    #[serde(rename="offerId")]
    
    pub offer_id: Option<String>,
    /// The price for the product.
    
    pub price: Option<Price>,
    /// The CLDR territory code of the target country of the product.
    #[serde(rename="targetCountry")]
    
    pub target_country: Option<String>,
    /// The title of the product.
    
    pub title: Option<String>,
    /// Variant attributes for the item. Optional.
    #[serde(rename="variantAttributes")]
    
    pub variant_attributes: Option<Vec<OrderLineItemProductVariantAttribute>>,
}

impl client::Part for TestOrderLineItemProduct {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestOrderPaymentMethod {
    /// The card expiration month (January = 1, February = 2 etc.).
    #[serde(rename="expirationMonth")]
    
    pub expiration_month: Option<i32>,
    /// The card expiration year (4-digit, e.g. 2015).
    #[serde(rename="expirationYear")]
    
    pub expiration_year: Option<i32>,
    /// The last four digits of the card number.
    #[serde(rename="lastFourDigits")]
    
    pub last_four_digits: Option<String>,
    /// The billing address.
    #[serde(rename="predefinedBillingAddress")]
    
    pub predefined_billing_address: Option<String>,
    /// The type of instrument. Note that real orders might have different values than the four values accepted by createTestOrder.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for TestOrderPaymentMethod {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UnitInvoice {
    /// Additional charges for a unit, e.g. shipping costs.
    #[serde(rename="additionalCharges")]
    
    pub additional_charges: Option<Vec<UnitInvoiceAdditionalCharge>>,
    /// Promotions applied to a unit.
    
    pub promotions: Option<Vec<Promotion>>,
    /// [required] Price of the unit, before applying taxes.
    #[serde(rename="unitPricePretax")]
    
    pub unit_price_pretax: Option<Price>,
    /// Tax amounts to apply to the unit price.
    #[serde(rename="unitPriceTaxes")]
    
    pub unit_price_taxes: Option<Vec<UnitInvoiceTaxLine>>,
}

impl client::Part for UnitInvoice {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UnitInvoiceAdditionalCharge {
    /// [required] Amount of the additional charge.
    #[serde(rename="additionalChargeAmount")]
    
    pub additional_charge_amount: Option<Amount>,
    /// Promotions applied to the additional charge.
    #[serde(rename="additionalChargePromotions")]
    
    pub additional_charge_promotions: Option<Vec<Promotion>>,
    /// [required] Type of the additional charge.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for UnitInvoiceAdditionalCharge {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UnitInvoiceTaxLine {
    /// [required] Tax amount for the tax type.
    #[serde(rename="taxAmount")]
    
    pub tax_amount: Option<Price>,
    /// Optional name of the tax type. This should only be provided if taxType is otherFeeTax.
    #[serde(rename="taxName")]
    
    pub tax_name: Option<String>,
    /// [required] Type of the tax.
    #[serde(rename="taxType")]
    
    pub tax_type: Option<String>,
}

impl client::Part for UnitInvoiceTaxLine {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *orderinvoice* resources.
/// It is not used directly, but through the [`ShoppingContent`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_content2_sandbox as content2_sandbox;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `createchargeinvoice(...)` and `createrefundinvoice(...)`
/// // to build up your call.
/// let rb = hub.orderinvoices();
/// # }
/// ```
pub struct OrderinvoiceMethods<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
}

impl<'a, S> client::MethodsBuilder for OrderinvoiceMethods<'a, S> {}

impl<'a, S> OrderinvoiceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a charge invoice for a shipment group, and triggers a charge capture for non-facilitated payment orders.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn createchargeinvoice(&self, request: OrderinvoicesCreateChargeInvoiceRequest, merchant_id: u64, order_id: &str) -> OrderinvoiceCreatechargeinvoiceCall<'a, S> {
        OrderinvoiceCreatechargeinvoiceCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a refund invoice for one or more shipment groups, and triggers a refund for non-facilitated payment orders. This can only be used for line items that have previously been charged using createChargeInvoice. All amounts (except for the summary) are incremental with respect to the previous invoice.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn createrefundinvoice(&self, request: OrderinvoicesCreateRefundInvoiceRequest, merchant_id: u64, order_id: &str) -> OrderinvoiceCreaterefundinvoiceCall<'a, S> {
        OrderinvoiceCreaterefundinvoiceCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *orderpayment* resources.
/// It is not used directly, but through the [`ShoppingContent`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_content2_sandbox as content2_sandbox;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `notifyauthapproved(...)`, `notifyauthdeclined(...)`, `notifycharge(...)` and `notifyrefund(...)`
/// // to build up your call.
/// let rb = hub.orderpayments();
/// # }
/// ```
pub struct OrderpaymentMethods<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
}

impl<'a, S> client::MethodsBuilder for OrderpaymentMethods<'a, S> {}

impl<'a, S> OrderpaymentMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Notify about successfully authorizing user's payment method for a given amount.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order for for which payment authorization is happening.
    pub fn notifyauthapproved(&self, request: OrderpaymentsNotifyAuthApprovedRequest, merchant_id: u64, order_id: &str) -> OrderpaymentNotifyauthapprovedCall<'a, S> {
        OrderpaymentNotifyauthapprovedCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Notify about failure to authorize user's payment method.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order for which payment authorization was declined.
    pub fn notifyauthdeclined(&self, request: OrderpaymentsNotifyAuthDeclinedRequest, merchant_id: u64, order_id: &str) -> OrderpaymentNotifyauthdeclinedCall<'a, S> {
        OrderpaymentNotifyauthdeclinedCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
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
    pub fn notifycharge(&self, request: OrderpaymentsNotifyChargeRequest, merchant_id: u64, order_id: &str) -> OrderpaymentNotifychargeCall<'a, S> {
        OrderpaymentNotifychargeCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
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
    pub fn notifyrefund(&self, request: OrderpaymentsNotifyRefundRequest, merchant_id: u64, order_id: &str) -> OrderpaymentNotifyrefundCall<'a, S> {
        OrderpaymentNotifyrefundCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *orderreturn* resources.
/// It is not used directly, but through the [`ShoppingContent`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_content2_sandbox as content2_sandbox;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.orderreturns();
/// # }
/// ```
pub struct OrderreturnMethods<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
}

impl<'a, S> client::MethodsBuilder for OrderreturnMethods<'a, S> {}

impl<'a, S> OrderreturnMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an order return from your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `returnId` - Merchant order return ID generated by Google.
    pub fn get(&self, merchant_id: u64, return_id: &str) -> OrderreturnGetCall<'a, S> {
        OrderreturnGetCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _return_id: return_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists order returns in your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    pub fn list(&self, merchant_id: u64) -> OrderreturnListCall<'a, S> {
        OrderreturnListCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _created_start_date: Default::default(),
            _created_end_date: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *order* resources.
/// It is not used directly, but through the [`ShoppingContent`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_content2_sandbox as content2_sandbox;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `acknowledge(...)`, `advancetestorder(...)`, `cancel(...)`, `cancellineitem(...)`, `canceltestorderbycustomer(...)`, `createtestorder(...)`, `createtestreturn(...)`, `custombatch(...)`, `get(...)`, `getbymerchantorderid(...)`, `gettestordertemplate(...)`, `instorerefundlineitem(...)`, `list(...)`, `refund(...)`, `rejectreturnlineitem(...)`, `returnlineitem(...)`, `returnrefundlineitem(...)`, `setlineitemmetadata(...)`, `shiplineitems(...)`, `updatelineitemshippingdetails(...)`, `updatemerchantorderid(...)` and `updateshipment(...)`
/// // to build up your call.
/// let rb = hub.orders();
/// # }
/// ```
pub struct OrderMethods<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
}

impl<'a, S> client::MethodsBuilder for OrderMethods<'a, S> {}

impl<'a, S> OrderMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Marks an order as acknowledged.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn acknowledge(&self, request: OrdersAcknowledgeRequest, merchant_id: u64, order_id: &str) -> OrderAcknowledgeCall<'a, S> {
        OrderAcknowledgeCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
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
    pub fn advancetestorder(&self, merchant_id: u64, order_id: &str) -> OrderAdvancetestorderCall<'a, S> {
        OrderAdvancetestorderCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
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
    pub fn cancel(&self, request: OrdersCancelRequest, merchant_id: u64, order_id: &str) -> OrderCancelCall<'a, S> {
        OrderCancelCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
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
    pub fn cancellineitem(&self, request: OrdersCancelLineItemRequest, merchant_id: u64, order_id: &str) -> OrderCancellineitemCall<'a, S> {
        OrderCancellineitemCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
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
    pub fn canceltestorderbycustomer(&self, request: OrdersCancelTestOrderByCustomerRequest, merchant_id: u64, order_id: &str) -> OrderCanceltestorderbycustomerCall<'a, S> {
        OrderCanceltestorderbycustomerCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
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
    pub fn createtestorder(&self, request: OrdersCreateTestOrderRequest, merchant_id: u64) -> OrderCreatetestorderCall<'a, S> {
        OrderCreatetestorderCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
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
    pub fn createtestreturn(&self, request: OrdersCreateTestReturnRequest, merchant_id: u64, order_id: &str) -> OrderCreatetestreturnCall<'a, S> {
        OrderCreatetestreturnCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves or modifies multiple orders in a single request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn custombatch(&self, request: OrdersCustomBatchRequest) -> OrderCustombatchCall<'a, S> {
        OrderCustombatchCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
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
    pub fn get(&self, merchant_id: u64, order_id: &str) -> OrderGetCall<'a, S> {
        OrderGetCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
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
    pub fn getbymerchantorderid(&self, merchant_id: u64, merchant_order_id: &str) -> OrderGetbymerchantorderidCall<'a, S> {
        OrderGetbymerchantorderidCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _merchant_order_id: merchant_order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
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
    pub fn gettestordertemplate(&self, merchant_id: u64, template_name: &str) -> OrderGettestordertemplateCall<'a, S> {
        OrderGettestordertemplateCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _template_name: template_name.to_string(),
            _country: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
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
    pub fn instorerefundlineitem(&self, request: OrdersInStoreRefundLineItemRequest, merchant_id: u64, order_id: &str) -> OrderInstorerefundlineitemCall<'a, S> {
        OrderInstorerefundlineitemCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the orders in your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    pub fn list(&self, merchant_id: u64) -> OrderListCall<'a, S> {
        OrderListCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _statuses: Default::default(),
            _placed_date_start: Default::default(),
            _placed_date_end: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _acknowledged: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
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
    pub fn refund(&self, request: OrdersRefundRequest, merchant_id: u64, order_id: &str) -> OrderRefundCall<'a, S> {
        OrderRefundCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
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
    pub fn rejectreturnlineitem(&self, request: OrdersRejectReturnLineItemRequest, merchant_id: u64, order_id: &str) -> OrderRejectreturnlineitemCall<'a, S> {
        OrderRejectreturnlineitemCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
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
    pub fn returnlineitem(&self, request: OrdersReturnLineItemRequest, merchant_id: u64, order_id: &str) -> OrderReturnlineitemCall<'a, S> {
        OrderReturnlineitemCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
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
    pub fn returnrefundlineitem(&self, request: OrdersReturnRefundLineItemRequest, merchant_id: u64, order_id: &str) -> OrderReturnrefundlineitemCall<'a, S> {
        OrderReturnrefundlineitemCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
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
    pub fn setlineitemmetadata(&self, request: OrdersSetLineItemMetadataRequest, merchant_id: u64, order_id: &str) -> OrderSetlineitemmetadataCall<'a, S> {
        OrderSetlineitemmetadataCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
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
    pub fn shiplineitems(&self, request: OrdersShipLineItemsRequest, merchant_id: u64, order_id: &str) -> OrderShiplineitemCall<'a, S> {
        OrderShiplineitemCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
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
    pub fn updatelineitemshippingdetails(&self, request: OrdersUpdateLineItemShippingDetailsRequest, merchant_id: u64, order_id: &str) -> OrderUpdatelineitemshippingdetailCall<'a, S> {
        OrderUpdatelineitemshippingdetailCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
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
    pub fn updatemerchantorderid(&self, request: OrdersUpdateMerchantOrderIdRequest, merchant_id: u64, order_id: &str) -> OrderUpdatemerchantorderidCall<'a, S> {
        OrderUpdatemerchantorderidCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
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
    pub fn updateshipment(&self, request: OrdersUpdateShipmentRequest, merchant_id: u64, order_id: &str) -> OrderUpdateshipmentCall<'a, S> {
        OrderUpdateshipmentCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Creates a charge invoice for a shipment group, and triggers a charge capture for non-facilitated payment orders.
///
/// A builder for the *createchargeinvoice* method supported by a *orderinvoice* resource.
/// It is not used directly, but through a [`OrderinvoiceMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::api::OrderinvoicesCreateChargeInvoiceRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrderinvoicesCreateChargeInvoiceRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orderinvoices().createchargeinvoice(req, 64, "orderId")
///              .doit().await;
/// # }
/// ```
pub struct OrderinvoiceCreatechargeinvoiceCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _request: OrderinvoicesCreateChargeInvoiceRequest,
    _merchant_id: u64,
    _order_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderinvoiceCreatechargeinvoiceCall<'a, S> {}

impl<'a, S> OrderinvoiceCreatechargeinvoiceCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrderinvoicesCreateChargeInvoiceResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orderinvoices.createchargeinvoice",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        params.push("orderId", self._order_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/orderinvoices/{orderId}/createChargeInvoice";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["orderId", "merchantId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: OrderinvoicesCreateChargeInvoiceRequest) -> OrderinvoiceCreatechargeinvoiceCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderinvoiceCreatechargeinvoiceCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderinvoiceCreatechargeinvoiceCall<'a, S> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderinvoiceCreatechargeinvoiceCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderinvoiceCreatechargeinvoiceCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderinvoiceCreatechargeinvoiceCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderinvoiceCreatechargeinvoiceCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderinvoiceCreatechargeinvoiceCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Creates a refund invoice for one or more shipment groups, and triggers a refund for non-facilitated payment orders. This can only be used for line items that have previously been charged using createChargeInvoice. All amounts (except for the summary) are incremental with respect to the previous invoice.
///
/// A builder for the *createrefundinvoice* method supported by a *orderinvoice* resource.
/// It is not used directly, but through a [`OrderinvoiceMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::api::OrderinvoicesCreateRefundInvoiceRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrderinvoicesCreateRefundInvoiceRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orderinvoices().createrefundinvoice(req, 85, "orderId")
///              .doit().await;
/// # }
/// ```
pub struct OrderinvoiceCreaterefundinvoiceCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _request: OrderinvoicesCreateRefundInvoiceRequest,
    _merchant_id: u64,
    _order_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderinvoiceCreaterefundinvoiceCall<'a, S> {}

impl<'a, S> OrderinvoiceCreaterefundinvoiceCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrderinvoicesCreateRefundInvoiceResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orderinvoices.createrefundinvoice",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        params.push("orderId", self._order_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/orderinvoices/{orderId}/createRefundInvoice";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["orderId", "merchantId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: OrderinvoicesCreateRefundInvoiceRequest) -> OrderinvoiceCreaterefundinvoiceCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderinvoiceCreaterefundinvoiceCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderinvoiceCreaterefundinvoiceCall<'a, S> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderinvoiceCreaterefundinvoiceCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderinvoiceCreaterefundinvoiceCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderinvoiceCreaterefundinvoiceCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderinvoiceCreaterefundinvoiceCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderinvoiceCreaterefundinvoiceCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Notify about successfully authorizing user's payment method for a given amount.
///
/// A builder for the *notifyauthapproved* method supported by a *orderpayment* resource.
/// It is not used directly, but through a [`OrderpaymentMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::api::OrderpaymentsNotifyAuthApprovedRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrderpaymentsNotifyAuthApprovedRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orderpayments().notifyauthapproved(req, 51, "orderId")
///              .doit().await;
/// # }
/// ```
pub struct OrderpaymentNotifyauthapprovedCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _request: OrderpaymentsNotifyAuthApprovedRequest,
    _merchant_id: u64,
    _order_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderpaymentNotifyauthapprovedCall<'a, S> {}

impl<'a, S> OrderpaymentNotifyauthapprovedCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrderpaymentsNotifyAuthApprovedResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orderpayments.notifyauthapproved",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        params.push("orderId", self._order_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/orderpayments/{orderId}/notifyAuthApproved";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["orderId", "merchantId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: OrderpaymentsNotifyAuthApprovedRequest) -> OrderpaymentNotifyauthapprovedCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderpaymentNotifyauthapprovedCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The ID of the order for for which payment authorization is happening.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderpaymentNotifyauthapprovedCall<'a, S> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderpaymentNotifyauthapprovedCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderpaymentNotifyauthapprovedCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderpaymentNotifyauthapprovedCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderpaymentNotifyauthapprovedCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderpaymentNotifyauthapprovedCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Notify about failure to authorize user's payment method.
///
/// A builder for the *notifyauthdeclined* method supported by a *orderpayment* resource.
/// It is not used directly, but through a [`OrderpaymentMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::api::OrderpaymentsNotifyAuthDeclinedRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrderpaymentsNotifyAuthDeclinedRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orderpayments().notifyauthdeclined(req, 94, "orderId")
///              .doit().await;
/// # }
/// ```
pub struct OrderpaymentNotifyauthdeclinedCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _request: OrderpaymentsNotifyAuthDeclinedRequest,
    _merchant_id: u64,
    _order_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderpaymentNotifyauthdeclinedCall<'a, S> {}

impl<'a, S> OrderpaymentNotifyauthdeclinedCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrderpaymentsNotifyAuthDeclinedResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orderpayments.notifyauthdeclined",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        params.push("orderId", self._order_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/orderpayments/{orderId}/notifyAuthDeclined";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["orderId", "merchantId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: OrderpaymentsNotifyAuthDeclinedRequest) -> OrderpaymentNotifyauthdeclinedCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderpaymentNotifyauthdeclinedCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The ID of the order for which payment authorization was declined.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderpaymentNotifyauthdeclinedCall<'a, S> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderpaymentNotifyauthdeclinedCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderpaymentNotifyauthdeclinedCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderpaymentNotifyauthdeclinedCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderpaymentNotifyauthdeclinedCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderpaymentNotifyauthdeclinedCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Notify about charge on user's selected payments method.
///
/// A builder for the *notifycharge* method supported by a *orderpayment* resource.
/// It is not used directly, but through a [`OrderpaymentMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::api::OrderpaymentsNotifyChargeRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrderpaymentsNotifyChargeRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orderpayments().notifycharge(req, 84, "orderId")
///              .doit().await;
/// # }
/// ```
pub struct OrderpaymentNotifychargeCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _request: OrderpaymentsNotifyChargeRequest,
    _merchant_id: u64,
    _order_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderpaymentNotifychargeCall<'a, S> {}

impl<'a, S> OrderpaymentNotifychargeCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrderpaymentsNotifyChargeResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orderpayments.notifycharge",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        params.push("orderId", self._order_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/orderpayments/{orderId}/notifyCharge";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["orderId", "merchantId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: OrderpaymentsNotifyChargeRequest) -> OrderpaymentNotifychargeCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderpaymentNotifychargeCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The ID of the order for which charge is happening.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderpaymentNotifychargeCall<'a, S> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderpaymentNotifychargeCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderpaymentNotifychargeCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderpaymentNotifychargeCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderpaymentNotifychargeCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderpaymentNotifychargeCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Notify about refund on user's selected payments method.
///
/// A builder for the *notifyrefund* method supported by a *orderpayment* resource.
/// It is not used directly, but through a [`OrderpaymentMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::api::OrderpaymentsNotifyRefundRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrderpaymentsNotifyRefundRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orderpayments().notifyrefund(req, 45, "orderId")
///              .doit().await;
/// # }
/// ```
pub struct OrderpaymentNotifyrefundCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _request: OrderpaymentsNotifyRefundRequest,
    _merchant_id: u64,
    _order_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderpaymentNotifyrefundCall<'a, S> {}

impl<'a, S> OrderpaymentNotifyrefundCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrderpaymentsNotifyRefundResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orderpayments.notifyrefund",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        params.push("orderId", self._order_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/orderpayments/{orderId}/notifyRefund";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["orderId", "merchantId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: OrderpaymentsNotifyRefundRequest) -> OrderpaymentNotifyrefundCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderpaymentNotifyrefundCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The ID of the order for which charge is happening.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderpaymentNotifyrefundCall<'a, S> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderpaymentNotifyrefundCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderpaymentNotifyrefundCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderpaymentNotifyrefundCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderpaymentNotifyrefundCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderpaymentNotifyrefundCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Retrieves an order return from your Merchant Center account.
///
/// A builder for the *get* method supported by a *orderreturn* resource.
/// It is not used directly, but through a [`OrderreturnMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orderreturns().get(15, "returnId")
///              .doit().await;
/// # }
/// ```
pub struct OrderreturnGetCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _merchant_id: u64,
    _return_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderreturnGetCall<'a, S> {}

impl<'a, S> OrderreturnGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, MerchantOrderReturn)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orderreturns.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "merchantId", "returnId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        params.push("returnId", self._return_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/orderreturns/{returnId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{returnId}", "returnId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["returnId", "merchantId"];
            params.remove_params(&to_remove);
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


    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderreturnGetCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// Merchant order return ID generated by Google.
    ///
    /// Sets the *return id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn return_id(mut self, new_value: &str) -> OrderreturnGetCall<'a, S> {
        self._return_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderreturnGetCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderreturnGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderreturnGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderreturnGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderreturnGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Lists order returns in your Merchant Center account.
///
/// A builder for the *list* method supported by a *orderreturn* resource.
/// It is not used directly, but through a [`OrderreturnMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orderreturns().list(31)
///              .page_token("sed")
///              .order_by("no")
///              .max_results(86)
///              .created_start_date("kasd")
///              .created_end_date("et")
///              .doit().await;
/// # }
/// ```
pub struct OrderreturnListCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _merchant_id: u64,
    _page_token: Option<String>,
    _order_by: Option<String>,
    _max_results: Option<u32>,
    _created_start_date: Option<String>,
    _created_end_date: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderreturnListCall<'a, S> {}

impl<'a, S> OrderreturnListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrderreturnsListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orderreturns.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "merchantId", "pageToken", "orderBy", "maxResults", "createdStartDate", "createdEndDate"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(8 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._order_by.as_ref() {
            params.push("orderBy", value);
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }
        if let Some(value) = self._created_start_date.as_ref() {
            params.push("createdStartDate", value);
        }
        if let Some(value) = self._created_end_date.as_ref() {
            params.push("createdEndDate", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/orderreturns";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["merchantId"];
            params.remove_params(&to_remove);
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


    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderreturnListCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The token returned by the previous request.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> OrderreturnListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Return the results in the specified order.
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> OrderreturnListCall<'a, S> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// The maximum number of order returns to return in the response, used for paging. The default value is 25 returns per page, and the maximum allowed value is 250 returns per page.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> OrderreturnListCall<'a, S> {
        self._max_results = Some(new_value);
        self
    }
    /// Obtains order returns created after this date (inclusively), in ISO 8601 format.
    ///
    /// Sets the *created start date* query property to the given value.
    pub fn created_start_date(mut self, new_value: &str) -> OrderreturnListCall<'a, S> {
        self._created_start_date = Some(new_value.to_string());
        self
    }
    /// Obtains order returns created before this date (inclusively), in ISO 8601 format.
    ///
    /// Sets the *created end date* query property to the given value.
    pub fn created_end_date(mut self, new_value: &str) -> OrderreturnListCall<'a, S> {
        self._created_end_date = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderreturnListCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderreturnListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderreturnListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderreturnListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderreturnListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Marks an order as acknowledged.
///
/// A builder for the *acknowledge* method supported by a *order* resource.
/// It is not used directly, but through a [`OrderMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::api::OrdersAcknowledgeRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersAcknowledgeRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().acknowledge(req, 58, "orderId")
///              .doit().await;
/// # }
/// ```
pub struct OrderAcknowledgeCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _request: OrdersAcknowledgeRequest,
    _merchant_id: u64,
    _order_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderAcknowledgeCall<'a, S> {}

impl<'a, S> OrderAcknowledgeCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrdersAcknowledgeResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orders.acknowledge",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        params.push("orderId", self._order_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/acknowledge";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["orderId", "merchantId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: OrdersAcknowledgeRequest) -> OrderAcknowledgeCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderAcknowledgeCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderAcknowledgeCall<'a, S> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderAcknowledgeCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderAcknowledgeCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderAcknowledgeCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderAcknowledgeCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderAcknowledgeCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Sandbox only. Moves a test order from state "inProgress" to state "pendingShipment".
///
/// A builder for the *advancetestorder* method supported by a *order* resource.
/// It is not used directly, but through a [`OrderMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().advancetestorder(33, "orderId")
///              .doit().await;
/// # }
/// ```
pub struct OrderAdvancetestorderCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _merchant_id: u64,
    _order_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderAdvancetestorderCall<'a, S> {}

impl<'a, S> OrderAdvancetestorderCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrdersAdvanceTestOrderResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orders.advancetestorder",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        params.push("orderId", self._order_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/testorders/{orderId}/advance";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["orderId", "merchantId"];
            params.remove_params(&to_remove);
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
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
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


    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderAdvancetestorderCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The ID of the test order to modify.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderAdvancetestorderCall<'a, S> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderAdvancetestorderCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderAdvancetestorderCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderAdvancetestorderCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderAdvancetestorderCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderAdvancetestorderCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Cancels all line items in an order, making a full refund.
///
/// A builder for the *cancel* method supported by a *order* resource.
/// It is not used directly, but through a [`OrderMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::api::OrdersCancelRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersCancelRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().cancel(req, 70, "orderId")
///              .doit().await;
/// # }
/// ```
pub struct OrderCancelCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _request: OrdersCancelRequest,
    _merchant_id: u64,
    _order_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderCancelCall<'a, S> {}

impl<'a, S> OrderCancelCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrdersCancelResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orders.cancel",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        params.push("orderId", self._order_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/cancel";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["orderId", "merchantId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: OrdersCancelRequest) -> OrderCancelCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderCancelCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The ID of the order to cancel.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderCancelCall<'a, S> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderCancelCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderCancelCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderCancelCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderCancelCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderCancelCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Cancels a line item, making a full refund.
///
/// A builder for the *cancellineitem* method supported by a *order* resource.
/// It is not used directly, but through a [`OrderMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::api::OrdersCancelLineItemRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersCancelLineItemRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().cancellineitem(req, 81, "orderId")
///              .doit().await;
/// # }
/// ```
pub struct OrderCancellineitemCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _request: OrdersCancelLineItemRequest,
    _merchant_id: u64,
    _order_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderCancellineitemCall<'a, S> {}

impl<'a, S> OrderCancellineitemCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrdersCancelLineItemResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orders.cancellineitem",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        params.push("orderId", self._order_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/cancelLineItem";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["orderId", "merchantId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: OrdersCancelLineItemRequest) -> OrderCancellineitemCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderCancellineitemCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderCancellineitemCall<'a, S> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderCancellineitemCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderCancellineitemCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderCancellineitemCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderCancellineitemCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderCancellineitemCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Sandbox only. Cancels a test order for customer-initiated cancellation.
///
/// A builder for the *canceltestorderbycustomer* method supported by a *order* resource.
/// It is not used directly, but through a [`OrderMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::api::OrdersCancelTestOrderByCustomerRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersCancelTestOrderByCustomerRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().canceltestorderbycustomer(req, 79, "orderId")
///              .doit().await;
/// # }
/// ```
pub struct OrderCanceltestorderbycustomerCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _request: OrdersCancelTestOrderByCustomerRequest,
    _merchant_id: u64,
    _order_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderCanceltestorderbycustomerCall<'a, S> {}

impl<'a, S> OrderCanceltestorderbycustomerCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrdersCancelTestOrderByCustomerResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orders.canceltestorderbycustomer",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        params.push("orderId", self._order_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/testorders/{orderId}/cancelByCustomer";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["orderId", "merchantId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: OrdersCancelTestOrderByCustomerRequest) -> OrderCanceltestorderbycustomerCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderCanceltestorderbycustomerCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The ID of the test order to cancel.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderCanceltestorderbycustomerCall<'a, S> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderCanceltestorderbycustomerCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderCanceltestorderbycustomerCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderCanceltestorderbycustomerCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderCanceltestorderbycustomerCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderCanceltestorderbycustomerCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Sandbox only. Creates a test order.
///
/// A builder for the *createtestorder* method supported by a *order* resource.
/// It is not used directly, but through a [`OrderMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::api::OrdersCreateTestOrderRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersCreateTestOrderRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().createtestorder(req, 99)
///              .doit().await;
/// # }
/// ```
pub struct OrderCreatetestorderCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _request: OrdersCreateTestOrderRequest,
    _merchant_id: u64,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderCreatetestorderCall<'a, S> {}

impl<'a, S> OrderCreatetestorderCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrdersCreateTestOrderResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orders.createtestorder",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "merchantId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/testorders";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["merchantId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: OrdersCreateTestOrderRequest) -> OrderCreatetestorderCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the account that should manage the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderCreatetestorderCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderCreatetestorderCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderCreatetestorderCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderCreatetestorderCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderCreatetestorderCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderCreatetestorderCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Sandbox only. Creates a test return.
///
/// A builder for the *createtestreturn* method supported by a *order* resource.
/// It is not used directly, but through a [`OrderMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::api::OrdersCreateTestReturnRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersCreateTestReturnRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().createtestreturn(req, 5, "orderId")
///              .doit().await;
/// # }
/// ```
pub struct OrderCreatetestreturnCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _request: OrdersCreateTestReturnRequest,
    _merchant_id: u64,
    _order_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderCreatetestreturnCall<'a, S> {}

impl<'a, S> OrderCreatetestreturnCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrdersCreateTestReturnResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orders.createtestreturn",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        params.push("orderId", self._order_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/testreturn";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["orderId", "merchantId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: OrdersCreateTestReturnRequest) -> OrderCreatetestreturnCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderCreatetestreturnCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderCreatetestreturnCall<'a, S> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderCreatetestreturnCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderCreatetestreturnCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderCreatetestreturnCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderCreatetestreturnCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderCreatetestreturnCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Retrieves or modifies multiple orders in a single request.
///
/// A builder for the *custombatch* method supported by a *order* resource.
/// It is not used directly, but through a [`OrderMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::api::OrdersCustomBatchRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersCustomBatchRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().custombatch(req)
///              .doit().await;
/// # }
/// ```
pub struct OrderCustombatchCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _request: OrdersCustomBatchRequest,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderCustombatchCall<'a, S> {}

impl<'a, S> OrderCustombatchCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrdersCustomBatchResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orders.custombatch",
                               http_method: hyper::Method::POST });

        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "orders/batch";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: OrdersCustomBatchRequest) -> OrderCustombatchCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderCustombatchCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderCustombatchCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderCustombatchCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderCustombatchCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderCustombatchCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Retrieves an order from your Merchant Center account.
///
/// A builder for the *get* method supported by a *order* resource.
/// It is not used directly, but through a [`OrderMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().get(52, "orderId")
///              .doit().await;
/// # }
/// ```
pub struct OrderGetCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _merchant_id: u64,
    _order_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderGetCall<'a, S> {}

impl<'a, S> OrderGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Order)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orders.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        params.push("orderId", self._order_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["orderId", "merchantId"];
            params.remove_params(&to_remove);
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


    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderGetCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderGetCall<'a, S> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderGetCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Retrieves an order using merchant order id.
///
/// A builder for the *getbymerchantorderid* method supported by a *order* resource.
/// It is not used directly, but through a [`OrderMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().getbymerchantorderid(79, "merchantOrderId")
///              .doit().await;
/// # }
/// ```
pub struct OrderGetbymerchantorderidCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _merchant_id: u64,
    _merchant_order_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderGetbymerchantorderidCall<'a, S> {}

impl<'a, S> OrderGetbymerchantorderidCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrdersGetByMerchantOrderIdResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orders.getbymerchantorderid",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "merchantId", "merchantOrderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        params.push("merchantOrderId", self._merchant_order_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/ordersbymerchantid/{merchantOrderId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{merchantOrderId}", "merchantOrderId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["merchantOrderId", "merchantId"];
            params.remove_params(&to_remove);
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


    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderGetbymerchantorderidCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The merchant order id to be looked for.
    ///
    /// Sets the *merchant order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_order_id(mut self, new_value: &str) -> OrderGetbymerchantorderidCall<'a, S> {
        self._merchant_order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderGetbymerchantorderidCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderGetbymerchantorderidCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderGetbymerchantorderidCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderGetbymerchantorderidCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderGetbymerchantorderidCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Sandbox only. Retrieves an order template that can be used to quickly create a new order in sandbox.
///
/// A builder for the *gettestordertemplate* method supported by a *order* resource.
/// It is not used directly, but through a [`OrderMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().gettestordertemplate(86, "templateName")
///              .country("duo")
///              .doit().await;
/// # }
/// ```
pub struct OrderGettestordertemplateCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _merchant_id: u64,
    _template_name: String,
    _country: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderGettestordertemplateCall<'a, S> {}

impl<'a, S> OrderGettestordertemplateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrdersGetTestOrderTemplateResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orders.gettestordertemplate",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "merchantId", "templateName", "country"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        params.push("templateName", self._template_name);
        if let Some(value) = self._country.as_ref() {
            params.push("country", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/testordertemplates/{templateName}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{templateName}", "templateName")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["templateName", "merchantId"];
            params.remove_params(&to_remove);
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


    /// The ID of the account that should manage the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderGettestordertemplateCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The name of the template to retrieve.
    ///
    /// Sets the *template name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn template_name(mut self, new_value: &str) -> OrderGettestordertemplateCall<'a, S> {
        self._template_name = new_value.to_string();
        self
    }
    /// The country of the template to retrieve. Defaults to US.
    ///
    /// Sets the *country* query property to the given value.
    pub fn country(mut self, new_value: &str) -> OrderGettestordertemplateCall<'a, S> {
        self._country = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderGettestordertemplateCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderGettestordertemplateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderGettestordertemplateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderGettestordertemplateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderGettestordertemplateCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Notifies that item return and refund was handled directly by merchant outside of Google payments processing (e.g. cash refund done in store).
///
/// A builder for the *instorerefundlineitem* method supported by a *order* resource.
/// It is not used directly, but through a [`OrderMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::api::OrdersInStoreRefundLineItemRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersInStoreRefundLineItemRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().instorerefundlineitem(req, 25, "orderId")
///              .doit().await;
/// # }
/// ```
pub struct OrderInstorerefundlineitemCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _request: OrdersInStoreRefundLineItemRequest,
    _merchant_id: u64,
    _order_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderInstorerefundlineitemCall<'a, S> {}

impl<'a, S> OrderInstorerefundlineitemCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrdersInStoreRefundLineItemResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orders.instorerefundlineitem",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        params.push("orderId", self._order_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/inStoreRefundLineItem";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["orderId", "merchantId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: OrdersInStoreRefundLineItemRequest) -> OrderInstorerefundlineitemCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderInstorerefundlineitemCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderInstorerefundlineitemCall<'a, S> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderInstorerefundlineitemCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderInstorerefundlineitemCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderInstorerefundlineitemCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderInstorerefundlineitemCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderInstorerefundlineitemCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Lists the orders in your Merchant Center account.
///
/// A builder for the *list* method supported by a *order* resource.
/// It is not used directly, but through a [`OrderMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().list(13)
///              .add_statuses("Stet")
///              .placed_date_start("vero")
///              .placed_date_end("elitr")
///              .page_token("Lorem")
///              .order_by("diam")
///              .max_results(40)
///              .acknowledged(false)
///              .doit().await;
/// # }
/// ```
pub struct OrderListCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _merchant_id: u64,
    _statuses: Vec<String>,
    _placed_date_start: Option<String>,
    _placed_date_end: Option<String>,
    _page_token: Option<String>,
    _order_by: Option<String>,
    _max_results: Option<u32>,
    _acknowledged: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderListCall<'a, S> {}

impl<'a, S> OrderListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrdersListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orders.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "merchantId", "statuses", "placedDateStart", "placedDateEnd", "pageToken", "orderBy", "maxResults", "acknowledged"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(10 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        if self._statuses.len() > 0 {
            for f in self._statuses.iter() {
                params.push("statuses", f);
            }
        }
        if let Some(value) = self._placed_date_start.as_ref() {
            params.push("placedDateStart", value);
        }
        if let Some(value) = self._placed_date_end.as_ref() {
            params.push("placedDateEnd", value);
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._order_by.as_ref() {
            params.push("orderBy", value);
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }
        if let Some(value) = self._acknowledged.as_ref() {
            params.push("acknowledged", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/orders";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["merchantId"];
            params.remove_params(&to_remove);
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


    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderListCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// Obtains orders that match any of the specified statuses. Multiple values can be specified with comma separation. Additionally, please note that active is a shortcut for pendingShipment and partiallyShipped, and completed is a shortcut for shipped , partiallyDelivered, delivered, partiallyReturned, returned, and canceled.
    ///
    /// Append the given value to the *statuses* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_statuses(mut self, new_value: &str) -> OrderListCall<'a, S> {
        self._statuses.push(new_value.to_string());
        self
    }
    /// Obtains orders placed after this date (inclusively), in ISO 8601 format.
    ///
    /// Sets the *placed date start* query property to the given value.
    pub fn placed_date_start(mut self, new_value: &str) -> OrderListCall<'a, S> {
        self._placed_date_start = Some(new_value.to_string());
        self
    }
    /// Obtains orders placed before this date (exclusively), in ISO 8601 format.
    ///
    /// Sets the *placed date end* query property to the given value.
    pub fn placed_date_end(mut self, new_value: &str) -> OrderListCall<'a, S> {
        self._placed_date_end = Some(new_value.to_string());
        self
    }
    /// The token returned by the previous request.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> OrderListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The ordering of the returned list. The only supported value are placedDate desc and placedDate asc for now, which returns orders sorted by placement date. "placedDate desc" stands for listing orders by placement date, from oldest to most recent. "placedDate asc" stands for listing orders by placement date, from most recent to oldest. In future releases we'll support other sorting criteria.
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> OrderListCall<'a, S> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// The maximum number of orders to return in the response, used for paging. The default value is 25 orders per page, and the maximum allowed value is 250 orders per page.
    /// Known issue: All List calls will return all Orders without limit regardless of the value of this field.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> OrderListCall<'a, S> {
        self._max_results = Some(new_value);
        self
    }
    /// Obtains orders that match the acknowledgement status. When set to true, obtains orders that have been acknowledged. When false, obtains orders that have not been acknowledged.
    /// We recommend using this filter set to false, in conjunction with the acknowledge call, such that only un-acknowledged orders are returned.
    ///
    /// Sets the *acknowledged* query property to the given value.
    pub fn acknowledged(mut self, new_value: bool) -> OrderListCall<'a, S> {
        self._acknowledged = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderListCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Deprecated, please use returnRefundLineItem instead.
///
/// A builder for the *refund* method supported by a *order* resource.
/// It is not used directly, but through a [`OrderMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::api::OrdersRefundRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersRefundRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().refund(req, 78, "orderId")
///              .doit().await;
/// # }
/// ```
pub struct OrderRefundCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _request: OrdersRefundRequest,
    _merchant_id: u64,
    _order_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderRefundCall<'a, S> {}

impl<'a, S> OrderRefundCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrdersRefundResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orders.refund",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        params.push("orderId", self._order_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/refund";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["orderId", "merchantId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: OrdersRefundRequest) -> OrderRefundCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderRefundCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The ID of the order to refund.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderRefundCall<'a, S> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderRefundCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderRefundCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderRefundCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderRefundCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderRefundCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Rejects return on an line item.
///
/// A builder for the *rejectreturnlineitem* method supported by a *order* resource.
/// It is not used directly, but through a [`OrderMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::api::OrdersRejectReturnLineItemRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersRejectReturnLineItemRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().rejectreturnlineitem(req, 55, "orderId")
///              .doit().await;
/// # }
/// ```
pub struct OrderRejectreturnlineitemCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _request: OrdersRejectReturnLineItemRequest,
    _merchant_id: u64,
    _order_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderRejectreturnlineitemCall<'a, S> {}

impl<'a, S> OrderRejectreturnlineitemCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrdersRejectReturnLineItemResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orders.rejectreturnlineitem",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        params.push("orderId", self._order_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/rejectReturnLineItem";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["orderId", "merchantId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: OrdersRejectReturnLineItemRequest) -> OrderRejectreturnlineitemCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderRejectreturnlineitemCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderRejectreturnlineitemCall<'a, S> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderRejectreturnlineitemCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderRejectreturnlineitemCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderRejectreturnlineitemCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderRejectreturnlineitemCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderRejectreturnlineitemCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Returns a line item.
///
/// A builder for the *returnlineitem* method supported by a *order* resource.
/// It is not used directly, but through a [`OrderMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::api::OrdersReturnLineItemRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersReturnLineItemRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().returnlineitem(req, 29, "orderId")
///              .doit().await;
/// # }
/// ```
pub struct OrderReturnlineitemCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _request: OrdersReturnLineItemRequest,
    _merchant_id: u64,
    _order_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderReturnlineitemCall<'a, S> {}

impl<'a, S> OrderReturnlineitemCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrdersReturnLineItemResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orders.returnlineitem",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        params.push("orderId", self._order_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/returnLineItem";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["orderId", "merchantId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: OrdersReturnLineItemRequest) -> OrderReturnlineitemCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderReturnlineitemCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderReturnlineitemCall<'a, S> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderReturnlineitemCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderReturnlineitemCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderReturnlineitemCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderReturnlineitemCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderReturnlineitemCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Returns and refunds a line item. Note that this method can only be called on fully shipped orders.
///
/// A builder for the *returnrefundlineitem* method supported by a *order* resource.
/// It is not used directly, but through a [`OrderMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::api::OrdersReturnRefundLineItemRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersReturnRefundLineItemRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().returnrefundlineitem(req, 5, "orderId")
///              .doit().await;
/// # }
/// ```
pub struct OrderReturnrefundlineitemCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _request: OrdersReturnRefundLineItemRequest,
    _merchant_id: u64,
    _order_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderReturnrefundlineitemCall<'a, S> {}

impl<'a, S> OrderReturnrefundlineitemCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrdersReturnRefundLineItemResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orders.returnrefundlineitem",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        params.push("orderId", self._order_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/returnRefundLineItem";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["orderId", "merchantId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: OrdersReturnRefundLineItemRequest) -> OrderReturnrefundlineitemCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderReturnrefundlineitemCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderReturnrefundlineitemCall<'a, S> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderReturnrefundlineitemCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderReturnrefundlineitemCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderReturnrefundlineitemCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderReturnrefundlineitemCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderReturnrefundlineitemCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Sets (overrides) merchant provided annotations on the line item.
///
/// A builder for the *setlineitemmetadata* method supported by a *order* resource.
/// It is not used directly, but through a [`OrderMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::api::OrdersSetLineItemMetadataRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersSetLineItemMetadataRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().setlineitemmetadata(req, 71, "orderId")
///              .doit().await;
/// # }
/// ```
pub struct OrderSetlineitemmetadataCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _request: OrdersSetLineItemMetadataRequest,
    _merchant_id: u64,
    _order_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderSetlineitemmetadataCall<'a, S> {}

impl<'a, S> OrderSetlineitemmetadataCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrdersSetLineItemMetadataResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orders.setlineitemmetadata",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        params.push("orderId", self._order_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/setLineItemMetadata";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["orderId", "merchantId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: OrdersSetLineItemMetadataRequest) -> OrderSetlineitemmetadataCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderSetlineitemmetadataCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderSetlineitemmetadataCall<'a, S> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderSetlineitemmetadataCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderSetlineitemmetadataCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderSetlineitemmetadataCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderSetlineitemmetadataCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderSetlineitemmetadataCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Marks line item(s) as shipped.
///
/// A builder for the *shiplineitems* method supported by a *order* resource.
/// It is not used directly, but through a [`OrderMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::api::OrdersShipLineItemsRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersShipLineItemsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().shiplineitems(req, 82, "orderId")
///              .doit().await;
/// # }
/// ```
pub struct OrderShiplineitemCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _request: OrdersShipLineItemsRequest,
    _merchant_id: u64,
    _order_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderShiplineitemCall<'a, S> {}

impl<'a, S> OrderShiplineitemCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrdersShipLineItemsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orders.shiplineitems",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        params.push("orderId", self._order_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/shipLineItems";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["orderId", "merchantId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: OrdersShipLineItemsRequest) -> OrderShiplineitemCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderShiplineitemCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderShiplineitemCall<'a, S> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderShiplineitemCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderShiplineitemCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderShiplineitemCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderShiplineitemCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderShiplineitemCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Updates ship by and delivery by dates for a line item.
///
/// A builder for the *updatelineitemshippingdetails* method supported by a *order* resource.
/// It is not used directly, but through a [`OrderMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::api::OrdersUpdateLineItemShippingDetailsRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersUpdateLineItemShippingDetailsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().updatelineitemshippingdetails(req, 27, "orderId")
///              .doit().await;
/// # }
/// ```
pub struct OrderUpdatelineitemshippingdetailCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _request: OrdersUpdateLineItemShippingDetailsRequest,
    _merchant_id: u64,
    _order_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderUpdatelineitemshippingdetailCall<'a, S> {}

impl<'a, S> OrderUpdatelineitemshippingdetailCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrdersUpdateLineItemShippingDetailsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orders.updatelineitemshippingdetails",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        params.push("orderId", self._order_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/updateLineItemShippingDetails";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["orderId", "merchantId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: OrdersUpdateLineItemShippingDetailsRequest) -> OrderUpdatelineitemshippingdetailCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderUpdatelineitemshippingdetailCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderUpdatelineitemshippingdetailCall<'a, S> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderUpdatelineitemshippingdetailCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderUpdatelineitemshippingdetailCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderUpdatelineitemshippingdetailCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderUpdatelineitemshippingdetailCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderUpdatelineitemshippingdetailCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Updates the merchant order ID for a given order.
///
/// A builder for the *updatemerchantorderid* method supported by a *order* resource.
/// It is not used directly, but through a [`OrderMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::api::OrdersUpdateMerchantOrderIdRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersUpdateMerchantOrderIdRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().updatemerchantorderid(req, 23, "orderId")
///              .doit().await;
/// # }
/// ```
pub struct OrderUpdatemerchantorderidCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _request: OrdersUpdateMerchantOrderIdRequest,
    _merchant_id: u64,
    _order_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderUpdatemerchantorderidCall<'a, S> {}

impl<'a, S> OrderUpdatemerchantorderidCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrdersUpdateMerchantOrderIdResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orders.updatemerchantorderid",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        params.push("orderId", self._order_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/updateMerchantOrderId";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["orderId", "merchantId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: OrdersUpdateMerchantOrderIdRequest) -> OrderUpdatemerchantorderidCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderUpdatemerchantorderidCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderUpdatemerchantorderidCall<'a, S> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderUpdatemerchantorderidCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderUpdatemerchantorderidCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderUpdatemerchantorderidCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderUpdatemerchantorderidCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderUpdatemerchantorderidCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Updates a shipment's status, carrier, and/or tracking ID.
///
/// A builder for the *updateshipment* method supported by a *order* resource.
/// It is not used directly, but through a [`OrderMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_content2_sandbox as content2_sandbox;
/// use content2_sandbox::api::OrdersUpdateShipmentRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = OrdersUpdateShipmentRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().updateshipment(req, 67, "orderId")
///              .doit().await;
/// # }
/// ```
pub struct OrderUpdateshipmentCall<'a, S>
    where S: 'a {

    hub: &'a ShoppingContent<S>,
    _request: OrdersUpdateShipmentRequest,
    _merchant_id: u64,
    _order_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for OrderUpdateshipmentCall<'a, S> {}

impl<'a, S> OrderUpdateshipmentCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OrdersUpdateShipmentResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "content.orders.updateshipment",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "merchantId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("merchantId", self._merchant_id.to_string());
        params.push("orderId", self._order_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{merchantId}/orders/{orderId}/updateShipment";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{merchantId}", "merchantId"), ("{orderId}", "orderId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["orderId", "merchantId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: OrdersUpdateShipmentRequest) -> OrderUpdateshipmentCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the account that manages the order. This cannot be a multi-client account.
    ///
    /// Sets the *merchant id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn merchant_id(mut self, new_value: u64) -> OrderUpdateshipmentCall<'a, S> {
        self._merchant_id = new_value;
        self
    }
    /// The ID of the order.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> OrderUpdateshipmentCall<'a, S> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OrderUpdateshipmentCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> OrderUpdateshipmentCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> OrderUpdateshipmentCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> OrderUpdateshipmentCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> OrderUpdateshipmentCall<'a, S> {
        self._scopes.clear();
        self
    }
}


