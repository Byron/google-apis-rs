use super::*;
/// An AdvertiserResource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get advertisers](AdvertiserGetCall) (response)
/// * [list advertisers](AdvertiserListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Advertiser {
    /// True if the advertiser allows publisher created links, otherwise false.
    #[serde(rename="allowPublisherCreatedLinks")]
    
    pub allow_publisher_created_links: Option<bool>,
    /// Category that this advertiser belongs to. A valid list of categories can be found here: http://www.google.com/support/affiliatenetwork/advertiser/bin/answer.py?hl=en&answer=107581
    
    pub category: Option<String>,
    /// The longest possible length of a commission (how long the cookies on the customer's browser last before they expire).
    #[serde(rename="commissionDuration")]
    
    pub commission_duration: Option<i32>,
    /// Email that this advertiser would like publishers to contact them with.
    #[serde(rename="contactEmail")]
    
    pub contact_email: Option<String>,
    /// Phone that this advertiser would like publishers to contact them with.
    #[serde(rename="contactPhone")]
    
    pub contact_phone: Option<String>,
    /// The default link id for this advertiser.
    #[serde(rename="defaultLinkId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub default_link_id: Option<i64>,
    /// Description of the website the advertiser advertises from.
    
    pub description: Option<String>,
    /// The sum of fees paid to publishers divided by the total number of clicks over the past three months. This value should be multiplied by 100 at the time of display.
    #[serde(rename="epcNinetyDayAverage")]
    
    pub epc_ninety_day_average: Option<Money>,
    /// The sum of fees paid to publishers divided by the total number of clicks over the past seven days. This value should be multiplied by 100 at the time of display.
    #[serde(rename="epcSevenDayAverage")]
    
    pub epc_seven_day_average: Option<Money>,
    /// The ID of this advertiser.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// The requested advertiser.
    
    pub item: Option<Option<Box<Advertiser>>>,
    /// Date that this advertiser was approved as a Google Affiliate Network advertiser.
    #[serde(rename="joinDate")]
    
    pub join_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The kind for an advertiser.
    
    pub kind: Option<String>,
    /// URL to the logo this advertiser uses on the Google Affiliate Network.
    #[serde(rename="logoUrl")]
    
    pub logo_url: Option<String>,
    /// List of merchant center ids for this advertiser
    #[serde(rename="merchantCenterIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub merchant_center_ids: Option<Vec<i64>>,
    /// The name of this advertiser.
    
    pub name: Option<String>,
    /// A rank based on commissions paid to publishers over the past 90 days. A number between 1 and 4 where 4 means the top quartile (most money paid) and 1 means the bottom quartile (least money paid).
    #[serde(rename="payoutRank")]
    
    pub payout_rank: Option<String>,
    /// Allows advertisers to submit product listings to Google Product Search.
    #[serde(rename="productFeedsEnabled")]
    
    pub product_feeds_enabled: Option<bool>,
    /// List of redirect URLs for this advertiser
    #[serde(rename="redirectDomains")]
    
    pub redirect_domains: Option<Vec<String>>,
    /// URL of the website this advertiser advertises from.
    #[serde(rename="siteUrl")]
    
    pub site_url: Option<String>,
    /// The status of the requesting publisher's relationship this advertiser.
    
    pub status: Option<String>,
}

impl client::Resource for Advertiser {}
impl client::ResponseResult for Advertiser {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list advertisers](AdvertiserListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Advertisers {
    /// The advertiser list.
    
    pub items: Option<Vec<Advertiser>>,
    /// The kind for a page of advertisers.
    
    pub kind: Option<String>,
    /// The 'pageToken' to pass to the next request to get the next page, if there are more to retrieve.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for Advertisers {}


/// A credit card offer. There are many possible result fields. We provide two different views of the data, or “projections.” The “full” projection includes every result field. And the “summary” projection, which is the default, includes a smaller subset of the fields. The fields included in the summary projection are marked as such in their descriptions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list cc offers](CcOfferListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CcOffer {
    /// More marketing copy about the card's benefits. A summary field.
    #[serde(rename="additionalCardBenefits")]
    
    pub additional_card_benefits: Option<Vec<String>>,
    /// Any extra fees levied on card holders.
    #[serde(rename="additionalCardHolderFee")]
    
    pub additional_card_holder_fee: Option<String>,
    /// The youngest a recipient of this card may be.
    #[serde(rename="ageMinimum")]
    
    pub age_minimum: Option<f64>,
    /// Text describing the details of the age minimum restriction.
    #[serde(rename="ageMinimumDetails")]
    
    pub age_minimum_details: Option<String>,
    /// The ongoing annual fee, in dollars.
    #[serde(rename="annualFee")]
    
    pub annual_fee: Option<f64>,
    /// Text describing the annual fee, including any difference for the first year. A summary field.
    #[serde(rename="annualFeeDisplay")]
    
    pub annual_fee_display: Option<String>,
    /// The largest number of units you may accumulate in a year.
    #[serde(rename="annualRewardMaximum")]
    
    pub annual_reward_maximum: Option<f64>,
    /// Possible categories for this card, eg "Low Interest" or "Good." A summary field.
    #[serde(rename="approvedCategories")]
    
    pub approved_categories: Option<Vec<String>>,
    /// Text describing the purchase APR. A summary field.
    #[serde(rename="aprDisplay")]
    
    pub apr_display: Option<String>,
    /// Text describing how the balance is computed. A summary field.
    #[serde(rename="balanceComputationMethod")]
    
    pub balance_computation_method: Option<String>,
    /// Text describing the terms for balance transfers. A summary field.
    #[serde(rename="balanceTransferTerms")]
    
    pub balance_transfer_terms: Option<String>,
    /// For cards with rewards programs, extra circumstances whereby additional rewards may be granted.
    #[serde(rename="bonusRewards")]
    
    pub bonus_rewards: Option<Vec<CcOfferBonusRewards>>,
    /// If you get coverage when you use the card for the given activity, this field describes it.
    #[serde(rename="carRentalInsurance")]
    
    pub car_rental_insurance: Option<String>,
    /// A list of what the issuer thinks are the most important benefits of the card. Usually summarizes the rewards program, if there is one. A summary field.
    #[serde(rename="cardBenefits")]
    
    pub card_benefits: Option<Vec<String>>,
    /// The issuer's name for the card, including any trademark or service mark designators. A summary field.
    #[serde(rename="cardName")]
    
    pub card_name: Option<String>,
    /// What kind of credit card this is, for example secured or unsecured.
    #[serde(rename="cardType")]
    
    pub card_type: Option<String>,
    /// Text describing the terms for cash advances. A summary field.
    #[serde(rename="cashAdvanceTerms")]
    
    pub cash_advance_terms: Option<String>,
    /// The high end for credit limits the issuer imposes on recipients of this card.
    #[serde(rename="creditLimitMax")]
    
    pub credit_limit_max: Option<f64>,
    /// The low end for credit limits the issuer imposes on recipients of this card.
    #[serde(rename="creditLimitMin")]
    
    pub credit_limit_min: Option<f64>,
    /// Text describing the credit ratings required for recipients of this card, for example "Excellent/Good." A summary field.
    #[serde(rename="creditRatingDisplay")]
    
    pub credit_rating_display: Option<String>,
    /// Fees for defaulting on your payments.
    #[serde(rename="defaultFees")]
    
    pub default_fees: Option<Vec<CcOfferDefaultFees>>,
    /// A notice that, if present, is referenced via an asterisk by many of the other summary fields. If this field is present, it will always start with an asterisk ("*"), and must be prominently displayed with the offer. A summary field.
    
    pub disclaimer: Option<String>,
    /// If you get coverage when you use the card for the given activity, this field describes it.
    #[serde(rename="emergencyInsurance")]
    
    pub emergency_insurance: Option<String>,
    /// Whether this card is only available to existing customers of the issuer.
    #[serde(rename="existingCustomerOnly")]
    
    pub existing_customer_only: Option<bool>,
    /// If you get coverage when you use the card for the given activity, this field describes it.
    #[serde(rename="extendedWarranty")]
    
    pub extended_warranty: Option<String>,
    /// The annual fee for the first year, if different from the ongoing fee. Optional.
    #[serde(rename="firstYearAnnualFee")]
    
    pub first_year_annual_fee: Option<f64>,
    /// If you get coverage when you use the card for the given activity, this field describes it.
    #[serde(rename="flightAccidentInsurance")]
    
    pub flight_accident_insurance: Option<String>,
    /// Fee for each transaction involving a foreign currency.
    #[serde(rename="foreignCurrencyTransactionFee")]
    
    pub foreign_currency_transaction_fee: Option<String>,
    /// If you get coverage when you use the card for the given activity, this field describes it.
    #[serde(rename="fraudLiability")]
    
    pub fraud_liability: Option<String>,
    /// Text describing the grace period before finance charges apply. A summary field.
    #[serde(rename="gracePeriodDisplay")]
    
    pub grace_period_display: Option<String>,
    /// The link to the image of the card that is shown on Connect Commerce. A summary field.
    #[serde(rename="imageUrl")]
    
    pub image_url: Option<String>,
    /// Fee for setting up the card.
    #[serde(rename="initialSetupAndProcessingFee")]
    
    pub initial_setup_and_processing_fee: Option<String>,
    /// Text describing the terms for introductory period balance transfers. A summary field.
    #[serde(rename="introBalanceTransferTerms")]
    
    pub intro_balance_transfer_terms: Option<String>,
    /// Text describing the terms for introductory period cash advances. A summary field.
    #[serde(rename="introCashAdvanceTerms")]
    
    pub intro_cash_advance_terms: Option<String>,
    /// Text describing the terms for introductory period purchases. A summary field.
    #[serde(rename="introPurchaseTerms")]
    
    pub intro_purchase_terms: Option<String>,
    /// Name of card issuer. A summary field.
    
    pub issuer: Option<String>,
    /// The Google Affiliate Network ID of the advertiser making this offer.
    #[serde(rename="issuerId")]
    
    pub issuer_id: Option<String>,
    /// The generic link to the issuer's site.
    #[serde(rename="issuerWebsite")]
    
    pub issuer_website: Option<String>,
    /// The kind for one credit card offer. A summary field.
    
    pub kind: Option<String>,
    /// The link to the issuer's page for this card. A summary field.
    #[serde(rename="landingPageUrl")]
    
    pub landing_page_url: Option<String>,
    /// Text describing how much a late payment will cost, eg "up to $35." A summary field.
    #[serde(rename="latePaymentFee")]
    
    pub late_payment_fee: Option<String>,
    /// If you get coverage when you use the card for the given activity, this field describes it.
    #[serde(rename="luggageInsurance")]
    
    pub luggage_insurance: Option<String>,
    /// The highest interest rate the issuer charges on this card. Expressed as an absolute number, not as a percentage.
    #[serde(rename="maxPurchaseRate")]
    
    pub max_purchase_rate: Option<f64>,
    /// The lowest interest rate the issuer charges on this card. Expressed as an absolute number, not as a percentage.
    #[serde(rename="minPurchaseRate")]
    
    pub min_purchase_rate: Option<f64>,
    /// Text describing how much missing the grace period will cost.
    #[serde(rename="minimumFinanceCharge")]
    
    pub minimum_finance_charge: Option<String>,
    /// Which network (eg Visa) the card belongs to. A summary field.
    
    pub network: Option<String>,
    /// This offer's ID. A summary field.
    #[serde(rename="offerId")]
    
    pub offer_id: Option<String>,
    /// Whether a cash reward program lets you get cash back sooner than end of year or other longish period.
    #[serde(rename="offersImmediateCashReward")]
    
    pub offers_immediate_cash_reward: Option<bool>,
    /// Fee for exceeding the card's charge limit.
    #[serde(rename="overLimitFee")]
    
    pub over_limit_fee: Option<String>,
    /// Categories in which the issuer does not wish the card to be displayed. A summary field.
    #[serde(rename="prohibitedCategories")]
    
    pub prohibited_categories: Option<Vec<String>>,
    /// Text describing any additional details for the purchase rate. A summary field.
    #[serde(rename="purchaseRateAdditionalDetails")]
    
    pub purchase_rate_additional_details: Option<String>,
    /// Fixed or variable.
    #[serde(rename="purchaseRateType")]
    
    pub purchase_rate_type: Option<String>,
    /// Text describing the fee for a payment that doesn't clear. A summary field.
    #[serde(rename="returnedPaymentFee")]
    
    pub returned_payment_fee: Option<String>,
    /// The company that redeems the rewards, if different from the issuer.
    #[serde(rename="rewardPartner")]
    
    pub reward_partner: Option<String>,
    /// For cards with rewards programs, the unit of reward. For example, miles, cash back, points.
    #[serde(rename="rewardUnit")]
    
    pub reward_unit: Option<String>,
    /// For cards with rewards programs, detailed rules about how the program works.
    
    pub rewards: Option<Vec<CcOfferRewards>>,
    /// Whether accumulated rewards ever expire.
    #[serde(rename="rewardsExpire")]
    
    pub rewards_expire: Option<bool>,
    /// For airline miles rewards, tells whether blackout dates apply to the miles.
    #[serde(rename="rewardsHaveBlackoutDates")]
    
    pub rewards_have_blackout_dates: Option<bool>,
    /// Fee for requesting a copy of your statement.
    #[serde(rename="statementCopyFee")]
    
    pub statement_copy_fee: Option<String>,
    /// The link to ping to register a click on this offer. A summary field.
    #[serde(rename="trackingUrl")]
    
    pub tracking_url: Option<String>,
    /// If you get coverage when you use the card for the given activity, this field describes it.
    #[serde(rename="travelInsurance")]
    
    pub travel_insurance: Option<String>,
    /// When variable rates were last updated.
    #[serde(rename="variableRatesLastUpdated")]
    
    pub variable_rates_last_updated: Option<String>,
    /// How often variable rates are updated.
    #[serde(rename="variableRatesUpdateFrequency")]
    
    pub variable_rates_update_frequency: Option<String>,
}

impl client::Resource for CcOffer {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list cc offers](CcOfferListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CcOffers {
    /// The credit card offers.
    
    pub items: Option<Vec<CcOffer>>,
    /// The kind for a page of credit card offers.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for CcOffers {}


/// An EventResource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list events](EventListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    /// The ID of advertiser for this event.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// The name of the advertiser for this event.
    #[serde(rename="advertiserName")]
    
    pub advertiser_name: Option<String>,
    /// The charge ID for this event. Only returned for charge events.
    #[serde(rename="chargeId")]
    
    pub charge_id: Option<String>,
    /// Charge type of the event (other|slotting_fee|monthly_minimum|tier_bonus|debit|credit). Only returned for charge events.
    #[serde(rename="chargeType")]
    
    pub charge_type: Option<String>,
    /// Amount of money exchanged during the transaction. Only returned for charge and conversion events.
    #[serde(rename="commissionableSales")]
    
    pub commissionable_sales: Option<Money>,
    /// Earnings by the publisher.
    
    pub earnings: Option<Money>,
    /// The date-time this event was initiated as a RFC 3339 date-time value.
    #[serde(rename="eventDate")]
    
    pub event_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The kind for one event.
    
    pub kind: Option<String>,
    /// The ID of the member attached to this event. Only returned for conversion events.
    #[serde(rename="memberId")]
    
    pub member_id: Option<String>,
    /// The date-time this event was last modified as a RFC 3339 date-time value.
    #[serde(rename="modifyDate")]
    
    pub modify_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Fee that the advertiser paid to the Google Affiliate Network.
    #[serde(rename="networkFee")]
    
    pub network_fee: Option<Money>,
    /// The order ID for this event. Only returned for conversion events.
    #[serde(rename="orderId")]
    
    pub order_id: Option<String>,
    /// Products associated with the event.
    
    pub products: Option<Vec<EventProducts>>,
    /// Fee that the advertiser paid to the publisher.
    #[serde(rename="publisherFee")]
    
    pub publisher_fee: Option<Money>,
    /// The ID of the publisher for this event.
    #[serde(rename="publisherId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub publisher_id: Option<i64>,
    /// The name of the publisher for this event.
    #[serde(rename="publisherName")]
    
    pub publisher_name: Option<String>,
    /// Status of the event (active|canceled). Only returned for charge and conversion events.
    
    pub status: Option<String>,
    /// Type of the event (action|transaction|charge).
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Resource for Event {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list events](EventListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Events {
    /// The event list.
    
    pub items: Option<Vec<Event>>,
    /// The kind for a page of events.
    
    pub kind: Option<String>,
    /// The 'pageToken' to pass to the next request to get the next page, if there are more to retrieve.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for Events {}


/// A LinkResource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get links](LinkGetCall) (response)
/// * [insert links](LinkInsertCall) (request|response)
/// * [list links](LinkListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    /// The advertiser id for the advertiser who owns this link.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Authorship
    
    pub authorship: Option<String>,
    /// Availability.
    
    pub availability: Option<String>,
    /// Tracking url for clicks.
    #[serde(rename="clickTrackingUrl")]
    
    pub click_tracking_url: Option<String>,
    /// Date that this link was created.
    #[serde(rename="createDate")]
    
    pub create_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Description.
    
    pub description: Option<String>,
    /// The destination URL for the link.
    #[serde(rename="destinationUrl")]
    
    pub destination_url: Option<String>,
    /// Duration
    
    pub duration: Option<String>,
    /// Date that this link becomes inactive.
    #[serde(rename="endDate")]
    
    pub end_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The sum of fees paid to publishers divided by the total number of clicks over the past three months on this link. This value should be multiplied by 100 at the time of display.
    #[serde(rename="epcNinetyDayAverage")]
    
    pub epc_ninety_day_average: Option<Money>,
    /// The sum of fees paid to publishers divided by the total number of clicks over the past seven days on this link. This value should be multiplied by 100 at the time of display.
    #[serde(rename="epcSevenDayAverage")]
    
    pub epc_seven_day_average: Option<Money>,
    /// The ID of this link.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// image alt text.
    #[serde(rename="imageAltText")]
    
    pub image_alt_text: Option<String>,
    /// Tracking url for impressions.
    #[serde(rename="impressionTrackingUrl")]
    
    pub impression_tracking_url: Option<String>,
    /// Flag for if this link is active.
    #[serde(rename="isActive")]
    
    pub is_active: Option<bool>,
    /// The kind for one entity.
    
    pub kind: Option<String>,
    /// The link type.
    #[serde(rename="linkType")]
    
    pub link_type: Option<String>,
    /// The logical name for this link.
    
    pub name: Option<String>,
    /// Promotion Type
    #[serde(rename="promotionType")]
    
    pub promotion_type: Option<String>,
    /// Special offers on the link.
    #[serde(rename="specialOffers")]
    
    pub special_offers: Option<LinkSpecialOffers>,
    /// Date that this link becomes active.
    #[serde(rename="startDate")]
    
    pub start_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Link {}
impl client::Resource for Link {}
impl client::ResponseResult for Link {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list links](LinkListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Links {
    /// The links.
    
    pub items: Option<Vec<Link>>,
    /// The kind for a page of links.
    
    pub kind: Option<String>,
    /// The next page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for Links {}


/// An ApiMoneyProto.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Money {
    /// The amount of money.
    
    pub amount: Option<f64>,
    /// The 3-letter code of the currency in question.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
}

impl client::Part for Money {}


/// A PublisherResource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get publishers](PublisherGetCall) (response)
/// * [list publishers](PublisherListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Publisher {
    /// Classification that this publisher belongs to. See this link for all publisher classifications: http://www.google.com/support/affiliatenetwork/advertiser/bin/answer.py?hl=en&answer=107625&ctx=cb&src=cb&cbid=-k5fihzthfaik&cbrank=4
    
    pub classification: Option<String>,
    /// The sum of fees paid to this publisher divided by the total number of clicks over the past three months. Values are multiplied by 100 for display purposes.
    #[serde(rename="epcNinetyDayAverage")]
    
    pub epc_ninety_day_average: Option<Money>,
    /// The sum of fees paid to this publisher divided by the total number of clicks over the past seven days. Values are multiplied by 100 for display purposes.
    #[serde(rename="epcSevenDayAverage")]
    
    pub epc_seven_day_average: Option<Money>,
    /// The ID of this publisher.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// The requested publisher.
    
    pub item: Option<Option<Box<Publisher>>>,
    /// Date that this publisher was approved as a Google Affiliate Network publisher.
    #[serde(rename="joinDate")]
    
    pub join_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The kind for a publisher.
    
    pub kind: Option<String>,
    /// The name of this publisher.
    
    pub name: Option<String>,
    /// A rank based on commissions paid to this publisher over the past 90 days. A number between 1 and 4 where 4 means the top quartile (most money paid) and 1 means the bottom quartile (least money paid).
    #[serde(rename="payoutRank")]
    
    pub payout_rank: Option<String>,
    /// Websites that this publisher uses to advertise.
    
    pub sites: Option<Vec<String>>,
    /// The status of the requesting advertiser's relationship with this publisher.
    
    pub status: Option<String>,
}

impl client::Resource for Publisher {}
impl client::ResponseResult for Publisher {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list publishers](PublisherListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Publishers {
    /// The entity list.
    
    pub items: Option<Vec<Publisher>>,
    /// The kind for a page of entities.
    
    pub kind: Option<String>,
    /// The 'pageToken' to pass to the next request to get the next page, if there are more to retrieve.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for Publishers {}


/// A ReportResource representing a report of a certain type either for an advertiser or publisher.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get reports](ReportGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Report {
    /// The column names for the report
    
    pub column_names: Option<Vec<String>>,
    /// The end of the date range for this report, exclusive.
    
    pub end_date: Option<String>,
    /// The kind for a report.
    
    pub kind: Option<String>,
    /// The number of matching rows before paging is applied.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub matching_row_count: Option<i64>,
    /// The rows of data for the report
    
    pub rows: Option<Vec<Vec<json::Value>>>,
    /// The start of the date range for this report, inclusive.
    
    pub start_date: Option<String>,
    /// The totals rows for the report
    
    pub totals_rows: Option<Vec<Vec<json::Value>>>,
    /// The report type.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Resource for Report {}
impl client::ResponseResult for Report {}


/// For cards with rewards programs, extra circumstances whereby additional rewards may be granted.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CcOfferBonusRewards {
    /// How many units of reward will be granted.
    
    pub amount: Option<f64>,
    /// The circumstances under which this rule applies, for example, booking a flight via Orbitz.
    
    pub details: Option<String>,
}

impl client::NestedType for CcOfferBonusRewards {}
impl client::Part for CcOfferBonusRewards {}


/// Fees for defaulting on your payments.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
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

impl client::NestedType for CcOfferDefaultFees {}
impl client::Part for CcOfferDefaultFees {}


/// For cards with rewards programs, detailed rules about how the program works.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CcOfferRewards {
    /// Other limits, for example, if this rule only applies during an introductory period.
    #[serde(rename="additionalDetails")]
    
    pub additional_details: Option<String>,
    /// The number of units rewarded per purchase dollar.
    
    pub amount: Option<f64>,
    /// The kind of purchases covered by this rule.
    
    pub category: Option<String>,
    /// How long rewards granted by this rule last.
    #[serde(rename="expirationMonths")]
    
    pub expiration_months: Option<f64>,
    /// The maximum purchase amount in the given category for this rule to apply.
    #[serde(rename="maxRewardTier")]
    
    pub max_reward_tier: Option<f64>,
    /// The minimum purchase amount in the given category before this rule applies.
    #[serde(rename="minRewardTier")]
    
    pub min_reward_tier: Option<f64>,
}

impl client::NestedType for CcOfferRewards {}
impl client::Part for CcOfferRewards {}


/// Products associated with the event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventProducts {
    /// Id of the category this product belongs to.
    #[serde(rename="categoryId")]
    
    pub category_id: Option<String>,
    /// Name of the category this product belongs to.
    #[serde(rename="categoryName")]
    
    pub category_name: Option<String>,
    /// Amount earned by the publisher on this product.
    
    pub earnings: Option<Money>,
    /// Fee that the advertiser paid to the Google Affiliate Network for this product.
    #[serde(rename="networkFee")]
    
    pub network_fee: Option<Money>,
    /// Fee that the advertiser paid to the publisehr for this product.
    #[serde(rename="publisherFee")]
    
    pub publisher_fee: Option<Money>,
    /// Quantity of this product bought/exchanged.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub quantity: Option<i64>,
    /// Sku of this product.
    
    pub sku: Option<String>,
    /// Sku name of this product.
    #[serde(rename="skuName")]
    
    pub sku_name: Option<String>,
    /// Price per unit of this product.
    #[serde(rename="unitPrice")]
    
    pub unit_price: Option<Money>,
}

impl client::NestedType for EventProducts {}
impl client::Part for EventProducts {}


/// Special offers on the link.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LinkSpecialOffers {
    /// Whether there is a free gift
    #[serde(rename="freeGift")]
    
    pub free_gift: Option<bool>,
    /// Whether there is free shipping
    #[serde(rename="freeShipping")]
    
    pub free_shipping: Option<bool>,
    /// Minimum purchase amount for free shipping promotion
    #[serde(rename="freeShippingMin")]
    
    pub free_shipping_min: Option<Money>,
    /// Percent off on the purchase
    #[serde(rename="percentOff")]
    
    pub percent_off: Option<f64>,
    /// Minimum purchase amount for percent off promotion
    #[serde(rename="percentOffMin")]
    
    pub percent_off_min: Option<Money>,
    /// Price cut on the purchase
    #[serde(rename="priceCut")]
    
    pub price_cut: Option<Money>,
    /// Minimum purchase amount for price cut promotion
    #[serde(rename="priceCutMin")]
    
    pub price_cut_min: Option<Money>,
    /// List of promotion code associated with the link
    #[serde(rename="promotionCodes")]
    
    pub promotion_codes: Option<Vec<String>>,
}

impl client::NestedType for LinkSpecialOffers {}
impl client::Part for LinkSpecialOffers {}


