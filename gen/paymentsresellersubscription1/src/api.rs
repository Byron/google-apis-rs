use std::collections::HashMap;
use std::cell::RefCell;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;

use crate::client;

// ##############
// UTILITIES ###
// ############




// ########
// HUB ###
// ######

/// Central instance to access all PaymentsResellerSubscription related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_paymentsresellersubscription1 as paymentsresellersubscription1;
/// use paymentsresellersubscription1::api::GoogleCloudPaymentsResellerSubscriptionV1Subscription;
/// use paymentsresellersubscription1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use paymentsresellersubscription1::{PaymentsResellerSubscription, oauth2, hyper, hyper_rustls};
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
/// let mut hub = PaymentsResellerSubscription::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudPaymentsResellerSubscriptionV1Subscription::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.partners().subscriptions_create(req, "parent")
///              .subscription_id("At")
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
pub struct PaymentsResellerSubscription<> {
    pub client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>,
    pub auth: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, > client::Hub for PaymentsResellerSubscription<> {}

impl<'a, > PaymentsResellerSubscription<> {

    pub fn new(client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>, authenticator: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>) -> PaymentsResellerSubscription<> {
        PaymentsResellerSubscription {
            client,
            auth: authenticator,
            _user_agent: "google-api-rust-client/3.0.2".to_string(),
            _base_url: "https://paymentsresellersubscription.googleapis.com/".to_string(),
            _root_url: "https://paymentsresellersubscription.googleapis.com/".to_string(),
        }
    }

    pub fn partners(&'a self) -> PartnerMethods<'a> {
        PartnerMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/3.0.2`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://paymentsresellersubscription.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://paymentsresellersubscription.googleapis.com/`.
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
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [subscriptions cancel partners](PartnerSubscriptionCancelCall) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequest {
    /// Optional. If true, the subscription will be cancelled immediately. Otherwise, the subscription will be cancelled at renewal_time, and therefore no prorated refund will be issued for the rest of the cycle.
    #[serde(rename="cancelImmediately")]
    pub cancel_immediately: Option<bool>,
    /// Specifies the reason for the cancellation.
    #[serde(rename="cancellationReason")]
    pub cancellation_reason: Option<String>,
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
/// 
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1Duration {
    /// number of duration units to be included.
    pub count: Option<i32>,
    /// The unit used for the duration
    pub unit: Option<String>,
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
/// 
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
/// 
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
/// 
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
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1ExtendSubscriptionResponse {
    /// The time at which the subscription is expected to be extended, in ISO 8061 format. UTC timezone. Example, "cycleEndTime":"2019-08-31T17:28:54.564Z"
    #[serde(rename="cycleEndTime")]
    pub cycle_end_time: Option<String>,
    /// End of the free trial period, in ISO 8061 format. UTC timezone. Example, "freeTrialEndTime":"2019-08-31T17:28:54.564Z" This time will be set the same as initial subscription creation time if no free trial period is offered to the partner.
    #[serde(rename="freeTrialEndTime")]
    pub free_trial_end_time: Option<String>,
    /// Output only. The time at which the subscription is expected to be renewed by Google - a new charge will be incurred and the service entitlement will be renewed. A non-immediate cancellation will take place at this time too, before which, the service entitlement for the end user will remain valid. UTC timezone in ISO 8061 format. For example: "2019-08-31T17:28:54.564Z"
    #[serde(rename="renewalTime")]
    pub renewal_time: Option<String>,
}

impl client::ResponseResult for GoogleCloudPaymentsResellerSubscriptionV1ExtendSubscriptionResponse {}


/// Describes the details of an extension request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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
/// * [products list partners](PartnerProductListCall) (response)
/// 
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
/// 
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1Product {
    /// Output only. Response only. Resource name of the subscription. It will have the format of "partners/{partner_id}/products/{product_id}"
    pub name: Option<String>,
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


/// A Promotion resource that defines a promotion for a subscription that can be resold.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1Promotion {
    /// Output only. The product ids this promotion can be applied to.
    #[serde(rename="applicableProducts")]
    pub applicable_products: Option<Vec<String>>,
    /// Optional. Specifies the end time (exclusive) of the period that the promotion is available in. If unset, the promotion is available indefinitely.
    #[serde(rename="endTime")]
    pub end_time: Option<String>,
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
    pub promotion_type: Option<String>,
    /// Output only. 2-letter ISO region code where the promotion is available in. Ex. "US" Please refers to: https://en.wikipedia.org/wiki/ISO_3166-1
    #[serde(rename="regionCodes")]
    pub region_codes: Option<Vec<String>>,
    /// Optional. Specifies the start time (inclusive) of the period that the promotion is available in.
    #[serde(rename="startTime")]
    pub start_time: Option<String>,
    /// Output only. Localized human readable name of the promotion.
    pub titles: Option<Vec<GoogleTypeLocalizedText>>,
}

impl client::Part for GoogleCloudPaymentsResellerSubscriptionV1Promotion {}


/// The details of a introductory pricing promotion.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1PromotionIntroductoryPricingDetailsIntroductoryPricingSpec {
    /// Output only. Output Only. The duration of an introductory offer in billing cycles.
    #[serde(rename="recurrenceCount")]
    pub recurrence_count: Option<i32>,
}

impl client::Part for GoogleCloudPaymentsResellerSubscriptionV1PromotionIntroductoryPricingDetailsIntroductoryPricingSpec {}


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
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1Subscription {
    /// Output only. Describes the details of a cancelled subscription. Only applicable to subscription of state `STATE_CANCELLED`.
    #[serde(rename="cancellationDetails")]
    pub cancellation_details: Option<GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetails>,
    /// Output only. System generated timestamp when the subscription is created. UTC timezone.
    #[serde(rename="createTime")]
    pub create_time: Option<String>,
    /// Output only. The time at which the subscription is expected to be extended, in ISO 8061 format. UTC timezone. For example: "2019-08-31T17:28:54.564Z"
    #[serde(rename="cycleEndTime")]
    pub cycle_end_time: Option<String>,
    /// Output only. Indicates if the subscription is entitled to the end user.
    #[serde(rename="endUserEntitled")]
    pub end_user_entitled: Option<bool>,
    /// Output only. End of the free trial period, in ISO 8061 format. For example, "2019-08-31T17:28:54.564Z". It will be set the same as createTime if no free trial promotion is specified.
    #[serde(rename="freeTrialEndTime")]
    pub free_trial_end_time: Option<String>,
    /// Output only. Response only. Resource name of the subscription. It will have the format of "partners/{partner_id}/subscriptions/{subscription_id}"
    pub name: Option<String>,
    /// Required. Identifier of the end-user in partner’s system. The value is restricted to 63 ASCII characters at the maximum.
    #[serde(rename="partnerUserToken")]
    pub partner_user_token: Option<String>,
    /// Output only. Describes the processing state of the subscription. See more details at [the lifecycle of a subscription](/payments/reseller/subscription/reference/index/Receive.Notifications#payments-subscription-lifecycle).
    #[serde(rename="processingState")]
    pub processing_state: Option<String>,
    /// Required. Resource name that identifies one or more subscription products. The format will be 'partners/{partner_id}/products/{product_id}'.
    pub products: Option<Vec<String>>,
    /// Optional. Resource name that identifies one or more promotions that can be applied on the product. A typical promotion for a subscription is Free trial. The format will be 'partners/{partner_id}/promotions/{promotion_id}'.
    pub promotions: Option<Vec<String>>,
    /// Output only. The place where partners should redirect the end-user to after creation. This field might also be populated when creation failed. However, Partners should always prepare a default URL to redirect the user in case this field is empty.
    #[serde(rename="redirectUri")]
    pub redirect_uri: Option<String>,
    /// Output only. The time at which the subscription is expected to be renewed by Google - a new charge will be incurred and the service entitlement will be renewed. A non-immediate cancellation will take place at this time too, before which, the service entitlement for the end user will remain valid. UTC timezone in ISO 8061 format. For example: "2019-08-31T17:28:54.564Z"
    #[serde(rename="renewalTime")]
    pub renewal_time: Option<String>,
    /// Required. The location that the service is provided as indicated by the partner.
    #[serde(rename="serviceLocation")]
    pub service_location: Option<GoogleCloudPaymentsResellerSubscriptionV1Location>,
    /// Output only. Describes the state of the subscription. See more details at [the lifecycle of a subscription](/payments/reseller/subscription/reference/index/Receive.Notifications#payments-subscription-lifecycle).
    pub state: Option<String>,
    /// Output only. System generated timestamp when the subscription is most recently updated. UTC timezone.
    #[serde(rename="updateTime")]
    pub update_time: Option<String>,
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetails {
    /// The reason of the cancellation.
    pub reason: Option<String>,
}

impl client::Part for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetails {}


/// Details about the previous subscription that this new subscription upgrades/downgrades from.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetails {
    /// Required. Specifies the billing cycle spec for the new upgraded/downgraded subscription.
    #[serde(rename="billingCycleSpec")]
    pub billing_cycle_spec: Option<String>,
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
/// 
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
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPaymentsResellerSubscriptionV1UndoCancelSubscriptionResponse {
    /// The updated subscription resource.
    pub subscription: Option<GoogleCloudPaymentsResellerSubscriptionV1Subscription>,
}

impl client::ResponseResult for GoogleCloudPaymentsResellerSubscriptionV1UndoCancelSubscriptionResponse {}


/// Localized variant of a text in a particular language.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeLocalizedText {
    /// The text's BCP-47 language code, such as "en-US" or "sr-Latn". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
    #[serde(rename="languageCode")]
    pub language_code: Option<String>,
    /// Localized string in the language corresponding to `language_code' below.
    pub text: Option<String>,
}

impl client::Part for GoogleTypeLocalizedText {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *partner* resources.
/// It is not used directly, but through the `PaymentsResellerSubscription` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_paymentsresellersubscription1 as paymentsresellersubscription1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use paymentsresellersubscription1::{PaymentsResellerSubscription, oauth2, hyper, hyper_rustls};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PaymentsResellerSubscription::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `products_list(...)`, `promotions_list(...)`, `subscriptions_cancel(...)`, `subscriptions_create(...)`, `subscriptions_entitle(...)`, `subscriptions_extend(...)`, `subscriptions_get(...)`, `subscriptions_provision(...)` and `subscriptions_undo_cancel(...)`
/// // to build up your call.
/// let rb = hub.partners();
/// # }
/// ```
pub struct PartnerMethods<'a>
    where  {

    hub: &'a PaymentsResellerSubscription<>,
}

impl<'a> client::MethodsBuilder for PartnerMethods<'a> {}

impl<'a> PartnerMethods<'a> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Used by partners to list products that can be resold to their customers. It should be called directly by the partner using service accounts.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, the partner that can resell. Format: partners/{partner}
    pub fn products_list(&self, parent: &str) -> PartnerProductListCall<'a> {
        PartnerProductListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Used by partners to list promotions, such as free trial, that can be applied on subscriptions. It should be called directly by the partner using service accounts.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, the partner that can resell. Format: partners/{partner}
    pub fn promotions_list(&self, parent: &str) -> PartnerPromotionListCall<'a> {
        PartnerPromotionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Used by partners to cancel a subscription service either immediately or by the end of the current billing cycle for their customers. It should be called directly by the partner using service accounts.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the subscription resource to be cancelled. It will have the format of "partners/{partner_id}/subscriptions/{subscription_id}"
    pub fn subscriptions_cancel(&self, request: GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequest, name: &str) -> PartnerSubscriptionCancelCall<'a> {
        PartnerSubscriptionCancelCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Used by partners to create a subscription for their customers. The created subscription is associated with the end user inferred from the end user credentials. This API must be authorized by the end user using OAuth.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource name, which is the identifier of the partner. It will have the format of "partners/{partner_id}".
    pub fn subscriptions_create(&self, request: GoogleCloudPaymentsResellerSubscriptionV1Subscription, parent: &str) -> PartnerSubscriptionCreateCall<'a> {
        PartnerSubscriptionCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _subscription_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Used by partners to entitle a previously provisioned subscription to the current end user. The end user identity is inferred from the authorized credential of the request. This API must be authorized by the end user using OAuth.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the subscription resource that is entitled to the current end user. It will have the format of "partners/{partner_id}/subscriptions/{subscription_id}"
    pub fn subscriptions_entitle(&self, request: GoogleCloudPaymentsResellerSubscriptionV1EntitleSubscriptionRequest, name: &str) -> PartnerSubscriptionEntitleCall<'a> {
        PartnerSubscriptionEntitleCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Used by partners to extend a subscription service for their customers on an ongoing basis for the subscription to remain active and renewable. It should be called directly by the partner using service accounts.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the subscription resource to be extended. It will have the format of "partners/{partner_id}/subscriptions/{subscription_id}".
    pub fn subscriptions_extend(&self, request: GoogleCloudPaymentsResellerSubscriptionV1ExtendSubscriptionRequest, name: &str) -> PartnerSubscriptionExtendCall<'a> {
        PartnerSubscriptionExtendCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Used by partners to get a subscription by id. It should be called directly by the partner using service accounts.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the subscription resource to retrieve. It will have the format of "partners/{partner_id}/subscriptions/{subscription_id}"
    pub fn subscriptions_get(&self, name: &str) -> PartnerSubscriptionGetCall<'a> {
        PartnerSubscriptionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Used by partners to provision a subscription for their customers. This creates a subscription without associating it with the end user account. EntitleSubscription must be called separately using OAuth in order for the end user account to be associated with the subscription. It should be called directly by the partner using service accounts.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource name, which is the identifier of the partner. It will have the format of "partners/{partner_id}".
    pub fn subscriptions_provision(&self, request: GoogleCloudPaymentsResellerSubscriptionV1Subscription, parent: &str) -> PartnerSubscriptionProvisionCall<'a> {
        PartnerSubscriptionProvisionCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _subscription_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Used by partners to revoke the pending cancellation of a subscription, which is currently in `STATE_CANCEL_AT_END_OF_CYCLE` state. If the subscription is already cancelled, the request will fail. It should be called directly by the partner using service accounts.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the subscription resource whose pending cancellation needs to be undone. It will have the format of "partners/{partner_id}/subscriptions/{subscription_id}"
    pub fn subscriptions_undo_cancel(&self, request: GoogleCloudPaymentsResellerSubscriptionV1UndoCancelSubscriptionRequest, name: &str) -> PartnerSubscriptionUndoCancelCall<'a> {
        PartnerSubscriptionUndoCancelCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Used by partners to list products that can be resold to their customers. It should be called directly by the partner using service accounts.
///
/// A builder for the *products.list* method supported by a *partner* resource.
/// It is not used directly, but through a `PartnerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_paymentsresellersubscription1 as paymentsresellersubscription1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use paymentsresellersubscription1::{PaymentsResellerSubscription, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PaymentsResellerSubscription::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.partners().products_list("parent")
///              .page_token("sed")
///              .page_size(-2)
///              .doit().await;
/// # }
/// ```
pub struct PartnerProductListCall<'a>
    where  {

    hub: &'a PaymentsResellerSubscription<>,
    _parent: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a> client::CallBuilder for PartnerProductListCall<'a> {}

impl<'a> PartnerProductListCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudPaymentsResellerSubscriptionV1ListProductsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "paymentsresellersubscription.partners.products.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        for &field in ["alt", "parent", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}/products";
        
        let key = dlg.api_key();
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["parent"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone());


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
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
                            sleep(d);
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


    /// Required. The parent, the partner that can resell. Format: partners/{partner}
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> PartnerProductListCall<'a> {
        self._parent = new_value.to_string();
        self
    }
    /// Optional. A page token, received from a previous `ListProducts` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListProducts` must match the call that provided the page token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> PartnerProductListCall<'a> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Optional. The maximum number of products to return. The service may return fewer than this value. If unspecified, at most 50 products will be returned. The maximum value is 1000; values above 1000 will be coerced to 1000.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> PartnerProductListCall<'a> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PartnerProductListCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PartnerProductListCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Used by partners to list promotions, such as free trial, that can be applied on subscriptions. It should be called directly by the partner using service accounts.
///
/// A builder for the *promotions.list* method supported by a *partner* resource.
/// It is not used directly, but through a `PartnerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_paymentsresellersubscription1 as paymentsresellersubscription1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use paymentsresellersubscription1::{PaymentsResellerSubscription, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PaymentsResellerSubscription::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.partners().promotions_list("parent")
///              .page_token("amet.")
///              .page_size(-20)
///              .filter("ipsum")
///              .doit().await;
/// # }
/// ```
pub struct PartnerPromotionListCall<'a>
    where  {

    hub: &'a PaymentsResellerSubscription<>,
    _parent: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _filter: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a> client::CallBuilder for PartnerPromotionListCall<'a> {}

impl<'a> PartnerPromotionListCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudPaymentsResellerSubscriptionV1ListPromotionsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "paymentsresellersubscription.partners.promotions.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._filter {
            params.push(("filter", value.to_string()));
        }
        for &field in ["alt", "parent", "pageToken", "pageSize", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}/promotions";
        
        let key = dlg.api_key();
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["parent"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone());


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
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
                            sleep(d);
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


    /// Required. The parent, the partner that can resell. Format: partners/{partner}
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> PartnerPromotionListCall<'a> {
        self._parent = new_value.to_string();
        self
    }
    /// Optional. A page token, received from a previous `ListPromotions` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListPromotions` must match the call that provided the page token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> PartnerPromotionListCall<'a> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Optional. The maximum number of promotions to return. The service may return fewer than this value. If unspecified, at most 50 products will be returned. The maximum value is 1000; values above 1000 will be coerced to 1000.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> PartnerPromotionListCall<'a> {
        self._page_size = Some(new_value);
        self
    }
    /// Optional. Specifies the filters for the promotion results. The syntax defined in the EBNF grammar: https://google.aip.dev/assets/misc/ebnf-filtering.txt. Examples: - applicable_products: "sku1" - region_codes: "US" - applicable_products: "sku1" AND region_codes: "US"
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> PartnerPromotionListCall<'a> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PartnerPromotionListCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PartnerPromotionListCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Used by partners to cancel a subscription service either immediately or by the end of the current billing cycle for their customers. It should be called directly by the partner using service accounts.
///
/// A builder for the *subscriptions.cancel* method supported by a *partner* resource.
/// It is not used directly, but through a `PartnerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_paymentsresellersubscription1 as paymentsresellersubscription1;
/// use paymentsresellersubscription1::api::GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use paymentsresellersubscription1::{PaymentsResellerSubscription, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PaymentsResellerSubscription::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.partners().subscriptions_cancel(req, "name")
///              .doit().await;
/// # }
/// ```
pub struct PartnerSubscriptionCancelCall<'a>
    where  {

    hub: &'a PaymentsResellerSubscription<>,
    _request: GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequest,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a> client::CallBuilder for PartnerSubscriptionCancelCall<'a> {}

impl<'a> PartnerSubscriptionCancelCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "paymentsresellersubscription.partners.subscriptions.cancel",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+name}:cancel";
        
        let key = dlg.api_key();
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["name"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone());


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
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
                            sleep(d);
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
    pub fn request(mut self, new_value: GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequest) -> PartnerSubscriptionCancelCall<'a> {
        self._request = new_value;
        self
    }
    /// Required. The name of the subscription resource to be cancelled. It will have the format of "partners/{partner_id}/subscriptions/{subscription_id}"
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> PartnerSubscriptionCancelCall<'a> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PartnerSubscriptionCancelCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PartnerSubscriptionCancelCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Used by partners to create a subscription for their customers. The created subscription is associated with the end user inferred from the end user credentials. This API must be authorized by the end user using OAuth.
///
/// A builder for the *subscriptions.create* method supported by a *partner* resource.
/// It is not used directly, but through a `PartnerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_paymentsresellersubscription1 as paymentsresellersubscription1;
/// use paymentsresellersubscription1::api::GoogleCloudPaymentsResellerSubscriptionV1Subscription;
/// # async fn dox() {
/// # use std::default::Default;
/// # use paymentsresellersubscription1::{PaymentsResellerSubscription, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PaymentsResellerSubscription::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudPaymentsResellerSubscriptionV1Subscription::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.partners().subscriptions_create(req, "parent")
///              .subscription_id("gubergren")
///              .doit().await;
/// # }
/// ```
pub struct PartnerSubscriptionCreateCall<'a>
    where  {

    hub: &'a PaymentsResellerSubscription<>,
    _request: GoogleCloudPaymentsResellerSubscriptionV1Subscription,
    _parent: String,
    _subscription_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a> client::CallBuilder for PartnerSubscriptionCreateCall<'a> {}

impl<'a> PartnerSubscriptionCreateCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudPaymentsResellerSubscriptionV1Subscription)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "paymentsresellersubscription.partners.subscriptions.create",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        if let Some(value) = self._subscription_id {
            params.push(("subscriptionId", value.to_string()));
        }
        for &field in ["alt", "parent", "subscriptionId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}/subscriptions";
        
        let key = dlg.api_key();
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["parent"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone());


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
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
                            sleep(d);
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
    pub fn request(mut self, new_value: GoogleCloudPaymentsResellerSubscriptionV1Subscription) -> PartnerSubscriptionCreateCall<'a> {
        self._request = new_value;
        self
    }
    /// Required. The parent resource name, which is the identifier of the partner. It will have the format of "partners/{partner_id}".
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> PartnerSubscriptionCreateCall<'a> {
        self._parent = new_value.to_string();
        self
    }
    /// Required. Identifies the subscription resource on the Partner side. The value is restricted to 63 ASCII characters at the maximum. If a subscription was previously created with the same subscription_id, we will directly return that one.
    ///
    /// Sets the *subscription id* query property to the given value.
    pub fn subscription_id(mut self, new_value: &str) -> PartnerSubscriptionCreateCall<'a> {
        self._subscription_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PartnerSubscriptionCreateCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PartnerSubscriptionCreateCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Used by partners to entitle a previously provisioned subscription to the current end user. The end user identity is inferred from the authorized credential of the request. This API must be authorized by the end user using OAuth.
///
/// A builder for the *subscriptions.entitle* method supported by a *partner* resource.
/// It is not used directly, but through a `PartnerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_paymentsresellersubscription1 as paymentsresellersubscription1;
/// use paymentsresellersubscription1::api::GoogleCloudPaymentsResellerSubscriptionV1EntitleSubscriptionRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use paymentsresellersubscription1::{PaymentsResellerSubscription, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PaymentsResellerSubscription::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudPaymentsResellerSubscriptionV1EntitleSubscriptionRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.partners().subscriptions_entitle(req, "name")
///              .doit().await;
/// # }
/// ```
pub struct PartnerSubscriptionEntitleCall<'a>
    where  {

    hub: &'a PaymentsResellerSubscription<>,
    _request: GoogleCloudPaymentsResellerSubscriptionV1EntitleSubscriptionRequest,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a> client::CallBuilder for PartnerSubscriptionEntitleCall<'a> {}

impl<'a> PartnerSubscriptionEntitleCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudPaymentsResellerSubscriptionV1EntitleSubscriptionResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "paymentsresellersubscription.partners.subscriptions.entitle",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+name}:entitle";
        
        let key = dlg.api_key();
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["name"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone());


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
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
                            sleep(d);
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
    pub fn request(mut self, new_value: GoogleCloudPaymentsResellerSubscriptionV1EntitleSubscriptionRequest) -> PartnerSubscriptionEntitleCall<'a> {
        self._request = new_value;
        self
    }
    /// Required. The name of the subscription resource that is entitled to the current end user. It will have the format of "partners/{partner_id}/subscriptions/{subscription_id}"
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> PartnerSubscriptionEntitleCall<'a> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PartnerSubscriptionEntitleCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PartnerSubscriptionEntitleCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Used by partners to extend a subscription service for their customers on an ongoing basis for the subscription to remain active and renewable. It should be called directly by the partner using service accounts.
///
/// A builder for the *subscriptions.extend* method supported by a *partner* resource.
/// It is not used directly, but through a `PartnerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_paymentsresellersubscription1 as paymentsresellersubscription1;
/// use paymentsresellersubscription1::api::GoogleCloudPaymentsResellerSubscriptionV1ExtendSubscriptionRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use paymentsresellersubscription1::{PaymentsResellerSubscription, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PaymentsResellerSubscription::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudPaymentsResellerSubscriptionV1ExtendSubscriptionRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.partners().subscriptions_extend(req, "name")
///              .doit().await;
/// # }
/// ```
pub struct PartnerSubscriptionExtendCall<'a>
    where  {

    hub: &'a PaymentsResellerSubscription<>,
    _request: GoogleCloudPaymentsResellerSubscriptionV1ExtendSubscriptionRequest,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a> client::CallBuilder for PartnerSubscriptionExtendCall<'a> {}

impl<'a> PartnerSubscriptionExtendCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudPaymentsResellerSubscriptionV1ExtendSubscriptionResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "paymentsresellersubscription.partners.subscriptions.extend",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+name}:extend";
        
        let key = dlg.api_key();
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["name"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone());


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
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
                            sleep(d);
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
    pub fn request(mut self, new_value: GoogleCloudPaymentsResellerSubscriptionV1ExtendSubscriptionRequest) -> PartnerSubscriptionExtendCall<'a> {
        self._request = new_value;
        self
    }
    /// Required. The name of the subscription resource to be extended. It will have the format of "partners/{partner_id}/subscriptions/{subscription_id}".
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> PartnerSubscriptionExtendCall<'a> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PartnerSubscriptionExtendCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PartnerSubscriptionExtendCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Used by partners to get a subscription by id. It should be called directly by the partner using service accounts.
///
/// A builder for the *subscriptions.get* method supported by a *partner* resource.
/// It is not used directly, but through a `PartnerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_paymentsresellersubscription1 as paymentsresellersubscription1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use paymentsresellersubscription1::{PaymentsResellerSubscription, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PaymentsResellerSubscription::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.partners().subscriptions_get("name")
///              .doit().await;
/// # }
/// ```
pub struct PartnerSubscriptionGetCall<'a>
    where  {

    hub: &'a PaymentsResellerSubscription<>,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a> client::CallBuilder for PartnerSubscriptionGetCall<'a> {}

impl<'a> PartnerSubscriptionGetCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudPaymentsResellerSubscriptionV1Subscription)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "paymentsresellersubscription.partners.subscriptions.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+name}";
        
        let key = dlg.api_key();
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["name"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone());


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
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
                            sleep(d);
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


    /// Required. The name of the subscription resource to retrieve. It will have the format of "partners/{partner_id}/subscriptions/{subscription_id}"
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> PartnerSubscriptionGetCall<'a> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PartnerSubscriptionGetCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PartnerSubscriptionGetCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Used by partners to provision a subscription for their customers. This creates a subscription without associating it with the end user account. EntitleSubscription must be called separately using OAuth in order for the end user account to be associated with the subscription. It should be called directly by the partner using service accounts.
///
/// A builder for the *subscriptions.provision* method supported by a *partner* resource.
/// It is not used directly, but through a `PartnerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_paymentsresellersubscription1 as paymentsresellersubscription1;
/// use paymentsresellersubscription1::api::GoogleCloudPaymentsResellerSubscriptionV1Subscription;
/// # async fn dox() {
/// # use std::default::Default;
/// # use paymentsresellersubscription1::{PaymentsResellerSubscription, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PaymentsResellerSubscription::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudPaymentsResellerSubscriptionV1Subscription::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.partners().subscriptions_provision(req, "parent")
///              .subscription_id("invidunt")
///              .doit().await;
/// # }
/// ```
pub struct PartnerSubscriptionProvisionCall<'a>
    where  {

    hub: &'a PaymentsResellerSubscription<>,
    _request: GoogleCloudPaymentsResellerSubscriptionV1Subscription,
    _parent: String,
    _subscription_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a> client::CallBuilder for PartnerSubscriptionProvisionCall<'a> {}

impl<'a> PartnerSubscriptionProvisionCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudPaymentsResellerSubscriptionV1Subscription)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "paymentsresellersubscription.partners.subscriptions.provision",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        if let Some(value) = self._subscription_id {
            params.push(("subscriptionId", value.to_string()));
        }
        for &field in ["alt", "parent", "subscriptionId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}/subscriptions:provision";
        
        let key = dlg.api_key();
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["parent"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone());


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
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
                            sleep(d);
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
    pub fn request(mut self, new_value: GoogleCloudPaymentsResellerSubscriptionV1Subscription) -> PartnerSubscriptionProvisionCall<'a> {
        self._request = new_value;
        self
    }
    /// Required. The parent resource name, which is the identifier of the partner. It will have the format of "partners/{partner_id}".
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> PartnerSubscriptionProvisionCall<'a> {
        self._parent = new_value.to_string();
        self
    }
    /// Required. Identifies the subscription resource on the Partner side. The value is restricted to 63 ASCII characters at the maximum. If a subscription was previously created with the same subscription_id, we will directly return that one.
    ///
    /// Sets the *subscription id* query property to the given value.
    pub fn subscription_id(mut self, new_value: &str) -> PartnerSubscriptionProvisionCall<'a> {
        self._subscription_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PartnerSubscriptionProvisionCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PartnerSubscriptionProvisionCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Used by partners to revoke the pending cancellation of a subscription, which is currently in `STATE_CANCEL_AT_END_OF_CYCLE` state. If the subscription is already cancelled, the request will fail. It should be called directly by the partner using service accounts.
///
/// A builder for the *subscriptions.undoCancel* method supported by a *partner* resource.
/// It is not used directly, but through a `PartnerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_paymentsresellersubscription1 as paymentsresellersubscription1;
/// use paymentsresellersubscription1::api::GoogleCloudPaymentsResellerSubscriptionV1UndoCancelSubscriptionRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use paymentsresellersubscription1::{PaymentsResellerSubscription, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PaymentsResellerSubscription::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudPaymentsResellerSubscriptionV1UndoCancelSubscriptionRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.partners().subscriptions_undo_cancel(req, "name")
///              .doit().await;
/// # }
/// ```
pub struct PartnerSubscriptionUndoCancelCall<'a>
    where  {

    hub: &'a PaymentsResellerSubscription<>,
    _request: GoogleCloudPaymentsResellerSubscriptionV1UndoCancelSubscriptionRequest,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a> client::CallBuilder for PartnerSubscriptionUndoCancelCall<'a> {}

impl<'a> PartnerSubscriptionUndoCancelCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudPaymentsResellerSubscriptionV1UndoCancelSubscriptionResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "paymentsresellersubscription.partners.subscriptions.undoCancel",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+name}:undoCancel";
        
        let key = dlg.api_key();
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["name"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone());


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
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
                            sleep(d);
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
    pub fn request(mut self, new_value: GoogleCloudPaymentsResellerSubscriptionV1UndoCancelSubscriptionRequest) -> PartnerSubscriptionUndoCancelCall<'a> {
        self._request = new_value;
        self
    }
    /// Required. The name of the subscription resource whose pending cancellation needs to be undone. It will have the format of "partners/{partner_id}/subscriptions/{subscription_id}"
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> PartnerSubscriptionUndoCancelCall<'a> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PartnerSubscriptionUndoCancelCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PartnerSubscriptionUndoCancelCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


