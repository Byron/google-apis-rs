use super::*;
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


