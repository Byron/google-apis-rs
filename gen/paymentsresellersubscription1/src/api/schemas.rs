use super::*;
/// Describes the amount unit including the currency code.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1Amount {
    /// Required. Amount in micros (1_000_000 micros = 1 currency unit)
    #[serde(rename="amountMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub amount_micros: Option<i64>,
    /// Required. Currency codes in accordance with [ISO-4217 Currency Codes] (https://en.wikipedia.org/wiki/ISO_4217). For example, USD.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
}

impl client::Part for GoogleCloudPaymentsResellerSubscriptionV1Amount {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptions cancel partners](PartnerSubscriptionCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequest {
    /// Optional. If true, Google will cancel the subscription immediately, and issue a prorated refund for the remainder of the billing cycle. Otherwise, Google defers the cancelation at renewal_time, and therefore, will not issue a refund.
    #[serde(rename="cancelImmediately")]
    
    pub cancel_immediately: Option<bool>,
    /// Specifies the reason for the cancellation.
    #[serde(rename="cancellationReason")]
    
    pub cancellation_reason: Option<GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum>,
}

impl client::RequestValue for GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptions cancel partners](PartnerSubscriptionCancelCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionResponse {
    /// The cancelled subscription resource.
    
    pub subscription: Option<GoogleCloudPaymentsResellerSubscriptionV1Subscription>,
}

impl client::ResponseResult for GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionResponse {}


/// Describes the length of a period of a time.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1Duration {
    /// number of duration units to be included.
    
    pub count: Option<i32>,
    /// The unit used for the duration
    
    pub unit: Option<GoogleCloudPaymentsResellerSubscriptionV1DurationUnitEnum>,
}

impl client::Part for GoogleCloudPaymentsResellerSubscriptionV1Duration {}


/// Partner request for entitling the previously provisioned subscription to an end user. The end user identity is inferred from the request OAuth context.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptions entitle partners](PartnerSubscriptionEntitleCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1EntitleSubscriptionRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudPaymentsResellerSubscriptionV1EntitleSubscriptionRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptions entitle partners](PartnerSubscriptionEntitleCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1EntitleSubscriptionResponse {
    /// The subscription that has user linked to it.
    
    pub subscription: Option<GoogleCloudPaymentsResellerSubscriptionV1Subscription>,
}

impl client::ResponseResult for GoogleCloudPaymentsResellerSubscriptionV1EntitleSubscriptionResponse {}


/// Request message for extending a Subscription resource. A new recurrence will be made based on the subscription schedule defined by the original product.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptions extend partners](PartnerSubscriptionExtendCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1ExtendSubscriptionRequest {
    /// Required. Specifies details of the extension. Currently, the duration of the extension must be exactly one billing cycle of the original subscription.
    
    pub extension: Option<GoogleCloudPaymentsResellerSubscriptionV1Extension>,
    /// Required. Restricted to 36 ASCII characters. A random UUID is recommended. The idempotency key for the request. The ID generation logic is controlled by the partner. request_id should be the same as on retries of the same request. A different request_id must be used for a extension of a different cycle. A random UUID is recommended.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for GoogleCloudPaymentsResellerSubscriptionV1ExtendSubscriptionRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptions extend partners](PartnerSubscriptionExtendCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1ExtendSubscriptionResponse {
    /// The time at which the subscription is expected to be extended, in ISO 8061 format. UTC timezone. Example, "cycleEndTime":"2019-08-31T17:28:54.564Z"
    #[serde(rename="cycleEndTime")]
    
    pub cycle_end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// End of the free trial period, in ISO 8061 format. UTC timezone. Example, "freeTrialEndTime":"2019-08-31T17:28:54.564Z" This time will be set the same as initial subscription creation time if no free trial period is offered to the partner.
    #[serde(rename="freeTrialEndTime")]
    
    pub free_trial_end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The time at which the subscription is expected to be renewed by Google - a new charge will be incurred and the service entitlement will be renewed. A non-immediate cancellation will take place at this time too, before which, the service entitlement for the end user will remain valid. UTC timezone in ISO 8061 format. For example: "2019-08-31T17:28:54.564Z"
    #[serde(rename="renewalTime")]
    
    pub renewal_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for GoogleCloudPaymentsResellerSubscriptionV1ExtendSubscriptionResponse {}


/// Describes the details of an extension request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1Extension {
    /// Specifies the period of access the subscription should grant.
    
    pub duration: Option<GoogleCloudPaymentsResellerSubscriptionV1Duration>,
    /// Required. Identifier of the end-user in partner’s system.
    #[serde(rename="partnerUserToken")]
    
    pub partner_user_token: Option<String>,
}

impl client::Part for GoogleCloudPaymentsResellerSubscriptionV1Extension {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [promotions find eligible partners](PartnerPromotionFindEligibleCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1FindEligiblePromotionsRequest {
    /// Optional. Specifies the filters for the promotion results. The syntax is defined in https://google.aip.dev/160 with the following caveats: - Only the following features are supported: - Logical operator `AND` - Comparison operator `=` (no wildcards `*`) - Traversal operator `.` - Has operator `:` (no wildcards `*`) - Only the following fields are supported: - `applicableProducts` - `regionCodes` - `youtubePayload.partnerEligibilityId` - `youtubePayload.postalCode` - Unless explicitly mentioned above, other features are not supported. Example: `applicableProducts:partners/partner1/products/product1 AND regionCodes:US AND youtubePayload.postalCode=94043 AND youtubePayload.partnerEligibilityId=eligibility-id`
    
    pub filter: Option<String>,
    /// Optional. The maximum number of promotions to return. The service may return fewer than this value. If unspecified, at most 50 products will be returned. The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// Optional. A page token, received from a previous `ListPromotions` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListPromotions` must match the call that provided the page token.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
}

impl client::RequestValue for GoogleCloudPaymentsResellerSubscriptionV1FindEligiblePromotionsRequest {}


/// Response containing the found promotions for the current user.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [promotions find eligible partners](PartnerPromotionFindEligibleCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1FindEligiblePromotionsResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is empty, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The promotions for the current user.
    
    pub promotions: Option<Vec<GoogleCloudPaymentsResellerSubscriptionV1Promotion>>,
}

impl client::ResponseResult for GoogleCloudPaymentsResellerSubscriptionV1FindEligiblePromotionsResponse {}


/// Payload specific to Google One products.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayload {
    /// Campaign attributed to sales of this subscription.
    
    pub campaigns: Option<Vec<String>>,
    /// The type of offering the subscription was sold by the partner. e.g. VAS.
    
    pub offering: Option<GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOfferingEnum>,
    /// The type of sales channel through which the subscription was sold.
    #[serde(rename="salesChannel")]
    
    pub sales_channel: Option<GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannelEnum>,
    /// The identifier for the partner store where the subscription was sold.
    #[serde(rename="storeId")]
    
    pub store_id: Option<String>,
}

impl client::Part for GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayload {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [products list partners](PartnerProductListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1ListProductsResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is empty, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The products for the specified partner.
    
    pub products: Option<Vec<GoogleCloudPaymentsResellerSubscriptionV1Product>>,
}

impl client::ResponseResult for GoogleCloudPaymentsResellerSubscriptionV1ListProductsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [promotions list partners](PartnerPromotionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1ListPromotionsResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is empty, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The promotions for the specified partner.
    
    pub promotions: Option<Vec<GoogleCloudPaymentsResellerSubscriptionV1Promotion>>,
}

impl client::ResponseResult for GoogleCloudPaymentsResellerSubscriptionV1ListPromotionsResponse {}


/// Describes a location of an end user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1Location {
    /// The postal code this location refers to. Ex. "94043"
    #[serde(rename="postalCode")]
    
    pub postal_code: Option<String>,
    /// 2-letter ISO region code for current content region. Ex. “US” Please refers to: https://en.wikipedia.org/wiki/ISO_3166-1
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
}

impl client::Part for GoogleCloudPaymentsResellerSubscriptionV1Location {}


/// A Product resource that defines a subscription service that can be resold.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1Product {
    /// Output only. Response only. Resource name of the product. It will have the format of "partners/{partner_id}/products/{product_id}"
    
    pub name: Option<String>,
    /// Output only. Price configs for the product in the available regions.
    #[serde(rename="priceConfigs")]
    
    pub price_configs: Option<Vec<GoogleCloudPaymentsResellerSubscriptionV1ProductPriceConfig>>,
    /// Output only. 2-letter ISO region code where the product is available in. Ex. "US" Please refers to: https://en.wikipedia.org/wiki/ISO_3166-1
    #[serde(rename="regionCodes")]
    
    pub region_codes: Option<Vec<String>>,
    /// Output only. Specifies the length of the billing cycle of the subscription.
    #[serde(rename="subscriptionBillingCycleDuration")]
    
    pub subscription_billing_cycle_duration: Option<GoogleCloudPaymentsResellerSubscriptionV1Duration>,
    /// Output only. Localized human readable name of the product.
    
    pub titles: Option<Vec<GoogleTypeLocalizedText>>,
}

impl client::Part for GoogleCloudPaymentsResellerSubscriptionV1Product {}


/// Specifies product specific payload.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1ProductPayload {
    /// Payload specific to Google One products.
    #[serde(rename="googleOnePayload")]
    
    pub google_one_payload: Option<GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayload>,
    /// Payload specific to Youtube products.
    #[serde(rename="youtubePayload")]
    
    pub youtube_payload: Option<GoogleCloudPaymentsResellerSubscriptionV1YoutubePayload>,
}

impl client::Part for GoogleCloudPaymentsResellerSubscriptionV1ProductPayload {}


/// Configs the prices in an available region.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1ProductPriceConfig {
    /// Output only. The price in the region.
    
    pub amount: Option<GoogleCloudPaymentsResellerSubscriptionV1Amount>,
    /// Output only. 2-letter ISO region code where the product is available in. Ex. "US".
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
}

impl client::Part for GoogleCloudPaymentsResellerSubscriptionV1ProductPriceConfig {}


/// A Promotion resource that defines a promotion for a subscription that can be resold.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1Promotion {
    /// Output only. The product ids this promotion can be applied to.
    #[serde(rename="applicableProducts")]
    
    pub applicable_products: Option<Vec<String>>,
    /// Optional. Specifies the end time (exclusive) of the period that the promotion is available in. If unset, the promotion is available indefinitely.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Specifies the duration of the free trial of the subscription when promotion_type is PROMOTION_TYPE_FREE_TRIAL
    #[serde(rename="freeTrialDuration")]
    
    pub free_trial_duration: Option<GoogleCloudPaymentsResellerSubscriptionV1Duration>,
    /// Optional. Specifies the introductory pricing details when the promotion_type is PROMOTION_TYPE_INTRODUCTORY_PRICING.
    #[serde(rename="introductoryPricingDetails")]
    
    pub introductory_pricing_details: Option<GoogleCloudPaymentsResellerSubscriptionV1PromotionIntroductoryPricingDetails>,
    /// Output only. Response only. Resource name of the subscription promotion. It will have the format of "partners/{partner_id}/promotion/{promotion_id}"
    
    pub name: Option<String>,
    /// Output only. Output Only. Specifies the type of the promotion.
    #[serde(rename="promotionType")]
    
    pub promotion_type: Option<GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionTypeEnum>,
    /// Output only. 2-letter ISO region code where the promotion is available in. Ex. "US" Please refers to: https://en.wikipedia.org/wiki/ISO_3166-1
    #[serde(rename="regionCodes")]
    
    pub region_codes: Option<Vec<String>>,
    /// Optional. Specifies the start time (inclusive) of the period that the promotion is available in.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Localized human readable name of the promotion.
    
    pub titles: Option<Vec<GoogleTypeLocalizedText>>,
}

impl client::Part for GoogleCloudPaymentsResellerSubscriptionV1Promotion {}


/// The details of a introductory pricing promotion.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1PromotionIntroductoryPricingDetails {
    /// Specifies the introductory pricing periods.
    #[serde(rename="introductoryPricingSpecs")]
    
    pub introductory_pricing_specs: Option<Vec<GoogleCloudPaymentsResellerSubscriptionV1PromotionIntroductoryPricingDetailsIntroductoryPricingSpec>>,
}

impl client::Part for GoogleCloudPaymentsResellerSubscriptionV1PromotionIntroductoryPricingDetails {}


/// The duration of an introductory pricing promotion.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1PromotionIntroductoryPricingDetailsIntroductoryPricingSpec {
    /// Output only. The discount amount. The value is positive.
    #[serde(rename="discountAmount")]
    
    pub discount_amount: Option<GoogleCloudPaymentsResellerSubscriptionV1Amount>,
    /// Output only. The discount percentage in micros. For example, 50,000 represents 5%.
    #[serde(rename="discountRatioMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub discount_ratio_micros: Option<i64>,
    /// Output only. Output Only. The duration of an introductory offer in billing cycles.
    #[serde(rename="recurrenceCount")]
    
    pub recurrence_count: Option<i32>,
    /// Output only. 2-letter ISO region code where the product is available in. Ex. "US".
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
}

impl client::Part for GoogleCloudPaymentsResellerSubscriptionV1PromotionIntroductoryPricingDetailsIntroductoryPricingSpec {}


/// A description of what time period or moment in time the product or service is being delivered over.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1ServicePeriod {
    /// Optional. The end time of the service period. Time is exclusive.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The start time of the service period. Time is inclusive.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleCloudPaymentsResellerSubscriptionV1ServicePeriod {}


/// A Subscription resource managed by 3P Partners.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptions create partners](PartnerSubscriptionCreateCall) (request|response)
/// * [subscriptions get partners](PartnerSubscriptionGetCall) (response)
/// * [subscriptions provision partners](PartnerSubscriptionProvisionCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1Subscription {
    /// Output only. Describes the details of a cancelled subscription. Only applicable to subscription of state `STATE_CANCELLED`.
    #[serde(rename="cancellationDetails")]
    
    pub cancellation_details: Option<GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetails>,
    /// Output only. System generated timestamp when the subscription is created. UTC timezone.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The time at which the subscription is expected to be extended, in ISO 8061 format. UTC timezone. For example: "2019-08-31T17:28:54.564Z"
    #[serde(rename="cycleEndTime")]
    
    pub cycle_end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Indicates if the subscription is entitled to the end user.
    #[serde(rename="endUserEntitled")]
    
    pub end_user_entitled: Option<bool>,
    /// Output only. End of the free trial period, in ISO 8061 format. For example, "2019-08-31T17:28:54.564Z". It will be set the same as createTime if no free trial promotion is specified.
    #[serde(rename="freeTrialEndTime")]
    
    pub free_trial_end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The line items of the subscription.
    #[serde(rename="lineItems")]
    
    pub line_items: Option<Vec<GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItem>>,
    /// Optional. Resource name of the subscription. It will have the format of "partners/{partner_id}/subscriptions/{subscription_id}". This is available for authorizeAddon, but otherwise is response only.
    
    pub name: Option<String>,
    /// Required. Identifier of the end-user in partner’s system. The value is restricted to 63 ASCII characters at the maximum.
    #[serde(rename="partnerUserToken")]
    
    pub partner_user_token: Option<String>,
    /// Output only. Describes the processing state of the subscription. See more details at [the lifecycle of a subscription](https://developers.google.com/payments/reseller/subscription/reference/index/Receive.Notifications#payments-subscription-lifecycle).
    #[serde(rename="processingState")]
    
    pub processing_state: Option<GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingStateEnum>,
    /// Required. Deprecated: consider using `line_items` as the input. Required. Resource name that identifies the purchased products. The format will be 'partners/{partner_id}/products/{product_id}'.
    
    pub products: Option<Vec<String>>,
    /// Optional. Subscription-level promotions. Only free trial is supported on this level. It determines the first renewal time of the subscription to be the end of the free trial period. Specify the promotion resource name only when used as input.
    #[serde(rename="promotionSpecs")]
    
    pub promotion_specs: Option<Vec<GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpec>>,
    /// Optional. Deprecated: consider using the top-level `promotion_specs` as the input. Optional. Resource name that identifies one or more promotions that can be applied on the product. A typical promotion for a subscription is Free trial. The format will be 'partners/{partner_id}/promotions/{promotion_id}'.
    
    pub promotions: Option<Vec<String>>,
    /// Output only. The place where partners should redirect the end-user to after creation. This field might also be populated when creation failed. However, Partners should always prepare a default URL to redirect the user in case this field is empty.
    #[serde(rename="redirectUri")]
    
    pub redirect_uri: Option<String>,
    /// Output only. The time at which the subscription is expected to be renewed by Google - a new charge will be incurred and the service entitlement will be renewed. A non-immediate cancellation will take place at this time too, before which, the service entitlement for the end user will remain valid. UTC timezone in ISO 8061 format. For example: "2019-08-31T17:28:54.564Z"
    #[serde(rename="renewalTime")]
    
    pub renewal_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The location that the service is provided as indicated by the partner.
    #[serde(rename="serviceLocation")]
    
    pub service_location: Option<GoogleCloudPaymentsResellerSubscriptionV1Location>,
    /// Output only. Describes the state of the subscription. See more details at [the lifecycle of a subscription](https://developers.google.com/payments/reseller/subscription/reference/index/Receive.Notifications#payments-subscription-lifecycle).
    
    pub state: Option<GoogleCloudPaymentsResellerSubscriptionV1SubscriptionStateEnum>,
    /// Output only. System generated timestamp when the subscription is most recently updated. UTC timezone.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Details about the previous subscription that this new subscription upgrades/downgrades from. Only populated if this subscription is an upgrade/downgrade from another subscription.
    #[serde(rename="upgradeDowngradeDetails")]
    
    pub upgrade_downgrade_details: Option<GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetails>,
}

impl client::RequestValue for GoogleCloudPaymentsResellerSubscriptionV1Subscription {}
impl client::ResponseResult for GoogleCloudPaymentsResellerSubscriptionV1Subscription {}


/// Describes the details of a cancelled or cancelling subscription.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetails {
    /// The reason of the cancellation.
    
    pub reason: Option<GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum>,
}

impl client::Part for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetails {}


/// Individual line item definition of a subscription.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItem {
    /// Output only. Description of this line item.
    
    pub description: Option<String>,
    /// Output only. It is set only if the line item has its own free trial applied. End time of the line item free trial period, in ISO 8061 format. For example, "2019-08-31T17:28:54.564Z". It will be set the same as createTime if no free trial promotion is specified.
    #[serde(rename="lineItemFreeTrialEndTime")]
    
    pub line_item_free_trial_end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. The promotions applied on the line item. It can be: - a free trial promotion, which overrides the subscription-level free trial promotion. - an introductory pricing promotion. When used as input in Create or Provision API, specify its resource name only.
    #[serde(rename="lineItemPromotionSpecs")]
    
    pub line_item_promotion_specs: Option<Vec<GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpec>>,
    /// Output only. Details only set for a ONE_TIME recurrence line item.
    #[serde(rename="oneTimeRecurrenceDetails")]
    
    pub one_time_recurrence_details: Option<GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemOneTimeRecurrenceDetails>,
    /// Required. Product resource name that identifies one the line item The format is 'partners/{partner_id}/products/{product_id}'.
    
    pub product: Option<String>,
    /// Optional. Product specific payload for this line item.
    #[serde(rename="productPayload")]
    
    pub product_payload: Option<GoogleCloudPaymentsResellerSubscriptionV1ProductPayload>,
    /// Output only. The recurrence type of the line item.
    #[serde(rename="recurrenceType")]
    
    pub recurrence_type: Option<GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceTypeEnum>,
    /// Output only. The state of the line item.
    
    pub state: Option<GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemStateEnum>,
}

impl client::Part for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItem {}


/// Details for a ONE_TIME recurrence line item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemOneTimeRecurrenceDetails {
    /// The service period of the ONE_TIME line item.
    #[serde(rename="servicePeriod")]
    
    pub service_period: Option<GoogleCloudPaymentsResellerSubscriptionV1ServicePeriod>,
}

impl client::Part for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemOneTimeRecurrenceDetails {}


/// Describes the spec for one promotion.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpec {
    /// Output only. The duration of the free trial if the promotion is of type FREE_TRIAL.
    #[serde(rename="freeTrialDuration")]
    
    pub free_trial_duration: Option<GoogleCloudPaymentsResellerSubscriptionV1Duration>,
    /// Output only. The details of the introductory pricing spec if the promotion is of type INTRODUCTORY_PRICING.
    #[serde(rename="introductoryPricingDetails")]
    
    pub introductory_pricing_details: Option<GoogleCloudPaymentsResellerSubscriptionV1PromotionIntroductoryPricingDetails>,
    /// Required. Promotion resource name that identifies a promotion. The format is 'partners/{partner_id}/promotions/{promotion_id}'.
    
    pub promotion: Option<String>,
    /// Output only. The type of the promotion for the spec.
    #[serde(rename="type")]
    
    pub type_: Option<GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecTypeEnum>,
}

impl client::Part for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpec {}


/// Details about the previous subscription that this new subscription upgrades/downgrades from.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetails {
    /// Required. Specifies the billing cycle spec for the new upgraded/downgraded subscription.
    #[serde(rename="billingCycleSpec")]
    
    pub billing_cycle_spec: Option<GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailBillingCycleSpecEnum>,
    /// Required. The previous subscription id to be replaced. This is not the full resource name, use the subscription_id segment only.
    #[serde(rename="previousSubscriptionId")]
    
    pub previous_subscription_id: Option<String>,
}

impl client::Part for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetails {}


/// Request to revoke a cancellation request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptions undo cancel partners](PartnerSubscriptionUndoCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1UndoCancelSubscriptionRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudPaymentsResellerSubscriptionV1UndoCancelSubscriptionRequest {}


/// Response that contains the updated subscription resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptions undo cancel partners](PartnerSubscriptionUndoCancelCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1UndoCancelSubscriptionResponse {
    /// The updated subscription resource.
    
    pub subscription: Option<GoogleCloudPaymentsResellerSubscriptionV1Subscription>,
}

impl client::ResponseResult for GoogleCloudPaymentsResellerSubscriptionV1UndoCancelSubscriptionResponse {}


/// Payload specific to Youtube products.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1YoutubePayload {
    /// The list of eligibility_ids which are applicable for the line item.
    #[serde(rename="partnerEligibilityIds")]
    
    pub partner_eligibility_ids: Option<Vec<String>>,
}

impl client::Part for GoogleCloudPaymentsResellerSubscriptionV1YoutubePayload {}


/// Localized variant of a text in a particular language.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeLocalizedText {
    /// The text's BCP-47 language code, such as "en-US" or "sr-Latn". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Localized string in the language corresponding to `language_code' below.
    
    pub text: Option<String>,
}

impl client::Part for GoogleTypeLocalizedText {}


