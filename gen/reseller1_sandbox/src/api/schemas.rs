use super::*;
/// JSON template for address of a customer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Address {
    /// Address line 1 of the address.
    #[serde(rename="addressLine1")]
    
    pub address_line1: Option<String>,
    /// Address line 2 of the address.
    #[serde(rename="addressLine2")]
    
    pub address_line2: Option<String>,
    /// Address line 3 of the address.
    #[serde(rename="addressLine3")]
    
    pub address_line3: Option<String>,
    /// Name of the contact person.
    #[serde(rename="contactName")]
    
    pub contact_name: Option<String>,
    /// ISO 3166 country code.
    #[serde(rename="countryCode")]
    
    pub country_code: Option<String>,
    /// Identifies the resource as a customer address.
    
    pub kind: Option<String>,
    /// Name of the locality. This is in accordance with - http://portablecontacts.net/draft-spec.html#address_element.
    
    pub locality: Option<String>,
    /// Name of the organization.
    #[serde(rename="organizationName")]
    
    pub organization_name: Option<String>,
    /// The postal code. This is in accordance with - http://portablecontacts.net/draft-spec.html#address_element.
    #[serde(rename="postalCode")]
    
    pub postal_code: Option<String>,
    /// Name of the region. This is in accordance with - http://portablecontacts.net/draft-spec.html#address_element.
    
    pub region: Option<String>,
}

impl client::Part for Address {}


/// JSON template for the ChangePlan rpc request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [change plan subscriptions](SubscriptionChangePlanCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChangePlanRequest {
    /// External name of the deal code applicable for the subscription. This field is optional. If missing, the deal price plan won't be used.
    #[serde(rename="dealCode")]
    
    pub deal_code: Option<String>,
    /// Identifies the resource as a subscription change plan request.
    
    pub kind: Option<String>,
    /// Name of the plan to change to.
    #[serde(rename="planName")]
    
    pub plan_name: Option<String>,
    /// Purchase order id for your order tracking purposes.
    #[serde(rename="purchaseOrderId")]
    
    pub purchase_order_id: Option<String>,
    /// Number/Limit of seats in the new plan.
    
    pub seats: Option<Seats>,
}

impl client::RequestValue for ChangePlanRequest {}


/// JSON template for a customer.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get customers](CustomerGetCall) (response)
/// * [insert customers](CustomerInsertCall) (request|response)
/// * [patch customers](CustomerPatchCall) (request|response)
/// * [update customers](CustomerUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Customer {
    /// The alternate email of the customer.
    #[serde(rename="alternateEmail")]
    
    pub alternate_email: Option<String>,
    /// The domain name of the customer.
    #[serde(rename="customerDomain")]
    
    pub customer_domain: Option<String>,
    /// Whether the customer's primary domain has been verified.
    #[serde(rename="customerDomainVerified")]
    
    pub customer_domain_verified: Option<bool>,
    /// The id of the customer.
    #[serde(rename="customerId")]
    
    pub customer_id: Option<String>,
    /// Identifies the resource as a customer.
    
    pub kind: Option<String>,
    /// The phone number of the customer.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
    /// The postal address of the customer.
    #[serde(rename="postalAddress")]
    
    pub postal_address: Option<Address>,
    /// Ui url for customer resource.
    #[serde(rename="resourceUiUrl")]
    
    pub resource_ui_url: Option<String>,
}

impl client::RequestValue for Customer {}
impl client::Resource for Customer {}
impl client::ResponseResult for Customer {}


/// JSON template for a subscription renewal settings.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [change renewal settings subscriptions](SubscriptionChangeRenewalSettingCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RenewalSettings {
    /// Identifies the resource as a subscription renewal setting.
    
    pub kind: Option<String>,
    /// Subscription renewal type.
    #[serde(rename="renewalType")]
    
    pub renewal_type: Option<String>,
}

impl client::RequestValue for RenewalSettings {}


/// JSON template for subscription seats.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [change seats subscriptions](SubscriptionChangeSeatCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Seats {
    /// Identifies the resource as a subscription change plan request.
    
    pub kind: Option<String>,
    /// Read-only field containing the current number of licensed seats for FLEXIBLE Google-Apps subscriptions and secondary subscriptions such as Google-Vault and Drive-storage.
    #[serde(rename="licensedNumberOfSeats")]
    
    pub licensed_number_of_seats: Option<i32>,
    /// Maximum number of seats that can be purchased. This needs to be provided only for a non-commitment plan. For a commitment plan it is decided by the contract.
    #[serde(rename="maximumNumberOfSeats")]
    
    pub maximum_number_of_seats: Option<i32>,
    /// Number of seats to purchase. This is applicable only for a commitment plan.
    #[serde(rename="numberOfSeats")]
    
    pub number_of_seats: Option<i32>,
}

impl client::RequestValue for Seats {}


/// JSON template for a subscription.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [activate subscriptions](SubscriptionActivateCall) (response)
/// * [change plan subscriptions](SubscriptionChangePlanCall) (response)
/// * [change renewal settings subscriptions](SubscriptionChangeRenewalSettingCall) (response)
/// * [change seats subscriptions](SubscriptionChangeSeatCall) (response)
/// * [delete subscriptions](SubscriptionDeleteCall) (none)
/// * [get subscriptions](SubscriptionGetCall) (response)
/// * [insert subscriptions](SubscriptionInsertCall) (request|response)
/// * [list subscriptions](SubscriptionListCall) (none)
/// * [start paid service subscriptions](SubscriptionStartPaidServiceCall) (response)
/// * [suspend subscriptions](SubscriptionSuspendCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Subscription {
    /// Billing method of this subscription.
    #[serde(rename="billingMethod")]
    
    pub billing_method: Option<String>,
    /// Creation time of this subscription in milliseconds since Unix epoch.
    #[serde(rename="creationTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub creation_time: Option<i64>,
    /// Primary domain name of the customer
    #[serde(rename="customerDomain")]
    
    pub customer_domain: Option<String>,
    /// The id of the customer to whom the subscription belongs.
    #[serde(rename="customerId")]
    
    pub customer_id: Option<String>,
    /// External name of the deal, if this subscription was provisioned under one. Otherwise this field will be empty.
    #[serde(rename="dealCode")]
    
    pub deal_code: Option<String>,
    /// Identifies the resource as a Subscription.
    
    pub kind: Option<String>,
    /// Plan details of the subscription
    
    pub plan: Option<SubscriptionPlan>,
    /// Purchase order id for your order tracking purposes.
    #[serde(rename="purchaseOrderId")]
    
    pub purchase_order_id: Option<String>,
    /// Renewal settings of the subscription.
    #[serde(rename="renewalSettings")]
    
    pub renewal_settings: Option<RenewalSettings>,
    /// Ui url for subscription resource.
    #[serde(rename="resourceUiUrl")]
    
    pub resource_ui_url: Option<String>,
    /// Number/Limit of seats in the new plan.
    
    pub seats: Option<Seats>,
    /// Name of the sku for which this subscription is purchased.
    #[serde(rename="skuId")]
    
    pub sku_id: Option<String>,
    /// Status of the subscription.
    
    pub status: Option<String>,
    /// The id of the subscription.
    #[serde(rename="subscriptionId")]
    
    pub subscription_id: Option<String>,
    /// Read-only field containing an enumerable of all the current suspension reasons for a subscription. It is possible for a subscription to have many concurrent, overlapping suspension reasons. A subscription's STATUS is SUSPENDED until all pending suspensions are removed. Possible options include:  
    /// - PENDING_TOS_ACCEPTANCE - The customer has not logged in and accepted the Google Apps Resold Terms of Services.  
    /// - RENEWAL_WITH_TYPE_CANCEL - The customer's commitment ended and their service was cancelled at the end of their term.  
    /// - RESELLER_INITIATED - A manual suspension invoked by a Reseller.  
    /// - TRIAL_ENDED - The customer's trial expired without a plan selected.  
    /// - OTHER - The customer is suspended for an internal Google reason (e.g. abuse or otherwise).
    #[serde(rename="suspensionReasons")]
    
    pub suspension_reasons: Option<Vec<String>>,
    /// Transfer related information for the subscription.
    #[serde(rename="transferInfo")]
    
    pub transfer_info: Option<SubscriptionTransferInfo>,
    /// Trial Settings of the subscription.
    #[serde(rename="trialSettings")]
    
    pub trial_settings: Option<SubscriptionTrialSettings>,
}

impl client::RequestValue for Subscription {}
impl client::Resource for Subscription {}
impl client::ResponseResult for Subscription {}


/// JSON template for a subscription list.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list subscriptions](SubscriptionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Subscriptions {
    /// Identifies the resource as a collection of subscriptions.
    
    pub kind: Option<String>,
    /// The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The subscriptions in this page of results.
    
    pub subscriptions: Option<Vec<Subscription>>,
}

impl client::ResponseResult for Subscriptions {}


/// Plan details of the subscription
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionPlan {
    /// Interval of the commitment if it is a commitment plan.
    #[serde(rename="commitmentInterval")]
    
    pub commitment_interval: Option<SubscriptionPlanCommitmentInterval>,
    /// Whether the plan is a commitment plan or not.
    #[serde(rename="isCommitmentPlan")]
    
    pub is_commitment_plan: Option<bool>,
    /// The plan name of this subscription's plan.
    #[serde(rename="planName")]
    
    pub plan_name: Option<String>,
}

impl client::NestedType for SubscriptionPlan {}
impl client::Part for SubscriptionPlan {}


/// Interval of the commitment if it is a commitment plan.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionPlanCommitmentInterval {
    /// End time of the commitment interval in milliseconds since Unix epoch.
    #[serde(rename="endTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub end_time: Option<i64>,
    /// Start time of the commitment interval in milliseconds since Unix epoch.
    #[serde(rename="startTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start_time: Option<i64>,
}

impl client::NestedType for SubscriptionPlanCommitmentInterval {}
impl client::Part for SubscriptionPlanCommitmentInterval {}


/// Transfer related information for the subscription.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionTransferInfo {
    /// no description provided
    #[serde(rename="minimumTransferableSeats")]
    
    pub minimum_transferable_seats: Option<i32>,
    /// Time when transfer token or intent to transfer will expire.
    #[serde(rename="transferabilityExpirationTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub transferability_expiration_time: Option<i64>,
}

impl client::NestedType for SubscriptionTransferInfo {}
impl client::Part for SubscriptionTransferInfo {}


/// Trial Settings of the subscription.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionTrialSettings {
    /// Whether the subscription is in trial.
    #[serde(rename="isInTrial")]
    
    pub is_in_trial: Option<bool>,
    /// End time of the trial in milliseconds since Unix epoch.
    #[serde(rename="trialEndTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub trial_end_time: Option<i64>,
}

impl client::NestedType for SubscriptionTrialSettings {}
impl client::Part for SubscriptionTrialSettings {}


